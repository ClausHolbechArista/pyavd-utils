# Copyright (c) 2026 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.
"""Generate nominal Rust views and matching Python type declarations."""

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
    python_destination: Path,
    pyi_destination: Path,
    rust_root_name: str,
    python_root_name: str,
    root_keys: list[str] | None = None,
    reused_schemas: dict[str, tuple[str, str]] | None = None,
) -> None:
    """
    Generate checked-in Rust views and Python declarations from one schema root.

    Args:
        source: Combined source schema containing ``schema_name``.
        schema_name: Name of the schema root to generate from.
        rust_destination: Destination for the Rust model registry root. Generated schema modules
            are written below a sibling directory with the same stem, replacing any previous
            generated module tree.
        python_destination: Destination for the Python runtime view classes.
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
    slots = {int(field["id"]): int(field["slot"]) for field in registry["fields"]}
    if root_keys is not None:
        registry = _project_registry(registry, root_keys)
    models = list(registry["models"])
    fields = list(registry["fields"])
    python_root_names = {name: names[1] for name, names in reused_schemas.items()} | {schema_name: python_root_name}
    rust_root_names = {name: names[0] for name, names in reused_schemas.items()} | {schema_name: rust_root_name}
    names = _model_names(models, python_root_names)
    _write_rust(rust_destination, _render_rust(registry, models, fields, slots, rust_root_names))
    root_model = int(registry["root"])
    python_destination.write_text(_render_python(models, fields, slots, names, rust_root_name, root_model), encoding="UTF-8")
    pyi_destination.write_text(_render_pyi(models, fields, names, rust_root_name, root_model), encoding="UTF-8")


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
    """Resolve the ordered scalar field slots forming one indexed-list primary key."""
    primary_key_names = [str(name) for name in model.get("primary_key_fields", [])]
    if not primary_key_names:
        return []
    model_id = int(model["id"])
    item = next((field for field in by_parent.get(model_id, []) if _relation(field)[0] == "Item"), None)
    if item is None:
        msg = f"indexed-list model {model_id} has no item relationship"
        raise ValueError(msg)
    target_kind, target = _target(item)
    if target_kind != "Model" or models.get(int(target), {}).get("kind") != "Dict":
        msg = f"indexed-list model {model_id} does not contain dictionary items"
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
            msg = f"indexed-list model {model_id} primary-key field {name!r} is missing"
            raise ValueError(msg)
        if _target(field)[0] != "Scalar":
            msg = f"indexed-list model {model_id} primary-key field {name!r} is not scalar"
            raise ValueError(msg)
        resolved.append(field)
    return resolved


def _render_rust(
    registry: dict[str, Any],
    models: list[dict[str, Any]],
    fields: list[dict[str, Any]],
    slots: dict[int, int],
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
                slots,
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
            "pub const REGISTRY: ::validation::archive::ModelRegistry = ::validation::archive::ModelRegistry {\n",
            (
                f"    root_model: <{root_modules[primary_schema_name]}::{primary_root_name}<'static> "
                "as ::validation::archive::ArchiveModel>::DESCRIPTOR,\n"
            ),
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
    slots: dict[int, int],
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
    child_names = _rust_child_names(fields, frozenset({type_name}))
    macro_name = "define_archive_dict_view"
    item = None
    if kind == "List":
        item = next((field for field in fields if _relation(field)[0] == "Item"), None)
        macro_name = "define_archive_indexed_list_view" if _primary_key_fields(model, models, by_parent) else "define_archive_list_view"
    output = [f"{indent}::validation::{macro_name}! {{\n", f"{indent}    #[derive(Clone, Copy, Debug)]\n", f"{indent}    pub struct {type_name} {{\n"]
    if kind == "Dict":
        for field in fields:
            relation_kind, key = _relation(field)
            if key is None:
                continue
            target_kind, target_type = _rust_macro_target(field, models, child_names, namespace_name, schema_name, scope, locations)
            if relation_kind == "DynamicKey":
                target_kind = f"dynamic_{target_kind}"
            output.append(
                f'{indent}        {target_kind} {_identifier(key)}({json.dumps(key)}, {slots[int(field["id"])]}) -> {target_type};\n'
            )
    elif item is None:
        output.append(f"{indent}        raw;\n")
    else:
        target_kind, target_type = _rust_macro_target(item, models, child_names, namespace_name, schema_name, scope, locations)
        output.append(f'{indent}        {target_kind} item ({slots[int(item["id"])]}) -> {target_type};\n')
        if macro_name == "define_archive_indexed_list_view":
            primary_key_slots = ", ".join(str(slots[int(field["id"])]) for field in _primary_key_fields(model, models, by_parent))
            output.append(f"{indent}        primary_key_fields: [{primary_key_slots}];\n")
    output.extend([f"{indent}    }}\n", f"{indent}}}\n"])

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
                slots,
                locations,
                visited,
                child_indent,
                module_outputs,
            )
        )
    if inline_namespace:
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
    schema_name: str,
    scope: tuple[str, ...],
    locations: dict[int, tuple[str, ...]],
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
    if str(target_model["path"][0]) != schema_name:
        location = locations.get(target_id)
        if location is None:
            msg = f"external nominal model {target_id} must be rendered before use"
            raise ValueError(msg)
        qualified = "super::" * len(scope) + "::".join(location)
    else:
        _, type_name = child_names[int(field["id"])]
        qualified = type_name if namespace_name is None else f"{namespace_name}::{type_name}"
    return (f"{qualified}<'a>", f".and_then({qualified}::from_value)")


def _rust_macro_target(
    field: dict[str, Any],
    models: dict[int, dict[str, Any]],
    child_names: dict[int, tuple[str, str]],
    namespace_name: str | None,
    schema_name: str,
    scope: tuple[str, ...],
    locations: dict[int, tuple[str, ...]],
) -> tuple[str, str]:
    """Return the macro field kind and its future typed Rust return marker."""
    target_kind, target = _target(field)
    if target_kind == "Scalar":
        scalar_type = {"Bool": "bool", "Int": "i64", "Str": "&'a str"}[str(target)]
        return ("scalar", scalar_type)
    return (
        "model",
        _rust_target(field, models, child_names, namespace_name, schema_name, scope, locations)[0],
    )


def _python_target_type(field: dict[str, Any], names: dict[int, str]) -> str:
    target_kind, target = _target(field)
    return names[int(target)].removesuffix("View") if target_kind == "Model" else _python_scalar_type(target)


def _python_field_type(field: dict[str, Any], names: dict[int, str], primary_key_field_ids: frozenset[int]) -> str:
    parts = [_python_target_type(field, names)]
    if int(field["id"]) in primary_key_field_ids:
        return parts[0]
    if not bool(field["required"]) and not bool(field["has_default"]):
        parts.append("UndefinedType")
    parts.append("None")
    return " | ".join(parts)


def _python_model_wrap(field: dict[str, Any], names: dict[int, str], value: str) -> str:
    target_kind, target = _target(field)
    if target_kind != "Model":
        return value
    target_name = names[int(target)].removesuffix("View")
    return f"_wrap_model({value}, {target_name})"


def _render_python(
    models: list[dict[str, Any]],
    fields: list[dict[str, Any]],
    slots: dict[int, int],
    names: dict[int, str],
    rust_root_name: str,
    root_model: int,
) -> str:
    """Render Python model identities backed by generic PyO3 archive handles."""
    by_parent: dict[int, list[dict[str, Any]]] = {}
    for field in fields:
        by_parent.setdefault(int(field["parent"]), []).append(field)
    models_by_id = {int(model["id"]): model for model in models}
    primary_key_field_ids = frozenset(int(field["id"]) for model in models for field in _primary_key_fields(model, models_by_id, by_parent))
    root_name = names[root_model].removesuffix("View")
    open_name = f"open_{_rust_module_identifier(rust_root_name)}"
    output = [
        "# Copyright (c) 2026 Arista Networks, Inc.\n",
        "# Generated from the AVD schema. Do not edit by hand.\n",
        "# ruff: noqa: EM101, TC003, TRY003\n",
        "from __future__ import annotations\n\n",
        "from collections.abc import Iterator\n",
        "from pathlib import Path\n",
        "from typing import Any\n\n",
        f"from pyavd._rust import _DictView, _ListView, _ValueHandle, _{open_name}_handle\n",
        "from pyavd._utils.undefined import Undefined, UndefinedType\n\n",
        "def _wrap_model(value: Any, model: type[Any]) -> Any:\n",
        "    return model(value) if isinstance(value, _ValueHandle) else value\n\n",
    ]
    for model in models:
        model_id = int(model["id"])
        name = names[model_id].removesuffix("View")
        model_fields = by_parent.get(model_id, [])
        if model["kind"] == "Dict":
            output.append(f"class {name}(_DictView):\n")
            static_fields = [field for field in model_fields if _relation(field)[0] == "Key"]
            if not static_fields:
                output.append("    pass\n\n")
                continue
            for field in static_fields:
                key = str(_relation(field)[1])
                annotation = _python_field_type(field, names, primary_key_field_ids)
                expression = _python_model_wrap(field, names, f"self._get_field({slots[int(field['id'])]})")
                output.extend(
                    [
                        "    @property\n",
                        f"    def {_identifier(key)}(self) -> {annotation}:\n",
                        f"        return {expression}\n",
                    ]
                )
            output.append("\n")
            continue

        item = next((field for field in model_fields if _relation(field)[0] == "Item"), None)
        item_type = _python_target_type(item, names) if item is not None else "Any"
        item_expression = _python_model_wrap(item, names, "self._get_item(index)") if item is not None else "self._get_item(index)"
        primary_key_fields = _primary_key_fields(model, models_by_id, by_parent)
        returned_item_type = item_type if primary_key_fields else f"{item_type} | None"
        output.append(f"class {name}(_ListView):\n")
        output.extend(
            [
                f"    def __iter__(self) -> Iterator[{returned_item_type}]:\n",
                "        for index in range(len(self)):\n",
                "            yield self._item_at(index)\n\n",
                f"    def _item_at(self, index: int) -> {returned_item_type}:\n",
            ]
        )
        if primary_key_fields:
            output.extend(
                [
                    f"        value = {item_expression}\n",
                    "        if value is None:\n",
                    '            raise RuntimeError("indexed-list item is null")\n',
                    "        return value\n",
                ]
            )
        else:
            output.append(f"        return {item_expression}\n")
        if not primary_key_fields:
            output.extend(
                [
                    f"    def __getitem__(self, index: int | slice) -> {item_type} | None | list[{item_type} | None]:\n",
                    "        if isinstance(index, slice):\n",
                    "            return [self._item_at(item_index) for item_index in range(*index.indices(len(self)))]\n",
                    "        return self._item_at(index)\n\n",
                ]
            )
            continue
        if len(primary_key_fields) != 1:
            msg = f"composite primary keys are not supported yet for model {model_id}"
            raise ValueError(msg)
        primary_key = primary_key_fields[0]
        primary_key_name = str(_relation(primary_key)[1])
        primary_key_type = _python_target_type(primary_key, names)
        if item is None:
            msg = f"indexed-list model {model_id} has no item relationship"
            raise ValueError(msg)
        wrapped_lookup = _python_model_wrap(item, names, "value")
        output.extend(
            [
                f"    def __contains__(self, key: {primary_key_type}) -> bool:\n",
                "        return self._contains_primary_key((key,))\n\n",
                f"    def __getitem__(self, key: {primary_key_type}) -> {item_type}:\n",
                "        value = self._get_by_primary_key((key,))\n",
                "        if isinstance(value, UndefinedType):\n",
                "            raise KeyError(key)\n",
                f"        return {wrapped_lookup}\n\n",
                f"    def get(self, key: {primary_key_type}, default: Any = Undefined) -> {item_type} | Any:\n",
                "        value = self._get_by_primary_key((key,))\n",
                "        if isinstance(value, UndefinedType):\n",
                "            return default\n",
                f"        return {wrapped_lookup}\n\n",
                f"    def keys(self) -> Iterator[{primary_key_type}]:\n",
                f"        return (item.{_identifier(primary_key_name)} for item in self)\n\n",
                f"    def values(self) -> Iterator[{item_type}]:\n",
                "        return iter(self)\n\n",
                f"    def items(self) -> Iterator[tuple[{primary_key_type}, {item_type}]]:\n",
                f"        return ((item.{_identifier(primary_key_name)}, item) for item in self)\n\n",
            ]
        )
    output.extend(
        [
            f"def {open_name}(archive: Path, schema_archive: Path) -> {root_name}:\n",
            f"    return {root_name}(_{open_name}_handle(archive, schema_archive))\n",
        ]
    )
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
    primary_key_field_ids = frozenset(int(field["id"]) for model in models for field in _primary_key_fields(model, models_by_id, by_parent))
    header = [
        "# Copyright (c) 2026 Arista Networks, Inc.\n",
        "# Generated from the AVD schema. Do not edit by hand.\n",
        "from collections.abc import Iterator, Sequence\n",
        "from pathlib import Path\n",
        "from typing import Any, overload\n\n",
        "from pyavd._utils.undefined import UndefinedType\n\n",
    ]
    definitions: list[str] = []
    for model in models:
        model_id = int(model["id"])
        name = names[model_id].removesuffix("View")
        model_fields = by_parent.get(model_id, [])
        if model["kind"] == "Dict":
            body = [f"class {name}:\n"]
            static_fields = [field for field in model_fields if _relation(field)[0] == "Key"]
            if not static_fields:
                body.append("    ...\n")
            for field in static_fields:
                key = str(_relation(field)[1])
                body.append(f"    @property\n    def {_identifier(key)}(self) -> {_python_field_type(field, names, primary_key_field_ids)}: ...\n")
            definitions.append("".join(body).rstrip())
            continue
        item = next((field for field in model_fields if _relation(field)[0] == "Item"), None)
        base_item_type = _python_target_type(item, names) if item is not None else "Any"
        primary_key_fields = _primary_key_fields(model, models_by_id, by_parent)
        item_type = base_item_type if primary_key_fields else f"{base_item_type} | None"
        body = [f"class {name}(Sequence[{item_type}]):\n", f"    def __iter__(self) -> Iterator[{item_type}]: ...\n"]
        if not primary_key_fields:
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
