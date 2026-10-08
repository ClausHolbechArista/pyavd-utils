# Copyright (c) 2026 Arista Networks, Inc.
# ruff: noqa: INP001  # Standalone diagnostic data, not a distributable package.

"""
Synthetic EOS output cases linked to inspected AVD Design handoffs.

These cases exercise stripping and public output validation, not full design
generation. Labels identify source declarations whose existing consumer evidence
establishes the handoff. An accepted/rejected case is not a declaration-wide
compatibility verdict: activation, parent pruning and overrides still matter.
"""

from __future__ import annotations

from copy import deepcopy


def output_cases() -> dict[str, dict]:
    """Return fresh probe inputs and their explicitly reviewed source associations."""
    cases = {
        "static_route_prefix_none": {
            "labels": ["$defs.node_type_l3_interfaces.static_routes.prefix", "$defs.static_routes.prefix"],
            "target_path": "static_routes[].prefix",
            "target_required": True,
            "input": {"static_routes": [{"prefix": None, "next_hop": "192.0.2.1"}]},
        },
        "ipv4_prefix_list_action_none": {
            "labels": ["ipv4_prefix_list_catalog.sequence_numbers.action"],
            "target_path": "prefix_lists[].sequence_numbers[].action",
            "target_required": True,
            "input": {"prefix_lists": [{"name": "PL", "sequence_numbers": [{"sequence": 10, "action": None}]}]},
        },
        "ipv6_prefix_list_action_none": {
            "labels": ["ipv6_prefix_list_catalog.sequence_numbers.action"],
            "target_path": "ipv6_prefix_lists[].sequence_numbers[].action",
            "target_required": True,
            "input": {"ipv6_prefix_lists": [{"name": "PL", "sequence_numbers": [{"sequence": 10, "action": None}]}]},
        },
        "vlan_internal_order_all_null": {
            "labels": ["internal_vlan_order.allocation", "internal_vlan_order.range.beginning", "internal_vlan_order.range.ending"],
            "target_path": "vlan_internal_order",
            "target_required": False,
            "input": {"vlan_internal_order": {"allocation": None, "range": {"beginning": None, "ending": None}}},
        },
        "dot1x_username_format_all_null": {
            "labels": [
                "dot1x_settings.mac_based_authentication.username_format.delimiter",
                "dot1x_settings.mac_based_authentication.username_format.letter_case",
            ],
            "target_path": "dot1x.radius_av_pair_username_format",
            "target_required": False,
            "input": {"dot1x": {"radius_av_pair_username_format": {"delimiter": None, "mac_string_case": None}}},
        },
        "optional_static_routes_parent_null": {
            "labels": [],
            "target_path": "static_routes",
            "target_required": False,
            "input": {"static_routes": None},
        },
        "optional_prefix_lists_parent_null": {
            "labels": [],
            "target_path": "prefix_lists",
            "target_required": False,
            "input": {"prefix_lists": None},
        },
    }
    for field in ("ipv4_address", "mac_address"):
        item = {"ipv4_address": "192.0.2.10", "vrf": "default", "mac_address": "0011.2233.4455"}
        item[field] = None
        cases[f"arp_{field}_none"] = {
            "labels": [f"network_services.vrfs.static_arp_entries.{field}"],
            "target_path": f"arp.static_entries[].{field}",
            "target_required": True,
            "input": {"arp": {"static_entries": [item]}},
        }
    for field in ("allocation", "beginning", "ending"):
        item = {"allocation": "ascending", "range": {"beginning": 2, "ending": 4094}}
        if field == "allocation":
            item[field] = None
            suffix = field
        else:
            item["range"][field] = None
            suffix = f"range.{field}"
        cases[f"vlan_internal_order_{field}_none"] = {
            "labels": [f"internal_vlan_order.{suffix}"],
            "target_path": f"vlan_internal_order.{suffix}",
            "target_required": True,
            "input": {"vlan_internal_order": item},
        }
    for field, source_field in (("delimiter", "delimiter"), ("mac_string_case", "letter_case")):
        item = {"delimiter": "colon", "mac_string_case": "lowercase"}
        item[field] = None
        cases[f"dot1x_username_format_{field}_none"] = {
            "labels": [f"dot1x_settings.mac_based_authentication.username_format.{source_field}"],
            "target_path": f"dot1x.radius_av_pair_username_format.{field}",
            "target_required": True,
            "input": {"dot1x": {"radius_av_pair_username_format": item}},
        }
    for field in ("interval", "min_rx", "multiplier"):
        item = {"interval": 300, "min_rx": 300, "multiplier": 3}
        item[field] = None
        cases[f"bfd_multihop_{field}_none"] = {
            "labels": [f"bfd_multihop.{field}"],
            "target_path": f"router_bfd.multihop.{field}",
            "target_required": False,
            "input": {"router_bfd": {"multihop": item}},
        }
    repair = deepcopy(cases["vlan_internal_order_allocation_none"])
    repair["override"] = {"vlan_internal_order": {"allocation": "ascending"}}
    cases["vlan_internal_order_allocation_repaired_by_override"] = repair
    return cases


def evaluation_for(group: dict, observations: dict, declaration_review: dict) -> dict:
    """Attach observed output outcomes without claiming whole-program safety."""
    label = ".".join(part for part in group["origin"].split("#/")[1].split("/") if part not in {"keys", "items"})
    findings = []
    for case_id, case in output_cases().items():
        if label not in case["labels"] or not group["origin"].startswith("eos_designs#"):
            continue
        result = observations[case_id]
        findings.append(
            {
                "case_id": case_id,
                "target_path": case["target_path"],
                "target_required": case["target_required"],
                "direct_null_accepted": result["direct_validation"]["accepted"],
                "after_strip_accepted": result["after_strip_validation"]["accepted"],
                "after_strip_outcome": "accepted" if result["after_strip_validation"]["accepted"] else "rejected-by-output-validation",
                "override_applied": "override" in case,
                "final_output_accepted": result["final_validation"]["accepted"],
                "evidence": f"probe-results.json#/output_validation/{case_id}",
            }
        )
    return {
        **declaration_review,
        "output_boundary_review": "probed-output-boundary" if findings else "static-analysis",
        "output_findings": findings,
    }
