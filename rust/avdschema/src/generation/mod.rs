// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Schema occurrence graph and artifact generators.

mod graph;
mod legacy_python;

pub use self::graph::Occurrence;
pub use self::graph::OccurrenceId;
pub use self::graph::OccurrenceKind;
pub use self::graph::SchemaGraph;
pub use self::legacy_python::GenerationError;
pub use self::legacy_python::generate_python_models;
pub use self::legacy_python::generate_python_models_projection;
