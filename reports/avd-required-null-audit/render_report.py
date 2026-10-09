# Copyright (c) 2026 Arista Networks, Inc.
# ruff: noqa: INP001  # Standalone report renderer stored beside the generated artifacts.

"""
Apply reviewed findings and render a per-declaration review ledger.

Automatic template matches remain explicitly labeled candidates. A missing match
is not evidence of safety. The full occurrence dataset retains defaults, relaxed
boundaries and reference chains; a source declaration is not a validation phase.
"""

from __future__ import annotations

import json
from collections import Counter, defaultdict
from pathlib import Path

from output_reviews import evaluation_for, output_cases
from reviews import review_config, review_design, source_label


def occurrence_scope(row: dict) -> str:
    """
    Separate modeled structured-config overrides from ordinary design inputs.

    Every relaxed occurrence in this snapshot must have an explicit override
    boundary. Check both naming and recorded reference boundaries so a future
    relaxed profile or differently named override cannot silently change scope.
    """
    named_override = any(part == "structured_config" or part.endswith("_structured_config") for part in row["path"].split("."))
    boundaries = row["relaxed_boundaries"]
    if named_override != bool(boundaries):
        msg = f"Override naming and relaxed boundaries disagree at {row['path']}; review scope before regenerating."
        raise ValueError(msg)
    for boundary in boundaries:
        name = boundary.split(".")[-1]
        if name != "structured_config" and not name.endswith("_structured_config"):
            msg = f"Unclassified relaxed boundary {boundary}; review scope before regenerating."
            raise ValueError(msg)
    return "structured_config" if boundaries else "avd_design"


def scope_summary(rows: list[dict], groups: list[dict], scope: str) -> dict:
    """Count occurrences separately from distinct source declarations."""
    origins = {row["required_origin"] for row in rows}
    selected = [group for group in groups if group["origin"] in origins]
    return {
        "occurrences": len(rows),
        "source_groups": len(selected),
        "scalar_occurrences": sum(row["type"] not in {"list", "dict"} for row in rows),
        "collection_occurrences": sum(row["type"] in {"list", "dict"} for row in rows),
        "scalar_groups": sum(group["type"] not in {"list", "dict"} for group in selected),
        "collection_groups": sum(group["type"] in {"list", "dict"} for group in selected),
        "groups_by_source": dict(Counter(group["origin"].split("#")[0] for group in selected)),
        "types": dict(Counter(group["type"] for group in selected)),
        "review_counts": dict(Counter(group["review_status"] for group in selected)),
        "classification_counts": dict(Counter(group["classification"] for group in selected)),
        "assessment_counts": dict(Counter(group["avd_design_evaluation"]["assessment_status"] for group in selected if "avd_design_evaluation" in group))
        if scope == "avd_design"
        else {},
        "compatibility_verdict_counts": dict(
            Counter(group["avd_design_evaluation"]["compatibility_verdict"] for group in selected if "avd_design_evaluation" in group)
        )
        if scope == "avd_design"
        else {},
        "output_boundary_review_counts": dict(
            Counter(group["avd_design_evaluation"]["output_boundary_review"] for group in selected if "avd_design_evaluation" in group)
        )
        if scope == "avd_design"
        else {},
    }


def candidate_review(group: dict) -> dict:
    """Summarize local template evidence, without a whole-program claim."""
    candidates = group["template_candidates"]
    reads = [entry for entry in candidates if entry["kind"] in {"output", "assignment", "expression"}]
    unguarded = [entry for entry in reads if not entry["exact_null_rejecting_guard"]]
    if not candidates:
        classification = "unresolved"
        impact = "No local template binding established. This is not evidence of an unused field or null safety."
        evidence = []
    elif group["type"] in {"list", "dict"}:
        classification = "template-collection-policy"
        impact = "Collection is read by templates. Empty typed models and serialized null differ; scalar-only strictness does not address this field."
        evidence = candidates[:2]
    elif unguarded:
        classification = "candidate-unguarded-read"
        impact = (
            "Indexed scalar read has no recognized exact-field null guard. This is a template-site candidate, not proof of ordinary-design "
            "failure: trace pre-strip consumption, target requiredness, stripping and output-validation gating before assigning compatibility."
        )
        evidence = unguarded[:2]
    elif reads:
        classification = "candidate-null-guard"
        impact = (
            "Indexed reads have exact-field null-rejecting guards. The template can suppress them, but a stripped required output child may "
            "already fail validation. Trace the design handoff and surviving parent before classifying this as successful omission."
        )
        evidence = reads[:2]
    else:
        classification = "candidate-condition"
        impact = (
            "Indexed field is used in conditions rather than direct output. Null can suppress a feature or select an alternate branch; "
            "inspect the retained expressions."
        )
        evidence = candidates[:2]
    return {
        "classification": classification,
        "review_status": "candidate-only",
        "impact": impact,
        "evidence": [{"file": entry["file"], "line": entry["line"]} for entry in evidence],
    }


def main() -> None:
    directory = Path(__file__).parent
    inventory = json.loads((directory / "inventory.json").read_text())
    probes = json.loads((directory / "probe-results.json").read_text())
    declaration_reviews = json.loads((directory / "declaration_reviews.json").read_text())
    output_observations = probes["output_validation"]
    if set(output_observations) != set(output_cases()):
        msg = "Output probe cases changed; rerun probe.py and refresh probe-results.json before rendering."
        raise ValueError(msg)
    for case_id, case in output_cases().items():
        result = output_observations[case_id]
        if result["input"] != case["input"] or result["override"] != case.get("override"):
            msg = f"Output probe input changed for {case_id}; refresh probe-results.json before rendering."
            raise ValueError(msg)
    metadata = inventory["metadata"]
    rows_by_origin = defaultdict(list)
    for row in inventory["fields"]:
        row["scope"] = occurrence_scope(row)
        rows_by_origin[row["required_origin"]].append(row)
    groups = metadata["groups"]
    for index, group in enumerate(groups, 1):
        group["id"] = f"R{index:03}"
        review = review_design(group) if group["origin"].startswith("eos_designs#") else review_config(group)
        group.update(review or candidate_review(group))
        rows = rows_by_origin[group["origin"]]
        group["relaxed_occurrences"] = sum(bool(row["relaxed_boundaries"]) for row in rows)
        group["default_occurrences"] = sum(row["has_default"] for row in rows)
        group["structured_config_occurrences"] = sum(row["scope"] == "structured_config" for row in rows)
        group["avd_design_occurrences"] = sum(row["scope"] == "avd_design" for row in rows)
        group["literal_structured_config_occurrences"] = sum(".structured_config." in row["path"] for row in rows)
        group["scopes"] = sorted({row["scope"] for row in rows})
        if "avd_design" in group["scopes"]:
            group["avd_design_evaluation"] = evaluation_for(group, output_observations, declaration_reviews["reviews"][group["origin"]])
        else:
            group.pop("avd_design_evaluation", None)
        for row in rows:
            row["classification"] = group["classification"]
            row["review_id"] = group["id"]
            row["review_status"] = group["review_status"]
            # Consumer evidence belongs to the shared declaration record, not
            # duplicated across thousands of reference use-sites.
            row.pop("evidence", None)
            row.pop("impact", None)
    primary_labels = {source_label(group["origin"]) for group in groups if "avd_design" in group["scopes"] and group["origin"].startswith("eos_designs#")}
    linked_labels = {label for case in output_cases().values() for label in case["labels"]}
    primary_origins = {group["origin"] for group in groups if "avd_design" in group["scopes"]}
    if set(declaration_reviews["reviews"]) != primary_origins:
        msg = "Declaration review inventory changed; reconcile declaration_reviews.json before rendering."
        raise ValueError(msg)
    metadata["enforcement_policy"] = declaration_reviews["policy"]
    if linked_labels - primary_labels:
        msg = f"Output cases refer to unknown primary declarations: {sorted(linked_labels - primary_labels)}"
        raise ValueError(msg)
    metadata["output_validation_context"] = probes["output_validation_context"]
    metadata["review_counts"] = dict(Counter(group["review_status"] for group in groups))
    metadata["classification_counts"] = dict(Counter(group["classification"] for group in groups))
    metadata["scalar_groups"] = sum(group["type"] not in {"list", "dict"} for group in groups)
    metadata["relaxed_occurrences"] = sum(bool(row["relaxed_boundaries"]) for row in inventory["fields"])
    metadata["structured_config_occurrences"] = sum(row["scope"] == "structured_config" for row in inventory["fields"])
    metadata["literal_structured_config_occurrences"] = sum(".structured_config." in row["path"] for row in inventory["fields"])
    metadata["scopes"] = {
        scope: scope_summary([row for row in inventory["fields"] if row["scope"] == scope], groups, scope) for scope in ("avd_design", "structured_config")
    }
    metadata["shared_source_groups"] = sum(len(group["scopes"]) == 2 for group in groups)
    metadata["structured_config_only_source_groups"] = sum(group["scopes"] == ["structured_config"] for group in groups)
    metadata["structured_config_boundaries"] = dict(
        Counter(row["relaxed_boundaries"][0].split(".")[-1] for row in inventory["fields"] if row["scope"] == "structured_config")
    )
    (directory / "inventory.json").write_text(json.dumps(inventory, indent=2) + "\n")

    def link(file: str, line: int | None = None) -> str:
        target = Path(metadata["avd_checkout"]) / file
        return f"[{Path(file).name}{':' + str(line) if line else ''}]({target}{':' + str(line) if line else ''})"

    def render_ledger(scope: str, *, candidates_only: bool = False) -> str:
        summary = metadata["scopes"][scope]
        selected = [
            group
            for group in groups
            if scope in group["scopes"]
            and (
                not candidates_only
                or group["avd_design_evaluation"]["assessment_status"] != "reviewed"
                or group["avd_design_evaluation"]["compatibility_verdict"] == "unresolved"
            )
        ]
        title = (
            "Open AVD Design consumer traces"
            if candidates_only
            else "AVD Design required fields"
            if scope == "avd_design"
            else "Structured-config override appendix"
        )
        lines = [
            f"# {title}",
            "",
            f"Snapshot: `{metadata['avd_revision']}` in `{metadata['avd_checkout']}`.",
            "",
            (
                f"This list contains {len(selected)} distinct required non-primary declarations. The full {scope} scope has "
                f"{summary['occurrences']} occurrences, {summary['scalar_groups']} scalar declarations and "
                f"{summary['collection_groups']} collection declarations."
            ),
            "",
            (
                "[Decision report](report.md) · [Full inventory](inventory.json) · [Design fields](fields.md) · "
                "[Open design traces](open-fields.md) · [Override appendix](structured-config-fields.md)"
            ),
            "",
            (
                "Only paths belonging to this scope are listed. A declaration can appear in both ledgers; "
                "override occurrences are never added to the design counts."
            ),
            "",
            (
                "The source/template index remains separate from the declaration assessment. `inspected-site` is local evidence, not transitive safety. "
                "`demonstrated-method` uses an isolated real-method probe; `demonstrated-fragment` uses a template fragment. "
                "`candidate-only` describes the original automated index, not the progress of the subsequent static assessment. "
                "A default does not replace explicit scalar null. Full candidate expressions and reference chains remain in the inventory."
            ),
            "",
            (
                "Classifications describe cited consumer sites, not compatibility verdicts. A template null guard does not establish "
                "successful omission: stripped required children can fail EOS Config validation. An optional parent can instead be pruned, "
                "and later overrides can repair the output. The separate ordinary-design assessment records the stricter-rule impact, "
                "not customer prevalence or EOS device acceptance."
            ),
            "",
            (
                "`reviewed` means the declaration's consumer families and occurrence routing were statically assessed, not that every input "
                "combination was tested. Accepted contexts are code/schema scenarios unless a saved observation is identified. "
                "`active-path-already-rejected` is an earlier-error candidate for the inspected regular active path, not universally safe rejection. "
                "`mixed-rejection` includes both failing and permitted contexts; `behavior-changing-rejection` identifies a permitted null path. "
                "Later overrides may repair otherwise invalid generated output."
                if scope == "avd_design"
                else "Ordinary-design assessments do not apply to override occurrences."
            ),
            "",
        ]
        if candidates_only:
            lines.extend(
                [
                    (
                        "These entries have incomplete declaration assessments or an unresolved compatibility verdict. "
                        "Confirmed partial findings are preserved; template sites alone do not establish the final workflow outcome."
                    ),
                    "",
                ]
            )
        for collection, section in ((False, "Scalars"), (True, "Collections — retained as a separate policy")):
            subset = [group for group in selected if (group["type"] in {"list", "dict"}) == collection]
            lines.extend([f"## {section} ({len(subset)} declarations)", ""])
            for group in subset:
                rows = [row for row in rows_by_origin[group["origin"]] if row["scope"] == scope]
                other_count = len(rows_by_origin[group["origin"]]) - len(rows)
                evidence = ", ".join(link(entry["file"], entry["line"]) for entry in group["evidence"])
                lines.extend(
                    [
                        f"### {group['id']} — `{source_label(group['origin'])}`",
                        "",
                        (
                            f"Type: `{group['type']}`. Scope occurrences: {len(rows)}. Other-scope occurrences excluded: {other_count}. "
                            + (
                                f"Initial consumer index: `{group['review_status']}` / `{group['classification']}`. "
                                "Use the declaration assessment below for the current finding."
                                if scope == "avd_design"
                                else f"Status: `{group['review_status']}` / `{group['classification']}`."
                            )
                        ),
                        "",
                        f"Source: {link(group['source_file'])}; declaration `{group['origin']}`.",
                        "",
                    ]
                )
                if scope != "avd_design":
                    lines.extend([group["impact"], "", f"Evidence: {evidence or 'Open — no verified consumer evidence.'}", ""])
                if scope == "avd_design":
                    evaluation = group["avd_design_evaluation"]
                    lines.extend(
                        [
                            (
                                f"Assessment: `{evaluation['assessment_status']}`. Compatibility verdict: `{evaluation['compatibility_verdict']}`. "
                                f"Output boundary: `{evaluation['output_boundary_review']}`."
                            ),
                            "",
                            f"Pattern: `{evaluation['risk_pattern']}`.",
                            "",
                            f"Stricter-rule finding: {evaluation['active_outcome']}",
                            "",
                            "Permitted contexts from static analysis / saved observations: "
                            + (
                                " ".join(evaluation["accepted_contexts"])
                                or "None established for the inspected regular active path; this is not a universal safety proof."
                            ),
                            "",
                            f"Migration/impact: {evaluation['migration']}",
                            "",
                            "Assessment evidence: " + (", ".join(link(entry["file"], entry.get("line")) for entry in evaluation["evidence"]) or "Pending."),
                            "",
                        ]
                    )
                    for observation in evaluation.get("prior_confirmed_observations", []):
                        lines.extend([f"Previously confirmed observation: {observation}", ""])
                    for case_id in evaluation.get("saved_method_observations", []):
                        lines.extend([f"Saved method observation: [{case_id}](probe-results.json#/methods/{case_id}).", ""])
                    for finding in evaluation["output_findings"]:
                        outcome = "accepted" if finding["after_strip_accepted"] else "rejected"
                        final = "accepted" if finding["final_output_accepted"] else "rejected"
                        lines.extend(
                            [
                                (
                                    f"- Output probe `{finding['case_id']}` → `{finding['target_path']}` "
                                    f"(target required: {finding['target_required']}): after stripping **{outcome}**; "
                                    f"override applied: {finding['override_applied']}; final output **{final}**. "
                                    f"[Observation](probe-results.json#/output_validation/{finding['case_id']})."
                                ),
                            ]
                        )
                    if evaluation["output_findings"]:
                        lines.extend(
                            [
                                "",
                                "Synthetic output-boundary probes support the inspected handoff, not an end-to-end design run or a safe-rejection verdict.",
                                "",
                            ]
                        )
                lines.extend(["Schema occurrence paths:", ""])
                lines.extend(f"- `{row['path']}`" for row in rows)
                lines.append("")
        return "\n".join(lines) + "\n"

    (directory / "fields.md").write_text(render_ledger("avd_design"))
    (directory / "open-fields.md").write_text(render_ledger("avd_design", candidates_only=True))
    (directory / "structured-config-fields.md").write_text(render_ledger("structured_config"))
    print(  # noqa: T201  # CLI summary for the renderer invocation.
        json.dumps(
            {
                key: metadata[key]
                for key in (
                    "source_groups",
                    "scalar_groups",
                    "occurrences",
                    "relaxed_occurrences",
                    "structured_config_occurrences",
                    "review_counts",
                    "classification_counts",
                    "scopes",
                    "shared_source_groups",
                )
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
