# ruff: noqa: INP001, CPY001 -- standalone report script, not a package or distributable module.

"""
Run read-only required-null diagnostics against an AVD checkout.

Invoke with the AVD virtualenv, for example::

    PYTHONDONTWRITEBYTECODE=1 /home/holbech/repos/avd/.venv/bin/python \
        reports/avd-required-null-audit/probe.py --avd /home/holbech/repos/avd

Template probes render real source fragments with synthetic dictionaries. Mixin probes call
real methods with real generated AVD models and small fake contexts, not the full design workflow.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import sys
import tempfile
from pathlib import Path
from types import SimpleNamespace
from typing import TYPE_CHECKING, Any

if TYPE_CHECKING:
    from collections.abc import Callable


def observe(function: Callable[[], Any]) -> dict[str, Any]:
    """Return the observed value or exception without asserting expected behavior."""
    try:
        return {"value": function()}
    except Exception as exc:
        return {"exception": {"type": type(exc).__name__, "message": str(exc)}}


def inventory_observations(report_dir: Path, avd_repo: Path) -> dict[str, Any]:
    """Compare the checked-in report data with an in-memory traversal of current YAML."""
    sys.path.insert(0, str(report_dir))
    from audit import collect  # noqa: PLC0415

    rows, metadata = collect(avd_repo)
    inventory = json.loads((report_dir / "inventory.json").read_text())
    field_core_keys = {
        "path",
        "type",
        "required_origin",
        "source_file",
        "reference_chain",
        "relaxed_boundaries",
        "has_default",
        "default",
        "valid_values",
        "description",
    }
    group_core_keys = {"origin", "type", "source_file", "paths", "eos_config_paths"}
    current_fields = [{key: value for key, value in row.items() if key in field_core_keys} for row in rows]
    artifact_fields = [{key: value for key, value in row.items() if key in field_core_keys} for row in inventory["fields"]]
    expected_groups = {group["origin"]: {key: value for key, value in group.items() if key in group_core_keys} for group in inventory["metadata"]["groups"]}
    actual_groups = {group["origin"]: {key: value for key, value in group.items() if key in group_core_keys} for group in metadata["groups"]}
    row_paths = {row["path"] for row in inventory["fields"]}
    return {
        "avd_revision": metadata["avd_revision"],
        "field_structure_matches_current_yaml": current_fields == artifact_fields,
        "group_structure_matches_current_yaml": actual_groups == expected_groups,
        "review_annotated_field_rows": sum("review_id" in row for row in inventory["fields"]),
        "review_annotated_source_groups": sum("review_status" in group for group in inventory["metadata"]["groups"]),
        "field_occurrences": len(rows),
        "source_groups": len(actual_groups),
        "resolved_reference_rows": sum(bool(row["reference_chain"]) for row in rows),
        "maximum_reference_chain_depth": max(len(row["reference_chain"]) for row in rows),
        "unresolved_references_or_cycles": "none; traversal completed",
        "removed_required_occurrences_excluded": metadata["counts"].get("removed_required_occurrences", 0),
        "direct_primary_required_occurrences_excluded": metadata["counts"].get("excluded_primary_required_occurrences", 0),
        "nested_name_below_name_primary_key_included": ("avd_design.application_classification.categories[].applications[].name" in row_paths),
        "required_direct_primary_key_excluded": ("avd_design.network_services[].vrfs[].l3_port_channels[].member_interfaces[].name" not in row_paths),
    }


def template_observations(avd_repo: Path) -> dict[str, Any]:
    """Render existing source Jinja fragments with specific None inputs."""
    sys.path.insert(0, str(avd_repo / "python-avd"))
    from pyavd.constants import EOS_CLI_CONFIG_GEN_JINJA2_TEMPLATE_PATH  # noqa: PLC0415
    from pyavd.templater import Templar  # noqa: PLC0415

    template_inputs = {
        "agents_environment_value_none": (
            "eos/agents-environment.j2",
            {"agents": [{"name": "agent-x", "environment_variables": [{"name": "TOKEN", "value": None}]}]},
        ),
        "vlan_internal_order_required_scalars_none": (
            "eos/vlan-internal-order.j2",
            {"vlan_internal_order": {"allocation": None, "range": {"beginning": None, "ending": None}}},
        ),
        "ethernet_required_child_none": (
            "eos/ethernet-interfaces.j2",
            {"ethernet_interfaces": [{"name": "Ethernet1", "link_tracking_groups": [{"name": "UPSTREAM", "direction": None}]}]},
        ),
        "ipv4_prefix_list_required_action_none": (
            "eos/prefix-lists.j2",
            {"prefix_lists": [{"name": "PL-TEST", "sequence_numbers": [{"sequence": 10, "action": None}]}]},
        ),
        "ipv6_prefix_list_required_action_none": (
            "eos/ipv6-prefix-lists.j2",
            {"ipv6_prefix_lists": [{"name": "PL-TEST", "sequence_numbers": [{"sequence": 10, "action": None}]}]},
        ),
        "static_route_required_prefix_none": (
            "eos/static-routes.j2",
            {"static_routes": [{"prefix": None, "next_hop": "192.0.2.1"}]},
        ),
        "dot1x_required_enabled_none": ("eos/dot1x.j2", {"dot1x": {"captive_portal": {"enabled": None}}}),
    }
    with tempfile.TemporaryDirectory(prefix="avd-required-null-audit-") as temporary_dir:
        templar = Templar(temporary_dir, [EOS_CLI_CONFIG_GEN_JINJA2_TEMPLATE_PATH])
        return {
            name: observe(lambda template=template, data=data: templar.render_template_from_file(template, data))
            for name, (template, data) in template_inputs.items()
        }


def model_observations() -> dict[str, Any]:
    """Check generated-model scalar None, collection null, and inheritance behavior."""
    from pyavd._eos_cli_config_gen.schema import EosCliConfigGen  # noqa: PLC0415

    scalar_null = EosCliConfigGen._from_dict({"vlan_internal_order": {"allocation": None, "range": {"beginning": None, "ending": None}}})
    null_collection = EosCliConfigGen._from_dict({"agents": None})
    explicit_null_child = EosCliConfigGen._from_dict({"vlan_internal_order": None})
    parent = EosCliConfigGen._from_dict({"vlan_internal_order": {"range": {"beginning": 100, "ending": 200}}})
    null_layer = EosCliConfigGen._from_dict({"vlan_internal_order": None})
    later_layer = EosCliConfigGen._from_dict({"vlan_internal_order": {"allocation": "ascending"}})
    parent._deepinherit(null_layer)
    after_null_layer = parent._dump()
    parent._deepinherit(later_layer)
    return {
        "scalar_null_is_preserved": scalar_null.vlan_internal_order.allocation is None,
        "scalar_null_dump": scalar_null._dump(),
        "null_collection_has_null_origin": null_collection.agents._created_from_null,
        "null_collection_dump": null_collection._dump(),
        "explicit_null_child_dump": explicit_null_child._dump(),
        "intermediate_null_sets_inheritance_block": parent.vlan_internal_order._block_inheritance,
        "after_null_layer": after_null_layer,
        "after_later_layer": parent._dump(),
    }


def output_validation_observations() -> dict[str, Any]:
    """
    Observe null, stripped output and optional override repair using public validation.

    These synthetic outputs isolate the final model boundary. They do not execute
    design generation or prove that every source occurrence reaches this boundary.
    """
    from output_reviews import output_cases  # noqa: PLC0415
    from pyavd._eos_cli_config_gen.schema import EosCliConfigGen  # noqa: PLC0415
    from pyavd.validate_structured_config import validate_structured_config  # noqa: PLC0415

    def validate(data: dict) -> dict:
        result = validate_structured_config(data)
        return {
            "accepted": result.validated_data is not None,
            "validated_data": result.validated_data,
            "violations": [{"path": violation.path, "message": violation.message} for violation in result.validation_result.violations],
        }

    observations = {}
    for case_id, case in output_cases().items():
        model = EosCliConfigGen._from_dict(case["input"])
        direct = validate(model._dump())
        model._strip_empties()
        stripped = model._dump()
        stripped_validation = validate(stripped)
        if "override" in case:
            model._deepmerge(EosCliConfigGen._from_dict(case["override"]))
        final = model._dump()
        observations[case_id] = {
            "input": case["input"],
            "direct_validation": direct,
            "after_strip": stripped,
            "after_strip_validation": stripped_validation,
            "override": case.get("override"),
            "final_output": final,
            "final_validation": validate(final),
        }
    return observations


def output_validation_context() -> dict[str, Any]:
    """Record the runtime schema archive and defaults used by public output validation."""
    from pyavd.constants import SCHEMA_STORE_ARCHIVE_FILE  # noqa: PLC0415

    from pyavd_utils.validation import Configuration  # noqa: PLC0415

    configuration = Configuration()
    with SCHEMA_STORE_ARCHIVE_FILE.open("rb") as archive:
        digest = hashlib.file_digest(archive, "sha256").hexdigest()
    return {
        "api": "pyavd.validate_structured_config",
        "schema_name": "eos_config",
        "schema_archive": str(SCHEMA_STORE_ARCHIVE_FILE),
        "schema_archive_sha256": digest,
        "restrict_null_values": configuration.restrict_null_values,
        "ignore_required_keys_on_root_dict": configuration.ignore_required_keys_on_root_dict,
        "scope": "Synthetic final EOS models, stripping, optional override merge and public validation; not full design generation.",
    }


def method_observations() -> dict[str, Any]:
    """Call actual mixin/helper code using generated models and minimal fake context."""
    from pyavd._eos_cli_config_gen.schema import EosCliConfigGen  # noqa: PLC0415
    from pyavd._eos_designs.schema import EosDesigns  # noqa: PLC0415
    from pyavd._eos_designs.shared_utils.cv_topology import CvTopology  # noqa: PLC0415
    from pyavd._eos_designs.shared_utils.filtered_tenants import FilteredTenantsMixin  # noqa: PLC0415
    from pyavd._eos_designs.structured_config.mlag import AvdStructuredConfigMlag  # noqa: PLC0415
    from pyavd._eos_designs.structured_config.network_services.router_bgp import RouterBgpMixin  # noqa: PLC0415
    from pyavd._eos_designs.structured_config.network_services.utils import UtilsMixin  # noqa: PLC0415
    from pyavd._eos_designs.structured_config.overlay.router_path_selection import RouterPathSelectionMixin  # noqa: PLC0415

    class CvTopologyProbe:
        cv_topology = CvTopology.cv_topology
        cv_topology_config = CvTopology.cv_topology_config
        get_cv_topology_level = CvTopology.get_cv_topology_level

        def __init__(self, inputs: EosDesigns, hostname: str, node_type: str) -> None:
            self.inputs = inputs
            self.hostname = hostname
            self.type = node_type
            self.node_config = EosDesigns._DynamicKeys.DynamicNodeTypesItem.NodeTypes.NodesItem()
            self.mlag = False
            self.mlag_peer = None

        def get_peer_facts(self, hostname: str, required: bool = False) -> SimpleNamespace:  # noqa: ARG002 -- canned peer fact is independent of requested hostname.
            return SimpleNamespace(type="spine")

    cv_probe = CvTopologyProbe(EosDesigns._from_dict({"use_cv_topology": False}), hostname="", node_type="")

    cv_level_inputs = EosDesigns._from_dict(
        {
            "use_cv_topology": True,
            "cv_topology": [
                {
                    "hostname": "leaf-a",
                    "platform": "DCS-7050",
                    "interfaces": [{"name": "Ethernet1", "neighbor": "spine-a", "neighbor_interface": "Ethernet1"}],
                }
            ],
            "cv_topology_levels": [{"type": "leaf", "level": None}, {"type": "spine", "level": 10}],
        }
    )
    cv_level_probe = CvTopologyProbe(cv_level_inputs, hostname="leaf-a", node_type="leaf")

    bundle_inputs = EosDesigns._from_dict({"evpn_vlan_bundles": [{"name": "bundle-a", "id": None}]})
    tenant_inputs = EosDesigns._from_dict({"network_services": [{"name": "TENANT", "vlan_aware_bundle_number_base": 100}]})
    bundle = bundle_inputs.evpn_vlan_bundles["bundle-a"]
    tenant = tenant_inputs.network_services["TENANT"]
    evpn_probe = SimpleNamespace(shared_utils=SimpleNamespace(overlay_rd_type_admin_subfield="10.0.0.1"))
    evpn_rd = observe(lambda: UtilsMixin.get_vlan_aware_bundle_rd(evpn_probe, bundle.id, None, tenant))
    evpn_rt = observe(lambda: UtilsMixin.get_vlan_aware_bundle_rt(evpn_probe, bundle.id, 0, tenant, is_vrf=False))

    p2p_inputs = EosDesigns._from_dict(
        {
            "network_services": [
                {
                    "name": "TENANT",
                    "pseudowire_rt_base": 1000,
                    "point_to_point_services": [
                        {
                            "name": "PW-A",
                            "endpoints": [
                                {"id": None, "nodes": ["leaf-a"], "interfaces": ["Ethernet1"]},
                                {"id": 2, "nodes": ["leaf-b"], "interfaces": ["Ethernet2"]},
                            ],
                            "subinterfaces": [{"number": 10}],
                        }
                    ],
                }
            ]
        }
    )
    p2p_tenant = p2p_inputs.network_services["TENANT"]
    p2p_probe = SimpleNamespace(
        shared_utils=SimpleNamespace(
            network_services_l1=True,
            overlay_ler=True,
            overlay_evpn_mpls=True,
            hostname="leaf-a",
            filtered_tenants=[p2p_tenant],
        )
    )

    ospf_inputs = EosDesigns._from_dict(
        {
            "underlay_ospf_area": "0.0.0.0",  # noqa: S104 -- OSPF area literal, not an address to bind.
            "underlay_ospf_authentication": {
                "enabled": True,
                "message_digest_keys": [{"id": 1, "cleartext_key": None}],
            },
        }
    )
    underlay_probe = SimpleNamespace(inputs=ospf_inputs, shared_utils=SimpleNamespace(underlay_routing_protocol="ospf"))
    vlan_interface = EosCliConfigGen.VlanInterfacesItem(name="Vlan10")

    vrf_ospf_key_type = EosDesigns._DynamicKeys.DynamicNetworkServicesItem.NetworkServicesItem.VrfsItem.Ospf.MessageDigestKeysItem
    vrf_ospf_key = vrf_ospf_key_type._from_dict({"id": 1, "cleartext_key": None})
    ethernet_interface = EosCliConfigGen.EthernetInterfacesItem(name="Ethernet1")

    wan_inputs = EosDesigns._from_dict({"wan_path_groups": [{"name": "DEFAULT", "id": None}]})
    wan_path_group = wan_inputs.wan_path_groups["DEFAULT"]
    wan_probe = SimpleNamespace(inputs=wan_inputs)

    return {
        "cv_topology_none_guard": {
            "cv_topology_is_none": cv_probe.cv_topology is None,
            "config_from_no_topology": observe(lambda: cv_probe.cv_topology_config._dump()),  # noqa: PLW0108 -- defer property access so observe captures exceptions.
        },
        "cv_topology_null_required_level_comparison": {
            "model_level": cv_level_inputs.cv_topology_levels["leaf"].level,
            "level_accessor": observe(lambda: cv_level_probe.get_cv_topology_level("leaf")),
            "config_with_neighbor": observe(lambda: cv_level_probe.cv_topology_config._dump()),  # noqa: PLW0108 -- defer property access so observe captures exceptions.
        },
        "evpn_bundle_null_id_rd_helper": {
            "model_id": bundle.id,
            "result": evpn_rd,
        },
        "evpn_bundle_null_id_rt_helper": {
            "model_id": bundle.id,
            "result": evpn_rt,
        },
        "point_to_point_null_endpoint_id_with_subinterface": {
            "model_id": p2p_tenant.point_to_point_services["PW-A"].endpoints[0].id,
            "result": observe(lambda: RouterBgpMixin._router_bgp_vpws(p2p_probe)),
        },
        "underlay_ospf_null_cleartext_encryption": {
            "model_cleartext_key": ospf_inputs.underlay_ospf_authentication.message_digest_keys[1].cleartext_key,
            "result": observe(lambda: AvdStructuredConfigMlag._set_mlag_l3_vlan_interface(underlay_probe, vlan_interface)),
        },
        "vrf_ospf_null_cleartext_missing_key_error": {
            "model_cleartext_key": vrf_ospf_key.cleartext_key,
            "result": observe(
                lambda: FilteredTenantsMixin.update_message_digest_key(
                    SimpleNamespace(), vrf_ospf_key, ethernet_interface, SimpleNamespace(), SimpleNamespace()
                )
            ),
        },
        "wan_path_group_null_id_loader_and_helper_fallback": {
            "model_id": wan_path_group.id,
            "configured_path_group_result": observe(lambda: RouterPathSelectionMixin._get_path_group_id(wan_probe, wan_path_group.name, wan_path_group.id)),
            "lan_ha_fallback_result": observe(lambda: RouterPathSelectionMixin._get_path_group_id(wan_probe, wan_inputs.wan_ha.lan_ha_path_group_name, None)),
        },
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--avd", type=Path, default=Path("/home/holbech/repos/avd"))
    args = parser.parse_args()
    avd_repo = args.avd.resolve()
    report_dir = Path(__file__).resolve().parent
    result = {
        "avd_checkout": str(avd_repo),
        "scope": "read-only schema traversal, source-template fragment renders, generated-model and isolated real-method probes",
        "inventory": inventory_observations(report_dir, avd_repo),
        "templates": template_observations(avd_repo),
        "models": model_observations(),
        "methods": method_observations(),
        "output_validation": output_validation_observations(),
        "output_validation_context": output_validation_context(),
    }
    print(json.dumps(result, indent=2, sort_keys=True))  # noqa: T201 -- command emits its diagnostic JSON to stdout.


if __name__ == "__main__":
    main()
