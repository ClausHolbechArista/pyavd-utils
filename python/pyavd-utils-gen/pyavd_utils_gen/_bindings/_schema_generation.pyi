# Copyright (c) 2026 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.
# ruff: noqa: PYI021
from collections.abc import Mapping
from pathlib import Path

def generate_schema_documentation(source: Path, schema_name: str, destination: Path) -> None:
    """
    Generate Markdown schema documentation from a raw schema store.

    Output names may contain lowercase ASCII letters, digits, underscores, hyphens,
    and dots. Invalid names raise ValueError before directory creation, cleanup, or writes.
    """

def generate_schema_documentation_from_paths(sources: Mapping[str, Path], schema_name: str, destination: Path) -> None:
    """
    Generate Markdown schema documentation from individually named schema files.

    Output names may contain lowercase ASCII letters, digits, underscores, hyphens,
    and dots. Invalid names raise ValueError before directory creation, cleanup, or writes.
    """
def build_nominal_model_registry(source: Path, schema_name: str) -> str:
    """Return the occurrence-specific nominal model registry as JSON."""

def generate_python_schema_models(
    source: Path,
    schema_name: str,
    destination: Path,
    generated_class_name: str | None = None,
    root_keys: list[str] | None = None,
) -> None:
    """Generate nested Python schema models from a source schema-store file."""

def generate_python_schema_models_from_paths(
    sources: Mapping[str, Path],
    schema_name: str,
    destination: Path,
    generated_class_name: str | None = None,
    root_keys: list[str] | None = None,
) -> None:
    """Generate nested Python schema models from individually named source schema paths."""
