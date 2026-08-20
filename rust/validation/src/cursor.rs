// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Borrowed, loader-driven validation cursors.
//!
//! Unlike [`crate::StoreValidate`], cursors do not recursively walk a value.
//! The caller selects fields and list items and therefore defines the validated
//! projection. Scalar validation still uses the regular schema validators.

use avdschema::any::AnySchema;
use avdschema::dict::DictKeyMatch;

use crate::context::Context;
use crate::context::ValidationState;
use crate::feedback::Path;
use crate::feedback::Violation;
use crate::validatable::ValidatableMapping as _;
use crate::validatable::ValidatableSequence as _;
use crate::validatable::ValidatableValue;
use crate::validation::NodeValidation;
use crate::validation::any;
use crate::validation::dict;
use crate::validation::list;

/// A raw value paired with the schema and path for that exact value.
#[derive(Clone)]
pub struct ValidationCursor<'data, 'schema, V> {
    value: &'data V,
    schema: &'schema AnySchema,
    path: Vec<String>,
}

impl<'data, 'schema, V: ValidatableValue> ValidationCursor<'data, 'schema, V> {
    #[must_use]
    pub fn new(value: &'data V, schema: &'schema AnySchema) -> Self {
        Self {
            value,
            schema,
            path: Vec::new(),
        }
    }

    /// Return a cursor for one schema-recognized mapping field.
    ///
    /// Absence is not an error here. Generated loaders decide whether the field
    /// is required by calling [`required_field`](Self::required_field).
    pub fn field(
        &self,
        key: &str,
        context: &Context<'_>,
    ) -> Option<ValidationCursor<'data, 'schema, V>> {
        let AnySchema::Dict(dict_schema) = self.schema else {
            return None;
        };
        let value = self.value.get(key)?;
        let child_schema = if let Some(static_schema) = dict_schema
            .keys
            .as_ref()
            .and_then(|static_keys| static_keys.get(key))
        {
            static_schema
        } else {
            let mapping = self.value.as_mapping()?;
            let resolved_keys = dict_schema.resolve_dict_keys(
                mapping.as_schema_data_mapping(),
                context.configuration.dynamic_key_overrides.as_deref(),
            );
            match resolved_keys.resolve(key) {
                DictKeyMatch::Dynamic(info) => info.schema,
                DictKeyMatch::Static(_) | DictKeyMatch::UnknownKey => return None,
            }
        };
        let mut path = self.path.clone();
        path.push(key.to_owned());
        Some(Self {
            value,
            schema: child_schema,
            path,
        })
    }

    /// Return a field cursor or report the schema-required key as missing.
    pub fn required_field(
        &self,
        key: &str,
        context: &mut Context<'_>,
    ) -> Option<ValidationCursor<'data, 'schema, V>> {
        if let Some(field) = self.field(key, context) {
            return Some(field);
        }
        let state = self.validation_state();
        context.add_error_for(
            &state,
            self.value,
            Violation::MissingRequiredKey {
                key: key.to_owned(),
            },
        );
        None
    }

    /// Validate this value as a mapping without visiting any children.
    pub fn mapping(&self, context: &mut Context<'_>) -> bool {
        let AnySchema::Dict(_) = self.schema else {
            return false;
        };
        matches!(
            dict::validate_node(self.value, context, &mut self.validation_state()),
            NodeValidation::Valid(_)
        )
    }

    /// Validate this value as a sequence and return a lazy view over its items.
    pub fn sequence(
        &self,
        context: &mut Context<'_>,
    ) -> Option<ValidationSequence<'data, 'schema, V>> {
        let AnySchema::List(schema) = self.schema else {
            return None;
        };
        let values =
            match list::validate_node(schema, self.value, context, &mut self.validation_state()) {
                NodeValidation::Valid(values) => values,
                NodeValidation::Null | NodeValidation::Invalid => return None,
            };
        let item_schema = schema.items.as_deref()?;
        Some(ValidationSequence {
            values,
            item_schema,
            path: self.path.clone(),
        })
    }

    /// Run the existing validator for one requested scalar value.
    pub fn scalar(&self, context: &mut Context<'_>) -> Option<V::Coerced> {
        if matches!(self.schema, AnySchema::Dict(_) | AnySchema::List(_)) {
            return None;
        }
        any::validate(
            self.schema,
            self.value,
            context,
            &mut self.validation_state(),
        )
    }

    fn validation_state(&self) -> ValidationState {
        ValidationState::with_path(self.path.iter().map(String::as_str).collect::<Path>())
    }
}

/// Schema-aware lazy view over a requested sequence.
pub struct ValidationSequence<'data, 'schema, V: ValidatableValue + 'data> {
    values: V::Sequence<'data>,
    item_schema: &'schema AnySchema,
    path: Vec<String>,
}

impl<'data, 'schema, V: ValidatableValue + 'data> ValidationSequence<'data, 'schema, V> {
    /// Yield cursors without validating or copying the items.
    pub fn iter(&self) -> impl Iterator<Item = ValidationCursor<'data, 'schema, V>> + '_ {
        self.values.iter().enumerate().map(|(index, value)| {
            let mut path = self.path.clone();
            path.push(index.to_string());
            ValidationCursor {
                value,
                schema: self.item_schema,
                path,
            }
        })
    }
}
