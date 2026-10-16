<!--
  ~ Copyright (c) 2026 Arista Networks, Inc.
  ~ Use of this source code is governed by the Apache License 2.0
  ~ that can be found in the LICENSE file.
  -->

# Shared test schema store

`schemas.json` is the local source store used by the Rust and Python schema-store,
validation, and generation tests. Tests do not download AVD release schemas.
Python tests compile this source with the installed `pyavd-utils-gen` package;
Rust tests locate the same file through the dependency-free `test_schema_store`
helper crate. No schema build command or Cargo installation is needed for pytest.

The store contains:

- `avd_design` and `eos_config`: small schemas preserving the paths and behavior
  exercised by the existing binding tests, including primary keys, integer to
  string coercion, unknown keys, and ignored EOS Config keys.
- `eos_cli_config_gen`: the reference target used by model-generation tests.
- `schema_generation_fixture`: representative models and cross-schema references.
- `schema_documentation_fixture`: the source for the captured Markdown outputs.

These are test fixtures, not complete copies of the AVD schemas or an exhaustive
schema-option coverage suite. Add fields when a test needs them. Focused inline
schemas remain beside the tests that exercise specific failures or combinations.
Expected generated outputs remain in `tests/schema_generation/artifacts`.

Full AVD schema benchmarks belong in AVD, where the schema sources are available.
The YAML parser's local benchmarks are independent of this store.
