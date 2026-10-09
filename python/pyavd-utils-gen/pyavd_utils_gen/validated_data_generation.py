# Copyright (c) 2026 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.
"""Generate nominal Rust views, native Python binding catalogs, and Python type declarations."""

from __future__ import annotations

import hashlib
import json
import keyword
import re
import shutil
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
    reused_schemas: dict[str, tuple[str, str]] | None = None,
) -> None:
    """
    Generate checked-in Rust views, native Python bindings, and Python typing declarations.

    Args:
        source: Combined source schema containing ``schema_name``.
        schema_name: Name of the schema root to generate from.
        rust_destination: Destination for the Rust model registry root. Generated schema modules
            are written below a sibling directory with the same stem, replacing any previous
            generated module tree.
        pyi_destination: Destination for the corresponding Python declarations.
        rust_root_name: Public Rust name of the root model, using Rust naming conventions.
        python_root_name: Public Python name of the root model, using Python naming conventions.
        root_keys: Optional static root keys to expose. Descendants of selected keys
            remain complete, while unselected root branches are omitted from the
            generated API. The runtime validated-data archive remains complete.
        reused_schemas: Schema names whose complete generated model catalogs should be
            reused for pure cross-schema references. Values contain the Rust and Python
            root names respectively.
    """
    reused_schemas = reused_schemas or {}
    registry = build_nominal_model_registry(source, schema_name, list(reused_schemas))
    if root_keys is not None:
        registry = _project_registry(registry, root_keys)
    slots = {int(field["id"]): int(field["slot"]) for field in registry["fields"]}
    models = list(registry["models"])
    fields = list(registry["fields"])
    python_root_names = {name: names[1] for name, names in reused_schemas.items()} | {schema_name: python_root_name}
    rust_root_names = {name: names[0] for name, names in reused_schemas.items()} | {schema_name: rust_root_name}
    names = _model_names(models, python_root_names)
    root_model = int(registry["root"])
    rust_sources = _render_rust(registry, models, fields, rust_root_names)
    rust_sources[()] += "\npub mod native;\n"
    rust_sources[("native",)] = _render_native_catalog(models, fields, slots, names, root_model)
    _write_rust(rust_destination, rust_sources)
    pyi_destination.write_text(_render_pyi(models, fields, names, rust_root_name, root_model), encoding="UTF-8")


def _render_native_catalog(models: list[dict[str, Any]], fields: list[dict[str, Any]], slots: dict[int, int], names: dict[int, str], root_model: int) -> str:
    """Emit native slot descriptors for the entire Python-visible graph, including keyed identities."""
    by_parent: dict[int, list[dict[str, Any]]] = {}
    for field in fields:
        by_parent.setdefault(int(field["parent"]), []).append(field)
    by_id = {int(model["id"]): model for model in models}

    def target(field: dict[str, Any]) -> str:
        kind, value = _target(field)
        if kind == "Scalar":
            return "Target::Scalar"
        if field["relaxed"]:
            return "Target::Opaque"
        return f"Target::Model({json.dumps(names[int(value)].removesuffix('View'))})"

    output = ["use ::validated_data_py::Target;\n\n::validated_data_py::python_data_views! {\n    pub BINDINGS {\n"]
    aliases = []
    for model in _python_visible_models(models, by_parent, root_model):
        model_id = int(model["id"])
        name = json.dumps(names[model_id].removesuffix("View"))
        model_fields = by_parent.get(model_id, [])
        if model["kind"] == "Dict":
            output.append(f"        {name} => dict {{\n")
            field_names = _field_names(model_fields)
            for field in model_fields:
                if _relation(field)[0] == "Key":
                    field_name = json.dumps(field_names[int(field["id"])])
                    output.append(f"            {field_name}: {slots[int(field['id'])]} => {target(field)},\n")
            output.append("        };\n")
            continue
        item = next((field for field in model_fields if _relation(field)[0] == "Item"), None)
        item_target = target(item) if item is not None else "Target::Scalar"
        keys = _primary_key_fields(model, by_id, by_parent)
        if keys:
            if item["relaxed"]:
                msg = "Python primary-key list items cannot expose an opaque relaxed item body"
                raise ValueError(msg)
            contextual_name = json.dumps(_keyed_item_name(model, names))
            item_target = f"Target::Model({contextual_name})"
            aliases.append(f"        {contextual_name} => alias({json.dumps(names[int(_target(item)[1])].removesuffix('View'))});\n")
        if model["indexed"]:
            key_slots = ", ".join(str(slots[int(key["id"])]) for key in keys)
            output.append(f"        {name} => indexed({item_target}, [{key_slots}]);\n")
        else:
            output.append(f"        {name} => list({item_target});\n")
    output.extend(aliases)
    output.append("    }\n}\n")
    return "".join(output)


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

    projected_fields = [dict(field) for field in registry["fields"] if int(field["id"]) in selected_fields]
    next_slots: dict[int, int] = {}
    for field in projected_fields:
        parent = int(field["parent"])
        field["slot"] = next_slots.get(parent, 0)
        next_slots[parent] = int(field["slot"]) + 1
    return {
        **registry,
        "models": [model for model in registry["models"] if int(model["id"]) in selected_models],
        "fields": projected_fields,
        "registry_hash": list(digest.digest()),
    }


def _model_names(models: list[dict[str, Any]], root_names: dict[str, str]) -> dict[int, str]:
    names: dict[int, str] = {}
    used: set[str] = set()
    for model in models:
        model_id = int(model["id"])
        schema_name = str(model["path"][0])
        root_name = root_names[schema_name]
        if len(model["path"]) == 1:
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


def _primary_key_fields(
    model: dict[str, Any],
    models: dict[int, dict[str, Any]],
    by_parent: dict[int, list[dict[str, Any]]],
) -> list[dict[str, Any]]:
    """Resolve item key-presence guarantees independently of key uniqueness."""
    primary_key_names = [str(name) for name in model.get("primary_key_fields", [])]
    if not primary_key_names:
        return []
    model_id = int(model["id"])
    item = next((field for field in by_parent.get(model_id, []) if _relation(field)[0] == "Item"), None)
    if item is None:
        msg = f"primary-key list model {model_id} has no item relationship"
        raise ValueError(msg)
    target_kind, target = _target(item)
    if target_kind != "Model" or models.get(int(target), {}).get("kind") != "Dict":
        msg = f"primary-key list model {model_id} does not contain dictionary items"
        raise ValueError(msg)
    item_fields = {
        relation_value: field
        for field in by_parent.get(int(target), [])
        if (relation_kind := _relation(field))[0] == "Key" and (relation_value := relation_kind[1]) is not None
    }
    resolved = []
    for name in primary_key_names:
        field = item_fields.get(name)
        if field is None:
            msg = f"primary-key list model {model_id} primary-key field {name!r} is missing"
            raise ValueError(msg)
        if _target(field)[0] != "Scalar":
            msg = f"primary-key list model {model_id} primary-key field {name!r} is not scalar"
            raise ValueError(msg)
        resolved.append(field)
    return resolved


def _render_rust(
    registry: dict[str, Any],
    models: list[dict[str, Any]],
    fields: list[dict[str, Any]],
    root_names: dict[str, str],
) -> dict[tuple[str, ...], str]:
    root_modules = {schema_name: _rust_module_identifier(root_name) for schema_name, root_name in root_names.items()}
    if "__pyavd_generated_registry" in root_modules.values():
        msg = "root name conflicts with the generated registry namespace"
        raise ValueError(msg)
    models_by_id = {int(model["id"]): model for model in models}
    by_parent: dict[int, list[dict[str, Any]]] = {}
    for field in fields:
        by_parent.setdefault(int(field["parent"]), []).append(field)
    output = [
        "// Copyright (c) 2026 Arista Networks, Inc.\n",
        "// Generated from the AVD schema. Do not edit by hand.\n\n",
        f"const REGISTRY_HASH: [u8; 32] = [{', '.join(str(value) for value in registry['registry_hash'])}];\n",
    ]
    module_outputs: dict[tuple[str, ...], list[str]] = {}
    visited: set[int] = set()
    locations: dict[int, tuple[str, ...]] = {}
    for schema_name, root_id in registry["roots"].items():
        root_name = root_names[schema_name]
        root_module = root_modules[schema_name]
        output.append(f"pub mod {root_module};\n")
        root_output = module_outputs.setdefault((root_module,), [])
        root_output.extend(
            _render_rust_model(
                int(root_id),
                _rust_type_identifier(root_name),
                None,
                schema_name,
                (root_module,),
                models_by_id,
                by_parent,
                locations,
                visited,
                "",
                module_outputs,
            )
        )
    primary_schema_name = next(name for name, root_id in registry["roots"].items() if int(root_id) == int(registry["root"]))
    primary_root_name = _rust_type_identifier(root_names[primary_schema_name])
    output.extend(
        [
            "pub const REGISTRY: ::validated_data::ModelRegistry = ::validated_data::ModelRegistry {\n",
            (f"    root_model: <{root_modules[primary_schema_name]}::{primary_root_name}<'static> as ::validated_data::ArchiveModel>::DESCRIPTOR,\n"),
            "    hash: REGISTRY_HASH,\n",
            "};\n",
        ]
    )
    if visited != models_by_id.keys():
        missing = ", ".join(str(model_id) for model_id in sorted(models_by_id.keys() - visited))
        msg = f"nominal model registry contains unreachable model(s): {missing}"
        raise ValueError(msg)
    return {(): "".join(output)} | {path: "".join(parts) for path, parts in module_outputs.items()}


def _write_rust(destination: Path, sources: dict[tuple[str, ...], str]) -> None:
    """Replace one generated Rust module tree with deterministic source files."""
    module_directory = destination.with_suffix("")
    if module_directory.exists():
        if not module_directory.is_dir():
            msg = f"generated Rust module path is not a directory: {module_directory}"
            raise ValueError(msg)
        shutil.rmtree(module_directory)
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_text(sources[()], encoding="UTF-8")
    header = "// Copyright (c) 2026 Arista Networks, Inc.\n// Generated from the AVD schema. Do not edit by hand.\n\n"
    for module_path, source in sorted(sources.items()):
        if not module_path:
            continue
        module_file = module_directory.joinpath(*module_path[:-1], f"{module_path[-1]}.rs")
        module_file.parent.mkdir(parents=True, exist_ok=True)
        module_file.write_text(f"{header}{source}", encoding="UTF-8")


def _render_rust_model(
    model_id: int,
    type_name: str,
    namespace_name: str | None,
    schema_name: str,
    scope: tuple[str, ...],
    models: dict[int, dict[str, Any]],
    by_parent: dict[int, list[dict[str, Any]]],
    locations: dict[int, tuple[str, ...]],
    visited: set[int],
    indent: str,
    module_outputs: dict[tuple[str, ...], list[str]],
) -> list[str]:
    """Render one nominal model and recursively nest its child occurrences."""
    if model_id in visited:
        msg = f"nominal model {model_id} is reachable from more than one relationship"
        raise ValueError(msg)
    visited.add(model_id)
    locations[model_id] = (*scope, type_name)
    model = models.get(model_id)
    if model is None:
        msg = f"nominal model registry references missing model {model_id}"
        raise ValueError(msg)

    kind = str(model["kind"])
    fields = by_parent.get(model_id, [])
    accessor_names = _field_names(fields)
    child_names = _rust_child_names(fields, frozenset({type_name}))
    item = None
    if kind == "List":
        item = next((field for field in fields if _relation(field)[0] == "Item"), None)
    primary_keys = _primary_key_fields(model, models, by_parent) if kind == "List" else []
    options = ""
    if kind == "List":
        options = "(list)"
        if primary_keys:
            item_fields = by_parent[int(_target(item)[1])]
            key_accessors = _field_names(item_fields)
            key_names = ", ".join(key_accessors[int(field["id"])] for field in primary_keys)
            list_kind = "indexed_list" if model["indexed"] else "list"
            options = f"({list_kind}, primary_key({key_names}))"
    output = [
        f"{indent}#[::validated_data::data_view{options}]\n",
        f"{indent}pub struct {type_name}<'a, Mode> ",
    ]
    if kind == "Dict":
        output.append("{\n")
        for field in fields:
            relation_kind, key = _relation(field)
            if key is None:
                continue
            target_type = _rust_target(field, models, child_names, namespace_name, schema_name, scope, locations)
            attribute = ""
            if relation_kind == "DynamicKey":
                attribute = f"dynamic = {json.dumps(key)}"
            elif accessor_names[int(field["id"])] != key:
                attribute = f"rename = {json.dumps(key)}"
            if field["relaxed"]:
                attribute = f"{attribute}, relaxed" if attribute else "relaxed"
            if attribute:
                output.append(f"{indent}    #[data_view({attribute})]\n")
            wrapper = f"::validated_data::RequiredValue<{target_type}, Mode>" if field["required"] else f"::validated_data::Field<{target_type}>"
            output.append(f"{indent}    pub {accessor_names[int(field['id'])]}: {wrapper},\n")
        output.append(f"{indent}}}\n")
    elif item is None:
        output.append(";\n")
    else:
        target_type = _rust_target(item, models, child_names, namespace_name, schema_name, scope, locations)
        wrapper = f"::validated_data::RequiredValue<{target_type}, Mode>" if item["required"] else f"::validated_data::Field<{target_type}>"
        attribute = "#[data_view(relaxed)] " if item["relaxed"] else ""
        output.append(f"({attribute}{wrapper});\n")
    model_fields = [field for field in fields if _target(field)[0] == "Model"]
    if not model_fields:
        return output

    child_output = output
    child_indent = indent
    inline_namespace = False
    if namespace_name is not None:
        if len(scope) == 1:
            output.append(f"\n{indent}pub mod {namespace_name};\n")
            child_output = module_outputs.setdefault((*scope, namespace_name), [])
            child_indent = ""
        else:
            output.append(f"\n{indent}pub mod {namespace_name} {{\n")
            child_indent += "    "
            inline_namespace = True
    for field in model_fields:
        target_kind, target = _target(field)
        if target_kind != "Model":  # pragma: no cover - filtered above
            continue
        target_id = int(target)
        target_model = models[target_id]
        if str(target_model["path"][0]) != schema_name:
            continue
        module_name, child_type_name = child_names[int(field["id"])]
        child_output.append("\n")
        child_output.extend(
            _render_rust_model(
                target_id,
                child_type_name,
                module_name,
                schema_name,
                (*scope, namespace_name) if namespace_name is not None else scope,
                models,
                by_parent,
                locations,
                visited,
                child_indent,
                module_outputs,
            )
        )
    if inline_namespace:
        output.append(f"{indent}}}\n")
    return output


def _field_names(fields: list[dict[str, Any]]) -> dict[int, str]:
    """Disambiguate normalized accessors without changing schema keys or declaration order."""
    bases = {int(field["id"]): _identifier(str(_relation(field)[1])) for field in fields if _relation(field)[1] is not None}
    counts: dict[str, int] = {}
    for base in bases.values():
        counts[base] = counts.get(base, 0) + 1
    names: dict[int, str] = {}
    occupied = set(bases.values())
    for field_id, base in bases.items():
        candidate = base if counts[base] == 1 else f"{base}_slot_{field_id}"
        while candidate in names.values() or (candidate != base and candidate in occupied):
            candidate += "_"
        names[field_id] = candidate
    return names


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
    schema_name: str,
    scope: tuple[str, ...],
    locations: dict[int, tuple[str, ...]],
) -> str:
    """Render a typed scalar or a nominal child with its relationship's validation mode."""
    target_kind, target = _target(field)
    if target_kind == "Scalar":
        return {"Bool": "bool", "Int": "i64", "Str": "&'a str"}[str(target)]
    target_id = int(target)
    target_model = models.get(target_id)
    if target_model is None:
        msg = f"nominal field {field['id']} references missing model {target_id}"
        raise ValueError(msg)
    if str(target_model["path"][0]) != schema_name:
        location = locations.get(target_id)
        if location is None:
            msg = f"external nominal model {target_id} must be rendered before use"
            raise ValueError(msg)
        qualified = "super::" * len(scope) + "::".join(location)
    else:
        _, type_name = child_names[int(field["id"])]
        qualified = type_name if namespace_name is None else f"{namespace_name}::{type_name}"
    mode = "::validated_data::RelaxedValidated" if field["relaxed"] else "Mode"
    return f"{qualified}<'a, {mode}>"


def _python_target_type(field: dict[str, Any], names: dict[int, str]) -> str:
    target_kind, target = _target(field)
    if target_kind == "Model" and field["relaxed"]:
        return "OpaqueData"
    return names[int(target)].removesuffix("View") if target_kind == "Model" else _python_scalar_type(target)


def _python_field_type(field: dict[str, Any], names: dict[int, str]) -> str:
    parts = [_python_target_type(field, names)]
    if not bool(field["required"]):
        parts.extend(["UndefinedType", "None"])
    return " | ".join(parts)


def _python_visible_models(models: list[dict[str, Any]], by_parent: dict[int, list[dict[str, Any]]], root_model: int) -> list[dict[str, Any]]:
    """Expose the strict root graph, stopping before opaque relaxed payloads."""
    visible = {root_model}
    pending = [root_model]
    while pending:
        for field in by_parent.get(pending.pop(), []):
            target_kind, target = _target(field)
            if target_kind == "Model" and _relation(field)[0] != "DynamicKey" and not field["relaxed"] and int(target) not in visible:
                visible.add(int(target))
                pending.append(int(target))
    return [model for model in models if int(model["id"]) in visible]


def _keyed_item_name(model: dict[str, Any], names: dict[int, str]) -> str:
    """Name the thin item wrapper carrying one list's primary-key guarantees."""
    suffix = "IndexedItem" if model["indexed"] else "KeyedItem"
    return f"{names[int(model['id'])].removesuffix('View')}{suffix}"


def _render_keyed_item_stubs(
    models: list[dict[str, Any]],
    by_parent: dict[int, list[dict[str, Any]]],
    names: dict[int, str],
    root_model: int,
) -> str:
    """Declare contextual key guarantees without changing the reusable item type's contract."""
    by_id = {int(model["id"]): model for model in models}
    output: list[str] = []
    for model in _python_visible_models(models, by_parent, root_model):
        primary_keys = _primary_key_fields(model, by_id, by_parent)
        if not primary_keys:
            continue
        item = next(field for field in by_parent[int(model["id"])] if _relation(field)[0] == "Item")
        if item["relaxed"]:
            msg = "Python primary-key list items cannot expose an opaque relaxed item body"
            raise ValueError(msg)
        output.append(f"class {_keyed_item_name(model, names)}({_python_target_type(item, names)}):\n")
        for key in primary_keys:
            name = _field_names(by_parent[int(_target(item)[1])])[int(key["id"])]
            output.append(f"    @property\n    def {name}(self) -> {_python_target_type(key, names)}:")
            output.append(" ...\n")
        output.append("\n")
    return "".join(output)


def _render_pyi(
    models: list[dict[str, Any]],
    fields: list[dict[str, Any]],
    names: dict[int, str],
    rust_root_name: str,
    root_model: int,
) -> str:
    by_parent: dict[int, list[dict[str, Any]]] = {}
    for field in fields:
        by_parent.setdefault(int(field["parent"]), []).append(field)
    models_by_id = {int(model["id"]): model for model in models}
    header = [
        "# Copyright (c) 2026 Arista Networks, Inc.\n",
        "# Generated from the AVD schema. Do not edit by hand.\n",
        "from collections.abc import Iterator, Sequence\n",
        "from pathlib import Path\n",
        "from typing import Any, overload\n\n",
        "from pyavd._utils.undefined import UndefinedType\n",
        "from pyavd._rust import OpaqueData\n\n",
    ]
    definitions: list[str] = []
    for model in _python_visible_models(models, by_parent, root_model):
        model_id = int(model["id"])
        name = names[model_id].removesuffix("View")
        model_fields = by_parent.get(model_id, [])
        if model["kind"] == "Dict":
            body = [f"class {name}:\n"]
            static_fields = [field for field in model_fields if _relation(field)[0] == "Key"]
            if not static_fields:
                body.append("    ...\n")
            for field in static_fields:
                name = _field_names(model_fields)[int(field["id"])]
                body.append(f"    @property\n    def {name}(self) -> {_python_field_type(field, names)}: ...\n")
            definitions.append("".join(body).rstrip())
            continue
        item = next((field for field in model_fields if _relation(field)[0] == "Item"), None)
        base_item_type = _python_target_type(item, names) if item is not None else "Any"
        primary_key_fields = _primary_key_fields(model, models_by_id, by_parent)
        if primary_key_fields:
            base_item_type = _keyed_item_name(model, names)
        item_type = base_item_type if primary_key_fields or (item is not None and item["required"]) else f"{base_item_type} | None"
        body = [f"class {name}(Sequence[{item_type}]):\n", f"    def __iter__(self) -> Iterator[{item_type}]: ...\n"]
        if not model["indexed"]:
            body.extend(
                [
                    "    @overload\n",
                    f"    def __getitem__(self, index: int) -> {item_type}: ...\n",
                    "    @overload\n",
                    f"    def __getitem__(self, index: slice) -> list[{item_type}]: ...\n",
                ]
            )
            definitions.append("".join(body).rstrip())
            continue
        if len(primary_key_fields) != 1:
            msg = f"composite primary keys are not supported yet for model {model_id}"
            raise ValueError(msg)
        primary_key = primary_key_fields[0]
        primary_key_type = _python_target_type(primary_key, names)
        item_without_none = base_item_type
        body.extend(
            [
                f"    def __contains__(self, key: {primary_key_type}) -> bool: ...\n",
                f"    def __getitem__(self, key: {primary_key_type}) -> {item_without_none}: ...\n",
                "    @overload\n",
                f"    def get(self, key: {primary_key_type}) -> {item_without_none} | UndefinedType: ...\n",
                "    @overload\n",
                f"    def get(self, key: {primary_key_type}, default: Any) -> {item_without_none} | Any: ...\n",
                f"    def keys(self) -> Iterator[{primary_key_type}]: ...\n",
                f"    def values(self) -> Iterator[{item_type}]: ...\n",
                f"    def items(self) -> Iterator[tuple[{primary_key_type}, {item_without_none}]]: ...\n",
            ]
        )
        definitions.append("".join(body).rstrip())
    definitions.append(_render_keyed_item_stubs(models, by_parent, names, root_model).rstrip())
    output = ["".join(header), "\n\n".join(definitions)]
    root_name = names[root_model].removesuffix("View")
    output.extend(
        [
            "\n\n",
            f"def open_{_rust_module_identifier(rust_root_name)}(archive: Path, schema_archive: Path) -> {root_name}: ...\n",
        ]
    )
    return "".join(output)


__all__ = ["generate_validated_data_models"]
