// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

use std::borrow::Cow;

use crate::span::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenTag {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    String,
    Number,
    True,
    False,
    Null,
    Invalid,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind<'input> {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    String(Cow<'input, str>),
    Number(Cow<'input, str>),
    True,
    False,
    Null,
    Invalid,
    Eof,
}

impl TokenKind<'_> {
    #[must_use]
    #[inline]
    pub const fn tag(&self) -> TokenTag {
        match self {
            Self::LeftBrace => TokenTag::LeftBrace,
            Self::RightBrace => TokenTag::RightBrace,
            Self::LeftBracket => TokenTag::LeftBracket,
            Self::RightBracket => TokenTag::RightBracket,
            Self::Colon => TokenTag::Colon,
            Self::Comma => TokenTag::Comma,
            Self::String(_) => TokenTag::String,
            Self::Number(_) => TokenTag::Number,
            Self::True => TokenTag::True,
            Self::False => TokenTag::False,
            Self::Null => TokenTag::Null,
            Self::Invalid => TokenTag::Invalid,
            Self::Eof => TokenTag::Eof,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'input> {
    pub tag: TokenTag,
    pub kind: TokenKind<'input>,
    pub span: Span,
}

impl<'input> Token<'input> {
    #[must_use]
    pub const fn new(kind: TokenKind<'input>, span: Span) -> Self {
        let tag = kind.tag();
        Self { tag, kind, span }
    }
}
