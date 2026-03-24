<!--
  ~ Copyright (c) 2026 Arista Networks, Inc.
  ~ Use of this source code is governed by the Apache License 2.0
  ~ that can be found in the LICENSE file.
  -->

# JSON Parser Benchmarks

**Date:** 2026-03-24  
**Command:** `cargo bench -p json-parser --features serde`  
**Comparison target:** `serde_json`

## Summary

This document reflects the benchmark run from the interactive terminal and should
be treated as the current source of truth.

Current shape:

- `json-parser` is ahead on parse throughput for:
  - `large_array`
  - `nested_object`
  - `escaped_strings`
- `serde_json` is ahead on:
  - `large_object` parse throughput
- `json-parser` is ahead on:
  - `small` and `medium` parse latency
  - `array_heavy` and `escaped_strings` document shapes
- `large` parse latency is effectively a tie in this run.
- `object_heavy` now favors `json-parser` over `serde_json` on absolute time,
  even though Criterion compared it against a faster local historical baseline.
- In the neutral-type serde benchmark, `json-parser` is ahead on all four
  corpora in this run.

## Environment Notes

- Criterion used the `plotters` backend because `gnuplot` was not installed.
- The bench run also executed the crate test binary in bench profile; all 8 tests were ignored as expected.
- The `change` sections printed by Criterion compare against previously stored local baselines.
- The serde benchmark now deserializes into a neutral recursive benchmark type
  instead of `serde_json::Value`, so historical `change` percentages for the
  `serde_deserialize_throughput/*` group are not directly comparable to older
  runs before that harness change.
- The numbers below are the absolute timings and throughputs from the latest
  terminal run, which are the values that matter here.

## Results

### Parse Throughput

| Corpus | json-parser time | json-parser throughput | serde_json time | serde_json throughput | Relative |
|---|---:|---:|---:|---:|---|
| `large_object` | 19.683-19.714 us | 144.35-144.58 MiB/s | 17.937-18.554 us | 153.37-158.66 MiB/s | serde_json ahead |
| `nested_object` | 2.8538-2.9105 us | 192.67-196.50 MiB/s | 3.2813-3.3600 us | 166.89-170.90 MiB/s | json-parser clearly ahead |
| `large_array` | 20.372-20.771 us | 104.41-106.45 MiB/s | 29.317-29.905 us | 72.519-73.973 MiB/s | json-parser clearly ahead |
| `escaped_strings` | 987.52 ns-1.0571 us | 307.62-329.31 MiB/s | 1.4380-1.4718 us | 220.95-226.15 MiB/s | json-parser clearly ahead |

### Parse Latency

| Case | json-parser | serde_json | Relative |
|---|---:|---:|---|
| `small` | 333.63-341.17 ns | 343.80-352.05 ns | json-parser slightly ahead |
| `medium` | 2.9621-3.0485 us | 3.4238-3.4920 us | json-parser clearly ahead |
| `large` | 16.623-17.235 us | 17.035-17.493 us | roughly tied |

### Document Shape Comparison

| Shape | json-parser | serde_json | Relative |
|---|---:|---:|---|
| `object_heavy` | 16.781-17.349 us | 18.849-19.335 us | json-parser ahead |
| `array_heavy` | 20.671-21.178 us | 29.678-31.689 us | json-parser clearly ahead |
| `escaped_strings` | 1.1525-1.2088 us | 1.2150-1.3301 us | json-parser ahead |

### Serde Deserialization Throughput

| Corpus | json-parser time | json-parser throughput | serde_json time | serde_json throughput | Relative |
|---|---:|---:|---:|---:|---|
| `large_object` | 103.80-106.62 us | 26.690-27.416 MiB/s | 156.30-161.39 us | 17.633-18.208 MiB/s | json-parser clearly ahead |
| `nested_object` | 23.994-24.406 us | 22.977-23.371 MiB/s | 27.199-27.679 us | 20.259-20.617 MiB/s | json-parser ahead |
| `large_array` | 96.494-98.548 us | 22.006-22.474 MiB/s | 181.59-194.62 us | 11.143-11.942 MiB/s | json-parser clearly ahead |
| `escaped_strings` | 9.8827-10.029 us | 32.426-32.906 MiB/s | 15.054-15.284 us | 21.278-21.603 MiB/s | json-parser clearly ahead |

## Interpretation

### What looks good

- The parse path is now clearly strong overall:
  - `nested_object`, `large_array`, and `escaped_strings` are strong wins
  - `small` and `medium` latency are also wins
- The object-parser work appears to have paid off:
  - `nested_object` parse throughput is now clearly ahead
  - `object_heavy` is ahead on absolute time in this run
- The recent allocation/string-path work continues to show up:
  - escaped-string parsing is ahead in both throughput and document-shape views
- The neutral-type serde benchmark no longer penalizes `json-parser` by
  comparing against `serde_json` deserializing into its own native value type.
- In this benchmark shape, `json-parser` is ahead on all four serde
  deserialization corpora.

### What still looks expensive

- `large_object` parse throughput is the clearest remaining parser loss.
- `large` parse latency is still only roughly tied rather than a clear win.
- Criterion local baselines are now far behind the current parser in several
  cases, so the `change:` sections are less useful than the absolute times.
- The run-to-run variance is high enough that narrow wins should be confirmed
  with focused sequential benchmarks before drawing strong conclusions.

## Recommended Next Focus

### 1. Parser fast path for valid object/member parsing

This paid off and should be continued where it generalizes cleanly.

Why:

- `nested_object` and `object_heavy` moved materially in the right direction
- the same consume-first/state-carrying ideas likely apply to arrays
- `large_object` remains a good candidate for further container-loop work

The most promising direction remains splitting hot success paths from recovery
logic in `src/parser.rs`, especially where arrays still use the older
peek-heavy style.

### 2. Serde deserializer path

This is now more of a secondary tuning area.

Why:

- the benchmark is now fairer, and `json-parser` performs well in this shape
- if we optimize this path further, it should be driven by focused profiling or
  targeted microbenchmarks rather than by the old biased comparison

## Reproducing

Run:

```bash
cargo bench -p json-parser --features serde
```

To compile benchmarks without running them:

```bash
cargo bench -p json-parser --features serde --no-run
```
