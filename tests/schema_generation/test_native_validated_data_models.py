# Copyright (c) 2026 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.

"""Native catalogs and stubs expose the same generated model identities."""

from pathlib import Path

from pyavd_utils_gen.validated_data_generation import generate_validated_data_models


def test_native_catalog_and_stub_generation_is_deterministic(tmp_path: Path) -> None:
    """Emit native properties, indexed identities, and declarations without executable Python."""
    source = Path(__file__).parent / "artifacts/schemas.json"
    pyi = tmp_path / "models.pyi"
    rust = tmp_path / "models.rs"
    arguments = (source, "schema_generation_fixture", rust, pyi, "SchemaGenerationFixture", "SchemaGenerationFixture")
    generate_validated_data_models(*arguments)
    stub_before = pyi.read_bytes()
    root_before = rust.read_text()
    generate_validated_data_models(*arguments)
    catalog = (tmp_path / "models/native.rs").read_text()
    assert not (tmp_path / "models.py").exists()
    assert pyi.read_bytes() == stub_before
    assert rust.read_text() == root_before
    assert '"SchemaGenerationFixture" => dict {' in catalog
    assert '"SchemaGenerationFixtureInterfaceProfilesList" => indexed(' in catalog
    assert '"SchemaGenerationFixtureInterfaceProfilesListIndexedItem" => alias(' in catalog
    assert "Target::Scalar" in catalog
    assert "::validated_data_py::python_data_views!" in catalog
