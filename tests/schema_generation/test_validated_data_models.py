# Copyright (c) 2026 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.

import ast
import json
from pathlib import Path

import pytest

from pyavd_utils_gen.validated_data_generation import generate_validated_data_models

ARTIFACTS = Path(__file__).parent / "artifacts"


def _rust_sources(root: Path) -> dict[str, str]:
    """Read one generated Rust module tree using stable relative names."""
    module_directory = root.with_suffix("")
    sources = {"__root__.rs": root.read_text(encoding="UTF-8")}
    sources.update((str(path.relative_to(module_directory)), path.read_text(encoding="UTF-8")) for path in sorted(module_directory.rglob("*.rs")))
    return sources


def _combined_rust_source(root: Path) -> str:
    """Combine generated Rust files for assertions independent of file boundaries."""
    return "\n".join(_rust_sources(root).values())


def test_generate_validated_data_models(tmp_path: Path) -> None:
    """Generate nominal Rust views and Python declarations from the shared fixture."""
    rust = tmp_path / "models.rs"
    pyi = tmp_path / "models.pyi"

    generate_validated_data_models(
        ARTIFACTS / "schemas.json",
        "schema_generation_fixture",
        rust,
        pyi,
        "SchemaGenerationFixture",
        "SchemaGenerationFixture",
    )

    rust_source = _combined_rust_source(rust)
    native_source = (rust.with_suffix("") / "native.rs").read_text(encoding="UTF-8")
    pyi_source = pyi.read_text(encoding="UTF-8")
    assert (rust.with_suffix("") / "schema_generation_fixture.rs").is_file()
    assert (rust.with_suffix("") / "schema_generation_fixture" / "interface_profiles.rs").is_file()
    assert "pub const REGISTRY: ::validated_data::ModelRegistry" in rust_source
    assert "pub mod schema_generation_fixture;" in rust_source
    assert "#[::validated_data::data_view(indexed_list, primary_key(name))]" in rust_source
    assert "#[derive(Clone, Copy, Debug)]" not in rust_source
    assert "pub struct InterfaceProfiles<'a, Mode>" in rust_source
    assert "pub mod interface_profiles;" in rust_source
    assert "pub struct Item<'a, Mode>" in rust_source
    assert "pub interface_profiles: ::validated_data::Field<" in rust_source
    assert "primary_key(name)" in rust_source
    assert '"SchemaGenerationFixture" => dict {' in native_source
    assert '"SchemaGenerationFixtureInterfaceProfilesList" => indexed(' in native_source
    assert "def __getitem__(self, key: str)" in pyi_source
    assert "class SchemaGenerationFixture:" in pyi_source
    assert "class StrValue:" not in pyi_source
    assert "Sequence[str | None]" in pyi_source
    assert "-> str | UndefinedType | None" in pyi_source
    assert not (tmp_path / "models.py").exists()
    ast.parse(pyi_source)


def test_generate_validated_data_models_projects_static_root_keys(tmp_path: Path) -> None:
    """Keep selected branches complete and preserve their registry identities."""
    full_rust = tmp_path / "full.rs"
    full_pyi = tmp_path / "full.pyi"
    projected_rust = tmp_path / "projected.rs"
    projected_pyi = tmp_path / "projected.pyi"
    reordered_rust = tmp_path / "reordered.rs"
    reordered_pyi = tmp_path / "reordered.pyi"

    generate_validated_data_models(
        ARTIFACTS / "schemas.json",
        "schema_generation_fixture",
        full_rust,
        full_pyi,
        "SchemaGenerationFixture",
        "SchemaGenerationFixture",
    )
    generate_validated_data_models(
        ARTIFACTS / "schemas.json",
        "schema_generation_fixture",
        projected_rust,
        projected_pyi,
        "SchemaGenerationFixture",
        "SchemaGenerationFixture",
        root_keys=["accounting", "interface_profiles"],
    )
    generate_validated_data_models(
        ARTIFACTS / "schemas.json",
        "schema_generation_fixture",
        reordered_rust,
        reordered_pyi,
        "SchemaGenerationFixture",
        "SchemaGenerationFixture",
        root_keys=["interface_profiles", "accounting"],
    )

    full_source = _combined_rust_source(full_rust)
    projected_source = _combined_rust_source(projected_rust)
    projected_pyi_source = projected_pyi.read_text(encoding="UTF-8")
    assert "pub accounting: ::validated_data::Field<" in projected_source
    assert "pub methods: ::validated_data::Field<" in projected_source
    assert "pub authentication: ::validated_data::Field<" not in projected_source
    assert "def accounting(" in projected_pyi_source
    assert "def authentication(" not in projected_pyi_source
    assert projected_source != full_source
    assert _rust_sources(projected_rust) == _rust_sources(reordered_rust)
    assert (projected_rust.with_suffix("") / "native.rs").read_bytes() == (reordered_rust.with_suffix("") / "native.rs").read_bytes()
    assert projected_pyi_source == reordered_pyi.read_text(encoding="UTF-8")

    full_accounting = next(line for line in full_source.splitlines() if "pub accounting: ::validated_data::Field<" in line)
    projected_accounting = next(line for line in projected_source.splitlines() if "pub accounting: ::validated_data::Field<" in line)
    assert projected_accounting == full_accounting
    full_hash = next(line for line in full_source.splitlines() if line.startswith("const REGISTRY_HASH"))
    projected_hash = next(line for line in projected_source.splitlines() if line.startswith("const REGISTRY_HASH"))
    assert projected_hash != full_hash
    ast.parse(projected_pyi_source)


def test_generate_validated_data_models_reuses_pure_cross_schema_references(tmp_path: Path) -> None:
    """Generate a referenced schema once and point pure cross-schema fields at its views."""
    source = tmp_path / "schemas.json"
    source.write_text(
        json.dumps(
            {
                "external": {"type": "dict", "keys": {"shared": {"type": "dict", "keys": {"name": {"type": "str"}}}}},
                "root": {"type": "dict", "keys": {"value": {"type": "dict", "$ref": "external#/keys/shared"}}},
            }
        ),
        encoding="UTF-8",
    )
    rust = tmp_path / "models.rs"
    pyi = tmp_path / "models.pyi"

    generate_validated_data_models(
        source,
        "root",
        rust,
        pyi,
        "Root",
        "Root",
        reused_schemas={"external": ("External", "External")},
    )

    rust_source = _combined_rust_source(rust)
    native_source = (rust.with_suffix("") / "native.rs").read_text(encoding="UTF-8")
    assert "pub mod external;" in rust_source
    assert rust_source.count("pub struct Shared<'a, Mode>") == 1
    assert "super::external::Shared<'a, Mode>" in rust_source
    assert '"ExternalShared" => dict {' in native_source
    assert '"RootValue" => dict {' not in native_source
    assert 'Target::Model("ExternalShared")' in native_source
    assert "def open_root(archive: Path, schema_archive: Path) -> Root:" in pyi.read_text(encoding="UTF-8")


def test_generate_validated_data_models_rejects_unknown_root_key(tmp_path: Path) -> None:
    """Reject a misspelled projection instead of silently generating an empty branch."""
    with pytest.raises(ValueError, match=r"unknown static root key.*missing"):
        generate_validated_data_models(
            ARTIFACTS / "schemas.json",
            "schema_generation_fixture",
            tmp_path / "models.rs",
            tmp_path / "models.pyi",
            "SchemaGenerationFixture",
            "SchemaGenerationFixture",
            root_keys=["missing"],
        )


def test_generate_validated_data_models_normalizes_identifiers(tmp_path: Path) -> None:
    """Escape language keywords while preserving case-distinct schema names."""
    source = tmp_path / "schemas.json"
    source.write_text(
        json.dumps(
            {
                "fixture": {
                    "type": "dict",
                    "keys": {
                        "match": {"type": "str"},
                        "override": {"type": "str"},
                        "Vxlan1": {"type": "dict"},
                        "vxlan1": {"type": "dict"},
                        "foo-bar": {"type": "dict"},
                        "foo_bar": {"type": "dict"},
                        "View": {"type": "dict"},
                    },
                }
            }
        ),
        encoding="UTF-8",
    )
    rust = tmp_path / "models.rs"
    pyi = tmp_path / "models.pyi"

    generate_validated_data_models(source, "fixture", rust, pyi, "Fixture", "Fixture")

    rust_source = _combined_rust_source(rust)
    pyi_source = pyi.read_text(encoding="UTF-8")
    assert '#[data_view(rename = "match")]' in rust_source
    assert "pub field_match: ::validated_data::Field<&'a str>" in rust_source
    assert '#[data_view(rename = "override")]' in rust_source
    assert "pub field_override: ::validated_data::Field<&'a str>" in rust_source
    assert "pub Vxlan1: ::validated_data::Field<" in rust_source
    assert "pub vxlan1: ::validated_data::Field<" in rust_source
    assert rust_source.count("pub struct Vxlan1Slot") == 2
    assert rust_source.count("pub struct FooBarSlot") == 2
    assert "pub struct View<'a, Mode>" in rust_source
    assert "def field_match(" in pyi_source
    assert "def field_override(" in pyi_source
    assert "def Vxlan1(" in pyi_source
    assert "def vxlan1(" in pyi_source
    ast.parse(pyi_source)


def test_validation_modes_preserve_reuse_and_python_presence_contracts(tmp_path: Path) -> None:
    """Reuse one Rust model across modes and expose relaxed payloads opaquely in Python."""
    source = tmp_path / "schemas.json"
    source.write_text(
        json.dumps(
            {
                "shared": {
                    "type": "dict",
                    "keys": {
                        "name": {"type": "str", "required": True},
                        "enabled": {"type": "bool", "default": True},
                    },
                },
                "fixture": {
                    "type": "dict",
                    "keys": {
                        "strict": {"$ref": "shared#", "type": "dict"},
                        "patch": {"$ref": "shared#", "type": "dict", "relaxed_validation": True},
                        "numbers": {"type": "list", "items": {"type": "int", "required": True}},
                        "entries": {"type": "list", "primary_key": "name", "items": {"type": "dict", "keys": {"name": {"type": "str"}}}},
                        "foo-bar": {"type": "str"},
                        "foo_bar": {"type": "str"},
                    },
                },
            }
        ),
        encoding="UTF-8",
    )
    rust, pyi = (tmp_path / name for name in ("models.rs", "models.pyi"))
    generate_validated_data_models(source, "fixture", rust, pyi, "Fixture", "Fixture", reused_schemas={"shared": ("Shared", "Shared")})
    rust_source = _combined_rust_source(rust)
    pyi_source = pyi.read_text(encoding="UTF-8")
    assert rust_source.count("pub struct Shared<'a, Mode>") == 1
    assert "pub strict: ::validated_data::Field<super::shared::Shared<'a, Mode>>" in rust_source
    assert "pub patch: ::validated_data::Field<super::shared::Shared<'a, ::validated_data::RelaxedValidated>>" in rust_source
    assert "(::validated_data::RequiredValue<i64, Mode>);" in rust_source
    assert "def strict(self) -> Shared | UndefinedType | None:" in pyi_source
    assert "def patch(self) -> OpaqueData | UndefinedType | None:" in pyi_source
    assert "def name(self) -> str:" in pyi_source
    assert "def enabled(self) -> bool | UndefinedType | None:" in pyi_source
    assert "class FixtureEntriesListIndexedItem(FixtureEntriesItems):" in pyi_source
    assert "class FixtureNumbersList(Sequence[int]):" in pyi_source
    root_class = next(node for node in ast.parse(pyi_source).body if isinstance(node, ast.ClassDef) and node.name == "Fixture")
    properties = [node.name for node in root_class.body if isinstance(node, ast.FunctionDef)]
    assert len(properties) == len(set(properties))
    assert sum(name.startswith("foo_bar_slot_") for name in properties) == 2
    ast.parse(pyi_source)


def test_duplicate_key_lists_keep_sequence_api_and_guaranteed_item_keys(tmp_path: Path) -> None:
    """Keep reusable dictionary fields optional while strengthening hybrid-list item access."""
    source = tmp_path / "schemas.json"
    source.write_text(
        json.dumps(
            {
                "shared": {"type": "dict", "keys": {"name": {"type": "str"}}},
                "fixture": {
                    "type": "dict",
                    "keys": {
                        "standalone": {"type": "dict", "$ref": "shared#"},
                        "duplicates": {
                            "type": "list",
                            "primary_key": "name",
                            "allow_duplicate_primary_key": True,
                            "items": {"type": "dict", "$ref": "shared#"},
                        },
                    },
                },
            }
        ),
        encoding="UTF-8",
    )
    rust, pyi = (tmp_path / name for name in ("models.rs", "models.pyi"))
    generate_validated_data_models(source, "fixture", rust, pyi, "Fixture", "Fixture", reused_schemas={"shared": ("Shared", "Shared")})
    rust_source = _combined_rust_source(rust)
    assert "#[::validated_data::data_view(list, primary_key(name))]" in rust_source
    assert rust_source.count("pub struct Shared<'a, Mode>") == 1
    assert "pub name: ::validated_data::Field<&'a str>" in rust_source
    pyi_source = pyi.read_text(encoding="UTF-8")
    assert "class FixtureDuplicatesList(Sequence[FixtureDuplicatesListKeyedItem]):" in pyi_source
    assert "class FixtureDuplicatesListKeyedItem(Shared):" in pyi_source
    assert "def name(self) -> str | UndefinedType | None:" in pyi_source
    classes = {node.name: node for node in ast.parse(pyi_source).body if isinstance(node, ast.ClassDef)}
    wrapper = classes["FixtureDuplicatesListKeyedItem"]
    name_property = next(node for node in wrapper.body if isinstance(node, ast.FunctionDef) and node.name == "name")
    assert isinstance(name_property.returns, ast.Name)
    assert name_property.returns.id == "str"
    sequence_methods = {node.name for node in classes["FixtureDuplicatesList"].body if isinstance(node, ast.FunctionDef)}
    assert sequence_methods == {"__iter__", "__getitem__"}
    native_source = (rust.with_suffix("") / "native.rs").read_text(encoding="UTF-8")
    assert '"FixtureDuplicatesList" => list(Target::Model("FixtureDuplicatesListKeyedItem"));' in native_source
    assert '"FixtureDuplicatesListKeyedItem" => alias("Shared");' in native_source
    assert '"FixtureDuplicatesList" => indexed(' not in native_source


def test_generate_validated_data_models_uses_language_specific_root_names(tmp_path: Path) -> None:
    """Honor the naming convention selected independently for each generated language."""
    rust = tmp_path / "models.rs"
    pyi = tmp_path / "models.pyi"

    generate_validated_data_models(
        ARTIFACTS / "schemas.json",
        "schema_generation_fixture",
        rust,
        pyi,
        "AvdDesign",
        "AVDDesign",
        root_keys=["accounting"],
    )

    rust_source = _combined_rust_source(rust)
    pyi_source = pyi.read_text(encoding="UTF-8")
    assert "pub mod avd_design;" in rust_source
    assert "pub struct AvdDesign<'a, Mode>" in rust_source
    assert "class AVDDesign:" in pyi_source
    assert "class AvdDesign:" not in pyi_source


def test_generate_validated_data_models_skips_removed_dangling_primary_key(tmp_path: Path) -> None:
    """Keep released-schema tombstones out of the nominal generated API."""
    source = tmp_path / "schemas.json"
    source.write_text(
        json.dumps(
            {
                "fixture": {
                    "type": "dict",
                    "keys": {
                        "historic_values": {
                            "type": "list",
                            "primary_key": "name",
                            "deprecation": {"warning": False, "removed": True},
                        }
                    },
                }
            }
        ),
        encoding="UTF-8",
    )
    rust = tmp_path / "models.rs"
    pyi = tmp_path / "models.pyi"

    generate_validated_data_models(source, "fixture", rust, pyi, "Fixture", "Fixture")

    rust_source = _combined_rust_source(rust)
    native_source = (rust.with_suffix("") / "native.rs").read_text(encoding="UTF-8")
    pyi_source = pyi.read_text(encoding="UTF-8")
    assert "primary_key_fields:" not in rust_source
    assert "historic_values" not in rust_source
    assert "historic_values" not in native_source
    assert "historic_values" not in pyi_source


def test_generate_validated_data_models_keeps_dynamic_model_descriptors(tmp_path: Path) -> None:
    """Keep collection typing below data-dependent keys without exposing a named accessor."""
    source = tmp_path / "schemas.json"
    source.write_text(
        json.dumps(
            {
                "fixture": {
                    "type": "dict",
                    "dynamic_keys": {
                        "selectors.names": {
                            "type": "dict",
                            "keys": {"enabled": {"type": "bool"}},
                        }
                    },
                }
            }
        ),
        encoding="UTF-8",
    )
    rust = tmp_path / "models.rs"
    pyi = tmp_path / "models.pyi"

    generate_validated_data_models(source, "fixture", rust, pyi, "Fixture", "Fixture")

    rust_source = _combined_rust_source(rust)
    assert '#[data_view(dynamic = "selectors.names")]' in rust_source
    assert "pub selectors_names: ::validated_data::Field<DynamicSlot0<'a, Mode>>" in rust_source
    assert "pub struct DynamicSlot0" in rust_source
    assert '"selectors_names"' not in (rust.with_suffix("") / "native.rs").read_text(encoding="UTF-8")
