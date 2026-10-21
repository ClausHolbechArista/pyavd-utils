// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Native Python access to immutable data through generated view descriptors.
//!
//! Data-view attributes derive Python names and exposed fields alongside Rust model descriptors.
//! Root-driven registration creates distinct Python types, while native descriptors and collection operations
//! share their compiled implementation. There is no generated Python executable source and
//! no per-field `PyO3` monomorphization. The schema's borrowed, mode-parameterized Rust views
//! remain separate: Python objects own archive handles rather than extending Rust borrows.
//!
//! Registration is interpreter-local. Descriptors cache child types and the undefined singleton;
//! their Python references participate in cyclic garbage collection. Values are converted only
//! when accessed. Child wrappers retain the mmap owner and do not copy collections.
//! A schema-backed root also retains its schema archive: absent fields borrow their schema
//! defaults, or expose empty collection views and `None` for scalars without defaults.
//! Explicit null suppresses fallback. This affects Python access only; the host archive and
//! raw Rust views retain supplied values for inheritance. `_is_defined(name)` tests for a supplied
//! non-null field, while `_is_default(name)` identifies fallback rather than value equality.
//! Dynamic payloads remain at their original keys and are exposed lazily through typed
//! `_dynamic_keys` selector groups, using the same selector resolution as validation.
//!
//! Positional lists implement the read-only Python sequence protocol. Indexed lists instead
//! index and test membership by primary key; they iterate values and support positional
//! `count`, `index`, and reverse iteration without claiming to be sequences. Dictionary models
//! expose schema attributes, not the general Python mapping protocol. Their truth value reflects
//! whether the archive contains any keys, including custom keys.
//!
//! Model equality requires the same generated Python type and compares archived contents,
//! including custom data. Dictionary order is ignored and list order is significant; booleans
//! and integers are distinct in structural comparisons. These immutable models are unhashable.
//! Equality between different archive nodes can traverse the entire compared subtree, but does
//! not materialize it in Python. Sequence searches follow Python scalar comparison semantics.

mod catalog;
mod collections;
mod descriptors;
mod dynamic;
mod values;

extern crate self as data_views_py;

pub use avdschema::SchemaId;
pub use catalog::{FieldBinding, ModelBinding, PythonModel, Target, install_models};
pub use descriptors::wrap_with_schema;
pub use descriptors::{PyValueHandle, wrap_named};
pub use values::ViewHandle;

#[cfg(test)]
mod default_tests;
#[cfg(test)]
mod tests;
