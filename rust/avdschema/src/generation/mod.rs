// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Schema traversal and artifact generators.

mod legacy_python;
mod nominal;
mod traversal;

pub use self::legacy_python::GenerationError;
pub use self::legacy_python::generate_python_models;
pub use self::legacy_python::generate_python_models_projection;
pub use self::nominal::FieldRelation;
pub use self::nominal::FieldTarget;
pub use self::nominal::ModelId;
pub use self::nominal::ModelKind;
pub use self::nominal::NominalField;
pub use self::nominal::NominalModel;
pub use self::nominal::NominalModelIr;
pub use self::nominal::ScalarKind;
pub use self::nominal::SlotId;
pub use self::nominal::build_nominal_model_ir;
pub use self::nominal::nominal_model_ir_json;
