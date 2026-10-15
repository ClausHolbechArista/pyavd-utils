# Copyright (c) 2026 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.

"""Attribute-derived bindings and stubs expose the same generated model identities."""

from pathlib import Path

from pyavd_utils_gen.validated_data_generation import generate_validated_data_models


def test_native_catalog_and_stub_generation_is_deterministic(tmp_path: Path) -> None:
    """Emit native annotations and declarations without a parallel binding catalog."""
    source = Path(__file__).parent / "artifacts/schemas.json"
    pyi = tmp_path / "models.pyi"
    rust = tmp_path / "models.rs"
    arguments = (source, "schema_generation_fixture", rust, pyi, "SchemaGenerationFixture", "SchemaGenerationFixture")
    generate_validated_data_models(*arguments)
    stub_before = pyi.read_bytes()
    root_before = rust.read_text()
    modules_before = {path.relative_to(rust.with_suffix("")): path.read_bytes() for path in rust.with_suffix("").rglob("*.rs")}
    generate_validated_data_models(*arguments)
    modules_after = {path.relative_to(rust.with_suffix("")): path.read_bytes() for path in rust.with_suffix("").rglob("*.rs")}
    declarations = "\n".join(path.read_text() for path in rust.with_suffix("").rglob("*.rs"))
    assert not (tmp_path / "models.py").exists()
    assert pyi.read_bytes() == stub_before
    assert rust.read_text() == root_before
    assert modules_before == modules_after
    assert 'python(name = "SchemaGenerationFixture")' in declarations
    assert 'python(name = "SchemaGenerationFixture.InterfaceProfiles", item_name = "SchemaGenerationFixture.InterfaceProfilesItem")' in declarations
    assert not (tmp_path / "models/native.rs").exists()
    assert "pub mod native;" not in root_before
