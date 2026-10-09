# AVD Design required fields

Snapshot: `d3ef7981bdc62f68bcc4211e7d7e691505cf5c3d` in `/home/holbech/repos/avd`.

This list contains 166 distinct required non-primary declarations. The full avd_design scope has 448 occurrences, 138 scalar declarations and 28 collection declarations.

[Decision report](report.md) · [Full inventory](inventory.json) · [Design fields](fields.md) · [Open design traces](open-fields.md) · [Override appendix](structured-config-fields.md)

Only paths belonging to this scope are listed. A declaration can appear in both ledgers; override occurrences are never added to the design counts.

The source/template index remains separate from the declaration assessment. `inspected-site` is local evidence, not transitive safety. `demonstrated-method` uses an isolated real-method probe; `demonstrated-fragment` uses a template fragment. `candidate-only` describes the original automated index, not the progress of the subsequent static assessment. A default does not replace explicit scalar null. Full candidate expressions and reference chains remain in the inventory.

Classifications describe cited consumer sites, not compatibility verdicts. A template null guard does not establish successful omission: stripped required children can fail EOS Config validation. An optional parent can instead be pruned, and later overrides can repair the output. The separate ordinary-design assessment records the stricter-rule impact, not customer prevalence or EOS device acceptance.

`reviewed` means the declaration's consumer families and occurrence routing were statically assessed, not that every input combination was tested. Accepted contexts are code/schema scenarios unless a saved observation is identified. `active-path-already-rejected` is an earlier-error candidate for the inspected regular active path, not universally safe rejection. `mixed-rejection` includes both failing and permitted contexts; `behavior-changing-rejection` identifies a permitted null path. Later overrides may repair otherwise invalid generated output.

## Scalars (138 declarations)

### R001 — `aaa_settings.tacacs.servers.host`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `runtime-risk`. Use the declaration assessment below for the current finding.

Source: [aaa_settings.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/aaa_settings.schema.yml); declaration `eos_designs#/keys/aaa_settings/keys/tacacs/keys/servers/items/keys/host`.

Assessment: `reviewed`. Compatibility verdict: `active-path-already-rejected`. Output boundary: `static-analysis`.

Pattern: `runtime-failure-or-output-key`.

Stricter-rule finding: Cleartext encryption iterates null host and fails. Pre-encrypted key bypasses encryption but preserves a non-null key in output; Radius TLS enabled preserves TLS settings. Consequently a retained server loses required/primary-key host after stripping and fails EOS validation. Tacacs also preserves resolved VRF. Missing both usable key forms already raises. No server-selection filter provides a null-specific success path.

Permitted contexts from static analysis / saved observations: None established for the inspected regular active path; this is not a universal safety proof.

Migration/impact: Supply host or remove server. Later structured-config override repair is a separate global possibility, not established here.

Assessment evidence: [aaa_settings.py:85](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/aaa_settings.py:85), [aaa_settings.py:152](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/aaa_settings.py:152), [utils.py:112](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/utils.py:112)

Schema occurrence paths:

- `avd_design.aaa_settings.tacacs.servers[].host`

### R002 — `aaa_settings.radius.servers.host`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `runtime-risk`. Use the declaration assessment below for the current finding.

Source: [aaa_settings.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/aaa_settings.schema.yml); declaration `eos_designs#/keys/aaa_settings/keys/radius/keys/servers/items/keys/host`.

Assessment: `reviewed`. Compatibility verdict: `active-path-already-rejected`. Output boundary: `static-analysis`.

Pattern: `runtime-failure-or-output-key`.

Stricter-rule finding: Cleartext encryption iterates null host and fails. Pre-encrypted key bypasses encryption but preserves a non-null key in output; Radius TLS enabled preserves TLS settings. Consequently a retained server loses required/primary-key host after stripping and fails EOS validation. Tacacs also preserves resolved VRF. Missing both usable key forms already raises. No server-selection filter provides a null-specific success path.

Permitted contexts from static analysis / saved observations: None established for the inspected regular active path; this is not a universal safety proof.

Migration/impact: Supply host or remove server. Later structured-config override repair is a separate global possibility, not established here.

Assessment evidence: [aaa_settings.py:85](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/aaa_settings.py:85), [aaa_settings.py:152](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/aaa_settings.py:152), [utils.py:112](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/utils.py:112)

Schema occurrence paths:

- `avd_design.aaa_settings.radius.servers[].host`

### R003 — `aaa_authentication.policies.lockout.failure`

Type: `int`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [aaa_authentication.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/aaa_authentication.schema.yml); declaration `eos_cli_config_gen#/keys/aaa_authentication/keys/policies/keys/lockout/keys/failure`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-parent-pruning`.

Stricter-rule finding: Authentication is cast unchanged. Null is stripped; lockout retained with the other required child fails EOS validation. If both children are null and nothing retains lockout, that optional model is removed.

Permitted contexts from static analysis / saved observations: All-null optional lockout model is pruned.

Migration/impact: Provide both lockout values or remove the lockout parent; remove only when no inheritance tombstone is needed.

Assessment evidence: [aaa_settings.py:173](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/aaa_settings.py:173)

Schema occurrence paths:

- `avd_design.aaa_settings.authentication.policies.lockout.failure`

### R004 — `aaa_authentication.policies.lockout.duration`

Type: `int`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [aaa_authentication.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/aaa_authentication.schema.yml); declaration `eos_cli_config_gen#/keys/aaa_authentication/keys/policies/keys/lockout/keys/duration`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-parent-pruning`.

Stricter-rule finding: Authentication is cast unchanged. Null is stripped; lockout retained with the other required child fails EOS validation. If both children are null and nothing retains lockout, that optional model is removed.

Permitted contexts from static analysis / saved observations: All-null optional lockout model is pruned.

Migration/impact: Provide both lockout values or remove the lockout parent; remove only when no inheritance tombstone is needed.

Assessment evidence: [aaa_settings.py:173](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/aaa_settings.py:173)

Schema occurrence paths:

- `avd_design.aaa_settings.authentication.policies.lockout.duration`

### R005 — `aaa_authorization.commands.privilege.level`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [aaa_authorization.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/aaa_authorization.schema.yml); declaration `eos_cli_config_gen#/keys/aaa_authorization/keys/commands/keys/privilege/items/keys/level`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-item-pruning`.

Stricter-rule finding: Authorization privilege items are cast unchanged. A remaining level/default/sibling retains an incomplete item and fails EOS requiredness; an all-null unindexed item is removed.

Permitted contexts from static analysis / saved observations: All-null privilege item is pruned; list has no item primary key.

Migration/impact: Remove disabled privilege items or specify both level and default.

Assessment evidence: [aaa_settings.py:181](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/aaa_settings.py:181)

Schema occurrence paths:

- `avd_design.aaa_settings.authorization.commands.privilege[].level`

### R006 — `aaa_authorization.commands.privilege.default`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [aaa_authorization.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/aaa_authorization.schema.yml); declaration `eos_cli_config_gen#/keys/aaa_authorization/keys/commands/keys/privilege/items/keys/default`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-item-pruning`.

Stricter-rule finding: Authorization privilege items are cast unchanged. A remaining level/default/sibling retains an incomplete item and fails EOS requiredness; an all-null unindexed item is removed.

Permitted contexts from static analysis / saved observations: All-null privilege item is pruned; list has no item primary key.

Migration/impact: Remove disabled privilege items or specify both level and default.

Assessment evidence: [aaa_settings.py:181](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/aaa_settings.py:181)

Schema occurrence paths:

- `avd_design.aaa_settings.authorization.commands.privilege[].default`

### R007 — `aaa_accounting.exec.console.type`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [aaa_accounting.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/aaa_accounting.schema.yml); declaration `eos_cli_config_gen#/keys/aaa_accounting/keys/exec/keys/console/keys/type`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-parent-pruning`.

Stricter-rule finding: Accounting is cast unchanged. Stripped type fails EOS requiredness when methods, level or another sibling retains its parent/item. Empty optional console/default/system models and unindexed command items can instead disappear.

Permitted contexts from static analysis / saved observations: Null type with no retaining content is removed with its parent/item.

Migration/impact: Use the valid string 'none' for explicit accounting disable where appropriate; otherwise remove the unused parent. YAML null is not the string 'none'.

Assessment evidence: [aaa_settings.py:189](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/aaa_settings.py:189)

Schema occurrence paths:

- `avd_design.aaa_settings.accounting.exec.console.type`

### R008 — `$defs.methods.method`

Type: `str`. Scope occurrences: 5. Other-scope occurrences excluded: 84. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [defs_aaa_methods.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/defs_aaa_methods.schema.yml); declaration `eos_cli_config_gen#/$defs/methods/items/keys/method`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-item-pruning`.

Stricter-rule finding: All five accounting method lists are copied unchanged. A null-only method item is removed; a retained group/multicast item without method fails requiredness. Removal of the final empty methods list removes that optional list rather than leaving [] for min_length validation.

Permitted contexts from static analysis / saved observations: Null-only method items disappear; optional methods collection may disappear too.

Migration/impact: Specify a valid method (including literal 'none' where supported), or remove unused entries. Do not assume [] satisfies min_length:1.

Assessment evidence: [aaa_settings.py:189](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/aaa_settings.py:189)

Schema occurrence paths:

- `avd_design.aaa_settings.accounting.exec.console.methods[].method`
- `avd_design.aaa_settings.accounting.exec.default.methods[].method`
- `avd_design.aaa_settings.accounting.system.default.methods[].method`
- `avd_design.aaa_settings.accounting.commands.console[].methods[].method`
- `avd_design.aaa_settings.accounting.commands.default[].methods[].method`

### R009 — `aaa_accounting.exec.default.type`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [aaa_accounting.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/aaa_accounting.schema.yml); declaration `eos_cli_config_gen#/keys/aaa_accounting/keys/exec/keys/default/keys/type`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-parent-pruning`.

Stricter-rule finding: Accounting is cast unchanged. Stripped type fails EOS requiredness when methods, level or another sibling retains its parent/item. Empty optional console/default/system models and unindexed command items can instead disappear.

Permitted contexts from static analysis / saved observations: Null type with no retaining content is removed with its parent/item.

Migration/impact: Use the valid string 'none' for explicit accounting disable where appropriate; otherwise remove the unused parent. YAML null is not the string 'none'.

Assessment evidence: [aaa_settings.py:189](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/aaa_settings.py:189)

Schema occurrence paths:

- `avd_design.aaa_settings.accounting.exec.default.type`

### R010 — `aaa_accounting.system.default.type`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [aaa_accounting.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/aaa_accounting.schema.yml); declaration `eos_cli_config_gen#/keys/aaa_accounting/keys/system/keys/default/keys/type`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-parent-pruning`.

Stricter-rule finding: Accounting is cast unchanged. Stripped type fails EOS requiredness when methods, level or another sibling retains its parent/item. Empty optional console/default/system models and unindexed command items can instead disappear.

Permitted contexts from static analysis / saved observations: Null type with no retaining content is removed with its parent/item.

Migration/impact: Use the valid string 'none' for explicit accounting disable where appropriate; otherwise remove the unused parent. YAML null is not the string 'none'.

Assessment evidence: [aaa_settings.py:189](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/aaa_settings.py:189)

Schema occurrence paths:

- `avd_design.aaa_settings.accounting.system.default.type`

### R011 — `aaa_accounting.commands.console.type`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [aaa_accounting.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/aaa_accounting.schema.yml); declaration `eos_cli_config_gen#/keys/aaa_accounting/keys/commands/keys/console/items/keys/type`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-parent-pruning`.

Stricter-rule finding: Accounting is cast unchanged. Stripped type fails EOS requiredness when methods, level or another sibling retains its parent/item. Empty optional console/default/system models and unindexed command items can instead disappear.

Permitted contexts from static analysis / saved observations: Null type with no retaining content is removed with its parent/item.

Migration/impact: Use the valid string 'none' for explicit accounting disable where appropriate; otherwise remove the unused parent. YAML null is not the string 'none'.

Assessment evidence: [aaa_settings.py:189](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/aaa_settings.py:189)

Schema occurrence paths:

- `avd_design.aaa_settings.accounting.commands.console[].type`

### R012 — `aaa_accounting.commands.default.type`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [aaa_accounting.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/aaa_accounting.schema.yml); declaration `eos_cli_config_gen#/keys/aaa_accounting/keys/commands/keys/default/items/keys/type`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-parent-pruning`.

Stricter-rule finding: Accounting is cast unchanged. Stripped type fails EOS requiredness when methods, level or another sibling retains its parent/item. Empty optional console/default/system models and unindexed command items can instead disappear.

Permitted contexts from static analysis / saved observations: Null type with no retaining content is removed with its parent/item.

Migration/impact: Use the valid string 'none' for explicit accounting disable where appropriate; otherwise remove the unused parent. YAML null is not the string 'none'.

Assessment evidence: [aaa_settings.py:189](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/aaa_settings.py:189)

Schema occurrence paths:

- `avd_design.aaa_settings.accounting.commands.default[].type`

### R013 — `address_locking.leases.ip`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [address_locking.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/address_locking.schema.yml); declaration `eos_cli_config_gen#/keys/address_locking/keys/leases/items/keys/ip`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `platform-filter-or-pruning`.

Stricter-rule finding: On supported platforms leases are cast directly. A sibling IP/MAC retains the lease and missing required child fails EOS validation. An all-null unindexed lease is removed. Unsupported address-locking platforms return before the handoff.

Permitted contexts from static analysis / saved observations: Unsupported platform does not generate leases. All-null optional unindexed lease item pruned.

Migration/impact: Supply both lease values or remove unused lease; check whether shared inventories rely on platform filtering.

Assessment evidence: [address_locking.py:27](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/address_locking.py:27), [address_locking.py:53](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/address_locking.py:53)

Schema occurrence paths:

- `avd_design.address_locking_settings.leases[].ip`

### R014 — `address_locking.leases.mac`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [address_locking.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/address_locking.schema.yml); declaration `eos_cli_config_gen#/keys/address_locking/keys/leases/items/keys/mac`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `platform-filter-or-pruning`.

Stricter-rule finding: On supported platforms leases are cast directly. A sibling IP/MAC retains the lease and missing required child fails EOS validation. An all-null unindexed lease is removed. Unsupported address-locking platforms return before the handoff.

Permitted contexts from static analysis / saved observations: Unsupported platform does not generate leases. All-null optional unindexed lease item pruned.

Migration/impact: Supply both lease values or remove unused lease; check whether shared inventories rely on platform filtering.

Assessment evidence: [address_locking.py:27](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/address_locking.py:27), [address_locking.py:53](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/address_locking.py:53)

Schema occurrence paths:

- `avd_design.address_locking_settings.leases[].mac`

### R015 — `application_traffic_recognition.categories.applications.name`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [application_traffic_recognition.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/application_traffic_recognition.schema.yml); declaration `eos_cli_config_gen#/keys/application_traffic_recognition/keys/categories/items/keys/applications/items/keys/name`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `catalog-selection-or-pruning`.

Stricter-rule finding: Selected profiles/categories retain application references unchanged. Null name does not match a user application lookup; the null-only unindexed reference is later stripped. A surviving sibling field leaves a required-name error; unused catalog entries are not emitted.

Permitted contexts from static analysis / saved observations: Null-only application reference is pruned. Unselected profile/category is not generated.

Migration/impact: Remove unused reference or name the application; optional service inheritance may still need explicit tombstones.

Assessment evidence: [application_traffic_recognition.py:57](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/application_traffic_recognition.py:57), [application_traffic_recognition.py:96](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/application_traffic_recognition.py:96), [application_traffic_recognition.py:106](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/application_traffic_recognition.py:106)

Schema occurrence paths:

- `avd_design.application_classification.categories[].applications[].name`

### R016 — `application_traffic_recognition.application_profiles.applications.name`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [application_traffic_recognition.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/application_traffic_recognition.schema.yml); declaration `eos_cli_config_gen#/keys/application_traffic_recognition/keys/application_profiles/items/keys/applications/items/keys/name`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `catalog-selection-or-pruning`.

Stricter-rule finding: Selected profiles/categories retain application references unchanged. Null name does not match a user application lookup; the null-only unindexed reference is later stripped. A surviving sibling field leaves a required-name error; unused catalog entries are not emitted.

Permitted contexts from static analysis / saved observations: Null-only application reference is pruned. Unselected profile/category is not generated.

Migration/impact: Remove unused reference or name the application; optional service inheritance may still need explicit tombstones.

Assessment evidence: [application_traffic_recognition.py:57](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/application_traffic_recognition.py:57), [application_traffic_recognition.py:96](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/application_traffic_recognition.py:96), [application_traffic_recognition.py:106](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/application_traffic_recognition.py:106)

Schema occurrence paths:

- `avd_design.application_classification.application_profiles[].applications[].name`

### R017 — `application_traffic_recognition.application_profiles.categories.name`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [application_traffic_recognition.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/application_traffic_recognition.schema.yml); declaration `eos_cli_config_gen#/keys/application_traffic_recognition/keys/application_profiles/items/keys/categories/items/keys/name`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `inactive-catalog`.

Stricter-rule finding: Selected application profile iterates category references before stripping; null name fails the category existence check. Unselected profiles never reach this lookup.

Permitted contexts from static analysis / saved observations: Unselected application profile containing the category reference.

Migration/impact: Remove unused reference/profile or use a known category.

Assessment evidence: [application_traffic_recognition.py:45](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/application_traffic_recognition.py:45), [application_traffic_recognition.py:85](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/application_traffic_recognition.py:85)

Schema occurrence paths:

- `avd_design.application_classification.application_profiles[].categories[].name`

### R018 — `bfd_multihop.interval`

Type: `int`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `handoff`. Use the declaration assessment below for the current finding.

Source: [bfd_multihop.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/bfd_multihop.schema.yml); declaration `eos_designs#/keys/bfd_multihop/keys/interval`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `probed-output-boundary`.

Pattern: `optional-downstream-field`.

Stricter-rule finding: BFD multihop is cast to EOS BFD multihop except on overlay CVX. The downstream scalar fields are optional, so stripping null omits tuning and passes output validation even with other timers retained.

Permitted contexts from static analysis / saved observations: Partial timer configuration with null field omitted downstream. Overlay CVX bypasses multihop generation.

Migration/impact: Specify desired tuning, or remove unused optional parent; dropping scalar null may reinstate inherited settings.

Assessment evidence: [router_bfd.py:24](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/router_bfd.py:24), [router_bfd.py:27](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/router_bfd.py:27)

- Output probe `bfd_multihop_interval_none` → `router_bfd.multihop.interval` (target required: False): after stripping **accepted**; override applied: False; final output **accepted**. [Observation](probe-results.json#/output_validation/bfd_multihop_interval_none).

Synthetic output-boundary probes support the inspected handoff, not an end-to-end design run or a safe-rejection verdict.

Schema occurrence paths:

- `avd_design.bfd_multihop.interval`

### R019 — `bfd_multihop.min_rx`

Type: `int`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `handoff`. Use the declaration assessment below for the current finding.

Source: [bfd_multihop.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/bfd_multihop.schema.yml); declaration `eos_designs#/keys/bfd_multihop/keys/min_rx`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `probed-output-boundary`.

Pattern: `optional-downstream-field`.

Stricter-rule finding: BFD multihop is cast to EOS BFD multihop except on overlay CVX. The downstream scalar fields are optional, so stripping null omits tuning and passes output validation even with other timers retained.

Permitted contexts from static analysis / saved observations: Partial timer configuration with null field omitted downstream. Overlay CVX bypasses multihop generation.

Migration/impact: Specify desired tuning, or remove unused optional parent; dropping scalar null may reinstate inherited settings.

Assessment evidence: [router_bfd.py:24](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/router_bfd.py:24), [router_bfd.py:27](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/router_bfd.py:27)

- Output probe `bfd_multihop_min_rx_none` → `router_bfd.multihop.min_rx` (target required: False): after stripping **accepted**; override applied: False; final output **accepted**. [Observation](probe-results.json#/output_validation/bfd_multihop_min_rx_none).

Synthetic output-boundary probes support the inspected handoff, not an end-to-end design run or a safe-rejection verdict.

Schema occurrence paths:

- `avd_design.bfd_multihop.min_rx`

### R020 — `bfd_multihop.multiplier`

Type: `int`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `handoff`. Use the declaration assessment below for the current finding.

Source: [bfd_multihop.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/bfd_multihop.schema.yml); declaration `eos_designs#/keys/bfd_multihop/keys/multiplier`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `probed-output-boundary`.

Pattern: `optional-downstream-field`.

Stricter-rule finding: BFD multihop is cast to EOS BFD multihop except on overlay CVX. The downstream scalar fields are optional, so stripping null omits tuning and passes output validation even with other timers retained.

Permitted contexts from static analysis / saved observations: Partial timer configuration with null field omitted downstream. Overlay CVX bypasses multihop generation.

Migration/impact: Specify desired tuning, or remove unused optional parent; dropping scalar null may reinstate inherited settings.

Assessment evidence: [router_bfd.py:24](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/router_bfd.py:24), [router_bfd.py:27](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/router_bfd.py:27)

- Output probe `bfd_multihop_multiplier_none` → `router_bfd.multihop.multiplier` (target required: False): after stripping **accepted**; override applied: False; final output **accepted**. [Observation](probe-results.json#/output_validation/bfd_multihop_multiplier_none).

Synthetic output-boundary probes support the inspected handoff, not an end-to-end design run or a safe-rejection verdict.

Schema occurrence paths:

- `avd_design.bfd_multihop.multiplier`

### R021 — `router_bgp.distance.external_routes`

Type: `int`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-null-guard`. Use the declaration assessment below for the current finding.

Source: [router_bgp.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/router_bgp.schema.yml); declaration `eos_cli_config_gen#/keys/router_bgp/keys/distance/keys/external_routes`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-parent-pruning`.

Stricter-rule finding: BGP distance is handed through when BGP exists. Missing child fails requiredness if another distance remains; all three null children leave no distance model. No BGP AS returns before the handoff.

Permitted contexts from static analysis / saved observations: All-null optional distance model pruned. No BGP AS: distance input not used.

Migration/impact: Supply all distances or remove unused distance parent; inspect inheritance before dropping explicit null.

Assessment evidence: [router_bgp.py:30](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/router_bgp.py:30), [router_bgp.py:49](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/router_bgp.py:49)

Schema occurrence paths:

- `avd_design.bgp_distance.external_routes`

### R022 — `router_bgp.distance.internal_routes`

Type: `int`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-null-guard`. Use the declaration assessment below for the current finding.

Source: [router_bgp.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/router_bgp.schema.yml); declaration `eos_cli_config_gen#/keys/router_bgp/keys/distance/keys/internal_routes`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-parent-pruning`.

Stricter-rule finding: BGP distance is handed through when BGP exists. Missing child fails requiredness if another distance remains; all three null children leave no distance model. No BGP AS returns before the handoff.

Permitted contexts from static analysis / saved observations: All-null optional distance model pruned. No BGP AS: distance input not used.

Migration/impact: Supply all distances or remove unused distance parent; inspect inheritance before dropping explicit null.

Assessment evidence: [router_bgp.py:30](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/router_bgp.py:30), [router_bgp.py:49](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/router_bgp.py:49)

Schema occurrence paths:

- `avd_design.bgp_distance.internal_routes`

### R023 — `router_bgp.distance.local_routes`

Type: `int`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-null-guard`. Use the declaration assessment below for the current finding.

Source: [router_bgp.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/router_bgp.schema.yml); declaration `eos_cli_config_gen#/keys/router_bgp/keys/distance/keys/local_routes`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-parent-pruning`.

Stricter-rule finding: BGP distance is handed through when BGP exists. Missing child fails requiredness if another distance remains; all three null children leave no distance model. No BGP AS returns before the handoff.

Permitted contexts from static analysis / saved observations: All-null optional distance model pruned. No BGP AS: distance input not used.

Migration/impact: Supply all distances or remove unused distance parent; inspect inheritance before dropping explicit null.

Assessment evidence: [router_bgp.py:30](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/router_bgp.py:30), [router_bgp.py:49](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/router_bgp.py:49)

Schema occurrence paths:

- `avd_design.bgp_distance.local_routes`

### R024 — `bgp_graceful_restart.enabled`

Type: `bool`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `null-suppresses`. Use the declaration assessment below for the current finding.

Source: [bgp_graceful_restart.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/bgp_graceful_restart.schema.yml); declaration `eos_designs#/keys/bgp_graceful_restart/keys/enabled`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `boolean-suppression`.

Stricter-rule finding: When BGP exists, None is falsy and skips graceful-restart generation, like false. Default false does not override explicit null.

Permitted contexts from static analysis / saved observations: BGP graceful restart disabled by null. No BGP AS: unused setting.

Migration/impact: Use explicit false for disable; unlike deleting the field this continues blocking inherited true.

Assessment evidence: [router_bgp.py:30](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/router_bgp.py:30), [router_bgp.py:61](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/router_bgp.py:61)

Previously confirmed observation: Earlier focused comparison: null and false both skipped graceful-restart generation.

Schema occurrence paths:

- `avd_design.bgp_graceful_restart.enabled`

### R026 — `peer_filters.sequence_numbers.match`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [peer_filters.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/peer_filters.schema.yml); declaration `eos_cli_config_gen#/keys/peer_filters/items/keys/sequence_numbers/items/keys/match`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `inactive-catalog`.

Stricter-rule finding: Referenced peer filter is copied from the catalog. Its name/sequence primary keys retain parents: null sequence_numbers is removed and violates required collection; null match leaves a sequence without required match. Unused peer filters are not copied.

Permitted contexts from static analysis / saved observations: Unreferenced peer-filter catalog entry.

Migration/impact: Repair selected filters; remove unused invalid entries. [] does not restore a stripped required collection after empty stripping.

Assessment evidence: [router_bgp.py:876](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:876)

Schema occurrence paths:

- `avd_design.bgp_peer_filters_catalog[].sequence_numbers[].match`

### R027 — `router_bgp.peer_groups.bfd_timers.interval`

Type: `int`. Scope occurrences: 6. Other-scope occurrences excluded: 24. Initial consumer index: `candidate-only` / `candidate-null-guard`. Use the declaration assessment below for the current finding.

Source: [router_bgp.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/router_bgp.schema.yml); declaration `eos_cli_config_gen#/keys/router_bgp/keys/peer_groups/items/keys/bfd_timers/keys/interval`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-parent-pruning-or-selection`.

Stricter-rule finding: Tenant/VRF selected peer groups cast BFD timers unchanged; WAN overlay and WAN RR groups explicitly copy each timer scalar (not whole group casting), materializing AVD defaults only for absent fields, not explicit null. A retained partial BFD parent fails requiredness; all three null children prune optional bfd_timers. All six ordinary occurrences enter these shared consumer families.

Permitted contexts from static analysis / saved observations: Unselected tenant/VRF peer group or inactive WAN overlay. All-null optional bfd_timers pruned.

Migration/impact: Supply complete active child settings or remove unused optional parent; explicit false is preferable to null for booleans only after checking intended disable/fallback semantics.

Assessment evidence: [router_bgp.py:101](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:101), [router_bgp.py:246](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/router_bgp.py:246), [router_bgp.py:300](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/router_bgp.py:300)

Schema occurrence paths:

- `avd_design.bgp_peer_groups.wan_overlay_peers.bfd_timers.interval`
- `avd_design.bgp_peer_groups.wan_rr_overlay_peers.bfd_timers.interval`
- `avd_design.network_services[].bgp_peer_groups[].bfd_timers.interval`
- `avd_design.network_services[].vrfs[].bgp_peer_groups[].bfd_timers.interval`
- `avd_design.<dynamic:network_services_keys.name>[].bgp_peer_groups[].bfd_timers.interval`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].bgp_peer_groups[].bfd_timers.interval`

### R028 — `router_bgp.peer_groups.bfd_timers.min_rx`

Type: `int`. Scope occurrences: 6. Other-scope occurrences excluded: 24. Initial consumer index: `candidate-only` / `candidate-null-guard`. Use the declaration assessment below for the current finding.

Source: [router_bgp.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/router_bgp.schema.yml); declaration `eos_cli_config_gen#/keys/router_bgp/keys/peer_groups/items/keys/bfd_timers/keys/min_rx`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-parent-pruning-or-selection`.

Stricter-rule finding: Tenant/VRF selected peer groups cast BFD timers unchanged; WAN overlay and WAN RR groups explicitly copy each timer scalar (not whole group casting), materializing AVD defaults only for absent fields, not explicit null. A retained partial BFD parent fails requiredness; all three null children prune optional bfd_timers. All six ordinary occurrences enter these shared consumer families.

Permitted contexts from static analysis / saved observations: Unselected tenant/VRF peer group or inactive WAN overlay. All-null optional bfd_timers pruned.

Migration/impact: Supply complete active child settings or remove unused optional parent; explicit false is preferable to null for booleans only after checking intended disable/fallback semantics.

Assessment evidence: [router_bgp.py:101](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:101), [router_bgp.py:246](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/router_bgp.py:246), [router_bgp.py:300](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/router_bgp.py:300)

Schema occurrence paths:

- `avd_design.bgp_peer_groups.wan_overlay_peers.bfd_timers.min_rx`
- `avd_design.bgp_peer_groups.wan_rr_overlay_peers.bfd_timers.min_rx`
- `avd_design.network_services[].bgp_peer_groups[].bfd_timers.min_rx`
- `avd_design.network_services[].vrfs[].bgp_peer_groups[].bfd_timers.min_rx`
- `avd_design.<dynamic:network_services_keys.name>[].bgp_peer_groups[].bfd_timers.min_rx`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].bgp_peer_groups[].bfd_timers.min_rx`

### R029 — `router_bgp.peer_groups.bfd_timers.multiplier`

Type: `int`. Scope occurrences: 6. Other-scope occurrences excluded: 24. Initial consumer index: `candidate-only` / `candidate-null-guard`. Use the declaration assessment below for the current finding.

Source: [router_bgp.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/router_bgp.schema.yml); declaration `eos_cli_config_gen#/keys/router_bgp/keys/peer_groups/items/keys/bfd_timers/keys/multiplier`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-parent-pruning-or-selection`.

Stricter-rule finding: Tenant/VRF selected peer groups cast BFD timers unchanged; WAN overlay and WAN RR groups explicitly copy each timer scalar (not whole group casting), materializing AVD defaults only for absent fields, not explicit null. A retained partial BFD parent fails requiredness; all three null children prune optional bfd_timers. All six ordinary occurrences enter these shared consumer families.

Permitted contexts from static analysis / saved observations: Unselected tenant/VRF peer group or inactive WAN overlay. All-null optional bfd_timers pruned.

Migration/impact: Supply complete active child settings or remove unused optional parent; explicit false is preferable to null for booleans only after checking intended disable/fallback semantics.

Assessment evidence: [router_bgp.py:101](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:101), [router_bgp.py:246](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/router_bgp.py:246), [router_bgp.py:300](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/router_bgp.py:300)

Schema occurrence paths:

- `avd_design.bgp_peer_groups.wan_overlay_peers.bfd_timers.multiplier`
- `avd_design.bgp_peer_groups.wan_rr_overlay_peers.bfd_timers.multiplier`
- `avd_design.network_services[].bgp_peer_groups[].bfd_timers.multiplier`
- `avd_design.network_services[].vrfs[].bgp_peer_groups[].bfd_timers.multiplier`
- `avd_design.<dynamic:network_services_keys.name>[].bgp_peer_groups[].bfd_timers.multiplier`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].bgp_peer_groups[].bfd_timers.multiplier`

### R030 — `$defs.maximum_accepted_routes.limit`

Type: `int`. Scope occurrences: 4. Other-scope occurrences excluded: 110. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [defs_bgp_maximum_accepted_routes.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/defs_bgp_maximum_accepted_routes.schema.yml); declaration `eos_cli_config_gen#/$defs/maximum_accepted_routes/keys/limit`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-parent-pruning-or-selection`.

Stricter-rule finding: Selected tenant/VRF peer-group settings are cast unchanged, including maximum_accepted_routes.limit. EOS child remains required: stripping null fails when maximum_accepted_routes retains sibling content; null-only (or all-null) optional parent is pruned instead. Node matching and referenced peer groups determine which settings reach output.

Permitted contexts from static analysis / saved observations: Null-only/all-null optional maximum_accepted_routes pruned. Unselected/nonmatching tenant or VRF peer group.

Migration/impact: Supply complete active child settings or remove unused optional parent; explicit false is preferable to null for booleans only after checking intended disable/fallback semantics.

Assessment evidence: [router_bgp.py:83](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:83), [router_bgp.py:101](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:101), [router_bgp.py:107](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:107)

Schema occurrence paths:

- `avd_design.network_services[].bgp_peer_groups[].maximum_accepted_routes.limit`
- `avd_design.network_services[].vrfs[].bgp_peer_groups[].maximum_accepted_routes.limit`
- `avd_design.<dynamic:network_services_keys.name>[].bgp_peer_groups[].maximum_accepted_routes.limit`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].bgp_peer_groups[].maximum_accepted_routes.limit`

### R031 — `router_bgp.peer_groups.missing_policy.direction_in.action`

Type: `str`. Scope occurrences: 4. Other-scope occurrences excluded: 24. Initial consumer index: `inspected-site` / `null-suppresses`. Use the declaration assessment below for the current finding.

Source: [router_bgp.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/router_bgp.schema.yml); declaration `eos_cli_config_gen#/keys/router_bgp/keys/peer_groups/items/keys/missing_policy/keys/direction_in/keys/action`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-parent-pruning-or-selection`.

Stricter-rule finding: Selected tenant/VRF peer-group settings are cast unchanged, including missing_policy.direction_in.action. EOS child remains required: stripping null fails when missing_policy.direction_in retains sibling content; null-only (or all-null) optional parent is pruned instead. Node matching and referenced peer groups determine which settings reach output.

Permitted contexts from static analysis / saved observations: Null-only/all-null optional missing_policy.direction_in pruned. Unselected/nonmatching tenant or VRF peer group.

Migration/impact: Supply complete active child settings or remove unused optional parent; explicit false is preferable to null for booleans only after checking intended disable/fallback semantics.

Assessment evidence: [router_bgp.py:83](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:83), [router_bgp.py:101](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:101), [router_bgp.py:107](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:107)

Schema occurrence paths:

- `avd_design.network_services[].bgp_peer_groups[].missing_policy.direction_in.action`
- `avd_design.network_services[].vrfs[].bgp_peer_groups[].missing_policy.direction_in.action`
- `avd_design.<dynamic:network_services_keys.name>[].bgp_peer_groups[].missing_policy.direction_in.action`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].bgp_peer_groups[].missing_policy.direction_in.action`

### R032 — `router_bgp.peer_groups.missing_policy.direction_out.action`

Type: `str`. Scope occurrences: 4. Other-scope occurrences excluded: 24. Initial consumer index: `inspected-site` / `null-suppresses`. Use the declaration assessment below for the current finding.

Source: [router_bgp.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/router_bgp.schema.yml); declaration `eos_cli_config_gen#/keys/router_bgp/keys/peer_groups/items/keys/missing_policy/keys/direction_out/keys/action`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-parent-pruning-or-selection`.

Stricter-rule finding: Selected tenant/VRF peer-group settings are cast unchanged, including missing_policy.direction_out.action. EOS child remains required: stripping null fails when missing_policy.direction_out retains sibling content; null-only (or all-null) optional parent is pruned instead. Node matching and referenced peer groups determine which settings reach output.

Permitted contexts from static analysis / saved observations: Null-only/all-null optional missing_policy.direction_out pruned. Unselected/nonmatching tenant or VRF peer group.

Migration/impact: Supply complete active child settings or remove unused optional parent; explicit false is preferable to null for booleans only after checking intended disable/fallback semantics.

Assessment evidence: [router_bgp.py:83](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:83), [router_bgp.py:101](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:101), [router_bgp.py:107](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:107)

Schema occurrence paths:

- `avd_design.network_services[].bgp_peer_groups[].missing_policy.direction_out.action`
- `avd_design.network_services[].vrfs[].bgp_peer_groups[].missing_policy.direction_out.action`
- `avd_design.<dynamic:network_services_keys.name>[].bgp_peer_groups[].missing_policy.direction_out.action`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].bgp_peer_groups[].missing_policy.direction_out.action`

### R033 — `router_bgp.peer_groups.shared_secret.profile`

Type: `str`. Scope occurrences: 4. Other-scope occurrences excluded: 24. Initial consumer index: `candidate-only` / `candidate-null-guard`. Use the declaration assessment below for the current finding.

Source: [router_bgp.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/router_bgp.schema.yml); declaration `eos_cli_config_gen#/keys/router_bgp/keys/peer_groups/items/keys/shared_secret/keys/profile`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-parent-pruning-or-selection`.

Stricter-rule finding: Selected tenant/VRF peer-group settings are cast unchanged, including shared_secret.profile. EOS child remains required: stripping null fails when shared_secret retains sibling content; null-only (or all-null) optional parent is pruned instead. Node matching and referenced peer groups determine which settings reach output.

Permitted contexts from static analysis / saved observations: Null-only/all-null optional shared_secret pruned. Unselected/nonmatching tenant or VRF peer group.

Migration/impact: Supply complete active child settings or remove unused optional parent; explicit false is preferable to null for booleans only after checking intended disable/fallback semantics.

Assessment evidence: [router_bgp.py:83](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:83), [router_bgp.py:101](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:101), [router_bgp.py:107](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:107)

Schema occurrence paths:

- `avd_design.network_services[].bgp_peer_groups[].shared_secret.profile`
- `avd_design.network_services[].vrfs[].bgp_peer_groups[].shared_secret.profile`
- `avd_design.<dynamic:network_services_keys.name>[].bgp_peer_groups[].shared_secret.profile`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].bgp_peer_groups[].shared_secret.profile`

### R034 — `router_bgp.peer_groups.shared_secret.hash_algorithm`

Type: `str`. Scope occurrences: 4. Other-scope occurrences excluded: 24. Initial consumer index: `candidate-only` / `candidate-null-guard`. Use the declaration assessment below for the current finding.

Source: [router_bgp.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/router_bgp.schema.yml); declaration `eos_cli_config_gen#/keys/router_bgp/keys/peer_groups/items/keys/shared_secret/keys/hash_algorithm`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-parent-pruning-or-selection`.

Stricter-rule finding: Selected tenant/VRF peer-group settings are cast unchanged, including shared_secret.hash_algorithm. EOS child remains required: stripping null fails when shared_secret retains sibling content; null-only (or all-null) optional parent is pruned instead. Node matching and referenced peer groups determine which settings reach output.

Permitted contexts from static analysis / saved observations: Null-only/all-null optional shared_secret pruned. Unselected/nonmatching tenant or VRF peer group.

Migration/impact: Supply complete active child settings or remove unused optional parent; explicit false is preferable to null for booleans only after checking intended disable/fallback semantics.

Assessment evidence: [router_bgp.py:83](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:83), [router_bgp.py:101](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:101), [router_bgp.py:107](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:107)

Schema occurrence paths:

- `avd_design.network_services[].bgp_peer_groups[].shared_secret.hash_algorithm`
- `avd_design.network_services[].vrfs[].bgp_peer_groups[].shared_secret.hash_algorithm`
- `avd_design.<dynamic:network_services_keys.name>[].bgp_peer_groups[].shared_secret.hash_algorithm`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].bgp_peer_groups[].shared_secret.hash_algorithm`

### R035 — `bgp_peer_groups.mlag_ipv4_vrfs_peer.name`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `handoff`. Use the declaration assessment below for the current finding.

Source: [bgp_peer_groups.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/bgp_peer_groups.schema.yml); declaration `eos_designs#/keys/bgp_peer_groups/keys/mlag_ipv4_vrfs_peer/keys/name`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `inactive-feature-or-output-key`.

Stricter-rule finding: With MLAG VRFs a null name differs from the underlay group, selecting a separate group. The generated peer-group and AF entries retain settings but lose their name primary key, causing output validation errors. Without a separate-group consumer it remains unused.

Permitted contexts from static analysis / saved observations: No MLAG VRF peering requiring this group.

Migration/impact: Supply peer-group name or remove unused group configuration.

Assessment evidence: [mlag.py:226](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/mlag.py:226), [mlag.py:58](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/structured_config_utils/mlag.py:58), [mlag.py:69](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/structured_config_utils/mlag.py:69)

Schema occurrence paths:

- `avd_design.bgp_peer_groups.mlag_ipv4_vrfs_peer.name`

### R054 — `dot1x.aaa.unresponsive.phone_action.cached_results_timeout.time_duration`

Type: `int`. Scope occurrences: 10. Other-scope occurrences excluded: 174. Initial consumer index: `inspected-site` / `template-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [dot1x.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/dot1x.schema.yml); declaration `eos_cli_config_gen#/keys/dot1x/keys/aaa/keys/unresponsive/keys/phone_action/keys/cached_results_timeout/keys/time_duration`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `profile-selection-or-pruning`.

Stricter-rule finding: Selected endpoint/network-port settings (including inherited port profiles) cast action and phone_action cached timeout into interface dot1x. Globally disabled dot1x errors for selected adapter settings, not silent success. Retained timeout with one null required child fails output validation; both-null optional timeout prunes. Unused profiles/unselected endpoints are not consumed.

Permitted contexts from static analysis / saved observations: Unused profile/unselected adapter. Both timeout children null: optional timeout parent pruned.

Migration/impact: Remove unused timeout parent or supply duration and unit; profile nulls may suppress inherited child settings.

Assessment evidence: [ethernet_interfaces.py:248](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/connected_endpoints/ethernet_interfaces.py:248), [utils.py:269](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/connected_endpoints/utils.py:269), [utils.py:276](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/connected_endpoints/utils.py:276)

Schema occurrence paths:

- `avd_design.connected_endpoints[].adapters[].dot1x.aaa.unresponsive.action.cached_results_timeout.time_duration`
- `avd_design.connected_endpoints[].adapters[].dot1x.aaa.unresponsive.phone_action.cached_results_timeout.time_duration`
- `avd_design.network_ports[].dot1x.aaa.unresponsive.action.cached_results_timeout.time_duration`
- `avd_design.network_ports[].dot1x.aaa.unresponsive.phone_action.cached_results_timeout.time_duration`
- `avd_design.port_profiles[].dot1x.aaa.unresponsive.action.cached_results_timeout.time_duration`
- `avd_design.port_profiles[].dot1x.aaa.unresponsive.phone_action.cached_results_timeout.time_duration`
- `avd_design.<dynamic:connected_endpoints_keys.key>[].adapters[].dot1x.aaa.unresponsive.action.cached_results_timeout.time_duration`
- `avd_design.<dynamic:connected_endpoints_keys.key>[].adapters[].dot1x.aaa.unresponsive.phone_action.cached_results_timeout.time_duration`
- `avd_design.<dynamic:custom_connected_endpoints_keys.key>[].adapters[].dot1x.aaa.unresponsive.action.cached_results_timeout.time_duration`
- `avd_design.<dynamic:custom_connected_endpoints_keys.key>[].adapters[].dot1x.aaa.unresponsive.phone_action.cached_results_timeout.time_duration`

### R055 — `dot1x.aaa.unresponsive.phone_action.cached_results_timeout.time_duration_unit`

Type: `str`. Scope occurrences: 10. Other-scope occurrences excluded: 174. Initial consumer index: `inspected-site` / `template-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [dot1x.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/dot1x.schema.yml); declaration `eos_cli_config_gen#/keys/dot1x/keys/aaa/keys/unresponsive/keys/phone_action/keys/cached_results_timeout/keys/time_duration_unit`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `profile-selection-or-pruning`.

Stricter-rule finding: Selected endpoint/network-port settings (including inherited port profiles) cast action and phone_action cached timeout into interface dot1x. Globally disabled dot1x errors for selected adapter settings, not silent success. Retained timeout with one null required child fails output validation; both-null optional timeout prunes. Unused profiles/unselected endpoints are not consumed.

Permitted contexts from static analysis / saved observations: Unused profile/unselected adapter. Both timeout children null: optional timeout parent pruned.

Migration/impact: Remove unused timeout parent or supply duration and unit; profile nulls may suppress inherited child settings.

Assessment evidence: [ethernet_interfaces.py:248](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/connected_endpoints/ethernet_interfaces.py:248), [utils.py:269](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/connected_endpoints/utils.py:269), [utils.py:276](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/connected_endpoints/utils.py:276)

Schema occurrence paths:

- `avd_design.connected_endpoints[].adapters[].dot1x.aaa.unresponsive.action.cached_results_timeout.time_duration_unit`
- `avd_design.connected_endpoints[].adapters[].dot1x.aaa.unresponsive.phone_action.cached_results_timeout.time_duration_unit`
- `avd_design.network_ports[].dot1x.aaa.unresponsive.action.cached_results_timeout.time_duration_unit`
- `avd_design.network_ports[].dot1x.aaa.unresponsive.phone_action.cached_results_timeout.time_duration_unit`
- `avd_design.port_profiles[].dot1x.aaa.unresponsive.action.cached_results_timeout.time_duration_unit`
- `avd_design.port_profiles[].dot1x.aaa.unresponsive.phone_action.cached_results_timeout.time_duration_unit`
- `avd_design.<dynamic:connected_endpoints_keys.key>[].adapters[].dot1x.aaa.unresponsive.action.cached_results_timeout.time_duration_unit`
- `avd_design.<dynamic:connected_endpoints_keys.key>[].adapters[].dot1x.aaa.unresponsive.phone_action.cached_results_timeout.time_duration_unit`
- `avd_design.<dynamic:custom_connected_endpoints_keys.key>[].adapters[].dot1x.aaa.unresponsive.action.cached_results_timeout.time_duration_unit`
- `avd_design.<dynamic:custom_connected_endpoints_keys.key>[].adapters[].dot1x.aaa.unresponsive.phone_action.cached_results_timeout.time_duration_unit`

### R101 — `$defs.monitor_sessions.name`

Type: `str`. Scope occurrences: 7. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `handoff`. Use the declaration assessment below for the current finding.

Source: [defs_monitor_sessions.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/defs_monitor_sessions.schema.yml); declaration `eos_designs#/$defs/monitor_sessions/items/keys/name`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `profile-selection-or-output-key`.

Stricter-rule finding: Selected monitor sessions are collected from all seven endpoint/network-port/L3-interface/profile occurrences. groupby_obj sorts names before grouping: multiple sessions including null can raise comparison TypeError (including multiple None keys); a single null-named session proceeds to a retained output item lacking name primary key and fails final validation. Unused profiles/unselected interfaces never collect sessions.

Permitted contexts from static analysis / saved observations: Unused profile or filtered endpoint/L3 interface.

Migration/impact: Supply session name or remove session entry.

Assessment evidence: [monitor_sessions.py:34](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/monitor_sessions.py:34), [monitor_sessions.py:104](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/monitor_sessions.py:104), [monitor_sessions.py:132](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/monitor_sessions.py:132), [monitor_sessions.py:165](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/monitor_sessions.py:165), [groupby.py:27](/home/holbech/repos/avd/python-avd/pyavd/_utils/groupby.py:27)

Schema occurrence paths:

- `avd_design.connected_endpoints[].adapters[].monitor_sessions[].name`
- `avd_design.network_ports[].monitor_sessions[].name`
- `avd_design.network_services[].vrfs[].l3_interfaces[].monitor_sessions[].name`
- `avd_design.port_profiles[].monitor_sessions[].name`
- `avd_design.<dynamic:connected_endpoints_keys.key>[].adapters[].monitor_sessions[].name`
- `avd_design.<dynamic:custom_connected_endpoints_keys.key>[].adapters[].monitor_sessions[].name`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].l3_interfaces[].monitor_sessions[].name`

### R102 — `$defs.adapter_config.ethernet_segment.short_esi`

Type: `str`. Scope occurrences: 5. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `null-suppresses`. Use the declaration assessment below for the current finding.

Source: [defs_adapter_config.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/defs_adapter_config.schema.yml); declaration `eos_designs#/$defs/adapter_config/keys/ethernet_segment/keys/short_esi`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `null-suppression`.

Stricter-rule finding: ESI helper returns None before string/ESI operations when no subinterface ESI overrides null short_esi. Non-EVPN/non-VTEP paths also bypass ESI. Shared adapter helper covers endpoint keys, network ports and port profiles.

Permitted contexts from static analysis / saved observations: Multihoming ESI generation suppressed by null. Subinterface ESI can take precedence over adapter value. Unused profile/non-EVPN path.

Migration/impact: Remove unused segment parent or provide ESI; deleting null may restore profile ESI, so migration is not automatically equivalent.

Assessment evidence: [utils.py:57](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/connected_endpoints/utils.py:57), [utils.py:60](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/connected_endpoints/utils.py:60)

Schema occurrence paths:

- `avd_design.connected_endpoints[].adapters[].ethernet_segment.short_esi`
- `avd_design.network_ports[].ethernet_segment.short_esi`
- `avd_design.port_profiles[].ethernet_segment.short_esi`
- `avd_design.<dynamic:connected_endpoints_keys.key>[].adapters[].ethernet_segment.short_esi`
- `avd_design.<dynamic:custom_connected_endpoints_keys.key>[].adapters[].ethernet_segment.short_esi`

### R104 — `cv_pathfinder_internet_exit_policies.type`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `runtime-risk`. Use the declaration assessment below for the current finding.

Source: [cv_pathfinder_internet_exit_policies.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/cv_pathfinder_internet_exit_policies.schema.yml); declaration `eos_designs#/keys/cv_pathfinder_internet_exit_policies/items/keys/type`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `wrong-branch-or-inactive-policy`.

Stricter-rule finding: Only type=='direct' selects direct exit; null selects Zscaler code. If reached with valid Zscaler settings/tunnels, policy metadata retains name and loses required type after stripping, failing output validation. No local policy interfaces returns before that branch.

Permitted contexts from static analysis / saved observations: Policy not assigned to any local WAN interface.

Migration/impact: Specify direct/zscaler or remove unused policy. Rejection eliminates ambiguous branch selection.

Assessment evidence: [router_internet_exit.py:66](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_internet_exit.py:66), [router_internet_exit.py:77](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_internet_exit.py:77), [metadata.py:35](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/metadata.py:35)

Schema occurrence paths:

- `avd_design.cv_pathfinder_internet_exit_policies[].type`

### R105 — `cv_pathfinder_internet_exit_policies.zscaler.ipsec_key_salt`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `explicit-error`. Use the declaration assessment below for the current finding.

Source: [cv_pathfinder_internet_exit_policies.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/cv_pathfinder_internet_exit_policies.schema.yml); declaration `eos_designs#/keys/cv_pathfinder_internet_exit_policies/items/keys/zscaler/keys/ipsec_key_salt`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `inactive-policy-or-explicit-error`.

Stricter-rule finding: Reached Zscaler credentials path truthiness-checks salt/domain and raises existing missing-variable error. Direct policy or policy without local Zscaler exit does not need these values.

Permitted contexts from static analysis / saved observations: Direct internet-exit policy with unused Zscaler settings. Unselected/no-local-interface Zscaler policy.

Migration/impact: Supply credentials for active Zscaler exit or remove dormant Zscaler block.

Assessment evidence: [utils_wan.py:353](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/utils_wan.py:353), [utils_wan.py:357](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/utils_wan.py:357), [router_internet_exit.py:66](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_internet_exit.py:66)

Schema occurrence paths:

- `avd_design.cv_pathfinder_internet_exit_policies[].zscaler.ipsec_key_salt`

### R106 — `cv_pathfinder_internet_exit_policies.zscaler.domain_name`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `explicit-error`. Use the declaration assessment below for the current finding.

Source: [cv_pathfinder_internet_exit_policies.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/cv_pathfinder_internet_exit_policies.schema.yml); declaration `eos_designs#/keys/cv_pathfinder_internet_exit_policies/items/keys/zscaler/keys/domain_name`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `inactive-policy-or-explicit-error`.

Stricter-rule finding: Reached Zscaler credentials path truthiness-checks salt/domain and raises existing missing-variable error. Direct policy or policy without local Zscaler exit does not need these values.

Permitted contexts from static analysis / saved observations: Direct internet-exit policy with unused Zscaler settings. Unselected/no-local-interface Zscaler policy.

Migration/impact: Supply credentials for active Zscaler exit or remove dormant Zscaler block.

Assessment evidence: [utils_wan.py:353](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/utils_wan.py:353), [utils_wan.py:357](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/utils_wan.py:357), [router_internet_exit.py:66](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_internet_exit.py:66)

Schema occurrence paths:

- `avd_design.cv_pathfinder_internet_exit_policies[].zscaler.domain_name`

### R107 — `router_adaptive_virtual_topology.region.id`

Type: `int`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-null-guard`. Use the declaration assessment below for the current finding.

Source: [router_adaptive_virtual_topology.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/router_adaptive_virtual_topology.schema.yml); declaration `eos_cli_config_gen#/keys/router_adaptive_virtual_topology/keys/region/keys/id`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `inactive-catalog`.

Stricter-rule finding: Selected CV Pathfinder region/site IDs are copied alongside non-null names, keeping the target model alive; stripped ID fails requiredness. Unselected regions/sites or non-CV-Pathfinder devices do not consume those IDs.

Permitted contexts from static analysis / saved observations: Unselected region/site catalog entry; non-CV-Pathfinder device.

Migration/impact: Supply IDs for selected entries, or remove unused incomplete entries.

Assessment evidence: [router_adaptive_virtual_topology.py:35](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/router_adaptive_virtual_topology.py:35), [router_adaptive_virtual_topology.py:51](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/router_adaptive_virtual_topology.py:51)

Schema occurrence paths:

- `avd_design.cv_pathfinder_regions[].id`

### R108 — `router_adaptive_virtual_topology.site.id`

Type: `int`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-null-guard`. Use the declaration assessment below for the current finding.

Source: [router_adaptive_virtual_topology.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/router_adaptive_virtual_topology.schema.yml); declaration `eos_cli_config_gen#/keys/router_adaptive_virtual_topology/keys/site/keys/id`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `inactive-catalog`.

Stricter-rule finding: Selected CV Pathfinder region/site IDs are copied alongside non-null names, keeping the target model alive; stripped ID fails requiredness. Unselected regions/sites or non-CV-Pathfinder devices do not consume those IDs.

Permitted contexts from static analysis / saved observations: Unselected region/site catalog entry; non-CV-Pathfinder device.

Migration/impact: Supply IDs for selected entries, or remove unused incomplete entries.

Assessment evidence: [router_adaptive_virtual_topology.py:35](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/router_adaptive_virtual_topology.py:35), [router_adaptive_virtual_topology.py:51](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/router_adaptive_virtual_topology.py:51)

Schema occurrence paths:

- `avd_design.cv_pathfinder_regions[].sites[].id`

### R109 — `cv_settings.cvaas.enabled`

Type: `bool`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `null-suppresses`. Use the declaration assessment below for the current finding.

Source: [cv_settings.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/cv_settings.schema.yml); declaration `eos_designs#/keys/cv_settings/keys/cvaas/keys/enabled`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `boolean-suppression`.

Stricter-rule finding: Null CVaaS enabled excludes CVaaS clusters; on-prem clusters remain independent. No active clusters returns without TerminAttr generation. Export dependencies can require CloudVision elsewhere but do not make null universally fail.

Permitted contexts from static analysis / saved observations: CVaaS suppressed; valid on-prem clusters or no CloudVision-dependent exporters.

Migration/impact: Use explicit false for intentional CVaaS disable.

Assessment evidence: [daemon_terminattr.py:38](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/daemon_terminattr.py:38), [daemon_terminattr.py:43](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/daemon_terminattr.py:43)

Schema occurrence paths:

- `avd_design.cv_settings.cvaas.enabled`

### R111 — `daemon_terminattr.custom_cv_options.flag`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [daemon_terminattr.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/daemon_terminattr.schema.yml); declaration `eos_cli_config_gen#/keys/daemon_terminattr/keys/custom_cv_options/items/keys/flag`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `feature-gating-or-pruning`.

Stricter-rule finding: With a configured CloudVision cluster custom options are cast unchanged. A value retained without required flag fails output validation; a null-only option item is pruned. No clusters returns before options are emitted.

Permitted contexts from static analysis / saved observations: No configured active clusters. Null-only unindexed custom-option item pruned.

Migration/impact: Remove unused custom option or provide flag; null is not a reliable disable when value remains.

Assessment evidence: [daemon_terminattr.py:43](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/daemon_terminattr.py:43), [daemon_terminattr.py:59](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/daemon_terminattr.py:59)

Schema occurrence paths:

- `avd_design.cv_settings.terminattr.custom_cv_options[].flag`

### R112 — `cv_topology.platform`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `null-fallback`. Use the declaration assessment below for the current finding.

Source: [cv_topology.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/cv_topology.schema.yml); declaration `eos_designs#/keys/cv_topology/items/keys/platform`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `null-fallback`.

Stricter-rule finding: Topology platform None is optional at lookup; node platform wins, otherwise platform lookup chooses default platform settings. Host lookup also depends on use_cv_topology.

Permitted contexts from static analysis / saved observations: Node-config platform supplies platform. Default platform settings used. Topology unused.

Migration/impact: Choose explicit platform or remove unused topology entry; default/platform behavior should be documented rather than assumed equivalent.

Assessment evidence: [cv_topology.py:57](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/cv_topology.py:57), [platform_mixin.py:31](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/platform_mixin.py:31), [platform_mixin.py:45](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/platform_mixin.py:45)

Previously confirmed observation: Earlier SharedUtils comparison: null topology platform selected node override or default platform settings.

Schema occurrence paths:

- `avd_design.cv_topology[].platform`

### R114 — `cv_topology_levels.level`

Type: `int`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `demonstrated-method` / `runtime-risk`. Use the declaration assessment below for the current finding.

Source: [cv_topology_levels.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/cv_topology_levels.schema.yml); declaration `eos_designs#/keys/cv_topology_levels/items/keys/level`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `inactive-catalog-or-runtime-failure`.

Stricter-rule finding: When a usable non-MLAG neighbor is compared, None level causes TypeError at neighbor_level < level. No usable neighbor/interfaces means no comparison; unused node-type level catalog never contributes links.

Permitted contexts from static analysis / saved observations: Topology disabled/unused level entry. No usable neighbor comparison.

Migration/impact: Supply numeric topology level or remove unused mapping.

Assessment evidence: [cv_topology.py:66](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/cv_topology.py:66), [cv_topology.py:76](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/cv_topology.py:76), [cv_topology.py:98](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/cv_topology.py:98)

Saved method observation: [cv_topology_null_required_level_comparison](probe-results.json#/methods/cv_topology_null_required_level_comparison).

Schema occurrence paths:

- `avd_design.cv_topology_levels[].level`

### R125 — `arp.persistent.enabled`

Type: `bool`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-condition`. Use the declaration assessment below for the current finding.

Source: [arp.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/arp.schema.yml); declaration `eos_cli_config_gen#/keys/arp/keys/persistent/keys/enabled`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-parent-pruning`.

Stricter-rule finding: Persistent ARP model is copied into output. If enabled is null and no timeout/sibling remains, optional persistent parent disappears; otherwise missing required enabled fails EOS validation.

Permitted contexts from static analysis / saved observations: Null-only persistent ARP parent pruned.

Migration/impact: Use explicit false if disabling persistence, or remove unused parent after checking inheritance.

Assessment evidence: [__init__.py:532](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/__init__.py:532)

Schema occurrence paths:

- `avd_design.general_settings.arp.persistent.enabled`

### R154 — `event_handlers.trigger_on_intf.interface`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-null-guard`. Use the declaration assessment below for the current finding.

Source: [event_handlers.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/event_handlers.schema.yml); declaration `eos_cli_config_gen#/keys/event_handlers/items/keys/trigger_on_intf/keys/interface`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-parent-pruning`.

Stricter-rule finding: Event handlers are assigned directly. Stripped interface/operation/action fails requiredness only when its trigger parent remains. Null-only trigger_on_intf or both-null trigger_on_maintenance can disappear while the handler name remains.

Permitted contexts from static analysis / saved observations: Empty optional trigger submodel pruned; handler item may remain with other settings.

Migration/impact: Supply complete trigger settings or remove unused trigger; generated schema acceptance does not prove remaining handler is useful on EOS.

Assessment evidence: [__init__.py:257](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/__init__.py:257)

Schema occurrence paths:

- `avd_design.event_handlers[].trigger_on_intf.interface`

### R155 — `event_handlers.trigger_on_maintenance.operation`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-null-guard`. Use the declaration assessment below for the current finding.

Source: [event_handlers.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/event_handlers.schema.yml); declaration `eos_cli_config_gen#/keys/event_handlers/items/keys/trigger_on_maintenance/keys/operation`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-parent-pruning`.

Stricter-rule finding: Event handlers are assigned directly. Stripped interface/operation/action fails requiredness only when its trigger parent remains. Null-only trigger_on_intf or both-null trigger_on_maintenance can disappear while the handler name remains.

Permitted contexts from static analysis / saved observations: Empty optional trigger submodel pruned; handler item may remain with other settings.

Migration/impact: Supply complete trigger settings or remove unused trigger; generated schema acceptance does not prove remaining handler is useful on EOS.

Assessment evidence: [__init__.py:257](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/__init__.py:257)

Schema occurrence paths:

- `avd_design.event_handlers[].trigger_on_maintenance.operation`

### R156 — `event_handlers.trigger_on_maintenance.action`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-null-guard`. Use the declaration assessment below for the current finding.

Source: [event_handlers.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/event_handlers.schema.yml); declaration `eos_cli_config_gen#/keys/event_handlers/items/keys/trigger_on_maintenance/keys/action`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-parent-pruning`.

Stricter-rule finding: Event handlers are assigned directly. Stripped interface/operation/action fails requiredness only when its trigger parent remains. Null-only trigger_on_intf or both-null trigger_on_maintenance can disappear while the handler name remains.

Permitted contexts from static analysis / saved observations: Empty optional trigger submodel pruned; handler item may remain with other settings.

Migration/impact: Supply complete trigger settings or remove unused trigger; generated schema acceptance does not prove remaining handler is useful on EOS.

Assessment evidence: [__init__.py:257](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/__init__.py:257)

Schema occurrence paths:

- `avd_design.event_handlers[].trigger_on_maintenance.action`

### R157 — `hardware_counters.features.name`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [hardware_counters.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/hardware_counters.schema.yml); declaration `eos_cli_config_gen#/keys/hardware_counters/keys/features/items/keys/name`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `platform-filter-and-runtime-failure`.

Stricter-rule finding: Supported hardware-counters path calls feature.name.replace before stripping; null raises AttributeError. Unsupported platforms bypass that operation and explicitly null the entire output hardware-counters model.

Permitted contexts from static analysis / saved observations: Unsupported hardware-counter platform.

Migration/impact: Remove unsupported/inactive feature entries or supply a valid feature name.

Assessment evidence: [__init__.py:165](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/__init__.py:165), [__init__.py:172](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/__init__.py:172)

Schema occurrence paths:

- `avd_design.hardware_counters.features[].name`

### R182 — `ipv6_access_lists.sequence_numbers.action`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [ipv6_access_lists.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/ipv6_access_lists.schema.yml); declaration `eos_cli_config_gen#/keys/ipv6_access_lists/items/keys/sequence_numbers/items/keys/action`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `inactive-catalog`.

Stricter-rule finding: Legacy IPv6 ACL sequence_numbers are handed through when the ACL is selected. Sequence primary key retains a null-action entry; output validation rejects missing action. Unused ACLs never reach output.

Permitted contexts from static analysis / saved observations: Unreferenced IPv6 ACL catalog.

Migration/impact: Repair or remove the incomplete sequence; empty/unused catalog entries need cleanup separately.

Assessment evidence: [utils.py:282](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/connected_endpoints/utils.py:282), [utils.py:46](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/structured_config_utils/utils.py:46), [misc.py:260](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/misc.py:260)

Schema occurrence paths:

- `avd_design.ipv6_acls[].sequence_numbers[].action`

### R206 — `logging.policy.match.match_lists.action`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [logging.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/logging.schema.yml); declaration `eos_cli_config_gen#/keys/logging/keys/policy/keys/match/keys/match_lists/items/keys/action`.

Assessment: `reviewed`. Compatibility verdict: `active-path-already-rejected`. Output boundary: `static-analysis`.

Pattern: `retained-required-output`.

Stricter-rule finding: Logging policy match-list names and logging facility primary keys retain entries copied unchanged by the contributor. Null action/severity is stripped and requiredness fails; no null-specific suppression or item filter here.

Permitted contexts from static analysis / saved observations: None established for the inspected regular active path; this is not a universal safety proof.

Migration/impact: Supply action/severity or remove the entry. Later custom override repair remains possible, but is not established for these declarations.

Assessment evidence: [logging.py:36](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/logging.py:36)

Schema occurrence paths:

- `avd_design.logging_settings.policy.match.match_lists[].action`

### R207 — `logging.level.severity`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [logging.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/logging.schema.yml); declaration `eos_cli_config_gen#/keys/logging/keys/level/items/keys/severity`.

Assessment: `reviewed`. Compatibility verdict: `active-path-already-rejected`. Output boundary: `static-analysis`.

Pattern: `retained-required-output`.

Stricter-rule finding: Logging policy match-list names and logging facility primary keys retain entries copied unchanged by the contributor. Null action/severity is stripped and requiredness fails; no null-specific suppression or item filter here.

Permitted contexts from static analysis / saved observations: None established for the inspected regular active path; this is not a universal safety proof.

Migration/impact: Supply action/severity or remove the entry. Later custom override repair remains possible, but is not established for these declarations.

Assessment evidence: [logging.py:36](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/logging.py:36)

Schema occurrence paths:

- `avd_design.logging_settings.level[].severity`

### R209 — `mac_address_table.static_entries.mac_address`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [mac_address_table.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/mac_address_table.schema.yml); declaration `eos_cli_config_gen#/keys/mac_address_table/keys/static_entries/items/keys/mac_address`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-item-pruning`.

Stricter-rule finding: Static MAC entries are assigned directly. Retaining MAC/VLAN/interface/drop leaves a missing required child after stripping; an all-null unindexed static entry is pruned.

Permitted contexts from static analysis / saved observations: All-null optional static MAC entry pruned.

Migration/impact: Supply both MAC and VLAN, or remove unused entry.

Assessment evidence: [__init__.py:400](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/__init__.py:400)

Schema occurrence paths:

- `avd_design.mac_address_table.static_entries[].mac_address`

### R210 — `mac_address_table.static_entries.vlan`

Type: `int`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [mac_address_table.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/mac_address_table.schema.yml); declaration `eos_cli_config_gen#/keys/mac_address_table/keys/static_entries/items/keys/vlan`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-item-pruning`.

Stricter-rule finding: Static MAC entries are assigned directly. Retaining MAC/VLAN/interface/drop leaves a missing required child after stripping; an all-null unindexed static entry is pruned.

Permitted contexts from static analysis / saved observations: All-null optional static MAC entry pruned.

Migration/impact: Supply both MAC and VLAN, or remove unused entry.

Assessment evidence: [__init__.py:400](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/__init__.py:400)

Schema occurrence paths:

- `avd_design.mac_address_table.static_entries[].vlan`

### R289 — `ntp.authentication_keys.hash_algorithm`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [ntp.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/ntp.schema.yml); declaration `eos_cli_config_gen#/keys/ntp/keys/authentication_keys/items/keys/hash_algorithm`.

Assessment: `reviewed`. Compatibility verdict: `active-path-already-rejected`. Output boundary: `static-analysis`.

Pattern: `retained-required-output`.

Stricter-rule finding: Both pre-encrypted and cleartext NTP key paths preserve hash_algorithm without fallback. Required ID and key retain the item; null algorithm is stripped and EOS requiredness fails. Missing both key forms already raises before output.

Permitted contexts from static analysis / saved observations: None established for the inspected regular active path; this is not a universal safety proof.

Migration/impact: Supply supported hash algorithm or remove the authentication-key entry; null is not an NTP disable.

Assessment evidence: [ntp.py:38](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/ntp.py:38), [ntp.py:49](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/ntp.py:49)

Schema occurrence paths:

- `avd_design.ntp_settings.authentication_keys[].hash_algorithm`

### R303 — `ptp.free_running.enabled`

Type: `bool`. Scope occurrences: 11. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [ptp.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/ptp.schema.yml); declaration `eos_cli_config_gen#/keys/ptp/keys/free_running/keys/enabled`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `null-fallback`.

Stricter-rule finding: When PTP is enabled and supported, node free_running.enabled is resolved with default(node, global): null node boolean selects global setting rather than necessarily disabling. Null global setting may leave a stripped optional parent, or fail if source_clock_hardware retains it. Device profiles/defaults/nodes resolve into the same node_config.

Permitted contexts from static analysis / saved observations: PTP disabled/unsupported. Node null replaced by non-null global free-running value. All-null optional free_running parent pruned.

Migration/impact: Use explicit false for disabling; removing node override may restore inheritance instead of matching null behavior.

Assessment evidence: [ptp.py:37](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/ptp.py:37), [ptp.py:69](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/ptp.py:69)

Schema occurrence paths:

- `avd_design.device_profiles[].ptp.free_running.enabled`
- `avd_design.devices[].ptp.free_running.enabled`
- `avd_design.ptp_settings.free_running.enabled`
- `avd_design.<dynamic:custom_node_type_keys.key>.defaults.ptp.free_running.enabled`
- `avd_design.<dynamic:custom_node_type_keys.key>.node_groups[].nodes[].ptp.free_running.enabled`
- `avd_design.<dynamic:custom_node_type_keys.key>.node_groups[].ptp.free_running.enabled`
- `avd_design.<dynamic:custom_node_type_keys.key>.nodes[].ptp.free_running.enabled`
- `avd_design.<dynamic:node_type_keys.key>.defaults.ptp.free_running.enabled`
- `avd_design.<dynamic:node_type_keys.key>.node_groups[].nodes[].ptp.free_running.enabled`
- `avd_design.<dynamic:node_type_keys.key>.node_groups[].ptp.free_running.enabled`
- `avd_design.<dynamic:node_type_keys.key>.nodes[].ptp.free_running.enabled`

### R326 — `queue_monitor_length.enabled`

Type: `bool`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [queue_monitor_length.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/queue_monitor_length.schema.yml); declaration `eos_cli_config_gen#/keys/queue_monitor_length/keys/enabled`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `platform-filter-or-pruning`.

Stricter-rule finding: On supported platforms queue monitor is cast unchanged. Stripped required child fails when its local parent remains; null-only optional thresholds/GRE model (or entire queue-monitor model) can disappear. Unsupported platforms bypass and apply an output null model.

Permitted contexts from static analysis / saved observations: Queue-monitor unsupported platform. Null-only optional parent pruned (enabled requires whole queue-monitor parent empty).

Migration/impact: Use explicit enabled:false where intended; supply complete retained thresholds/GRE settings or remove unused parent.

Assessment evidence: [__init__.py:275](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/__init__.py:275), [__init__.py:282](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/__init__.py:282)

Schema occurrence paths:

- `avd_design.queue_monitor_length.enabled`

### R327 — `queue_monitor_length.default_thresholds.high`

Type: `int`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [queue_monitor_length.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/queue_monitor_length.schema.yml); declaration `eos_cli_config_gen#/keys/queue_monitor_length/keys/default_thresholds/keys/high`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `platform-filter-or-pruning`.

Stricter-rule finding: On supported platforms queue monitor is cast unchanged. Stripped required child fails when its local parent remains; null-only optional thresholds/GRE model (or entire queue-monitor model) can disappear. Unsupported platforms bypass and apply an output null model.

Permitted contexts from static analysis / saved observations: Queue-monitor unsupported platform. Null-only optional parent pruned (enabled requires whole queue-monitor parent empty).

Migration/impact: Use explicit enabled:false where intended; supply complete retained thresholds/GRE settings or remove unused parent.

Assessment evidence: [__init__.py:275](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/__init__.py:275), [__init__.py:282](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/__init__.py:282)

Schema occurrence paths:

- `avd_design.queue_monitor_length.default_thresholds.high`

### R328 — `queue_monitor_length.cpu.thresholds.high`

Type: `int`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [queue_monitor_length.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/queue_monitor_length.schema.yml); declaration `eos_cli_config_gen#/keys/queue_monitor_length/keys/cpu/keys/thresholds/keys/high`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `platform-filter-or-pruning`.

Stricter-rule finding: On supported platforms queue monitor is cast unchanged. Stripped required child fails when its local parent remains; null-only optional thresholds/GRE model (or entire queue-monitor model) can disappear. Unsupported platforms bypass and apply an output null model.

Permitted contexts from static analysis / saved observations: Queue-monitor unsupported platform. Null-only optional parent pruned (enabled requires whole queue-monitor parent empty).

Migration/impact: Use explicit enabled:false where intended; supply complete retained thresholds/GRE settings or remove unused parent.

Assessment evidence: [__init__.py:275](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/__init__.py:275), [__init__.py:282](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/__init__.py:282)

Schema occurrence paths:

- `avd_design.queue_monitor_length.cpu.thresholds.high`

### R329 — `queue_monitor_length.mirror.destination.tunnel_mode_gre.source`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [queue_monitor_length.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/queue_monitor_length.schema.yml); declaration `eos_cli_config_gen#/keys/queue_monitor_length/keys/mirror/keys/destination/keys/tunnel_mode_gre/keys/source`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `platform-filter-or-pruning`.

Stricter-rule finding: On supported platforms queue monitor is cast unchanged. Stripped required child fails when its local parent remains; null-only optional thresholds/GRE model (or entire queue-monitor model) can disappear. Unsupported platforms bypass and apply an output null model.

Permitted contexts from static analysis / saved observations: Queue-monitor unsupported platform. Null-only optional parent pruned (enabled requires whole queue-monitor parent empty).

Migration/impact: Use explicit enabled:false where intended; supply complete retained thresholds/GRE settings or remove unused parent.

Assessment evidence: [__init__.py:275](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/__init__.py:275), [__init__.py:282](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/__init__.py:282)

Schema occurrence paths:

- `avd_design.queue_monitor_length.mirror.destination.tunnel_mode_gre.source`

### R330 — `queue_monitor_length.mirror.destination.tunnel_mode_gre.destination`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [queue_monitor_length.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/queue_monitor_length.schema.yml); declaration `eos_cli_config_gen#/keys/queue_monitor_length/keys/mirror/keys/destination/keys/tunnel_mode_gre/keys/destination`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `platform-filter-or-pruning`.

Stricter-rule finding: On supported platforms queue monitor is cast unchanged. Stripped required child fails when its local parent remains; null-only optional thresholds/GRE model (or entire queue-monitor model) can disappear. Unsupported platforms bypass and apply an output null model.

Permitted contexts from static analysis / saved observations: Queue-monitor unsupported platform. Null-only optional parent pruned (enabled requires whole queue-monitor parent empty).

Migration/impact: Use explicit enabled:false where intended; supply complete retained thresholds/GRE settings or remove unused parent.

Assessment evidence: [__init__.py:275](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/__init__.py:275), [__init__.py:282](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/__init__.py:282)

Schema occurrence paths:

- `avd_design.queue_monitor_length.mirror.destination.tunnel_mode_gre.destination`

### R341 — `router_adaptive_virtual_topology.profiles.metric_order.preferred_metric`

Type: `str`. Scope occurrences: 3. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [router_adaptive_virtual_topology.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/router_adaptive_virtual_topology.schema.yml); declaration `eos_cli_config_gen#/keys/router_adaptive_virtual_topology/keys/profiles/items/keys/metric_order/keys/preferred_metric`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-parent-pruning-or-selection`.

Stricter-rule finding: CV Pathfinder control-plane, application and default topology all copy metric_order via the shared profile updater. Null preferred_metric fails requiredness if secondary/sibling content retains metric_order; null-only optional model disappears. Unselected/empty application topology is skipped; legacy-autovpn does not emit AVT metric_order.

Permitted contexts from static analysis / saved observations: Empty optional metric_order pruned. Unselected topology/policy or non-CV-Pathfinder feature path.

Migration/impact: Specify valid preferred metric or remove unused metric-order parent.

Assessment evidence: [router_adaptive_virtual_topology.py:71](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_adaptive_virtual_topology.py:71), [router_adaptive_virtual_topology.py:125](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_adaptive_virtual_topology.py:125), [router_adaptive_virtual_topology.py:234](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_adaptive_virtual_topology.py:234)

Schema occurrence paths:

- `avd_design.wan_virtual_topologies.control_plane_virtual_topology.metric_order.preferred_metric`
- `avd_design.wan_virtual_topologies.policies[].application_virtual_topologies[].metric_order.preferred_metric`
- `avd_design.wan_virtual_topologies.policies[].default_virtual_topology.metric_order.preferred_metric`

### R371 — `router_bgp.address_family_ipv4.peer_groups.next_hop.address_family_ipv6.enabled`

Type: `bool`. Scope occurrences: 4. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-condition`. Use the declaration assessment below for the current finding.

Source: [router_bgp.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/router_bgp.schema.yml); declaration `eos_cli_config_gen#/keys/router_bgp/keys/address_family_ipv4/keys/peer_groups/items/keys/next_hop/keys/address_family_ipv6/keys/enabled`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-parent-pruning-or-selection`.

Stricter-rule finding: Selected tenant/VRF peer-group settings are cast unchanged, including IPv4 AF next_hop.address_family_ipv6.enabled. EOS child remains required: stripping null fails when next_hop.address_family_ipv6 retains sibling content; null-only (or all-null) optional parent is pruned instead. Node matching and referenced peer groups determine which settings reach output.

Permitted contexts from static analysis / saved observations: Null-only/all-null optional next_hop.address_family_ipv6 pruned. Unselected/nonmatching tenant or VRF peer group.

Migration/impact: Supply complete active child settings or remove unused optional parent; explicit false is preferable to null for booleans only after checking intended disable/fallback semantics.

Assessment evidence: [router_bgp.py:83](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:83), [router_bgp.py:101](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:101), [router_bgp.py:107](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:107)

Schema occurrence paths:

- `avd_design.network_services[].bgp_peer_groups[].address_family_ipv4.next_hop.address_family_ipv6.enabled`
- `avd_design.network_services[].vrfs[].bgp_peer_groups[].address_family_ipv4.next_hop.address_family_ipv6.enabled`
- `avd_design.<dynamic:network_services_keys.name>[].bgp_peer_groups[].address_family_ipv4.next_hop.address_family_ipv6.enabled`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].bgp_peer_groups[].address_family_ipv4.next_hop.address_family_ipv6.enabled`

### R431 — `router_bgp.vrfs.neighbors.bfd_timers.interval`

Type: `int`. Scope occurrences: 2. Other-scope occurrences excluded: 16. Initial consumer index: `candidate-only` / `candidate-null-guard`. Use the declaration assessment below for the current finding.

Source: [router_bgp.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/router_bgp.schema.yml); declaration `eos_cli_config_gen#/keys/router_bgp/keys/vrfs/items/keys/neighbors/items/keys/bfd_timers/keys/interval`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-parent-pruning-or-selection`.

Stricter-rule finding: Selected VRF BGP peers are cast unchanged. Retained partial BFD timer parent fails requiredness; all three null timer children prune optional bfd_timers. Node-filtered peers/non-BGP services are not emitted.

Permitted contexts from static analysis / saved observations: Node-filtered/unused BGP peer. All-null optional bfd_timers pruned.

Migration/impact: Supply complete timers or remove unused BFD timer parent.

Assessment evidence: [router_bgp.py:238](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:238), [router_bgp.py:250](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:250)

Schema occurrence paths:

- `avd_design.network_services[].vrfs[].bgp_peers[].bfd_timers.interval`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].bgp_peers[].bfd_timers.interval`

### R432 — `router_bgp.vrfs.neighbors.bfd_timers.min_rx`

Type: `int`. Scope occurrences: 2. Other-scope occurrences excluded: 16. Initial consumer index: `candidate-only` / `candidate-null-guard`. Use the declaration assessment below for the current finding.

Source: [router_bgp.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/router_bgp.schema.yml); declaration `eos_cli_config_gen#/keys/router_bgp/keys/vrfs/items/keys/neighbors/items/keys/bfd_timers/keys/min_rx`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-parent-pruning-or-selection`.

Stricter-rule finding: Selected VRF BGP peers are cast unchanged. Retained partial BFD timer parent fails requiredness; all three null timer children prune optional bfd_timers. Node-filtered peers/non-BGP services are not emitted.

Permitted contexts from static analysis / saved observations: Node-filtered/unused BGP peer. All-null optional bfd_timers pruned.

Migration/impact: Supply complete timers or remove unused BFD timer parent.

Assessment evidence: [router_bgp.py:238](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:238), [router_bgp.py:250](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:250)

Schema occurrence paths:

- `avd_design.network_services[].vrfs[].bgp_peers[].bfd_timers.min_rx`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].bgp_peers[].bfd_timers.min_rx`

### R433 — `router_bgp.vrfs.neighbors.bfd_timers.multiplier`

Type: `int`. Scope occurrences: 2. Other-scope occurrences excluded: 16. Initial consumer index: `candidate-only` / `candidate-null-guard`. Use the declaration assessment below for the current finding.

Source: [router_bgp.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/router_bgp.schema.yml); declaration `eos_cli_config_gen#/keys/router_bgp/keys/vrfs/items/keys/neighbors/items/keys/bfd_timers/keys/multiplier`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-parent-pruning-or-selection`.

Stricter-rule finding: Selected VRF BGP peers are cast unchanged. Retained partial BFD timer parent fails requiredness; all three null timer children prune optional bfd_timers. Node-filtered peers/non-BGP services are not emitted.

Permitted contexts from static analysis / saved observations: Node-filtered/unused BGP peer. All-null optional bfd_timers pruned.

Migration/impact: Supply complete timers or remove unused BFD timer parent.

Assessment evidence: [router_bgp.py:238](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:238), [router_bgp.py:250](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:250)

Schema occurrence paths:

- `avd_design.network_services[].vrfs[].bgp_peers[].bfd_timers.multiplier`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].bgp_peers[].bfd_timers.multiplier`

### R544 — `spanning_tree.port_id_allocation_port_channel_range.minimum`

Type: `int`. Scope occurrences: 11. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [spanning_tree.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/spanning_tree.schema.yml); declaration `eos_cli_config_gen#/keys/spanning_tree/keys/port_id_allocation_port_channel_range/keys/minimum`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `null-fallback-or-pruning`.

Stricter-rule finding: When L2 services are active, node range takes precedence over global range by model truthiness. Null minimum/maximum is not individually defaulted; retained partial range fails EOS validation. An all-null node range can be selected before stripping and then pruned, suppressing a global range. Device profiles/node defaults/nodes resolve through node_config.

Permitted contexts from static analysis / saved observations: No L2 services: range ignored. All-null optional selected range pruned. Global range shadowed by a non-empty node range.

Migration/impact: Provide both bounds or remove unused range; removing a node range may restore global config, unlike a null tombstone.

Assessment evidence: [__init__.py:305](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/__init__.py:305), [__init__.py:314](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/__init__.py:314), [__init__.py:329](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/__init__.py:329)

Schema occurrence paths:

- `avd_design.device_profiles[].spanning_tree_port_id_allocation_port_channel_range.minimum`
- `avd_design.devices[].spanning_tree_port_id_allocation_port_channel_range.minimum`
- `avd_design.spanning_tree_settings.port_id_allocation_port_channel_range.minimum`
- `avd_design.<dynamic:custom_node_type_keys.key>.defaults.spanning_tree_port_id_allocation_port_channel_range.minimum`
- `avd_design.<dynamic:custom_node_type_keys.key>.node_groups[].nodes[].spanning_tree_port_id_allocation_port_channel_range.minimum`
- `avd_design.<dynamic:custom_node_type_keys.key>.node_groups[].spanning_tree_port_id_allocation_port_channel_range.minimum`
- `avd_design.<dynamic:custom_node_type_keys.key>.nodes[].spanning_tree_port_id_allocation_port_channel_range.minimum`
- `avd_design.<dynamic:node_type_keys.key>.defaults.spanning_tree_port_id_allocation_port_channel_range.minimum`
- `avd_design.<dynamic:node_type_keys.key>.node_groups[].nodes[].spanning_tree_port_id_allocation_port_channel_range.minimum`
- `avd_design.<dynamic:node_type_keys.key>.node_groups[].spanning_tree_port_id_allocation_port_channel_range.minimum`
- `avd_design.<dynamic:node_type_keys.key>.nodes[].spanning_tree_port_id_allocation_port_channel_range.minimum`

### R545 — `spanning_tree.port_id_allocation_port_channel_range.maximum`

Type: `int`. Scope occurrences: 11. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `candidate-unguarded-read`. Use the declaration assessment below for the current finding.

Source: [spanning_tree.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/spanning_tree.schema.yml); declaration `eos_cli_config_gen#/keys/spanning_tree/keys/port_id_allocation_port_channel_range/keys/maximum`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `null-fallback-or-pruning`.

Stricter-rule finding: When L2 services are active, node range takes precedence over global range by model truthiness. Null minimum/maximum is not individually defaulted; retained partial range fails EOS validation. An all-null node range can be selected before stripping and then pruned, suppressing a global range. Device profiles/node defaults/nodes resolve through node_config.

Permitted contexts from static analysis / saved observations: No L2 services: range ignored. All-null optional selected range pruned. Global range shadowed by a non-empty node range.

Migration/impact: Provide both bounds or remove unused range; removing a node range may restore global config, unlike a null tombstone.

Assessment evidence: [__init__.py:305](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/__init__.py:305), [__init__.py:314](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/__init__.py:314), [__init__.py:329](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/__init__.py:329)

Schema occurrence paths:

- `avd_design.device_profiles[].spanning_tree_port_id_allocation_port_channel_range.maximum`
- `avd_design.devices[].spanning_tree_port_id_allocation_port_channel_range.maximum`
- `avd_design.spanning_tree_settings.port_id_allocation_port_channel_range.maximum`
- `avd_design.<dynamic:custom_node_type_keys.key>.defaults.spanning_tree_port_id_allocation_port_channel_range.maximum`
- `avd_design.<dynamic:custom_node_type_keys.key>.node_groups[].nodes[].spanning_tree_port_id_allocation_port_channel_range.maximum`
- `avd_design.<dynamic:custom_node_type_keys.key>.node_groups[].spanning_tree_port_id_allocation_port_channel_range.maximum`
- `avd_design.<dynamic:custom_node_type_keys.key>.nodes[].spanning_tree_port_id_allocation_port_channel_range.maximum`
- `avd_design.<dynamic:node_type_keys.key>.defaults.spanning_tree_port_id_allocation_port_channel_range.maximum`
- `avd_design.<dynamic:node_type_keys.key>.node_groups[].nodes[].spanning_tree_port_id_allocation_port_channel_range.maximum`
- `avd_design.<dynamic:node_type_keys.key>.node_groups[].spanning_tree_port_id_allocation_port_channel_range.maximum`
- `avd_design.<dynamic:node_type_keys.key>.nodes[].spanning_tree_port_id_allocation_port_channel_range.maximum`

### R574 — `$defs.node_type.defaults.evpn_gateway.d_path.local_domain_id`

Type: `str`. Scope occurrences: 10. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `null-fallback`. Use the declaration assessment below for the current finding.

Source: [defs_node_type.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/defs_node_type.schema.yml); declaration `eos_designs#/$defs/node_type/keys/defaults/keys/evpn_gateway/keys/d_path/keys/local_domain_id`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-downstream-field`.

Stricter-rule finding: Active all-active gateway selects d_path IDs with OR against deprecated domain IDs. Both old and new model definitions first raise a conflict, so explicit old-field fallback cannot be claimed as a valid migration. With no legacy fallback, null ID becomes optional EOS domain_identifier/domain_identifier_remote and is stripped without requiredness failure.

Permitted contexts from static analysis / saved observations: Optional downstream domain ID omitted. All-active gateway inactive.

Migration/impact: Supply new domain IDs or remove unused d_path block; do not combine new and old domain-ID models to repair null.

Assessment evidence: [router_bgp.py:377](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/router_bgp.py:377), [router_bgp.py:388](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/router_bgp.py:388)

Schema occurrence paths:

- `avd_design.device_profiles[].evpn_gateway.d_path.local_domain_id`
- `avd_design.devices[].evpn_gateway.d_path.local_domain_id`
- `avd_design.<dynamic:custom_node_type_keys.key>.defaults.evpn_gateway.d_path.local_domain_id`
- `avd_design.<dynamic:custom_node_type_keys.key>.node_groups[].nodes[].evpn_gateway.d_path.local_domain_id`
- `avd_design.<dynamic:custom_node_type_keys.key>.node_groups[].evpn_gateway.d_path.local_domain_id`
- `avd_design.<dynamic:custom_node_type_keys.key>.nodes[].evpn_gateway.d_path.local_domain_id`
- `avd_design.<dynamic:node_type_keys.key>.defaults.evpn_gateway.d_path.local_domain_id`
- `avd_design.<dynamic:node_type_keys.key>.node_groups[].nodes[].evpn_gateway.d_path.local_domain_id`
- `avd_design.<dynamic:node_type_keys.key>.node_groups[].evpn_gateway.d_path.local_domain_id`
- `avd_design.<dynamic:node_type_keys.key>.nodes[].evpn_gateway.d_path.local_domain_id`

### R575 — `$defs.node_type.defaults.evpn_gateway.d_path.remote_domain_id`

Type: `str`. Scope occurrences: 10. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `null-fallback`. Use the declaration assessment below for the current finding.

Source: [defs_node_type.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/defs_node_type.schema.yml); declaration `eos_designs#/$defs/node_type/keys/defaults/keys/evpn_gateway/keys/d_path/keys/remote_domain_id`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-downstream-field`.

Stricter-rule finding: Active all-active gateway selects d_path IDs with OR against deprecated domain IDs. Both old and new model definitions first raise a conflict, so explicit old-field fallback cannot be claimed as a valid migration. With no legacy fallback, null ID becomes optional EOS domain_identifier/domain_identifier_remote and is stripped without requiredness failure.

Permitted contexts from static analysis / saved observations: Optional downstream domain ID omitted. All-active gateway inactive.

Migration/impact: Supply new domain IDs or remove unused d_path block; do not combine new and old domain-ID models to repair null.

Assessment evidence: [router_bgp.py:377](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/router_bgp.py:377), [router_bgp.py:388](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/router_bgp.py:388)

Schema occurrence paths:

- `avd_design.device_profiles[].evpn_gateway.d_path.remote_domain_id`
- `avd_design.devices[].evpn_gateway.d_path.remote_domain_id`
- `avd_design.<dynamic:custom_node_type_keys.key>.defaults.evpn_gateway.d_path.remote_domain_id`
- `avd_design.<dynamic:custom_node_type_keys.key>.node_groups[].nodes[].evpn_gateway.d_path.remote_domain_id`
- `avd_design.<dynamic:custom_node_type_keys.key>.node_groups[].evpn_gateway.d_path.remote_domain_id`
- `avd_design.<dynamic:custom_node_type_keys.key>.nodes[].evpn_gateway.d_path.remote_domain_id`
- `avd_design.<dynamic:node_type_keys.key>.defaults.evpn_gateway.d_path.remote_domain_id`
- `avd_design.<dynamic:node_type_keys.key>.node_groups[].nodes[].evpn_gateway.d_path.remote_domain_id`
- `avd_design.<dynamic:node_type_keys.key>.node_groups[].evpn_gateway.d_path.remote_domain_id`
- `avd_design.<dynamic:node_type_keys.key>.nodes[].evpn_gateway.d_path.remote_domain_id`

### R576 — `$defs.node_type.defaults.evpn_gateway.all_active_multihoming.enabled`

Type: `bool`. Scope occurrences: 10. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `null-suppresses`. Use the declaration assessment below for the current finding.

Source: [defs_node_type.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/defs_node_type.schema.yml); declaration `eos_designs#/$defs/node_type/keys/defaults/keys/evpn_gateway/keys/all_active_multihoming/keys/enabled`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `boolean-suppression`.

Stricter-rule finding: All-active enabled null fails the feature truthiness gate; IPVPN enabled null makes overlay_ipvpn_gateway falsy. Node defaults/groups/nodes share resolved node_config. No downstream non-null guarantee is established by the source required flag.

Permitted contexts from static analysis / saved observations: Gateway feature disabled by null. Inactive/nonmatching node configuration.

Migration/impact: Use explicit false to disable while preserving scalar inheritance blocking.

Assessment evidence: [router_bgp.py:358](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/router_bgp.py:358), [overlay.py:165](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/overlay.py:165)

Schema occurrence paths:

- `avd_design.device_profiles[].evpn_gateway.all_active_multihoming.enabled`
- `avd_design.devices[].evpn_gateway.all_active_multihoming.enabled`
- `avd_design.<dynamic:custom_node_type_keys.key>.defaults.evpn_gateway.all_active_multihoming.enabled`
- `avd_design.<dynamic:custom_node_type_keys.key>.node_groups[].nodes[].evpn_gateway.all_active_multihoming.enabled`
- `avd_design.<dynamic:custom_node_type_keys.key>.node_groups[].evpn_gateway.all_active_multihoming.enabled`
- `avd_design.<dynamic:custom_node_type_keys.key>.nodes[].evpn_gateway.all_active_multihoming.enabled`
- `avd_design.<dynamic:node_type_keys.key>.defaults.evpn_gateway.all_active_multihoming.enabled`
- `avd_design.<dynamic:node_type_keys.key>.node_groups[].nodes[].evpn_gateway.all_active_multihoming.enabled`
- `avd_design.<dynamic:node_type_keys.key>.node_groups[].evpn_gateway.all_active_multihoming.enabled`
- `avd_design.<dynamic:node_type_keys.key>.nodes[].evpn_gateway.all_active_multihoming.enabled`

### R578 — `$defs.node_type.defaults.evpn_gateway.all_active_multihoming.evpn_ethernet_segment.identifier`

Type: `str`. Scope occurrences: 10. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `handoff`. Use the declaration assessment below for the current finding.

Source: [defs_node_type.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/defs_node_type.schema.yml); declaration `eos_designs#/$defs/node_type/keys/defaults/keys/evpn_gateway/keys/all_active_multihoming/keys/evpn_ethernet_segment/keys/identifier`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-downstream-field`.

Stricter-rule finding: Enabled all-active gateway unconditionally reads segment children and creates domain='all'. Null segment produces an empty typed model with unset children; null identifier/rt_import values are stripped. EOS identifier and route_target_import are OPTIONAL, so retained domain entry alone does not cause required-output failure.

Permitted contexts from static analysis / saved observations: Active segment values omitted from schema-valid output (device behavior unverified). Gateway inactive.

Migration/impact: Specify complete segment for active all-active multihoming; retaining collection tombstones preserves this incomplete-output risk.

Assessment evidence: [router_bgp.py:393](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/router_bgp.py:393)

Schema occurrence paths:

- `avd_design.device_profiles[].evpn_gateway.all_active_multihoming.evpn_ethernet_segment.identifier`
- `avd_design.devices[].evpn_gateway.all_active_multihoming.evpn_ethernet_segment.identifier`
- `avd_design.<dynamic:custom_node_type_keys.key>.defaults.evpn_gateway.all_active_multihoming.evpn_ethernet_segment.identifier`
- `avd_design.<dynamic:custom_node_type_keys.key>.node_groups[].nodes[].evpn_gateway.all_active_multihoming.evpn_ethernet_segment.identifier`
- `avd_design.<dynamic:custom_node_type_keys.key>.node_groups[].evpn_gateway.all_active_multihoming.evpn_ethernet_segment.identifier`
- `avd_design.<dynamic:custom_node_type_keys.key>.nodes[].evpn_gateway.all_active_multihoming.evpn_ethernet_segment.identifier`
- `avd_design.<dynamic:node_type_keys.key>.defaults.evpn_gateway.all_active_multihoming.evpn_ethernet_segment.identifier`
- `avd_design.<dynamic:node_type_keys.key>.node_groups[].nodes[].evpn_gateway.all_active_multihoming.evpn_ethernet_segment.identifier`
- `avd_design.<dynamic:node_type_keys.key>.node_groups[].evpn_gateway.all_active_multihoming.evpn_ethernet_segment.identifier`
- `avd_design.<dynamic:node_type_keys.key>.nodes[].evpn_gateway.all_active_multihoming.evpn_ethernet_segment.identifier`

### R579 — `$defs.node_type.defaults.evpn_gateway.all_active_multihoming.evpn_ethernet_segment.rt_import`

Type: `str`. Scope occurrences: 10. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `handoff`. Use the declaration assessment below for the current finding.

Source: [defs_node_type.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/defs_node_type.schema.yml); declaration `eos_designs#/$defs/node_type/keys/defaults/keys/evpn_gateway/keys/all_active_multihoming/keys/evpn_ethernet_segment/keys/rt_import`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-downstream-field`.

Stricter-rule finding: Enabled all-active gateway unconditionally reads segment children and creates domain='all'. Null segment produces an empty typed model with unset children; null identifier/rt_import values are stripped. EOS identifier and route_target_import are OPTIONAL, so retained domain entry alone does not cause required-output failure.

Permitted contexts from static analysis / saved observations: Active segment values omitted from schema-valid output (device behavior unverified). Gateway inactive.

Migration/impact: Specify complete segment for active all-active multihoming; retaining collection tombstones preserves this incomplete-output risk.

Assessment evidence: [router_bgp.py:393](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/router_bgp.py:393)

Schema occurrence paths:

- `avd_design.device_profiles[].evpn_gateway.all_active_multihoming.evpn_ethernet_segment.rt_import`
- `avd_design.devices[].evpn_gateway.all_active_multihoming.evpn_ethernet_segment.rt_import`
- `avd_design.<dynamic:custom_node_type_keys.key>.defaults.evpn_gateway.all_active_multihoming.evpn_ethernet_segment.rt_import`
- `avd_design.<dynamic:custom_node_type_keys.key>.node_groups[].nodes[].evpn_gateway.all_active_multihoming.evpn_ethernet_segment.rt_import`
- `avd_design.<dynamic:custom_node_type_keys.key>.node_groups[].evpn_gateway.all_active_multihoming.evpn_ethernet_segment.rt_import`
- `avd_design.<dynamic:custom_node_type_keys.key>.nodes[].evpn_gateway.all_active_multihoming.evpn_ethernet_segment.rt_import`
- `avd_design.<dynamic:node_type_keys.key>.defaults.evpn_gateway.all_active_multihoming.evpn_ethernet_segment.rt_import`
- `avd_design.<dynamic:node_type_keys.key>.node_groups[].nodes[].evpn_gateway.all_active_multihoming.evpn_ethernet_segment.rt_import`
- `avd_design.<dynamic:node_type_keys.key>.node_groups[].evpn_gateway.all_active_multihoming.evpn_ethernet_segment.rt_import`
- `avd_design.<dynamic:node_type_keys.key>.nodes[].evpn_gateway.all_active_multihoming.evpn_ethernet_segment.rt_import`

### R580 — `$defs.node_type.defaults.ipvpn_gateway.enabled`

Type: `bool`. Scope occurrences: 10. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `null-suppresses`. Use the declaration assessment below for the current finding.

Source: [defs_node_type.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/defs_node_type.schema.yml); declaration `eos_designs#/$defs/node_type/keys/defaults/keys/ipvpn_gateway/keys/enabled`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `boolean-suppression`.

Stricter-rule finding: All-active enabled null fails the feature truthiness gate; IPVPN enabled null makes overlay_ipvpn_gateway falsy. Node defaults/groups/nodes share resolved node_config. No downstream non-null guarantee is established by the source required flag.

Permitted contexts from static analysis / saved observations: Gateway feature disabled by null. Inactive/nonmatching node configuration.

Migration/impact: Use explicit false to disable while preserving scalar inheritance blocking.

Assessment evidence: [router_bgp.py:358](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/router_bgp.py:358), [overlay.py:165](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/overlay.py:165)

Schema occurrence paths:

- `avd_design.device_profiles[].ipvpn_gateway.enabled`
- `avd_design.devices[].ipvpn_gateway.enabled`
- `avd_design.<dynamic:custom_node_type_keys.key>.defaults.ipvpn_gateway.enabled`
- `avd_design.<dynamic:custom_node_type_keys.key>.node_groups[].nodes[].ipvpn_gateway.enabled`
- `avd_design.<dynamic:custom_node_type_keys.key>.node_groups[].ipvpn_gateway.enabled`
- `avd_design.<dynamic:custom_node_type_keys.key>.nodes[].ipvpn_gateway.enabled`
- `avd_design.<dynamic:node_type_keys.key>.defaults.ipvpn_gateway.enabled`
- `avd_design.<dynamic:node_type_keys.key>.node_groups[].nodes[].ipvpn_gateway.enabled`
- `avd_design.<dynamic:node_type_keys.key>.node_groups[].ipvpn_gateway.enabled`
- `avd_design.<dynamic:node_type_keys.key>.nodes[].ipvpn_gateway.enabled`

### R581 — `$defs.node_type.defaults.ipvpn_gateway.remote_peers.ip_address`

Type: `str`. Scope occurrences: 10. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `handoff`. Use the declaration assessment below for the current finding.

Source: [defs_node_type.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/defs_node_type.schema.yml); declaration `eos_designs#/$defs/node_type/keys/defaults/keys/ipvpn_gateway/keys/remote_peers/items/keys/ip_address`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `inactive-feature-or-output-key`.

Stricter-rule finding: Active IPVPN remote peer null IP is passed to _create_neighbor alongside peer_group/description/metadata, retaining output item while ip_address primary key is stripped. Output validation then fails. Gateway inactive bypasses peers.

Permitted contexts from static analysis / saved observations: IPVPN gateway disabled/nonmatching node configuration.

Migration/impact: Supply remote peer IP or remove unused peer.

Assessment evidence: [router_bgp.py:697](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/router_bgp.py:697), [router_bgp.py:482](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/router_bgp.py:482)

Schema occurrence paths:

- `avd_design.device_profiles[].ipvpn_gateway.remote_peers[].ip_address`
- `avd_design.devices[].ipvpn_gateway.remote_peers[].ip_address`
- `avd_design.<dynamic:custom_node_type_keys.key>.defaults.ipvpn_gateway.remote_peers[].ip_address`
- `avd_design.<dynamic:custom_node_type_keys.key>.node_groups[].nodes[].ipvpn_gateway.remote_peers[].ip_address`
- `avd_design.<dynamic:custom_node_type_keys.key>.node_groups[].ipvpn_gateway.remote_peers[].ip_address`
- `avd_design.<dynamic:custom_node_type_keys.key>.nodes[].ipvpn_gateway.remote_peers[].ip_address`
- `avd_design.<dynamic:node_type_keys.key>.defaults.ipvpn_gateway.remote_peers[].ip_address`
- `avd_design.<dynamic:node_type_keys.key>.node_groups[].nodes[].ipvpn_gateway.remote_peers[].ip_address`
- `avd_design.<dynamic:node_type_keys.key>.node_groups[].ipvpn_gateway.remote_peers[].ip_address`
- `avd_design.<dynamic:node_type_keys.key>.nodes[].ipvpn_gateway.remote_peers[].ip_address`

### R582 — `$defs.node_type.defaults.ipvpn_gateway.remote_peers.bgp_as`

Type: `str`. Scope occurrences: 10. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `handoff`. Use the declaration assessment below for the current finding.

Source: [defs_node_type.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/defs_node_type.schema.yml); declaration `eos_designs#/$defs/node_type/keys/defaults/keys/ipvpn_gateway/keys/remote_peers/items/keys/bgp_as`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-downstream-field`.

Stricter-rule finding: _create_neighbor only sets remote_as when non-null; get_asn(None) returns None and comparison may select ebgp_multihop but does not reject. EOS remote_as is optional, so a valid-IP neighbor can survive with remote AS omitted.

Permitted contexts from static analysis / saved observations: Active valid-IP peer with remote_as omitted. IPVPN gateway inactive.

Migration/impact: Supply remote AS for intended peer semantics; output schema acceptance is not evidence of functioning peering.

Assessment evidence: [router_bgp.py:702](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/router_bgp.py:702), [router_bgp.py:491](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/router_bgp.py:491), [routing.py:107](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/routing.py:107)

Schema occurrence paths:

- `avd_design.device_profiles[].ipvpn_gateway.remote_peers[].bgp_as`
- `avd_design.devices[].ipvpn_gateway.remote_peers[].bgp_as`
- `avd_design.<dynamic:custom_node_type_keys.key>.defaults.ipvpn_gateway.remote_peers[].bgp_as`
- `avd_design.<dynamic:custom_node_type_keys.key>.node_groups[].nodes[].ipvpn_gateway.remote_peers[].bgp_as`
- `avd_design.<dynamic:custom_node_type_keys.key>.node_groups[].ipvpn_gateway.remote_peers[].bgp_as`
- `avd_design.<dynamic:custom_node_type_keys.key>.nodes[].ipvpn_gateway.remote_peers[].bgp_as`
- `avd_design.<dynamic:node_type_keys.key>.defaults.ipvpn_gateway.remote_peers[].bgp_as`
- `avd_design.<dynamic:node_type_keys.key>.node_groups[].nodes[].ipvpn_gateway.remote_peers[].bgp_as`
- `avd_design.<dynamic:node_type_keys.key>.node_groups[].ipvpn_gateway.remote_peers[].bgp_as`
- `avd_design.<dynamic:node_type_keys.key>.nodes[].ipvpn_gateway.remote_peers[].bgp_as`

### R583 — `$defs.node_type_l3_interfaces.bgp.peer_as`

Type: `str`. Scope occurrences: 11. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `handoff`. Use the declaration assessment below for the current finding.

Source: [defs_node_type_l3_interfaces.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/defs_node_type_l3_interfaces.schema.yml); declaration `eos_designs#/$defs/node_type_l3_interfaces/items/keys/bgp/keys/peer_as`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-downstream-field`.

Stricter-rule finding: Shared L3 interface/port-channel IPv4 and IPv6 BGP helpers copy peer_as to optional EOS remote_as. Null is stripped without requiredness failure. No peer_ip/peer_ipv6 or no BGP block bypasses neighbor generation. Node-level profiles/defaults/nodes share the helpers.

Permitted contexts from static analysis / saved observations: Active neighbor with remote_as omitted. No selected BGP peering on this interface.

Migration/impact: Provide peer_as for intended BGP semantics or remove unused BGP block.

Assessment evidence: [misc.py:413](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/misc.py:413), [misc.py:429](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/misc.py:429), [misc.py:459](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/misc.py:459), [misc.py:471](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/misc.py:471)

Schema occurrence paths:

- `avd_design.device_profiles[].l3_interfaces[].bgp.peer_as`
- `avd_design.devices[].l3_interfaces[].bgp.peer_as`
- `avd_design.l3_interface_profiles[].bgp.peer_as`
- `avd_design.<dynamic:custom_node_type_keys.key>.defaults.l3_interfaces[].bgp.peer_as`
- `avd_design.<dynamic:custom_node_type_keys.key>.node_groups[].nodes[].l3_interfaces[].bgp.peer_as`
- `avd_design.<dynamic:custom_node_type_keys.key>.node_groups[].l3_interfaces[].bgp.peer_as`
- `avd_design.<dynamic:custom_node_type_keys.key>.nodes[].l3_interfaces[].bgp.peer_as`
- `avd_design.<dynamic:node_type_keys.key>.defaults.l3_interfaces[].bgp.peer_as`
- `avd_design.<dynamic:node_type_keys.key>.node_groups[].nodes[].l3_interfaces[].bgp.peer_as`
- `avd_design.<dynamic:node_type_keys.key>.node_groups[].l3_interfaces[].bgp.peer_as`
- `avd_design.<dynamic:node_type_keys.key>.nodes[].l3_interfaces[].bgp.peer_as`

### R584 — `$defs.node_type_l3_interfaces.static_routes.prefix`

Type: `str`. Scope occurrences: 11. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `stripped-required-output`. Use the declaration assessment below for the current finding.

Source: [defs_node_type_l3_interfaces.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/defs_node_type_l3_interfaces.schema.yml); declaration `eos_designs#/$defs/node_type_l3_interfaces/items/keys/static_routes/items/keys/prefix`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `probed-output-boundary`.

Pattern: `inactive-node-or-output-error`.

Stricter-rule finding: Node-level L3 static routes are constructed with route prefix and required peer next-hop. Null prefix is stripped while next-hop retains route, causing missing required output prefix. Node defaults/group/node/device-profile occurrences resolve via node_config; unused node configuration is not consumed.

Permitted contexts from static analysis / saved observations: Unused node/interface configuration.

Migration/impact: Supply prefix or remove route. Later overrides may repair output, but not established for this declaration.

Assessment evidence: [static_routes.py:32](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/underlay/static_routes.py:32), [static_routes.py:43](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/underlay/static_routes.py:43)

- Output probe `static_route_prefix_none` → `static_routes[].prefix` (target required: True): after stripping **rejected**; override applied: False; final output **rejected**. [Observation](probe-results.json#/output_validation/static_route_prefix_none).

Synthetic output-boundary probes support the inspected handoff, not an end-to-end design run or a safe-rejection verdict.

Schema occurrence paths:

- `avd_design.device_profiles[].l3_interfaces[].static_routes[].prefix`
- `avd_design.devices[].l3_interfaces[].static_routes[].prefix`
- `avd_design.l3_interface_profiles[].static_routes[].prefix`
- `avd_design.<dynamic:custom_node_type_keys.key>.defaults.l3_interfaces[].static_routes[].prefix`
- `avd_design.<dynamic:custom_node_type_keys.key>.node_groups[].nodes[].l3_interfaces[].static_routes[].prefix`
- `avd_design.<dynamic:custom_node_type_keys.key>.node_groups[].l3_interfaces[].static_routes[].prefix`
- `avd_design.<dynamic:custom_node_type_keys.key>.nodes[].l3_interfaces[].static_routes[].prefix`
- `avd_design.<dynamic:node_type_keys.key>.defaults.l3_interfaces[].static_routes[].prefix`
- `avd_design.<dynamic:node_type_keys.key>.node_groups[].nodes[].l3_interfaces[].static_routes[].prefix`
- `avd_design.<dynamic:node_type_keys.key>.node_groups[].l3_interfaces[].static_routes[].prefix`
- `avd_design.<dynamic:node_type_keys.key>.nodes[].l3_interfaces[].static_routes[].prefix`

### R585 — `$defs.node_type_l3_port_channels.bgp.peer_as`

Type: `str`. Scope occurrences: 10. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `handoff`. Use the declaration assessment below for the current finding.

Source: [defs_node_type_l3_port_channels.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/defs_node_type_l3_port_channels.schema.yml); declaration `eos_designs#/$defs/node_type_l3_port_channels/items/keys/bgp/keys/peer_as`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-downstream-field`.

Stricter-rule finding: Shared L3 interface/port-channel IPv4 and IPv6 BGP helpers copy peer_as to optional EOS remote_as. Null is stripped without requiredness failure. No peer_ip/peer_ipv6 or no BGP block bypasses neighbor generation. Node-level profiles/defaults/nodes share the helpers.

Permitted contexts from static analysis / saved observations: Active neighbor with remote_as omitted. No selected BGP peering on this interface.

Migration/impact: Provide peer_as for intended BGP semantics or remove unused BGP block.

Assessment evidence: [misc.py:413](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/misc.py:413), [misc.py:429](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/misc.py:429), [misc.py:459](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/misc.py:459), [misc.py:471](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/misc.py:471)

Schema occurrence paths:

- `avd_design.device_profiles[].l3_port_channels[].bgp.peer_as`
- `avd_design.devices[].l3_port_channels[].bgp.peer_as`
- `avd_design.<dynamic:custom_node_type_keys.key>.defaults.l3_port_channels[].bgp.peer_as`
- `avd_design.<dynamic:custom_node_type_keys.key>.node_groups[].nodes[].l3_port_channels[].bgp.peer_as`
- `avd_design.<dynamic:custom_node_type_keys.key>.node_groups[].l3_port_channels[].bgp.peer_as`
- `avd_design.<dynamic:custom_node_type_keys.key>.nodes[].l3_port_channels[].bgp.peer_as`
- `avd_design.<dynamic:node_type_keys.key>.defaults.l3_port_channels[].bgp.peer_as`
- `avd_design.<dynamic:node_type_keys.key>.node_groups[].nodes[].l3_port_channels[].bgp.peer_as`
- `avd_design.<dynamic:node_type_keys.key>.node_groups[].l3_port_channels[].bgp.peer_as`
- `avd_design.<dynamic:node_type_keys.key>.nodes[].l3_port_channels[].bgp.peer_as`

### R588 — `dns_settings.servers.ip_address`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `handoff`. Use the declaration assessment below for the current finding.

Source: [dns_settings.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/dns_settings.schema.yml); declaration `eos_designs#/keys/dns_settings/keys/servers/items/keys/ip_address`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `retained-required-output-or-item-pruning`.

Stricter-rule finding: Null IP with non-null priority retains incomplete server and fails requiredness/primary-key validation. Without priority, null-only server item is pruned. Its VRF name remains and requires servers, so pruning the only server still fails; a second valid server in the SAME VRF can retain required servers and allow omission. Source priority has no default.

Permitted contexts from static analysis / saved observations: Null-only server (no priority) pruned while another valid server remains in the same output VRF.

Migration/impact: Supply address or remove null server entry; do not assume removal of the only server leaves valid output.

Assessment evidence: [dns_settings.py:43](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/dns_settings.py:43), [dns_settings.py:52](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/dns_settings.py:52)

Schema occurrence paths:

- `avd_design.dns_settings.servers[].ip_address`

### R589 — `dot1x_settings.mac_based_authentication.username_format.delimiter`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `output-validation-dependent`. Use the declaration assessment below for the current finding.

Source: [dot1x_settings.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/dot1x_settings.schema.yml); declaration `eos_designs#/keys/dot1x_settings/keys/mac_based_authentication/keys/username_format/keys/delimiter`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `probed-output-boundary`.

Pattern: `optional-parent-pruning`.

Stricter-rule finding: With dot1x globally enabled, username format is mapped to required EOS delimiter/mac_string_case. One null child with retained sibling fails after stripping; both null prune optional username-format parent. Dot1x disabled bypasses the entire contributor.

Permitted contexts from static analysis / saved observations: Dot1x disabled. Both format children null: optional parent pruned. Saved output-boundary override repair case, where supplied.

Migration/impact: Supply both values or remove format parent. Deleting scalar null may restore inherited formatting.

Assessment evidence: [dot1x.py:31](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/dot1x.py:31), [dot1x.py:99](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/dot1x.py:99)

- Output probe `dot1x_username_format_all_null` → `dot1x.radius_av_pair_username_format` (target required: False): after stripping **accepted**; override applied: False; final output **accepted**. [Observation](probe-results.json#/output_validation/dot1x_username_format_all_null).
- Output probe `dot1x_username_format_delimiter_none` → `dot1x.radius_av_pair_username_format.delimiter` (target required: True): after stripping **rejected**; override applied: False; final output **rejected**. [Observation](probe-results.json#/output_validation/dot1x_username_format_delimiter_none).

Synthetic output-boundary probes support the inspected handoff, not an end-to-end design run or a safe-rejection verdict.

Schema occurrence paths:

- `avd_design.dot1x_settings.mac_based_authentication.username_format.delimiter`

### R590 — `dot1x_settings.mac_based_authentication.username_format.letter_case`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `output-validation-dependent`. Use the declaration assessment below for the current finding.

Source: [dot1x_settings.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/dot1x_settings.schema.yml); declaration `eos_designs#/keys/dot1x_settings/keys/mac_based_authentication/keys/username_format/keys/letter_case`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `probed-output-boundary`.

Pattern: `optional-parent-pruning`.

Stricter-rule finding: With dot1x globally enabled, username format is mapped to required EOS delimiter/mac_string_case. One null child with retained sibling fails after stripping; both null prune optional username-format parent. Dot1x disabled bypasses the entire contributor.

Permitted contexts from static analysis / saved observations: Dot1x disabled. Both format children null: optional parent pruned. Saved output-boundary override repair case, where supplied.

Migration/impact: Supply both values or remove format parent. Deleting scalar null may restore inherited formatting.

Assessment evidence: [dot1x.py:31](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/dot1x.py:31), [dot1x.py:99](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/dot1x.py:99)

- Output probe `dot1x_username_format_all_null` → `dot1x.radius_av_pair_username_format` (target required: False): after stripping **accepted**; override applied: False; final output **accepted**. [Observation](probe-results.json#/output_validation/dot1x_username_format_all_null).
- Output probe `dot1x_username_format_mac_string_case_none` → `dot1x.radius_av_pair_username_format.mac_string_case` (target required: True): after stripping **rejected**; override applied: False; final output **rejected**. [Observation](probe-results.json#/output_validation/dot1x_username_format_mac_string_case_none).

Synthetic output-boundary probes support the inspected handoff, not an end-to-end design run or a safe-rejection verdict.

Schema occurrence paths:

- `avd_design.dot1x_settings.mac_based_authentication.username_format.letter_case`

### R591 — `dot1x_settings.web_authentication.enabled`

Type: `bool`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `null-suppresses`. Use the declaration assessment below for the current finding.

Source: [dot1x_settings.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/dot1x_settings.schema.yml); declaration `eos_designs#/keys/dot1x_settings/keys/web_authentication/keys/enabled`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `boolean-suppression`.

Stricter-rule finding: Null web-authentication enabled returns before captive-portal configuration/dependencies. Dot1x disabled also bypasses global contributor.

Permitted contexts from static analysis / saved observations: Captive portal suppressed by null.

Migration/impact: Use explicit false for disable.

Assessment evidence: [dot1x.py:111](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/dot1x.py:111)

Schema occurrence paths:

- `avd_design.dot1x_settings.web_authentication.enabled`

### R592 — `eos_designs_custom_templates.template`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `handoff`. Use the declaration assessment below for the current finding.

Source: [eos_designs_custom_templates.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/eos_designs_custom_templates.schema.yml); declaration `eos_designs#/keys/eos_designs_custom_templates/items/keys/template`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `workflow-bypass-or-file-lookup-error`.

Stricter-rule finding: Ansible custom-template action passes null template to the file lookup; installed Ansible DataLoader warns for null then raises AnsibleFileNotFound. Direct pyAVD structured-config API has no equivalent custom-template execution, and structured_config:false skips the Ansible build action.

Permitted contexts from static analysis / saved observations: Direct pyAVD builds do not execute the action's eos_designs_custom_templates list. Ansible structured_config:false bypasses generation.

Migration/impact: Supply template path or remove unused entry. Early source validation rejects dormant template settings too.

Assessment evidence: [eos_designs_structured_config.py:109](/home/holbech/repos/avd/ansible_collections/arista/avd/plugins/action/eos_designs_structured_config.py:109), [template.py:38](/home/holbech/repos/avd/python-avd/pyavd/_utils/template.py:38), [get_device_structured_config.py:43](/home/holbech/repos/avd/python-avd/pyavd/get_device_structured_config.py:43)

Schema occurrence paths:

- `avd_design.eos_designs_custom_templates[].template`

### R593 — `evpn_vlan_bundles.id`

Type: `int`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `demonstrated-method` / `runtime-risk`. Use the declaration assessment below for the current finding.

Source: [evpn_vlan_bundles.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/evpn_vlan_bundles.schema.yml); declaration `eos_designs#/keys/evpn_vlan_bundles/items/keys/id`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `calculation-bypass-or-runtime-failure`.

Stricter-rule finding: Used bundle ID enters RD and RT arithmetic and fails with None unless both override helpers return first. RD override containing ':' or 'auto' returns before arithmetic; RT override containing ':' also returns first. With both supported explicit overrides, null bundle ID is not used numerically in the only bundle-ID consumers. RD/RT helper failures remain saved observations, not full-build observations.

Permitted contexts from static analysis / saved observations: Unreferenced EVPN VLAN bundle catalog. Both RD and RT full overrides bypass ID arithmetic in selected bundle; schema-valid settings can avoid the helper failure.

Migration/impact: Supply bundle ID even when full RD/RT overrides exist, or remove unused bundle; rejection closes a currently accepted calculation bypass.

Assessment evidence: [router_bgp.py:640](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:640), [utils.py:403](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/utils.py:403), [utils.py:423](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/utils.py:423)

Saved method observation: [evpn_bundle_null_id_rd_helper](probe-results.json#/methods/evpn_bundle_null_id_rd_helper).

Saved method observation: [evpn_bundle_null_id_rt_helper](probe-results.json#/methods/evpn_bundle_null_id_rt_helper).

Schema occurrence paths:

- `avd_design.evpn_vlan_bundles[].id`

### R594 — `fabric_name`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `explicit-error`. Use the declaration assessment below for the current finding.

Source: [fabric_name.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/fabric_name.schema.yml); declaration `eos_designs#/keys/fabric_name`.

Assessment: `reviewed`. Compatibility verdict: `active-path-already-rejected`. Output boundary: `static-analysis`.

Pattern: `unconditional-explicit-error`.

Stricter-rule finding: Every regular structured-config metadata contributor reads shared_utils.fabric_name; falsy/null fabric_name raises existing missing-variable error before output. Required-null check is earlier and clearer on this build path.

Permitted contexts from static analysis / saved observations: None established for the inspected regular active path; this is not a universal safety proof.

Migration/impact: Supply fabric_name.

Assessment evidence: [misc.py:169](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/misc.py:169), [__init__.py:33](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/metadata/__init__.py:33)

Schema occurrence paths:

- `avd_design.fabric_name`

### R595 — `generate_cv_tags.device_tags.name`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `handoff`. Use the declaration assessment below for the current finding.

Source: [generate_cv_tags.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/generate_cv_tags.schema.yml); declaration `eos_designs#/keys/generate_cv_tags/keys/device_tags/items/keys/name`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `dynamic-value-suppression-or-output-key`.

Stricter-rule finding: Custom tag name is passed unchecked when static or data_path value is nonempty. Metadata name is required; stripping null with value retained fails output validation. Dynamic data_path returning None/empty silently skips tag before emission, permitting null name in that unused tag.

Permitted contexts from static analysis / saved observations: Dynamic tag data_path has no value for this device.

Migration/impact: Supply tag name or remove inactive tag definition; shared inventories may rely on device-specific data_path omission.

Assessment evidence: [cv_tags.py:149](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/metadata/cv_tags.py:149), [cv_tags.py:173](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/metadata/cv_tags.py:173)

Schema occurrence paths:

- `avd_design.generate_cv_tags.device_tags[].name`

### R596 — `internal_vlan_order.allocation`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `output-validation-dependent`. Use the declaration assessment below for the current finding.

Source: [internal_vlan_order.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/internal_vlan_order.schema.yml); declaration `eos_designs#/keys/internal_vlan_order/keys/allocation`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `probed-output-boundary`.

Pattern: `optional-parent-pruning-or-override-repair`.

Stricter-rule finding: Internal VLAN order is a direct model cast. Stripped required child fails when its parent remains; all children null prune optional order/range parents. Saved allocation override scenario repairs stripped child before output validation.

Permitted contexts from static analysis / saved observations: All-null optional parent pruned. Later override repairs allocation (saved case).

Migration/impact: Supply complete allocation/range or remove unused parent; inspect inherited intent before deleting tombstones.

Assessment evidence: [__init__.py:224](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/__init__.py:224)

- Output probe `vlan_internal_order_all_null` → `vlan_internal_order` (target required: False): after stripping **accepted**; override applied: False; final output **accepted**. [Observation](probe-results.json#/output_validation/vlan_internal_order_all_null).
- Output probe `vlan_internal_order_allocation_none` → `vlan_internal_order.allocation` (target required: True): after stripping **rejected**; override applied: False; final output **rejected**. [Observation](probe-results.json#/output_validation/vlan_internal_order_allocation_none).
- Output probe `vlan_internal_order_allocation_repaired_by_override` → `vlan_internal_order.allocation` (target required: True): after stripping **rejected**; override applied: True; final output **accepted**. [Observation](probe-results.json#/output_validation/vlan_internal_order_allocation_repaired_by_override).

Synthetic output-boundary probes support the inspected handoff, not an end-to-end design run or a safe-rejection verdict.

Schema occurrence paths:

- `avd_design.internal_vlan_order.allocation`

### R597 — `internal_vlan_order.range.beginning`

Type: `int`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `output-validation-dependent`. Use the declaration assessment below for the current finding.

Source: [internal_vlan_order.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/internal_vlan_order.schema.yml); declaration `eos_designs#/keys/internal_vlan_order/keys/range/keys/beginning`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `probed-output-boundary`.

Pattern: `optional-parent-pruning-or-override-repair`.

Stricter-rule finding: Internal VLAN order is a direct model cast. Stripped required child fails when its parent remains; all children null prune optional order/range parents. Saved allocation override scenario repairs stripped child before output validation.

Permitted contexts from static analysis / saved observations: All-null optional parent pruned. Later override repairs allocation (saved case).

Migration/impact: Supply complete allocation/range or remove unused parent; inspect inherited intent before deleting tombstones.

Assessment evidence: [__init__.py:224](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/__init__.py:224)

- Output probe `vlan_internal_order_all_null` → `vlan_internal_order` (target required: False): after stripping **accepted**; override applied: False; final output **accepted**. [Observation](probe-results.json#/output_validation/vlan_internal_order_all_null).
- Output probe `vlan_internal_order_beginning_none` → `vlan_internal_order.range.beginning` (target required: True): after stripping **rejected**; override applied: False; final output **rejected**. [Observation](probe-results.json#/output_validation/vlan_internal_order_beginning_none).

Synthetic output-boundary probes support the inspected handoff, not an end-to-end design run or a safe-rejection verdict.

Schema occurrence paths:

- `avd_design.internal_vlan_order.range.beginning`

### R598 — `internal_vlan_order.range.ending`

Type: `int`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `output-validation-dependent`. Use the declaration assessment below for the current finding.

Source: [internal_vlan_order.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/internal_vlan_order.schema.yml); declaration `eos_designs#/keys/internal_vlan_order/keys/range/keys/ending`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `probed-output-boundary`.

Pattern: `optional-parent-pruning-or-override-repair`.

Stricter-rule finding: Internal VLAN order is a direct model cast. Stripped required child fails when its parent remains; all children null prune optional order/range parents. Saved allocation override scenario repairs stripped child before output validation.

Permitted contexts from static analysis / saved observations: All-null optional parent pruned. Later override repairs allocation (saved case).

Migration/impact: Supply complete allocation/range or remove unused parent; inspect inherited intent before deleting tombstones.

Assessment evidence: [__init__.py:224](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/__init__.py:224)

- Output probe `vlan_internal_order_all_null` → `vlan_internal_order` (target required: False): after stripping **accepted**; override applied: False; final output **accepted**. [Observation](probe-results.json#/output_validation/vlan_internal_order_all_null).
- Output probe `vlan_internal_order_ending_none` → `vlan_internal_order.range.ending` (target required: True): after stripping **rejected**; override applied: False; final output **rejected**. [Observation](probe-results.json#/output_validation/vlan_internal_order_ending_none).

Synthetic output-boundary probes support the inspected handoff, not an end-to-end design run or a safe-rejection verdict.

Schema occurrence paths:

- `avd_design.internal_vlan_order.range.ending`

### R601 — `ipv4_prefix_list_catalog.sequence_numbers.action`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `stripped-required-output`. Use the declaration assessment below for the current finding.

Source: [ipv4_prefix_lists.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/ipv4_prefix_lists.schema.yml); declaration `eos_designs#/keys/ipv4_prefix_list_catalog/items/keys/sequence_numbers/items/keys/action`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `probed-output-boundary`.

Pattern: `inactive-catalog-or-retained-required-output`.

Stricter-rule finding: Referenced prefix-list sequence retains its sequence primary key; null action is stripped and missing required action fails EOS validation. Unreferenced catalog entry is not emitted. IPv4 Jinja guard versus IPv6 unguarded rendering does not change the validated-output verdict.

Permitted contexts from static analysis / saved observations: Unreferenced prefix-list catalog.

Migration/impact: Supply action or remove invalid sequence/catalog. Custom null applied after stripping remains separate.

Assessment evidence: [misc.py:308](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/misc.py:308), [misc.py:314](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/misc.py:314)

- Output probe `ipv4_prefix_list_action_none` → `prefix_lists[].sequence_numbers[].action` (target required: True): after stripping **rejected**; override applied: False; final output **rejected**. [Observation](probe-results.json#/output_validation/ipv4_prefix_list_action_none).

Synthetic output-boundary probes support the inspected handoff, not an end-to-end design run or a safe-rejection verdict.

Schema occurrence paths:

- `avd_design.ipv4_prefix_list_catalog[].sequence_numbers[].action`

### R604 — `ipv6_prefix_list_catalog.sequence_numbers.action`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `stripped-required-output`. Use the declaration assessment below for the current finding.

Source: [ipv6_prefix_lists.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/ipv6_prefix_lists.schema.yml); declaration `eos_designs#/keys/ipv6_prefix_list_catalog/items/keys/sequence_numbers/items/keys/action`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `probed-output-boundary`.

Pattern: `inactive-catalog-or-retained-required-output`.

Stricter-rule finding: Referenced prefix-list sequence retains its sequence primary key; null action is stripped and missing required action fails EOS validation. Unreferenced catalog entry is not emitted. IPv4 Jinja guard versus IPv6 unguarded rendering does not change the validated-output verdict.

Permitted contexts from static analysis / saved observations: Unreferenced prefix-list catalog.

Migration/impact: Supply action or remove invalid sequence/catalog. Custom null applied after stripping remains separate.

Assessment evidence: [misc.py:308](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/misc.py:308), [misc.py:314](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/misc.py:314)

- Output probe `ipv6_prefix_list_action_none` → `ipv6_prefix_lists[].sequence_numbers[].action` (target required: True): after stripping **rejected**; override applied: False; final output **rejected**. [Observation](probe-results.json#/output_validation/ipv6_prefix_list_action_none).

Synthetic output-boundary probes support the inspected handoff, not an end-to-end design run or a safe-rejection verdict.

Schema occurrence paths:

- `avd_design.ipv6_prefix_list_catalog[].sequence_numbers[].action`

### R605 — `$defs.l2vlans.private_vlan.type`

Type: `str`. Scope occurrences: 3. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `handoff`. Use the declaration assessment below for the current finding.

Source: [defs_l2vlans.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/defs_l2vlans.schema.yml); declaration `eos_designs#/$defs/l2vlans/items/keys/private_vlan/keys/type`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-downstream-field`.

Stricter-rule finding: Selected private VLAN type is copied into optional EOS type; valid primary_vlan retains model and membership check can succeed. Null type is stripped with no required-output failure. L2/tenant/VLAN selection controls activation.

Permitted contexts from static analysis / saved observations: Valid primary VLAN, null type omitted downstream. Unselected/disabled L2 service.

Migration/impact: Provide community/isolated or remove unused private-VLAN block; schema acceptance is not proof of correct private VLAN configuration.

Assessment evidence: [vlans.py:69](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/vlans.py:69), [vlans.py:77](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/vlans.py:77)

Schema occurrence paths:

- `avd_design.l2vlan_profiles[].private_vlan.type`
- `avd_design.network_services[].l2vlans[].private_vlan.type`
- `avd_design.<dynamic:network_services_keys.name>[].l2vlans[].private_vlan.type`

### R606 — `$defs.l2vlans.private_vlan.primary_vlan`

Type: `int`. Scope occurrences: 3. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `handoff`. Use the declaration assessment below for the current finding.

Source: [defs_l2vlans.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/defs_l2vlans.schema.yml); declaration `eos_designs#/$defs/l2vlans/items/keys/private_vlan/keys/primary_vlan`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `inactive-feature-or-membership-error`.

Stricter-rule finding: Active private VLAN adds null primary VLAN to referenced-primary set. It cannot be a valid generated VLAN ID and the missing-primary check raises (or formatting the mixed invalid set may fail). Filtered/disabled L2 services do not reach it.

Permitted contexts from static analysis / saved observations: Private VLAN input filtered out/not deployed by L2 services.

Migration/impact: Supply existing primary VLAN or remove unused private-VLAN block.

Assessment evidence: [vlans.py:76](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/vlans.py:76), [vlans.py:83](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/vlans.py:83)

Schema occurrence paths:

- `avd_design.l2vlan_profiles[].private_vlan.primary_vlan`
- `avd_design.network_services[].l2vlans[].private_vlan.primary_vlan`
- `avd_design.<dynamic:network_services_keys.name>[].l2vlans[].private_vlan.primary_vlan`

### R607 — `logging_settings.hosts.name`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `handoff`. Use the declaration assessment below for the current finding.

Source: [logging_settings.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/logging_settings.yml); declaration `eos_designs#/keys/logging_settings/keys/hosts/items/keys/name`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `output-primary-key-or-item-pruning`.

Stricter-rule finding: Null host name with inherited protocol default 'udp' leaves a retained host lacking its primary key and fails output validation. If protocol is explicitly null and ports/SSL profile are absent/null, whole host item is pruned; the output VRF hosts collection is optional.

Permitted contexts from static analysis / saved observations: Null-only host after explicitly suppressing protocol default; no ports/SSL content retains it.

Migration/impact: Supply name or remove unused host; null default suppression must be considered when cleaning input.

Assessment evidence: [logging.py:61](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/logging.py:61), [logging.py:78](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/logging.py:78)

Schema occurrence paths:

- `avd_design.logging_settings.hosts[].name`

### R608 — `management_eapi.vrfs.enabled`

Type: `bool`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `null-suppresses`. Use the declaration assessment below for the current finding.

Source: [management_eapi.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/management_eapi.schema.yml); declaration `eos_designs#/keys/management_eapi/keys/vrfs/items/keys/enabled`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `boolean-suppression`.

Stricter-rule finding: With management_eapi enabled only truthy per-VRF enabled emits the VRF. Null suppresses it like false. ACT eAPI enforcement can separately create default VRF.

Permitted contexts from static analysis / saved observations: Per-VRF eAPI suppressed. Management eAPI globally disabled.

Migration/impact: Use explicit false for per-VRF disable; ACT enforcement remains separate.

Assessment evidence: [__init__.py:424](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/__init__.py:424)

Previously confirmed observation: Earlier real management API contributor comparison: null/false omitted VRF and true emitted it; those outputs validated.

Schema occurrence paths:

- `avd_design.management_eapi.vrfs[].enabled`

### R611 — `network_services.bgp_peer_groups.address_family_ipv6.default_originate.enabled`

Type: `bool`. Scope occurrences: 2. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `handoff`. Use the declaration assessment below for the current finding.

Source: [network_services.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/network_services.schema.yml); declaration `eos_designs#/keys/network_services/items/keys/bgp_peer_groups/items/keys/address_family_ipv6/keys/default_originate/keys/enabled`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-downstream-field`.

Stricter-rule finding: Selected tenant/VRF peer-group IPv6 default_originate is cast into EOS AF IPv6 peer groups. Unlike source required enabled, EOS enabled is OPTIONAL. Null is stripped even when always/route_map retain parent; output requiredness does not reject. Null-aware template enables default origination only with appropriate enabled value.

Permitted contexts from static analysis / saved observations: Selected peer group with default-originate enable omitted. Unselected/nonmatching tenant/VRF peer group.

Migration/impact: Use explicit intended bool; null, false and deleting inherited value need not render identically.

Assessment evidence: [router_bgp.py:83](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:83), [router_bgp.py:101](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:101), [router_bgp.py:110](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:110)

Schema occurrence paths:

- `avd_design.network_services[].bgp_peer_groups[].address_family_ipv6.default_originate.enabled`
- `avd_design.<dynamic:network_services_keys.name>[].bgp_peer_groups[].address_family_ipv6.default_originate.enabled`

### R612 — `network_services.bgp_peer_groups.listen_ranges.prefix`

Type: `str`. Scope occurrences: 4. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `handoff`. Use the declaration assessment below for the current finding.

Source: [network_services.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/network_services.schema.yml); declaration `eos_designs#/keys/network_services/items/keys/bgp_peer_groups/items/keys/listen_ranges/items/keys/prefix`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `inactive-peer-group-or-output-error`.

Stricter-rule finding: Selected node-matching peer-group listen ranges copy prefix to required output prefix alongside peer_group and remote_as/peer_filter; null stripping retains item and causes requiredness failure. Invalid selector credentials raise earlier; unmatched ranges/groups are unused.

Permitted contexts from static analysis / saved observations: Unselected/nonmatching peer group.

Migration/impact: Supply prefix or remove unused listen-range entry.

Assessment evidence: [router_bgp.py:854](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:854), [router_bgp.py:865](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:865)

Schema occurrence paths:

- `avd_design.network_services[].bgp_peer_groups[].listen_ranges[].prefix`
- `avd_design.network_services[].vrfs[].bgp_peer_groups[].listen_ranges[].prefix`
- `avd_design.<dynamic:network_services_keys.name>[].bgp_peer_groups[].listen_ranges[].prefix`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].bgp_peer_groups[].listen_ranges[].prefix`

### R613 — `network_services.vxlan_flood_multicast.enabled`

Type: `bool`. Scope occurrences: 2. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `null-suppresses`. Use the declaration assessment below for the current finding.

Source: [network_services.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/network_services.schema.yml); declaration `eos_designs#/keys/network_services/items/keys/vxlan_flood_multicast/keys/enabled`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `boolean-suppression`.

Stricter-rule finding: VLAN enable takes precedence via default(vlan,tenant). Null tenant enabled is falsy when no VLAN override, suppressing flood multicast; explicit VLAN true can still enable. Static/dynamic tenant keys share implementation.

Permitted contexts from static analysis / saved observations: Flood multicast suppressed. Per-VLAN true overrides null tenant setting.

Migration/impact: Use explicit false for tenant disable; it still allows explicit per-VLAN setting.

Assessment evidence: [vxlan_interface.py:207](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/vxlan_interface.py:207)

Schema occurrence paths:

- `avd_design.network_services[].vxlan_flood_multicast.enabled`
- `avd_design.<dynamic:network_services_keys.name>[].vxlan_flood_multicast.enabled`

### R614 — `network_services.evpn_l3_multicast.evpn_underlay_l3_multicast_group_ipv4_pool`

Type: `str`. Scope occurrences: 2. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `explicit-error`. Use the declaration assessment below for the current finding.

Source: [network_services.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/network_services.schema.yml); declaration `eos_designs#/keys/network_services/items/keys/evpn_l3_multicast/keys/evpn_underlay_l3_multicast_group_ipv4_pool`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `override-fallback-or-active-error`.

Stricter-rule finding: When EVPN L3 multicast needs automatic allocation, missing/null tenant pool raises. Explicit per-VRF multicast group bypasses pool; disabled/unselected multicast does not need it.

Permitted contexts from static analysis / saved observations: Explicit VRF multicast group. EVPN L3 multicast not active.

Migration/impact: Supply pool or remove dormant tenant multicast block; explicitly configured VRF group is a valid current bypass.

Assessment evidence: [vxlan_interface.py:153](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/vxlan_interface.py:153), [vxlan_interface.py:157](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/vxlan_interface.py:157)

Schema occurrence paths:

- `avd_design.network_services[].evpn_l3_multicast.evpn_underlay_l3_multicast_group_ipv4_pool`
- `avd_design.<dynamic:network_services_keys.name>[].evpn_l3_multicast.evpn_underlay_l3_multicast_group_ipv4_pool`

### R615 — `network_services.vrfs.ospf.message_digest_keys.cleartext_key`

Type: `str`. Scope occurrences: 2. Other-scope occurrences excluded: 0. Initial consumer index: `demonstrated-method` / `runtime-risk`. Use the declaration assessment below for the current finding.

Source: [network_services.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/network_services.schema.yml); declaration `eos_designs#/keys/network_services/items/keys/vrfs/items/keys/ospf/keys/message_digest_keys/items/keys/cleartext_key`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `inactive-feature-or-runtime-failure`.

Stricter-rule finding: Reached VRF digest key with null cleartext takes unmatched 'impossible' branch and raises via unbound msg. Key ID zero returns early; inactive OSPF/authentication/filtered interfaces/VRFs do not consume it.

Permitted contexts from static analysis / saved observations: Authentication not applied to any retained interface. Interface-level digest keys supersede the VRF-level key list. id:0 key returns without encryption.

Migration/impact: Supply cleartext key or remove unused digest-key entry.

Assessment evidence: [filtered_tenants.py:649](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/filtered_tenants.py:649), [filtered_tenants.py:673](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/filtered_tenants.py:673), [filtered_tenants.py:692](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/filtered_tenants.py:692)

Saved method observation: [vrf_ospf_null_cleartext_missing_key_error](probe-results.json#/methods/vrf_ospf_null_cleartext_missing_key_error).

Schema occurrence paths:

- `avd_design.network_services[].vrfs[].ospf.message_digest_keys[].cleartext_key`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].ospf.message_digest_keys[].cleartext_key`

### R616 — `network_services.vrfs.svis.name`

Type: `str`. Scope occurrences: 2. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `null-fallback`. Use the declaration assessment below for the current finding.

Source: [network_services.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/network_services.schema.yml); declaration `eos_designs#/keys/network_services/items/keys/vrfs/items/keys/svis/items/keys/name`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-downstream-field`.

Stricter-rule finding: Shared VLAN builder copies name to optional EOS VLAN name. SVI interface description uses explicit description then SVI name; null fallback omits optional description. Selected VLAN ID remains valid, so downstream requiredness does not reject nameless VLAN. SVIs additionally undergo profile/node merge.

Permitted contexts from static analysis / saved observations: Selected VLAN without name/description. Filtered VLAN/SVI.

Migration/impact: Supply desired name or remove unused entry; replacing null with absence can restore profile name.

Assessment evidence: [vlans.py:102](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/vlans.py:102), [vlan_interfaces.py:80](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/vlan_interfaces.py:80)

Schema occurrence paths:

- `avd_design.network_services[].vrfs[].svis[].name`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].svis[].name`

### R617 — `$defs.static_routes.prefix`

Type: `str`. Scope occurrences: 24. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `stripped-required-output`. Use the declaration assessment below for the current finding.

Source: [defs_static_routes.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/defs_static_routes.schema.yml); declaration `eos_designs#/$defs/static_routes/items/keys/prefix`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `probed-output-boundary`.

Pattern: `inactive-route-or-required-output`.

Stricter-rule finding: VRF IPv4/IPv6 static routes include non-null VRF name, preserving items after null prefix stripping; missing required output prefix fails. Routes on selected SVIs/L3 interfaces/port-channels are cast into the same VRF route collections. Node-level port-channel route has retained peer next-hop. Default-VRF redistribution can additionally format 'permit None' before EOS validation. Filtering can leave invalid route inputs unused. Unused svi_profiles and nonmatching SVI node overrides also bypass route consumption; both static/dynamic tenant paths enter the same SVI inheritance and filtering methods.

Permitted contexts from static analysis / saved observations: Node/tenant/VLAN/interface-filtered route. Unused SVI profile or SVI node override for another host. Unselected node configuration.

Migration/impact: Supply prefix or remove route; active path already fails but blanket input rejection also affects dormant/shared route definitions.

Assessment evidence: [filtered_tenants.py:263](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/filtered_tenants.py:263), [filtered_tenants.py:408](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/filtered_tenants.py:408), [filtered_tenants.py:431](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/filtered_tenants.py:431), [filtered_tenants.py:458](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/filtered_tenants.py:458), [static_routes.py:41](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/static_routes.py:41), [ipv6_static_routes.py:38](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/ipv6_static_routes.py:38), [prefix_lists.py:83](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/prefix_lists.py:83)

- Output probe `static_route_prefix_none` → `static_routes[].prefix` (target required: True): after stripping **rejected**; override applied: False; final output **rejected**. [Observation](probe-results.json#/output_validation/static_route_prefix_none).

Synthetic output-boundary probes support the inspected handoff, not an end-to-end design run or a safe-rejection verdict.

Schema occurrence paths:

- `avd_design.network_services[].vrfs[].svis[].nodes[].static_routes[].prefix`
- `avd_design.network_services[].vrfs[].svis[].nodes[].ipv6_static_routes[].prefix`
- `avd_design.network_services[].vrfs[].svis[].static_routes[].prefix`
- `avd_design.network_services[].vrfs[].svis[].ipv6_static_routes[].prefix`
- `avd_design.network_services[].vrfs[].l3_interfaces[].static_routes[].prefix`
- `avd_design.network_services[].vrfs[].l3_interfaces[].ipv6_static_routes[].prefix`
- `avd_design.network_services[].vrfs[].l3_port_channels[].static_routes[].prefix`
- `avd_design.network_services[].vrfs[].l3_port_channels[].ipv6_static_routes[].prefix`
- `avd_design.network_services[].vrfs[].static_routes[].prefix`
- `avd_design.network_services[].vrfs[].ipv6_static_routes[].prefix`
- `avd_design.svi_profiles[].nodes[].static_routes[].prefix`
- `avd_design.svi_profiles[].nodes[].ipv6_static_routes[].prefix`
- `avd_design.svi_profiles[].static_routes[].prefix`
- `avd_design.svi_profiles[].ipv6_static_routes[].prefix`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].svis[].nodes[].static_routes[].prefix`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].svis[].nodes[].ipv6_static_routes[].prefix`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].svis[].static_routes[].prefix`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].svis[].ipv6_static_routes[].prefix`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].l3_interfaces[].static_routes[].prefix`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].l3_interfaces[].ipv6_static_routes[].prefix`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].l3_port_channels[].static_routes[].prefix`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].l3_port_channels[].ipv6_static_routes[].prefix`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].static_routes[].prefix`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].ipv6_static_routes[].prefix`

### R618 — `network_services.vrfs.l3_port_channels.name`

Type: `str`. Scope occurrences: 2. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `runtime-risk`. Use the declaration assessment below for the current finding.

Source: [network_services.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/network_services.schema.yml); declaration `eos_designs#/keys/network_services/items/keys/vrfs/items/keys/l3_port_channels/items/keys/name`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `host-filter-or-runtime-failure`.

Stricter-rule finding: Selected tenant L3 port-channel name is used in '.' in name before output; null raises TypeError. Host node filtering/unaccepted VRFs keeps the entry unused.

Permitted contexts from static analysis / saved observations: Port-channel not assigned to this host or VRF not accepted.

Migration/impact: Supply port-channel name or remove unused entry.

Assessment evidence: [filtered_tenants.py:453](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/filtered_tenants.py:453), [port_channel_interfaces.py:60](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/port_channel_interfaces.py:60)

Schema occurrence paths:

- `avd_design.network_services[].vrfs[].l3_port_channels[].name`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].l3_port_channels[].name`

### R619 — `network_services.vrfs.l3_port_channels.node`

Type: `str`. Scope occurrences: 2. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `null-suppresses`. Use the declaration assessment below for the current finding.

Source: [network_services.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/network_services.schema.yml); declaration `eos_designs#/keys/network_services/items/keys/vrfs/items/keys/l3_port_channels/items/keys/node`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `host-suppression`.

Stricter-rule finding: Required node null fails equality with hostname and excludes L3 port-channel/loopback from every device before building it.

Permitted contexts from static analysis / saved observations: Object excluded from every host.

Migration/impact: Remove unused object or assign node; explicit null is currently an effective host-selection suppressor.

Assessment evidence: [filtered_tenants.py:453](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/filtered_tenants.py:453), [filtered_tenants.py:269](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/filtered_tenants.py:269)

Schema occurrence paths:

- `avd_design.network_services[].vrfs[].l3_port_channels[].node`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].l3_port_channels[].node`

### R620 — `network_services.vrfs.loopbacks.node`

Type: `str`. Scope occurrences: 2. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `null-suppresses`. Use the declaration assessment below for the current finding.

Source: [network_services.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/network_services.schema.yml); declaration `eos_designs#/keys/network_services/items/keys/vrfs/items/keys/loopbacks/items/keys/node`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `host-suppression`.

Stricter-rule finding: Required node null fails equality with hostname and excludes L3 port-channel/loopback from every device before building it.

Permitted contexts from static analysis / saved observations: Object excluded from every host.

Migration/impact: Remove unused object or assign node; explicit null is currently an effective host-selection suppressor.

Assessment evidence: [filtered_tenants.py:453](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/filtered_tenants.py:453), [filtered_tenants.py:269](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/filtered_tenants.py:269)

Schema occurrence paths:

- `avd_design.network_services[].vrfs[].loopbacks[].node`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].loopbacks[].node`

### R621 — `network_services.vrfs.loopbacks.loopback`

Type: `int`. Scope occurrences: 2. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `runtime-risk`. Use the declaration assessment below for the current finding.

Source: [network_services.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/network_services.schema.yml); declaration `eos_designs#/keys/network_services/items/keys/vrfs/items/keys/loopbacks/items/keys/loopback`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `non-null-invalid-formatting`.

Stricter-rule finding: Null loopback number is interpolated into 'LoopbackNone'; this string survives null stripping. EOS interface-name type does not enforce numeric suffix, so output schema acceptance is possible but device acceptance is unproven. Network-service node filtering and underlay multicast activation decide use.

Permitted contexts from static analysis / saved observations: Generated LoopbackNone can pass output schema (saved network-service observation). Unselected node/disabled relevant services.

Migration/impact: Supply integer loopback number or remove unused object. Do not describe schema-valid LoopbackNone as deployable.

Assessment evidence: [loopback_interfaces.py:46](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/loopback_interfaces.py:46), [underlay.py:76](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/underlay.py:76), [underlay.py:87](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/underlay.py:87)

Previously confirmed observation: Earlier real contributor observation: LoopbackNone survived stripping and was accepted by EOS schema; no EOS device acceptance test.

Schema occurrence paths:

- `avd_design.network_services[].vrfs[].loopbacks[].loopback`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].loopbacks[].loopback`

### R622 — `network_services.vrfs.loopbacks.ip_address`

Type: `str`. Scope occurrences: 2. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `handoff`. Use the declaration assessment below for the current finding.

Source: [network_services.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/network_services.schema.yml); declaration `eos_designs#/keys/network_services/items/keys/vrfs/items/keys/loopbacks/items/keys/ip_address`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-downstream-field`.

Stricter-rule finding: Null loopback IP is copied to optional output ip_address and stripped; interface name/shutdown/VRF retain interface. Source-NAT helper guards the IP conversion and emits optional null IP rather than throwing. No downstream non-null IP guarantee.

Permitted contexts from static analysis / saved observations: Selected loopback emitted without IP address. Node-filtered loopback.

Migration/impact: Supply IP if desired or remove unused loopback; unaddressed loopback semantics depend on use.

Assessment evidence: [loopback_interfaces.py:47](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/loopback_interfaces.py:47), [loopback_interfaces.py:58](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/loopback_interfaces.py:58), [virtual_source_nat_vrfs.py:41](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/virtual_source_nat_vrfs.py:41)

Schema occurrence paths:

- `avd_design.network_services[].vrfs[].loopbacks[].ip_address`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].loopbacks[].ip_address`

### R623 — `network_services.vrfs.static_arp_entries.ipv4_address`

Type: `str`. Scope occurrences: 2. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `stripped-required-output`. Use the declaration assessment below for the current finding.

Source: [network_services.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/network_services.schema.yml); declaration `eos_designs#/keys/network_services/items/keys/vrfs/items/keys/static_arp_entries/items/keys/ipv4_address`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `probed-output-boundary`.

Pattern: `node-filter-or-retained-required-output`.

Stricter-rule finding: Selected static ARP input copies both addresses to required EOS ARP fields. VRF/siblings retain the item after null stripping and output validation fails. Node filters/unaccepted VRF/L3-disabled path leave inputs unused.

Permitted contexts from static analysis / saved observations: ARP entry node-filtered or VRF not accepted.

Migration/impact: Supply both addresses or remove entry. Overrides applied after stripping may repair generated output; no declaration-specific repair demonstrated.

Assessment evidence: [filtered_tenants.py:265](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/filtered_tenants.py:265), [arp.py:37](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/arp.py:37)

- Output probe `arp_ipv4_address_none` → `arp.static_entries[].ipv4_address` (target required: True): after stripping **rejected**; override applied: False; final output **rejected**. [Observation](probe-results.json#/output_validation/arp_ipv4_address_none).

Synthetic output-boundary probes support the inspected handoff, not an end-to-end design run or a safe-rejection verdict.

Schema occurrence paths:

- `avd_design.network_services[].vrfs[].static_arp_entries[].ipv4_address`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].static_arp_entries[].ipv4_address`

### R624 — `network_services.vrfs.static_arp_entries.mac_address`

Type: `str`. Scope occurrences: 2. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `stripped-required-output`. Use the declaration assessment below for the current finding.

Source: [network_services.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/network_services.schema.yml); declaration `eos_designs#/keys/network_services/items/keys/vrfs/items/keys/static_arp_entries/items/keys/mac_address`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `probed-output-boundary`.

Pattern: `node-filter-or-retained-required-output`.

Stricter-rule finding: Selected static ARP input copies both addresses to required EOS ARP fields. VRF/siblings retain the item after null stripping and output validation fails. Node filters/unaccepted VRF/L3-disabled path leave inputs unused.

Permitted contexts from static analysis / saved observations: ARP entry node-filtered or VRF not accepted.

Migration/impact: Supply both addresses or remove entry. Overrides applied after stripping may repair generated output; no declaration-specific repair demonstrated.

Assessment evidence: [filtered_tenants.py:265](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/filtered_tenants.py:265), [arp.py:37](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/arp.py:37)

- Output probe `arp_mac_address_none` → `arp.static_entries[].mac_address` (target required: True): after stripping **rejected**; override applied: False; final output **rejected**. [Observation](probe-results.json#/output_validation/arp_mac_address_none).

Synthetic output-boundary probes support the inspected handoff, not an end-to-end design run or a safe-rejection verdict.

Schema occurrence paths:

- `avd_design.network_services[].vrfs[].static_arp_entries[].mac_address`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].static_arp_entries[].mac_address`

### R625 — `network_services.vrfs.bgp.graceful_restart.enabled`

Type: `bool`. Scope occurrences: 2. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `handoff`. Use the declaration assessment below for the current finding.

Source: [network_services.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/network_services.schema.yml); declaration `eos_designs#/keys/network_services/items/keys/vrfs/items/keys/bgp/keys/graceful_restart/keys/enabled`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-downstream-field`.

Stricter-rule finding: Non-default active BGP VRF tests enabled is False; None instead copies enabled/restart_time. Enabled is OPTIONAL in EOS graceful_restart; stripping None can leave timer-only or no graceful_restart model. Default VRF rejects this entire input block, while BGP-disabled VRF ignores it.

Permitted contexts from static analysis / saved observations: Non-default VRF null enabled omits enable, distinct from false/no_graceful_restart. BGP-inactive VRF.

Migration/impact: Use explicit false for intended disable; None currently differs from false and deletion/inheritance.

Assessment evidence: [router_bgp.py:149](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:149), [router_bgp.py:158](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:158)

Schema occurrence paths:

- `avd_design.network_services[].vrfs[].bgp.graceful_restart.enabled`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].bgp.graceful_restart.enabled`

### R626 — `network_services.vrfs.bgp_peer_groups.address_family_ipv6.default_originate.enabled`

Type: `bool`. Scope occurrences: 2. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `handoff`. Use the declaration assessment below for the current finding.

Source: [network_services.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/network_services.schema.yml); declaration `eos_designs#/keys/network_services/items/keys/vrfs/items/keys/bgp_peer_groups/items/keys/address_family_ipv6/keys/default_originate/keys/enabled`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-downstream-field`.

Stricter-rule finding: Selected tenant/VRF peer-group IPv6 default_originate is cast into EOS AF IPv6 peer groups. Unlike source required enabled, EOS enabled is OPTIONAL. Null is stripped even when always/route_map retain parent; output requiredness does not reject. Null-aware template enables default origination only with appropriate enabled value.

Permitted contexts from static analysis / saved observations: Selected peer group with default-originate enable omitted. Unselected/nonmatching tenant/VRF peer group.

Migration/impact: Use explicit intended bool; null, false and deleting inherited value need not render identically.

Assessment evidence: [router_bgp.py:83](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:83), [router_bgp.py:101](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:101), [router_bgp.py:110](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:110)

Schema occurrence paths:

- `avd_design.network_services[].vrfs[].bgp_peer_groups[].address_family_ipv6.default_originate.enabled`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].bgp_peer_groups[].address_family_ipv6.default_originate.enabled`

### R627 — `network_services.vrfs.additional_route_targets.type`

Type: `str`. Scope occurrences: 2. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `runtime-risk`. Use the declaration assessment below for the current finding.

Source: [network_services.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/network_services.schema.yml); declaration `eos_designs#/keys/network_services/items/keys/vrfs/items/keys/additional_route_targets/items/keys/type`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `wrong-semantic-branch`.

Stricter-rule finding: Only literal import selects import; null chooses export and emits a valid route target when remaining fields are valid. Node/VRF filtering can also skip input. Correct direction is not enforced by downstream schema.

Permitted contexts from static analysis / saved observations: Null selects export with schema-valid output. Filtered target/VRF.

Migration/impact: Use explicit export/import. Explicit export preserves observed branch but must match operator intent.

Assessment evidence: [filtered_tenants.py:306](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/filtered_tenants.py:306), [router_bgp.py:370](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:370)

Previously confirmed observation: Earlier branch comparison: null selected export; an export-shaped output was schema-valid, but the complete contributor was not probed.

Schema occurrence paths:

- `avd_design.network_services[].vrfs[].additional_route_targets[].type`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].additional_route_targets[].type`

### R628 — `network_services.vrfs.additional_route_targets.address_family`

Type: `str`. Scope occurrences: 2. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `handoff`. Use the declaration assessment below for the current finding.

Source: [network_services.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/network_services.schema.yml); declaration `eos_designs#/keys/network_services/items/keys/vrfs/items/keys/additional_route_targets/items/keys/address_family`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `node-filter-or-output-key`.

Stricter-rule finding: Null address_family is used as indexed key in import/export route-target entry. Non-null route_target retains entry and null AF stripping causes primary-key validation error. If route_target is also null, scalar-list stripping removes it and optional empty AF entry can be pruned. The source requires keys present but accepts explicit null, so the both-null source scenario is allowed.

Permitted contexts from static analysis / saved observations: Node-filtered/unaccepted VRF target. All-null added target entry may be pruned if nothing else retains it.

Migration/impact: Specify address family or remove target entry; check collision/combination effects with other generated targets.

Assessment evidence: [router_bgp.py:370](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:370)

Schema occurrence paths:

- `avd_design.network_services[].vrfs[].additional_route_targets[].address_family`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].additional_route_targets[].address_family`

### R629 — `network_services.vrfs.additional_route_targets.route_target`

Type: `str`. Scope occurrences: 2. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `handoff`. Use the declaration assessment below for the current finding.

Source: [network_services.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/network_services.schema.yml); declaration `eos_designs#/keys/network_services/items/keys/vrfs/items/keys/additional_route_targets/items/keys/route_target`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `scalar-list-item-stripping`.

Stricter-rule finding: Null route_target is appended as scalar list item. Scalar-list stripping removes None; optional route_targets list is then removed if empty, while AF item may remain. EOS route_targets is optional. No automatic required-target failure follows.

Permitted contexts from static analysis / saved observations: Null route target omitted after list-item stripping. Filtered target/VRF.

Migration/impact: Remove unused additional target or supply route target. This illustrates required dictionary-field policy differs from scalar list-item null policy.

Assessment evidence: [router_bgp.py:370](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:370)

Schema occurrence paths:

- `avd_design.network_services[].vrfs[].additional_route_targets[].route_target`
- `avd_design.<dynamic:network_services_keys.name>[].vrfs[].additional_route_targets[].route_target`

### R630 — `network_services.l2vlans.name`

Type: `str`. Scope occurrences: 2. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `handoff`. Use the declaration assessment below for the current finding.

Source: [network_services.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/network_services.schema.yml); declaration `eos_designs#/keys/network_services/items/keys/l2vlans/items/keys/name`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-downstream-field`.

Stricter-rule finding: Shared VLAN builder copies name to optional EOS VLAN name. SVI interface description uses explicit description then SVI name; null fallback omits optional description. Selected VLAN ID remains valid, so downstream requiredness does not reject nameless VLAN. SVIs additionally undergo profile/node merge.

Permitted contexts from static analysis / saved observations: Selected VLAN without name/description. Filtered VLAN/SVI.

Migration/impact: Supply desired name or remove unused entry; replacing null with absence can restore profile name.

Assessment evidence: [vlans.py:102](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/vlans.py:102), [vlan_interfaces.py:80](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/vlan_interfaces.py:80)

Schema occurrence paths:

- `avd_design.network_services[].l2vlans[].name`
- `avd_design.<dynamic:network_services_keys.name>[].l2vlans[].name`

### R631 — `network_services.point_to_point_services.endpoints.id`

Type: `int`. Scope occurrences: 2. Other-scope occurrences excluded: 0. Initial consumer index: `demonstrated-method` / `runtime-risk`. Use the declaration assessment below for the current finding.

Source: [network_services.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/network_services.schema.yml); declaration `eos_designs#/keys/network_services/items/keys/point_to_point_services/items/keys/endpoints/items/keys/id`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `arithmetic-error-or-optional-output`.

Stricter-rule finding: With subinterfaces endpoint ID participates in addition and null fails. Without subinterfaces IDs are copied to optional EOS pseudowire id_local/id_remote, then stripped without requiredness failure. Remote ID can be consumed even when that remote endpoint is not locally selected.

Permitted contexts from static analysis / saved observations: No-subinterface pseudowire with null ID omitted from schema-valid output. P2P service not active.

Migration/impact: Supply endpoint IDs; dropping endpoint solely based on node selection is insufficient because remote ID may be used.

Assessment evidence: [router_bgp.py:815](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:815), [router_bgp.py:826](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:826), [router_bgp.py:832](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:832)

Saved method observation: [point_to_point_null_endpoint_id_with_subinterface](probe-results.json#/methods/point_to_point_null_endpoint_id_with_subinterface).

Schema occurrence paths:

- `avd_design.network_services[].point_to_point_services[].endpoints[].id`
- `avd_design.<dynamic:network_services_keys.name>[].point_to_point_services[].endpoints[].id`

### R634 — `sflow_settings.destinations.destination`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `handoff`. Use the declaration assessment below for the current finding.

Source: [sflow_settings.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/sflow_settings.schema.yml); declaration `eos_designs#/keys/sflow_settings/keys/destinations/items/keys/destination`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `feature-gating-or-output-key`.

Stricter-rule finding: sFlow is generated only for enabled supported interfaces. Null destination with non-null port retains an item without destination primary key and fails output validation. No port default is declared; null-only destination is pruned when port absent/null, and output destinations collections are optional, so run:true can remain without destinations.

Permitted contexts from static analysis / saved observations: No enabled/supported sFlow interface. Null destination with absent/null port pruned; requiredness does not enforce a usable destination.

Migration/impact: Supply destination or remove unused entry; downstream schema acceptance does not establish functioning sFlow.

Assessment evidence: [sflow.py:17](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/structured_config_utils/sflow.py:17), [sflow.py:43](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/structured_config_utils/sflow.py:43), [sflow.py:69](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/structured_config_utils/sflow.py:69)

Schema occurrence paths:

- `avd_design.sflow_settings.destinations[].destination`

### R635 — `ssh_settings.vrfs.enabled`

Type: `bool`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `null-suppresses`. Use the declaration assessment below for the current finding.

Source: [ssh_settings.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/ssh_settings.schema.yml); declaration `eos_designs#/keys/ssh_settings/keys/vrfs/items/keys/enabled`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `boolean-suppression`.

Stricter-rule finding: Per-VRF null enabled is passed to optional EOS enable and stripped; null also prevents ACL assignments. VRF name can remain but no enable/ACL command results; null and false differ because false is emitted as disable.

Permitted contexts from static analysis / saved observations: SSH VRF enable and ACL configuration omitted.

Migration/impact: Use explicit false if disabling SSH; deleting null can restore inherited enabled value.

Assessment evidence: [management_ssh.py:43](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/management_ssh.py:43)

Schema occurrence paths:

- `avd_design.ssh_settings.vrfs[].enabled`

### R636 — `underlay_multicast_rps.nodes.loopback_number`

Type: `int`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `runtime-risk`. Use the declaration assessment below for the current finding.

Source: [underlay_multicast_rps.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/underlay_multicast_rps.schema.yml); declaration `eos_designs#/keys/underlay_multicast_rps/items/keys/nodes/items/keys/loopback_number`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `non-null-invalid-formatting`.

Stricter-rule finding: Null loopback number is interpolated into 'LoopbackNone'; this string survives null stripping. EOS interface-name type does not enforce numeric suffix, so output schema acceptance is possible but device acceptance is unproven. Network-service node filtering and underlay multicast activation decide use.

Permitted contexts from static analysis / saved observations: Generated LoopbackNone can pass output schema (saved network-service observation). Unselected node/disabled relevant services.

Migration/impact: Supply integer loopback number or remove unused object. Do not describe schema-valid LoopbackNone as deployable.

Assessment evidence: [loopback_interfaces.py:46](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/loopback_interfaces.py:46), [underlay.py:76](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/underlay.py:76), [underlay.py:87](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/underlay.py:87)

Schema occurrence paths:

- `avd_design.underlay_multicast_rps[].nodes[].loopback_number`

### R637 — `underlay_ospf_authentication.enabled`

Type: `bool`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `null-suppresses`. Use the declaration assessment below for the current finding.

Source: [underlay_ospf_authentication.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/underlay_ospf_authentication.schema.yml); declaration `eos_designs#/keys/underlay_ospf_authentication/keys/enabled`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `boolean-suppression`.

Stricter-rule finding: Falsy enabled skips underlay authentication insertion on Ethernet and MLAG interfaces; no scalar type error. Default false does not replace explicit null.

Permitted contexts from static analysis / saved observations: Underlay OSPF authentication suppressed.

Migration/impact: Use explicit false for disable.

Assessment evidence: [ethernet_interfaces.py:101](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/underlay/ethernet_interfaces.py:101), [__init__.py:126](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/mlag/__init__.py:126)

Schema occurrence paths:

- `avd_design.underlay_ospf_authentication.enabled`

### R639 — `underlay_ospf_authentication.message_digest_keys.cleartext_key`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `demonstrated-method` / `runtime-risk`. Use the declaration assessment below for the current finding.

Source: [underlay_ospf_authentication.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/underlay_ospf_authentication.schema.yml); declaration `eos_designs#/keys/underlay_ospf_authentication/keys/message_digest_keys/items/keys/cleartext_key`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `inactive-feature-or-runtime-failure`.

Stricter-rule finding: Active underlay/MLAG OSPF authentication sends null cleartext key to encryption and fails (saved method probe). Authentication disabled or consumer branch inactive leaves invalid key unused.

Permitted contexts from static analysis / saved observations: Authentication disabled/inactive underlay or MLAG path.

Migration/impact: Supply cleartext key or remove unused key block.

Assessment evidence: [ethernet_interfaces.py:109](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/underlay/ethernet_interfaces.py:109), [__init__.py:129](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/mlag/__init__.py:129)

Saved method observation: [underlay_ospf_null_cleartext_encryption](probe-results.json#/methods/underlay_ospf_null_cleartext_encryption).

Schema occurrence paths:

- `avd_design.underlay_ospf_authentication.message_digest_keys[].cleartext_key`

### R640 — `wan_carriers.path_group`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `explicit-error`. Use the declaration assessment below for the current finding.

Source: [wan_carriers.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/wan_carriers.schema.yml); declaration `eos_designs#/keys/wan_carriers/items/keys/path_group`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `carrier-selection-or-explicit-error`.

Stricter-rule finding: Local WAN carrier path_group uses get(required=True), rejecting null before output. Nonlocal/unselected carrier or non-WAN device may only perform comparisons/no association.

Permitted contexts from static analysis / saved observations: Carrier not selected by any local WAN interface.

Migration/impact: Supply path group or remove unused carrier.

Assessment evidence: [wan.py:161](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/wan.py:161), [cv_pathfinder.py:73](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/metadata/cv_pathfinder.py:73)

Schema occurrence paths:

- `avd_design.wan_carriers[].path_group`

### R642 — `wan_path_groups.id`

Type: `int`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `null-fallback`. Use the declaration assessment below for the current finding.

Source: [wan_path_groups.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/wan_path_groups.schema.yml); declaration `eos_designs#/keys/wan_path_groups/items/keys/id`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `numeric-fallback`.

Stricter-rule finding: Path group helper returns explicit non-null ID, otherwise 500; LAN HA group always returns 65535 regardless of configured ID. Null therefore selects defined valid integer fallback instead of failing.

Permitted contexts from static analysis / saved observations: Null ID replaced with 500. LAN HA ID fixed at 65535. Unused group.

Migration/impact: Explicit 500 preserves helper fallback for normal group; verify intended uniqueness/semantics instead of copying fallback blindly.

Assessment evidence: [router_path_selection.py:124](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/router_path_selection.py:124)

Saved method observation: [wan_path_group_null_id_loader_and_helper_fallback](probe-results.json#/methods/wan_path_group_null_id_loader_and_helper_fallback).

Schema occurrence paths:

- `avd_design.wan_path_groups[].id`

### R644 — `wan_virtual_topologies.vrfs.wan_vni`

Type: `int`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `handoff`. Use the declaration assessment below for the current finding.

Source: [wan_virtual_topologies.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/wan_virtual_topologies.schema.yml); declaration `eos_designs#/keys/wan_virtual_topologies/keys/vrfs/items/keys/wan_vni`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-downstream-field-or-suppression`.

Stricter-rule finding: Selected WAN-client VRF null VNI returns before VXLAN VRF mapping; WAN server appends name with null optional EOS VNI. Duplicate bookkeeping accepts a single None VNI but multiple conflicting None-VNI entries raise. CV Pathfinder metadata VNI is also optional (default VRF metadata forces 1). All five wan_vni code uses are covered.

Permitted contexts from static analysis / saved observations: Client null VNI suppresses mapping. Single server VRF with null VNI omits optional VXLAN/metadata VNI. Unused WAN VRF.

Migration/impact: Supply VNI for intended overlay use or remove unused WAN VRF; output schema acceptance does not prove correct routing.

Assessment evidence: [vxlan_interface.py:112](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/vxlan_interface.py:112), [vxlan_interface.py:140](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/vxlan_interface.py:140), [vxlan_interface.py:293](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/vxlan_interface.py:293), [cv_pathfinder.py:125](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/metadata/cv_pathfinder.py:125), [utils_wan.py:28](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/utils_wan.py:28)

Schema occurrence paths:

- `avd_design.wan_virtual_topologies.vrfs[].wan_vni`

### R648 — `zscaler_endpoints.primary.ip_address`

Type: `str`. Scope occurrences: 3. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `runtime-risk`. Use the declaration assessment below for the current finding.

Source: [zscaler_endpoints.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/zscaler_endpoints.schema.yml); declaration `eos_designs#/keys/zscaler_endpoints/keys/primary/keys/ip_address`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `invalid-formatting-and-required-metadata`.

Stricter-rule finding: Selected primary/secondary/tertiary endpoint null IP is copied to optional tunnel/monitor fields and formatted into static prefix 'None/32'. Endpoint metadata has required ip_address and retained location fields; stripping causes requiredness failure (prefix string alone has no null). Inactive Zscaler endpoint input is not consumed.

Permitted contexts from static analysis / saved observations: No active Zscaler exit.

Migration/impact: Supply endpoint IP or remove unused endpoint. Null primary scalar differs from null entire endpoint.

Assessment evidence: [router_internet_exit.py:200](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_internet_exit.py:200), [router_internet_exit.py:207](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_internet_exit.py:207), [router_internet_exit.py:215](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_internet_exit.py:215)

Schema occurrence paths:

- `avd_design.zscaler_endpoints.primary.ip_address`
- `avd_design.zscaler_endpoints.secondary.ip_address`
- `avd_design.zscaler_endpoints.tertiary.ip_address`

### R649 — `zscaler_endpoints.primary.datacenter`

Type: `str`. Scope occurrences: 3. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `metadata-handoff`. Use the declaration assessment below for the current finding.

Source: [zscaler_endpoints.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/zscaler_endpoints.schema.yml); declaration `eos_designs#/keys/zscaler_endpoints/keys/primary/keys/datacenter`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `inactive-feature-or-retained-required-metadata`.

Stricter-rule finding: Each selected endpoint (primary/secondary/tertiary) is cast into tunnel metadata. All location fields are required in EOS endpoint metadata; valid endpoint IP/other fields keep endpoint/tunnel alive, so stripped null location field causes output-validation error. These are CloudVision metadata, not CLI interpolation.

Permitted contexts from static analysis / saved observations: No active Zscaler exit consuming these endpoints.

Migration/impact: Supply endpoint location metadata or remove unused endpoint. No blanket omission exemption for non-CLI fields.

Assessment evidence: [router_internet_exit.py:215](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_internet_exit.py:215)

Schema occurrence paths:

- `avd_design.zscaler_endpoints.primary.datacenter`
- `avd_design.zscaler_endpoints.secondary.datacenter`
- `avd_design.zscaler_endpoints.tertiary.datacenter`

### R650 — `zscaler_endpoints.primary.city`

Type: `str`. Scope occurrences: 3. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `metadata-handoff`. Use the declaration assessment below for the current finding.

Source: [zscaler_endpoints.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/zscaler_endpoints.schema.yml); declaration `eos_designs#/keys/zscaler_endpoints/keys/primary/keys/city`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `inactive-feature-or-retained-required-metadata`.

Stricter-rule finding: Each selected endpoint (primary/secondary/tertiary) is cast into tunnel metadata. All location fields are required in EOS endpoint metadata; valid endpoint IP/other fields keep endpoint/tunnel alive, so stripped null location field causes output-validation error. These are CloudVision metadata, not CLI interpolation.

Permitted contexts from static analysis / saved observations: No active Zscaler exit consuming these endpoints.

Migration/impact: Supply endpoint location metadata or remove unused endpoint. No blanket omission exemption for non-CLI fields.

Assessment evidence: [router_internet_exit.py:215](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_internet_exit.py:215)

Schema occurrence paths:

- `avd_design.zscaler_endpoints.primary.city`
- `avd_design.zscaler_endpoints.secondary.city`
- `avd_design.zscaler_endpoints.tertiary.city`

### R651 — `zscaler_endpoints.primary.country`

Type: `str`. Scope occurrences: 3. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `metadata-handoff`. Use the declaration assessment below for the current finding.

Source: [zscaler_endpoints.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/zscaler_endpoints.schema.yml); declaration `eos_designs#/keys/zscaler_endpoints/keys/primary/keys/country`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `inactive-feature-or-retained-required-metadata`.

Stricter-rule finding: Each selected endpoint (primary/secondary/tertiary) is cast into tunnel metadata. All location fields are required in EOS endpoint metadata; valid endpoint IP/other fields keep endpoint/tunnel alive, so stripped null location field causes output-validation error. These are CloudVision metadata, not CLI interpolation.

Permitted contexts from static analysis / saved observations: No active Zscaler exit consuming these endpoints.

Migration/impact: Supply endpoint location metadata or remove unused endpoint. No blanket omission exemption for non-CLI fields.

Assessment evidence: [router_internet_exit.py:215](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_internet_exit.py:215)

Schema occurrence paths:

- `avd_design.zscaler_endpoints.primary.country`
- `avd_design.zscaler_endpoints.secondary.country`
- `avd_design.zscaler_endpoints.tertiary.country`

### R652 — `zscaler_endpoints.primary.region`

Type: `str`. Scope occurrences: 3. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `metadata-handoff`. Use the declaration assessment below for the current finding.

Source: [zscaler_endpoints.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/zscaler_endpoints.schema.yml); declaration `eos_designs#/keys/zscaler_endpoints/keys/primary/keys/region`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `inactive-feature-or-retained-required-metadata`.

Stricter-rule finding: Each selected endpoint (primary/secondary/tertiary) is cast into tunnel metadata. All location fields are required in EOS endpoint metadata; valid endpoint IP/other fields keep endpoint/tunnel alive, so stripped null location field causes output-validation error. These are CloudVision metadata, not CLI interpolation.

Permitted contexts from static analysis / saved observations: No active Zscaler exit consuming these endpoints.

Migration/impact: Supply endpoint location metadata or remove unused endpoint. No blanket omission exemption for non-CLI fields.

Assessment evidence: [router_internet_exit.py:215](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_internet_exit.py:215)

Schema occurrence paths:

- `avd_design.zscaler_endpoints.primary.region`
- `avd_design.zscaler_endpoints.secondary.region`
- `avd_design.zscaler_endpoints.tertiary.region`

### R653 — `zscaler_endpoints.primary.latitude`

Type: `str`. Scope occurrences: 3. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `metadata-handoff`. Use the declaration assessment below for the current finding.

Source: [zscaler_endpoints.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/zscaler_endpoints.schema.yml); declaration `eos_designs#/keys/zscaler_endpoints/keys/primary/keys/latitude`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `inactive-feature-or-retained-required-metadata`.

Stricter-rule finding: Each selected endpoint (primary/secondary/tertiary) is cast into tunnel metadata. All location fields are required in EOS endpoint metadata; valid endpoint IP/other fields keep endpoint/tunnel alive, so stripped null location field causes output-validation error. These are CloudVision metadata, not CLI interpolation.

Permitted contexts from static analysis / saved observations: No active Zscaler exit consuming these endpoints.

Migration/impact: Supply endpoint location metadata or remove unused endpoint. No blanket omission exemption for non-CLI fields.

Assessment evidence: [router_internet_exit.py:215](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_internet_exit.py:215)

Schema occurrence paths:

- `avd_design.zscaler_endpoints.primary.latitude`
- `avd_design.zscaler_endpoints.secondary.latitude`
- `avd_design.zscaler_endpoints.tertiary.latitude`

### R654 — `zscaler_endpoints.primary.longitude`

Type: `str`. Scope occurrences: 3. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `metadata-handoff`. Use the declaration assessment below for the current finding.

Source: [zscaler_endpoints.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/zscaler_endpoints.schema.yml); declaration `eos_designs#/keys/zscaler_endpoints/keys/primary/keys/longitude`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `inactive-feature-or-retained-required-metadata`.

Stricter-rule finding: Each selected endpoint (primary/secondary/tertiary) is cast into tunnel metadata. All location fields are required in EOS endpoint metadata; valid endpoint IP/other fields keep endpoint/tunnel alive, so stripped null location field causes output-validation error. These are CloudVision metadata, not CLI interpolation.

Permitted contexts from static analysis / saved observations: No active Zscaler exit consuming these endpoints.

Migration/impact: Supply endpoint location metadata or remove unused endpoint. No blanket omission exemption for non-CLI fields.

Assessment evidence: [router_internet_exit.py:215](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_internet_exit.py:215)

Schema occurrence paths:

- `avd_design.zscaler_endpoints.primary.longitude`
- `avd_design.zscaler_endpoints.secondary.longitude`
- `avd_design.zscaler_endpoints.tertiary.longitude`

### R655 — `zscaler_endpoints.cloud_name`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `runtime-risk`. Use the declaration assessment below for the current finding.

Source: [zscaler_endpoints.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/zscaler_endpoints.schema.yml); declaration `eos_designs#/keys/zscaler_endpoints/keys/cloud_name`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `non-null-invalid-formatting`.

Stricter-rule finding: Null cloud_name is interpolated into monitor URL http://gateway.None.net/vpntest, which is non-null and passes URL string type checks. Metadata policy has no separate cloud_name required-output safeguard. Inactive Zscaler exit skips use.

Permitted contexts from static analysis / saved observations: Active Zscaler monitor URL with literal None can be schema-valid (endpoint/location fields otherwise valid). No active Zscaler exit.

Migration/impact: Supply cloud name; EOS/CloudVision/network acceptance of malformed URL is unmeasured.

Assessment evidence: [monitor_connectivity.py:46](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/monitor_connectivity.py:46), [monitor_connectivity.py:61](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/monitor_connectivity.py:61)

Schema occurrence paths:

- `avd_design.zscaler_endpoints.cloud_name`

### R657 — `zscaler_endpoints.device_location.city`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `metadata-handoff`. Use the declaration assessment below for the current finding.

Source: [zscaler_endpoints.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/zscaler_endpoints.schema.yml); declaration `eos_designs#/keys/zscaler_endpoints/keys/device_location/keys/city`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `inactive-feature-or-retained-required-metadata`.

Stricter-rule finding: Active Zscaler policy reads device_location city/country; null dict exposes unset children. Nonempty policy credentials/tunnels/name keep parent alive, so stripping missing required city/country fails EOS metadata validation. Earlier real contributor observations confirm scalar city/country with valid nonempty tunnels.

Permitted contexts from static analysis / saved observations: No active Zscaler exit.

Migration/impact: Provide location or remove inactive endpoint parent; no collection-parent pruning exemption for retained active policy.

Assessment evidence: [metadata.py:39](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/metadata.py:39)

Previously confirmed observation: Earlier real Zscaler metadata contributor observation with valid nonempty tunnels: null city was copied; after stripping only required city was reported missing. Credential retrieval was stubbed.

Schema occurrence paths:

- `avd_design.zscaler_endpoints.device_location.city`

### R658 — `zscaler_endpoints.device_location.country`

Type: `str`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `metadata-handoff`. Use the declaration assessment below for the current finding.

Source: [zscaler_endpoints.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/zscaler_endpoints.schema.yml); declaration `eos_designs#/keys/zscaler_endpoints/keys/device_location/keys/country`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `inactive-feature-or-retained-required-metadata`.

Stricter-rule finding: Active Zscaler policy reads device_location city/country; null dict exposes unset children. Nonempty policy credentials/tunnels/name keep parent alive, so stripping missing required city/country fails EOS metadata validation. Earlier real contributor observations confirm scalar city/country with valid nonempty tunnels.

Permitted contexts from static analysis / saved observations: No active Zscaler exit.

Migration/impact: Provide location or remove inactive endpoint parent; no collection-parent pruning exemption for retained active policy.

Assessment evidence: [metadata.py:39](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/metadata.py:39)

Previously confirmed observation: Earlier real Zscaler metadata contributor observation with valid nonempty tunnels: null country was copied; after stripping only required country was reported missing. Credential retrieval was stubbed.

Schema occurrence paths:

- `avd_design.zscaler_endpoints.device_location.country`

## Collections — retained as a separate policy (28 declarations)

### R025 — `peer_filters.sequence_numbers`

Type: `list`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `template-collection-policy`. Use the declaration assessment below for the current finding.

Source: [peer_filters.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/peer_filters.schema.yml); declaration `eos_cli_config_gen#/keys/peer_filters/items/keys/sequence_numbers`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `inactive-catalog`.

Stricter-rule finding: Referenced peer filter is copied from the catalog. Its name/sequence primary keys retain parents: null sequence_numbers is removed and violates required collection; null match leaves a sequence without required match. Unused peer filters are not copied.

Permitted contexts from static analysis / saved observations: Unreferenced peer-filter catalog entry.

Migration/impact: Repair selected filters; remove unused invalid entries. [] does not restore a stripped required collection after empty stripping.

Assessment evidence: [router_bgp.py:876](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:876)

Schema occurrence paths:

- `avd_design.bgp_peer_filters_catalog[].sequence_numbers`

### R036 — `connected_endpoints.adapters.switch_ports`

Type: `list`. Scope occurrences: 3. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `explicit-error`. Use the declaration assessment below for the current finding.

Source: [connected_endpoints.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/connected_endpoints.schema.yml); declaration `eos_designs#/keys/connected_endpoints/items/keys/adapters/items/keys/switch_ports`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `host-filter-or-length-error`.

Stricter-rule finding: After adapter profile merging, host-selected switches plus null switch_ports produces a typed empty list and existing length mismatch error. Nonmatching switches/null switches filter adapter before that check. Static and dynamic endpoint keys share this implementation.

Permitted contexts from static analysis / saved observations: Adapter not connected to this host.

Migration/impact: Fix paired switch/port lists or remove unneeded adapter. [] gives the same active mismatch, not a universal repair.

Assessment evidence: [connected_endpoints.py:41](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/connected_endpoints.py:41), [connected_endpoints.py:47](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/connected_endpoints.py:47)

Schema occurrence paths:

- `avd_design.connected_endpoints[].adapters[].switch_ports`
- `avd_design.<dynamic:connected_endpoints_keys.key>[].adapters[].switch_ports`
- `avd_design.<dynamic:custom_connected_endpoints_keys.key>[].adapters[].switch_ports`

### R037 — `connected_endpoints.adapters.switches`

Type: `list`. Scope occurrences: 3. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `collection-suppression`. Use the declaration assessment below for the current finding.

Source: [connected_endpoints.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/connected_endpoints.schema.yml); declaration `eos_designs#/keys/connected_endpoints/items/keys/adapters/items/keys/switches`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `collection-host-suppression`.

Stricter-rule finding: Null switches becomes empty and filters adapter out for every host after profile merging. Port profiles do not declare switches, so do not describe this as overriding profile-provided switch membership.

Permitted contexts from static analysis / saved observations: Adapter deliberately/unintentionally excluded from all hosts.

Migration/impact: Remove unused adapter or provide host membership; [] is equivalent at this consumer if source length constraints permit it.

Assessment evidence: [connected_endpoints.py:41](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/connected_endpoints.py:41)

Previously confirmed observation: Earlier source/model comparison: null and [] switches both filtered the adapter; current port profiles have neither switches nor switch_ports.

Schema occurrence paths:

- `avd_design.connected_endpoints[].adapters[].switches`
- `avd_design.<dynamic:connected_endpoints_keys.key>[].adapters[].switches`
- `avd_design.<dynamic:custom_connected_endpoints_keys.key>[].adapters[].switches`

### R103 — `$defs.l3_edge.p2p_links.nodes`

Type: `list`. Scope occurrences: 2. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `collection-suppression`. Use the declaration assessment below for the current finding.

Source: [defs_l3_edge.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/defs_l3_edge.schema.yml); declaration `eos_designs#/$defs/l3_edge/keys/p2p_links/items/keys/nodes`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `collection-host-suppression`.

Stricter-rule finding: Both core_interfaces and l3_edge apply link profiles, then filter links by hostname membership in nodes. Null typed nodes list selects no device, before positional node access.

Permitted contexts from static analysis / saved observations: Link filtered out for all hosts.

Migration/impact: Remove unused link or provide both endpoints; preserve intentional profile tombstone behavior separately.

Assessment evidence: [utils.py:47](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/core_interfaces_and_l3_edge/utils.py:47), [utils.py:50](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/core_interfaces_and_l3_edge/utils.py:50)

Schema occurrence paths:

- `avd_design.core_interfaces.p2p_links[].nodes`
- `avd_design.l3_edge.p2p_links[].nodes`

### R110 — `cv_settings.onprem_clusters.servers`

Type: `list`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `collection-risk`. Use the declaration assessment below for the current finding.

Source: [cv_settings.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/cv_settings.schema.yml); declaration `eos_designs#/keys/cv_settings/keys/onprem_clusters/items/keys/servers`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `empty-output-or-runtime-failure`.

Stricter-rule finding: Typed-null servers yields empty cvaddrs. A SINGLE CloudVision cluster uses optional root daemon_terminattr.cvaddrs; empty list is stripped while authentication/VRF remain and can pass output requiredness with valid NTP dependencies. MULTIPLE clusters use required per-cluster cvaddrs, so empty addresses fail after stripping. In-band ZTP selection also calls next(iter(servers)) and fails.

Permitted contexts from static analysis / saved observations: Single on-prem cluster, valid NTP dependencies, no in-band ZTP: optional root address list omitted.

Migration/impact: Supply servers or remove unused cluster; empty-list migration must satisfy source constraints.

Assessment evidence: [daemon_terminattr.py:116](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/daemon_terminattr.py:116), [dhcp_servers.py:54](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/underlay/dhcp_servers.py:54)

Schema occurrence paths:

- `avd_design.cv_settings.onprem_clusters[].servers`

### R113 — `cv_topology.interfaces`

Type: `list`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `collection-suppression`. Use the declaration assessment below for the current finding.

Source: [cv_topology.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/cv_topology.schema.yml); declaration `eos_designs#/keys/cv_topology/items/keys/interfaces`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `collection-suppression`.

Stricter-rule finding: Topology interface list is iterated for management, MLAG and uplink derivation. Null/empty produces no derived interfaces, retaining possibility of other node configuration; topology lookup may be inactive.

Permitted contexts from static analysis / saved observations: No links derived from this topology entry. Topology disabled/host entry unused.

Migration/impact: Provide desired topology interfaces or remove unused entry; check source min_length before replacing null with [].

Assessment evidence: [cv_topology.py:71](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/cv_topology.py:71), [cv_topology.py:76](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/cv_topology.py:76)

Schema occurrence paths:

- `avd_design.cv_topology[].interfaces`

### R115 — `default_interfaces.types`

Type: `list`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `collection-suppression`. Use the declaration assessment below for the current finding.

Source: [default_interfaces.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/default_interfaces.schema.yml); declaration `eos_designs#/keys/default_interfaces/items/keys/types`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `collection-selection-fallback`.

Stricter-rule finding: Null types/platforms cannot match default-interface selection (empty regex iterable matches false). Search continues to other entries/default platform or returns empty model; explicit interface settings can still drive generation.

Permitted contexts from static analysis / saved observations: Alternative entry or explicit interface settings supply interfaces. Unused/nonmatching default-interface entry.

Migration/impact: Remove nonmatching entry or supply correct selectors; null selector is not a valid populated match.

Assessment evidence: [platform_mixin.py:71](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/platform_mixin.py:71), [utils.py:192](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/utils.py:192)

Schema occurrence paths:

- `avd_design.default_interfaces[].types`

### R116 — `default_interfaces.platforms`

Type: `list`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `collection-suppression`. Use the declaration assessment below for the current finding.

Source: [default_interfaces.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/default_interfaces.schema.yml); declaration `eos_designs#/keys/default_interfaces/items/keys/platforms`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `collection-selection-fallback`.

Stricter-rule finding: Null types/platforms cannot match default-interface selection (empty regex iterable matches false). Search continues to other entries/default platform or returns empty model; explicit interface settings can still drive generation.

Permitted contexts from static analysis / saved observations: Alternative entry or explicit interface settings supply interfaces. Unused/nonmatching default-interface entry.

Migration/impact: Remove nonmatching entry or supply correct selectors; null selector is not a valid populated match.

Assessment evidence: [platform_mixin.py:71](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/platform_mixin.py:71), [utils.py:192](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/utils.py:192)

Schema occurrence paths:

- `avd_design.default_interfaces[].platforms`

### R117 — `default_node_types.match_hostnames`

Type: `list`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `collection-suppression`. Use the declaration assessment below for the current finding.

Source: [default_node_types.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/default_node_types.schema.yml); declaration `eos_designs#/keys/default_node_types/items/keys/match_hostnames`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `collection-selection-fallback`.

Stricter-rule finding: Empty/null hostname patterns match no host. Explicit device/input type bypasses default-node-type lookup; another matching entry succeeds; absence of any type raises existing diagnostic.

Permitted contexts from static analysis / saved observations: Explicit device/input type. Another matching node-type entry.

Migration/impact: Remove unused matching rule or provide patterns. Empty list remains nonmatching but check source constraints.

Assessment evidence: [node_type.py:29](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/node_type.py:29), [node_type.py:43](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/node_type.py:43)

Schema occurrence paths:

- `avd_design.default_node_types[].match_hostnames`

### R169 — `ip_hosts.ipv4_addresses`

Type: `list`. Scope occurrences: 1. Other-scope occurrences excluded: 14. Initial consumer index: `candidate-only` / `template-collection-policy`. Use the declaration assessment below for the current finding.

Source: [ip_hosts.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_cli_config_gen/schema/schema_fragments/ip_hosts.schema.yml); declaration `eos_cli_config_gen#/keys/ip_hosts/items/keys/ipv4_addresses`.

Assessment: `reviewed`. Compatibility verdict: `active-path-already-rejected`. Output boundary: `static-analysis`.

Pattern: `retained-required-output`.

Stricter-rule finding: IP host entries are handed through. Hostname primary key keeps each item; null IPv4 addresses becomes an empty list removed by stripping, producing a missing required collection. No per-item host selection.

Permitted contexts from static analysis / saved observations: None established for the inspected regular active path; this is not a universal safety proof.

Migration/impact: Supply addresses; an empty list is also removed, so it is not a migration for a retained item.

Assessment evidence: [dns_settings.py:32](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/dns_settings.py:32)

Schema occurrence paths:

- `avd_design.dns_settings.ip_hosts[].ipv4_addresses`

### R577 — `$defs.node_type.defaults.evpn_gateway.all_active_multihoming.evpn_ethernet_segment`

Type: `dict`. Scope occurrences: 10. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `handoff`. Use the declaration assessment below for the current finding.

Source: [defs_node_type.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/defs_node_type.schema.yml); declaration `eos_designs#/$defs/node_type/keys/defaults/keys/evpn_gateway/keys/all_active_multihoming/keys/evpn_ethernet_segment`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-downstream-field`.

Stricter-rule finding: Enabled all-active gateway unconditionally reads segment children and creates domain='all'. Null segment produces an empty typed model with unset children; null identifier/rt_import values are stripped. EOS identifier and route_target_import are OPTIONAL, so retained domain entry alone does not cause required-output failure.

Permitted contexts from static analysis / saved observations: Active segment values omitted from schema-valid output (device behavior unverified). Gateway inactive.

Migration/impact: Specify complete segment for active all-active multihoming; retaining collection tombstones preserves this incomplete-output risk.

Assessment evidence: [router_bgp.py:393](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/router_bgp.py:393)

Schema occurrence paths:

- `avd_design.device_profiles[].evpn_gateway.all_active_multihoming.evpn_ethernet_segment`
- `avd_design.devices[].evpn_gateway.all_active_multihoming.evpn_ethernet_segment`
- `avd_design.<dynamic:custom_node_type_keys.key>.defaults.evpn_gateway.all_active_multihoming.evpn_ethernet_segment`
- `avd_design.<dynamic:custom_node_type_keys.key>.node_groups[].nodes[].evpn_gateway.all_active_multihoming.evpn_ethernet_segment`
- `avd_design.<dynamic:custom_node_type_keys.key>.node_groups[].evpn_gateway.all_active_multihoming.evpn_ethernet_segment`
- `avd_design.<dynamic:custom_node_type_keys.key>.nodes[].evpn_gateway.all_active_multihoming.evpn_ethernet_segment`
- `avd_design.<dynamic:node_type_keys.key>.defaults.evpn_gateway.all_active_multihoming.evpn_ethernet_segment`
- `avd_design.<dynamic:node_type_keys.key>.node_groups[].nodes[].evpn_gateway.all_active_multihoming.evpn_ethernet_segment`
- `avd_design.<dynamic:node_type_keys.key>.node_groups[].evpn_gateway.all_active_multihoming.evpn_ethernet_segment`
- `avd_design.<dynamic:node_type_keys.key>.nodes[].evpn_gateway.all_active_multihoming.evpn_ethernet_segment`

### R586 — `digital_twin.fabric`

Type: `dict`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `collection-risk`. Use the declaration assessment below for the current finding.

Source: [digital_twin.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/digital_twin.schema.yml); declaration `eos_designs#/keys/digital_twin/keys/fabric`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `collection-default-backed`.

Stricter-rule finding: Null fabric loads empty typed model. ACT accesses default-backed credentials/internet/eAPI flags, OS version uses node/platform fallback, and metadata credentials are optional. With valid node/platform management prerequisites this can pass the schema boundary; null fabric does not disable ACT. Non-Digital-Twin paths do not use it.

Permitted contexts from static analysis / saved observations: Digital Twin disabled. ACT uses class defaults and OS-version fallback (ACT API acceptance unmeasured).

Migration/impact: Prefer explicit fabric settings or an explicitly disabled Digital Twin; {} versus null has different inheritance behavior.

Assessment evidence: [__init__.py:561](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/__init__.py:561), [digital_twin.py:65](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/metadata/digital_twin.py:65)

Schema occurrence paths:

- `avd_design.digital_twin.fabric`

### R587 — `dns_settings.servers`

Type: `list`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `collection-suppression`. Use the declaration assessment below for the current finding.

Source: [dns_settings.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/dns_settings.schema.yml); declaration `eos_designs#/keys/dns_settings/keys/servers`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `collection-suppression`.

Stricter-rule finding: Null DNS server list yields no name-server entries; DNS domain/IP hosts remain independent. CloudVision dependency validation requires DNS in some active exporter paths, so active use can also fail.

Permitted contexts from static analysis / saved observations: No DNS-dependent CloudVision use; DNS server generation omitted.

Migration/impact: Supply servers if needed, otherwise remove unused dns_settings parent; check constraints before [] replacement.

Assessment evidence: [dns_settings.py:42](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/dns_settings.py:42), [daemon_terminattr.py:171](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/daemon_terminattr.py:171)

Schema occurrence paths:

- `avd_design.dns_settings.servers`

### R599 — `ipv4_acls.entries`

Type: `list`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `collection-risk`. Use the declaration assessment below for the current finding.

Source: [ipv4_acls.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/ipv4_acls.schema.yml); declaration `eos_designs#/keys/ipv4_acls/items/keys/entries`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-downstream-collection`.

Stricter-rule finding: Selected ACL is cast into EOS ACL with name. Null entries list is empty and stripped; EOS entries is optional, so schema validation does not reject named ACL with no entries. Legacy IPv6 sequence_numbers may independently remain. Generic interface getters may inspect other entry fields only when entries exist.

Permitted contexts from static analysis / saved observations: Named ACL with empty entries omitted from output. Unselected ACL catalog.

Migration/impact: Use explicit intended ACL or remove catalog entry; an empty ACL can have traffic-policy consequences not measured by schema acceptance.

Assessment evidence: [utils.py:42](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/structured_config_utils/utils.py:42), [utils.py:46](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/structured_config_utils/utils.py:46), [misc.py:260](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/misc.py:260)

Schema occurrence paths:

- `avd_design.ipv4_acls[].entries`

### R600 — `ipv4_prefix_list_catalog.sequence_numbers`

Type: `list`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `collection-suppression`. Use the declaration assessment below for the current finding.

Source: [ipv4_prefix_lists.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/ipv4_prefix_lists.schema.yml); declaration `eos_designs#/keys/ipv4_prefix_list_catalog/items/keys/sequence_numbers`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-downstream-collection`.

Stricter-rule finding: Referenced prefix-list is cast into EOS catalog item. Null sequence_numbers is stripped, leaving the name; EOS sequence_numbers is optional. Unused prefix-list catalogs are not generated.

Permitted contexts from static analysis / saved observations: Named prefix list without sequence entries passes requiredness checks. Unreferenced catalog.

Migration/impact: Supply desired sequences or remove unused list; traffic-policy consequences of empty named lists require operator review.

Assessment evidence: [misc.py:308](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/misc.py:308), [misc.py:314](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/misc.py:314)

Schema occurrence paths:

- `avd_design.ipv4_prefix_list_catalog[].sequence_numbers`

### R602 — `ipv6_acls.entries`

Type: `list`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `collection-risk`. Use the declaration assessment below for the current finding.

Source: [ipv6_acls.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/ipv6_acls.schema.yml); declaration `eos_designs#/keys/ipv6_acls/items/keys/entries`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-downstream-collection`.

Stricter-rule finding: Selected ACL is cast into EOS ACL with name. Null entries list is empty and stripped; EOS entries is optional, so schema validation does not reject named ACL with no entries. Legacy IPv6 sequence_numbers may independently remain. Generic interface getters may inspect other entry fields only when entries exist.

Permitted contexts from static analysis / saved observations: Named ACL with empty entries omitted from output. Unselected ACL catalog.

Migration/impact: Use explicit intended ACL or remove catalog entry; an empty ACL can have traffic-policy consequences not measured by schema acceptance.

Assessment evidence: [utils.py:42](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/structured_config_utils/utils.py:42), [utils.py:46](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/structured_config_utils/utils.py:46), [misc.py:260](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/misc.py:260)

Schema occurrence paths:

- `avd_design.ipv6_acls[].entries`

### R603 — `ipv6_prefix_list_catalog.sequence_numbers`

Type: `list`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `collection-suppression`. Use the declaration assessment below for the current finding.

Source: [ipv6_prefix_lists.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/ipv6_prefix_lists.schema.yml); declaration `eos_designs#/keys/ipv6_prefix_list_catalog/items/keys/sequence_numbers`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `optional-downstream-collection`.

Stricter-rule finding: Referenced prefix-list is cast into EOS catalog item. Null sequence_numbers is stripped, leaving the name; EOS sequence_numbers is optional. Unused prefix-list catalogs are not generated.

Permitted contexts from static analysis / saved observations: Named prefix list without sequence entries passes requiredness checks. Unreferenced catalog.

Migration/impact: Supply desired sequences or remove unused list; traffic-policy consequences of empty named lists require operator review.

Assessment evidence: [misc.py:308](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/misc.py:308), [misc.py:314](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/misc.py:314)

Schema occurrence paths:

- `avd_design.ipv6_prefix_list_catalog[].sequence_numbers`

### R609 — `monitor_connectivity.interface_sets.interfaces`

Type: `list`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `collection-risk`. Use the declaration assessment below for the current finding.

Source: [monitor_connectivity.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/monitor_connectivity.schema.yml); declaration `eos_designs#/keys/monitor_connectivity/keys/interface_sets/items/keys/interfaces`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `empty-non-null-output`.

Stricter-rule finding: Both root and VRF interface sets join null typed list to empty string ''. This is not stripped as None; output schema has no nonempty constraint on interfaces (root required, VRF optional). Thus required-null rejection changes accepted output, with device semantics unverified.

Permitted contexts from static analysis / saved observations: Interface set emitted with empty interfaces string.

Migration/impact: Supply interface names or remove unused set and its references; [] yields same consumer string only if source constraints permit.

Assessment evidence: [monitor_connectivity.py:35](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/monitor_connectivity.py:35), [monitor_connectivity.py:56](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/monitor_connectivity.py:56)

Schema occurrence paths:

- `avd_design.monitor_connectivity.interface_sets[].interfaces`

### R610 — `monitor_connectivity.vrfs.interface_sets.interfaces`

Type: `list`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `collection-risk`. Use the declaration assessment below for the current finding.

Source: [monitor_connectivity.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/monitor_connectivity.schema.yml); declaration `eos_designs#/keys/monitor_connectivity/keys/vrfs/items/keys/interface_sets/items/keys/interfaces`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `empty-non-null-output`.

Stricter-rule finding: Both root and VRF interface sets join null typed list to empty string ''. This is not stripped as None; output schema has no nonempty constraint on interfaces (root required, VRF optional). Thus required-null rejection changes accepted output, with device semantics unverified.

Permitted contexts from static analysis / saved observations: Interface set emitted with empty interfaces string.

Migration/impact: Supply interface names or remove unused set and its references; [] yields same consumer string only if source constraints permit.

Assessment evidence: [monitor_connectivity.py:35](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/monitor_connectivity.py:35), [monitor_connectivity.py:56](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/base/monitor_connectivity.py:56)

Schema occurrence paths:

- `avd_design.monitor_connectivity.vrfs[].interface_sets[].interfaces`

### R632 — `network_services.point_to_point_services.endpoints.nodes`

Type: `list`. Scope occurrences: 2. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `collection-risk`. Use the declaration assessment below for the current finding.

Source: [network_services.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/network_services.schema.yml); declaration `eos_designs#/keys/network_services/items/keys/point_to_point_services/items/keys/endpoints/items/keys/nodes`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `collection-host-suppression`.

Stricter-rule finding: Null nodes list excludes endpoint in patch-panel and local pseudowire generation. As a remote endpoint its ID may still be read, so null nodes is not proof of entire service inactivity. No nodes membership null failure itself.

Permitted contexts from static analysis / saved observations: Endpoint not locally connected; remote endpoint ID can remain valid.

Migration/impact: Remove unused endpoint/service or populate nodes, checking paired endpoint and remote ID semantics.

Assessment evidence: [router_bgp.py:815](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:815), [patch_panel.py:39](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/patch_panel.py:39)

Schema occurrence paths:

- `avd_design.network_services[].point_to_point_services[].endpoints[].nodes`
- `avd_design.<dynamic:network_services_keys.name>[].point_to_point_services[].endpoints[].nodes`

### R633 — `network_services.point_to_point_services.endpoints.interfaces`

Type: `list`. Scope occurrences: 2. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `collection-risk`. Use the declaration assessment below for the current finding.

Source: [network_services.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/network_services.schema.yml); declaration `eos_designs#/keys/network_services/items/keys/point_to_point_services/items/keys/endpoints/items/keys/interfaces`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `collection-index-error-or-suppression`.

Stricter-rule finding: Pseudowire path skips local endpoint with empty interfaces; patch-panel path indexes interfaces[node_index] after selecting nodes and raises IndexError for null/empty. Host-inactive endpoints bypass both; network-services L1/feature gates select consumer.

Permitted contexts from static analysis / saved observations: Endpoint nodes do not select host. Pseudowire-only path may skip empty local interfaces, provided other service dependencies succeed.

Migration/impact: Populate paired interfaces/nodes or remove endpoint; [] repeats active index error.

Assessment evidence: [router_bgp.py:815](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_bgp.py:815), [patch_panel.py:39](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/patch_panel.py:39)

Schema occurrence paths:

- `avd_design.network_services[].point_to_point_services[].endpoints[].interfaces`
- `avd_design.<dynamic:network_services_keys.name>[].point_to_point_services[].endpoints[].interfaces`

### R638 — `underlay_ospf_authentication.message_digest_keys`

Type: `list`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `collection-suppression`. Use the declaration assessment below for the current finding.

Source: [underlay_ospf_authentication.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/underlay_ospf_authentication.schema.yml); declaration `eos_designs#/keys/underlay_ospf_authentication/keys/message_digest_keys`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `collection-authentication-suppression`.

Stricter-rule finding: Null digest-key list loads empty and iteration creates no keys even when authentication enabled. Downstream interface digest-key list is optional; schema does not ensure working authenticated adjacency. Source [] fails min_length:1 even when enabled:false, unlike explicit null.

Permitted contexts from static analysis / saved observations: Authentication enabled with no generated digest keys can pass schema. Authentication disabled.

Migration/impact: Do not prescribe []: it violates source min_length. Disable/remove whole authentication parent or supply actual keys; collections merit separate policy.

Assessment evidence: [ethernet_interfaces.py:105](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/underlay/ethernet_interfaces.py:105), [__init__.py:129](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/mlag/__init__.py:129)

Previously confirmed observation: Earlier input validation comparison: null keys accepted with enabled:true; [] failed min_length:1 even with enabled:false.

Schema occurrence paths:

- `avd_design.underlay_ospf_authentication.message_digest_keys`

### R641 — `wan_ipsec_profiles.control_plane`

Type: `dict`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `collection-risk`. Use the declaration assessment below for the current finding.

Source: [wan_ipsec_profiles.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/wan_ipsec_profiles.schema.yml); declaration `eos_designs#/keys/wan_ipsec_profiles/keys/control_plane`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `inactive-feature-or-explicit-error`.

Stricter-rule finding: WAN IPsec consumer reads typed null control_plane defaults, then raises missing shared_key/cleartext_shared_key. This is not a raw-None dereference. Non-WAN contributor returns first.

Permitted contexts from static analysis / saved observations: Non-WAN device.

Migration/impact: Supply valid control-plane profile/key or remove unused WAN IPsec parent; {} is not an active repair without credentials.

Assessment evidence: [ip_security.py:30](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/ip_security.py:30), [ip_security.py:76](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/ip_security.py:76)

Schema occurrence paths:

- `avd_design.wan_ipsec_profiles.control_plane`

### R643 — `wan_route_servers.path_groups.interfaces`

Type: `list`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `collection-suppression`. Use the declaration assessment below for the current finding.

Source: [wan_route_servers.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/wan_route_servers.schema.yml); declaration `eos_designs#/keys/wan_route_servers/items/keys/path_groups/items/keys/interfaces`.

Assessment: `reviewed`. Compatibility verdict: `behavior-changing-rejection`. Output boundary: `static-analysis`.

Pattern: `collection-suppression-or-interface-fallback`.

Stricter-rule finding: Null interfaces on a declared route-server path group produces no public IPs for static-peer discovery; optional output ipv4_addresses is removed. Own-server public-IP lookup falls back to local interface settings. Peer facts reconstruct whole path_groups only when the entire path_groups collection is falsy, NOT each null interfaces child.

Permitted contexts from static analysis / saved observations: Own route server uses local interface public_ip/IP fallback. Static peer has no generated IPv4 addresses (device connectivity unverified). Unselected route server/path group.

Migration/impact: Supply static peer interfaces or rely explicitly on supported local/fact fallback; do not claim child-level fact inheritance.

Assessment evidence: [wan.py:207](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/wan.py:207), [wan.py:344](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/shared_utils/wan.py:344), [router_path_selection.py:202](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/overlay/router_path_selection.py:202)

Schema occurrence paths:

- `avd_design.wan_route_servers[].path_groups[].interfaces`

### R645 — `$defs.virtual_topology.path_groups.names`

Type: `list`. Scope occurrences: 3. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `collection-suppression`. Use the declaration assessment below for the current finding.

Source: [defs_virtual_topology.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/defs_virtual_topology.schema.yml); declaration `eos_designs#/$defs/virtual_topology/keys/path_groups/items/keys/names`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `collection-empty-policy`.

Stricter-rule finding: All topology variants iterate names to add usable load-balance path groups. Null names adds none. Other entries/HA path group can still make policy usable. Empty control-plane policy errors; application/default policy may skip matches or error when no matches remain.

Permitted contexts from static analysis / saved observations: Other valid policy entries or HA path group retain usable policy. Unused topology/policy.

Migration/impact: Populate path-group names or remove unused entry; [] may violate source min length and entire policy outcome depends on other entries.

Assessment evidence: [utils_wan.py:194](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/utils_wan.py:194), [utils_wan.py:218](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/utils_wan.py:218), [router_adaptive_virtual_topology.py:71](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_adaptive_virtual_topology.py:71)

Schema occurrence paths:

- `avd_design.wan_virtual_topologies.control_plane_virtual_topology.path_groups[].names`
- `avd_design.wan_virtual_topologies.policies[].application_virtual_topologies[].path_groups[].names`
- `avd_design.wan_virtual_topologies.policies[].default_virtual_topology.path_groups[].names`

### R646 — `wan_virtual_topologies.policies.default_virtual_topology`

Type: `dict`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `collection-risk`. Use the declaration assessment below for the current finding.

Source: [wan_virtual_topologies.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/wan_virtual_topologies.schema.yml); declaration `eos_designs#/keys/wan_virtual_topologies/keys/policies/items/keys/default_virtual_topology`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `policy-selection-or-explicit-error`.

Stricter-rule finding: Selected WAN policy verification explicitly rejects falsy default_virtual_topology; typed-null model initially falsy. Default child access may alter truthiness, but absent path_groups/drop_unmatched is subsequently rejected. Unselected policies/non-WAN paths remain unused.

Permitted contexts from static analysis / saved observations: Unselected WAN policy.

Migration/impact: Provide valid default topology (drop_unmatched or path_groups) or remove unused policy.

Assessment evidence: [utils_wan.py:105](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/utils_wan.py:105), [router_path_selection.py:108](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_path_selection.py:108), [router_adaptive_virtual_topology.py:171](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_adaptive_virtual_topology.py:171)

Schema occurrence paths:

- `avd_design.wan_virtual_topologies.policies[].default_virtual_topology`

### R647 — `zscaler_endpoints.primary`

Type: `dict`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `collection-suppression`. Use the declaration assessment below for the current finding.

Source: [zscaler_endpoints.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/zscaler_endpoints.schema.yml); declaration `eos_designs#/keys/zscaler_endpoints/keys/primary`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `collection-endpoint-suppression`.

Stricter-rule finding: Null primary endpoint is falsy and skipped; valid secondary/tertiary can retain required nonempty metadata tunnels. All endpoints absent instead strips required tunnels and fails. Primary has no independent output 'primary required' field. Active alternate-only path still requires valid credentials/device location and sufficient tunnel ID range.

Permitted contexts from static analysis / saved observations: Valid alternate endpoint, credentials/location and tunnel range retain required nonempty metadata. No active Zscaler exit.

Migration/impact: Supply primary endpoint or explicitly review alternate-only intent; scalar-only policy leaves this collection null available.

Assessment evidence: [router_internet_exit.py:182](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/router_internet_exit.py:182), [metadata.py:35](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/metadata.py:35)

Schema occurrence paths:

- `avd_design.zscaler_endpoints.primary`

### R656 — `zscaler_endpoints.device_location`

Type: `dict`. Scope occurrences: 1. Other-scope occurrences excluded: 0. Initial consumer index: `inspected-site` / `metadata-handoff`. Use the declaration assessment below for the current finding.

Source: [zscaler_endpoints.schema.yml](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/schema/schema_fragments/zscaler_endpoints.schema.yml); declaration `eos_designs#/keys/zscaler_endpoints/keys/device_location`.

Assessment: `reviewed`. Compatibility verdict: `mixed-rejection`. Output boundary: `static-analysis`.

Pattern: `inactive-feature-or-retained-required-metadata`.

Stricter-rule finding: Active Zscaler policy reads device_location city/country; null dict exposes unset children. Nonempty policy credentials/tunnels/name keep parent alive, so stripping missing required city/country fails EOS metadata validation. Earlier real contributor observations confirm scalar city/country with valid nonempty tunnels.

Permitted contexts from static analysis / saved observations: No active Zscaler exit.

Migration/impact: Provide location or remove inactive endpoint parent; no collection-parent pruning exemption for retained active policy.

Assessment evidence: [metadata.py:39](/home/holbech/repos/avd/python-avd/pyavd/_eos_designs/structured_config/network_services/metadata.py:39)

Schema occurrence paths:

- `avd_design.zscaler_endpoints.device_location`

