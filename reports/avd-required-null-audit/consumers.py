# Copyright (c) 2026 Arista Networks, Inc.
# ruff: noqa: INP001  # This is a standalone diagnostic script beside its report artifacts.
# ruff: noqa: ANN001, ANN202, B023  # Recursive AST walkers use local Jinja nodes and synchronous per-file closures.

"""
Index candidate consumer sites without claiming whole-program null safety.

Jinja paths follow simple assignments and loop aliases. Includes, macros, dynamic
indexing and Python helper calls are not a complete data-flow analysis. Guards are
recorded verbatim for human review: ``is defined`` is not a non-null guarantee.
Python matches are intentionally candidate evidence, not semantic bindings.
"""

from __future__ import annotations

import argparse
import ast
import json
from collections import Counter, defaultdict
from pathlib import Path

from jinja2 import Environment, nodes


def template_index(repo: Path) -> tuple[dict, list]:
    """Index scalar reads in outputs and conditionals using local Jinja aliases."""
    index = defaultdict(list)
    limitations = []
    environment = Environment(extensions=["jinja2.ext.do", "jinja2.ext.loopcontrols"])  # noqa: S701  # Used only to parse templates; this environment never renders content.
    template_root = repo / "python-avd/pyavd/_eos_cli_config_gen/j2templates"

    def path(node, aliases):
        if isinstance(node, nodes.Name):
            return aliases.get(node.name, "eos_config." + node.name)
        if isinstance(node, nodes.Getattr):
            parent = path(node.node, aliases)
            return parent + "." + node.attr if parent else None
        if isinstance(node, nodes.Getitem):
            parent = path(node.node, aliases)
            if parent and isinstance(node.arg, nodes.Const):
                return parent + ("[]" if isinstance(node.arg.value, int) else "." + str(node.arg.value))
            return None
        if isinstance(node, (nodes.Filter, nodes.Test)):
            return path(node.node, aliases)
        return None

    def paths(node, aliases):
        # Select maximal reads: do not count parent models as scalar reads.
        if isinstance(node, (nodes.Getattr, nodes.Getitem, nodes.Name)):
            resolved = path(node, aliases)
            if resolved:
                yield resolved
            if isinstance(node, nodes.Getitem):
                yield from paths(node.arg, aliases)
            return
        for child in node.iter_child_nodes():
            yield from paths(child, aliases)

    def nonnull(node, aliases):
        """
        Fields whose null value prevents entry into this positive branch.

        Only obvious local predicates are recognized. This does not prove a whole
        field is safe, or evaluate parent guards, includes or filter semantics.
        """
        if isinstance(node, nodes.And):
            return nonnull(node.left, aliases) | nonnull(node.right, aliases)
        if isinstance(node, nodes.Or):
            return nonnull(node.left, aliases) & nonnull(node.right, aliases)
        if isinstance(node, nodes.Test) and node.name == "arista.avd.defined":
            resolved = path(node.node, aliases)
            return {resolved} if resolved else set()
        if isinstance(node, (nodes.Name, nodes.Getattr, nodes.Getitem)):
            resolved = path(node, aliases)
            return {resolved} if resolved else set()
        if isinstance(node, nodes.Not) and isinstance(node.node, nodes.Test) and node.node.name == "none":
            resolved = path(node.node.node, aliases)
            return {resolved} if resolved else set()
        return set()

    for file in sorted(template_root.rglob("*.j2")):
        source = file.read_text()
        lines = source.splitlines()
        try:
            tree = environment.parse(source)
        except Exception as exc:
            limitations.append({"file": str(file.relative_to(repo)), "issue": str(exc)})
            continue

        def statement(line):
            start = line - 1
            end = start
            while end < len(lines) - 1 and "%}" not in lines[end]:
                end += 1
            return "\n".join(lines[start : end + 1]).strip()

        def record(node, aliases, guards, kind, protected):
            for resolved in set(paths(node, aliases)):
                index[resolved].append(
                    {
                        "file": str(file.relative_to(repo)),
                        "line": node.lineno,
                        "kind": kind,
                        "expression": lines[node.lineno - 1].strip(),
                        "guards": list(guards),
                        "exact_null_rejecting_guard": resolved in protected,
                    }
                )

        def walk(body, aliases, guards, protected=frozenset()):
            aliases = aliases.copy()
            for node in body:
                if isinstance(node, nodes.For):
                    record(node.iter, aliases, guards, "iteration", protected)
                    loop_aliases = aliases.copy()
                    resolved = path(node.iter, aliases)
                    if isinstance(node.target, nodes.Name):
                        loop_aliases[node.target.name] = resolved + "[]" if resolved else None
                    if node.test:
                        record(node.test, loop_aliases, guards, "loop-test", protected)
                    walk(node.body, loop_aliases, guards, protected | (nonnull(node.test, loop_aliases) if node.test else set()))
                    walk(node.else_, aliases, guards, protected)
                elif isinstance(node, nodes.Assign):
                    record(node.node, aliases, guards, "assignment", protected)
                    if isinstance(node.target, nodes.Name):
                        aliases[node.target.name] = path(node.node, aliases)
                elif isinstance(node, nodes.If):
                    record(node.test, aliases, guards, "condition", protected)
                    condition = statement(node.lineno)
                    walk(node.body, aliases, (*guards, condition), protected | nonnull(node.test, aliases))
                    walk(node.elif_, aliases, (*guards, "NOT " + condition), protected)
                    walk(node.else_, aliases, (*guards, "NOT " + condition), protected)
                elif isinstance(node, nodes.Output):
                    for expr in node.nodes:
                        if not isinstance(expr, nodes.TemplateData):
                            record(expr, aliases, guards, "output", protected)
                elif isinstance(node, (nodes.Macro, nodes.Include, nodes.Import, nodes.FromImport)):
                    limitations.append({"file": str(file.relative_to(repo)), "line": node.lineno, "issue": type(node).__name__ + " needs cross-context review"})
                    if isinstance(node, nodes.Macro):
                        walk(node.body, {**aliases, **{arg.name: None for arg in node.args}}, guards, protected)
                else:
                    record(node, aliases, guards, "expression", protected)

        walk(tree.body, {}, ())
    return index, limitations


def python_index(repo: Path) -> dict:
    """Index attribute tokens outside generated models for manual consumer tracing."""
    index = defaultdict(list)
    root = repo / "python-avd/pyavd/_eos_designs"
    for file in sorted(root.rglob("*.py")):
        if "schema" in file.relative_to(root).parts:
            continue
        lines = file.read_text().splitlines()
        tree = ast.parse("\n".join(lines))
        for node in ast.walk(tree):
            if isinstance(node, ast.Attribute) and isinstance(node.ctx, ast.Load):
                index[node.attr].append(
                    {"file": str(file.relative_to(repo)), "line": node.lineno, "expression": lines[node.lineno - 1].strip(), "access": ast.unparse(node)}
                )
    return index


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--avd", type=Path, default=Path("/home/holbech/repos/avd"))
    args = parser.parse_args()
    directory = Path(__file__).parent
    inventory = json.loads((directory / "inventory.json").read_text())
    templates, limitations = template_index(args.avd)
    python = python_index(args.avd)
    for group in inventory["metadata"]["groups"]:
        candidates = []
        for path in group.get("eos_config_paths", []):
            candidates.extend({"path": path, **entry} for entry in templates.get(path, []))
        # Deduplicate repeated schema paths pointing to the same template site.
        group["template_candidates"] = list({json.dumps(entry, sort_keys=True): entry for entry in candidates}.values())
        if group["origin"].startswith("eos_designs#"):
            field = group["origin"].split("/")[-1]
            group["python_candidates"] = python.get(field, [])
    inventory["metadata"]["template_analysis_limitations"] = limitations
    (directory / "inventory.json").write_text(json.dumps(inventory, indent=2) + "\n")
    print(  # noqa: T201  # CLI summary for the consumer-index invocation.
        json.dumps(
            {
                "groups_with_template_candidates": sum(bool(g["template_candidates"]) for g in inventory["metadata"]["groups"]),
                "template_limitations": len(limitations),
                "groups_by_schema": dict(Counter(g["origin"].split("#")[0] for g in inventory["metadata"]["groups"])),
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
