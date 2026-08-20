// Copyright (c) 2025-2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

pub(crate) mod any;
pub(crate) mod boolean;
pub(crate) mod dict;
pub(crate) mod int;
pub(crate) mod list;
pub(crate) mod store;
pub(crate) mod str;
pub(crate) mod valid_values;

use crate::context::Context;
use crate::context::ValidationState;
use crate::feedback::Type;
use crate::feedback::Violation;
use crate::validatable::ValidatableValue;

/// Trait for validating values against a schema.
///
/// Accepts any type implementing [`ValidatableValue`], allowing validation
/// to stay decoupled from a specific input representation.
///
/// Optionally returns the coerced value with types corrected based on schema expectations.
/// For example, a string "123" validated against an Int schema returns an Int value.
/// When `ctx.configuration.return_coerced_data` is false, returns `None` to avoid allocations.
pub trait Validation {
    /// Validate any value implementing [`ValidatableValue`] against this schema.
    ///
    /// This method validates the value and optionally returns a coerced version with types
    /// adjusted based on the schema while remaining generic over the input type.
    ///
    /// Returns `Some(coerced)` when `ctx.configuration.return_coerced_data` is true,
    /// `None` otherwise (to avoid expensive allocations in validation-only use cases).
    ///
    /// The coerced value preserves metadata exposed by the original value type.
    fn validate<V: ValidatableValue>(&self, value: &V, ctx: &mut Context) -> Option<V::Coerced>;

    fn handle_invalid_type<V: ValidatableValue>(
        value: &V,
        ctx: &mut Context,
        expected: Type,
    ) -> Option<V::Coerced> {
        handle_invalid_type(value, ctx, &ValidationState::default(), expected)
    }
}

pub(crate) fn handle_invalid_type<V: ValidatableValue>(
    value: &V,
    ctx: &mut Context,
    state: &ValidationState,
    expected: Type,
) -> Option<V::Coerced> {
    if value.is_null() && !ctx.configuration.restrict_null_values {
        // Null is allowed when not restricted
        ctx.configuration
            .return_coerced_data
            .then(|| value.coerce_null())
    } else {
        ctx.add_error_for(
            state,
            value,
            Violation::InvalidType {
                expected,
                found: value.value_type(),
            },
        );
        None
    }
}

/// Outcome of validating one node without recursively validating its children.
///
/// Container validators use this to separate node validation from traversal:
/// a caller may traverse [`Valid`](Self::Valid), preserve an accepted
/// [`Null`](Self::Null), or stop after [`Invalid`](Self::Invalid). Validation
/// diagnostics are added to the [`Context`] before `Invalid` is returned.
pub(crate) enum NodeValidation<T> {
    /// The node has the expected type and may be traversed through this view.
    Valid(T),
    /// Null is accepted because `restrict_null_values` is disabled.
    Null,
    /// The node is invalid and the relevant diagnostic has already been added.
    Invalid,
}

#[cfg(test)]
pub(crate) mod test_utils;
