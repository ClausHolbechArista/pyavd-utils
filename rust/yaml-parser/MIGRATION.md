<!--
  ~ Copyright (c) 2026 Arista Networks, Inc.
  ~ Use of this source code is governed by the Apache License 2.0
  ~ that can be found in the LICENSE file.
  -->

# Replacing production serde_yaml

The compatibility target is the repository's existing schema-loading,
schema-dumping, and Markdown-generation uses of `serde_yaml`. General parser
parity and serializer configuration are separate work.

## Dependency boundary

The shared data-view traits, path walker, and JSON implementations live in
`schema-data`. Consumers import them directly; `avdschema` has no re-exports.
This lets `avdschema` use `yaml-parser` without a production dependency cycle.
The parser retains an `avdschema` development dependency for integration tests.

The parser's optional `schema-data` feature enables the shared data-view
integration. The former `avdschema` and `avdschema-core` feature names are removed.
Consumers select `avdschema` capabilities directly on their own dependency.

`serde_yaml` and `unsafe-libyaml` are removed from production dependency graphs.
They remain in the lockfile for development-only comparisons and benchmarks.

The workspace advisory check includes development dependencies, so it still
reports `RUSTSEC-2024-0449` for `unsafe-libyaml` through these references. This
existing advisory remains after the production migration; removing the reference
backend or agreeing on a dependency-policy exception is separate follow-up work.

## Migrated callers

- Schema files and stdin deserialize into `SourceRootSchema` and `StoreSource`,
  which feed compilation, model generation, and documentation generation.
- Markdown examples serialize compiled defaults through `YamlValue`, a small
  Serde adapter preserving the primitive representation previously constructed
  with `serde_yaml::Value`. There is no JSON intermediate. The Markdown-specific
  `wrap_yaml_plain_scalars` helper is removed.
- `Dump::to_yaml_file`, including YAML destinations passed to `Dump::to_file`,
  uses the new serializer. Repository callers are tests; no Python production
  caller was found.

Runtime validation and the LSP do not use the YAML serializer.

## Required parser fixes

### Unsigned schema defaults

An internally tagged recursive `SourceSchema` buffers defaults through Serde.
Previously, integers between `i64::MAX + 1` and `u64::MAX` were delivered as
`i128`, which that buffer rejects. The resolver now tries `u64` after positive
`i64` parsing fails, before trying wider integer representations. Decimal,
hexadecimal, octal, and explicitly tagged integers share this behavior.

The common `i64` path is unchanged. Validation still rejects integers beyond
its `i64` contract. The compiler already preserves unsigned defaults as archived
`SchemaValueView::U64`; recursive schema-loading tests verify this through
`StoreSource` and `Store::compile`, including `u64::MAX`. Both schema readers
reject `u64::MAX + 1` for this target.

### Empty collections

Empty block collections previously emitted no value and reloaded as null.
The writer now always uses `[]` or `{}` for an empty collection, even when block
style was requested. Regressions cover nested defaults, collection properties,
complex keys, Serde loading, and schema-file dump/load.

### Quoting and list layout

Safe single-line strings and keys use plain style. YAML 1.2 type-like strings
remain quoted. Defensive quoting also covers YAML 1.1 spellings such as `yes`,
`ON`, `1:20`, dates, and merge/value keys. Syntax-sensitive strings, multiline
strings, and controls use escaped double quotes. These lexical checks affect
serialization only, without numeric parsing or regular expressions.

Block mapping list items put their first scalar key beside `-` and retain
extra list indentation. Mapping properties and first alias/complex keys keep
their existing separate-marker layout. Regressions verify null list entries
before and after a mapping whose first value is null.

The existing Markdown fixtures are the reference. Only the `profiles.md`
fixture changes: three lines receive the accepted extra list indentation,
without changes to values or wrapping. The other two fixtures remain identical.

```yaml
profiles:
  - name: default
    settings:
      long_value: flexible exact-match 16384 l2-shared 98304 l3-shared 131072
```

## API and remaining discrepancies

- YAML loading and dumping expose `DeError` and `SerError` in place of
  `serde_yaml::Error` in the corresponding wrapper variants.
- Empty schema input remains rejected; the new error is `NoDocument`.
- Serialization helpers require sized `T`. `Dump` passes `&self` so its existing
  support for unsized implementations is retained.
- Direct generic `f64` deserialization rejects an integer scalar such as `42`,
  where `serde_yaml` converts it. Migrated schema targets do not deserialize
  directly into `f64`.
- Empty input into generic `serde_json::Value` reports `NoDocument` instead of
  null. Both readers reject empty input for the migrated schema targets.
- A compact first alias key (`- *key : value`) is rejected by our parser;
  the explicit-key equivalent works. The writer retains its supported layout.
- Reading a serialized JSON-number wrapper directly into `serde_json::Value`
  differs between readers. No production workflow using this path was found.[^numbers]

No serializer configuration API is added. Broader Python-formatting alignment
and unrelated generic parser differences remain separate work.

## Compatibility evidence

- All 499 current AVD YAML fragments matched through named `StoreSource` inputs.
- Recursive unsigned defaults survive loading and compilation without loss.
- Parser regressions cover empty collections, defensive quoting, controls,
  mapping layout, and null list entries through both Serde readers.
- A cached PyYAML 6.0.3 SafeLoader preserved exact string types and values for
  198 scalar cases, plus keys, values, sequence items, and nested collections.
- Generated Markdown matches the reviewed fixtures with only the indentation
  adjustment described above.

[^numbers]: With `serde_json`'s `arbitrary_precision` feature, both serializers
    emit `$serde_json::private::Number` mappings for numbers in
    `serde_json::Value`. This existing shared behavior is not a migration fix.
