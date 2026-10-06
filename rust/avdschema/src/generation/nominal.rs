// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Occurrence-specific model identities for generated typed views.
//!
//! Compiled schemas intern structural nodes. This build-time IR instead identifies each collection
//! and relationship at its use-site, so generated APIs can retain nominal type identity without
//! adding generation-only data to the runtime schema archive.

use super::traversal::SchemaOccurrence;
use super::traversal::SchemaRelation;
use super::traversal::SchemaTraverser;
use super::traversal::SchemaVisitor;
use super::traversal::TraversalControl;
use crate::CompileError;
use crate::StoreSource;
use crate::compiled::SchemaId;

/// Stable model identifier within one generated registry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize)]
pub struct ModelId(pub u32);
/// Stable field or relationship identifier within one generated registry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize)]
pub struct SlotId(pub u32);
/// Collection shape represented by a nominal model.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize)]
pub enum ModelKind {
    /// Dictionary model.
    Dict,
    /// Sequence model.
    List,
}
/// Scalar shape accepted at a field occurrence.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize)]
pub enum ScalarKind {
    /// Boolean value.
    Bool,
    /// Signed integer value.
    Int,
    /// String value.
    Str,
}
/// How a relationship is connected to its parent.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize)]
pub enum FieldRelation {
    /// Statically named dictionary key.
    Key(String),
    /// Dictionary key selected by a dynamic-key schema path.
    DynamicKey(String),
    /// Sequence item.
    Item,
}
/// Target shape of a relationship.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize)]
pub enum FieldTarget {
    /// Scalar terminal.
    Scalar(ScalarKind),
    /// Another nominal model.
    Model(ModelId),
}
/// One dictionary field, dynamic field, or list-item relationship.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize)]
pub struct NominalField {
    /// Deterministic slot identifier.
    pub id: SlotId,
    /// Containing model.
    pub parent: ModelId,
    /// Relationship to the parent.
    pub relation: FieldRelation,
    /// Effective target shape.
    pub target: FieldTarget,
    /// Compiled schema node used by validation and default lookup.
    pub schema_id: SchemaId,
    /// Whether the effective schema requires the field.
    pub required: bool,
    /// Whether the effective schema supplies a default.
    pub has_default: bool,
    /// Effective description.
    pub description: Option<String>,
}
/// One collection occurrence used by generated APIs.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize)]
pub struct NominalModel {
    /// Deterministic model identifier.
    pub id: ModelId,
    /// Collection shape.
    pub kind: ModelKind,
    /// Absolute source-schema path.
    pub path: Vec<String>,
    /// Compiled structural node represented by this occurrence.
    pub schema_id: SchemaId,
    /// Ordered primary-key field names for an indexed list.
    ///
    /// The current source schema supports one field. Keeping the nominal contract component-based
    /// avoids making generated consumers depend on that restriction.
    pub primary_key_fields: Vec<String>,
}
/// Build-time nominal model registry for one schema root.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct NominalModelIr {
    /// Root model.
    pub root: ModelId,
    /// Models in depth-first order.
    pub models: Vec<NominalModel>,
    /// Relationships in depth-first order.
    pub fields: Vec<NominalField>,
    /// Hash of all registry semantics consumed by generated code.
    pub registry_hash: [u8; 32],
}

/// Build a nominal registry without retaining a second schema graph.
pub fn build_nominal_model_ir(
    source: &StoreSource,
    schema_name: &str,
) -> Result<NominalModelIr, CompileError> {
    let traverser = SchemaTraverser::compile(source, schema_name)?;
    let mut builder = Builder::default();
    traverser.traverse(&mut builder)?;
    builder.finish()
}

/// Serialize a nominal registry for an external artifact renderer.
pub fn nominal_model_ir_json(
    source: &StoreSource,
    schema_name: &str,
) -> Result<String, CompileError> {
    let registry = build_nominal_model_ir(source, schema_name)?;
    serde_json::to_string_pretty(&registry)
        .map_err(|error| CompileError::Archive(error.to_string()))
}

#[derive(Debug, Default)]
struct Builder {
    models: Vec<NominalModel>,
    fields: Vec<NominalField>,
    parents: Vec<Option<ModelId>>,
}

impl SchemaVisitor for Builder {
    type Error = CompileError;

    fn enter(
        &mut self,
        occurrence: &SchemaOccurrence<'_>,
    ) -> Result<TraversalControl, Self::Error> {
        let parent = self.parents.last().copied().flatten();
        if occurrence.relation() != SchemaRelation::Root
            && occurrence
                .common()
                .deprecation
                .as_ref()
                .is_some_and(|deprecation| deprecation.removed)
        {
            self.parents.push(parent);
            return Ok(TraversalControl::SkipChildren);
        }
        let model = if let Some(kind) = model_kind(occurrence) {
            let id = ModelId(u32::try_from(self.models.len()).map_err(|error| {
                CompileError::Archive(format!("nominal model count exceeds u32: {error}"))
            })?);
            self.models.push(NominalModel {
                id,
                kind,
                path: occurrence.path().to_vec(),
                schema_id: occurrence.schema_id(),
                primary_key_fields: primary_key_fields(occurrence),
            });
            Some(id)
        } else {
            None
        };
        if let Some(parent) = parent
            && let Some(relation) = relation(occurrence.relation())
        {
            let target = if let Some(model) = model {
                FieldTarget::Model(model)
            } else {
                FieldTarget::Scalar(scalar_kind(occurrence.schema_id()).ok_or_else(|| {
                    CompileError::Archive(
                        "collection occurrence is missing a nominal model".to_owned(),
                    )
                })?)
            };
            self.fields.push(NominalField {
                id: SlotId(u32::try_from(self.fields.len()).map_err(|error| {
                    CompileError::Archive(format!("nominal field count exceeds u32: {error}"))
                })?),
                parent,
                relation,
                target,
                schema_id: occurrence.schema_id(),
                required: occurrence.common().required,
                has_default: occurrence.common().default.is_some(),
                description: occurrence.common().description.clone(),
            });
        }
        self.parents.push(model.or(parent));
        Ok(TraversalControl::Descend)
    }

    fn leave(&mut self, _occurrence: &SchemaOccurrence<'_>) -> Result<(), Self::Error> {
        let _ = self.parents.pop();
        Ok(())
    }
}

impl Builder {
    fn finish(self) -> Result<NominalModelIr, CompileError> {
        let root = self.models.first().map(|model| model.id).ok_or_else(|| {
            CompileError::Archive("typed view generation requires a collection root".to_owned())
        })?;
        let registry_hash = registry_hash(root, &self.models, &self.fields);
        Ok(NominalModelIr {
            root,
            models: self.models,
            fields: self.fields,
            registry_hash,
        })
    }
}

fn model_kind(value: &SchemaOccurrence<'_>) -> Option<ModelKind> {
    value
        .dict()
        .map(|_| ModelKind::Dict)
        .or_else(|| value.list().map(|_| ModelKind::List))
}

fn primary_key_fields(value: &SchemaOccurrence<'_>) -> Vec<String> {
    value
        .list()
        .filter(|list| !list.allow_duplicate_primary_key)
        .and_then(|list| list.primary_key.as_deref())
        .map(|primary_key| vec![primary_key.to_owned()])
        .unwrap_or_default()
}
fn scalar_kind(value: SchemaId) -> Option<ScalarKind> {
    match value {
        SchemaId::Bool(_) => Some(ScalarKind::Bool),
        SchemaId::Int(_) => Some(ScalarKind::Int),
        SchemaId::Str(_) => Some(ScalarKind::Str),
        SchemaId::List(_) | SchemaId::Dict(_) => None,
    }
}
fn relation(value: SchemaRelation<'_>) -> Option<FieldRelation> {
    match value {
        SchemaRelation::Root => None,
        SchemaRelation::Key(key) => Some(FieldRelation::Key(key.to_owned())),
        SchemaRelation::DynamicKey(path) => Some(FieldRelation::DynamicKey(path.to_owned())),
        SchemaRelation::Items => Some(FieldRelation::Item),
    }
}
fn registry_hash(root: ModelId, models: &[NominalModel], fields: &[NominalField]) -> [u8; 32] {
    let mut state = blake3::Hasher::new();
    state.update(&root.0.to_le_bytes());
    for model in models {
        state.update(&model.id.0.to_le_bytes());
        state.update(&[match model.kind {
            ModelKind::Dict => 0,
            ModelKind::List => 1,
        }]);
        hash_schema_id(&mut state, model.schema_id);
        for part in &model.path {
            hash_string(&mut state, part);
        }
        state.update(&[0xff]);
        state.update(
            &u64::try_from(model.primary_key_fields.len())
                .unwrap_or(u64::MAX)
                .to_le_bytes(),
        );
        for field in &model.primary_key_fields {
            hash_string(&mut state, field);
        }
    }
    for field in fields {
        state.update(&field.id.0.to_le_bytes());
        state.update(&field.parent.0.to_le_bytes());
        match &field.relation {
            FieldRelation::Key(key) => {
                state.update(&[0]);
                hash_string(&mut state, key);
            }
            FieldRelation::DynamicKey(path) => {
                state.update(&[1]);
                hash_string(&mut state, path);
            }
            FieldRelation::Item => {
                state.update(&[2]);
            }
        }
        match field.target {
            FieldTarget::Scalar(ScalarKind::Bool) => {
                state.update(&[10]);
            }
            FieldTarget::Scalar(ScalarKind::Int) => {
                state.update(&[11]);
            }
            FieldTarget::Scalar(ScalarKind::Str) => {
                state.update(&[12]);
            }
            FieldTarget::Model(id) => {
                state.update(&[20]);
                state.update(&id.0.to_le_bytes());
            }
        }
        hash_schema_id(&mut state, field.schema_id);
        state.update(&[u8::from(field.required), u8::from(field.has_default)]);
    }
    *state.finalize().as_bytes()
}

fn hash_schema_id(state: &mut blake3::Hasher, id: SchemaId) {
    let (kind, index) = match id {
        SchemaId::Bool(index) => (0, index),
        SchemaId::Int(index) => (1, index),
        SchemaId::Str(index) => (2, index),
        SchemaId::List(index) => (3, index),
        SchemaId::Dict(index) => (4, index),
    };
    state.update(&[kind]);
    state.update(&index.to_le_bytes());
}

fn hash_string(state: &mut blake3::Hasher, value: &str) {
    state.update(&u64::try_from(value.len()).unwrap_or(u64::MAX).to_le_bytes());
    state.update(value.as_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Load as _;

    #[test]
    fn shared_structures_keep_distinct_occurrence_identity() {
        let source = StoreSource::from_json(
            r#"{"shared":{"type":"dict","keys":{"name":{"type":"str"}}},"root":{"type":"dict","keys":{"a":{"type":"dict","$ref":"shared#"},"b":{"type":"dict","$ref":"shared#"}}}}"#,
        ).expect("valid source");
        let ir = build_nominal_model_ir(&source, "root").expect("valid registry");
        assert_eq!(ir.models.len(), 3);
        assert_ne!(ir.models[1].id, ir.models[2].id);
        assert_eq!(ir.models[1].schema_id, ir.models[2].schema_id);
        assert_ne!(ir.registry_hash, [0; 32]);
        let repeated = build_nominal_model_ir(&source, "root").expect("valid registry");
        assert_eq!(ir.registry_hash, repeated.registry_hash);

        let changed_source = StoreSource::from_json(
            r#"{"shared":{"type":"dict","keys":{"name":{"type":"str","required":true}}},"root":{"type":"dict","keys":{"a":{"type":"dict","$ref":"shared#"},"b":{"type":"dict","$ref":"shared#"}}}}"#,
        )
        .expect("valid changed source");
        let changed =
            build_nominal_model_ir(&changed_source, "root").expect("valid changed registry");
        assert_ne!(ir.registry_hash, changed.registry_hash);
    }

    #[test]
    fn indexed_lists_retain_component_based_primary_key_metadata() {
        let source = StoreSource::from_json(
            r#"{"root":{"type":"dict","keys":{"indexed":{"type":"list","primary_key":"name","items":{"type":"dict","keys":{"name":{"type":"str"}}}},"duplicates":{"type":"list","primary_key":"name","allow_duplicate_primary_key":true,"items":{"type":"dict","keys":{"name":{"type":"str"}}}}}}}"#,
        )
        .expect("valid source");
        let ir = build_nominal_model_ir(&source, "root").expect("valid registry");

        assert_eq!(ir.models[1].primary_key_fields, ["name"]);
        assert!(ir.models[3].primary_key_fields.is_empty());
    }
}
