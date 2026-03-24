// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Serde serialization and roundtrip tests for json-parser.

#![cfg(feature = "serde")]
#![allow(
    clippy::tests_outside_test_module,
    reason = "integration tests in tests/ are top-level by design"
)]
#![allow(
    clippy::expect_used,
    reason = "focused serde tests use expect with explicit messages"
)]

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct ExampleConfig {
    name: String,
    enabled: bool,
    count: i64,
    tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Mode {
    Disabled,
    Weighted { weight: i64 },
}

#[derive(Debug, Serialize)]
struct NonFiniteFloat {
    value: f64,
}

#[derive(Debug, Serialize)]
struct BadMapKey {
    value: BTreeMap<i64, bool>,
}

#[test]
fn to_string_roundtrips_via_json_parser_serde() {
    let value = ExampleConfig {
        name: "leaf1".to_owned(),
        enabled: true,
        count: 42,
        tags: vec!["edge".to_owned(), "json".to_owned()],
    };

    let json = json_parser::serde::to_string(&value)
        .expect("serialization to JSON via json-parser should succeed");
    let decoded: ExampleConfig = json_parser::serde::from_str(&json)
        .expect("deserialization from JSON via json-parser should succeed");

    assert_eq!(decoded, value);
}

#[test]
fn to_writer_matches_to_string() {
    let value = Mode::Weighted { weight: 7 };

    let json = json_parser::serde::to_string(&value).expect("to_string should succeed");
    let mut buf = Vec::new();
    json_parser::serde::to_writer(&mut buf, &value).expect("to_writer should succeed");

    assert_eq!(
        String::from_utf8(buf).expect("writer output must be UTF-8"),
        json
    );
}

#[test]
fn non_finite_floats_return_error() {
    let error = json_parser::serde::to_string(&NonFiniteFloat { value: f64::NAN })
        .expect_err("NaN should be rejected by strict JSON serialization");
    assert!(
        error
            .to_string()
            .contains("unsupported floating-point value")
    );
}

#[test]
fn non_string_map_keys_return_error() {
    let mut value = BTreeMap::new();
    value.insert(7, true);

    let error = json_parser::serde::to_string(&BadMapKey { value })
        .expect_err("non-string JSON object keys should be rejected");
    assert!(error.to_string().contains("keys must serialize to strings"));
}

#[test]
fn enum_roundtrips_through_string_and_mapping_forms() {
    let disabled = Mode::Disabled;
    let weighted = Mode::Weighted { weight: 99 };

    let disabled_json =
        json_parser::serde::to_string(&disabled).expect("unit enum should serialize");
    let weighted_json =
        json_parser::serde::to_string(&weighted).expect("struct variant should serialize");

    let disabled_roundtrip: Mode =
        json_parser::serde::from_str(&disabled_json).expect("unit enum should deserialize");
    let weighted_roundtrip: Mode =
        json_parser::serde::from_str(&weighted_json).expect("struct variant should deserialize");

    assert_eq!(disabled_roundtrip, disabled);
    assert_eq!(weighted_roundtrip, weighted);
}
