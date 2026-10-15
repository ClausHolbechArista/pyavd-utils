// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Reject declarations whose field contracts or list metadata cannot be interpreted.

use super::expand;
use quote::quote;
use syn::parse_quote;

#[test]
fn indexed_lists_require_named_keys() {
    let item = parse_quote!(
        struct Items<'a, Mode>(Field<Item<'a, Mode>>);
    );
    assert!(expand(quote!(indexed_list), &item).is_err());
    assert!(expand(quote!(primary_key(name)), &item).is_err());
    assert!(expand(quote!(indexed_list, primary_key(name)), &item).is_ok());
}

#[test]
fn declarations_require_presence_wrappers() {
    let dictionary = parse_quote!(
        struct Item<'a, Mode> {
            name: &'a str,
        }
    );
    assert!(expand(quote!(), &dictionary).is_err());
    let list = parse_quote!(
        struct Names<'a, Mode>(&'a str);
    );
    assert!(expand(quote!(list), &list).is_err());
}

#[test]
fn unknown_field_options_are_not_silently_ignored() {
    let dictionary = parse_quote!(
        struct Item<'a, Mode> {
            #[data_view(unknown)]
            name: Field<&'a str>,
        }
    );
    assert!(expand(quote!(), &dictionary).is_err());
    let list = parse_quote!(
        struct Items<'a, Mode>(#[data_view(rename = "name")] Field<Item<'a, Mode>>);
    );
    assert!(expand(quote!(list), &list).is_err());
}

#[test]
fn scalar_items_cannot_be_indexed_dictionary_items() {
    let list = parse_quote!(
        struct Names<'a, Mode>(Field<&'a str>);
    );
    assert!(expand(quote!(indexed_list, primary_key(name)), &list).is_err());
}

#[test]
fn duplicate_key_lists_expose_positions_but_not_unique_lookup() {
    let list = parse_quote!(
        struct Items<'a, Mode>(Field<Item<'a, Mode>>);
    );
    let tokens = expand(quote!(list, primary_key(name)), &list)
        .expect("valid hybrid declaration")
        .to_string();
    assert!(tokens.contains("pub fn get"));
    assert!(tokens.contains("Guaranteed"));
    assert!(!tokens.contains("get_by_primary_key"));
    let raw = parse_quote!(
        struct Items<'a, Mode>;
    );
    assert!(expand(quote!(list, primary_key(name)), &raw).is_err());
}

#[test]
fn python_metadata_is_opt_in_and_reuses_field_descriptors() {
    let dictionary = parse_quote!(
        struct Item<'a, Mode> {
            #[data_view(rename = "type")]
            field_type: Field<&'a str>,
        }
    );
    let plain = expand(quote!(), &dictionary)
        .expect("Rust-only view")
        .to_string();
    assert!(!plain.contains("validated_data_py"));
    let bound = expand(quote!(python(name = "Item")), &dictionary)
        .expect("Python-exposed view")
        .to_string();
    assert!(bound.contains("PythonModel"));
    assert!(bound.contains("DESCRIPTOR . fields"));
    assert!(bound.contains("field_type"));
    assert!(expand(quote!(python(item_name = "Item")), &dictionary).is_err());
    assert!(
        expand(
            quote!(python(name = "Item", item_name = "KeyedItem")),
            &dictionary
        )
        .is_err()
    );
    let list = parse_quote!(
        struct Items<'a, Mode>(Field<Item<'a, Mode>>);
    );
    assert!(
        expand(
            quote!(list, primary_key(name), python(name = "Items")),
            &list
        )
        .is_err()
    );
    assert!(
        expand(
            quote!(
                list,
                primary_key(name),
                python(name = "Items", item_name = "KeyedItem")
            ),
            &list
        )
        .is_ok()
    );
}
