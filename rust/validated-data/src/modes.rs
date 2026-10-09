// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Presence guarantees for structurally checked, immutable data.
//!
//! Both modes guarantee scalar types and collection structure. Validated mode additionally
//! enforces required-key presence and non-nullability. Relaxed mode propagates through descendants;
//! a child accessor declaring relaxed validation changes the mode without copying its backing data.

mod sealed {
    pub trait Sealed {}
}

/// A field's explicit state. Absence and null remain distinct during merging.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Field<T> {
    /// No value was supplied for this relationship.
    Unset,
    /// Explicit null was supplied.
    Null,
    /// A structurally checked value.
    Value(T),
}

impl<T> Field<T> {
    /// Transform a present value without changing unset or null states.
    pub fn map<U, F: FnOnce(T) -> U>(self, transform: F) -> Field<U> {
        match self {
            Self::Unset => Field::Unset,
            Self::Null => Field::Null,
            Self::Value(value) => Field::Value(transform(value)),
        }
    }
}

/// A present, non-null value established by validation or indexed-list membership.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guaranteed<T>(T);

impl<T> Guaranteed<T> {
    /// Access the guaranteed value without inspecting presence states.
    pub fn get(self) -> T {
        self.0
    }

    pub(crate) fn from_field(field: Field<T>) -> Self {
        match field {
            Field::Value(value) => Self(value),
            Field::Unset | Field::Null => invariant_failure(),
        }
    }
}

/// Required fields are present and non-null; optional fields retain their three states.
#[derive(Clone, Copy, Debug)]
pub struct Validated;

/// Types are checked, but ordinary required fields may be unset or null.
#[derive(Clone, Copy, Debug)]
pub struct RelaxedValidated;

impl sealed::Sealed for Validated {}
impl sealed::Sealed for RelaxedValidated {}

/// Closed set of validation contracts understood by generated accessors.
pub trait ValidationMode: sealed::Sealed + Copy + std::fmt::Debug {
    /// Whether descendant required-key checks are relaxed.
    const RELAXED: bool;
    /// Return representation of a schema-required field.
    type Required<T>;
    /// Convert a structurally checked field into this mode's presence representation.
    fn required<T>(field: Field<T>) -> Self::Required<T>;
}

impl ValidationMode for Validated {
    const RELAXED: bool = false;
    type Required<T> = Guaranteed<T>;

    fn required<T>(field: Field<T>) -> Guaranteed<T> {
        Guaranteed::from_field(field)
    }
}

impl ValidationMode for RelaxedValidated {
    const RELAXED: bool = true;
    type Required<T> = Field<T>;

    fn required<T>(field: Field<T>) -> Field<T> {
        field
    }
}

/// Mode-dependent representation of a required field.
///
/// In validated mode this is an infallible wrapper. In relaxed mode it is a three-state field.
pub type RequiredValue<T, Mode> = <Mode as ValidationMode>::Required<T>;

#[allow(
    clippy::panic,
    reason = "checked model construction makes absent guaranteed values a programming invariant violation"
)]
pub(crate) fn invariant_failure() -> ! {
    panic!("checked validated-data view invariant violated")
}
