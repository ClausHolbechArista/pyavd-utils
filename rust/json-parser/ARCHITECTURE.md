<!--
  ~ Copyright (c) 2026 Arista Networks, Inc.
  ~ Use of this source code is governed by the Apache License 2.0
  ~ that can be found in the LICENSE file.
  -->

# JSON Parser Architecture

**Date:** 2026-03-24
**Status:** Current workspace design
**Scope:** Internal architecture and public entrypoints

## Overview

`json-parser` is a strict JSON parser with four main goals:

- recover from syntax errors without stopping at the first issue
- preserve source spans throughout the AST and diagnostic pipeline
- keep the hot path allocation-light and zero-copy where possible
- support both direct serde integration and AST-based writing

The crate exposes three main user-facing styles:

- AST parsing via `parse`
- AST writing via `writer::write_json`
- serde deserialization / serialization via `json_parser::serde`

## Pipeline

The public pipeline is intentionally smaller than YAML:

```text
AST path:         Lexer -> TokenCursor -> Parser -> Node
AST write path:   Node -> writer
serde read path:  Lexer -> TokenCursor -> Deserializer
serde write path: Serialize -> ValueSerializer -> Node -> writer
```

That means:

- lexing happens once per parse/deserialization path
- JSON structure is handled directly by the recovering parser or serde
  deserializer
- writing is AST-based rather than event-based

## Layer 1: Lexer and Token Cursor

The lexer lives in `src/lexer/` and tokenizes strict JSON:

- braces, brackets, colon, comma
- strings
- numbers
- `true`, `false`, `null`
- EOF and invalid tokens

Important design points:

- token payloads borrow from the input where possible
- tokens cache a lightweight `TokenTag` so hot loops can branch on kind
  without re-matching full payload variants
- lexer errors are collected and exposed through the cursor

The `TokenCursor` is the shared consumer interface for both the recovering
parser and the serde deserializer.

## Layer 2A: Recovering AST Parser

The AST parser lives in `src/parser.rs`.

Key responsibilities:

- parse a single strict JSON value into `Node` / `Value`
- recover from missing commas, missing colons, and malformed members/items
- preserve source spans on every AST node
- preserve bigint text when integer values exceed native integer ranges

The parser is state-machine oriented in its container loops so the valid-object
and valid-array fast paths stay cheap.

## Layer 2B: Serde Deserializer

The serde read path lives in `src/serde/de.rs`.

Key responsibilities:

- deserialize directly from the shared token cursor
- avoid building the AST for ordinary serde use
- preserve span-aware custom errors where practical
- keep type-directed entrypoints strict rather than falling back broadly to
  `deserialize_any`

## Layer 3A: AST Writer

The writer lives in `src/writer.rs`.

Key responsibilities:

- emit strict compact JSON from `Node` / `Value`
- preserve AST ordering for arrays and objects
- escape strings correctly
- preserve bigint textual form when the AST stores `Integer::BigIntStr`
- reject invalid manual AST states such as non-string object keys

This layer does not attempt to preserve original formatting or comments.

## Layer 3B: Serde Serializer

The serde write path lives in `src/serde/ser.rs`.

It mirrors the JSON writer shape rather than the YAML event pipeline:

- generic serde serialization first builds a JSON `Value<'static>` tree
- that tree is wrapped in a root `Node<'static>`
- the AST writer emits final text

This keeps the public serialization path aligned with the AST model and avoids
introducing a JSON-specific event layer that the format does not need.

## Error Model

Errors are collected rather than thrown immediately on parse:

- `ParseError` carries kind and span
- parser recovery can return both a partial tree and accumulated diagnostics
- serde deserialization and serialization use dedicated `DeError` / `SerError`
  types with custom wording

## Span Model

Source locations live in `src/span.rs`.

Important details:

- byte offsets use `u32`
- spans are attached to AST nodes and parse errors
- `SourceMap` converts offsets to line/column positions for diagnostics

## Performance Notes

Current performance work focuses on:

- reducing repeated peek/next patterns in parser hot loops
- using `TokenTag` for cheap kind checks
- preserving zero-copy strings when no escaping is required
- avoiding unnecessary allocation in serde and write paths

The benchmark reference document is `BENCHMARKS.md`.
