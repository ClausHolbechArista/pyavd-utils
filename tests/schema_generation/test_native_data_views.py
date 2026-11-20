# Copyright (c) 2026 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.

"""Attribute-derived bindings and stubs expose the same generated model identities."""

import ast
import re
from pathlib import Path

from pyavd_utils_gen.data_view_generation import generate_data_views


def test_native_catalog_and_stub_generation_is_deterministic(tmp_path: Path) -> None:
    """Emit deterministic split stubs with shared bases and explicit root exports."""
    source = Path(__file__).parent / "artifacts/schemas.json"
    pyi = tmp_path / "models_stubs"
    rust = tmp_path / "models.rs"
    arguments = (source, "schema_generation_fixture", rust, pyi, "SchemaGenerationFixture", "SchemaGenerationFixture")
    generate_data_views(*arguments)
    stub_before = {path.relative_to(pyi): path.read_bytes() for path in sorted(pyi.glob("*.pyi"))}
    root_before = rust.read_text()
    modules_before = {path.relative_to(rust.with_suffix("")): path.read_bytes() for path in rust.with_suffix("").rglob("*.rs")}
    (pyi / "obsolete.pyi").write_text("class Obsolete: ...\n")
    (pyi / "keep.txt").write_text("not generator-owned\n")
    generate_data_views(*arguments)
    modules_after = {path.relative_to(rust.with_suffix("")): path.read_bytes() for path in rust.with_suffix("").rglob("*.rs")}
    declarations = "\n".join(path.read_text() for path in rust.with_suffix("").rglob("*.rs"))
    stub_after = {path.relative_to(pyi): path.read_bytes() for path in sorted(pyi.glob("*.pyi"))}
    assert not (tmp_path / "models.py").exists()
    assert not (pyi / "obsolete.pyi").exists()
    assert (pyi / "keep.txt").read_text() == "not generator-owned\n"
    assert stub_after == stub_before
    assert set(stub_after) == {Path("__init__.pyi"), Path("_base.pyi"), Path("schema_generation_fixture.pyi")}
    for content in stub_after.values():
        ast.parse(content.decode())
    initializer = (pyi / "__init__.pyi").read_text()
    shared = (pyi / "_base.pyi").read_text()
    root_module = (pyi / "schema_generation_fixture.pyi").read_text()
    assert "from .schema_generation_fixture import SchemaGenerationFixture as SchemaGenerationFixture" in initializer
    assert "def open_schema_generation_fixture(archive: Path, schema_archive: Path) -> SchemaGenerationFixture:" in initializer
    assert "class _Model:" in shared
    assert "class _List(" in shared
    root_imports = [node for node in ast.parse(root_module).body if isinstance(node, ast.ImportFrom)]
    assert any(node.module == "_base" and "_Model" in {alias.name for alias in node.names} for node in root_imports)
    assert "class _Model:" not in root_module
    assert rust.read_text() == root_before
    assert modules_before == modules_after
    assert re.search(r'python\(name = "SchemaGenerationFixture", schema = Dict\(\d+\)\)', declarations)
    assert re.search(
        r'python\(name = "SchemaGenerationFixture.InterfaceProfiles", schema = List\(\d+\), item_name = "SchemaGenerationFixture.InterfaceProfilesItem"\)',
        declarations,
    )
    assert not (tmp_path / "models/native.rs").exists()
    assert "pub mod native;" not in root_before
