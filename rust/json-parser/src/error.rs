// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Error types for JSON parsing.

use crate::span::Span;
use derive_more::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub kind: ErrorKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum ErrorKind {
    #[display("unexpected end of input")]
    UnexpectedEof,
    #[display("invalid character")]
    InvalidCharacter,
    #[display("invalid value")]
    InvalidValue,
    #[display("object keys must be strings")]
    ObjectKeyMustBeString,
    #[display("unterminated string literal")]
    UnterminatedString,
    #[display("invalid escape sequence '\\{_0}'")]
    InvalidEscape(char),
    #[display("invalid unicode escape")]
    InvalidUnicodeEscape,
    #[display("invalid number format")]
    InvalidNumber,
    #[display("invalid literal")]
    InvalidLiteral,
    #[display("missing colon after object key")]
    MissingColon,
    #[display("missing comma between elements")]
    MissingComma,
    #[display("unexpected content after value")]
    TrailingContent,
}

impl ErrorKind {
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
    #[must_use]
    pub const fn new(kind: ErrorKind, span: Span) -> Self {
        Self { kind, span }
    }

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
