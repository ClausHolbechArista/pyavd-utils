// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Basic tests for the AST-based JSON writer.

#![allow(
    clippy::tests_outside_test_module,
    reason = "integration tests in tests/ are top-level by design"
)]
#![allow(
    clippy::expect_used,
    reason = "focused roundtrip tests use expect with explicit messages"
)]
#![allow(
    clippy::float_cmp,
    reason = "exact float comparisons are intentional for AST roundtrip checks"
)]
#![allow(
    clippy::panic,
    reason = "panic is acceptable for structural mismatch reporting in tests"
)]

use std::borrow::Cow;

use json_parser::{Integer, Node, Value, parse, writer};

fn assert_value_eq_ignoring_spans(expected: &Value<'_>, actual: &Value<'_>) {
    match (expected, actual) {
        (Value::Null, Value::Null) => {}
        (Value::Bool(left), Value::Bool(right)) => assert_eq!(left, right, "bool value changed"),
        (Value::Int(left), Value::Int(right)) => assert_eq!(left, right, "integer value changed"),
        (Value::Float(left), Value::Float(right)) => {
            assert_eq!(left, right, "float value changed");
        }
        (Value::String(left), Value::String(right)) => {
            assert_eq!(left, right, "string value changed");
        }
        (Value::Sequence(left_items), Value::Sequence(right_items)) => {
            assert_eq!(
                left_items.len(),
                right_items.len(),
                "sequence length changed"
            );
            for (left_node, right_node) in left_items.iter().zip(right_items.iter()) {
                assert_value_eq_ignoring_spans(&left_node.value, &right_node.value);
            }
        }
        (Value::Mapping(left_pairs), Value::Mapping(right_pairs)) => {
            assert_eq!(
                left_pairs.len(),
                right_pairs.len(),
                "mapping length changed"
            );
            for ((left_key, left_value), (right_key, right_value)) in
                left_pairs.iter().zip(right_pairs.iter())
            {
                assert_value_eq_ignoring_spans(&left_key.value, &right_key.value);
                assert_value_eq_ignoring_spans(&left_value.value, &right_value.value);
            }
        }
        (left, right) => {
            panic!("value kind changed after roundtrip: left={left:?}, right={right:?}");
        }
    }
}

fn roundtrip_value(input: &str) {
    let (parsed_before_opt, errors_before) = parse(input);
    assert!(
        errors_before.is_empty(),
        "expected no parse errors before roundtrip, got: {errors_before:?}"
    );
    let parsed_before = parsed_before_opt.expect("expected a node before roundtrip");

    let mut buf = Vec::new();
    writer::write_json(&mut buf, &parsed_before).expect("writing JSON should succeed");
    let output = String::from_utf8(buf).expect("writer must produce valid UTF-8");

    let (parsed_after_opt, errors_after) = parse(&output);
    assert!(
        errors_after.is_empty(),
        "expected no parse errors after roundtrip, got: {errors_after:?}\nOUTPUT:\n{output}"
    );
    let parsed_after = parsed_after_opt.expect("expected a node after roundtrip");

    assert_value_eq_ignoring_spans(&parsed_before.value, &parsed_after.value);
}

#[test]
fn writer_roundtrips_simple_object() {
    roundtrip_value(r#"{"name":"leaf1","enabled":true,"count":42}"#);
}

#[test]
fn writer_emits_scalar_values() {
    let cases = [
        (Node::new(Value::Null, json_parser::Span::default()), "null"),
        (
            Node::new(Value::Bool(true), json_parser::Span::default()),
            "true",
        ),
        (
            Node::new(Value::Int(Integer::I64(42)), json_parser::Span::default()),
            "42",
        ),
        (
            Node::new(Value::Float(3.5), json_parser::Span::default()),
            "3.5",
        ),
        (
            Node::new(
                Value::String(Cow::Borrowed("leaf")),
                json_parser::Span::default(),
            ),
            r#""leaf""#,
        ),
    ];

    for (node, expected) in cases {
        let mut buf = Vec::new();
        writer::write_json(&mut buf, &node).expect("scalar writing should succeed");
        let output = String::from_utf8(buf).expect("writer output should be UTF-8");
        assert_eq!(output, expected);
    }
}

#[test]
fn writer_roundtrips_nested_structures() {
    roundtrip_value(r#"{"outer":{"inner":[1,2,3],"flag":false},"name":"leaf"}"#);
}

#[test]
fn writer_escapes_control_characters_and_quotes() {
    let node = Node::new(
        Value::String(Cow::Borrowed("\"slash\\line\n\t\u{0008}\u{000C}")),
        json_parser::Span::default(),
    );

    let mut buf = Vec::new();
    writer::write_json(&mut buf, &node).expect("writing escaped string should succeed");
    let output = String::from_utf8(buf).expect("writer output should be UTF-8");

    assert_eq!(output, r#""\"slash\\line\n\t\b\f""#);
}

#[test]
fn writer_preserves_big_integer_text() {
    let bigint = "12345678901234567890123456789012345678901234567890";
    let node = Node::new(
        Value::Int(Integer::BigIntStr(Cow::Borrowed(bigint))),
        json_parser::Span::default(),
    );

    let mut buf = Vec::new();
    writer::write_json(&mut buf, &node).expect("writing bigint should succeed");
    let output = String::from_utf8(buf).expect("writer output should be UTF-8");

    assert_eq!(output, bigint);

    let (roundtripped_node_opt, errors) = parse(&output);
    assert!(errors.is_empty(), "bigint output should parse cleanly");
    let roundtripped_node = roundtripped_node_opt.expect("expected bigint node after roundtrip");
    assert_eq!(roundtripped_node.value, node.value);
}

#[test]
fn writer_preserves_object_order_from_ast() {
    let input = r#"{"first":1,"second":2,"third":3}"#;
    let (parsed_node_opt, errors) = parse(input);
    assert!(errors.is_empty());
    let parsed_node = parsed_node_opt.expect("expected ordered mapping");

    let mut buf = Vec::new();
    writer::write_json(&mut buf, &parsed_node).expect("writing ordered mapping should succeed");
    let output = String::from_utf8(buf).expect("writer output should be UTF-8");

    assert_eq!(output, input);
}

#[test]
fn writer_rejects_non_string_object_keys() {
    let node = Node::new(
        Value::Mapping(vec![(
            Node::new(Value::Int(Integer::I64(1)), json_parser::Span::default()),
            Node::new(Value::Bool(true), json_parser::Span::default()),
        )]),
        json_parser::Span::default(),
    );

    let error =
        writer::write_json(Vec::new(), &node).expect_err("non-string object keys must fail");
    assert!(error.to_string().contains("object key must be a string"));
}
