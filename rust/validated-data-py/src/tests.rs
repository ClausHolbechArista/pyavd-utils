// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Exercise native descriptors and type ownership against observable Python behavior.

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyModule};
use std::sync::Arc;
use validated_data::{
    ArchiveModel as _, DataStore, Field, ModelRegistry, RequiredValue, data_view,
};

use crate::{FieldBinding, ModelBinding, PythonModel as _, Target, install_models, wrap_named};

#[data_view(python(name = "Root"))]
struct Root<'a, Mode> {
    name: RequiredValue<&'a str, Mode>,
    children: Field<Children<'a, Mode>>,
    absent: Field<&'a str>,
    #[data_view(rename = "schema-name")]
    renamed: Field<&'a str>,
    #[data_view(relaxed)]
    opaque: Field<Hidden<'a, validated_data::RelaxedValidated>>,
    #[data_view(dynamic = "selectors.names")]
    dynamic: Field<Hidden<'a, Mode>>,
}

#[data_view(
    indexed_list,
    primary_key(name),
    python(name = "Children", item_name = "KeyedItem")
)]
struct Children<'a, Mode>(Field<Item<'a, Mode>>);

#[data_view(python(name = "Item"))]
struct Item<'a, Mode> {
    name: RequiredValue<&'a str, Mode>,
}

// Rust-only descendants require no PythonModel implementation.
#[data_view]
struct Hidden<'a, Mode> {}

#[data_view(python(name = "ProtocolRoot"))]
struct ProtocolRoot<'a, Mode> {
    numbers: Field<ProtocolNumbers<'a, Mode>>,
    strings: Field<ProtocolStrings<'a, Mode>>,
    dictionaries: Field<ProtocolDictionaries<'a, Mode>>,
    children: Field<Children<'a, Mode>>,
    hybrids: Field<ProtocolHybrids<'a, Mode>>,
    empty: Field<ProtocolDict<'a, Mode>>,
    different_type: Field<OtherProtocolDict<'a, Mode>>,
}

#[data_view(list, python(name = "ProtocolNumbers"))]
struct ProtocolNumbers<'a, Mode>(Field<i64>);

#[data_view(list, python(name = "ProtocolStrings"))]
struct ProtocolStrings<'a, Mode>(Field<&'a str>);

#[data_view(list, python(name = "ProtocolDictionaries"))]
struct ProtocolDictionaries<'a, Mode>(Field<ProtocolDict<'a, Mode>>);

#[data_view(
    list,
    primary_key(name),
    python(name = "ProtocolHybrids", item_name = "HybridItem")
)]
struct ProtocolHybrids<'a, Mode>(Field<Item<'a, Mode>>);

#[data_view(python(name = "ProtocolDict"))]
struct ProtocolDict<'a, Mode> {
    enabled: Field<bool>,
}

#[data_view(python(name = "OtherProtocolDict"))]
struct OtherProtocolDict<'a, Mode> {
    enabled: Field<bool>,
}

fn protocol_store(value: &serde_json::Value) -> PyResult<Arc<DataStore>> {
    let registry = ModelRegistry {
        root_model: ProtocolRoot::<'static, validated_data::Validated>::DESCRIPTOR,
        hash: [9; 32],
    };
    let directory = tempfile::tempdir()?;
    let file = directory.path().join("data.rkyv");
    validated_data::publish(value, [2; 32], &file, registry)
        .map_err(|error| pyo3::exceptions::PyRuntimeError::new_err(error.to_string()))?;
    let store = Arc::new(
        DataStore::from_file(&file, [2; 32], registry)
            .map_err(|error| pyo3::exceptions::PyRuntimeError::new_err(error.to_string()))?,
    );
    store
        .root_as::<ProtocolRoot<'_>>()
        .map_err(|error| pyo3::exceptions::PyRuntimeError::new_err(error.to_string()))?;
    Ok(store)
}

#[test]
#[allow(
    clippy::panic_in_result_fn,
    clippy::too_many_lines,
    reason = "The embedded Python script checks the public collection protocols together."
)]
fn collection_protocols_match_the_typing_contract() -> PyResult<()> {
    let value = serde_json::json!({
        "numbers": [1, 2, 1, 3, null], "strings": ["one", "two", "one"],
        "dictionaries": [{}, {"enabled": false}, {}],
        "children": [{"name": "leaf"}], "hybrids": [{"name": "leaf"}, {"name": "leaf"}],
        "empty": {}, "different_type": {}, "custom": {"opaque": [1, null, true]}
    });
    let first = protocol_store(&value)?;
    let second = protocol_store(&value)?;
    Python::initialize();
    Python::attach(|py| {
        let module = PyModule::new(py, "protocol_fixture")?;
        install_models::<ProtocolRoot<'static>>(
            &module,
            &py.get_type::<PyDict>(),
            &py.None().into_bound(py),
        )?;
        let locals = PyDict::new(py);
        locals.set_item(
            "root",
            wrap_named(&module, "ProtocolRoot", first.root_handle())?,
        )?;
        locals.set_item(
            "other",
            wrap_named(&module, "ProtocolRoot", second.root_handle())?,
        )?;
        py.run(
            c"\
from collections.abc import Sequence
import operator
numbers = root.numbers
assert isinstance(numbers, Sequence)
assert list(numbers) == [1, 2, 1, 3, None]
assert len(numbers) == 5
assert numbers.count(1) == numbers.count(True) == numbers.count(1.0) == 2
assert numbers.count(None) == 1
assert numbers.count(10**100) == 0
assert 2 in numbers and 99 not in numbers
assert numbers.index(1) == 0
assert numbers.index(1, 1) == 2
assert numbers.index(1, -3) == 2
assert numbers.index(1, -10**100) == 0
assert numbers.index(1, stop=-1) == 0
assert root.strings.count('one') == 2
assert root.strings.count('\\ud800') == 0
assert root.strings.index('one', 1) == 2
assert root.strings.count(chr(0xD800)) == 0
assert numbers == other.numbers and numbers != root.strings
for bounds in [(1, 2), (4, None), (10**100, None), (3, 1)]:
    try:
        numbers.index(1, *bounds)
    except ValueError:
        pass
    else:
        raise AssertionError('bounded search must fail')
class Index:
    def __index__(self):
        return 1
assert numbers.index(1, Index()) == 2
try:
    numbers.index(1, 1.5)
except TypeError:
    pass
else:
    raise AssertionError('bounds require integer indices')
try:
    numbers[10**100]
except IndexError:
    pass
else:
    raise AssertionError('large out of range index')
class EqualToThree:
    def __eq__(self, other):
        return other == 3
assert numbers.count(EqualToThree()) == 1
class Raises:
    def __eq__(self, other):
        raise RuntimeError('comparison failure')
try:
    numbers.count(Raises())
except RuntimeError as error:
    assert str(error) == 'comparison failure'
else:
    raise AssertionError('comparison exceptions must propagate')
assert not root.empty
assert root.dictionaries[1]  # a supplied false value still makes a model nonempty
assert root.empty == other.empty
assert root.empty != root.different_type
assert root.empty != {} and root.empty != None
assert root == other
assert root.dictionaries.count(root.empty) == 2
assert root.dictionaries.index(root.empty, 1) == 2
for view in [root, root.empty, numbers, root.children['leaf']]:
    try:
        hash(view)
    except TypeError:
        pass
    else:
        raise AssertionError('structurally equal views cannot use identity hashes')
assert not isinstance(root.children, Sequence)
assert root.children['leaf'].name == 'leaf'
assert 'leaf' in root.children
assert root.children.count(root.children['leaf']) == 1
assert root.children.index(root.children['leaf']) == 0
assert isinstance(root.hybrids, Sequence)
assert root.hybrids.count(root.hybrids[0]) == 2
reverse = reversed(numbers)
assert operator.length_hint(reverse) == 5
assert next(reverse) is None
assert operator.length_hint(reverse) == 4
del root, other, numbers
assert list(reverse) == [3, 1, 2, 1]
assert list(reverse) == []
",
            None,
            Some(&locals),
        )
    })
}

#[test]
#[allow(
    clippy::panic_in_result_fn,
    reason = "Assertions express the expected Python behavior of these integration tests."
)]
fn native_descriptors_and_iterators_retain_the_archive() -> PyResult<()> {
    let registry = ModelRegistry {
        root_model: Root::<'static, validated_data::Validated>::DESCRIPTOR,
        hash: [1; 32],
    };
    let directory = tempfile::tempdir()?;
    let file = directory.path().join("data.rkyv");
    validated_data::publish(
        &serde_json::json!({"name":"root", "children":[{"name":"leaf"}], "schema-name":"renamed"}),
        [2; 32],
        &file,
        registry,
    )
    .map_err(|error| pyo3::exceptions::PyRuntimeError::new_err(error.to_string()))?;
    let store = Arc::new(
        DataStore::from_file(&file, [2; 32], registry)
            .map_err(|error| pyo3::exceptions::PyRuntimeError::new_err(error.to_string()))?,
    );
    let root = store
        .root_as::<Root<'_, validated_data::Validated>>()
        .map_err(|error| pyo3::exceptions::PyRuntimeError::new_err(error.to_string()))?;
    assert_eq!(root.name().get(), "root");
    assert!(matches!(root.children(), Field::Value(_)));
    assert!(matches!(root.absent(), Field::Unset));
    Python::initialize();
    Python::attach(|py| {
        let module = PyModule::new(py, "native_fixture")?;
        let undefined = py.import("builtins")?.getattr("object")?.call0()?;
        install_models::<Root<'static>>(&module, &py.get_type::<PyDict>(), &undefined)?;
        let wrapper = wrap_named(&module, "Root", store.root_handle())?;
        drop(store);
        let locals = PyDict::new(py);
        locals.set_item("root", wrapper)?;
        locals.set_item("undefined", undefined)?;
        locals.set_item("models", module)?;
        py.run(
            c"\
assert root.name == 'root'
assert root.absent is undefined
assert root.renamed == 'renamed'
assert not hasattr(models, 'Hidden')
assert not hasattr(models.Root, 'dynamic')
assert hasattr(models.Root.name, '__get__')
children = root.children
assert children.get('absent') is undefined
assert children.get('absent', None) is None
assert isinstance(children['leaf'], models.Item)
assert isinstance(children['leaf'], models.KeyedItem)
iterator = children.items()
del root, children
assert [(key, item.name) for key, item in iterator] == [('leaf', 'leaf')]
",
            None,
            Some(&locals),
        )
    })
}

#[test]
#[allow(
    clippy::panic_in_result_fn,
    reason = "The assertion expresses the expected Python garbage-collection behavior."
)]
fn cached_child_types_participate_in_python_cycle_collection() -> PyResult<()> {
    Python::initialize();
    Python::attach(|py| {
        let module = PyModule::new(py, "collectible_fixture")?;
        let undefined = py.None().into_bound(py);
        crate::catalog::install_root(
            &module,
            node_binding(),
            &py.get_type::<PyDict>(),
            &undefined,
        )?;
        let reference = py
            .import("weakref")?
            .getattr("ref")?
            .call1((module.getattr("Node")?,))?;
        drop(module);
        py.import("gc")?.getattr("collect")?.call0()?;
        assert!(reference.call0()?.is_none());
        Ok(())
    })
}

fn node_binding() -> &'static ModelBinding {
    const FIELD: validated_data::FieldDescriptor = validated_data::FieldDescriptor {
        id: 0,
        relation: validated_data::FieldRelation::Key("child"),
        target_model: None,
        scalar_type: None,
        required: false,
        relaxed: false,
    };
    const NODE: ModelBinding = ModelBinding {
        name: "Node",
        descriptor: &validated_data::ModelDescriptor {
            identity: "gc_fixture::Node",
            kind: validated_data::ModelKind::Dict,
            fields: &[FIELD],
            primary_key_fields: &[],
            indexed: false,
        },
        fields: &[FieldBinding {
            name: "child",
            descriptor: &FIELD,
            target: Target::Model(node_binding),
        }],
        item: None,
        keyed_item_name: None,
    };
    &NODE
}

#[test]
fn binding_fields_reference_the_original_rust_descriptors() {
    let root = Root::<'static, validated_data::Validated>::binding();
    let renamed_field = root
        .fields
        .iter()
        .find(|field| field.name == "renamed")
        .expect("renamed property");
    assert_eq!(
        renamed_field.descriptor,
        &Root::<'static, validated_data::Validated>::DESCRIPTOR.fields[3]
    );
    assert_eq!(
        renamed_field.descriptor.relation,
        validated_data::FieldRelation::Key("schema-name")
    );
    assert!(
        root.fields
            .iter()
            .any(|candidate| matches!(candidate.target, Target::Opaque))
    );
    assert!(
        !root
            .fields
            .iter()
            .any(|candidate| candidate.name == "dynamic")
    );
    assert_eq!(
        Children::<'static, validated_data::Validated>::binding()
            .descriptor
            .primary_key_fields,
        Children::<'static, validated_data::Validated>::DESCRIPTOR.primary_key_fields
    );
}
