// Copyright (c) 2025-2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! AVD schema source models and compiled runtime views.
//!
//! [`StoreSource`] and the `Source*` schema types deserialize JSON, YAML, and
//! compressed schema sources. [`Store`] compiles those models into a resolved,
//! deduplicated DAG and is the canonical runtime representation used by
//! validation and schema navigation. Runtime nodes are exposed as borrowed
//! [`SchemaView`] variants and typed child views.
//!
//! [`Store::from_file`] memory-maps compiled archives. [`Store::from_json`] and
//! [`Store::from_gz_bytes`] compile ad-hoc schema sources into process-owned
//! archive bytes for consumers such as language servers. The archived schema data is immutable;
//! only derived regular expressions are cached per process. Dynamic keys are
//! resolved per operation from the input data and are not stored in schema
//! views.
// TODO: Reevaluate the allow
#![allow(
    missing_docs,
    clippy::empty_structs_with_brackets,
    clippy::empty_enum_variants_with_brackets,
    clippy::iter_over_hash_type,
    clippy::impl_trait_in_params,
    clippy::needless_pass_by_value,
    clippy::module_name_repetitions,
    clippy::multiple_inherent_impl,
    clippy::partial_pub_fields,
    clippy::pub_underscore_fields,
    clippy::redundant_type_annotations,
    clippy::used_underscore_binding,
    clippy::unwrap_used,
    clippy::tests_outside_test_module,
    reason = "Existing schema models and feature-gated shared test helpers predate workspace lint inheritance"
)]
#![deny(unused_crate_dependencies)]

#[cfg(test)]
use test_schema_store as _;

mod archive;
mod compiled;
#[cfg(feature = "generation")]
mod generation;
mod inherit;
mod resolve;
mod schema;
mod store;
mod utils;

pub use self::archive::BoolView;
pub use self::archive::CommonView;
pub use self::archive::DeprecationView;
pub use self::archive::DictView;
pub use self::archive::DocumentationOptionsView;
pub use self::archive::IntView;
pub use self::archive::ListView;
pub use self::archive::SchemaListValueView;
pub use self::archive::SchemaObjectValueView;
pub use self::archive::SchemaPathError;
pub use self::archive::SchemaValueView;
pub use self::archive::SchemaView;
pub use self::archive::Store;
pub use self::archive::StoreError;
pub use self::archive::StrView;
pub use self::archive::StringFormatView;
pub use self::archive::resolve_dynamic_keys;
pub use self::compiled::CompileError;
pub use self::compiled::SchemaDiagnostic;
pub use self::compiled::SchemaDiagnostics;
pub use self::compiled::SchemaId;
#[cfg(feature = "generation")]
pub use self::generation::GenerationError;
#[cfg(feature = "generation")]
pub use self::generation::generate_python_models;
#[cfg(feature = "generation")]
pub use self::generation::generate_python_models_projection;
pub use self::inherit::Inherit;
pub use self::resolve::errors::SchemaResolverError;
pub use self::resolve::walker::SchemaWalkError;
pub use self::schema::any;
pub use self::schema::base;
pub use self::schema::boolean;
pub use self::schema::dict;
pub use self::schema::dict::DynamicKeyOverrides;
pub use self::schema::int;
pub use self::schema::list;
pub use self::schema::str;
pub use self::store::SchemaStoreError;
pub use self::store::StoreSource;
pub use self::utils::dump::Dump;
pub use self::utils::dump::DumpError;
pub use self::utils::load::Load;
pub use self::utils::load::LoadError;
#[cfg(feature = "dump_load_files")]
pub use self::utils::load::LoadFromFragments;
pub use self::utils::schema_data::SchemaDataMapping;
pub use self::utils::schema_data::SchemaDataSequence;
pub use self::utils::schema_data::SchemaDataValue;
