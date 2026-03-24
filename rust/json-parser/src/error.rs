// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Error types for JSON parsing.

use crate::span::Span;
use derive_more::Display;

/// An error encountered during JSON parsing.
///
/// Errors carry their source span so callers can convert them into editor or
/// CLI diagnostics with byte-accurate locations.
#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    /// The parse error category.
    pub kind: ErrorKind,
    /// The source range where the error occurred.
    pub span: Span,
}

/// The category of JSON parse error.
#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum ErrorKind {
    /// Unexpected end of input while more tokens were required.
    #[display("unexpected end of input")]
    UnexpectedEof,
    /// Invalid character that cannot begin any JSON token.
    #[display("invalid character")]
    InvalidCharacter,
    /// Invalid value where a scalar, array, or object was expected.
    #[display("invalid value")]
    InvalidValue,
    /// Object key was not a JSON string.
    #[display("object keys must be strings")]
    ObjectKeyMustBeString,
    /// String literal reached EOF before closing quote.
    #[display("unterminated string literal")]
    UnterminatedString,
    /// Invalid backslash escape in a string literal.
    #[display("invalid escape sequence '\\{_0}'")]
    InvalidEscape(char),
    /// Invalid `\uXXXX` escape in a string literal.
    #[display("invalid unicode escape")]
    InvalidUnicodeEscape,
    /// Invalid JSON number syntax.
    #[display("invalid number format")]
    InvalidNumber,
    /// Invalid literal where `true`, `false`, or `null` was expected.
    #[display("invalid literal")]
    InvalidLiteral,
    /// Missing `:` between an object key and value.
    #[display("missing colon after object key")]
    MissingColon,
    /// Missing `,` between array items or object members.
    #[display("missing comma between elements")]
    MissingComma,
    /// Extra input after a complete top-level JSON value.
    #[display("unexpected content after value")]
    TrailingContent,
}

impl ErrorKind {
    /// Return a short fix suggestion when a targeted one exists.
    #[must_use]
    pub fn suggestion(&self) -> Option<&'static str> {
        match self {
            Self::UnterminatedString => Some("add the matching closing quote character"),
            Self::InvalidEscape(_) => {
                Some("valid escape sequences: \\\", \\\\, \\/, \\b, \\f, \\n, \\r, \\t, \\uXXXX")
            }
            Self::InvalidUnicodeEscape => {
                Some("use four hexadecimal digits after \\u, e.g. \\u0041")
            }
            Self::InvalidNumber => Some("use JSON number syntax, e.g. -12, 3.14, or 6.02e23"),
            Self::InvalidLiteral => Some("valid JSON literals are true, false, and null"),
            Self::MissingColon => Some("add a ':' between the object key and its value"),
            Self::MissingComma => Some("add a ',' between array items or object members"),
            Self::TrailingContent => Some("remove the extra content after the top-level value"),
            Self::ObjectKeyMustBeString => Some("quote the object key with double quotes"),
            Self::UnexpectedEof | Self::InvalidCharacter | Self::InvalidValue => None,
        }
    }
}

impl ParseError {
    /// Create a parse error from an error kind and source span.
    #[must_use]
    pub const fn new(kind: ErrorKind, span: Span) -> Self {
        Self { kind, span }
    }

    /// Forward to [`ErrorKind::suggestion`].
    #[must_use]
    pub fn suggestion(&self) -> Option<&'static str> {
        self.kind.suggestion()
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} at bytes {}..{}",
            self.kind,
            self.span.start_usize(),
            self.span.end_usize()
        )
    }
}

impl std::error::Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_literal_has_fix_suggestion() {
        assert_eq!(
            ErrorKind::InvalidLiteral.suggestion(),
            Some("valid JSON literals are true, false, and null")
        );
    }

    #[test]
    fn unexpected_eof_has_no_specific_suggestion() {
        assert_eq!(ErrorKind::UnexpectedEof.suggestion(), None);
    }
}
