// Copyright (c) 2025-2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

use avdschema::base::valid_values::ValidValues;

use crate::context::Context;
use crate::context::ValidationState;
use crate::feedback::Violation;
use crate::validatable::ValidatableValue;

pub(crate) trait ValidateValidValues<T: ?Sized> {
    /// Validate that the value is one of the valid values.
    fn validate<V: ValidatableValue>(
        &self,
        source_value: &V,
        input: &T,
        ctx: &mut Context,
        state: &ValidationState,
    );
}

impl ValidateValidValues<i64> for ValidValues<i64> {
    fn validate<V: ValidatableValue>(
        &self,
        source_value: &V,
        input: &i64,
        ctx: &mut Context,
        state: &ValidationState,
    ) {
        if let Some(valid_values) = self.valid_values.as_ref()
            && !valid_values.contains(input)
        {
            ctx.add_error_for(
                state,
                source_value,
                Violation::InvalidValue {
                    expected: valid_values.to_owned().into(),
                    found: input.to_owned().into(),
                },
            );
        }
    }
}

impl ValidateValidValues<str> for ValidValues<String> {
    fn validate<V: ValidatableValue>(
        &self,
        source_value: &V,
        input: &str,
        ctx: &mut Context,
        state: &ValidationState,
    ) {
        if let Some(valid_values) = self.valid_values.as_ref()
            && !valid_values.iter().any(|value| value == input)
        {
            ctx.add_error_for(
                state,
                source_value,
                Violation::InvalidValue {
                    expected: valid_values.to_owned().into(),
                    found: input.to_owned().into(),
                },
            );
        }
    }
}
