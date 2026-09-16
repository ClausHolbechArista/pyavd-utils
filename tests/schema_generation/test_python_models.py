# Copyright (c) 2026 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.
import subprocess
import sys
from pathlib import Path

import pytest

from pyavd_utils.schema_generation import generate_python_schema_models
from pyavd_utils.schema_store import compile_schema_archive

ARTIFACTS = Path(__file__).parent / "artifacts"


def test_regenerate_python_model_fixture() -> None:
    """Regenerate committed artifacts in place so a mismatch remains visible in git diff."""
    source = ARTIFACTS / "schemas.json"
    archive = ARTIFACTS / "schemas.rkyv"
    generated = ARTIFACTS / "schema_generation_fixture.py"
    expected = generated.read_bytes()

    compile_schema_archive(source, archive)
    generate_python_schema_models(source, "schema_generation_fixture", generated)
    subprocess.run([sys.executable, "-m", "ruff", "check", "--fix", generated], check=True)  # noqa: S603
    subprocess.run([sys.executable, "-m", "ruff", "format", generated], check=True)  # noqa: S603

    assert generated.read_bytes() == expected


def test_generate_root_key_projection(tmp_path: Path) -> None:
    generated = tmp_path / "projection.py"
    generate_python_schema_models(
        ARTIFACTS / "schemas.json",
        "schema_generation_fixture",
        generated,
        "ProjectedSchema",
        ["interface_profiles"],
    )

    output = generated.read_text(encoding="UTF-8")
    assert "class ProjectedSchema(AvdModel):" in output
    assert "class InterfaceProfilesItem(AvdModel):" in output
    assert "class Accounting(AvdModel):" not in output


def test_projection_requires_explicit_generated_class_name(tmp_path: Path) -> None:
    with pytest.raises(ValueError, match="generated_class_name is required"):
        generate_python_schema_models(
            ARTIFACTS / "schemas.json",
            "schema_generation_fixture",
            tmp_path / "projection.py",
            root_keys=["interface_profiles"],
        )


def test_projection_rejects_unknown_root_key(tmp_path: Path) -> None:
    with pytest.raises(RuntimeError, match="Root key 'missing' was not found"):
        generate_python_schema_models(
            ARTIFACTS / "schemas.json",
            "schema_generation_fixture",
            tmp_path / "projection.py",
            "ProjectedSchema",
            ["missing"],
        )


def test_generation_rejects_scalar_root(tmp_path: Path) -> None:
    source = tmp_path / "schemas.json"
    source.write_text('{"scalar": {"type": "str"}}', encoding="UTF-8")

    with pytest.raises(RuntimeError, match="requires a dictionary root"):
        generate_python_schema_models(source, "scalar", tmp_path / "scalar.py")


def test_generation_rejects_unsupported_schema_feature(tmp_path: Path) -> None:
    source = tmp_path / "schemas.json"
    source.write_text(
        '{"model": {"type": "dict", "dynamic_keys": {"selectors.names": {"type": "str"}}}}',
        encoding="UTF-8",
    )

    with pytest.raises(RuntimeError, match="does not yet support dynamic keys"):
        generate_python_schema_models(source, "model", tmp_path / "model.py")
