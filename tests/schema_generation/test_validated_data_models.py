# Copyright (c) 2026 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.

import ast
import json
import re
from pathlib import Path

import pytest

from pyavd_utils_gen.validated_data_generation import generate_validated_data_models

ARTIFACTS = Path(__file__).parent / "artifacts"


def _rust_sources(root: Path) -> dict[str, str]:
    """Read one generated Rust module tree using stable relative names."""
    module_directory = root.with_suffix("")
    sources = {"__root__.rs": root.read_text(encoding="UTF-8")}
    sources.update((str(path.relative_to(module_directory)), path.read_text(encoding="UTF-8")) for path in sorted(module_directory.rglob("*.rs")))
    return sources


def _combined_rust_source(root: Path) -> str:
    """Combine generated Rust files for assertions independent of file boundaries."""
    return "\n".join(_rust_sources(root).values())


def _stub_sources(package: Path) -> dict[str, str]:
    """Read one generated stub package using stable relative names."""
    return {path.name: path.read_text(encoding="UTF-8") for path in sorted(package.glob("*.pyi"))}


def _combined_stub_source(package: Path) -> str:
    """Combine generated stub modules for assertions independent of file boundaries."""
    return "\n".join(_stub_sources(package).values())


def _stub_classes(package: Path) -> dict[str, ast.ClassDef]:
    """Index nested declarations from every stub module by public qualified class paths."""
    classes: dict[str, ast.ClassDef] = {}

    def collect(body: list[ast.stmt], scope: str = "") -> None:
        for node in body:
            if isinstance(node, ast.ClassDef):
                name = f"{scope}.{node.name}" if scope else node.name
                classes[name] = node
                collect(node.body, name)

    for source in _stub_sources(package).values():
        collect(ast.parse(source).body)
    return classes


def test_collection_api_signatures_are_declared_once(tmp_path: Path) -> None:
    """Keep native utility signatures and documentation in shared bases, not every model."""
    rust, pyi = tmp_path / "models.rs", tmp_path / "models_stubs"
    generate_validated_data_models(ARTIFACTS / "schemas.json", "schema_generation_fixture", rust, pyi, "SchemaGenerationFixture", "SchemaGenerationFixture")
    classes = [node for node in ast.parse(_combined_stub_source(pyi)).body if isinstance(node, ast.ClassDef)]
    methods = [node.name for model in classes for node in model.body if isinstance(node, ast.FunctionDef)]
    for name in ["__bool__", "__len__", "__iter__", "__reversed__", "count", "index", "keys", "values", "items"]:
        assert methods.count(name) == 1
    assert methods.count("__eq__") == 2  # Dictionary and collection equality have distinct documentation.
    assert methods.count("get") == 2  # Both overloads are declared only on the indexed base.


def test_collection_stubs_distinguish_positional_and_primary_key_access(tmp_path: Path) -> None:
    """Promise Sequence only where integer indexing denotes a position, not a primary key."""
    source = tmp_path / "schemas.json"
    source.write_text(
        json.dumps(
            {
                "fixture": {
                    "type": "dict",
                    "keys": {
                        "empty": {"type": "dict"},
                        "values": {"type": "list", "items": {"type": "str"}},
                        "entries": {
                            "type": "list",
                            "primary_key": "id",
                            "items": {
                                "type": "dict",
                                "keys": {"id": {"type": "int"}},
                            },
                        },
                    },
                }
            }
        ),
        encoding="UTF-8",
    )
    rust, pyi = tmp_path / "models.rs", tmp_path / "models_stubs"
    generate_validated_data_models(source, "fixture", rust, pyi, "Fixture", "Fixture")
    classes = _stub_classes(pyi)
    positional = classes["Fixture.Values"]
    keyed = classes["Fixture.Entries"]
    assert [ast.unparse(base) for base in positional.bases] == ["_List[str | None]"]
    assert [ast.unparse(base) for base in classes["_List"].bases] == ["_Collection[_T_co]", "Sequence[_T_co]"]
    assert not any(isinstance(node, ast.FunctionDef) for node in classes["_List"].body)
    assert [ast.unparse(base) for base in keyed.bases] == ["_IndexedList[int, Fixture.EntriesItem]"]
    assert [ast.unparse(base) for base in classes["Fixture.EntriesItem"].bases] == ["Fixture.EntriesItemBase"]
    collection_methods = {node.name: node for node in classes["_Collection"].body if isinstance(node, ast.FunctionDef)}
    assert {"__len__", "__iter__", "__reversed__", "count", "index", "__eq__"} == collection_methods.keys()
    assert ast.unparse(collection_methods["count"].args.args[1].annotation) == "object"
    assert ast.unparse(collection_methods["__iter__"].returns) == "Iterator[_T_co]"
    assert not any(isinstance(node, ast.FunctionDef) for node in positional.body)
    indexed_methods = {node.name: node for node in classes["_IndexedList"].body if isinstance(node, ast.FunctionDef)}
    assert {"__contains__", "__getitem__", "get", "keys", "values", "items"} == indexed_methods.keys()
    key_getitem = indexed_methods["__getitem__"]
    assert key_getitem.args.args[1].arg == "key"
    assert ast.unparse(key_getitem.args.args[1].annotation) == "_K"
    assert ast.unparse(key_getitem.returns) == "_T_co"
    assert [ast.unparse(base) for base in classes["Fixture.Empty"].bases] == ["_Model"]
    model_methods = {node.name for node in classes["_Model"].body if isinstance(node, ast.FunctionDef)}
    assert {"__bool__", "__eq__", "_is_defined", "_is_default"} == model_methods


def test_generate_validated_data_models(tmp_path: Path) -> None:
    """Generate nominal Rust views and Python declarations from the shared fixture."""
    rust = tmp_path / "models.rs"
    pyi = tmp_path / "models_stubs"

    generate_validated_data_models(
        ARTIFACTS / "schemas.json",
        "schema_generation_fixture",
        rust,
        pyi,
        "SchemaGenerationFixture",
        "SchemaGenerationFixture",
    )

    rust_source = _combined_rust_source(rust)
    assert not (rust.with_suffix("") / "native.rs").exists()
    pyi_source = _combined_stub_source(pyi)
    assert (rust.with_suffix("") / "schema_generation_fixture.rs").is_file()
    assert (rust.with_suffix("") / "schema_generation_fixture" / "interface_profiles.rs").is_file()
    assert "pub const REGISTRY: ::validated_data::ModelRegistry" in rust_source
    assert "pub mod schema_generation_fixture;" in rust_source
    assert "data_view(indexed_list, primary_key(name), python(" in rust_source
    assert "#[derive(Clone, Copy, Debug)]" not in rust_source
    assert "pub struct InterfaceProfiles<'a, Mode>" in rust_source
    assert "pub mod interface_profiles;" in rust_source
    assert "pub struct Item<'a, Mode>" in rust_source
    assert "pub interface_profiles: ::validated_data::Field<" in rust_source
    assert "primary_key(name)" in rust_source
    assert re.search(r'python\(name = "SchemaGenerationFixture", schema = Dict\(\d+\)\)', rust_source)
    assert re.search(
        r'python\(name = "SchemaGenerationFixture.InterfaceProfiles", schema = List\(\d+\), item_name = "SchemaGenerationFixture.InterfaceProfilesItem"\)',
        rust_source,
    )
    assert "class InterfaceProfiles(_IndexedList[str, SchemaGenerationFixture.InterfaceProfilesItem]):" in pyi_source
    assert "class SchemaGenerationFixture(_Model):" in pyi_source
    assert "class StrValue:" not in pyi_source
    assert "_List[str | None]" in pyi_source
    assert "-> str | None" in pyi_source
    assert not (tmp_path / "models.py").exists()
    ast.parse(pyi_source)


def test_root_level_collection_has_parent_reexport_and_keeps_item_namespace(tmp_path: Path) -> None:
    """Expose a root collection as a sibling type while keeping its item in the module."""
    source = tmp_path / "schemas.json"
    source.write_text(
        json.dumps(
            {
                "fixture": {
                    "type": "dict",
                    "keys": {
                        "devices": {
                            "type": "list",
                            "primary_key": "name",
                            "items": {
                                "type": "dict",
                                "keys": {
                                    "name": {"type": "str"},
                                    "metadata": {"type": "dict", "keys": {"description": {"type": "str"}}},
                                },
                            },
                        }
                    },
                }
            }
        ),
        encoding="UTF-8",
    )
    rust = tmp_path / "avd_design.rs"
    stubs = tmp_path / "avd_design_stubs"
    generate_validated_data_models(source, "fixture", rust, stubs, "AvdDesign", "AVDDesign")

    root_module = (rust.with_suffix("") / "avd_design.rs").read_text(encoding="UTF-8")
    devices_module = (rust.with_suffix("") / "avd_design" / "devices.rs").read_text(encoding="UTF-8")
    assert "pub mod devices;" in root_module
    assert "pub use devices::Devices;" in root_module
    assert "pub devices: ::validated_data::Field<Devices<'a, Mode>>," in root_module
    assert "pub struct Devices<'a, Mode>" in devices_module
    assert "pub struct Item<'a, Mode>" in devices_module
    assert "pub mod item {" in devices_module
    assert "pub struct Metadata<'a, Mode>" in devices_module
    assert "pub use devices::Item;" not in root_module


def test_generate_validated_data_models_projects_static_root_keys(tmp_path: Path) -> None:
    """Keep selected branches complete and preserve their registry identities."""
    full_rust = tmp_path / "full.rs"
    full_pyi = tmp_path / "full_stubs"
    projected_rust = tmp_path / "projected.rs"
    projected_pyi = tmp_path / "projected_stubs"
    reordered_rust = tmp_path / "reordered.rs"
    reordered_pyi = tmp_path / "reordered_stubs"

    generate_validated_data_models(
        ARTIFACTS / "schemas.json",
        "schema_generation_fixture",
        full_rust,
        full_pyi,
        "SchemaGenerationFixture",
        "SchemaGenerationFixture",
    )
    generate_validated_data_models(
        ARTIFACTS / "schemas.json",
        "schema_generation_fixture",
        projected_rust,
        projected_pyi,
        "SchemaGenerationFixture",
        "SchemaGenerationFixture",
        root_keys=["accounting", "interface_profiles"],
    )
    generate_validated_data_models(
        ARTIFACTS / "schemas.json",
        "schema_generation_fixture",
        reordered_rust,
        reordered_pyi,
        "SchemaGenerationFixture",
        "SchemaGenerationFixture",
        root_keys=["interface_profiles", "accounting"],
    )

    full_source = _combined_rust_source(full_rust)
    projected_source = _combined_rust_source(projected_rust)
    projected_pyi_source = _combined_stub_source(projected_pyi)
    assert "pub accounting: ::validated_data::Field<" in projected_source
    assert "pub methods: ::validated_data::Field<" in projected_source
    assert "pub authentication: ::validated_data::Field<" not in projected_source
    assert "def accounting(" in projected_pyi_source
    assert "def authentication(" not in projected_pyi_source
    assert projected_source != full_source
    assert _rust_sources(projected_rust) == _rust_sources(reordered_rust)
    assert not (projected_rust.with_suffix("") / "native.rs").exists()
    assert _stub_sources(projected_pyi) == _stub_sources(reordered_pyi)

    full_accounting = next(line for line in full_source.splitlines() if "pub accounting: ::validated_data::Field<" in line)
    projected_accounting = next(line for line in projected_source.splitlines() if "pub accounting: ::validated_data::Field<" in line)
    assert projected_accounting == full_accounting
    full_hash = next(line for line in full_source.splitlines() if line.startswith("const REGISTRY_HASH"))
    projected_hash = next(line for line in projected_source.splitlines() if line.startswith("const REGISTRY_HASH"))
    assert projected_hash != full_hash
    ast.parse(projected_pyi_source)


def test_generate_validated_data_models_reuses_pure_cross_schema_references(tmp_path: Path) -> None:
    """Generate a referenced schema once and point pure cross-schema fields at its views."""
    source = tmp_path / "schemas.json"
    source.write_text(
        json.dumps(
            {
                "external": {"type": "dict", "keys": {"shared": {"type": "dict", "keys": {"name": {"type": "str"}}}}},
                "root": {"type": "dict", "keys": {"value": {"type": "dict", "$ref": "external#/keys/shared"}}},
            }
        ),
        encoding="UTF-8",
    )
    rust = tmp_path / "models.rs"
    pyi = tmp_path / "models_stubs"

    generate_validated_data_models(
        source,
        "root",
        rust,
        pyi,
        "Root",
        "Root",
        reused_schemas={"external": ("External", "External")},
    )

    rust_source = _combined_rust_source(rust)
    assert "pub mod external;" in rust_source
    assert rust_source.count("pub struct Shared<'a, Mode>") == 1
    assert "super::external::Shared<'a, Mode>" in rust_source
    assert re.search(r'python\(name = "External.Shared", schema = Dict\(\d+\)\)', rust_source)
    assert 'python(name = "Root.Value",' not in rust_source
    assert rust_source.count('python(name = "External.Shared",') == 1
    classes = _stub_classes(pyi)
    assert classes["External"].bases == []  # Only a namespace for the referenced subtree.
    assert "External.Shared" in classes
    assert "from .external import External" in (pyi / "root.pyi").read_text(encoding="UTF-8")
    assert "def value(self) -> External.Shared | None:" in (pyi / "root.pyi").read_text(encoding="UTF-8")
    assert "from .root import Root as Root" in (pyi / "__init__.pyi").read_text(encoding="UTF-8")
    assert "def open_root(archive: Path, schema_archive: Path) -> Root:" in (pyi / "__init__.pyi").read_text(encoding="UTF-8")


def test_generate_validated_data_models_rejects_unknown_root_key(tmp_path: Path) -> None:
    """Reject a misspelled projection instead of silently generating an empty branch."""
    with pytest.raises(ValueError, match=r"unknown static root key.*missing"):
        generate_validated_data_models(
            ARTIFACTS / "schemas.json",
            "schema_generation_fixture",
            tmp_path / "models.rs",
            tmp_path / "models_stubs",
            "SchemaGenerationFixture",
            "SchemaGenerationFixture",
            root_keys=["missing"],
        )


def test_generate_validated_data_models_rejects_stub_package_inside_rust_module_tree(tmp_path: Path) -> None:
    """Reject colliding output trees before replacing any caller-owned files."""
    rust = tmp_path / "models.rs"
    stub_package = tmp_path / "models"
    stub_package.mkdir()
    marker = stub_package / "keep.txt"
    marker.write_text("preserve this file\n", encoding="UTF-8")

    with pytest.raises(ValueError, match="stub package must be outside the generated Rust module tree"):
        generate_validated_data_models(
            ARTIFACTS / "schemas.json",
            "schema_generation_fixture",
            rust,
            stub_package,
            "SchemaGenerationFixture",
            "SchemaGenerationFixture",
        )

    assert marker.read_text(encoding="UTF-8") == "preserve this file\n"
    assert not rust.exists()


def test_nested_names_preserve_canonical_reuse_and_item_guarantees(tmp_path: Path) -> None:
    """Nest local item bases and reuse an external subtree without copying its types."""
    source = tmp_path / "schemas.json"
    source.write_text(
        json.dumps(
            {
                "external": {
                    "type": "dict",
                    "keys": {"section": {"type": "dict", "keys": {"settings": {"type": "dict", "keys": {"enabled": {"type": "bool"}}}}}},
                },
                "fixture": {
                    "type": "dict",
                    "keys": {
                        "first": {"type": "dict", "$ref": "external#/keys/section/keys/settings"},
                        "second": {"type": "dict", "$ref": "external#/keys/section/keys/settings"},
                        "devices": {
                            "type": "list",
                            "primary_key": "name",
                            "items": {"type": "dict", "keys": {"name": {"type": "str"}, "items": {"type": "dict", "keys": {"keys": {"type": "dict"}}}}},
                        },
                    },
                },
            }
        ),
        encoding="UTF-8",
    )
    rust, pyi = tmp_path / "models.rs", tmp_path / "models_stubs"
    generate_validated_data_models(source, "fixture", rust, pyi, "Fixture", "Fixture", reused_schemas={"external": ("EosConfig", "EOSConfig")})
    stub = _combined_stub_source(pyi)
    classes = _stub_classes(pyi)
    assert {
        "Fixture",
        "EOSConfig",
        "EOSConfig.Section",
        "EOSConfig.Section.Settings",
        "Fixture.Devices",
        "Fixture.DevicesItemBase",
        "Fixture.DevicesItem",
    } <= classes.keys()
    assert classes["EOSConfig"].bases == []
    assert classes["EOSConfig.Section"].bases == []
    assert "def first(self) -> EOSConfig.Section.Settings | None:" in stub
    assert "def second(self) -> EOSConfig.Section.Settings | None:" in stub
    assert "Fixture.First" not in classes
    assert "Fixture.Second" not in classes
    assert "Fixture.DevicesItemBase.Items.Keys" in classes
    assert [ast.unparse(base) for base in classes["Fixture.DevicesItem"].bases] == ["Fixture.DevicesItemBase"]
    base_name = next(node for node in classes["Fixture.DevicesItemBase"].body if isinstance(node, ast.FunctionDef) and node.name == "name")
    guaranteed_name = next(node for node in classes["Fixture.DevicesItem"].body if isinstance(node, ast.FunctionDef) and node.name == "name")
    assert ast.unparse(base_name.returns) == "str | None"
    assert ast.unparse(guaranteed_name.returns) == "str"
    rust_source = _combined_rust_source(rust)
    assert rust_source.count('python(name = "EOSConfig.Section.Settings",') == 1
    assert "pub mod eos_config;" in rust_source
    projected_rust, projected_pyi = tmp_path / "projected.rs", tmp_path / "projected_stubs"
    generate_validated_data_models(
        source, "fixture", projected_rust, projected_pyi, "Fixture", "Fixture", root_keys=["first"], reused_schemas={"external": ("EosConfig", "EOSConfig")}
    )
    assert "EOSConfig.Section.Settings" in _stub_classes(projected_pyi)


def test_generate_validated_data_models_normalizes_identifiers(tmp_path: Path) -> None:
    """Escape language keywords while preserving case-distinct schema names."""
    source = tmp_path / "schemas.json"
    source.write_text(
        json.dumps(
            {
                "fixture": {
                    "type": "dict",
                    "keys": {
                        "match": {"type": "str"},
                        "override": {"type": "str"},
                        "Vxlan1": {"type": "dict"},
                        "vxlan1": {"type": "dict"},
                        "foo-bar": {"type": "dict"},
                        "foo_bar": {"type": "dict"},
                        "View": {"type": "dict"},
                    },
                }
            }
        ),
        encoding="UTF-8",
    )
    rust = tmp_path / "models.rs"
    pyi = tmp_path / "models_stubs"

    generate_validated_data_models(source, "fixture", rust, pyi, "Fixture", "Fixture")

    rust_source = _combined_rust_source(rust)
    pyi_source = _combined_stub_source(pyi)
    assert re.search(r'#\[data_view\(schema = Str\(\d+\), rename = "match"\)\]', rust_source)
    assert "pub field_match: ::validated_data::Field<&'a str>" in rust_source
    assert re.search(r'#\[data_view\(schema = Str\(\d+\), rename = "override"\)\]', rust_source)
    assert "pub field_override: ::validated_data::Field<&'a str>" in rust_source
    assert "pub Vxlan1: ::validated_data::Field<" in rust_source
    assert "pub vxlan1: ::validated_data::Field<" in rust_source
    assert rust_source.count("pub struct Vxlan1Slot") == 2
    assert rust_source.count("pub struct FooBarSlot") == 2
    assert "pub struct View<'a, Mode>" in rust_source
    assert "def field_match(" in pyi_source
    assert "def field_override(" in pyi_source
    assert "def Vxlan1(" in pyi_source
    assert "def vxlan1(" in pyi_source
    ast.parse(pyi_source)


def test_validation_modes_preserve_reuse_and_python_presence_contracts(tmp_path: Path) -> None:
    """Reuse one Rust model across modes and expose relaxed payloads opaquely in Python."""
    source = tmp_path / "schemas.json"
    source.write_text(
        json.dumps(
            {
                "shared": {
                    "type": "dict",
                    "keys": {
                        "name": {"type": "str", "required": True},
                        "enabled": {"type": "bool", "default": True},
                    },
                },
                "fixture": {
                    "type": "dict",
                    "keys": {
                        "strict": {"$ref": "shared#", "type": "dict"},
                        "patch": {"$ref": "shared#", "type": "dict", "relaxed_validation": True},
                        "numbers": {"type": "list", "items": {"type": "int", "required": True}},
                        "entries": {"type": "list", "primary_key": "name", "items": {"type": "dict", "keys": {"name": {"type": "str"}}}},
                        "foo-bar": {"type": "str"},
                        "foo_bar": {"type": "str"},
                    },
                },
            }
        ),
        encoding="UTF-8",
    )
    rust, pyi = tmp_path / "models.rs", tmp_path / "models_stubs"
    generate_validated_data_models(source, "fixture", rust, pyi, "Fixture", "Fixture", reused_schemas={"shared": ("Shared", "Shared")})
    rust_source = _combined_rust_source(rust)
    pyi_source = _combined_stub_source(pyi)
    assert rust_source.count("pub struct Shared<'a, Mode>") == 1
    assert "pub strict: ::validated_data::Field<super::shared::Shared<'a, Mode>>" in rust_source
    assert "pub patch: ::validated_data::Field<super::shared::Shared<'a, ::validated_data::RelaxedValidated>>" in rust_source
    assert "(::validated_data::RequiredValue<i64, Mode>);" in rust_source
    assert "def strict(self) -> Shared | None:" in pyi_source
    assert "def patch(self) -> OpaqueData | None:" in pyi_source
    assert "def name(self) -> str:" in pyi_source
    assert "def enabled(self) -> bool | None:" in pyi_source
    assert "class EntriesItem(Fixture.EntriesItemBase):" in pyi_source
    assert "class Numbers(_List[int]):" in pyi_source
    root_class = next(node for node in ast.parse(pyi_source).body if isinstance(node, ast.ClassDef) and node.name == "Fixture")
    properties = [node.name for node in root_class.body if isinstance(node, ast.FunctionDef)]
    assert len(properties) == len(set(properties))
    assert sum(name.startswith("foo_bar_slot_") for name in properties) == 2
    ast.parse(pyi_source)


def test_duplicate_key_lists_keep_sequence_api_and_guaranteed_item_keys(tmp_path: Path) -> None:
    """Keep reusable dictionary fields optional while strengthening hybrid-list item access."""
    source = tmp_path / "schemas.json"
    source.write_text(
        json.dumps(
            {
                "shared": {"type": "dict", "keys": {"name": {"type": "str"}}},
                "fixture": {
                    "type": "dict",
                    "keys": {
                        "standalone": {"type": "dict", "$ref": "shared#"},
                        "duplicates": {
                            "type": "list",
                            "primary_key": "name",
                            "allow_duplicate_primary_key": True,
                            "items": {"type": "dict", "$ref": "shared#"},
                        },
                    },
                },
            }
        ),
        encoding="UTF-8",
    )
    rust, pyi = tmp_path / "models.rs", tmp_path / "models_stubs"
    generate_validated_data_models(source, "fixture", rust, pyi, "Fixture", "Fixture", reused_schemas={"shared": ("Shared", "Shared")})
    rust_source = _combined_rust_source(rust)
    assert "data_view(list, primary_key(name), python(" in rust_source
    assert rust_source.count("pub struct Shared<'a, Mode>") == 1
    assert "pub name: ::validated_data::Field<&'a str>" in rust_source
    pyi_source = _combined_stub_source(pyi)
    assert "class Duplicates(_List[Fixture.DuplicatesItem]):" in pyi_source
    assert "class DuplicatesItem(Shared):" in pyi_source
    assert "def name(self) -> str | None:" in pyi_source
    classes = _stub_classes(pyi)
    wrapper = classes["Fixture.DuplicatesItem"]
    name_property = next(node for node in wrapper.body if isinstance(node, ast.FunctionDef) and node.name == "name")
    assert isinstance(name_property.returns, ast.Name)
    assert name_property.returns.id == "str"
    sequence_methods = {node.name for node in classes["Fixture.Duplicates"].body if isinstance(node, ast.FunctionDef)}
    assert sequence_methods == set()
    assert [ast.unparse(base) for base in classes["Fixture.Duplicates"].bases] == [
        "_List[Fixture.DuplicatesItem]",
    ]
    collection_methods = {node.name for node in classes["_Collection"].body if isinstance(node, ast.FunctionDef)}
    assert {"__iter__", "__len__", "__reversed__", "__eq__", "count", "index"} == collection_methods
    assert re.search(r'python\(name = "Fixture.Duplicates", schema = List\(\d+\), item_name = "Fixture.DuplicatesItem"\)', rust_source)
    assert re.search(r'python\(name = "Shared", schema = Dict\(\d+\)\)', rust_source)
    assert "indexed_list" not in rust_source


def test_generate_validated_data_models_uses_language_specific_root_names(tmp_path: Path) -> None:
    """Honor the naming convention selected independently for each generated language."""
    rust = tmp_path / "models.rs"
    pyi = tmp_path / "models_stubs"

    generate_validated_data_models(
        ARTIFACTS / "schemas.json",
        "schema_generation_fixture",
        rust,
        pyi,
        "AvdDesign",
        "AVDDesign",
        root_keys=["accounting"],
    )

    rust_source = _combined_rust_source(rust)
    pyi_source = _combined_stub_source(pyi)
    assert "pub mod avd_design;" in rust_source
    assert "pub struct AvdDesign<'a, Mode>" in rust_source
    assert "class AVDDesign(_Model):" in pyi_source
    assert "class AvdDesign(_Model):" not in pyi_source


def test_generate_validated_data_models_skips_removed_dangling_primary_key(tmp_path: Path) -> None:
    """Keep released-schema tombstones out of the nominal generated API."""
    source = tmp_path / "schemas.json"
    source.write_text(
        json.dumps(
            {
                "fixture": {
                    "type": "dict",
                    "keys": {
                        "historic_values": {
                            "type": "list",
                            "primary_key": "name",
                            "deprecation": {"warning": False, "removed": True},
                        }
                    },
                }
            }
        ),
        encoding="UTF-8",
    )
    rust = tmp_path / "models.rs"
    pyi = tmp_path / "models_stubs"

    generate_validated_data_models(source, "fixture", rust, pyi, "Fixture", "Fixture")

    rust_source = _combined_rust_source(rust)
    pyi_source = _combined_stub_source(pyi)
    assert "primary_key_fields:" not in rust_source
    assert "historic_values" not in rust_source
    assert "historic_values" not in pyi_source


def test_generate_validated_data_models_keeps_dynamic_model_descriptors(tmp_path: Path) -> None:
    """Keep collection typing below data-dependent keys without exposing a named accessor."""
    source = tmp_path / "schemas.json"
    source.write_text(
        json.dumps(
            {
                "fixture": {
                    "type": "dict",
                    "dynamic_keys": {
                        "selectors.names": {
                            "type": "dict",
                            "keys": {"enabled": {"type": "bool"}},
                        }
                    },
                }
            }
        ),
        encoding="UTF-8",
    )
    rust = tmp_path / "models.rs"
    pyi = tmp_path / "models_stubs"

    generate_validated_data_models(source, "fixture", rust, pyi, "Fixture", "Fixture")

    rust_source = _combined_rust_source(rust)
    assert re.search(r'#\[data_view\(schema = Dict\(\d+\), python_name = "selectors", dynamic = "selectors.names"\)\]', rust_source)
    assert "pub selectors_names: ::validated_data::Field<DynamicSlot0<'a, Mode>>" in rust_source
    assert "pub struct DynamicSlot0" in rust_source
    assert re.search(r'python\(name = "Fixture._DynamicKeys.Selectors", schema = Dict\(\d+\)\)', rust_source)
    classes = _stub_classes(pyi)
    assert "Fixture._DynamicKeys.Selectors" in classes
    group = next(node for node in classes["Fixture._DynamicKeys"].body if isinstance(node, ast.FunctionDef))
    assert group.name == "selectors"
    assert ast.unparse(group.returns) == "_DynamicCollection[Fixture._DynamicKeys.Selectors]"
    get_overloads = [node for node in classes["_DynamicCollection"].body if isinstance(node, ast.FunctionDef) and node.name == "get"]
    assert [ast.unparse(node.returns) for node in get_overloads] == ["_DynamicEntry[_T_co] | UndefinedType", "_DynamicEntry[_T_co] | _D"]
    assert not (rust.with_suffix("") / "native.rs").exists()


def test_dynamic_groups_use_schema_names_and_preserve_reused_types(tmp_path: Path) -> None:
    """Group under schema display names, escaping keywords without copying canonical models."""
    source = tmp_path / "schemas.json"
    source.write_text(
        json.dumps(
            {
                "external": {"type": "dict", "display_name": "Class", "keys": {"enabled": {"type": "bool", "default": True}}},
                "fixture": {
                    "type": "dict",
                    "dynamic_keys": {
                        "node_type_keys.key": {"type": "dict", "display_name": "Node types", "keys": {"id": {"type": "int"}}},
                        "other_keys.name": {"type": "dict", "$ref": "external#"},
                    },
                },
            }
        ),
        encoding="UTF-8",
    )
    rust, pyi = tmp_path / "models.rs", tmp_path / "models_stubs"
    generate_validated_data_models(source, "fixture", rust, pyi, "Fixture", "Fixture", reused_schemas={"external": ("External", "External")})
    classes = _stub_classes(pyi)
    groups = {node.name: ast.unparse(node.returns) for node in classes["Fixture._DynamicKeys"].body if isinstance(node, ast.FunctionDef)}
    assert groups == {"node_types": "_DynamicCollection[Fixture._DynamicKeys.NodeTypes]", "field_class": "_DynamicCollection[External]"}
    declarations = _combined_rust_source(rust)
    assert 'python_name = "node_types"' in declarations
    assert 'python_name = "field_class"' in declarations
    assert declarations.count("pub struct External<'a, Mode>") == 1


def test_dynamic_group_name_collisions_are_rejected(tmp_path: Path) -> None:
    """Do not silently shadow selector groups after applying Python naming rules."""
    source = tmp_path / "schemas.json"
    source.write_text(
        json.dumps(
            {
                "fixture": {
                    "type": "dict",
                    "dynamic_keys": {
                        "first.names": {"type": "dict", "display_name": "Same name"},
                        "second.names": {"type": "dict", "display_name": "Same name"},
                    },
                }
            }
        ),
        encoding="UTF-8",
    )
    with pytest.raises(ValueError, match=r"conflicting dynamic key group Fixture\.same_name"):
        generate_validated_data_models(source, "fixture", tmp_path / "models.rs", tmp_path / "models_stubs", "Fixture", "Fixture")
