// Copyright (c) 2025-2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

use avdschema::any::AnySchema;

use super::Validation;
use crate::context::Context;
use crate::context::ValidationState;
use crate::validatable::ValidatableValue;

impl Validation for AnySchema {
    fn validate<V: ValidatableValue>(&self, value: &V, ctx: &mut Context) -> Option<V::Coerced> {
        validate(self, value, ctx, &mut ValidationState::default())
    }
}

/// Dispatch validation for a schema node while preserving traversal state.
pub(crate) fn validate<V: ValidatableValue>(
    schema: &AnySchema,
    value: &V,
    context: &mut Context,
    state: &mut ValidationState,
) -> Option<V::Coerced> {
    match schema {
        AnySchema::Bool(schema) => super::boolean::validate(schema, value, context, state),
        AnySchema::Int(schema) => super::int::validate(schema, value, context, state),
        AnySchema::Str(schema) => super::str::validate(schema, value, context, state),
        AnySchema::List(schema) => super::list::validate(schema, value, context, state),
        AnySchema::Dict(schema) => super::dict::validate(schema, value, context, state),
    }
}
