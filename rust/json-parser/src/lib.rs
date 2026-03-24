// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! A strict JSON parser with error recovery, span tracking, and optional serde support.

mod error;
mod lexer;
mod parser;
mod span;
mod value;

#[cfg(feature = "serde")]
pub mod serde;

pub use error::{ErrorKind, ParseError};
pub use span::{Position, SourceMap, Span, Spanned};
pub use value::{Integer, Node, Value};

/// Parse JSON input and return a recovered AST node plus any parse errors.
///
/// The parser is strict JSON, but recovers from a subset of malformed inputs so
/// editor tooling can keep working with partial trees.
#[must_use]
pub fn parse(input: &str) -> (Option<Node<'static>>, Vec<ParseError>) {
    let mut parser = parser::Parser::new(input);
    let node = parser.parse().map(Node::into_owned);
    let errors = parser.take_errors();
    (node, errors)
}

#[cfg(test)]
mod tests;
