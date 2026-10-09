// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Exercise generated types against published and reopened archive data.

use super::*;

#[data_view]
struct Root<'a, Mode> {
    name: RequiredValue<&'a str, Mode>,
    enabled: Field<bool>,
    #[data_view(relaxed)]
    patch: RequiredValue<Payload<'a, RelaxedValidated>, Mode>,
    strict: Field<Payload<'a, Mode>>,
    numbers: Field<Numbers<'a, Mode>>,
    nullable_numbers: Field<NullableNumbers<'a, Mode>>,
    patches: Field<Patches<'a, Mode>>,
    mode: Field<names::Mode<'a, Mode>>,
    hybrids: Field<HybridEntries<'a, Mode>>,
}

#[data_view(list, primary_key(name))]
struct HybridEntries<'a, Mode>(Field<Item<'a, Mode>>);

mod names {
    use super::*;

    #[data_view]
    pub struct Mode<'a, Mode> {
        pub label: RequiredValue<&'a str, Mode>,
    }
}

#[data_view(list)]
struct Patches<'a, Mode>(#[data_view(relaxed)] RequiredValue<Payload<'a, RelaxedValidated>, Mode>);

#[data_view(list)]
struct Numbers<'a, Mode>(RequiredValue<i64, Mode>);

#[data_view(list)]
struct NullableNumbers<'a, Mode>(Field<i64>);

#[data_view]
struct Payload<'a, Mode> {
    id: RequiredValue<i64, Mode>,
    entries: Field<Entries<'a, Mode>>,
}

#[data_view(indexed_list, primary_key(name))]
struct Entries<'a, Mode>(Field<Item<'a, Mode>>);

#[data_view]
struct Item<'a, Mode> {
    #[data_view(rename = "enabled")]
    active: Field<bool>,
    name: Field<&'a str>,
    id: RequiredValue<i64, Mode>,
}

fn registry() -> ModelRegistry {
    ModelRegistry {
        root_model: Root::<'static, Validated>::DESCRIPTOR,
        hash: [3; 32],
    }
}

fn open(value: &serde_json::Value) -> DataStore {
    let directory = tempfile::tempdir().expect("temporary directory");
    let destination = directory.path().join("host.rkyv");
    publish(value, [1; 32], &destination, registry()).expect("publish fixture");
    DataStore::from_file(&destination, [1; 32], registry()).expect("open compatible fixture")
}

#[test]
fn required_fields_are_infallible_and_relaxed_children_keep_their_mode() {
    let store = open(&serde_json::json!({
        "name": "leaf1", "enabled": null,
        "patch": {"id": null, "entries": [{"name":"Ethernet1","enabled":true}]}
    }));
    let root = store
        .root_as::<Root<'_, Validated>>()
        .expect("checked strict root");
    let name: Guaranteed<&str> = root.name();
    assert_eq!(name.get(), "leaf1");
    assert_eq!(root.enabled(), Field::Null);
    assert!(matches!(root.strict(), Field::Unset));
    let payload: Payload<'_, RelaxedValidated> = root.patch().get();
    assert_eq!(payload.id(), Field::Null);
    let Field::Value(entries) = payload.entries() else {
        panic!("entries should be present")
    };
    let item = entries
        .get_by_primary_key(&[PrimaryKeyValue::Str("Ethernet1")])
        .expect("indexed item");
    let key: Guaranteed<&str> = item.name();
    assert_eq!(key.get(), "Ethernet1");
    assert_eq!(item.active(), Field::Value(true));
    assert_eq!(item.id(), Field::Unset);
    let by_position = entries.get(0).expect("first indexed item");
    assert_eq!(by_position.name().get(), "Ethernet1");
    assert!(entries.get(1).is_none());
    assert_eq!(
        Entries::<'static, Validated>::DESCRIPTOR.primary_key_fields,
        &[1]
    );
}

#[test]
fn modes_reuse_one_descriptor_but_do_not_promote_invalid_required_fields() {
    assert_eq!(
        Payload::<'static, Validated>::DESCRIPTOR,
        Payload::<'static, RelaxedValidated>::DESCRIPTOR
    );
    for value in [
        serde_json::json!({"patch":{}}),
        serde_json::json!({"name":null,"patch":{}}),
    ] {
        let store = open(&value);
        assert!(store.root_as::<Root<'_, Validated>>().is_err());
        assert!(store.root_as::<Root<'_, RelaxedValidated>>().is_ok());
    }
    let store = open(&serde_json::json!({"name":"leaf1","patch":{},"strict":{}}));
    assert!(store.root_as::<Root<'_, Validated>>().is_err());
    let relaxed = store
        .root_as::<Root<'_, RelaxedValidated>>()
        .expect("relaxed root");
    let name: Field<&str> = relaxed.name();
    assert_eq!(name, Field::Value("leaf1"));
}

#[test]
fn relaxed_modes_still_check_types_and_indexed_primary_keys() {
    let store = open(&serde_json::json!({"name":"leaf1","patch":{"id":"wrong"}}));
    assert!(store.root_as::<Root<'_, RelaxedValidated>>().is_err());
    for value in [
        serde_json::json!({"name":"leaf1","patch":{"entries":[{"name":null}]}}),
        serde_json::json!({"name":"leaf1","patch":{"entries":[{}]}}),
    ] {
        // Indexed-list structure is rejected before publishing an archive.
        let directory = tempfile::tempdir().expect("temporary directory");
        let destination = directory.path().join("host.rkyv");
        assert!(matches!(
            publish(&value, [1; 32], &destination, registry()),
            Err(ArchiveError::Build(_))
        ));
        assert!(!destination.exists());
    }
}

#[test]
fn opaque_payload_serialization_preserves_null_and_unknown_keys() {
    let store = std::sync::Arc::new(open(
        &serde_json::json!({"name":"leaf1","patch":{"id":null,"extra":{"items":[1,false,null]}}}),
    ));
    let handle = store
        .root_handle()
        .as_dict()
        .expect("root")
        .field(2)
        .expect("patch");
    drop(store);
    assert!(!handle.is_empty());
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&handle.to_json()).expect("JSON"),
        serde_json::json!({"id":null,"extra":{"items":[1,false,null]}})
    );
}

#[test]
fn list_item_requiredness_is_independent_of_list_presence() {
    let store = open(
        &serde_json::json!({"name":"leaf1","patch":{},"numbers":[42],"nullable_numbers":[null,7]}),
    );
    let root = store.root_as::<Root<'_>>().expect("strict root");
    let Field::Value(numbers) = root.numbers() else {
        panic!("numbers should be present")
    };
    let number: Guaranteed<i64> = numbers.get(0).expect("first number");
    assert_eq!(number.get(), 42);
    assert!(numbers.get(1).is_none());
    let Field::Value(nullable) = root.nullable_numbers() else {
        panic!("nullable numbers should be present")
    };
    assert_eq!(nullable.get(0), Some(Field::Null));
    assert_eq!(nullable.get(1), Some(Field::Value(7)));

    let relaxed_store = open(&serde_json::json!({"name":"leaf1","patch":{},"numbers":[null]}));
    assert!(relaxed_store.root_as::<Root<'_>>().is_err());
    let relaxed_root = relaxed_store
        .root_as::<Root<'_, RelaxedValidated>>()
        .expect("relaxed root");
    let Field::Value(relaxed_numbers) = relaxed_root.numbers() else {
        panic!("numbers should be present")
    };
    let relaxed_number: Field<i64> = relaxed_numbers.get(0).expect("first number");
    assert_eq!(relaxed_number, Field::Null);
}

#[test]
fn list_item_relationship_can_enter_relaxed_validation() {
    let store = open(&serde_json::json!({"name":"leaf1","patch":{},"patches":[{"id":null}]}));
    let root = store.root_as::<Root<'_>>().expect("strict root");
    let Field::Value(patches) = root.patches() else {
        panic!("patches should be present")
    };
    let patch: Guaranteed<Payload<'_, RelaxedValidated>> = patches.get(0).expect("first patch");
    assert_eq!(patch.get().id(), Field::Null);
}

#[test]
fn relaxed_boundary_does_not_relax_its_parent_field_presence() {
    for value in [
        serde_json::json!({"name":"leaf1"}),
        serde_json::json!({"name":"leaf1","patch":null}),
    ] {
        let store = open(&value);
        assert!(store.root_as::<Root<'_, Validated>>().is_err());
        let root = store
            .root_as::<Root<'_, RelaxedValidated>>()
            .expect("relaxed root");
        assert!(matches!(root.patch(), Field::Unset | Field::Null));
    }
}

#[test]
fn nominal_mode_type_does_not_shadow_the_validation_parameter() {
    let store = open(&serde_json::json!({"name":"leaf1","patch":{},"mode":{"label":"active"}}));
    let root = store.root_as::<Root<'_>>().expect("strict root");
    let Field::Value(mode) = root.mode() else {
        panic!("mode should be present")
    };
    let label: Guaranteed<&str> = mode.label();
    assert_eq!(label.get(), "active");
}

#[test]
#[allow(
    clippy::assertions_on_constants,
    reason = "The descriptor flag is a generated compile-time contract of duplicate-key lists."
)]
fn duplicate_key_lists_guarantee_keys_without_unique_lookup() {
    let store = open(&serde_json::json!({"name":"leaf1","patch":{},"hybrids":[
        {"name":"same","id":1}, {"name":"same","id":2}
    ]}));
    let root = store.root_as::<Root<'_>>().expect("strict root");
    let Field::Value(hybrids) = root.hybrids() else {
        panic!("hybrids should be present")
    };
    assert_eq!(hybrids.len(), 2);
    let first = hybrids.get(0).expect("first duplicate");
    let second = hybrids.get(1).expect("second duplicate");
    let name: Guaranteed<&str> = first.name();
    assert_eq!(name.get(), "same");
    assert_eq!(second.name().get(), "same");
    assert_eq!(first.id().get(), 1);
    assert_eq!(second.id().get(), 2);
    let base: Item<'_, Validated> = *first;
    assert_eq!(base.name(), Field::Value("same"));
    assert!(!HybridEntries::<'static, Validated>::DESCRIPTOR.indexed);
    let raw = store
        .root()
        .as_dict()
        .expect("root")
        .key("hybrids")
        .expect("hybrids")
        .as_list()
        .expect("list");
    assert!(
        raw.get_by_primary_key(&[PrimaryKeyValue::Str("same")])
            .is_none()
    );

    let relaxed_store = open(
        &serde_json::json!({"name":"leaf1","patch":{},"hybrids":[{"name":"same"},{"name":"same"}]}),
    );
    assert!(relaxed_store.root_as::<Root<'_>>().is_err());
    let relaxed_root = relaxed_store
        .root_as::<Root<'_, RelaxedValidated>>()
        .expect("relaxed root");
    let Field::Value(relaxed_hybrids) = relaxed_root.hybrids() else {
        panic!("hybrids should be present")
    };
    let relaxed_first = relaxed_hybrids.get(0).expect("first duplicate");
    let relaxed_name: Guaranteed<&str> = relaxed_first.name();
    assert_eq!(relaxed_name.get(), "same");
    assert_eq!(relaxed_first.id(), Field::Unset);
}

#[test]
fn duplicate_key_lists_reject_unset_or_null_primary_keys() {
    let directory = tempfile::tempdir().expect("temporary directory");
    for value in [
        serde_json::json!({"hybrids":[{}]}),
        serde_json::json!({"hybrids":[{"name":null}]}),
    ] {
        let destination = directory.path().join("host.rkyv");
        assert!(matches!(
            publish(&value, [1; 32], &destination, registry()),
            Err(ArchiveError::Build(_))
        ));
        assert!(!destination.exists());
    }
}
