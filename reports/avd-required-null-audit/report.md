# AVD Design required-scalar null audit

## Decision scope

Outside **all** structured-config overrides, AVD Design has **138 required non-primary scalar declarations and 28 collection declarations**, used at **448 schema occurrences**. This is the primary decision scope. Root `structured_config` and every named `*_structured_config` override are inventoried separately rather than inflating the ordinary-design totals.

All 166 declarations now have a static assessment covering their consumer families and ordinary occurrence routing. The proposed decision is **reject explicit null on required fields**, accepting that some currently permitted inputs will start failing. Scalar-only enforcement and enforcement covering required collections are evaluated separately. Optional-field nulls and structured-config overrides are not implicitly included. This report changes neither validation nor consumers.

The initial consumer index had 107 inspected-site, 5 isolated-method and 54 candidate-only entries. That index is retained for provenance, **not current review progress**. The subsequent declaration assessments now trace the Python handoffs, selection, null stripping and target requiredness. The separate override appendix retains the 29 unresolved EOS metadata declarations, which are **override-only**.

No primary declaration is pending or unresolved in the static assessment. This does not certify every combination of surrounding inputs, full-fabric builds, external APIs or EOS device acceptance. Previously saved probes are preserved; no new probes were needed for this review. Customer prevalence remains **unmeasured**.

| Static assessment | Scalars | Collections | Total |
| --- | ---: | ---: | ---: |
| Inspected active path already rejects null; earlier-error candidate | 6 | 1 | 7 |
| Both failing and permitted contexts | 95 | 11 | 106 |
| Permitted null suppression, fallback, omission or formatting path identified | 37 | 16 | 53 |
| Total assessed | 138 | 28 | 166 |

These are declaration counts, not affected deployments. In particular, the 106 mixed findings combine active errors with dormant definitions, parent/item pruning or supported bypasses. They do **not** establish that 106 active deployment scenarios are silently broken today. Conversely, the seven earlier-error candidates are not universally safe changes: later custom overrides can repair missing generated output, and direct pyAVD consumers may omit final output validation.

## Scope and dataset

Snapshot: AVD checkout `/home/holbech/repos/avd`, HEAD `d3ef7981bdc62f68bcc4211e7d7e691505cf5c3d`, using current YAML fragments and consumer code. The checkout has unrelated experimental/staged work; it was not reset or changed by this audit. This is not a release-history comparison.

The traversal combines fragments in build order and uses AVD's existing reference resolver. It visits static keys, dynamic keys and list items; documentation visibility and pure-reference boundaries do not stop traversal. `$defs` are counted through their actual uses, not separately as additional input occurrences. Effective required fields are grouped by the declaration that supplied `required`, retaining local defaults, references and relaxed contexts on each occurrence. Direct list-item primary keys and removed subtrees are excluded. A nested dictionary key whose name happens to equal an ancestor list's primary key is **not** excluded.

| Measure | Count |
| --- | ---: |
| Required non-primary occurrences outside overrides | 448 |
| Scalar / collection occurrences | 402 / 46 |
| Distinct source declarations outside overrides | 166 |
| Declarations authored in AVD Design | 108 |
| Declarations inherited from EOS Config | 58 |
| Scalar declarations: string / integer / boolean | 138: 89 / 33 / 16 |
| Collection declarations: list / dictionary | 28: 22 / 6 |
| Completed static declaration assessments | 166 |
| Pending / unresolved primary assessments | 0 / 0 |
| Relaxed required occurrences outside overrides | 0 |

Counts describe schema occurrences, not populated fields on a device, independent consumers, or affected devices. Source declarations can be reused at multiple occurrences and in both scopes. Grouping references does not erase their different application contexts.

A separate literal-fragment scan found 108 authored AVD Design required non-primary declarations, all represented by effective occurrences in the dataset; no unused authored declaration was silently omitted. Regeneration also matched every occurrence and source group against the current YAML snapshot.

The [primary per-declaration ledger](fields.md) lists all 166 declarations with only their ordinary-design paths, active outcomes, permitted contexts, migration advice and source evidence. The [open assessment list](open-fields.md) is now empty. Editable assessments live in [declaration_reviews.json](declaration_reviews.json), keyed by exact source declaration origin so regeneration preserves findings. The [structured-config appendix](structured-config-fields.md) lists the 550 override declarations. The [machine-readable dataset](inventory.json) retains both scopes, every effective path, default, valid-values constraint, description, reference chain and relaxed boundary. Each primary group has `avd_design_evaluation`; shared declarations never apply that assessment to override occurrences. `review_id` is a snapshot convenience, not persistent identity.

## Structured-config overrides: separate scope

Every relaxed required occurrence in this snapshot lies inside one of these override boundaries. The earlier suggestion that ordinary profiles supplied additional relaxed required occurrences was incorrect: **there are none outside overrides**. This does not settle when requiredness should apply during future staged validation.

| Override boundary | Required non-primary occurrences |
| --- | ---: |
| `structured_config` | 13,461 |
| `uplink_ethernet_structured_config` | 600 |
| `uplink_switch_ethernet_structured_config` | 600 |
| `uplink_port_channel_structured_config` | 450 |
| `uplink_switch_port_channel_structured_config` | 450 |
| `mlag_port_channel_structured_config` | 450 |
| `mlag_peer_vlan_structured_config` | 350 |
| `mlag_peer_l3_vlan_structured_config` | 350 |
| `ethernet_structured_config` | 240 |
| `port_channel_structured_config` | 180 |
| **Total overrides** | **17,131** |

Filtering only literal `.structured_config.` missed **3,670** named-override occurrences. Scope classification now checks both override names and recorded relaxed-reference boundaries, and stops regeneration if they disagree.

Overrides contain 550 distinct EOS Config declarations: 515 scalars and 35 collections. Of these, 58 also occur in ordinary design and 492 are override-only. Thus the full inventory remains **17,579 occurrences and 658 declarations**: `448 + 17,131` occurrences, and `166 + 550 − 58` declarations. Across both scopes there are 597 scalar and 61 collection declarations. The traversal also excluded 837 explicitly required primary-key occurrences and one removed required occurrence; these are full-traversal exclusion counts, not primary-scope counts.

## What validation and loading actually guarantee

Current Rust validation checks required-key **presence**, not a non-null scalar value. Its default configuration accepts null for scalars and collections. Primary keys are different: a missing or null primary key is explicitly rejected, including duplicate-allowed lists. Relaxed reference subtrees skip required-key checks, but are still visited for other validation.

Relevant implementation: [required keys](/home/holbech/repos/pyavd-utils/rust/validation/src/validation/dict.rs:80), [scalar null acceptance](/home/holbech/repos/pyavd-utils/rust/validation/src/validation/mod.rs:24), [primary keys](/home/holbech/repos/pyavd-utils/rust/validation/src/validation/list.rs:102).

Legacy Python loading leaves scalar `None` untouched. Collection/model nulls become typed empty objects with `_created_from_null=True`. Both scalar null and collection null count as explicitly set for inheritance; defaults are supplied for absent attributes, not substituted for explicitly set scalar null. `_deepinherit` does not fill a null scalar from a lower-priority source. Null-created models also block inheritance.

Relevant implementation: [coercion](/home/holbech/repos/avd/python-avd/pyavd/_schema/coerce_type.py:29), [defined-attribute lookup](/home/holbech/repos/avd/python-avd/pyavd/_schema/models/avd_model.py:157), [inheritance](/home/holbech/repos/avd/python-avd/pyavd/_schema/models/avd_model.py:341).

Consequently a generated Python annotation such as `field: str` or `field: int` is not currently a runtime guarantee for a non-primary required field. Future non-optional Rust accessors must not infer a stronger guarantee from `required` alone.

Required-key scalar strictness would also not settle null scalar **list items**: they are values under an item schema, not required dictionary keys. Their nullability needs a separate contract before generating a guaranteed `list[str]`/Rust string-item view. This ledger intentionally audits required keys, not every nullable value in the schema.

## Ordinary-design consumer outcomes

These findings concern consumers of ordinary-design fields. A cited code inspection is weaker evidence than an isolated real-method probe, and neither is a complete fabric compatibility test.

| Required field / family | Explicit-null behavior | Evidence |
| --- | --- | --- |
| `wan_path_groups[].id` | Actual helper returns 500 for `None` (except the LAN-HA special case). The Python loader preserves explicit null ID. | [fallback](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/router_path_selection.py:135) |
| `bgp_graceful_restart.enabled` | Inspected consumer's truthiness test suppresses graceful-restart generation. | [consumer](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/router_bgp.py:61) |
| `cv_topology_levels[].level` | Isolated real-method probe with a usable peer raises `TypeError` at the neighbor/device level comparison. | [return](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/cv_topology.py:66), [comparison](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/cv_topology.py:98) |
| Point-to-point endpoint ID; EVPN bundle ID | Isolated real-method probes raise `TypeError` when null IDs participate in addition. | [endpoint](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:826), [bundle](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/utils.py:403) |
| Loopback number; Zscaler cloud name | String formatting embeds the null as `LoopbackNone` or `http://gateway.None.net/vpntest`. The result is a non-null string, so stripping nulls cannot fix it. | [loopback](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/loopback_interfaces.py:46), [URL](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/monitor_connectivity.py:61) |
| Additional route-target `type` | Any value other than `import`, including null, selects export. This is a semantic error rather than necessarily printing `None`. | [branch](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:370) |
| Internal VLAN order scalars | Design uses `_cast_as` to convert a model, not a numeric cast. Partial-null output fails required-key validation after stripping; all-null output can prune the optional parent and pass. Template suppression alone is not the outcome of the validated workflow. | [model cast](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/__init__.py:224), [output observations](probe-results.json#/output_validation) |
| `fabric_name`; Zscaler domain/key salt; required multicast pool | Existing explicit checks already report a missing value on the relevant active paths. | [fabric](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/misc.py:169), [Zscaler](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/utils_wan.py:353), [pool](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/vxlan_interface.py:157) |

These examples are not a list of every risky field. The ledger contains all additional static findings. The VRF OSPF digest-key probe exposed `UnboundLocalError`: the fallback match handles interface key types, not the VRF type. The underlay OSPF probe raises `ValueError: Password is required for encryption/decryption`. These are observed late failures, not hidden test setup errors. Raw observations remain in [probe-results.json](probe-results.json). Static review additionally found that interface-level digest keys bypass the VRF key list, and full RD/RT overrides bypass EVPN bundle-ID arithmetic: the failing probes never established unconditional declaration-wide failure.

The five `demonstrated-method` declarations produce six recorded exception observations: EVPN bundle ID is probed separately through its RD and RT helpers. Exception counts must not be substituted for declaration counts; all six observations remain in the saved results.

The four initially template-only inspected declarations now have ordinary-design handoffs traced:

- `router_bgp.peer_groups[].missing_policy.direction_in.action`
- `router_bgp.peer_groups[].missing_policy.direction_out.action`
- `dot1x.aaa.unresponsive.phone_action.cached_results_timeout.time_duration`
- `dot1x.aaa.unresponsive.phone_action.cached_results_timeout.time_duration_unit`

Missing-policy actions enter via tenant/VRF peer-group casts at `network_services/router_bgp.py:101`. Both dot1x timeout variants enter via `_get_adapter_dot1x` at `connected_endpoints/utils.py:276`. Retained partial child models fail after stripping; entirely empty optional child models can be pruned. Unused profiles and unmatched peer groups also bypass emission. Source-schema labels above map to actual ordinary-design paths in the ledger.

## Override/template-sink observations

These probes use the real AVD `Templar`, repository templates and filters/tests, but exercise only individual fragments. They establish what an EOS rendering sink does when explicitly given null, **not** that an ordinary-design field reaches that sink unchanged. Override inputs are a separate scope, and generated output is stripped before custom overrides are applied.

| EOS field / family | Fragment behavior with explicit null | Evidence |
| --- | --- | --- |
| `agents[].environment_variables[].value` | Renders `agent agent-x environment TOKEN=None`. | [concatenation](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/j2templates/eos/agents-environment.j2:14) |
| `ethernet_interfaces[].link_tracking_groups[].direction` | Renders `link tracking group UPSTREAM None`. | [interpolation](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/j2templates/eos/ethernet-interfaces.j2:1134) |
| `ipv6_prefix_lists[].sequence_numbers[].action` | Renders `seq 10 None`. | [IPv6 template](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/j2templates/eos/ipv6-prefix-lists.j2:11) |
| `static_routes[].prefix` | Renders `ip route None 192.0.2.1`. | [prefix concatenation](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/j2templates/eos/static-routes.j2:20) |
| IPv4 prefix-list action | Omits the sequence, retaining the list header. | [null-aware guard](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/j2templates/eos/prefix-lists.j2:11) |
| Internal VLAN order allocation/range scalars | Suppresses the complete command when a required scalar is null. | [three-field guard](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/j2templates/eos/vlan-internal-order.j2:7) |

## The output boundary changes the risk

The final custom structured-configuration contributor first calls `_strip_empties()` on generated EOS Config, **then** merges node/root/nested/custom overrides, explicitly including null. See [merge order](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/custom_structured_configuration/__init__.py:23).

For a direct scalar copy on the generated path, null becomes an absent output field. If that field is required and its containing dictionary/item survives, output validation reports a missing required key. This ends the null-propagation trace at the strip/validation boundary; it is not successful omission or necessarily literal `None` text. If the field is optional downstream, or the entire optional parent is pruned, validation may instead succeed. Stringification or arithmetic before stripping has its own risk.

For custom overrides merged afterward, explicit scalar null can survive and be accepted as a present key by output validation. Conversely, an override can supply a generated field that stripping removed, repairing the missing-key problem. Both are separate from the unrepaired generated-output case.

### Current workflow gate

The current Ansible `eos_cli_config_gen` role **does validate EOS Config before rendering**, using `validate_inputs` with `schema_name: eos_config`. Although `fail_on_validation_errors` is false for the batch task, errors yield no validated data. Temporary paths are cleaned before validation, the worker only writes a host JSON file when validated data exists, and rendering requires that file. Missing-key errors therefore prevent rendering for that host, rather than merely allowing the template to suppress the command.

Evidence: [role validation task](/home/holbech/repos/avd/ansible_collections/arista/avd/roles/eos_cli_config_gen/tasks/main.yml:18), [clean temporary paths](/home/holbech/repos/avd/ansible_collections/arista/avd/plugins/action/validate_inputs.py:197), [validated-file emission](/home/holbech/repos/avd/ansible_collections/arista/avd/plugins/action/validate_inputs.py:578), [renderer requires file](/home/holbech/repos/avd/ansible_collections/arista/avd/plugins/action/eos_cli_config_gen.py:162), [errors yield no validated data](/home/holbech/repos/pyavd-utils/rust/python-bindings/src/validation/mod.rs:227).

Direct pyAVD calls differ: `get_device_structured_config` returns the built model without a final schema-validation pass; `get_device_config` dumps and renders it without validating again. Callers can use `validate_structured_config`, which accepts explicit null but rejects stripped missing required keys. See [builder](/home/holbech/repos/avd/python-avd/pyavd/get_device_structured_config.py:43), [rendering](/home/holbech/repos/avd/python-avd/pyavd/get_device_config.py:49), [public output validation](/home/holbech/repos/avd/python-avd/pyavd/validate_structured_config.py:38). A compatibility assessment must name the workflow rather than assume every caller has the Ansible gate.

### Observed output-boundary cases

[Eighteen synthetic cases](probe-results.json#/output_validation) exercise real EOS models, `_strip_empties()`, optional override merging and public `validate_structured_config()`. They are not complete AVD Design runs. Every direct-null case is accepted under the observed defaults; the following outcomes occur afterward:

| Output case | After stripping / final validation |
| --- | --- |
| Static route prefix null, next hop retained | Missing required prefix: rejected. |
| IPv4 or IPv6 prefix-list action null, sequence retained | Missing required action: rejected, regardless of template guards. |
| Static ARP IPv4 address or MAC null, siblings retained | Missing required address: rejected. |
| Internal VLAN allocation, beginning or ending null, siblings retained | Missing required child: rejected. |
| All internal VLAN children null | Entire optional parent pruned: accepted. |
| One dot1x username-format child null, sibling retained | Missing required delimiter/case: rejected. |
| Both dot1x username-format children null | Optional format parent pruned: accepted. |
| BFD multihop interval, min_rx or multiplier null, siblings retained | Removed field is optional downstream: accepted. |
| Optional static-route or prefix-list parent null | Parent removed: accepted. |
| Internal VLAN allocation null, then supplied by an override | Rejected immediately after stripping, accepted after override repair. |

The saved [probe context](probe-results.json#/output_validation_context) identifies the schema archive by path and SHA-256. Public validation uses default `restrict_null_values=False` and `ignore_required_keys_on_root_dict=False`. Violations are returned with `validated_data=None`, not raised as Python exceptions.

`arista.avd.defined` rejects `None`; ordinary Jinja `is defined` does not. The candidate index records complete guard statements and recognizes a small set of exact-field null-rejecting predicates. A parent-model guard does not guarantee its required child scalars are non-null.

## Collection nulls are not universally safe either

All 28 required collection declarations are assessed separately. Null can filter objects out of every device (switches, endpoint nodes), block inherited collections, expose dictionary child defaults (Digital Twin fabric), or emit incomplete output. Other paths already fail: adapter switch-port mismatch, empty WAN control-plane IPsec credentials, or in-band ZTP server selection. For on-prem CloudVision, a single cluster uses optional root `cvaddrs`, while multiple clusters use required per-cluster `cvaddrs`; the same null server list therefore has different output-validation outcomes.

Do not prescribe `[]` or `{}` as universal migrations. For example, `underlay_ospf_authentication.message_digest_keys: []` violates `min_length: 1`, even when authentication is disabled, whereas null currently skips the constraint. Null and an empty collection also differ in inheritance behavior. Each ledger entry records its applicable migration rather than assuming equivalence.

Keeping required collections nullable does not provide a universal non-null scalar accessor guarantee. A null required dictionary such as `evpn_ethernet_segment` or `device_location` skips child validation, yet its typed empty view exposes required children as unset/None. Required-only null rejection also does not reject null optional ancestors or scalar list items; consumers still need the appropriate optional-parent/phase contract.

Also, a null-created model starts falsy but attribute default access populates its `__dict__`; `AvdModel.__bool__` then becomes true without clearing `_created_from_null`. That is another reason not to use model truthiness as an unconditional null guarantee. This audit does not change that behavior.

## Decision implications

The ledger distinguishes the following patterns rather than labeling any required declaration universally safe:

| Established pattern | How it informs stricter requiredness |
| --- | --- |
| Explicit error or runtime failure before stripping | Candidate for earlier rejection on that active path; unused profiles/branches still require review. |
| Missing required output key rejected after stripping | Already rejected in the output-validated workflow; stronger candidate for earlier rejection, not a declaration-wide guarantee. |
| Successful omission, suppression or fallback | Rejection changes previously accepted behavior. |
| Optional parent/item pruned, or inheritance tombstone honored | May be accepted behavior; track separately from a missing child in a retained parent. |
| Malformed non-null output or wrong semantic branch | Bug candidate, but establish whether validation/deployment already rejects it. |
| Later override supplies the stripped field | Final output can be valid despite null in the design input. |

### What materially changes

- **Boolean suppressors:** global BGP graceful restart, CVaaS, gateway enable flags, captive portal, management eAPI, tenant flood multicast and underlay authentication often permit intentional null disable today. Explicit `false` is a practical migration for those gates and still blocks inherited true. It is not universally equivalent: SSH null omits enable whereas false emits disable, VRF graceful-restart null differs from `no_graceful_restart`, and PTP null can select a global fallback.
- **Required source, optional target:** L3 BGP peer AS, BFD multihop timers, all-active EVPN segment values, VLAN names, loopback IP, WAN VNI and IPv6 peer-group default-originate enable are not guaranteed by output requiredness. Null omission can be schema-valid while operationally wrong. This supports stricter source checks for correctness, but it is a behavior change.
- **Pruning:** AAA lockout/privilege/accounting, BGP child settings, internal VLAN order, event-handler triggers and dot1x timeouts can lose a whole optional model instead of failing. DNS has an additional boundary: pruning a null-only server succeeds only when another valid server remains in the same required server collection. Logging host protocol defaults normally retain a bad item, but explicitly nulling protocol can allow pruning.
- **Supported bypasses:** a VRF's invalid OSPF cleartext key may be unused because interface-level keys take precedence. A null EVPN bundle ID can avoid arithmetic with both complete RD and RT overrides. Tenant multicast pool can be unused with explicit VRF group settings. These are stronger compatibility examples than merely hypothesizing dormant data.
- **Shared/dormant definitions:** node, tenant, VLAN, topology, WAN-policy and profile selection can skip fields that strict upfront source validation would reject. This is a cleanup obligation even when every active null case already errors. Static/dynamic occurrences sharing the same consumer are not counted as different failure mechanisms.
- **Non-null mistakes:** `LoopbackNone`, `gateway.None.net` and null route-target direction selecting export are not fixed by output null stripping. Schema acceptance does not establish device/network acceptance; these remain concrete bug risks.

### Recommendation

Given the willingness to reject permitted misuse, the findings **do not justify abandoning required-scalar null rejection**. They do rule out presenting it as only an earlier error for configurations that cannot exist in deployments. Adopt it as an intentional breaking schema contract, with field-specific migration guidance and an appropriate release boundary.

Rejecting null on **all required fields** is also a coherent contract, but additionally removes the 28 required-collection null paths. That is a separate migration choice, not a consequence of scalar tightening. The assessment identifies the lost suppressors/tombstones and collection-specific failures; it does not measure how heavily customers use them. Keep optional null and structured-config override decisions explicit rather than silently broadening the rule.

The seven inspected active-path earlier-error candidates are TACACS/RADIUS host, IP-host IPv4 addresses, logging match-list action, logging severity, NTP hash algorithm and fabric name. Only fabric name has an unconditional direct regular-build missing-value guard; the others rely on the named consumer and final output validation. None is a blanket guarantee across custom overrides or bypassing APIs.

The existing `restrict_null_values` knob rejects null across requiredness, including **optional** fields. Simply enabling it does **not** implement either required-only proposal. Rejecting explicit null is also separate from moving missing-key checks to a new validation phase. No production change is made here.

### Remaining uncertainty, not a declaration-review backlog

Customer prevalence and operational acceptance of schema-valid malformed/incomplete outputs remain unknown. Real deployment inventories would measure migration frequency; more synthetic probes would not. Before implementation, choose the release/collection/override boundaries and verify those selected semantics with focused regressions and end-to-end builds. Future staged validation and provenance remain separate work.

If structured-config overrides are later included, use their 550-declaration appendix; its 29 unresolved metadata declarations are not extra unresolved ordinary-design fields.

## Reproduction and review workflow

Use the AVD development environment; the scripts import its existing resolver, YAML parser and Jinja environment. They are diagnostic tools, not production schema-generation code. Run from the pyavd-utils checkout:

```bash
PYTHONDONTWRITEBYTECODE=1 /home/holbech/repos/avd/.venv/bin/python reports/avd-required-null-audit/audit.py --avd /home/holbech/repos/avd
PYTHONDONTWRITEBYTECODE=1 /home/holbech/repos/avd/.venv/bin/python reports/avd-required-null-audit/consumers.py --avd /home/holbech/repos/avd
PYTHONDONTWRITEBYTECODE=1 /home/holbech/repos/avd/.venv/bin/python reports/avd-required-null-audit/probe.py --avd /home/holbech/repos/avd
PYTHONDONTWRITEBYTECODE=1 /home/holbech/repos/avd/.venv/bin/python reports/avd-required-null-audit/render_report.py
```

The first step recreates the inventory. The probe prints observations to stdout; refresh `probe-results.json` only when probe inputs/results actually change. For ordinary finding edits, update the exact-origin record in `declaration_reviews.json` and run **only the renderer**. It preserves the initial site index from `reviews.py`, applies declaration assessments, and associates saved output cases from `output_reviews.py`. It rejects a changed case set or an assessment-origin set not exactly covering the primary inventory. Do not hand-edit generated ledger rows. Update this decision summary when counts/conclusions change. The original template index has 617 cross-context limitations, not parse failures; subsequent static tracing, not that index alone, supports these assessments.

No production code, validation policy, existing test assertions, branches or staged changes were modified. No commit or push is part of this audit.
