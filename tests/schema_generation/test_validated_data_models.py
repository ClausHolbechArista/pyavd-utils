# Copyright (c) 2026 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.

import ast
from pathlib import Path

from pyavd_utils_gen.validated_data_generation import generate_validated_data_models

ARTIFACTS = Path(__file__).parent / "artifacts"


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
    )

    rust_source = rust.read_text(encoding="UTF-8")
    pyi_source = pyi.read_text(encoding="UTF-8")
    assert "pub const REGISTRY: ModelRegistry" in rust_source
    assert "pub struct SchemaGenerationFixtureView<'a>(DictView<'a>);" in rust_source
    assert "pub fn interface_profiles(" in rust_source
    assert "class SchemaGenerationFixture:" in pyi_source
    ast.parse(pyi_source)
