// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! JSON value types with span information.

use std::borrow::Cow;

use crate::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum Integer<'input> {
    I64(i64),
    U64(u64),
    I128(i128),
    U128(u128),
    BigIntStr(Cow<'input, str>),
}

impl Integer<'_> {
    #[must_use]
    pub fn into_owned(self) -> Integer<'static> {
        match self {
            Self::I64(value) => Integer::I64(value),
            Self::U64(value) => Integer::U64(value),
            Self::I128(value) => Integer::I128(value),
            Self::U128(value) => Integer::U128(value),
            Self::BigIntStr(text) => Integer::BigIntStr(Cow::Owned(text.into_owned())),
        }
    }

    #[must_use]
    pub fn to_decimal_string(&self) -> Cow<'_, str> {
        match self {
            Self::I64(value) => Cow::Owned(value.to_string()),
            Self::U64(value) => Cow::Owned(value.to_string()),
            Self::I128(value) => Cow::Owned(value.to_string()),
            Self::U128(value) => Cow::Owned(value.to_string()),
            Self::BigIntStr(text) => Cow::Borrowed(text.as_ref()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node<'input> {
    pub value: Value<'input>,
    pub span: Span,
}

impl<'input> Node<'input> {
    #[must_use]
    pub const fn new(value: Value<'input>, span: Span) -> Self {
        Self { value, span }
    }

    #[must_use]
    pub fn into_owned(self) -> Node<'static> {
        Node {
            value: self.value.into_owned(),
            span: self.span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value<'input> {
    Null,
    Bool(bool),
    Int(Integer<'input>),
    Float(f64),
    String(Cow<'input, str>),
    Sequence(Vec<Node<'input>>),
    Mapping(Vec<(Node<'input>, Node<'input>)>),
}

impl Value<'_> {
    #[must_use]
    pub fn into_owned(self) -> Value<'static> {
        match self {
            Self::Null => Value::Null,
            Self::Bool(value) => Value::Bool(value),
            Self::Int(value) => Value::Int(value.into_owned()),
            Self::Float(value) => Value::Float(value),
            Self::String(value) => Value::String(Cow::Owned(value.into_owned())),
            Self::Sequence(items) => {
                Value::Sequence(items.into_iter().map(Node::into_owned).collect())
            }
            Self::Mapping(pairs) => Value::Mapping(
                pairs
                    .into_iter()
                    .map(|(key, value)| (key.into_owned(), value.into_owned()))
                    .collect(),
            ),
        }
    }
}
