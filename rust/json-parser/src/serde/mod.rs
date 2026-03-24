// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Serde integration for `json-parser`.
//!
//! This module exposes the crate's public serde entrypoints for both:
//! - deserialization from JSON text into `T`
//! - serialization from `T` back to JSON text
//!
//! Deserialization uses the lexer and token cursor directly instead of
//! building the AST first. Serialization follows the crate's AST writer shape:
//! it first constructs a JSON `Value<'static>` tree and then writes strict
//! compact JSON text from that tree.

mod de;
mod ser;

pub use de::{DeError, from_str};
pub use ser::{SerError, to_string, to_writer};
