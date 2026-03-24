// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! A strict JSON parser with error recovery, span tracking, writing, and
//! optional serde support.
//!
//! This crate provides a JSON parser that:
//! - recovers from syntax errors and reports multiple issues in one pass
//! - tracks source spans on all AST nodes
//! - preserves very large integer text when the value exceeds native integer
//!   ranges
//! - can write the AST back to strict compact JSON
//! - optionally supports direct serde deserialization and serialization
//!
//! # Example
//!
//! ```
//! use json_parser::{Value, parse};
//!
//! let (node, errors) = parse(r#"{"name":"leaf1","enabled":true}"#);
//! assert!(errors.is_empty());
//! let node = node.expect("expected a JSON document");
//! assert!(matches!(node.value, Value::Mapping(_)));
//! ```

mod error;
mod lexer;
mod parser;
mod span;
mod value;

#[cfg(feature = "serde")]
pub mod serde;
pub mod writer;

pub use error::{ErrorKind, ParseError};
pub use span::{Position, SourceMap, Span, Spanned};
pub use value::{Integer, Node, Value};

/// Parse JSON input and return a recovered AST node plus any parse errors.
///
/// The parser is strict JSON, but it recovers from a subset of malformed inputs
/// so editor tooling can keep working with partial trees. On success, the
/// returned node is converted to owned data (`Node<'static>`) so it can outlive
/// the input string.
///
/// When errors are present, the returned node may still contain a useful
/// partial tree.
#[must_use]
pub fn parse(input: &str) -> (Option<Node<'static>>, Vec<ParseError>) {
    let mut parser = parser::Parser::new(input);
    let node = parser.parse().map(Node::into_owned);
    let errors = parser.take_errors();
    (node, errors)
}

#[cfg(test)]
mod tests;
