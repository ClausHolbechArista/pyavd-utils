// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Validate and coerce a document before publishing immutable validated data.
//!
//! Archive storage and views belong to the validated-data crate. This module owns the validation
//! policy and prevents publication when parsing or schema validation reports an error.

use crate::Configuration;
use crate::StoreValidateError;
use crate::StoreValidateInput as _;
use crate::ValidationResult;
use crate::feedback::InputDiagnostic;
use avdschema::Store;
use std::path::Path;
use validated_data::ModelRegistry;

/// Failure during validation or archive publication.
#[derive(Debug, derive_more::Display, derive_more::From)]
pub enum PublicationError {
    /// Schema selection or validation failed.
    #[display("Validation failed: {_0}")]
    Validation(StoreValidateError),
    /// Archive construction or publication failed.
    #[display("{_0}")]
    Archive(validated_data::ArchiveError),
}

/// Result of validation and conditional archive publication.
#[derive(Debug)]
#[allow(
    clippy::module_name_repetitions,
    reason = "The qualified public name is explicit at call sites"
)]
pub struct ArchivePublication {
    /// Parse diagnostics produced before schema validation.
    pub input_diagnostics: Vec<InputDiagnostic>,
    /// Schema validation errors, warnings, and coercion notes.
    pub validation: ValidationResult,
    /// True only when a complete archive was atomically published.
    pub published: bool,
}

/// Validate one JSON document and atomically publish its coerced representation when valid.
#[allow(
    clippy::module_name_repetitions,
    reason = "The qualified public name describes the complete operation"
)]
pub fn validate_json_to_archive(
    schemas: &Store,
    schema_name: &str,
    input: &str,
    destination: &Path,
    registry: ModelRegistry,
) -> Result<ArchivePublication, PublicationError> {
    let configuration = Configuration {
        return_coerced_data: true,
        return_coercion_infos: true,
        warn_eos_config_keys: matches!(schema_name, "avd_design" | "eos_designs"),
        ..Configuration::default()
    };
    let output = schemas.validate_json(input, schema_name, Some(&configuration))?;
    let mut publication = ArchivePublication {
        input_diagnostics: output.input_diagnostics,
        validation: output.document.result,
        published: false,
    };
    if !publication.input_diagnostics.is_empty() || !publication.validation.errors.is_empty() {
        return Ok(publication);
    }
    let value = output.document.coerced.ok_or_else(|| {
        validated_data::ArchiveError::Build(
            "successful validation did not return coerced data".to_owned(),
        )
    })?;
    validated_data::publish(&value, schemas.archive_hash(), destination, registry)?;
    publication.published = true;
    Ok(publication)
}

#[cfg(test)]
mod tests {
    use avdschema::Load as _;
    use avdschema::StoreSource;

    use super::*;
    use std::sync::Arc;
    use validated_data::*;

    const ITEM_FIELDS: &[FieldDescriptor] = &[
        FieldDescriptor {
            id: 0,
            relation: FieldRelation::Key("enabled"),
            target_model: None,
            scalar_type: Some(ScalarType::Bool),
            required: false,
            relaxed: false,
        },
        FieldDescriptor {
            id: 1,
            relation: FieldRelation::Key("name"),
            target_model: None,
            scalar_type: Some(ScalarType::Str),
            required: false,
            relaxed: false,
        },
    ];
    const ITEM_MODEL: ModelDescriptor = ModelDescriptor {
        identity: "ITEM",
        kind: ModelKind::Dict,
        fields: ITEM_FIELDS,
        primary_key_fields: &[],
        indexed: false,
    };
    const LIST_FIELDS: &[FieldDescriptor] = &[FieldDescriptor {
        id: 0,
        relation: FieldRelation::Item,
        target_model: Some(&ITEM_MODEL),
        scalar_type: None,
        required: false,
        relaxed: false,
    }];
    const LIST_MODEL: ModelDescriptor = ModelDescriptor {
        identity: "LIST",
        kind: ModelKind::List,
        fields: LIST_FIELDS,
        primary_key_fields: &[1],
        indexed: true,
    };
    const ROOT_FIELDS: &[FieldDescriptor] = &[
        FieldDescriptor {
            id: 0,
            relation: FieldRelation::Key("name"),
            target_model: None,
            scalar_type: Some(ScalarType::Str),
            required: false,
            relaxed: false,
        },
        FieldDescriptor {
            id: 1,
            relation: FieldRelation::Key("items"),
            target_model: Some(&LIST_MODEL),
            scalar_type: None,
            required: false,
            relaxed: false,
        },
    ];
    const ROOT_MODEL: ModelDescriptor = ModelDescriptor {
        identity: "ROOT",
        kind: ModelKind::Dict,
        fields: ROOT_FIELDS,
        primary_key_fields: &[],
        indexed: false,
    };
    const REGISTRY: ModelRegistry = ModelRegistry {
        root_model: ROOT_MODEL,
        hash: [7; 32],
    };

    fn schemas() -> Store {
        let source = StoreSource::from_json(r#"{"root":{"type":"dict","keys":{"name":{"type":"str"},"items":{"type":"list","primary_key":"name","items":{"type":"dict","keys":{"name":{"type":"str"},"enabled":{"type":"bool"}}}}}}}"#).expect("valid source");
        Store::compile(&source).expect("valid store")
    }

    #[test]
    fn publishes_and_memory_maps_valid_data() {
        let directory = tempfile::tempdir().expect("temporary directory");
        let destination = directory.path().join("data.rkyv");
        let schemas = schemas();
        let result = validate_json_to_archive(
            &schemas,
            "root",
            r#"{"name":"leaf1","items":[{"name":"Ethernet1","enabled":true}]}"#,
            &destination,
            REGISTRY,
        )
        .expect("publication");
        assert!(result.published);
        let data = Arc::new(
            DataStore::from_file(&destination, schemas.archive_hash(), REGISTRY)
                .expect("compatible archive"),
        );
        let root = data.root().as_dict().expect("dictionary root");
        assert_eq!(root.field(0).and_then(ValueView::as_str), Some("leaf1"));
        let items = root.field(1).and_then(ValueView::as_list).expect("items");
        assert_eq!(items.len(), 1);
        let indexed_item = items
            .get_by_primary_key(&[PrimaryKeyValue::Str("Ethernet1")])
            .and_then(ValueView::as_dict)
            .expect("indexed item");
        assert_eq!(
            indexed_item.field(1).and_then(ValueView::as_str),
            Some("Ethernet1")
        );
        assert_eq!(
            items
                .get(0)
                .and_then(ValueView::as_dict)
                .and_then(|item| item.field(0))
                .and_then(ValueView::as_bool),
            Some(true)
        );
        let root_handle = data.root_handle().as_dict().expect("owned dictionary root");
        drop(data);
        let indexed_handle = root_handle
            .field(1)
            .and_then(|value| value.as_list())
            .and_then(|list_handle| {
                list_handle.get_by_primary_key(&[PrimaryKeyValue::Str("Ethernet1")])
            })
            .and_then(|value| value.as_dict())
            .expect("owned indexed item");
        assert_eq!(
            indexed_handle
                .field(1)
                .and_then(|value| value.as_str().map(str::to_owned)),
            Some("Ethernet1".to_owned())
        );
    }

    #[test]
    fn does_not_publish_invalid_data() {
        let directory = tempfile::tempdir().expect("temporary directory");
        let destination = directory.path().join("data.rkyv");
        let schemas = schemas();
        let result =
            validate_json_to_archive(&schemas, "root", r#"{"name":[]}"#, &destination, REGISTRY)
                .expect("validation result");
        assert!(!result.published);
        assert!(!destination.exists());
        assert!(!result.validation.errors.is_empty());
    }

    #[test]
    fn enables_eos_config_key_warnings_for_avd_design_aliases() {
        let source = crate::validation::test_utils::get_test_store();
        let schemas = Store::compile(&source).expect("valid store");
        let directory = tempfile::tempdir().expect("temporary directory");

        for schema_name in ["avd_design", "eos_designs"] {
            let destination = directory.path().join(format!("{schema_name}.rkyv"));
            let result = validate_json_to_archive(
                &schemas,
                schema_name,
                r#"{"key3":"valid_avd_design_key","key1":"eos_config_key"}"#,
                &destination,
                REGISTRY,
            )
            .expect("publication");
            assert!(result.published);
            assert!(result.validation.errors.is_empty());
            assert_eq!(result.validation.warnings.len(), 1);
            assert!(matches!(
                &result.validation.warnings[0].issue,
                crate::feedback::WarningIssue::IgnoredEosConfigKey(_)
            ));
        }
    }
}
