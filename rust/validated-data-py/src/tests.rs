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
