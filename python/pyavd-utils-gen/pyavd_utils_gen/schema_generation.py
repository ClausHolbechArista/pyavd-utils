# Copyright (c) 2026 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.
"""Schema-driven artifact generation."""

from __future__ import annotations

import json
from typing import TYPE_CHECKING, Any

from ._bindings import _schema_generation  # pyright: ignore[reportMissingModuleSource]

generate_schema_documentation = _schema_generation.generate_schema_documentation
generate_schema_documentation_from_paths = _schema_generation.generate_schema_documentation_from_paths
if TYPE_CHECKING:
    from pathlib import Path


def build_nominal_model_registry(source: Path, schema_name: str) -> dict[str, Any]:
    """Build occurrence-specific model and field identities for typed artifact generation."""
    return json.loads(_schema_generation.build_nominal_model_registry(source, schema_name))


generate_python_schema_models = _schema_generation.generate_python_schema_models
generate_python_schema_models_from_paths = _schema_generation.generate_python_schema_models_from_paths

__all__ = [
    "build_nominal_model_registry",
    "generate_python_schema_models",
    "generate_python_schema_models_from_paths",
    "generate_schema_documentation",
    "generate_schema_documentation_from_paths",
]
