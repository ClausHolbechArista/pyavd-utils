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

use crate::{Target, install_models, python_data_views, wrap_named};

#[data_view]
struct Root<'a, Mode> {
    name: RequiredValue<&'a str, Mode>,
    children: Field<Children<'a, Mode>>,
    absent: Field<&'a str>,
}

#[data_view(indexed_list, primary_key(name))]
struct Children<'a, Mode>(Field<Item<'a, Mode>>);

#[data_view]
struct Item<'a, Mode> {
    name: RequiredValue<&'a str, Mode>,
}

python_data_views! {
    BINDINGS {
        "Root" => dict { "name": 0 => Target::Scalar, "children": 1 => Target::Model("Children"), "absent": 2 => Target::Scalar };
        "Children" => indexed(Target::Model("KeyedItem"), [0]);
        "Item" => dict { "name": 0 => Target::Scalar };
        "KeyedItem" => alias("Item");
    }
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
        &serde_json::json!({"name":"root", "children":[{"name":"leaf"}]}),
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
        install_models(&module, BINDINGS, &py.get_type::<PyDict>(), &undefined)?;
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
assert hasattr(models.Root.name, '__get__')
children = root.children
assert children.get('absent') is undefined
assert children.get('absent', None) is None
assert isinstance(children['leaf'], models.Item)
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
        let models = [crate::ModelBinding {
            name: "Node",
            shape: crate::ModelShape::Dict(&[crate::FieldBinding {
                name: "child",
                slot: 0,
                target: Target::Model("Node"),
            }]),
        }];
        install_models(&module, &models, &py.get_type::<PyDict>(), &undefined)?;
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
