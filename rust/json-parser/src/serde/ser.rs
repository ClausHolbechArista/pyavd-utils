// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Serde-based JSON serialization for `json-parser`.
//!
//! This module builds a JSON AST (`Node` / `Value`) from any `T: Serialize`
//! and then writes strict compact JSON using the AST writer.

use std::fmt;
use std::io::{self, Write};

use serde::ser::Error as _;
use serde::ser::{
    self, Serialize, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant,
    SerializeTuple, SerializeTupleStruct, SerializeTupleVariant, Serializer,
};

use crate::writer::{self, WriteError};
use crate::{Integer, Node, Span, Value};

/// Error type for serde-based JSON serialization.
#[derive(Debug, derive_more::Display)]
pub enum SerError {
    /// I/O error while writing JSON.
    #[display("I/O error while writing JSON: {}", _0)]
    Io(io::Error),

    /// Non-finite float values (`NaN` or `±inf`) are not valid JSON.
    #[display("unsupported floating-point value {}", _0)]
    UnsupportedFloat(f64),

    /// JSON object keys must be strings.
    #[display("JSON object keys must serialize to strings")]
    NonStringKey,

    /// A bigint string stored in the AST was not valid JSON integer text.
    #[display("invalid bigint text for JSON integer: {}", _0)]
    InvalidBigInt(String),

    /// Generic serde error created via `serde::ser::Error::custom`.
    #[display("serde custom error: {}", _0)]
    Custom(String),
}

impl std::error::Error for SerError {}

impl ser::Error for SerError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self::Custom(msg.to_string())
    }
}

impl From<io::Error> for SerError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<WriteError> for SerError {
    fn from(err: WriteError) -> Self {
        match err {
            WriteError::Io(io_error) => Self::Io(io_error),
            WriteError::UnsupportedFloat(float_value) => Self::UnsupportedFloat(float_value),
            WriteError::NonStringKey => Self::NonStringKey,
            WriteError::InvalidBigInt(text) => Self::InvalidBigInt(text),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct ValueSerializer;

impl ValueSerializer {
    pub(crate) fn to_value<T>(value: &T) -> Result<Value<'static>, SerError>
    where
        T: Serialize,
    {
        value.serialize(Self)
    }
}

impl Serializer for ValueSerializer {
    type Ok = Value<'static>;
    type Error = SerError;

    type SerializeSeq = SeqSerializer;
    type SerializeTuple = SeqSerializer;
    type SerializeTupleStruct = SeqSerializer;
    type SerializeTupleVariant = TupleVariantSerializer;
    type SerializeMap = MapSerializer;
    type SerializeStruct = StructSerializer;
    type SerializeStructVariant = StructVariantSerializer;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Bool(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Int(Integer::I64(i64::from(v))))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Int(Integer::I64(i64::from(v))))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Int(Integer::I64(i64::from(v))))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Int(Integer::I64(v)))
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Int(Integer::I128(v)))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Int(Integer::U64(u64::from(v))))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Int(Integer::U64(u64::from(v))))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Int(Integer::U64(u64::from(v))))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Int(Integer::U64(v)))
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Int(Integer::U128(v)))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        if !v.is_finite() {
            return Err(SerError::UnsupportedFloat(v));
        }
        Ok(Value::Float(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(Value::String(std::borrow::Cow::Owned(v.to_string())))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::String(std::borrow::Cow::Owned(v.to_owned())))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let mut elements = Vec::with_capacity(v.len());
        for &byte in v {
            elements.push(node_from_value(Value::Int(Integer::U64(u64::from(byte)))));
        }
        Ok(Value::Sequence(elements))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Null)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Null)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Null)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(Value::String(std::borrow::Cow::Owned(variant.to_owned())))
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        let key = node_from_value(Value::String(std::borrow::Cow::Owned(variant.to_owned())));
        let inner = node_from_value(value.serialize(ValueSerializer)?);
        Ok(Value::Mapping(vec![(key, inner)]))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SeqSerializer {
            elements: Vec::with_capacity(len.unwrap_or(0)),
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(TupleVariantSerializer {
            name: variant.to_owned(),
            elements: Vec::with_capacity(len),
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(MapSerializer {
            entries: Vec::with_capacity(len.unwrap_or(0)),
            next_key: None,
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(StructSerializer {
            entries: Vec::with_capacity(len),
        })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(StructVariantSerializer {
            name: variant.to_owned(),
            entries: Vec::with_capacity(len),
        })
    }
}

pub(crate) struct SeqSerializer {
    elements: Vec<Node<'static>>,
}

impl SerializeSeq for SeqSerializer {
    type Ok = Value<'static>;
    type Error = SerError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        self.elements
            .push(node_from_value(value.serialize(ValueSerializer)?));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Sequence(self.elements))
    }
}

impl SerializeTuple for SeqSerializer {
    type Ok = Value<'static>;
    type Error = SerError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}

impl SerializeTupleStruct for SeqSerializer {
    type Ok = Value<'static>;
    type Error = SerError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}

pub(crate) struct TupleVariantSerializer {
    name: String,
    elements: Vec<Node<'static>>,
}

impl SerializeTupleVariant for TupleVariantSerializer {
    type Ok = Value<'static>;
    type Error = SerError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        self.elements
            .push(node_from_value(value.serialize(ValueSerializer)?));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let key = node_from_value(Value::String(std::borrow::Cow::Owned(self.name)));
        let val = node_from_value(Value::Sequence(self.elements));
        Ok(Value::Mapping(vec![(key, val)]))
    }
}

pub(crate) struct MapSerializer {
    entries: Vec<(Node<'static>, Node<'static>)>,
    next_key: Option<Node<'static>>,
}

impl SerializeMap for MapSerializer {
    type Ok = Value<'static>;
    type Error = SerError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        let key_value = key.serialize(ValueSerializer)?;
        if !matches!(key_value, Value::String(_)) {
            return Err(SerError::NonStringKey);
        }
        self.next_key = Some(node_from_value(key_value));
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        let Some(key) = self.next_key.take() else {
            return Err(SerError::custom(
                "value without corresponding key in mapping",
            ));
        };
        let val_node = node_from_value(value.serialize(ValueSerializer)?);
        self.entries.push((key, val_node));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if self.next_key.is_some() {
            return Err(SerError::custom(
                "value without corresponding key in mapping",
            ));
        }
        Ok(Value::Mapping(self.entries))
    }
}

pub(crate) struct StructSerializer {
    entries: Vec<(Node<'static>, Node<'static>)>,
}

impl SerializeStruct for StructSerializer {
    type Ok = Value<'static>;
    type Error = SerError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        let key_node = node_from_value(Value::String(std::borrow::Cow::Owned(key.to_owned())));
        let val_node = node_from_value(value.serialize(ValueSerializer)?);
        self.entries.push((key_node, val_node));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Mapping(self.entries))
    }
}

pub(crate) struct StructVariantSerializer {
    name: String,
    entries: Vec<(Node<'static>, Node<'static>)>,
}

impl SerializeStructVariant for StructVariantSerializer {
    type Ok = Value<'static>;
    type Error = SerError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        let key_node = node_from_value(Value::String(std::borrow::Cow::Owned(key.to_owned())));
        let val_node = node_from_value(value.serialize(ValueSerializer)?);
        self.entries.push((key_node, val_node));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let key = node_from_value(Value::String(std::borrow::Cow::Owned(self.name)));
        let val = node_from_value(Value::Mapping(self.entries));
        Ok(Value::Mapping(vec![(key, val)]))
    }
}

fn node_from_value(value: Value<'static>) -> Node<'static> {
    Node::new(value, Span::default())
}

/// Serialize any `T: Serialize` directly to a writer as JSON text.
pub fn to_writer<W, T>(mut writer: W, value: &T) -> Result<(), SerError>
where
    W: Write,
    T: Serialize,
{
    let root_value = ValueSerializer::to_value(value)?;
    let node = node_from_value(root_value);
    writer::write_json(&mut writer, &node)?;
    Ok(())
}

/// Serialize any `T: Serialize` to a JSON string.
pub fn to_string<T>(value: &T) -> Result<String, SerError>
where
    T: Serialize,
{
    let mut buf = Vec::new();
    to_writer(&mut buf, value)?;
    String::from_utf8(buf)
        .map_err(|err| SerError::Io(io::Error::new(io::ErrorKind::InvalidData, err)))
}
