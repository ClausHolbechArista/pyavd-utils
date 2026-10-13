// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Native Python access to immutable validated-data archives.
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

extern crate self as validated_data_py;

pub use catalog::{FieldBinding, ModelBinding, PythonModel, Target, install_models};
pub use descriptors::{PyValueHandle, wrap_named};

#[cfg(test)]
mod tests;
