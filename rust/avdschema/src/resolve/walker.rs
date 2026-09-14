// Copyright (c) 2025-2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

use std::iter::Peekable;

use ordermap::OrderMap;

use crate::any::SourceSchema;
use crate::dict::SourceDict;
use crate::list::SourceList;

pub(crate) trait Walker {
    fn walk<'a, I>(&self, path: Peekable<I>) -> Result<&SourceSchema, SchemaWalkError>
    where
        I: Iterator<Item = &'a str> + std::fmt::Debug;
}

impl Walker for SourceList {
    fn walk<'a, I>(&self, mut path: Peekable<I>) -> Result<&SourceSchema, SchemaWalkError>
    where
        I: Iterator<Item = &'a str> + std::fmt::Debug,
    {
        match path.next() {
            Some("items") => match &self.items {
                Some(schema) => schema.walk(path),
                None => Err(SchemaWalkError::PathNotFound {
                    element: "items".into(),
                }),
            },
            Some(value) => Err(SchemaWalkError::PathNotFound {
                element: value.into(),
            }),
            None => Err(SchemaWalkError::Internal),
        }
    }
}

impl Walker for OrderMap<String, SourceSchema> {
    fn walk<'a, I>(&self, mut path: Peekable<I>) -> Result<&SourceSchema, SchemaWalkError>
    where
        I: Iterator<Item = &'a str> + std::fmt::Debug,
    {
        match path.next() {
            Some(key) => match self.get(key) {
                Some(value) => value.walk(path),
                None => Err(SchemaWalkError::PathNotFound {
                    element: key.into(),
                }),
            },
            None => Err(SchemaWalkError::PointingToKeys),
        }
    }
}

impl Walker for SourceDict {
    fn walk<'a, I>(&self, mut path: Peekable<I>) -> Result<&SourceSchema, SchemaWalkError>
    where
        I: Iterator<Item = &'a str> + std::fmt::Debug,
    {
        match path.next() {
            Some("keys") => self.keys.as_ref().map_or_else(
                || {
                    Err(SchemaWalkError::PathNotFound {
                        element: "keys".into(),
                    })
                },
                |keys| keys.walk(path),
            ),
            Some("dynamic_keys") => self.dynamic_keys.as_ref().map_or_else(
                || {
                    Err(SchemaWalkError::PathNotFound {
                        element: "dynamic_keys".into(),
                    })
                },
                |dynamic_keys| dynamic_keys.walk(path),
            ),
            Some("$defs") => self.schema_defs.as_ref().map_or_else(
                || {
                    Err(SchemaWalkError::PathNotFound {
                        element: "$defs".into(),
                    })
                },
                |schema_defs| schema_defs.walk(path),
            ),
            Some(element) => Err(SchemaWalkError::InvalidPathElement {
                element: element.into(),
            }),
            None => Err(SchemaWalkError::Internal),
        }
    }
}

impl Walker for SourceSchema {
    fn walk<'a, I>(&self, mut path: Peekable<I>) -> Result<&SourceSchema, SchemaWalkError>
    where
        I: Iterator<Item = &'a str> + std::fmt::Debug,
    {
        if path.peek().is_none() {
            return Ok(self);
        }
        match self {
            SourceSchema::List(schema) => schema.walk(path),
            SourceSchema::Dict(schema) => schema.walk(path),
            _ => Err(SchemaWalkError::NotDictOrList),
        }
    }
}

/// Structured failure encountered while walking a reference path.
#[derive(Debug, derive_more::Display)]
pub enum SchemaWalkError {
    #[display("Internal error. SourceSchema should have returned the schema.")]
    Internal,
    #[display(
        "Invalid schema path. The element '{element}' is invalid. All path elements except the last must go via lists or dicts."
    )]
    InvalidPathElement { element: String },
    #[display(
        "Invalid schema path. An intermediate element pointed to a schema that is not a dict or list."
    )]
    NotDictOrList,
    #[display("Invalid schema path. The element '{element}' was not found.")]
    PathNotFound { element: String },
    #[display("Invalid schema path. A path can not point to 'keys' of a dict schema.")]
    PointingToKeys,
}
