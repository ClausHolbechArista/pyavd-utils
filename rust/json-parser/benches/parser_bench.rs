// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Benchmarks for json-parser against serde_json.
//!
//! Run with: `cargo bench -p json-parser --features serde`

#[cfg(feature = "serde")]
use std::collections::BTreeMap;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

#[cfg(feature = "serde")]
use serde::Deserialize;

const LARGE_OBJECT: &str = include_str!("data/large_object.json");
const NESTED_OBJECT: &str = include_str!("data/nested_object.json");
const LARGE_ARRAY: &str = include_str!("data/large_array.json");
const ESCAPED_STRINGS: &str = include_str!("data/escaped_strings.json");

#[cfg(feature = "serde")]
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum BenchValue {
    Null,
    Bool(bool),
    I64(i64),
    U64(u64),
    Float(f64),
    String(String),
    Sequence(Vec<BenchValue>),
    Mapping(BTreeMap<String, BenchValue>),
}

fn bench_parse_throughput(criterion: &mut Criterion) {
    let test_cases: &[(&str, &str)] = &[
        ("large_object", LARGE_OBJECT),
        ("nested_object", NESTED_OBJECT),
        ("large_array", LARGE_ARRAY),
        ("escaped_strings", ESCAPED_STRINGS),
    ];

    let mut group = criterion.benchmark_group("parse_throughput");

    for (name, input) in test_cases {
        group.throughput(Throughput::Bytes(u64::try_from(input.len()).unwrap()));

        group.bench_with_input(
            BenchmarkId::new("json_parser", name),
            input,
            |bench, data| {
                bench.iter(|| std::hint::black_box(json_parser::parse(data)));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("serde_json", name),
            input,
            |bench, data| {
                bench.iter(|| {
                    let value: serde_json::Value = serde_json::from_str(data).unwrap();
                    std::hint::black_box(value);
                });
            },
        );
    }

    group.finish();
}

fn bench_parse_latency(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("parse_latency");

    let small = r#"{"name":"leaf1","enabled":true,"id":42}"#;
    group.bench_function("json_parser/small", |bench| {
        bench.iter(|| std::hint::black_box(json_parser::parse(small)));
    });
    group.bench_function("serde_json/small", |bench| {
        bench.iter(|| std::hint::black_box(serde_json::from_str::<serde_json::Value>(small).unwrap()));
    });

    group.bench_function("json_parser/medium", |bench| {
        bench.iter(|| std::hint::black_box(json_parser::parse(NESTED_OBJECT)));
    });
    group.bench_function("serde_json/medium", |bench| {
        bench.iter(|| {
            std::hint::black_box(serde_json::from_str::<serde_json::Value>(NESTED_OBJECT).unwrap())
        });
    });

    group.bench_function("json_parser/large", |bench| {
        bench.iter(|| std::hint::black_box(json_parser::parse(LARGE_OBJECT)));
    });
    group.bench_function("serde_json/large", |bench| {
        bench.iter(|| std::hint::black_box(serde_json::from_str::<serde_json::Value>(LARGE_OBJECT).unwrap()));
    });

    group.finish();
}

fn bench_document_shapes(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("document_shapes");

    group.bench_function("json_parser/object_heavy", |bench| {
        bench.iter(|| std::hint::black_box(json_parser::parse(LARGE_OBJECT)));
    });
    group.bench_function("serde_json/object_heavy", |bench| {
        bench.iter(|| std::hint::black_box(serde_json::from_str::<serde_json::Value>(LARGE_OBJECT).unwrap()));
    });

    group.bench_function("json_parser/array_heavy", |bench| {
        bench.iter(|| std::hint::black_box(json_parser::parse(LARGE_ARRAY)));
    });
    group.bench_function("serde_json/array_heavy", |bench| {
        bench.iter(|| std::hint::black_box(serde_json::from_str::<serde_json::Value>(LARGE_ARRAY).unwrap()));
    });

    group.bench_function("json_parser/escaped_strings", |bench| {
        bench.iter(|| std::hint::black_box(json_parser::parse(ESCAPED_STRINGS)));
    });
    group.bench_function("serde_json/escaped_strings", |bench| {
        bench.iter(|| {
            std::hint::black_box(serde_json::from_str::<serde_json::Value>(ESCAPED_STRINGS).unwrap())
        });
    });

    group.finish();
}

#[cfg(feature = "serde")]
fn bench_serde_deserialize_throughput(criterion: &mut Criterion) {
    let test_cases: &[(&str, &str)] = &[
        ("large_object", LARGE_OBJECT),
        ("nested_object", NESTED_OBJECT),
        ("large_array", LARGE_ARRAY),
        ("escaped_strings", ESCAPED_STRINGS),
    ];

    let mut group = criterion.benchmark_group("serde_deserialize_throughput");

    for (name, input) in test_cases {
        group.throughput(Throughput::Bytes(u64::try_from(input.len()).unwrap()));

        group.bench_with_input(
            BenchmarkId::new("json_parser", name),
            input,
            |bench, data| {
                bench.iter(|| {
                    let value: BenchValue = json_parser::serde::from_str(data).unwrap();
                    std::hint::black_box(value);
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("serde_json", name),
            input,
            |bench, data| {
                bench.iter(|| {
                    let value: BenchValue = serde_json::from_str(data).unwrap();
                    std::hint::black_box(value);
                });
            },
        );
    }

    group.finish();
}

#[cfg(not(feature = "serde"))]
fn bench_serde_deserialize_throughput(_criterion: &mut Criterion) {}

criterion_group!(
    benches,
    bench_parse_throughput,
    bench_parse_latency,
    bench_document_shapes,
    bench_serde_deserialize_throughput,
);
criterion_main!(benches);
