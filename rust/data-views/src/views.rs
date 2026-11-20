// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Checked nominal views and field conversion.
//!
//! A typed root is checked once against its generated descriptor. Child views retain that proof.
//! A generic value handle cannot be promoted merely because its value is a dictionary or list.

use crate::ArchiveError;
use crate::ArchiveModel;
use crate::DataStore;
use crate::Field;
use crate::FieldRelation;
use crate::Guaranteed;
use crate::ModelDescriptor;
use crate::PrimaryKeyValue;
use crate::ScalarType;
use crate::ValidationMode;
use crate::ValueView;
use crate::modes::invariant_failure;
use std::marker::PhantomData;

/// Generated model whose backing value has passed its structural and mode-specific checks.
pub trait DataView<'a>: ArchiveModel + Sized {
    /// Requiredness contract carried by this model.
    type Mode: ValidationMode;
    /// Construct a wrapper from a proof tied to this exact nominal model and mode.
    fn from_checked(value: CheckedModel<'a, Self>) -> Self;
}

/// A model-specific proof and borrowed value. Only checked entry points can create this token.
pub struct CheckedModel<'a, Model: DataView<'a>> {
    value: ValueView<'a>,
    model: PhantomData<Model>,
}

impl<'a, Model: DataView<'a>> Copy for CheckedModel<'a, Model> {}
impl<'a, Model: DataView<'a>> Clone for CheckedModel<'a, Model> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a, Model: DataView<'a>> std::fmt::Debug for CheckedModel<'a, Model> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CheckedModel").field(&self.value).finish()
    }
}

#[allow(
    clippy::multiple_inherent_impl,
    reason = "Keep typed-root checks beside the checked-view implementation."
)]
impl DataStore {
    /// Check a root against a generated model and expose its requested validation contract.
    ///
    /// Schema/registry/policy compatibility is checked by file opening. This additional walk
    /// verifies the actual field shapes, required values, and list primary keys used by accessors.
    pub fn root_as<'a, Model: DataView<'a>>(&'a self) -> Result<Model, ArchiveError> {
        if self.root_model_identity() != Model::DESCRIPTOR.identity {
            return Err(ArchiveError::Compatibility(
                "requested model is not the registered root".into(),
            ));
        }
        check_model(self.root(), &Model::DESCRIPTOR, Model::Mode::RELAXED)?;
        Ok(Model::from_checked(CheckedModel {
            value: self.root(),
            model: PhantomData,
        }))
    }
}

impl<'a, Model: DataView<'a>> CheckedModel<'a, Model> {
    /// Read a statically named scalar field without materializing it.
    pub fn scalar<T: Scalar<'a>>(self, slot: u32) -> Field<T> {
        decode(self.value.as_dict().and_then(|dict| dict.field(slot)))
    }

    /// Read a collection field, preserving its generated mode transition.
    pub fn child<Child: DataView<'a>>(self, slot: u32) -> Field<Child> {
        Self::check_target::<Child>(slot);
        self.value
            .as_dict()
            .and_then(|dict| dict.field(slot))
            .map_or(Field::Unset, |value| {
                if value.is_null() {
                    Field::Null
                } else {
                    Field::Value(Child::from_checked(CheckedModel {
                        value,
                        model: PhantomData,
                    }))
                }
            })
    }

    /// Number of relationships in a checked list.
    pub fn len(self) -> usize {
        self.value.as_list().map_or(0, crate::store::ListView::len)
    }
    /// Whether a checked list has no items.
    pub fn is_empty(self) -> bool {
        self.len() == 0
    }
    /// Read one collection item, distinguishing out-of-range from a null item.
    pub fn item<Child: DataView<'a>>(self, index: usize) -> Option<Field<Child>> {
        Self::check_target::<Child>(0);
        self.value.as_list()?.get(index).map(|value| {
            if value.is_null() {
                Field::Null
            } else {
                Field::Value(Child::from_checked(CheckedModel {
                    value,
                    model: PhantomData,
                }))
            }
        })
    }
    /// Read one scalar item, distinguishing out-of-range from explicit null.
    pub fn scalar_item<T: Scalar<'a>>(self, index: usize) -> Option<Field<T>> {
        self.value
            .as_list()?
            .get(index)
            .map(|value| decode(Some(value)))
    }
    /// Find a collection item by the indexed list's ordered primary-key components.
    pub fn indexed_item<Child: DataView<'a>>(self, key: &[PrimaryKeyValue<'_>]) -> Option<Child> {
        Self::check_target::<Child>(0);
        let value = self.value.as_list()?.get_by_primary_key(key)?;
        Some(Child::from_checked(CheckedModel {
            value,
            model: PhantomData,
        }))
    }
    /// Read an untyped list item when the schema intentionally declares no item model.
    pub fn raw_item(self, index: usize) -> Option<ValueView<'a>> {
        self.value.as_list()?.get(index)
    }

    fn check_target<Child: DataView<'a>>(slot: u32) {
        let field = Model::DESCRIPTOR
            .fields
            .iter()
            .find(|field| field.id == slot)
            .unwrap_or_else(|| invariant_failure());
        if field
            .target_model
            .is_none_or(|target| target.identity != Child::DESCRIPTOR.identity)
            || Child::Mode::RELAXED != (Model::Mode::RELAXED || field.relaxed)
        {
            invariant_failure();
        }
    }
}

mod sealed {
    pub trait Sealed {}
}
/// Supported scalar conversion from checked archive values.
pub trait Scalar<'a>: sealed::Sealed + Sized {
    /// Decode a scalar of the matching generated schema type.
    fn decode(value: ValueView<'a>) -> Option<Self>;
}
impl sealed::Sealed for bool {}
impl sealed::Sealed for i64 {}
impl sealed::Sealed for &str {}
impl<'a> Scalar<'a> for bool {
    fn decode(value: ValueView<'a>) -> Option<Self> {
        value.as_bool()
    }
}
impl<'a> Scalar<'a> for i64 {
    fn decode(value: ValueView<'a>) -> Option<Self> {
        value.as_i64()
    }
}
impl<'a> Scalar<'a> for &'a str {
    fn decode(value: ValueView<'a>) -> Option<Self> {
        value.as_str()
    }
}

fn decode<'a, T: Scalar<'a>>(value: Option<ValueView<'a>>) -> Field<T> {
    match value {
        None => Field::Unset,
        Some(value) if value.is_null() => Field::Null,
        Some(value) => Field::Value(T::decode(value).unwrap_or_else(|| invariant_failure())),
    }
}

/// Generated field access used to expose a list's primary-key guarantees on its item wrapper.
pub trait KeyField<'a, const NAME: u64> {
    /// Structurally validated scalar type of this field.
    type Value;
    /// Read the scalar while preserving presence states.
    fn key_field(&self) -> Field<Self::Value>;
    /// Slot assigned by the item declaration's field order.
    const SLOT: u32;
}

/// An item reached through a keyed list. Its wrapper exposes key-presence guarantees, not uniqueness.
#[derive(Clone, Copy, Debug)]
pub struct KeyedItem<T>(T);
impl<T> KeyedItem<T> {
    /// Wrap an item returned by a checked list with primary-key presence guarantees.
    ///
    /// This wrapper alone does not promote fields: guaranteed access verifies their state.
    pub fn new(item: T) -> Self {
        Self(item)
    }
    /// Read one guaranteed primary-key scalar using generated field metadata.
    pub fn key<'a, const NAME: u64>(&self) -> Guaranteed<<T as KeyField<'a, NAME>>::Value>
    where
        T: KeyField<'a, NAME>,
    {
        Guaranteed::from_field(self.0.key_field())
    }
}
impl<T> std::ops::Deref for KeyedItem<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn check_model(
    value: ValueView<'_>,
    model: &ModelDescriptor,
    relaxed: bool,
) -> Result<(), ArchiveError> {
    if model.kind == crate::ModelKind::List && value.as_list().is_none()
        || model.kind == crate::ModelKind::Dict && value.as_dict().is_none()
    {
        return Err(ArchiveError::Invalid(
            "value does not match the generated collection shape".into(),
        ));
    }
    if let Some(item) = model
        .fields
        .iter()
        .find(|field| field.relation == FieldRelation::Item)
    {
        let list = value
            .as_list()
            .ok_or_else(|| ArchiveError::Invalid("expected list model".into()))?;
        for index in 0..list.len() {
            let child = list
                .get(index)
                .ok_or_else(|| ArchiveError::Invalid("invalid list item".into()))?;
            check_field(child, item, relaxed)?;
            if !model.primary_key_fields.is_empty() {
                let dict = child.as_dict().ok_or_else(|| {
                    ArchiveError::Invalid("primary-key list item is not a dictionary".into())
                })?;
                for slot in model.primary_key_fields {
                    if dict.field(*slot).is_none_or(ValueView::is_null) {
                        return Err(ArchiveError::Invalid(
                            "list item has an unset or null primary key".into(),
                        ));
                    }
                }
            }
        }
        if model.indexed {
            list.check_index(model.primary_key_fields)?;
        }
    } else if let Some(dict) = value.as_dict() {
        for field in model.fields {
            let FieldRelation::Key(key) = field.relation else {
                continue;
            };
            if let Some(child) = dict.field(field.id) {
                if dict.key(key).is_none_or(|named| !named.same_node(child)) {
                    return Err(ArchiveError::Invalid(
                        "field slot does not match its schema key".into(),
                    ));
                }
                check_field(child, field, relaxed)?;
            } else if field.required && !relaxed {
                return Err(ArchiveError::Invalid("required field is unset".into()));
            }
        }
    } else if value.as_list().is_none() || !model.fields.is_empty() {
        return Err(ArchiveError::Invalid("expected dictionary model".into()));
    }
    Ok(())
}

fn check_field(
    value: ValueView<'_>,
    field: &crate::FieldDescriptor,
    relaxed: bool,
) -> Result<(), ArchiveError> {
    if value.is_null() {
        return if field.required && !relaxed {
            Err(ArchiveError::Invalid("required field is null".into()))
        } else {
            Ok(())
        };
    }
    if let Some(child) = field.target_model {
        return check_model(value, child, relaxed || field.relaxed);
    }
    let valid = match field.scalar_type {
        Some(ScalarType::Bool) => value.as_bool().is_some(),
        Some(ScalarType::Int) => value.as_i64().is_some(),
        Some(ScalarType::Str) => value.as_str().is_some(),
        None => true,
    };
    if valid {
        Ok(())
    } else {
        Err(ArchiveError::Invalid(
            "field has the wrong scalar type".into(),
        ))
    }
}
