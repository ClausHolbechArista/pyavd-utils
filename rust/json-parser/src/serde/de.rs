// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

use std::fmt;

use serde::de::Error as _;
use serde::de::{
    self, Deserialize, DeserializeSeed, IntoDeserializer as _, MapAccess, SeqAccess, Visitor,
};
use serde::forward_to_deserialize_any;

use crate::{
    error::{ErrorKind, ParseError},
    lexer::{Token, TokenCursor, TokenKind, TokenTag},
    span::Span,
};

#[derive(Debug, Clone)]
pub struct DeError {
    message: String,
    span: Option<Span>,
}

impl DeError {
    #[allow(
        clippy::needless_pass_by_value,
        reason = "Consumes the parse error into an owned serde error"
    )]
    fn from_parse_error(error: ParseError) -> Self {
        Self {
            message: error.to_string(),
            span: Some(error.span),
        }
    }

    fn custom_with_span(message: impl Into<String>, span: Span) -> Self {
        Self {
            message: message.into(),
            span: Some(span),
        }
    }
}

impl fmt::Display for DeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.span {
            Some(span) => write!(
                f,
                "{} at bytes {}..{}",
                self.message,
                span.start_usize(),
                span.end_usize()
            ),
            None => f.write_str(&self.message),
        }
    }
}

impl std::error::Error for DeError {}

impl de::Error for DeError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self {
            message: msg.to_string(),
            span: None,
        }
    }
}

pub fn from_str<'de, T>(input: &'de str) -> Result<T, DeError>
where
    T: Deserialize<'de>,
{
    let mut deserializer = Deserializer::new(input);
    let value = T::deserialize(&mut deserializer)?;
    let (trailing_kind, trailing_span) = deserializer.cursor.peek_kind_with_span();
    if !matches!(trailing_kind, TokenKind::Eof) {
        return Err(DeError::from_parse_error(ParseError::new(
            ErrorKind::TrailingContent,
            trailing_span,
        )));
    }
    let lexer_errors = deserializer.cursor.take_errors();
    if let Some(error) = lexer_errors.into_iter().next() {
        return Err(DeError::from_parse_error(error));
    }
    Ok(value)
}

struct Deserializer<'de> {
    cursor: TokenCursor<'de>,
}

impl<'de> Deserializer<'de> {
    fn new(input: &'de str) -> Self {
        Self {
            cursor: TokenCursor::new(input),
        }
    }

    fn next_number_value(&mut self) -> Result<(NumberValue<'de>, Span), DeError> {
        let token = self.cursor.next();
        if token.tag != TokenTag::Number {
            return Err(expected_token_error("number", &token));
        }
        let span = token.span;
        let value = Self::parse_number(&token)?;
        Ok((value, span))
    }

    fn parse_number(token: &Token<'de>) -> Result<NumberValue<'de>, DeError> {
        let TokenKind::Number(raw) = &token.kind else {
            return Err(DeError::custom("expected number token"));
        };
        let text = raw.as_ref();
        if text.contains(['.', 'e', 'E']) {
            return text
                .parse::<f64>()
                .map(NumberValue::Float)
                .map_err(|_error| {
                    DeError::from_parse_error(ParseError::new(ErrorKind::InvalidNumber, token.span))
                });
        }
        if let Ok(value) = text.parse::<i64>() {
            return Ok(NumberValue::I64(value));
        }
        if !text.starts_with('-') {
            if let Ok(value) = text.parse::<u64>() {
                return Ok(NumberValue::U64(value));
            }
            if let Ok(value) = text.parse::<u128>() {
                return Ok(NumberValue::U128(value));
            }
        }
        if let Ok(value) = text.parse::<i128>() {
            return Ok(NumberValue::I128(value));
        }

        Ok(NumberValue::BigIntStr(raw.clone()))
    }
}

fn integer_range_error(target: &'static str, span: Span) -> DeError {
    DeError::custom_with_span(format!("integer out of range for {target}"), span)
}

fn expected_integer_error(span: Span) -> DeError {
    DeError::custom_with_span("expected integer, found float", span)
}

fn float_range_error(span: Span) -> DeError {
    DeError::custom_with_span("number out of range for f64", span)
}

fn token_kind_name(token: &Token<'_>) -> &'static str {
    match token.tag {
        TokenTag::Null => "null",
        TokenTag::True | TokenTag::False => "boolean",
        TokenTag::String => "string",
        TokenTag::Number => "number",
        TokenTag::LeftBracket => "sequence",
        TokenTag::LeftBrace => "mapping",
        TokenTag::RightBracket => "]",
        TokenTag::RightBrace => "}",
        TokenTag::Colon => ":",
        TokenTag::Comma => ",",
        TokenTag::Invalid => "invalid token",
        TokenTag::Eof => "end of input",
    }
}

fn expected_token_error(expected: &str, token: &Token<'_>) -> DeError {
    DeError::custom_with_span(
        format!("expected {expected}, found {}", token_kind_name(token)),
        token.span,
    )
}

enum NumberValue<'de> {
    I64(i64),
    U64(u64),
    I128(i128),
    U128(u128),
    Float(f64),
    BigIntStr(std::borrow::Cow<'de, str>),
}

impl<'de> de::Deserializer<'de> for &mut Deserializer<'de> {
    type Error = DeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let token = self.cursor.next();
        match token.kind {
            TokenKind::Null => visitor.visit_unit(),
            TokenKind::True => visitor.visit_bool(true),
            TokenKind::False => visitor.visit_bool(false),
            TokenKind::String(value) => match value {
                std::borrow::Cow::Borrowed(text) => visitor.visit_borrowed_str(text),
                std::borrow::Cow::Owned(text) => visitor.visit_string(text),
            },
            TokenKind::Number(_) => match Deserializer::parse_number(&token)? {
                NumberValue::I64(value) => visitor.visit_i64(value),
                NumberValue::U64(value) => visitor.visit_u64(value),
                NumberValue::I128(value) => visitor.visit_i128(value),
                NumberValue::U128(value) => visitor.visit_u128(value),
                NumberValue::Float(value) => visitor.visit_f64(value),
                NumberValue::BigIntStr(value) => match value {
                    std::borrow::Cow::Borrowed(text) => visitor.visit_borrowed_str(text),
                    std::borrow::Cow::Owned(text) => visitor.visit_string(text),
                },
            },
            TokenKind::LeftBracket => visitor.visit_seq(SeqAccessImpl {
                de: self,
                first: true,
                finished: false,
            }),
            TokenKind::LeftBrace => visitor.visit_map(MapAccessImpl {
                de: self,
                first: true,
                finished: false,
            }),
            TokenKind::Invalid => Err(DeError::from_parse_error(ParseError::new(
                ErrorKind::InvalidValue,
                token.span,
            ))),
            TokenKind::Eof => Err(DeError::from_parse_error(ParseError::new(
                ErrorKind::UnexpectedEof,
                token.span,
            ))),
            TokenKind::RightBrace
            | TokenKind::RightBracket
            | TokenKind::Colon
            | TokenKind::Comma => Err(DeError::custom_with_span("unexpected token", token.span)),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if matches!(self.cursor.peek_tag(), TokenTag::Null) {
            let _ = self.cursor.next();
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let token = self.cursor.next();
        match token.kind {
            TokenKind::True => visitor.visit_bool(true),
            TokenKind::False => visitor.visit_bool(false),
            _ => Err(expected_token_error("boolean", &token)),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let (number, span) = self.next_number_value()?;
        match number {
            NumberValue::I64(value) => visitor.visit_i64(value),
            NumberValue::U64(value) => i64::try_from(value)
                .map_err(|_error| integer_range_error("i64", span))
                .and_then(|converted| visitor.visit_i64(converted)),
            NumberValue::I128(value) => i64::try_from(value)
                .map_err(|_error| integer_range_error("i64", span))
                .and_then(|converted| visitor.visit_i64(converted)),
            NumberValue::U128(value) => i64::try_from(value)
                .map_err(|_error| integer_range_error("i64", span))
                .and_then(|converted| visitor.visit_i64(converted)),
            NumberValue::Float(_) => Err(expected_integer_error(span)),
            NumberValue::BigIntStr(_) => Err(integer_range_error("i64", span)),
        }
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let (number, span) = self.next_number_value()?;
        match number {
            NumberValue::I64(value) => visitor.visit_i128(i128::from(value)),
            NumberValue::U64(value) => visitor.visit_i128(i128::from(value)),
            NumberValue::I128(value) => visitor.visit_i128(value),
            NumberValue::U128(value) => i128::try_from(value)
                .map_err(|_error| integer_range_error("i128", span))
                .and_then(|converted| visitor.visit_i128(converted)),
            NumberValue::Float(_) => Err(expected_integer_error(span)),
            NumberValue::BigIntStr(_) => Err(integer_range_error("i128", span)),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_u64(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_u64(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_u64(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let (number, span) = self.next_number_value()?;
        match number {
            NumberValue::I64(value) => u64::try_from(value)
                .map_err(|_error| integer_range_error("u64", span))
                .and_then(|converted| visitor.visit_u64(converted)),
            NumberValue::U64(value) => visitor.visit_u64(value),
            NumberValue::I128(value) => u64::try_from(value)
                .map_err(|_error| integer_range_error("u64", span))
                .and_then(|converted| visitor.visit_u64(converted)),
            NumberValue::U128(value) => u64::try_from(value)
                .map_err(|_error| integer_range_error("u64", span))
                .and_then(|converted| visitor.visit_u64(converted)),
            NumberValue::Float(_) => Err(expected_integer_error(span)),
            NumberValue::BigIntStr(_) => Err(integer_range_error("u64", span)),
        }
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let (number, span) = self.next_number_value()?;
        match number {
            NumberValue::I64(value) => u128::try_from(value)
                .map_err(|_error| integer_range_error("u128", span))
                .and_then(|converted| visitor.visit_u128(converted)),
            NumberValue::U64(value) => visitor.visit_u128(u128::from(value)),
            NumberValue::I128(value) => u128::try_from(value)
                .map_err(|_error| integer_range_error("u128", span))
                .and_then(|converted| visitor.visit_u128(converted)),
            NumberValue::U128(value) => visitor.visit_u128(value),
            NumberValue::Float(_) => Err(expected_integer_error(span)),
            NumberValue::BigIntStr(_) => Err(integer_range_error("u128", span)),
        }
    }

    #[allow(
        clippy::as_conversions,
        clippy::cast_precision_loss,
        reason = "serde numeric visitors accept f64 coercions from JSON integers"
    )]
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_f64(visitor)
    }

    #[allow(
        clippy::as_conversions,
        clippy::cast_precision_loss,
        reason = "serde numeric visitors accept f64 coercions from JSON integers"
    )]
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let (number, span) = self.next_number_value()?;
        match number {
            NumberValue::I64(value) => visitor.visit_f64(value as f64),
            NumberValue::U64(value) => visitor.visit_f64(value as f64),
            NumberValue::I128(value) => visitor.visit_f64(value as f64),
            NumberValue::U128(value) => visitor.visit_f64(value as f64),
            NumberValue::Float(value) => visitor.visit_f64(value),
            NumberValue::BigIntStr(value) => value
                .as_ref()
                .parse::<f64>()
                .ok()
                .filter(|float| float.is_finite())
                .map_or_else(
                    || Err(float_range_error(span)),
                    |float| visitor.visit_f64(float),
                ),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let token = self.cursor.next();
        match token.kind {
            TokenKind::String(value) => match value {
                std::borrow::Cow::Borrowed(text) => visitor.visit_borrowed_str(text),
                std::borrow::Cow::Owned(text) => visitor.visit_string(text),
            },
            _ => Err(expected_token_error("string", &token)),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let token = self.cursor.next();
        match token.tag {
            TokenTag::Null => visitor.visit_unit(),
            _ => Err(expected_token_error("null", &token)),
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let token = self.cursor.next();
        match token.tag {
            TokenTag::LeftBracket => visitor.visit_seq(SeqAccessImpl {
                de: self,
                first: true,
                finished: false,
            }),
            _ => Err(expected_token_error("sequence", &token)),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let token = self.cursor.next();
        match token.tag {
            TokenTag::LeftBrace => visitor.visit_map(MapAccessImpl {
                de: self,
                first: true,
                finished: false,
            }),
            _ => Err(expected_token_error("mapping", &token)),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let token = self.cursor.next();
        match token.kind {
            TokenKind::String(value) => match value {
                std::borrow::Cow::Borrowed(text) => visitor.visit_borrowed_str(text),
                std::borrow::Cow::Owned(text) => visitor.visit_string(text),
            },
            _ => Err(expected_token_error("string", &token)),
        }
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use serde::de::value::MapAccessDeserializer;
        match self.cursor.next() {
            Token {
                kind: TokenKind::String(value),
                ..
            } => visitor.visit_enum(value.into_deserializer()),
            token @ Token {
                tag: TokenTag::LeftBrace,
                ..
            } => {
                let _ = token;
                let value = visitor.visit_enum(MapAccessDeserializer::new(MapAccessImpl {
                    de: self,
                    first: true,
                    finished: false,
                }))?;
                let closing = self.cursor.next();
                if closing.tag != TokenTag::RightBrace {
                    return Err(expected_token_error("}", &closing));
                }
                Ok(value)
            }
            token => Err(expected_token_error("enum representation", &token)),
        }
    }

    forward_to_deserialize_any! {
        char bytes byte_buf ignored_any
    }
}

struct SeqAccessImpl<'a, 'de> {
    de: &'a mut Deserializer<'de>,
    first: bool,
    finished: bool,
}

impl<'de> SeqAccess<'de> for SeqAccessImpl<'_, 'de> {
    type Error = DeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.finished {
            return Ok(None);
        }
        let token = self.de.cursor.next();
        if self.first {
            match token.tag {
                TokenTag::RightBracket => {
                    self.finished = true;
                    return Ok(None);
                }
                _ => self.de.cursor.push_back(token),
            }
        } else {
            match token.tag {
                TokenTag::Comma => {}
                TokenTag::RightBracket => {
                    self.finished = true;
                    return Ok(None);
                }
                _ => {
                    return Err(DeError::from_parse_error(ParseError::new(
                        ErrorKind::MissingComma,
                        token.span,
                    )));
                }
            }
        }
        self.first = false;
        seed.deserialize(&mut *self.de).map(Some)
    }
}

struct MapAccessImpl<'a, 'de> {
    de: &'a mut Deserializer<'de>,
    first: bool,
    finished: bool,
}

impl<'de> MapAccess<'de> for MapAccessImpl<'_, 'de> {
    type Error = DeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.finished {
            return Ok(None);
        }
        let token = self.de.cursor.next();
        if self.first {
            match token.tag {
                TokenTag::RightBrace => {
                    self.finished = true;
                    return Ok(None);
                }
                _ => self.de.cursor.push_back(token),
            }
        } else {
            match token.tag {
                TokenTag::Comma => {}
                TokenTag::RightBrace => {
                    self.finished = true;
                    return Ok(None);
                }
                _ => {
                    return Err(DeError::from_parse_error(ParseError::new(
                        ErrorKind::MissingComma,
                        token.span,
                    )));
                }
            }
        }
        self.first = false;

        let key_token = self.de.cursor.next();
        let TokenKind::String(key) = key_token.kind else {
            return Err(expected_token_error("string", &key_token));
        };

        let colon = self.de.cursor.next();
        if colon.tag != TokenTag::Colon {
            return Err(DeError::from_parse_error(ParseError::new(
                ErrorKind::MissingColon,
                colon.span,
            )));
        }

        match key {
            std::borrow::Cow::Borrowed(value) => seed.deserialize(value.into_deserializer()),
            std::borrow::Cow::Owned(value) => seed.deserialize(value.into_deserializer()),
        }
        .map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}
