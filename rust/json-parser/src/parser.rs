// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

use std::borrow::Cow;

use crate::{
    error::{ErrorKind, ParseError},
    lexer::{Token, TokenCursor, TokenKind, TokenTag},
    span::Span,
    value::{Integer, Node, Value},
};

pub(crate) struct Parser<'input> {
    cursor: TokenCursor<'input>,
    errors: Vec<ParseError>,
}

#[derive(Clone, Copy)]
enum ObjectPhase {
    BeforeKey,
    AfterKey,
    BeforeValue,
    AfterEntry,
}

#[derive(Clone, Copy)]
enum ArrayPhase {
    BeforeValue,
    AfterValue,
}

impl<'input> Parser<'input> {
    #[inline]
    pub(crate) fn new(input: &'input str) -> Self {
        Self {
            cursor: TokenCursor::new(input),
            errors: Vec::new(),
        }
    }

    #[inline]
    pub(crate) fn parse(&mut self) -> Option<Node<'input>> {
        let node = self.parse_value();
        if node.is_some() {
            if !matches!(self.cursor.peek_kind(), TokenKind::Eof) {
                self.errors
                    .push(ParseError::new(ErrorKind::TrailingContent, self.cursor.peek_span()));
            }
        }
        node
    }

    #[inline]
    pub(crate) fn take_errors(&mut self) -> Vec<ParseError> {
        let mut all_errors = self.cursor.take_errors();
        all_errors.append(&mut self.errors);
        all_errors
    }

    #[inline]
    fn parse_value(&mut self) -> Option<Node<'input>> {
        let token = self.cursor.next();
        self.parse_value_token(token)
    }

    #[inline]
    fn parse_value_token(&mut self, token: Token<'input>) -> Option<Node<'input>> {
        match token.kind {
            TokenKind::Null => Some(Node::new(Value::Null, token.span)),
            TokenKind::True => Some(Node::new(Value::Bool(true), token.span)),
            TokenKind::False => Some(Node::new(Value::Bool(false), token.span)),
            TokenKind::String(value) => Some(Node::new(Value::String(value), token.span)),
            TokenKind::Number(value) => self.parse_number(value, token.span),
            TokenKind::LeftBracket => Some(self.parse_array(token.span)),
            TokenKind::LeftBrace => Some(self.parse_object(token.span)),
            TokenKind::Eof => {
                self.errors
                    .push(ParseError::new(ErrorKind::UnexpectedEof, token.span));
                None
            }
            TokenKind::Invalid => {
                self.errors
                    .push(ParseError::new(ErrorKind::InvalidValue, token.span));
                None
            }
            TokenKind::RightBracket
            | TokenKind::RightBrace
            | TokenKind::Colon
            | TokenKind::Comma => {
                self.errors
                    .push(ParseError::new(ErrorKind::InvalidValue, token.span));
                None
            }
        }
    }

    #[inline]
    fn parse_number(&mut self, raw: Cow<'input, str>, span: Span) -> Option<Node<'input>> {
        let slice = raw.as_ref();
        let kind = if raw.contains(['.', 'e', 'E']) {
            slice
                .parse::<f64>()
                .ok()
                .map(Value::Float)
                .filter(|value| !matches!(value, Value::Float(float) if !float.is_finite()))
        } else {
            parse_integer(raw).map(Value::Int)
        };

        if let Some(value) = kind {
            Some(Node::new(value, span))
        } else {
            self.errors
                .push(ParseError::new(ErrorKind::InvalidNumber, span));
            None
        }
    }

    fn parse_array(&mut self, open_span: Span) -> Node<'input> {
        let mut items = Vec::new();
        let mut end_span = open_span;
        let mut phase = ArrayPhase::BeforeValue;

        loop {
            match phase {
                ArrayPhase::BeforeValue => match self.cursor.next() {
                    Token {
                        tag: TokenTag::RightBracket,
                        span,
                        ..
                    } => {
                        end_span = span;
                        break;
                    }
                    Token {
                        tag: TokenTag::Eof,
                        span,
                        ..
                    } => {
                        self.errors
                            .push(ParseError::new(ErrorKind::UnexpectedEof, span));
                        break;
                    }
                    Token {
                        tag: TokenTag::Comma,
                        span,
                        ..
                    } => {
                        self.errors
                            .push(ParseError::new(ErrorKind::InvalidValue, span));
                    }
                    token => {
                        if let Some(value) = self.parse_value_token(token) {
                            end_span = value.span;
                            items.push(value);
                            phase = ArrayPhase::AfterValue;
                        } else {
                            self.synchronize_array();
                        }
                    }
                },
                ArrayPhase::AfterValue => match self.cursor.next() {
                    Token {
                        tag: TokenTag::Comma,
                        ..
                    } => {
                        phase = ArrayPhase::BeforeValue;
                    }
                    Token {
                        tag: TokenTag::RightBracket,
                        span,
                        ..
                    } => {
                        end_span = span;
                        break;
                    }
                    Token {
                        tag: TokenTag::Eof,
                        span,
                        ..
                    } => {
                        self.errors
                            .push(ParseError::new(ErrorKind::UnexpectedEof, span));
                        break;
                    }
                    token => {
                        self.errors
                            .push(ParseError::new(ErrorKind::MissingComma, token.span));
                        self.cursor.push_back(token);
                        phase = ArrayPhase::BeforeValue;
                    }
                },
            }
        }

        Node::new(Value::Sequence(items), open_span.union(end_span))
    }

    fn parse_object(&mut self, open_span: Span) -> Node<'input> {
        let mut pairs = Vec::new();
        let mut end_span = open_span;
        let mut phase = ObjectPhase::BeforeKey;
        let mut pending_key: Option<Node<'input>> = None;
        let mut pending_value_token: Option<Token<'input>> = None;

        loop {
            match phase {
                ObjectPhase::BeforeKey => {
                    let token = self.cursor.next();
                    match token.tag {
                        TokenTag::RightBrace => {
                            end_span = token.span;
                            break;
                        }
                        TokenTag::Eof => {
                            self.errors
                                .push(ParseError::new(ErrorKind::UnexpectedEof, token.span));
                            break;
                        }
                        TokenTag::String => {
                            let TokenKind::String(value) = token.kind else {
                                unreachable!("token tag reported String");
                            };
                            pending_key = Some(Node::new(Value::String(value), token.span));
                            phase = ObjectPhase::AfterKey;
                        }
                        _ => {
                            self.errors.push(ParseError::new(
                                ErrorKind::ObjectKeyMustBeString,
                                token.span,
                            ));
                            self.synchronize_object();
                            phase = ObjectPhase::BeforeKey;
                        }
                    }
                }
                ObjectPhase::AfterKey => {
                    let separator = self.cursor.next();
                    match separator.tag {
                        TokenTag::Colon => {
                            pending_value_token = None;
                            phase = ObjectPhase::BeforeValue;
                        }
                        TokenTag::Comma => {
                            self.errors
                                .push(ParseError::new(ErrorKind::MissingColon, separator.span));
                            self.errors
                                .push(ParseError::new(ErrorKind::InvalidValue, separator.span));
                            phase = ObjectPhase::BeforeKey;
                        }
                        TokenTag::RightBrace => {
                            self.errors
                                .push(ParseError::new(ErrorKind::MissingColon, separator.span));
                            self.errors
                                .push(ParseError::new(ErrorKind::InvalidValue, separator.span));
                            end_span = separator.span;
                            break;
                        }
                        TokenTag::Eof => {
                            self.errors
                                .push(ParseError::new(ErrorKind::MissingColon, separator.span));
                            self.errors
                                .push(ParseError::new(ErrorKind::UnexpectedEof, separator.span));
                            break;
                        }
                        _ => {
                            self.errors
                                .push(ParseError::new(ErrorKind::MissingColon, separator.span));
                            pending_value_token = Some(separator);
                            phase = ObjectPhase::BeforeValue;
                        }
                    }
                }
                ObjectPhase::BeforeValue => {
                    let value_token = pending_value_token.take().unwrap_or_else(|| self.cursor.next());
                    match value_token.tag {
                        TokenTag::Comma => {
                            self.errors.push(ParseError::new(
                                ErrorKind::InvalidValue,
                                value_token.span,
                            ));
                            phase = ObjectPhase::BeforeKey;
                        }
                        TokenTag::RightBrace => {
                            self.errors.push(ParseError::new(
                                ErrorKind::InvalidValue,
                                value_token.span,
                            ));
                            end_span = value_token.span;
                            break;
                        }
                        TokenTag::Eof => {
                            let _ = self.parse_value_token(value_token);
                            break;
                        }
                        _ => {
                            let key = pending_key.take().expect("object value requires key");
                            if let Some(value) = self.parse_value_token(value_token) {
                                end_span = value.span;
                                pairs.push((key, value));
                            }
                            phase = ObjectPhase::AfterEntry;
                        }
                    }
                }
                ObjectPhase::AfterEntry => match self.cursor.next() {
                    Token {
                        tag: TokenTag::Comma,
                        ..
                    } => {
                        phase = ObjectPhase::BeforeKey;
                    }
                    Token {
                        tag: TokenTag::RightBrace,
                        span,
                        ..
                    } => {
                        end_span = span;
                        break;
                    }
                    Token {
                        tag: TokenTag::Eof,
                        span,
                        ..
                    } => {
                        self.errors
                            .push(ParseError::new(ErrorKind::UnexpectedEof, span));
                        break;
                    }
                    token => {
                        self.errors
                            .push(ParseError::new(ErrorKind::MissingComma, token.span));
                        self.cursor.push_back(token);
                        phase = ObjectPhase::BeforeKey;
                    }
                },
            }
        }

        Node::new(Value::Mapping(pairs), open_span.union(end_span))
    }

    fn synchronize_object(&mut self) {
        loop {
            match self.cursor.peek_tag() {
                TokenTag::Comma => {
                    let _ = self.cursor.next();
                    break;
                }
                TokenTag::RightBrace | TokenTag::Eof => break,
                _ => {
                    let _ = self.cursor.next();
                }
            }
        }
    }

    fn synchronize_array(&mut self) {
        loop {
            match self.cursor.peek_tag() {
                TokenTag::Comma => {
                    let _ = self.cursor.next();
                    break;
                }
                TokenTag::RightBracket | TokenTag::Eof => break,
                _ => {
                    let _ = self.cursor.next();
                }
            }
        }
    }
}

fn parse_integer(input: Cow<'_, str>) -> Option<Integer<'_>> {
    if !is_valid_json_number(input.as_ref()) {
        return None;
    }

    if let Ok(value) = input.parse::<i64>() {
        return Some(Integer::I64(value));
    }
    if !input.starts_with('-') {
        if let Ok(value) = input.parse::<u64>() {
            return Some(Integer::U64(value));
        }
        if let Ok(value) = input.parse::<u128>() {
            return Some(Integer::U128(value));
        }
    }
    if let Ok(value) = input.parse::<i128>() {
        return Some(Integer::I128(value));
    }

    Some(Integer::BigIntStr(input))
}

fn is_valid_json_number(input: &str) -> bool {
    let bytes = input.as_bytes();
    let mut index = 0;

    if bytes.is_empty() {
        return false;
    }

    if bytes[index] == b'-' {
        index += 1;
        if index >= bytes.len() {
            return false;
        }
    }

    match bytes[index] {
        b'0' => {
            index += 1;
            if index < bytes.len() && bytes[index].is_ascii_digit() {
                return false;
            }
        }
        b'1'..=b'9' => {
            index += 1;
            while index < bytes.len() && bytes[index].is_ascii_digit() {
                index += 1;
            }
        }
        _ => return false,
    }

    if index < bytes.len() && bytes[index] == b'.' {
        index += 1;
        let fraction_start = index;
        while index < bytes.len() && bytes[index].is_ascii_digit() {
            index += 1;
        }
        if fraction_start == index {
            return false;
        }
    }

    if index < bytes.len() && matches!(bytes[index], b'e' | b'E') {
        index += 1;
        if index < bytes.len() && matches!(bytes[index], b'+' | b'-') {
            index += 1;
        }
        let exponent_start = index;
        while index < bytes.len() && bytes[index].is_ascii_digit() {
            index += 1;
        }
        if exponent_start == index {
            return false;
        }
    }

    index == bytes.len()
}
