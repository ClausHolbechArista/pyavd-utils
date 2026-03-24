// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

#![allow(clippy::indexing_slicing, reason = "panics are acceptable in tests")]
#![allow(
    clippy::expect_used,
    reason = "expect() in tests provides precise failure messages for invariants"
)]
#![allow(
    clippy::panic,
    reason = "panic is acceptable for structural mismatches in tests"
)]

use std::borrow::Cow;

#[cfg(feature = "serde")]
use std::collections::BTreeMap;

use crate::{ErrorKind, Integer, Node, Value, parse};

#[test]
fn parse_string_and_number_spans() {
    let (parsed_node_opt, errors) = parse(r#"{"name":"Alice","age":42}"#);
    assert!(errors.is_empty());
    let parsed_node = parsed_node_opt.expect("expected parsed node");
    let Value::Mapping(pairs) = parsed_node.value else {
        panic!("expected mapping");
    };
    assert_eq!(pairs.len(), 2);
    assert_eq!(pairs[0].0.span.start_usize(), 1);
    assert_eq!(pairs[0].1.span.start_usize(), 8);
    assert_eq!(pairs[1].1.span.start_usize(), 22);
}

#[test]
fn parse_recovery_missing_comma_in_array() {
    let (node, errors) = parse("[1 2, 3]");
    assert!(
        errors
            .iter()
            .any(|error| error.kind == ErrorKind::MissingComma)
    );
    let Some(Node {
        value: Value::Sequence(items),
        ..
    }) = node
    else {
        panic!("expected recovered sequence")
    };
    assert_eq!(items.len(), 3);
}

#[test]
fn parse_recovery_missing_colon_in_object() {
    let (node, errors) = parse(r#"{"a" 1, "b": 2}"#);
    assert!(
        errors
            .iter()
            .any(|error| error.kind == ErrorKind::MissingColon)
    );
    let Some(Node {
        value: Value::Mapping(pairs),
        ..
    }) = node
    else {
        panic!("expected recovered mapping")
    };
    assert_eq!(pairs.len(), 2);
}

#[test]
fn parse_unterminated_string_reports_error() {
    let (node, errors) = parse(r#"{"a":"unterminated}"#);
    assert!(node.is_some());
    assert!(
        errors
            .iter()
            .any(|error| error.kind == ErrorKind::UnterminatedString)
    );
}

#[test]
fn parse_top_level_invalid_input_returns_no_document() {
    let (node, errors) = parse("");
    assert!(node.is_none());
    assert_eq!(errors[0].kind, ErrorKind::UnexpectedEof);
}

#[test]
fn parse_big_integer_preserves_text() {
    let value = "1234567890123456789012345678901234567890";
    let (parsed_node_opt, errors) = parse(value);
    assert!(errors.is_empty());
    let parsed_node = parsed_node_opt.expect("expected node");
    assert_eq!(
        parsed_node,
        Node::new(
            Value::Int(Integer::BigIntStr(Cow::Borrowed(value))),
            parsed_node.span
        )
    );
}

#[cfg(feature = "serde")]
#[test]
fn serde_from_str_directly_from_tokens() {
    let value: BTreeMap<String, i64> =
        crate::serde::from_str(r#"{"a":1,"b":2}"#).expect("expected valid map");
    assert_eq!(value.get("a"), Some(&1));
    assert_eq!(value.get("b"), Some(&2));
}

#[cfg(feature = "serde")]
#[test]
fn serde_from_str_borrowed_string() {
    #[derive(Debug, serde::Deserialize, PartialEq)]
    struct Borrowed<'a> {
        value: &'a str,
    }

    let value: Borrowed<'_> =
        crate::serde::from_str(r#"{"value":"hello"}"#).expect("expected borrowed string");
    assert_eq!(value, Borrowed { value: "hello" });
}

#[cfg(feature = "serde")]
#[test]
fn serde_from_str_bool_type_mismatch_errors() {
    let error = crate::serde::from_str::<bool>("1").expect_err("expected bool type mismatch");
    assert!(error.to_string().contains("expected boolean, found number"));
}

#[cfg(feature = "serde")]
#[test]
fn serde_from_str_seq_type_mismatch_errors() {
    let error = crate::serde::from_str::<Vec<i64>>(r#"{"a":1}"#)
        .expect_err("expected sequence type mismatch");
    assert!(
        error
            .to_string()
            .contains("expected sequence, found mapping")
    );
}

#[cfg(feature = "serde")]
#[test]
fn serde_from_str_big_integer_out_of_range_errors() {
    let error = crate::serde::from_str::<i128>("1234567890123456789012345678901234567890")
        .expect_err("expected bigint to be out of range for i128");
    assert!(error.to_string().contains("out of range"));
}

#[cfg(feature = "serde")]
#[test]
fn serde_from_str_big_integer_preserves_text_in_any() {
    let value: serde_json::Value =
        crate::serde::from_str("1234567890123456789012345678901234567890")
            .expect("expected generic value deserialization");
    assert_eq!(
        value,
        serde_json::Value::String("1234567890123456789012345678901234567890".to_owned())
    );
}
