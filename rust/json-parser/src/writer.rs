// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! AST-based JSON writer.
//!
//! This module writes strict compact JSON text from the crate's `Node` / `Value`
//! AST. It is intentionally conservative:
//! - output is always valid JSON
//! - output is deterministic and compact
//! - original formatting is not preserved
//! - unsupported AST states created manually are rejected with `WriteError`

use std::io::{self, Write};

use derive_more::Display;

use crate::{Integer, Node, Value};

/// Error type for writing JSON text from the AST.
#[derive(Debug, Display)]
pub enum WriteError {
    /// I/O error while emitting JSON.
    #[display("I/O error while writing JSON: {}", _0)]
    Io(io::Error),

    /// Non-finite float values cannot be represented in strict JSON.
    #[display("unsupported floating-point value {}", _0)]
    UnsupportedFloat(f64),

    /// JSON object keys must be strings.
    #[display("object key must be a string")]
    NonStringKey,

    /// The AST contained a bigint string that is not valid JSON integer text.
    #[display("invalid bigint text for JSON integer: {}", _0)]
    InvalidBigInt(String),
}

impl std::error::Error for WriteError {}

impl From<io::Error> for WriteError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

/// Write a JSON AST node as compact strict JSON.
pub fn write_json<W>(mut writer: W, node: &Node<'_>) -> Result<(), WriteError>
where
    W: Write,
{
    write_value(&mut writer, &node.value)
}

/// Write a JSON value as compact strict JSON.
pub fn write_json_value<W>(mut writer: W, value: &Value<'_>) -> Result<(), WriteError>
where
    W: Write,
{
    write_value(&mut writer, value)
}

fn write_value<W>(writer: &mut W, value: &Value<'_>) -> Result<(), WriteError>
where
    W: Write,
{
    match value {
        Value::Null => writer.write_all(b"null")?,
        Value::Bool(bool_value) => {
            if *bool_value {
                writer.write_all(b"true")?;
            } else {
                writer.write_all(b"false")?;
            }
        }
        Value::Int(integer) => write_integer(writer, integer)?,
        Value::Float(float_value) => write_float(writer, *float_value)?,
        Value::String(string_value) => write_string(writer, string_value.as_ref())?,
        Value::Sequence(items) => {
            writer.write_all(b"[")?;
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    writer.write_all(b",")?;
                }
                write_value(writer, &item.value)?;
            }
            writer.write_all(b"]")?;
        }
        Value::Mapping(pairs) => {
            writer.write_all(b"{")?;
            for (index, (key, pair_value)) in pairs.iter().enumerate() {
                if index > 0 {
                    writer.write_all(b",")?;
                }
                let Value::String(key_text) = &key.value else {
                    return Err(WriteError::NonStringKey);
                };
                write_string(writer, key_text.as_ref())?;
                writer.write_all(b":")?;
                write_value(writer, &pair_value.value)?;
            }
            writer.write_all(b"}")?;
        }
    }
    Ok(())
}

fn write_integer<W>(writer: &mut W, integer: &Integer<'_>) -> Result<(), WriteError>
where
    W: Write,
{
    match integer {
        Integer::I64(value) => write!(writer, "{value}")?,
        Integer::U64(value) => write!(writer, "{value}")?,
        Integer::I128(value) => write!(writer, "{value}")?,
        Integer::U128(value) => write!(writer, "{value}")?,
        Integer::BigIntStr(text) => {
            let text_ref = text.as_ref();
            if !is_valid_json_integer(text_ref) {
                return Err(WriteError::InvalidBigInt(text_ref.to_owned()));
            }
            writer.write_all(text_ref.as_bytes())?;
        }
    }
    Ok(())
}

fn write_float<W>(writer: &mut W, value: f64) -> Result<(), WriteError>
where
    W: Write,
{
    if !value.is_finite() {
        return Err(WriteError::UnsupportedFloat(value));
    }

    let text = value.to_string();
    if text.contains(['.', 'e', 'E']) {
        writer.write_all(text.as_bytes())?;
    } else if value == 0.0 && value.is_sign_negative() {
        writer.write_all(b"-0.0")?;
    } else {
        write!(writer, "{text}.0")?;
    }
    Ok(())
}

fn write_string<W>(writer: &mut W, value: &str) -> Result<(), WriteError>
where
    W: Write,
{
    writer.write_all(b"\"")?;
    for ch in value.chars() {
        match ch {
            '"' => writer.write_all(b"\\\"")?,
            '\\' => writer.write_all(b"\\\\")?,
            '\u{08}' => writer.write_all(b"\\b")?,
            '\u{0C}' => writer.write_all(b"\\f")?,
            '\n' => writer.write_all(b"\\n")?,
            '\r' => writer.write_all(b"\\r")?,
            '\t' => writer.write_all(b"\\t")?,
            control if control.is_control() => write!(writer, "\\u{:04X}", u32::from(control))?,
            other => {
                let mut buf = [0; 4];
                writer.write_all(other.encode_utf8(&mut buf).as_bytes())?;
            }
        }
    }
    writer.write_all(b"\"")?;
    Ok(())
}

fn is_valid_json_integer(text: &str) -> bool {
    if text.is_empty() {
        return false;
    }

    let digits = if let Some(rest) = text.strip_prefix('-') {
        if rest.is_empty() {
            return false;
        }
        rest
    } else {
        text
    };

    if digits == "0" {
        return true;
    }

    let Some(first) = digits.as_bytes().first() else {
        return false;
    };
    if *first == b'0' || !first.is_ascii_digit() {
        return false;
    }

    digits.as_bytes().iter().all(u8::is_ascii_digit)
}
