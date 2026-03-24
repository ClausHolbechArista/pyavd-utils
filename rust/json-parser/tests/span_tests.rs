// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Span-focused integration tests for json-parser.

#![allow(
    clippy::tests_outside_test_module,
    reason = "integration tests in tests/ are top-level by design"
)]
#![allow(clippy::indexing_slicing, reason = "panics are acceptable in tests")]
#![allow(
    clippy::expect_used,
    reason = "expect() in tests provides precise failure messages for invariants"
)]
#![allow(
    clippy::panic,
    reason = "panic is acceptable for structural mismatches in tests"
)]

use json_parser::{Position, SourceMap, Value, parse};

#[test]
fn source_map_reports_line_and_column() {
    let input = "{\n  \"name\": \"leaf1\",\n  \"count\": 42\n}\n";
    let map = SourceMap::new(input);

    assert_eq!(map.position(0), Position::new(1, 1));
    assert_eq!(map.position(4), Position::new(2, 3));
    assert_eq!(map.position(21), Position::new(3, 1));
}

#[test]
fn parsed_value_spans_map_to_expected_positions() {
    let input = "{\n  \"name\": \"leaf1\",\n  \"count\": 42\n}";
    let map = SourceMap::new(input);
    let (parsed_node_opt, errors) = parse(input);
    assert!(errors.is_empty(), "expected clean parse for span test");
    let parsed_node = parsed_node_opt.expect("expected a parsed mapping");

    let Value::Mapping(pairs) = parsed_node.value else {
        panic!("expected mapping at root");
    };

    let value_span = pairs[1].1.span;
    assert_eq!(map.position(value_span.start_usize()), Position::new(3, 12));
    assert_eq!(map.position(value_span.end_usize()), Position::new(3, 14));
}
