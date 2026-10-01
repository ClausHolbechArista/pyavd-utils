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


def generate_validated_data_models(
    source: Path,
    schema_name: str,
    rust_destination: Path,
    pyi_destination: Path,
    root_name: str,
    root_keys: list[str] | None = None,
) -> None:
    """
    Generate checked-in Rust views and Python declarations from one schema root.

    Args:
        source: Combined source schema containing ``schema_name``.
        schema_name: Name of the schema root to generate from.
        rust_destination: Destination for the Rust model registry and views.
        pyi_destination: Destination for the corresponding Python declarations.
        root_name: Public name of the root model.
        root_keys: Optional static root keys to expose. Descendants of selected keys
            remain complete, while unselected root branches are omitted from the
            generated API. The runtime validated-data archive remains complete.
    """
    registry = build_nominal_model_registry(source, schema_name)
    if root_keys is not None:
        registry = _project_registry(registry, root_keys)
    models = list(registry["models"])
    fields = list(registry["fields"])
    names = _model_names(models, root_name)
    rust_destination.write_text(_render_rust(registry, models, fields, names), encoding="UTF-8")
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
    identifier = re.sub(r"(.)([A-Z][a-z]+)", r"\1_\2", identifier)
    identifier = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", identifier).lower()
    if not identifier or identifier[0].isdigit() or keyword.iskeyword(identifier) or identifier in _RUST_KEYWORDS:
        identifier = f"field_{identifier}"
    return identifier


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


def _render_rust(
    registry: dict[str, Any],
    models: list[dict[str, Any]],
    fields: list[dict[str, Any]],
    names: dict[int, str],
) -> str:
    output = [
        "// Copyright (c) 2026 Arista Networks, Inc.\n",
        "// Generated from the AVD schema. Do not edit by hand.\n\n",
        "use validation::archive::{DictView, FieldDescriptor, FieldRelation, ListView, ModelRegistry, ValueView};\n\n",
    ]
    digest = ", ".join(str(value) for value in registry["registry_hash"])
    output.append(f"pub const REGISTRY_HASH: [u8; 32] = [{digest}];\n")
    output.append("pub static FIELDS: &[FieldDescriptor] = &[\n")
    for field in fields:
        kind, value = _relation(field)
        relation = f"FieldRelation::{kind}" if value is None else f"FieldRelation::{kind}({json.dumps(value)})"
        target_kind, target = _target(field)
        target_model = f"Some({target})" if target_kind == "Model" else "None"
        output.append(f"    FieldDescriptor {{ id: {field['id']}, parent_model: {field['parent']}, relation: {relation}, target_model: {target_model} }},\n")
    output.append("];\n")
    output.append(f"pub const REGISTRY: ModelRegistry = ModelRegistry {{ root_model: {registry['root']}, hash: REGISTRY_HASH, fields: FIELDS }};\n\n")
    by_parent: dict[int, list[dict[str, Any]]] = {}
    for field in fields:
        by_parent.setdefault(int(field["parent"]), []).append(field)
    for model in models:
        model_id = int(model["id"])
        name = names[model_id]
        kind = str(model["kind"])
        view = "DictView" if kind == "Dict" else "ListView"
        output.append(f"#[derive(Clone, Copy, Debug)]\npub struct {name}<'a>({view}<'a>);\n")
        output.append(f"impl<'a> {name}<'a> {{\n")
        output.append(f"    pub fn from_value(value: ValueView<'a>) -> Option<Self> {{ value.as_{'dict' if kind == 'Dict' else 'list'}().map(Self) }}\n")
        if kind == "Dict":
            for field in by_parent.get(model_id, []):
                relation_kind, key = _relation(field)
                if relation_kind != "Key" or key is None:
                    continue
                target_kind, target = _target(field)
                return_type = "ValueView<'a>"
                conversion = ""
                if target_kind == "Model":
                    return_type = names[int(target)] + "<'a>"
                    conversion = f".and_then({names[int(target)]}::from_value)"
                output.append(f"    pub fn {_identifier(key)}(self) -> Option<{return_type}> {{ self.0.field({field['id']}){conversion} }}\n")
        else:
            item = next((field for field in by_parent.get(model_id, []) if _relation(field)[0] == "Item"), None)
            output.append("    pub fn len(self) -> usize { self.0.len() }\n")
            output.append("    pub fn is_empty(self) -> bool { self.0.is_empty() }\n")
            if item is not None:
                target_kind, target = _target(item)
                return_type = "ValueView<'a>"
                conversion = ""
                if target_kind == "Model":
                    return_type = names[int(target)] + "<'a>"
                    conversion = f".and_then({names[int(target)]}::from_value)"
                output.append(f"    pub fn get(self, index: usize) -> Option<{return_type}> {{ self.0.get(index){conversion} }}\n")
        output.append("}\n\n")
    return "".join(output)


def _render_pyi(
    models: list[dict[str, Any]],
    fields: list[dict[str, Any]],
    names: dict[int, str],
) -> str:
    output = [
        "# Copyright (c) 2026 Arista Networks, Inc.\n",
        "# Generated from the AVD schema. Do not edit by hand.\n",
        "from collections.abc import Sequence\n\n",
        "class BoolValue:\n    @property\n    def value(self) -> bool | None: ...\n\n",
        "class IntValue:\n    @property\n    def value(self) -> int | None: ...\n\n",
        "class StrValue:\n    @property\n    def value(self) -> str | None: ...\n\n",
        "class _Value: ...\n\n",
    ]
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
                item_type = names[int(target)].removesuffix("View") if target_kind == "Model" else f"{target}Value"
            output.append(f"class {name}(Sequence[{item_type}]): ...\n\n")
            continue
        output.append(f"class {name}:\n")
        static_fields = [field for field in by_parent.get(model_id, []) if _relation(field)[0] == "Key"]
        if not static_fields:
            output.append("    ...\n\n")
            continue
        for field in static_fields:
            key = _relation(field)[1]
            target_kind, target = _target(field)
            annotation = names[int(target)].removesuffix("View") if target_kind == "Model" else f"{target}Value"
            output.append(f"    @property\n    def {_identifier(str(key))}(self) -> {annotation} | None: ...\n")
        output.append("\n")
    return "".join(output)


__all__ = ["generate_validated_data_models"]
