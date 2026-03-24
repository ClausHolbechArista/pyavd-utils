// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Implementation of ValidatableValue traits for json_parser types.

use std::borrow::Cow;

use json_parser::{Integer, Node, Value};

use super::{ValidatableMapping, ValidatableSequence, ValidatableValue};

impl<'input> ValidatableValue for Node<'input> {
    type Mapping<'a>
        = NodeMapping<'a, 'input>
    where
        Self: 'a;
    type Sequence<'a>
        = NodeSequence<'a, 'input>
    where
        Self: 'a;
    type Coerced = Node<'static>;

    fn is_null(&self) -> bool {
        matches!(self.value, Value::Null)
    }

    fn is_str(&self) -> bool {
        matches!(self.value, Value::String(_))
    }

    fn is_int(&self) -> bool {
        matches!(self.value, Value::Int(_))
    }

    fn is_bool(&self) -> bool {
        matches!(self.value, Value::Bool(_))
    }

    fn as_str(&self) -> Option<Cow<'_, str>> {
        match &self.value {
            Value::String(cow) => Some(Cow::Borrowed(cow.as_ref())),
            Value::Int(i) => Some(i.to_decimal_string()),
            Value::Float(f) => Some(Cow::Owned(f.to_string())),
            Value::Bool(b) => Some(Cow::Borrowed(if *b { "True" } else { "False" })),
            _ => None,
        }
    }

    fn as_i64(&self) -> Option<i64> {
        match &self.value {
            Value::Int(Integer::I64(i)) => Some(*i),
            Value::String(s) => s.parse().ok(),
            Value::Bool(b) => Some(if *b { 1 } else { 0 }),
            _ => None,
        }
    }

    fn as_bool(&self) -> Option<bool> {
        match &self.value {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    fn as_mapping(&self) -> Option<Self::Mapping<'_>> {
        match &self.value {
            Value::Mapping(pairs) => Some(NodeMapping { pairs }),
            _ => None,
        }
    }

    fn as_sequence(&self) -> Option<Self::Sequence<'_>> {
        match &self.value {
            Value::Sequence(items) => Some(NodeSequence { items }),
            _ => None,
        }
    }

    fn get(&self, key: &str) -> Option<&Self> {
        match &self.value {
            Value::Mapping(pairs) => {
                for (k, v) in pairs {
                    if let Value::String(k_str) = &k.value
                        && k_str.as_ref() == key
                    {
                        return Some(v);
                    }
                }
                None
            }
            _ => None,
        }
    }

    fn value_type(&self) -> crate::feedback::Type {
        use crate::feedback::Type;
        match &self.value {
            Value::Null => Type::Null,
            Value::Bool(_) => Type::Bool,
            Value::Int(_) | Value::Float(_) => Type::Int,
            Value::String(_) => Type::Str,
            Value::Sequence(_) => Type::List,
            Value::Mapping(_) => Type::Dict,
        }
    }

    fn to_feedback_value(&self) -> crate::feedback::Value {
        use crate::feedback::Value as FeedbackValue;
        match &self.value {
            Value::Null => FeedbackValue::Null(),
            Value::Bool(value) => FeedbackValue::Bool(*value),
            Value::Int(Integer::I64(value)) => FeedbackValue::Int(*value),
            Value::Int(_) => FeedbackValue::Str(self.as_str().unwrap_or_default().into_owned()),
            Value::Float(value) => FeedbackValue::Float(*value),
            Value::String(value) => FeedbackValue::Str(value.to_string()),
            Value::Sequence(items) => FeedbackValue::List(
                items.iter().map(ValidatableValue::to_feedback_value).collect(),
            ),
            Value::Mapping(items) => FeedbackValue::Dict(
                items.iter()
                    .filter_map(|(key, value)| {
                        key.as_str()
                            .map(|key_str| (key_str.into_owned(), value.to_feedback_value()))
                    })
                    .collect(),
            ),
        }
    }

    fn is_float(&self) -> bool {
        matches!(self.value, Value::Float(_))
    }

    fn source_span(&self) -> Option<crate::feedback::SourceSpan> {
        Some(self.span.into())
    }

    fn coerce_null(&self) -> Self::Coerced {
        Node::new(Value::Null, self.span)
    }

    fn coerce_bool(&self, value: bool) -> Self::Coerced {
        Node::new(Value::Bool(value), self.span)
    }

    fn coerce_int(&self, value: i64) -> Self::Coerced {
        Node::new(Value::Int(Integer::I64(value)), self.span)
    }

    fn coerce_str(&self, value: String) -> Self::Coerced {
        Node::new(Value::String(Cow::Owned(value)), self.span)
    }

    fn coerce_sequence(&self, items: Vec<Self::Coerced>) -> Self::Coerced {
        Node::new(Value::Sequence(items), self.span)
    }

    fn coerce_mapping(&self, items: Vec<(String, Self::Coerced)>) -> Self::Coerced {
        let pairs = items
            .into_iter()
            .map(|(key, value)| (Node::new(Value::String(Cow::Owned(key)), value.span), value))
            .collect();
        Node::new(Value::Mapping(pairs), self.span)
    }

    fn clone_to_coerced(&self) -> Self::Coerced {
        self.clone().into_owned()
    }
}

pub struct NodeMapping<'a, 'input> {
    pairs: &'a [(Node<'input>, Node<'input>)],
}

impl<'a, 'input: 'a> ValidatableMapping<'a> for NodeMapping<'a, 'input> {
    type Value = Node<'input>;
    type Iter = NodeMappingIter<'a, 'input>;

    fn get(&self, key: &str) -> Option<&Self::Value> {
        for (k, v) in self.pairs {
            if let Value::String(k_str) = &k.value
                && k_str == key
            {
                return Some(v);
            }
        }
        None
    }

    fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    fn iter(&self) -> Self::Iter {
        NodeMappingIter {
            inner: self.pairs.iter(),
        }
    }

    fn len(&self) -> usize {
        self.pairs.len()
    }
}

pub struct NodeMappingIter<'a, 'input> {
    inner: std::slice::Iter<'a, (Node<'input>, Node<'input>)>,
}

impl<'a, 'input: 'a> Iterator for NodeMappingIter<'a, 'input> {
    type Item = (Cow<'a, str>, &'a Node<'input>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (key, value) = self.inner.next()?;
            if let Value::String(key_str) = &key.value {
                return Some((Cow::Borrowed(key_str.as_ref()), value));
            }
        }
    }
}

pub struct NodeSequence<'a, 'input> {
    items: &'a [Node<'input>],
}

impl<'a, 'input: 'a> ValidatableSequence<'a> for NodeSequence<'a, 'input> {
    type Value = Node<'input>;
    type Iter = std::slice::Iter<'a, Node<'input>>;

    fn iter(&self) -> Self::Iter {
        self.items.iter()
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}
