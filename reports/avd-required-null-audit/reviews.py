# Copyright (c) 2026 Arista Networks, Inc.
# ruff: noqa: INP001  # Standalone review data stored beside its generated report artifacts.

"""
Human-reviewed consumer evidence, keyed by required declaration identity.

Each entry establishes behavior at the cited site, not a proof that all callers
are safe. Regexes only group declarations sharing the inspected implementation;
the expanded dataset keeps each declaration and every effective use-site.
"""

from __future__ import annotations

import re


def source_label(origin: str) -> str:
    """Return a readable source pointer without key/item schema scaffolding."""
    return ".".join(part for part in origin.split("#/")[1].split("/") if part not in {"keys", "items"})


# Pattern, observed-site classification, impact, and AVD-relative evidence sites.
DESIGN_REVIEWS = [
    (
        r"aaa_settings\.(tacacs|radius)\.servers\.host",
        "runtime-risk",
        "Host is copied unchecked into output and iterated to derive an encryption salt when a cleartext key is supplied; None is not iterable.",
        [("structured_config/base/utils.py", 112), ("structured_config/base/aaa_settings.py", 85)],
    ),
    (
        r"bfd_multihop\.(interval|min_rx|multiplier)",
        "handoff",
        (
            "Model is cast directly to EOS BFD multihop; generated null scalars are stripped. These downstream fields are optional. "
            "Output-boundary probes pass with a removed field and retained siblings, so output validation does not prevent omission."
        ),
        [("structured_config/overlay/router_bfd.py", 27)],
    ),
    (
        r"bgp_graceful_restart\.enabled",
        "null-suppresses",
        "Truthiness gates graceful restart. None suppresses configuration rather than causing a type failure.",
        [("structured_config/base/router_bgp.py", 61)],
    ),
    (
        r"bgp_peer_groups\.mlag_ipv4_vrfs_peer\.name",
        "handoff",
        "Name is compared and returned without a non-null guard; downstream peer-group naming must not be assumed valid.",
        [("shared_utils/mlag.py", 226), ("shared_utils/mlag.py", 232)],
    ),
    (
        r"connected_endpoints\.adapters\.switches",
        "collection-suppression",
        (
            "Merged adapter with a null/empty switches list is filtered out for this host. Current port profiles do not declare switches; "
            "generic model inheritance behavior is not evidence of production port-profile switch inheritance."
        ),
        [("shared_utils/connected_endpoints.py", 41)],
    ),
    (
        r"connected_endpoints\.adapters\.switch_ports",
        "explicit-error",
        "If switches selects this host, null switch_ports becomes an empty typed list and triggers the length mismatch diagnostic after profile merging.",
        [("shared_utils/connected_endpoints.py", 47)],
    ),
    (
        r"\$defs\.l3_edge\.p2p_links\.nodes",
        "collection-suppression",
        "Host-membership filtering excludes a null/empty node list; selected links subsequently rely on positional node access.",
        [("structured_config/core_interfaces_and_l3_edge/utils.py", 50), ("structured_config/core_interfaces_and_l3_edge/utils.py", 148)],
    ),
    (
        r"\$defs\.monitor_sessions\.name",
        "handoff",
        "Session configs are grouped by name and copied to output; no non-null session-name check at this site.",
        [("structured_config/base/monitor_sessions.py", 34), ("structured_config/base/monitor_sessions.py", 50)],
    ),
    (
        r"\$defs\.adapter_config\.ethernet_segment\.short_esi",
        "null-suppresses",
        "None returns from ESI selection before ESI operations, suppressing the generated Ethernet segment.",
        [("structured_config/connected_endpoints/utils.py", 60)],
    ),
    (
        r"cv_pathfinder_internet_exit_policies\.type",
        "runtime-risk",
        "The direct-policy test has a zscaler else branch; None is not a validated policy kind and takes that other branch.",
        [("structured_config/network_services/router_internet_exit.py", 77)],
    ),
    (
        r"cv_pathfinder_internet_exit_policies\.zscaler\.(ipsec_key_salt|domain_name)",
        "explicit-error",
        "A truthiness check raises a missing-variable error before generating the IPsec identity/key.",
        [("structured_config/network_services/utils_wan.py", 353), ("structured_config/network_services/utils_wan.py", 357)],
    ),
    (
        r"cv_settings\.cvaas\.enabled",
        "null-suppresses",
        "None selects no CVaaS clusters; this is a behavioral suppression, not literal None output.",
        [("structured_config/base/daemon_terminattr.py", 38)],
    ),
    (
        r"cv_settings\.onprem_clusters\.servers",
        "collection-risk",
        "Empty/null list gives empty cvaddrs; the in-band ZTP path also calls next(iter(servers)) and can raise StopIteration.",
        [("structured_config/base/daemon_terminattr.py", 116), ("structured_config/underlay/dhcp_servers.py", 54)],
    ),
    (
        r"cv_topology\.platform",
        "null-fallback",
        "Returned as an optional platform. Platform lookup uses a default fallback; None does not establish the specified topology platform.",
        [("shared_utils/cv_topology.py", 57), ("shared_utils/platform_mixin.py", 66)],
    ),
    (
        r"cv_topology\.interfaces",
        "collection-suppression",
        "Iteration over the typed null list produces no derived links.",
        [("shared_utils/cv_topology.py", 76)],
    ),
    (
        r"cv_topology_levels\.level",
        "runtime-risk",
        "Returned without a scalar guard, then compared with neighbor_level using <; None causes TypeError when a usable neighbor is present.",
        [("shared_utils/cv_topology.py", 66), ("shared_utils/cv_topology.py", 98)],
    ),
    (
        r"default_interfaces\.(types|platforms)",
        "collection-suppression",
        "Null list cannot match platform/type selection; later lookup falls back to another entry or an empty model.",
        [("shared_utils/platform_mixin.py", 71)],
    ),
    (
        r"default_node_types\.match_hostnames",
        "collection-suppression",
        "Null list matches no hostname; device-type selection can ultimately raise its existing no-type diagnostic.",
        [("shared_utils/node_type.py", 43), ("shared_utils/node_type.py", 37)],
    ),
    (
        r"\$defs\.node_type\.defaults\.evpn_gateway\.d_path\.(local_domain_id|remote_domain_id)",
        "null-fallback",
        "Uses `or` to fall back to the old all-active domain-ID settings; the fallback itself may be unset.",
        [("structured_config/overlay/router_bgp.py", 388)],
    ),
    (
        r"\$defs\.node_type\.defaults\.(evpn_gateway\.all_active_multihoming|ipvpn_gateway)\.enabled",
        "null-suppresses",
        "Truthiness gates the gateway feature. None disables the branch; it is not equivalent to a type-guaranteed bool.",
        [("structured_config/overlay/router_bgp.py", 358), ("shared_utils/overlay.py", 165)],
    ),
    (
        r"\$defs\.node_type\.defaults\.evpn_gateway\.all_active_multihoming\.evpn_ethernet_segment(\.(identifier|rt_import))?",
        "handoff",
        (
            "When all-active is enabled, required segment scalars are copied without checks. Null dict also exposes unset children; stripped output "
            "can lack required segment fields."
        ),
        [("structured_config/overlay/router_bgp.py", 393)],
    ),
    (
        r"\$defs\.node_type\.defaults\.ipvpn_gateway\.remote_peers\.(ip_address|bgp_as)",
        "handoff",
        "Copied directly into neighbor generation. get_asn explicitly preserves None rather than rejecting it; downstream neighbor fields may remain unset.",
        [("structured_config/overlay/router_bgp.py", 702), ("shared_utils/routing.py", 107)],
    ),
    (
        r"\$defs\.node_type_l3_(interfaces|port_channels)\.bgp\.peer_as",
        "handoff",
        "Copied to BGP neighbor remote_as without scalar non-null check; generated None can be stripped rather than rendered.",
        [("shared_utils/misc.py", 429), ("shared_utils/misc.py", 471)],
    ),
    (
        r"(\$defs\.node_type_l3_interfaces\.static_routes|\$defs\.static_routes)\.prefix",
        "stripped-required-output",
        (
            "Copied into required EOS static-route prefix. Stripping removes null; output validation rejects the missing prefix if the route "
            "item survives. A retained-next-hop probe demonstrates this. This is not successful omission in the validated workflow; "
            "inactive inputs, item pruning and later overrides still require context review."
        ),
        [("structured_config/underlay/static_routes.py", 43), ("structured_config/network_services/static_routes.py", 41)],
    ),
    (
        r"digital_twin\.fabric",
        "collection-risk",
        (
            "Null model blocks fabric inheritance but callers access its children directly. Defaults may supply some children; this is not an "
            "automatic feature-disable guarantee."
        ),
        [("structured_config/base/__init__.py", 561), ("structured_config/metadata/digital_twin.py", 65)],
    ),
    (
        r"dns_settings\.servers",
        "collection-suppression",
        "Typed null list yields no DNS server entries; other paths separately require DNS for CloudVision.",
        [("structured_config/base/dns_settings.py", 42), ("structured_config/base/daemon_terminattr.py", 171)],
    ),
    (
        r"dns_settings\.servers\.ip_address",
        "handoff",
        "Required non-primary IP address is copied unchecked to an indexed EOS server item; not a valid input guarantee.",
        [("structured_config/base/dns_settings.py", 52)],
    ),
    (
        r"dot1x_settings\.mac_based_authentication\.username_format\.(delimiter|letter_case)",
        "output-validation-dependent",
        (
            "Copied into required EOS delimiter/mac_string_case. A single null with the sibling retained fails output validation after stripping; "
            "both null children prune the optional username-format parent and validate successfully. The template guard alone is not the verdict."
        ),
        [("structured_config/base/dot1x.py", 99)],
    ),
    (
        r"dot1x_settings\.web_authentication\.enabled",
        "null-suppresses",
        "False/None returns before captive-portal generation.",
        [("structured_config/base/dot1x.py", 111)],
    ),
    (
        r"evpn_vlan_bundles\.id",
        "runtime-risk",
        "Required ID is sent to RD/RT derivation as id and vni with no non-null check at the call site.",
        [("structured_config/network_services/router_bgp.py", 640), ("structured_config/network_services/utils.py", 403)],
    ),
    (
        r"fabric_name",
        "explicit-error",
        "Truthiness guard raises the existing missing-variable error before returning fabric name.",
        [("shared_utils/misc.py", 169)],
    ),
    (
        r"generate_cv_tags\.device_tags\.name",
        "handoff",
        "Only reserved-name membership is checked; None is passed as a tag name when a value exists, reaching metadata rather than CLI.",
        [("structured_config/metadata/cv_tags.py", 149), ("structured_config/metadata/cv_tags.py", 173)],
    ),
    (
        r"internal_vlan_order\.(allocation|range\.(beginning|ending))",
        "output-validation-dependent",
        (
            "Direct _cast_as model conversion, not a numeric cast. EOS allocation/range children are required. Partial-null output with a "
            "retained parent fails validation after stripping; all-null output prunes the optional parent and validates. Later override repair "
            "can also make output valid. Template suppression does not establish successful generation in the validated workflow."
        ),
        [("structured_config/base/__init__.py", 224)],
    ),
    (
        r"ipv[46]_acls\.entries",
        "collection-risk",
        (
            "Null collection remains empty; ACL-specific validation and consumers must decide whether an empty ACL is acceptable. No scalar-tightening "
            "benefit here."
        ),
        [("structured_config/connected_endpoints/utils.py", 279)],
    ),
    (
        r"ipv[46]_prefix_list_catalog\.sequence_numbers",
        "collection-suppression",
        "Catalog is cast to output; empty sequence collection yields no sequence commands.",
        [("shared_utils/misc.py", 312)],
    ),
    (
        r"ipv4_prefix_list_catalog\.sequence_numbers\.action",
        "stripped-required-output",
        (
            "Catalog cast passes action into required EOS sequence action. Stripping null retains the sequence primary key; output validation "
            "then rejects missing action. The template's null guard does not make this successful suppression when output validation is used. "
            "Unused catalogs and overrides remain compatibility checks."
        ),
        [("shared_utils/misc.py", 312)],
    ),
    (
        r"ipv6_prefix_list_catalog\.sequence_numbers\.action",
        "stripped-required-output",
        (
            "Catalog cast passes action into required EOS sequence action. Normal generated null is stripped, leaving the sequence key; "
            "output validation rejects missing action. Custom null applied after stripping is a separate path and can reach the template."
        ),
        [("shared_utils/misc.py", 315)],
    ),
    (
        r"\$defs\.l2vlans\.private_vlan\.(type|primary_vlan)",
        "handoff",
        "Required scalars are copied unchecked; primary_vlan is also added to the primary VLAN set.",
        [("structured_config/network_services/vlans.py", 77)],
    ),
    (
        r"logging_settings\.hosts\.name",
        "handoff",
        "Required non-primary host name is passed to an indexed EOS logging host without a null guard.",
        [("structured_config/base/logging.py", 79)],
    ),
    (
        r"management_eapi\.vrfs\.enabled",
        "null-suppresses",
        "Truthiness guard excludes this VRF when enabled is None.",
        [("structured_config/base/__init__.py", 424)],
    ),
    (
        r"monitor_connectivity\.(vrfs\.interface_sets|interface_sets)\.interfaces",
        "collection-risk",
        "Null list is joined into an empty interfaces string rather than None. Keeping null collections does not by itself prevent incomplete output.",
        [("structured_config/base/monitor_connectivity.py", 35), ("structured_config/base/monitor_connectivity.py", 56)],
    ),
    (
        r"network_services\.(vrfs\.)?bgp_peer_groups\.address_family_ipv6\.default_originate\.enabled",
        "handoff",
        "Peer-group IPv6 settings are cast into EOS BGP. Final default-originate tests are null-aware; source requiredness is not a non-null bool guarantee.",
        [("structured_config/network_services/router_bgp.py", 110)],
    ),
    (
        r"network_services\.(vrfs\.)?bgp_peer_groups\.listen_ranges\.prefix",
        "handoff",
        "Listen-range prefix is copied directly into EOS output when the node matches, with no scalar non-null check.",
        [("structured_config/network_services/router_bgp.py", 865)],
    ),
    (
        r"network_services\.vxlan_flood_multicast\.enabled",
        "null-suppresses",
        "default(vlan.enabled, tenant.enabled) chooses the tenant setting; None at tenant level makes multicast configuration falsy.",
        [("structured_config/network_services/vxlan_interface.py", 207)],
    ),
    (
        r"network_services\.evpn_l3_multicast\.evpn_underlay_l3_multicast_group_ipv4_pool",
        "explicit-error",
        "When L3 multicast needs allocation, falsy pool raises a missing-variable diagnostic before address allocation.",
        [("structured_config/network_services/vxlan_interface.py", 157)],
    ),
    (
        r"network_services\.vrfs\.ospf\.message_digest_keys\.cleartext_key",
        "runtime-risk",
        "None reaches an else branch documented as impossible for VRF keys. The match only handles interface key types, leaving msg unassigned before raise.",
        [("shared_utils/filtered_tenants.py", 683), ("shared_utils/filtered_tenants.py", 692)],
    ),
    (
        r"network_services\.vrfs\.svis\.name",
        "null-fallback",
        "Used as fallback description and in diagnostics; None omits description rather than guaranteeing a name. Profile inheritance remains relevant.",
        [("shared_utils/filtered_tenants.py", 331), ("structured_config/network_services/vlan_interfaces.py", 80)],
    ),
    (
        r"network_services\.vrfs\.l3_port_channels\.name",
        "runtime-risk",
        "Selected port-channel name is used in `'.' in name` and string operations without scalar guard.",
        [("structured_config/network_services/port_channel_interfaces.py", 60)],
    ),
    (
        r"network_services\.vrfs\.(l3_port_channels|loopbacks)\.node",
        "null-suppresses",
        "Host equality filtering excludes a None node from every device.",
        [("shared_utils/filtered_tenants.py", 453), ("shared_utils/filtered_tenants.py", 269)],
    ),
    (
        r"network_services\.vrfs\.loopbacks\.loopback",
        "runtime-risk",
        "Name is constructed by interpolation with no null guard, producing LoopbackNone before scalar stripping can help.",
        [("structured_config/network_services/loopback_interfaces.py", 46)],
    ),
    (
        r"network_services\.vrfs\.loopbacks\.ip_address",
        "handoff",
        "Copied unchecked to output and passed to source-NAT helper. End-to-end behavior is context dependent; not certified safe.",
        [("structured_config/network_services/loopback_interfaces.py", 47), ("structured_config/network_services/loopback_interfaces.py", 58)],
    ),
    (
        r"network_services\.vrfs\.static_arp_entries\.(ipv4_address|mac_address)",
        "stripped-required-output",
        (
            "Copied into required EOS static ARP address fields. Stripping null with siblings/VRF retained leaves an incomplete item that fails "
            "output validation. These handoffs are not proven successful omission. Custom null merged later remains a separate path."
        ),
        [("structured_config/network_services/arp.py", 37)],
    ),
    (
        r"network_services\.vrfs\.bgp\.graceful_restart\.enabled",
        "handoff",
        (
            "False emits no_graceful_restart; None instead enters the copy branch. Scalar stripping then removes enabled, so None is not equivalent "
            "to explicit false."
        ),
        [("structured_config/network_services/router_bgp.py", 158)],
    ),
    (
        r"network_services\.vrfs\.additional_route_targets\.type",
        "runtime-risk",
        "Only 'import' chooses import; None takes export branch. This is silent wrong-direction behavior, not necessarily None output.",
        [("structured_config/network_services/router_bgp.py", 370)],
    ),
    (
        r"network_services\.vrfs\.additional_route_targets\.(address_family|route_target)",
        "handoff",
        "Used to obtain an indexed address family / extend scalar route-target list without null checks; scalar list items need their own validation policy.",
        [("structured_config/network_services/router_bgp.py", 371)],
    ),
    (
        r"network_services\.l2vlans\.name",
        "handoff",
        "Copied to VLAN output name without a scalar guard; generated null name is stripped and does not provide a required name guarantee.",
        [("structured_config/network_services/vlans.py", 104)],
    ),
    (
        r"network_services\.point_to_point_services\.endpoints\.id",
        "runtime-risk",
        "With subinterfaces, required endpoint id participates directly in addition; None raises TypeError. Without them it is copied to output.",
        [("structured_config/network_services/router_bgp.py", 826)],
    ),
    (
        r"network_services\.point_to_point_services\.endpoints\.(nodes|interfaces)",
        "collection-risk",
        "Null nodes selects no local endpoint; interfaces has guards in some consumers but positional indexing in others. Preserve separate collection policy.",
        [("structured_config/network_services/router_bgp.py", 815), ("structured_config/network_services/patch_panel.py", 39)],
    ),
    (
        r"sflow_settings\.destinations\.destination",
        "handoff",
        "Required non-primary destination copied to indexed EOS sFlow destination without scalar check.",
        [("structured_config/structured_config_utils/sflow.py", 43)],
    ),
    (
        r"ssh_settings\.vrfs\.enabled",
        "null-suppresses",
        "None prevents ACL assignment and is copied to output enable; stripping/defined guards omit enable configuration.",
        [("structured_config/base/management_ssh.py", 43)],
    ),
    (
        r"underlay_multicast_rps\.nodes\.loopback_number",
        "runtime-risk",
        "Direct interpolation creates a LoopbackNone interface name, which is no longer null and cannot be stripped as null.",
        [("shared_utils/underlay.py", 87)],
    ),
    (
        r"underlay_ospf_authentication\.enabled",
        "null-suppresses",
        "Truthiness gates adding OSPF authentication on underlay interfaces.",
        [("structured_config/underlay/ethernet_interfaces.py", 101)],
    ),
    (
        r"underlay_ospf_authentication\.message_digest_keys",
        "collection-suppression",
        "Null list yields no digest keys even when authentication is enabled.",
        [("structured_config/underlay/ethernet_interfaces.py", 105)],
    ),
    (
        r"underlay_ospf_authentication\.message_digest_keys\.cleartext_key",
        "runtime-risk",
        "Null cleartext_key in an entry reaches encryption without a scalar guard.",
        [("structured_config/underlay/ethernet_interfaces.py", 109), ("structured_config/mlag/__init__.py", 129)],
    ),
    (
        r"wan_carriers\.path_group",
        "explicit-error",
        "Active WAN carrier path_group goes through get(required=True), which rejects None; otherwise metadata association only matches named groups.",
        [("shared_utils/wan.py", 161), ("structured_config/metadata/cv_pathfinder.py", 73)],
    ),
    (
        r"wan_ipsec_profiles\.control_plane",
        "collection-risk",
        "Null dict blocks inheritance; callers read profile names and then explicitly reject missing shared key. It is not simply an ignored empty model.",
        [("structured_config/overlay/ip_security.py", 76)],
    ),
    (
        r"wan_path_groups\.id",
        "null-fallback",
        "Explicit None takes the helper fallback 500 (65535 for LAN HA); rejecting null would change this accepted branch.",
        [("structured_config/overlay/router_path_selection.py", 124)],
    ),
    (
        r"wan_route_servers\.path_groups\.interfaces",
        "collection-suppression",
        "Empty/null interface list yields no usable public IPs in static peer discovery; some route-server paths reconstruct data from facts.",
        [("structured_config/overlay/router_path_selection.py", 202), ("shared_utils/wan.py", 207)],
    ),
    (
        r"wan_virtual_topologies\.vrfs\.wan_vni",
        "handoff",
        "Used directly as VNI and dict key in generated VXLAN config and metadata. None loses VNI configuration after stripping; not a guaranteed int.",
        [("structured_config/network_services/vxlan_interface.py", 112), ("structured_config/metadata/cv_pathfinder.py", 125)],
    ),
    (
        r"\$defs\.virtual_topology\.path_groups\.names",
        "collection-suppression",
        "Empty typed list adds no path groups and can leave the resulting load balance policy empty and omitted.",
        [("structured_config/network_services/utils_wan.py", 194), ("structured_config/network_services/utils_wan.py", 218)],
    ),
    (
        r"wan_virtual_topologies\.policies\.default_virtual_topology",
        "collection-risk",
        "Consumer tests its path_groups and treats empty topology separately; typed-null model still exposes child defaults. Keep model-null policy separate.",
        [("structured_config/network_services/utils_wan.py", 113)],
    ),
    (
        r"zscaler_endpoints\.primary",
        "collection-suppression",
        "Endpoint loop skips falsy null model; this can produce no primary tunnel. Accessing default attributes can affect model truthiness.",
        [("structured_config/network_services/router_internet_exit.py", 184)],
    ),
    (
        r"zscaler_endpoints\.primary\.ip_address",
        "runtime-risk",
        "Used for tunnel destination, monitor host and static route construction without scalar non-null check once endpoint model is truthy.",
        [("structured_config/network_services/router_internet_exit.py", 200), ("structured_config/network_services/router_internet_exit.py", 207)],
    ),
    (
        r"zscaler_endpoints\.primary\.(datacenter|city|country|region|latitude|longitude)",
        "metadata-handoff",
        (
            "Endpoint is cast to CloudVision Internet Exit metadata; these are not direct CLI scalars. Stripping may omit them; downstream API "
            "requiredness is not proven here."
        ),
        [("structured_config/network_services/router_internet_exit.py", 215)],
    ),
    (
        r"zscaler_endpoints\.cloud_name",
        "runtime-risk",
        "Interpolated without a scalar guard into monitor URL, giving http://gateway.None.net/vpntest.",
        [("structured_config/network_services/monitor_connectivity.py", 61)],
    ),
    (
        r"zscaler_endpoints\.device_location(\.(city|country))?",
        "metadata-handoff",
        "Location children are read directly and copied into Internet Exit metadata; null model/children do not provide a required scalar guarantee.",
        [("structured_config/network_services/metadata.py", 39)],
    ),
]


def review_design(group: dict) -> dict | None:
    """Expand a reviewed implementation-family finding to one declaration."""
    label = source_label(group["origin"])
    demonstrated_methods = {
        "cv_topology_levels.level",
        "evpn_vlan_bundles.id",
        "network_services.point_to_point_services.endpoints.id",
        "network_services.vrfs.ospf.message_digest_keys.cleartext_key",
        "underlay_ospf_authentication.message_digest_keys.cleartext_key",
    }
    for pattern, classification, impact, evidence in DESIGN_REVIEWS:
        if re.fullmatch(pattern, label):
            return {
                "classification": classification,
                "review_status": "demonstrated-method" if label in demonstrated_methods else "inspected-site" if evidence else "needs-consumer-trace",
                "impact": impact,
                "evidence": [{"file": "python-avd/pyavd/_eos_designs/" + file, "line": line} for file, line in evidence],
            }
    if label == "eos_designs_custom_templates.template":
        return {
            "classification": "handoff",
            "review_status": "inspected-site",
            "impact": (
                "Ansible action reads template_item['template'] and passes it to the templar. None is not a valid template path; this is outside "
                "pyAVD consumers."
            ),
            "evidence": [{"file": "ansible_collections/arista/avd/plugins/action/eos_designs_structured_config.py", "line": 109}],
        }
    return None


CONFIG_REVIEWS = [
    (
        r"agents\.environment_variables\.value",
        "literal-none",
        "demonstrated-fragment",
        "AVD Templar renders TOKEN=None because the value is concatenated into an environment assignment without a value guard.",
        [("eos/agents-environment.j2", 14)],
    ),
    (
        r"(ethernet_interfaces|port_channel_interfaces)\.link_tracking_groups\.direction",
        "literal-none",
        "inspected-site",
        "Direction is directly interpolated. Ethernet fragment probe rendered link tracking group UPSTREAM None.",
        [("eos/ethernet-interfaces.j2", 1134), ("eos/port-channel-interfaces.j2", 975)],
    ),
    (
        r"vlan_internal_order\.(allocation|range\.(beginning|ending))",
        "null-suppresses",
        "demonstrated-fragment",
        "The three-scalar non-null guard suppresses the entire internal VLAN order command.",
        [("eos/vlan-internal-order.j2", 7)],
    ),
    (
        r"prefix_lists\.sequence_numbers\.action",
        "null-suppresses",
        "demonstrated-fragment",
        "IPv4 sequence.action has arista.avd.defined guard; probe emits the list header without the sequence.",
        [("eos/prefix-lists.j2", 11)],
    ),
    (
        r"ipv6_prefix_lists\.sequence_numbers\.action",
        "literal-none",
        "demonstrated-fragment",
        "IPv6 action is directly interpolated; probe emits seq 10 None.",
        [("eos/ipv6-prefix-lists.j2", 11)],
    ),
    (
        r"static_routes\.prefix",
        "literal-none",
        "demonstrated-fragment",
        "Prefix is concatenated without a scalar guard; probe emits ip route None 192.0.2.1.",
        [("eos/static-routes.j2", 20)],
    ),
    (
        r"arp\.static_entries\.(ipv4_address|mac_address)",
        "literal-none",
        "inspected-site",
        "The list is repartitioned into VRF aliases, so the simple index misses it; both required ARP scalars are directly interpolated with no child guard.",
        [("eos/arp.j2", 31), ("eos/arp.j2", 34)],
    ),
    (
        (
            r"(router_bgp\.(peer_groups|neighbors)|router_bgp\.address_family_ipv4_labeled_unicast\.(bgp|peer_groups|neighbors))"
            r"\.missing_policy\.(direction_in|direction_out)\.action"
        ),
        "null-suppresses",
        "inspected-site",
        "Dynamic direction lookup prevented simple path indexing. policy.action is checked with arista.avd.defined before CLI construction.",
        [("eos/router-bgp.j2", 309), ("eos/router-bgp.j2", 1754)],
    ),
    (
        r"ethernet_interfaces\.ipv6_dhcp_relay_destinations\.address",
        "template-unguarded-read",
        "inspected-site",
        "Default/non-default VRF partitions are built via list addition; each destination address is concatenated without a child scalar guard.",
        [("eos/ethernet-interfaces.j2", 370)],
    ),
    (
        r"dot1x\.aaa\.unresponsive\.phone_action\.cached_results_timeout\.(time_duration|time_duration_unit)",
        "template-unguarded-read",
        "inspected-site",
        "Global dot1x template checks only timeout model presence before concatenating the required children; interface-scoped template has child guards.",
        [("eos/dot1x.j2", 42), ("eos/dot1x.j2", 43), ("eos/ethernet-interfaces.j2", 1268)],
    ),
    (
        r"ethernet_interfaces\.(uc_tx_queues|tx_queues)\.random_detect\.ecn\.threshold\.(units|min|max)",
        "template-unguarded-read",
        "inspected-site",
        "Included queue partial checks threshold model, not these scalar children, before building minimum/maximum ECN thresholds.",
        [("eos/ethernet-interface-uc-tx-queues.j2", 27), ("eos/ethernet-interface-tx-queues.j2", 28)],
    ),
    (
        r"ethernet_interfaces\.tx_queues\.random_detect\.ecn\.threshold\.max_probability",
        "null-suppresses",
        "inspected-site",
        "Queue partial uses arista.avd.defined on max_probability before adding its command fragment.",
        [("eos/ethernet-interface-tx-queues.j2", 34)],
    ),
    (
        r"management_security\.shared_secret_profiles\.secrets\.(receive_lifetime|transmit_lifetime)",
        "collection-risk",
        "inspected-site",
        (
            "Required lifetime dicts are not scalar scope. Template reads their child fields in a compound guard; null dict acceptance needs separate "
            "rendering tests."
        ),
        [("eos/management-security.j2", 169)],
    ),
    (
        r"traffic_policies\.vrfs\.cpu\.traffic_policy",
        "collection-risk",
        "inspected-site",
        "Required policy dict is accessed through its name child under a cpu-parent guard; a null dict is not a guaranteed complete policy.",
        [("eos/traffic-policies.j2", 31)],
    ),
    (
        r"vlan_internal_order\.range",
        "collection-risk",
        "inspected-site",
        "Child guards suppress invalid/unset range values; collection remains outside proposed scalar strictness.",
        [("eos/vlan-internal-order.j2", 7)],
    ),
]


def review_config(group: dict) -> dict | None:
    """Return manual evidence for template boundaries the local index misses."""
    label = source_label(group["origin"])
    for pattern, classification, status, impact, evidence in CONFIG_REVIEWS:
        if re.fullmatch(pattern, label):
            return {
                "classification": classification,
                "review_status": status,
                "impact": impact,
                "evidence": [{"file": "python-avd/pyavd/_eos_cli_config_gen/j2templates/" + file, "line": line} for file, line in evidence],
            }
    if label.startswith("metadata."):
        return {
            "classification": "metadata-handoff",
            "review_status": "downstream-unresolved",
            "impact": (
                "Required metadata is carried to structured-config consumers, not necessarily CLI. Null/omitted data may affect CloudVision deployment "
                "or ANTA; downstream compatibility has not been certified."
            ),
            "evidence": [],
        }
    return None
