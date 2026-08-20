// Copyright (c) 2025-2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

use avdschema::any::AnySchema;
use avdschema::boolean::Bool;
use avdschema::resolve_ref;

use super::Validation;
use super::handle_invalid_type;
use crate::context::Context;
use crate::context::ValidationState;
use crate::feedback::Type;
use crate::validatable::ValidatableValue;

impl Validation for Bool {
    fn validate<V: ValidatableValue>(&self, value: &V, ctx: &mut Context) -> Option<V::Coerced> {
        validate(self, value, ctx, &mut ValidationState::default())
    }
}

pub(crate) fn validate<V: ValidatableValue>(
    schema: &Bool,
    value: &V,
    ctx: &mut Context,
    state: &mut ValidationState,
) -> Option<V::Coerced> {
    if let Some(maybe_coerced) = validate_ref(schema, value, ctx, state) {
        return maybe_coerced;
    }

    if let Some(boolean) = value.as_bool() {
        // Bool schema has no constraints to validate beyond type checking
        ctx.configuration
            .return_coerced_data
            .then(|| value.coerce_bool(boolean))
    } else {
        handle_invalid_type(value, ctx, state, Type::Bool)
    }
}

/// Validate against a referenced schema (for unresolved $ref ending with #).
fn validate_ref<V: ValidatableValue>(
    schema: &Bool,
    value: &V,
    ctx: &mut Context,
    state: &mut ValidationState,
) -> Option<Option<V::Coerced>> {
    if let Some(ref_) = schema.base.schema_ref.as_ref()
        && let Ok(AnySchema::Bool(ref_schema)) = resolve_ref(ref_, ctx.store)
    {
        return Some(validate(ref_schema, value, ctx, state));
    }
    None
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::*;
    use crate::feedback::Feedback;
    use crate::feedback::Type;
    use crate::feedback::Violation;
    use crate::validation::test_utils::get_test_store;

    #[test]
    fn validate_type_ok() {
        let schema = Bool::default();
        let input: Value = true.into();
        let store = get_test_store();
        let mut ctx = Context::new(&store, None);
        let _ = schema.validate(&input, &mut ctx);
        assert!(ctx.result.errors.is_empty() && ctx.result.infos.is_empty());
    }

    #[test]
    fn validate_type_err() {
        let schema = Bool::default();
        let input = serde_json::json!([]);
        let store = get_test_store();
        let mut ctx = Context::new(&store, None);
        let _ = schema.validate(&input, &mut ctx);
        assert!(ctx.result.infos.is_empty());
        assert_eq!(
            ctx.result.errors,
            vec![Feedback {
                path: vec![].into(),
                span: None,
                issue: Violation::InvalidType {
                    expected: Type::Bool,
                    found: Type::List,
                }
                .into(),
            }],
        );
    }
}
