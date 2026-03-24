<!--
  ~ Copyright (c) 2026 Arista Networks, Inc.
  ~ Use of this source code is governed by the Apache License 2.0
  ~ that can be found in the LICENSE file.
  -->

# Benchmark Report

**Date:** 2026-03-24
**Parser Version:** 0.0.2
**Source of truth for this snapshot:** interactive terminal run from 2026-03-24 using `cargo bench -p json-parser --features serde`

## How To Run

From the workspace root:

```bash
cargo bench -p json-parser --features serde
```

To compile benchmark binaries without running them:

```bash
cargo bench -p json-parser --features serde --no-run
```

For analysis work, prefer one captured run:

```bash
cargo bench -p json-parser --features serde \
  > /tmp/json-parser-bench-YYYYMMDD-full.txt 2>&1
```

Then extract the comparisons you need from that file rather than relying on
Criterion's baseline-change percentages alone.

## Comparison Targets

- `json_parser`: this crate
- `serde_json`: reference implementation

Notes:

- `parse_throughput`, `parse_latency`, and `document_shapes` compare the parse
  path against `serde_json` parsing into `serde_json::Value`.
- `serde_deserialize_throughput` deserializes both backends into the same
  neutral recursive benchmark type rather than `serde_json::Value`, to avoid
  fast-path bias.
- The benchmark harness now also includes:
  - `ast_write_throughput`
  - `serde_serialize_throughput`
- This snapshot predates the first captured run of those new write/serialize
  groups, so their sections below describe intent rather than reporting numbers.
- Absolute numbers vary with machine load. Relative comparisons are the main
  signal.

## Current Takeaways

- Parse throughput is ahead of `serde_json` on `nested_object`,
  `large_array`, and `escaped_strings`.
- `large_object` parse throughput remains the clearest parse-path loss.
- Parse latency is ahead on `small` and `medium`, with `large` roughly tied in
  the current snapshot.
- Document-shape benchmarks favor `json-parser` on `object_heavy`,
  `array_heavy`, and `escaped_strings` in absolute time.
- In the neutral-type serde deserialization benchmark, `json-parser` is ahead
  on all four corpora in this snapshot.
- The newly added AST-write and serde-serialize benchmarks should now become
  the next source of truth for output-path work.

## Parse Throughput

Median ranges from the captured terminal run.

| Corpus | json-parser time | json-parser throughput | serde_json time | serde_json throughput | Relative |
| --- | ---: | ---: | ---: | ---: | --- |
| `large_object` | 19.683-19.714 us | 144.35-144.58 MiB/s | 17.937-18.554 us | 153.37-158.66 MiB/s | serde_json ahead |
| `nested_object` | 2.8538-2.9105 us | 192.67-196.50 MiB/s | 3.2813-3.3600 us | 166.89-170.90 MiB/s | json-parser clearly ahead |
| `large_array` | 20.372-20.771 us | 104.41-106.45 MiB/s | 29.317-29.905 us | 72.519-73.973 MiB/s | json-parser clearly ahead |
| `escaped_strings` | 987.52 ns-1.0571 us | 307.62-329.31 MiB/s | 1.4380-1.4718 us | 220.95-226.15 MiB/s | json-parser clearly ahead |

## Parse Latency

Median time per parse from the captured terminal run.

| Case | json-parser | serde_json | Relative |
| --- | ---: | ---: | --- |
| `small` | 333.63-341.17 ns | 343.80-352.05 ns | json-parser slightly ahead |
| `medium` | 2.9621-3.0485 us | 3.4238-3.4920 us | json-parser clearly ahead |
| `large` | 16.623-17.235 us | 17.035-17.493 us | roughly tied |

## Document Shape Comparison

Median time per parse from the captured terminal run.

| Shape | json-parser | serde_json | Relative |
| --- | ---: | ---: | --- |
| `object_heavy` | 16.781-17.349 us | 18.849-19.335 us | json-parser ahead |
| `array_heavy` | 20.671-21.178 us | 29.678-31.689 us | json-parser clearly ahead |
| `escaped_strings` | 1.1525-1.2088 us | 1.2150-1.3301 us | json-parser ahead |

## Serde Deserialize Throughput

Both backends deserialize into the same neutral recursive benchmark type.

| Corpus | json-parser time | json-parser throughput | serde_json time | serde_json throughput | Relative |
| --- | ---: | ---: | ---: | ---: | --- |
| `large_object` | 103.80-106.62 us | 26.690-27.416 MiB/s | 156.30-161.39 us | 17.633-18.208 MiB/s | json-parser clearly ahead |
| `nested_object` | 23.994-24.406 us | 22.977-23.371 MiB/s | 27.199-27.679 us | 20.259-20.617 MiB/s | json-parser ahead |
| `large_array` | 96.494-98.548 us | 22.006-22.474 MiB/s | 181.59-194.62 us | 11.143-11.942 MiB/s | json-parser clearly ahead |
| `escaped_strings` | 9.8827-10.029 us | 32.426-32.906 MiB/s | 15.054-15.284 us | 21.278-21.603 MiB/s | json-parser clearly ahead |

## AST Write Throughput

This group is now part of the benchmark harness but does not yet have a
captured source-of-truth run in this document.

Intent:

- `json_parser`: write a previously parsed `Node` via `writer::write_json`
- `serde_json`: serialize an equivalent `serde_json::Value` via `to_writer`

This measures the AST-to-text path rather than parse or generic serde entrypoints.

## Serde Serialize Throughput

This group is now part of the benchmark harness but does not yet have a
captured source-of-truth run in this document.

Intent:

- `json_parser`: serialize a neutral recursive benchmark type via
  `json_parser::serde::to_writer`
- `serde_json`: serialize the same neutral recursive benchmark type via
  `serde_json::to_writer`

This keeps the serialization comparison apples-to-apples, just as the serde
deserialize benchmark does for the read path.

## Benchmark Matrix Notes

- The benchmark suite intentionally keeps separate parse, AST-write, and serde
  groups because they answer different questions.
- `parse_throughput` and `parse_latency` are about the lexer/parser core.
- `ast_write_throughput` is about the public AST writer.
- `serde_deserialize_throughput` and `serde_serialize_throughput` are about the
  public serde API on top of the core parser and writer layers.
- Historical Criterion `change:` sections may be misleading when the harness or
  comparison target changed; absolute times and throughputs in this document are
  the source of truth.

## Recommended Next Focus

- Continue parser work on the `large_object` valid-member hot path.
- Measure the new write-path groups and identify whether string escaping, float
  rendering, or object emission dominates.
- Keep validating serde comparisons against neutral target types rather than
  `serde_json`'s native value type.
