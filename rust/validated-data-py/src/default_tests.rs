// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Observable default/presence semantics and grouped dynamic access over one small schema.

use crate::{install_models, wrap_with_schema};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyModule};
use std::sync::Arc;
use validated_data::{ArchiveModel as _, DataStore, Field, ModelRegistry, data_view};

// These IDs describe the paired source below, just as generated declarations describe the
// compiled schema they ship with. The test exercises metadata-driven access, not test hooks.
#[data_view(python(name = "Fixture", schema = Dict(3)))]
struct Fixture<'a, Mode> {
    #[data_view(schema = Str(0))]
    label: Field<&'a str>,
    #[data_view(schema = Str(0))]
    nullable: Field<&'a str>,
    #[data_view(schema = Bool(0))]
    enabled: Field<bool>,
    #[data_view(schema = Int(0))]
    count: Field<i64>,
    #[data_view(schema = List(0))]
    flags: Field<Flags<'a, Mode>>,
    #[data_view(schema = Dict(0))]
    settings: Field<Settings<'a, Mode>>,
    #[data_view(schema = Dict(1))]
    object: Field<Settings<'a, Mode>>,
    #[data_view(schema = List(1))]
    selectors: Field<Selectors<'a, Mode>>,
    #[data_view(schema = Str(1))]
    no_default: Field<&'a str>,
    #[data_view(schema = List(2))]
    entries: Field<Entries<'a, Mode>>,
    #[data_view(schema = List(3))]
    other_selectors: Field<Selectors<'a, Mode>>,
    #[data_view(dynamic = "selectors", python_name = "selectors", schema = Dict(0))]
    selected: Field<Settings<'a, Mode>>,
    #[data_view(dynamic = "other_selectors", python_name = "other", schema = Dict(0))]
    other: Field<Settings<'a, Mode>>,
}

#[data_view(indexed_list, primary_key(name), python(name = "Fixture.Entries", schema = List(2), item_name = "Fixture.EntriesItem"))]
struct Entries<'a, Mode>(Field<Entry<'a, Mode>>);

#[data_view(python(name = "Fixture.EntriesItemBase", schema = Dict(2)))]
struct Entry<'a, Mode> {
    #[data_view(schema = Str(1))]
    name: Field<&'a str>,
    #[data_view(schema = Str(0))]
    label: Field<&'a str>,
}

#[data_view(list, python(name = "Fixture.Flags", schema = List(0)))]
struct Flags<'a, Mode>(Field<bool>);

#[data_view(list, python(name = "Fixture.Selectors", schema = List(1)))]
struct Selectors<'a, Mode>(Field<&'a str>);

#[data_view(python(name = "Fixture.Settings", schema = Dict(0)))]
struct Settings<'a, Mode> {
    #[data_view(schema = Str(0))]
    label: Field<&'a str>,
}

fn schema() -> PyResult<Arc<avdschema::Store>> {
    avdschema::Store::from_json(r#"{"fixture":{"type":"dict","keys":{
        "label":{"type":"str","default":"schema"},
        "nullable":{"type":"str","default":"schema"},
        "enabled":{"type":"bool","default":true},
        "count":{"type":"int","default":7},
        "flags":{"type":"list","items":{"type":"bool"},"default":[true,false]},
        "settings":{"type":"dict","keys":{"label":{"type":"str","default":"schema"}}},
        "object":{"type":"dict","keys":{"label":{"type":"str","default":"schema"}},"default":{"label":"from-object","custom":18446744073709551615}},
        "selectors":{"type":"list","items":{"type":"str"},"default":["fabric","null_payload","absent"]},
        "no_default":{"type":"str"},
        "entries":{"type":"list","primary_key":"name","items":{"type":"dict","keys":{"name":{"type":"str"},"label":{"type":"str","default":"schema"}}},"default":[{"name":"first"},{"name":"second","label":"supplied"}]},
        "other_selectors":{"type":"list","items":{"type":"str"}}
    },"dynamic_keys":{
        "selectors":{"type":"dict","keys":{"label":{"type":"str","default":"schema"}}},
        "other_selectors":{"type":"dict","keys":{"label":{"type":"str","default":"schema"}}}
    }}}"#)
        .map(Arc::new).map_err(|error| pyo3::exceptions::PyRuntimeError::new_err(error.to_string()))
}

fn data(value: &serde_json::Value, schemas: &avdschema::Store) -> PyResult<Arc<DataStore>> {
    let registry = ModelRegistry {
        root_model: Fixture::<'static, validated_data::Validated>::DESCRIPTOR,
        hash: [31; 32],
    };
    let directory = tempfile::tempdir()?;
    let path = directory.path().join("host.rkyv");
    validated_data::publish(value, schemas.archive_hash(), &path, registry)
        .map_err(|error| pyo3::exceptions::PyRuntimeError::new_err(error.to_string()))?;
    let store = Arc::new(
        DataStore::from_file(&path, schemas.archive_hash(), registry)
            .map_err(|error| pyo3::exceptions::PyRuntimeError::new_err(error.to_string()))?,
    );
    store
        .root_as::<Fixture<'_>>()
        .map_err(|error| pyo3::exceptions::PyRuntimeError::new_err(error.to_string()))?;
    Ok(store)
}

#[test]
#[allow(
    clippy::too_many_lines,
    reason = "One end-to-end Python scenario checks default access together with archive lifetime."
)]
fn defaults_preserve_presence_nulls_dynamic_payloads_and_archive_ownership() {
    let schemas = schema().expect("fixture schema");
    let input = serde_json::json!({"label":"schema", "nullable":null, "enabled":false, "count":0, "flags":[], "fabric":{}, "null_payload":null, "custom":{"keep":[1,null,true]}});
    let store = data(&input, &schemas).expect("fixture input");
    let fallback =
        data(&serde_json::json!({"fabric":{}, "settings":{}}), &schemas).expect("fallback input");
    let suppressed = data(&serde_json::json!({"selectors":[], "fabric":{}}), &schemas)
        .expect("suppressed selector input");
    let null_selector = data(
        &serde_json::json!({"selectors":null, "fabric":{}}),
        &schemas,
    )
    .expect("null selector input");
    let snapshot = store.root_handle().to_json();
    let unsigned = data(
        &serde_json::json!({"object":{"label":"from-object","custom":u64::MAX}}),
        &schemas,
    )
    .expect("unsigned input");
    let overlap = data(
        &serde_json::json!({"selectors":["fabric", "object"], "other_selectors":["fabric"], "fabric":{}, "object":{}}),
        &schemas,
    )
    .expect("overlapping selector input");
    Python::initialize();
    Python::attach(|py| -> PyResult<()> {
        let module = PyModule::new(py, "defaults_fixture")?;
        let undefined = py.import("builtins")?.getattr("object")?.call0()?;
        install_models::<Fixture<'static>>(&module, &py.get_type::<PyDict>(), &undefined)?;
        let locals = PyDict::new(py);
        locals.set_item("undefined", &undefined)?;
        locals.set_item(
            "unsigned",
            wrap_with_schema(&module, "Fixture", unsigned.root_handle(), schemas.clone())?,
        )?;
        locals.set_item(
            "overlap",
            wrap_with_schema(&module, "Fixture", overlap.root_handle(), schemas.clone())?,
        )?;
        locals.set_item(
            "root",
            wrap_with_schema(&module, "Fixture", store.root_handle(), schemas.clone())?,
        )?;
        locals.set_item(
            "fallback",
            wrap_with_schema(&module, "Fixture", fallback.root_handle(), schemas.clone())?,
        )?;
        locals.set_item(
            "suppressed",
            wrap_with_schema(
                &module,
                "Fixture",
                suppressed.root_handle(),
                schemas.clone(),
            )?,
        )?;
        locals.set_item(
            "null_selector",
            wrap_with_schema(
                &module,
                "Fixture",
                null_selector.root_handle(),
                schemas.clone(),
            )?,
        )?;
        py.run(c"\
assert root.label == 'schema' and root._is_defined('label') and not root._is_default('label')
assert root.nullable is None and not root._is_defined('nullable') and not root._is_default('nullable')
assert root.enabled is False and root._is_defined('enabled')
assert root.count == 0 and root._is_defined('count')
assert len(root.flags) == 0 and root._is_defined('flags') and not root._is_default('flags')
assert fallback.label == 'schema' and not fallback._is_defined('label') and fallback._is_default('label')
assert list(fallback.flags) == [True, False] and fallback._is_default('flags')
assert not root.settings and root.settings.label == 'schema'
assert not root.settings._is_defined('label') and root.settings._is_default('label')
assert root.object.label == 'from-object' and not root.object._is_defined('label') and root.object._is_default('label')
assert root._is_default('settings') and root._is_default('object')
assert root.no_default is None and not root._is_defined('no_default') and not root._is_default('no_default')
assert fallback._is_defined('settings') and not fallback._is_default('settings')
assert fallback.settings.label == 'schema' and fallback.settings._is_default('label')
assert root.entries['first'].name == 'first' and root.entries['first'].label == 'schema'
assert root.entries['second'].label == 'supplied'
assert root.entries.get('missing', None) is None
assert list(root.entries.keys()) == ['first', 'second']
assert root.entries == fallback.entries and root.flags != fallback.flags
assert root.object == unsigned.object
assert root._is_default('entries')
for helper in (root._is_default, root._is_defined):
    try: helper('typo')
    except AttributeError: pass
    else: raise AssertionError('unknown fields must fail')
group = root._dynamic_keys.selectors
assert list(group.keys()) == ['fabric'] and 'fabric' in group and len(group) == 1
assert group['fabric'].key == 'fabric' and group['fabric'].value.label == 'schema'
assert group.get('missing') is undefined
assert group.get('missing', None) is None and group.get('missing', default=None) is None
assert [(key, entry.key) for key, entry in group.items()] == [('fabric', 'fabric')]
assert [entry.key for entry in group] == ['fabric']
assert not suppressed._dynamic_keys.selectors and not null_selector._dynamic_keys.selectors
assert not overlap._dynamic_keys.selectors
assert [entry.key for entry in overlap._dynamic_keys.other] == ['fabric']
default_iterator = iter(fallback.flags)
dynamic_iterator = iter(group)
default_object = root.object
del root, fallback, suppressed, null_selector, overlap, unsigned, group
", None, Some(&locals))?;
        assert_eq!(store.root_handle().to_json(), snapshot);
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&snapshot).expect("stored JSON"),
            input
        );
        drop(store);
        drop(fallback);
        drop(suppressed);
        drop(null_selector);
        drop(overlap);
        drop(unsigned);
        drop(schemas);
        py.run(
            c"\
assert list(default_iterator) == [True, False]
assert next(dynamic_iterator).value.label == 'schema'
assert default_object.label == 'from-object'
",
            None,
            Some(&locals),
        )?;
        Ok(())
    })
    .expect("Python default access scenario");
}
