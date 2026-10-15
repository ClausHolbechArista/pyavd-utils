// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Immutable validated data, checked archive ownership, and generated typed views.
//!
//! Publication stores one coerced document in flat value and relationship tables. Views borrow
//! those tables without materializing collections. Every generated model shares a descriptor
//! across validation modes; mode transitions affect accessor types, not archive layout.
//!
//! Structural validation checks scalar types and list primary keys in both modes. [`Validated`]
//! additionally promises present, non-null required fields; [`RelaxedValidated`] permits absent
//! and null ordinary required fields. This is a requiredness contract, not a guarantee that an
//! arbitrary cross-field or application-specific validation has run.
//! Lists allowing duplicate primary keys retain positional access and guaranteed key fields,
//! but do not publish a key lookup table or claim unique-key lookup semantics.
//!
//! ```
//! use validated_data::{data_view, Field, RequiredValue, Validated, RelaxedValidated};
//!
//! #[data_view]
//! pub struct Device<'a, Mode> {
//!     pub name: RequiredValue<&'a str, Mode>,
//!     pub enabled: Field<bool>,
//!     #[data_view(relaxed)]
//!     pub structured_config: Field<Config<'a, RelaxedValidated>>,
//! }
//!
//! #[data_view]
//! pub struct Config<'a, Mode> {
//!     pub description: RequiredValue<&'a str, Mode>,
//! }
//!
//! fn strict_name<'a>(device: Device<'a, Validated>) -> &'a str {
//!     device.name().get()
//! }
//! fn partial_name<'a>(device: Device<'a, RelaxedValidated>) -> Field<&'a str> {
//!     device.name()
//! }
//! ```
//!
//! [`DataStore::root_as`] checks the registered root and the archive's actual shape before
//! constructing typed views. Descendants borrow that checked backing store; accessors do not
//! repeat the walk or create owned scalar/collection copies. Optional fields distinguish
//! [`Field::Unset`] from [`Field::Null`]. A schema default does not erase either state: these views
//! expose the published data and do not insert defaults on access.

extern crate self as validated_data;

mod modes;
mod store;
mod views;

pub use self::modes::*;
pub use self::store::*;
pub use self::views::*;
pub use validated_data_macros::data_view;

#[cfg(test)]
mod tests;
