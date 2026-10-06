# Copyright (c) 2026 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.

import ast
import json
from pathlib import Path

import pytest

from pyavd_utils_gen.validated_data_generation import generate_validated_data_models

ARTIFACTS = Path(__file__).parent / "artifacts"


def test_generate_validated_data_models(tmp_path: Path) -> None:
    """Generate nominal Rust views and Python declarations from the shared fixture."""
    rust = tmp_path / "models.rs"
    python = tmp_path / "models.py"
    pyi = tmp_path / "models.pyi"

    generate_validated_data_models(
        ARTIFACTS / "schemas.json",
        "schema_generation_fixture",
        rust,
        python,
        pyi,
        "SchemaGenerationFixture",
        "SchemaGenerationFixture",
    )

    rust_source = rust.read_text(encoding="UTF-8")
    python_source = python.read_text(encoding="UTF-8")
    pyi_source = pyi.read_text(encoding="UTF-8")
    assert "pub const REGISTRY: ModelRegistry" in rust_source
    assert "pub mod schema_generation_fixture {" in rust_source
    assert "pub struct InterfaceProfiles<'a>(::validation::archive::ListView<'a>);" in rust_source
    assert "pub mod interface_profiles {" in rust_source
    assert "pub struct Item<'a>(::validation::archive::DictView<'a>);" in rust_source
    assert "pub fn interface_profiles(" in rust_source
    assert "PrimaryKeyDescriptor { model:" in rust_source
    assert "pub fn get_by_primary_key(" in rust_source
    assert "class SchemaGenerationFixture(_DictView):" in python_source
    assert "class SchemaGenerationFixtureInterfaceProfilesList(_ListView):" in python_source
    assert "def __getitem__(self, key: str)" in python_source
    assert "class SchemaGenerationFixture:" in pyi_source
    assert "class StrValue:" not in pyi_source
    assert "Sequence[str | None]" in pyi_source
    assert "-> str | UndefinedType | None" in pyi_source
    ast.parse(python_source)
    ast.parse(pyi_source)


def test_generate_validated_data_models_projects_static_root_keys(tmp_path: Path) -> None:
    """Keep selected branches complete and preserve their registry identities."""
    full_rust = tmp_path / "full.rs"
    full_python = tmp_path / "full.py"
    full_pyi = tmp_path / "full.pyi"
    projected_rust = tmp_path / "projected.rs"
    projected_python = tmp_path / "projected.py"
    projected_pyi = tmp_path / "projected.pyi"
    reordered_rust = tmp_path / "reordered.rs"
    reordered_python = tmp_path / "reordered.py"
    reordered_pyi = tmp_path / "reordered.pyi"

    generate_validated_data_models(
        ARTIFACTS / "schemas.json",
        "schema_generation_fixture",
        full_rust,
        full_python,
        full_pyi,
        "SchemaGenerationFixture",
        "SchemaGenerationFixture",
    )
    generate_validated_data_models(
        ARTIFACTS / "schemas.json",
        "schema_generation_fixture",
        projected_rust,
        projected_python,
        projected_pyi,
        "SchemaGenerationFixture",
        "SchemaGenerationFixture",
        root_keys=["accounting", "interface_profiles"],
    )
    generate_validated_data_models(
        ARTIFACTS / "schemas.json",
        "schema_generation_fixture",
        reordered_rust,
        reordered_python,
        reordered_pyi,
        "SchemaGenerationFixture",
        "SchemaGenerationFixture",
        root_keys=["interface_profiles", "accounting"],
    )

    full_source = full_rust.read_text(encoding="UTF-8")
    projected_source = projected_rust.read_text(encoding="UTF-8")
    projected_pyi_source = projected_pyi.read_text(encoding="UTF-8")
    assert 'relation: FieldRelation::Key("accounting")' in projected_source
    assert 'relation: FieldRelation::Key("methods")' in projected_source
    assert 'relation: FieldRelation::Key("authentication")' not in projected_source
    assert "def accounting(" in projected_pyi_source
    assert "def authentication(" not in projected_pyi_source
    assert projected_source != full_source
    assert projected_source == reordered_rust.read_text(encoding="UTF-8")
    assert projected_python.read_bytes() == reordered_python.read_bytes()
    assert projected_pyi_source == reordered_pyi.read_text(encoding="UTF-8")

    full_accounting = next(line for line in full_source.splitlines() if 'relation: FieldRelation::Key("accounting")' in line)
    projected_accounting = next(line for line in projected_source.splitlines() if 'relation: FieldRelation::Key("accounting")' in line)
    assert projected_accounting == full_accounting
    full_hash = next(line for line in full_source.splitlines() if line.startswith("const REGISTRY_HASH"))
    projected_hash = next(line for line in projected_source.splitlines() if line.startswith("const REGISTRY_HASH"))
    assert projected_hash != full_hash
    ast.parse(projected_pyi_source)


def test_generate_validated_data_models_rejects_unknown_root_key(tmp_path: Path) -> None:
    """Reject a misspelled projection instead of silently generating an empty branch."""
    with pytest.raises(ValueError, match=r"unknown static root key.*missing"):
        generate_validated_data_models(
            ARTIFACTS / "schemas.json",
            "schema_generation_fixture",
            tmp_path / "models.rs",
            tmp_path / "models.py",
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
    python = tmp_path / "models.py"
    pyi = tmp_path / "models.pyi"

    generate_validated_data_models(source, "fixture", rust, python, pyi, "Fixture", "Fixture")

    rust_source = rust.read_text(encoding="UTF-8")
    pyi_source = pyi.read_text(encoding="UTF-8")
    assert "pub fn field_match(" in rust_source
    assert "pub fn field_override(" in rust_source
    assert "pub fn Vxlan1(" in rust_source
    assert "pub fn vxlan1(" in rust_source
    assert rust_source.count("pub struct Vxlan1Slot") == 2
    assert rust_source.count("pub struct FooBarSlot") == 2
    assert "pub struct View<'a>" in rust_source
    assert "def field_match(" in pyi_source
    assert "def field_override(" in pyi_source
    assert "def Vxlan1(" in pyi_source
    assert "def vxlan1(" in pyi_source
    ast.parse(pyi_source)


def test_generate_validated_data_models_uses_language_specific_root_names(tmp_path: Path) -> None:
    """Honor the naming convention selected independently for each generated language."""
    rust = tmp_path / "models.rs"
    python = tmp_path / "models.py"
    pyi = tmp_path / "models.pyi"

    generate_validated_data_models(
        ARTIFACTS / "schemas.json",
        "schema_generation_fixture",
        rust,
        python,
        pyi,
        "AvdDesign",
        "AVDDesign",
        root_keys=["accounting"],
    )

    rust_source = rust.read_text(encoding="UTF-8")
    pyi_source = pyi.read_text(encoding="UTF-8")
    assert "pub mod avd_design {" in rust_source
    assert "pub struct AvdDesign<'a>" in rust_source
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
    python = tmp_path / "models.py"
    pyi = tmp_path / "models.pyi"

    generate_validated_data_models(source, "fixture", rust, python, pyi, "Fixture", "Fixture")

    rust_source = rust.read_text(encoding="UTF-8")
    python_source = python.read_text(encoding="UTF-8")
    pyi_source = pyi.read_text(encoding="UTF-8")
    assert "PrimaryKeyDescriptor { model:" not in rust_source
    assert "historic_values" not in rust_source
    assert "historic_values" not in python_source
    assert "historic_values" not in pyi_source
