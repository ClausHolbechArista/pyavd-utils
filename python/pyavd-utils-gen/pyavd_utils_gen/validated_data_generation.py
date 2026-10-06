# Copyright (c) 2026 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.
"""Generate nominal Rust views and matching Python type declarations."""

from __future__ import annotations

import hashlib
import json
import keyword
import re
from typing import TYPE_CHECKING, Any

from .schema_generation import build_nominal_model_registry

if TYPE_CHECKING:
    from pathlib import Path

_RUST_KEYWORDS = frozenset(
    [
        "abstract",
        "as",
        "async",
        "await",
        "become",
        "box",
        "break",
        "const",
        "continue",
        "crate",
        "do",
        "dyn",
        "else",
        "enum",
        "extern",
        "false",
        "final",
        "fn",
        "for",
        "gen",
        "if",
        "impl",
        "in",
        "let",
        "loop",
        "macro",
        "match",
        "mod",
        "move",
        "mut",
        "override",
        "priv",
        "pub",
        "ref",
        "return",
        "self",
        "Self",
        "static",
        "struct",
        "super",
        "trait",
        "true",
        "try",
        "type",
        "typeof",
        "union",
        "unsafe",
        "unsized",
        "use",
        "virtual",
        "where",
        "while",
        "yield",
    ]
)
_PYTHON_SCALAR_TYPES = {"Bool": "bool", "Int": "int", "Str": "str"}


def generate_validated_data_models(
    source: Path,
    schema_name: str,
    rust_destination: Path,
    pyi_destination: Path,
    rust_root_name: str,
    python_root_name: str,
    root_keys: list[str] | None = None,
) -> None:
    """
    Generate checked-in Rust views and Python declarations from one schema root.

    Args:
        source: Combined source schema containing ``schema_name``.
        schema_name: Name of the schema root to generate from.
        rust_destination: Destination for the Rust model registry and views.
        pyi_destination: Destination for the corresponding Python declarations.
        rust_root_name: Public Rust name of the root model, using Rust naming conventions.
        python_root_name: Public Python name of the root model, using Python naming conventions.
        root_keys: Optional static root keys to expose. Descendants of selected keys
            remain complete, while unselected root branches are omitted from the
            generated API. The runtime validated-data archive remains complete.
    """
    registry = build_nominal_model_registry(source, schema_name)
    if root_keys is not None:
        registry = _project_registry(registry, root_keys)
    models = list(registry["models"])
    fields = list(registry["fields"])
    names = _model_names(models, python_root_name)
    rust_destination.write_text(_render_rust(registry, models, fields, rust_root_name), encoding="UTF-8")
    pyi_destination.write_text(_render_pyi(models, fields, names), encoding="UTF-8")


def _project_registry(registry: dict[str, Any], root_keys: list[str]) -> dict[str, Any]:
    """Select static root branches while preserving identities from the full registry."""
    root = int(registry["root"])
    fields_by_parent: dict[int, list[dict[str, Any]]] = {}
    for field in registry["fields"]:
        fields_by_parent.setdefault(int(field["parent"]), []).append(field)

    available_root_keys = {key for field in fields_by_parent.get(root, []) if (relation := _relation(field))[0] == "Key" and (key := relation[1]) is not None}
    requested_root_keys = set(root_keys)
    unknown_root_keys = requested_root_keys - available_root_keys
    if unknown_root_keys:
        unknown = ", ".join(sorted(unknown_root_keys))
        msg = f"unknown static root key(s) for validated-data generation: {unknown}"
        raise ValueError(msg)

    selected_models = {root}
    selected_fields: set[int] = set()
    pending_models = [root]
    while pending_models:
        parent = pending_models.pop()
        for field in fields_by_parent.get(parent, []):
            relation_kind, relation_value = _relation(field)
            if parent == root and (relation_kind != "Key" or relation_value not in requested_root_keys):
                continue
            selected_fields.add(int(field["id"]))
            target_kind, target = _target(field)
            if target_kind == "Model" and int(target) not in selected_models:
                selected_models.add(int(target))
                pending_models.append(int(target))

    digest = hashlib.sha256()
    digest.update(bytes(registry["registry_hash"]))
    for key in sorted(requested_root_keys):
        encoded_key = key.encode()
        digest.update(len(encoded_key).to_bytes(8, "little"))
        digest.update(encoded_key)

    return {
        **registry,
        "models": [model for model in registry["models"] if int(model["id"]) in selected_models],
        "fields": [field for field in registry["fields"] if int(field["id"]) in selected_fields],
        "registry_hash": list(digest.digest()),
    }


def _model_names(models: list[dict[str, Any]], root_name: str) -> dict[int, str]:
    names: dict[int, str] = {}
    used: set[str] = set()
    for model in models:
        model_id = int(model["id"])
        if model_id == 0:
            candidate = root_name
        else:
            parts = [part for part in model["path"] if part != "keys"]
            candidate = root_name + "".join(_pascal(part) for part in parts[1:])
        if model["kind"] == "List" and not candidate.endswith("List"):
            candidate += "List"
        base = candidate
        suffix = 2
        while candidate in used:
            candidate = f"{base}{suffix}"
            suffix += 1
        used.add(candidate)
        names[model_id] = candidate + "View"
    return names


def _pascal(value: str) -> str:
    return "".join(part.capitalize() for part in re.split(r"[^A-Za-z0-9]+", value) if part)


def _identifier(value: str) -> str:
    identifier = re.sub(r"\W", "_", value)
    if not identifier or identifier[0].isdigit() or keyword.iskeyword(identifier) or identifier in _RUST_KEYWORDS:
        identifier = f"field_{identifier}"
    return identifier


def _rust_module_identifier(value: str) -> str:
    """Convert a schema relationship name into a snake-case Rust module identifier."""
    identifier = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", value)
    identifier = re.sub(r"\W", "_", identifier).lower()
    if not identifier or identifier[0].isdigit() or identifier in _RUST_KEYWORDS:
        identifier = f"field_{identifier}"
    return identifier


def _rust_type_identifier(value: str) -> str:
    """Convert a schema relationship name into a PascalCase Rust type identifier."""
    return _pascal(_rust_module_identifier(value))


def _relation(field: dict[str, Any]) -> tuple[str, str | None]:
    relation = field["relation"]
    if relation == "Item":
        return ("Item", None)
    if "Key" in relation:
        return ("Key", str(relation["Key"]))
    return ("DynamicKey", str(relation["DynamicKey"]))


def _target(field: dict[str, Any]) -> tuple[str, int | str]:
    target = field["target"]
    if "Model" in target:
        return ("Model", int(target["Model"]))
    return ("Scalar", str(target["Scalar"]))


def _python_scalar_type(target: int | str) -> str:
    """Return the native Python type exposed for one scalar target."""
    try:
        return _PYTHON_SCALAR_TYPES[str(target)]
    except KeyError as error:
        msg = f"unsupported scalar target for Python declarations: {target}"
        raise ValueError(msg) from error


def _render_rust(
    registry: dict[str, Any],
    models: list[dict[str, Any]],
    fields: list[dict[str, Any]],
    root_name: str,
) -> str:
    root_module = _rust_module_identifier(root_name)
    if root_module == "__pyavd_generated_registry":
        msg = f"root name {root_name!r} conflicts with the generated registry namespace"
        raise ValueError(msg)
    output = [
        "// Copyright (c) 2026 Arista Networks, Inc.\n",
        "// Generated from the AVD schema. Do not edit by hand.\n\n",
        f"const REGISTRY_HASH: [u8; 32] = [{', '.join(str(value) for value in registry['registry_hash'])}];\n",
        "mod __pyavd_generated_registry {\n",
        "    use super::REGISTRY_HASH;\n",
        "    use ::validation::archive::{FieldDescriptor, FieldRelation, ModelRegistry};\n\n",
    ]
    output.append("    static FIELDS: &[FieldDescriptor] = &[\n")
    for field in fields:
        kind, value = _relation(field)
        relation = f"FieldRelation::{kind}" if value is None else f"FieldRelation::{kind}({json.dumps(value)})"
        target_kind, target = _target(field)
        target_model = f"Some({target})" if target_kind == "Model" else "None"
        output.append(
            f"        FieldDescriptor {{ id: {field['id']}, parent_model: {field['parent']}, relation: {relation}, target_model: {target_model} }},\n"
        )
    output.append("    ];\n")
    output.append(f"    pub const REGISTRY: ModelRegistry = ModelRegistry {{ root_model: {registry['root']}, hash: REGISTRY_HASH, fields: FIELDS }};\n")
    output.append("}\n")
    output.append("pub use __pyavd_generated_registry::REGISTRY;\n\n")
    models_by_id = {int(model["id"]): model for model in models}
    by_parent: dict[int, list[dict[str, Any]]] = {}
    for field in fields:
        by_parent.setdefault(int(field["parent"]), []).append(field)
    visited: set[int] = set()
    output.append(f"pub mod {root_module} {{\n")
    output.extend(
        _render_rust_model(
            int(registry["root"]),
            _rust_type_identifier(root_name),
            None,
            models_by_id,
            by_parent,
            visited,
            "    ",
        )
    )
    output.append("}\n")
    if visited != models_by_id.keys():
        missing = ", ".join(str(model_id) for model_id in sorted(models_by_id.keys() - visited))
        msg = f"nominal model registry contains unreachable model(s): {missing}"
        raise ValueError(msg)
    return "".join(output)


def _render_rust_model(
    model_id: int,
    type_name: str,
    namespace_name: str | None,
    models: dict[int, dict[str, Any]],
    by_parent: dict[int, list[dict[str, Any]]],
    visited: set[int],
    indent: str,
) -> list[str]:
    """Render one nominal model and recursively nest its child occurrences."""
    if model_id in visited:
        msg = f"nominal model {model_id} is reachable from more than one relationship"
        raise ValueError(msg)
    visited.add(model_id)
    model = models.get(model_id)
    if model is None:
        msg = f"nominal model registry references missing model {model_id}"
        raise ValueError(msg)

    kind = str(model["kind"])
    archive_view = "DictView" if kind == "Dict" else "ListView"
    fields = by_parent.get(model_id, [])
    child_names = _rust_child_names(fields, frozenset({type_name}))
    output = [
        f"{indent}#[derive(Clone, Copy, Debug)]\n",
        f"{indent}pub struct {type_name}<'a>(::validation::archive::{archive_view}<'a>);\n",
        f"{indent}impl<'a> {type_name}<'a> {{\n",
        (
            f"{indent}    pub fn from_value(value: ::validation::archive::ValueView<'a>) -> ::core::option::Option<Self> "
            f"{{ value.as_{'dict' if kind == 'Dict' else 'list'}().map(Self) }}\n"
        ),
    ]
    if kind == "Dict":
        for field in fields:
            relation_kind, key = _relation(field)
            if relation_kind != "Key" or key is None:
                continue
            return_type, conversion = _rust_target(field, models, child_names, namespace_name)
            output.append(
                f"{indent}    pub fn {_identifier(key)}(self) -> ::core::option::Option<{return_type}> {{ self.0.field({field['id']}){conversion} }}\n"
            )
    else:
        item = next((field for field in fields if _relation(field)[0] == "Item"), None)
        output.append(f"{indent}    pub fn len(self) -> usize {{ self.0.len() }}\n")
        output.append(f"{indent}    pub fn is_empty(self) -> bool {{ self.0.is_empty() }}\n")
        if item is not None:
            return_type, conversion = _rust_target(item, models, child_names, namespace_name)
            output.append(f"{indent}    pub fn get(self, index: usize) -> ::core::option::Option<{return_type}> {{ self.0.get(index){conversion} }}\n")
    output.append(f"{indent}}}\n")

    model_fields = [field for field in fields if _target(field)[0] == "Model"]
    if not model_fields:
        return output

    child_indent = indent
    if namespace_name is not None:
        output.append(f"\n{indent}pub mod {namespace_name} {{\n")
        child_indent += "    "
    for field in model_fields:
        target_kind, target = _target(field)
        if target_kind != "Model":  # pragma: no cover - filtered above
            continue
        module_name, child_type_name = child_names[int(field["id"])]
        output.append("\n")
        output.extend(
            _render_rust_model(
                int(target),
                child_type_name,
                module_name,
                models,
                by_parent,
                visited,
                child_indent,
            )
        )
    if namespace_name is not None:
        output.append(f"{indent}}}\n")
    return output


def _rust_child_names(fields: list[dict[str, Any]], reserved_types: frozenset[str]) -> dict[int, tuple[str, str]]:
    """Build deterministic module and type names for child collection relationships."""
    bases: dict[int, tuple[str, str]] = {}
    module_counts: dict[str, int] = {}
    type_counts: dict[str, int] = {}
    for field in fields:
        if _target(field)[0] != "Model":
            continue
        field_id = int(field["id"])
        relation_kind, value = _relation(field)
        if relation_kind == "Key" and value is not None:
            module_base = _rust_module_identifier(value)
            type_base = _rust_type_identifier(value)
        elif relation_kind == "Item":
            module_base = "item"
            type_base = "Item"
        else:
            module_base = f"dynamic_slot_{field_id}"
            type_base = f"DynamicSlot{field_id}"
        bases[field_id] = (module_base, type_base)
        module_counts[module_base] = module_counts.get(module_base, 0) + 1
        type_counts[type_base] = type_counts.get(type_base, 0) + 1
    return {
        field_id: (
            module_base if module_counts[module_base] == 1 else f"{module_base}_slot_{field_id}",
            type_base if type_counts[type_base] == 1 and type_base not in reserved_types else f"{type_base}Slot{field_id}",
        )
        for field_id, (module_base, type_base) in bases.items()
    }


def _rust_target(
    field: dict[str, Any],
    models: dict[int, dict[str, Any]],
    child_names: dict[int, tuple[str, str]],
    namespace_name: str | None,
) -> tuple[str, str]:
    """Render one accessor target type and its zero-copy conversion."""
    target_kind, target = _target(field)
    if target_kind != "Model":
        return ("::validation::archive::ValueView<'a>", "")
    target_id = int(target)
    target_model = models.get(target_id)
    if target_model is None:
        msg = f"nominal field {field['id']} references missing model {target_id}"
        raise ValueError(msg)
    _, type_name = child_names[int(field["id"])]
    qualified = type_name if namespace_name is None else f"{namespace_name}::{type_name}"
    return (f"{qualified}<'a>", f".and_then({qualified}::from_value)")


def _render_pyi(
    models: list[dict[str, Any]],
    fields: list[dict[str, Any]],
    names: dict[int, str],
) -> str:
    header = [
        "# Copyright (c) 2026 Arista Networks, Inc.\n",
        "# Generated from the AVD schema. Do not edit by hand.\n",
        "# ruff: noqa: N802\n",
        "from collections.abc import Sequence\n\n",
        "class _Value: ...\n\n",
    ]
    definitions: list[tuple[str, bool]] = []
    by_parent: dict[int, list[dict[str, Any]]] = {}
    for field in fields:
        by_parent.setdefault(int(field["parent"]), []).append(field)
    for model in models:
        model_id = int(model["id"])
        name = names[model_id].removesuffix("View")
        if model["kind"] == "List":
            item = next((field for field in by_parent.get(model_id, []) if _relation(field)[0] == "Item"), None)
            item_type = "_Value"
            if item is not None:
                target_kind, target = _target(item)
                item_type = names[int(target)].removesuffix("View") if target_kind == "Model" else _python_scalar_type(target)
            definitions.append((f"class {name}(\n    Sequence[{item_type}],\n): ...", True))
            continue
        static_fields = [field for field in by_parent.get(model_id, []) if _relation(field)[0] == "Key"]
        if not static_fields:
            definitions.append((f"class {name}: ...", True))
            continue
        body = [f"class {name}:\n"]
        for field in static_fields:
            key = _relation(field)[1]
            target_kind, target = _target(field)
            annotation = names[int(target)].removesuffix("View") if target_kind == "Model" else _python_scalar_type(target)
            body.append(f"    @property\n    def {_identifier(str(key))}(\n        self,\n    ) -> {annotation} | None: ...\n")
        definitions.append(("".join(body).rstrip(), False))
    output = ["".join(header)]
    for index, (definition, is_empty) in enumerate(definitions):
        if index:
            output.append("\n" if is_empty and definitions[index - 1][1] else "\n\n")
        output.append(definition)
    return "".join(output) + "\n"


__all__ = ["generate_validated_data_models"]
