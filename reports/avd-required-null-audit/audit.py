# Copyright (c) 2026 Arista Networks, Inc.
# ruff: noqa: INP001  # Standalone CLI scripts live beside report artifacts, not in an importable package.

"""
Inventory effective non-primary required AVD Design fields for a null-safety review.

This diagnostic uses AVD's existing reference resolver, and traverses keys, dynamic keys,
and items. It follows references regardless of documentation visibility. Definitions are
visited through their actual uses rather than counted as additional input occurrences.
Source markers survive reference merging so the report can group repeated use-sites without
losing the complete list of paths. Run with the AVD development environment.
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from collections import Counter
from copy import deepcopy
from pathlib import Path

import yaml
from deepmerge import always_merger


def mark(node: dict, schema_name: str, pointer: tuple[str, ...], file: Path, marks: dict) -> None:
    """Attach source locations to schema nodes before references are merged."""
    origin = schema_name + "#/" + "/".join(pointer)
    node["__audit_origin"] = origin
    if "required" in node:
        node["__audit_required_origin"] = origin
    if "$ref" in node:
        node["__audit_ref_origin"] = origin
    marks[origin] = str(file)
    for collection in ("keys", "dynamic_keys", "$defs"):
        for key, child in node.get(collection, {}).items():
            mark(child, schema_name, (*pointer, collection, key), file, marks)
    if "items" in node:
        mark(node["items"], schema_name, (*pointer, "items"), file, marks)


def collect(repo: Path) -> tuple[list[dict], dict]:
    """Resolve each visited node with the repository's own inheritance semantics."""
    sys.path[:0] = [str(repo / "python-avd")]
    from schema_tools.avdschemaresolver import AvdSchemaResolver  # noqa: PLC0415  # Add the selected AVD checkout to sys.path first.

    store = {}
    source_files = {}
    for name, directory in (
        ("eos_designs", "_eos_designs"),
        ("eos_cli_config_gen", "_eos_cli_config_gen"),
    ):
        schema = {}
        fragments = repo / "python-avd/pyavd" / directory / "schema/schema_fragments"
        for file in sorted(fragments.glob("*.yml")):
            fragment = yaml.safe_load(file.read_text())
            mark(fragment, name, (), file.relative_to(repo), source_files)
            always_merger.merge(schema, fragment)
        store[name] = schema
    store["avd_meta_schema"] = {}
    resolver = AvdSchemaResolver("eos_designs", store)
    rows = []
    counts = Counter()

    def visit(node: dict, path: str, refs: tuple[str, ...], primary_key: str | None, field_name: str | None, relaxed: tuple[str, ...], removed: bool) -> None:
        node = deepcopy(node)
        own_refs = []
        begins_relaxed = bool(node.get("$ref") and node.get("relaxed_validation"))
        while "$ref" in node:
            ref = node["$ref"]
            if ref in own_refs:
                raise ValueError(f"reference cycle at {path}: {ref}")  # noqa: TRY003, EM102  # Include the path and ref in this audit diagnostic.
            own_refs.append(ref)
            resolver._ref_on_child(node)
        refs = (*refs, *own_refs)
        if begins_relaxed:
            relaxed = (*relaxed, path)
        removed = removed or bool(node.get("deprecation", {}).get("removed"))
        counts["visited_nodes"] += 1
        if node.get("required") and field_name is not None:
            if removed:
                counts["removed_required_occurrences"] += 1
            elif field_name == primary_key:
                counts["excluded_primary_required_occurrences"] += 1
            else:
                origin = node["__audit_required_origin"]
                rows.append(
                    {
                        "path": path,
                        "type": node["type"],
                        "required_origin": origin,
                        "source_file": source_files[origin],
                        "reference_chain": list(refs),
                        "relaxed_boundaries": list(relaxed),
                        "has_default": "default" in node,
                        "default": node.get("default"),
                        "valid_values": node.get("valid_values"),
                        "description": node.get("description"),
                        "classification": "unreviewed",
                        "evidence": [],
                        "impact": None,
                    }
                )
        for key, child in node.get("keys", {}).items():
            visit(child, f"{path}.{key}", refs, primary_key if field_name is None else None, key, relaxed, removed)
        for key, child in node.get("dynamic_keys", {}).items():
            visit(child, f"{path}.<dynamic:{key}>", refs, None, key, relaxed, removed)
        if "items" in node:
            visit(node["items"], f"{path}[]", refs, node.get("primary_key"), None, relaxed, removed)

    visit(store["eos_designs"], "avd_design", (), None, None, (), removed=False)
    design_counts = dict(counts)
    design_rows = rows
    rows = []
    visit(store["eos_cli_config_gen"], "eos_config", (), None, None, (), removed=False)
    config_rows = rows
    rows = design_rows
    groups = {}
    for row in rows:
        key = row["required_origin"]
        group = groups.setdefault(
            key,
            {
                "origin": key,
                "type": row["type"],
                "source_file": row["source_file"],
                "paths": [],
                "classification": "unreviewed",
                "evidence": [],
                "impact": None,
            },
        )
        group["paths"].append(row["path"])
    for row in config_rows:
        if row["required_origin"] in groups:
            groups[row["required_origin"]].setdefault("eos_config_paths", []).append(row["path"])
    metadata = {
        "avd_checkout": str(repo),
        "avd_revision": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=repo, text=True).strip(),  # noqa: S607  # Resolve git from the selected checkout's environment.
        "source": "current YAML fragments, combined in build_schemas order",
        "counts": design_counts,
        "occurrences": len(rows),
        "source_groups": len(groups),
        "types": dict(Counter(row["type"] for row in rows)),
        "groups": list(groups.values()),
    }
    return rows, metadata


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--avd", type=Path, default=Path("/home/holbech/repos/avd"))
    parser.add_argument("--output", type=Path, default=Path(__file__).parent)
    args = parser.parse_args()
    rows, metadata = collect(args.avd.resolve())
    args.output.mkdir(parents=True, exist_ok=True)
    (args.output / "inventory.json").write_text(json.dumps({"metadata": metadata, "fields": rows}, indent=2) + "\n")
    print(json.dumps({key: value for key, value in metadata.items() if key != "groups"}, indent=2))  # noqa: T201  # CLI summary for the audit invocation.


if __name__ == "__main__":
    main()
