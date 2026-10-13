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

mod catalog;
mod collections;
mod descriptors;

extern crate self as validated_data_py;

pub use catalog::{FieldBinding, ModelBinding, PythonModel, Target, install_models};
pub use descriptors::{PyValueHandle, wrap_named};

#[cfg(test)]
mod tests;
