// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Nominal model identities for generated typed views.
//!
//! Compiled schemas intern structural nodes. This build-time IR normally identifies collections
//! and relationships at their use-sites, so generated APIs retain nominal type identity without
//! adding generation-only data to the runtime schema archive. Generators may also provide complete
//! model catalogs for referenced schemas. Pure cross-schema references then target those existing
//! identities, matching the source model reuse without expanding their descendants again.

use super::traversal::SchemaOccurrence;
use super::traversal::SchemaRelation;
use super::traversal::SchemaTraverser;
use super::traversal::SchemaVisitor;
use super::traversal::TraversalControl;
use crate::CompileError;
use crate::StoreSource;
use crate::compiled::SchemaId;
use indexmap::IndexMap;
use std::collections::{HashMap, HashSet};

/// Stable model identifier within one generated registry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize)]
pub struct ModelId(pub u32);
/// Stable field identity within one generated registry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize)]
pub struct FieldId(pub u32);
/// Relationship slot local to one containing model.
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
    /// Deterministic registry-wide field identity.
    pub id: FieldId,
    /// Runtime relationship slot local to the containing model.
    pub slot: SlotId,
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
/// One collection model used by generated APIs.
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
/// Build-time nominal model registry for a primary root and any reusable schema catalogs.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct NominalModelIr {
    /// Root model.
    pub root: ModelId,
    /// Generated schema roots, including reusable cross-schema model catalogs.
    pub roots: IndexMap<String, ModelId>,
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
    build_nominal_model_ir_with_reused_schemas(source, schema_name, &[])
}

/// Build a nominal registry which reuses pure references to separately generated schemas.
///
/// Reusable schemas are traversed first and retain their own nominal identities. A pure
/// cross-schema reference from the primary schema then targets the existing referenced model
/// instead of expanding another copy of its descendants. Same-schema references and references
/// carrying local schema changes remain occurrence-specific.
pub fn build_nominal_model_ir_with_reused_schemas(
    source: &StoreSource,
    schema_name: &str,
    reused_schema_names: &[String],
) -> Result<NominalModelIr, CompileError> {
    if reused_schema_names.iter().any(|name| name == schema_name) {
        return Err(CompileError::Archive(format!(
            "primary schema '{schema_name}' cannot also be a reused schema"
        )));
    }
    if reused_schema_names.iter().collect::<HashSet<_>>().len() != reused_schema_names.len() {
        return Err(CompileError::Archive(
            "reused schema names must be unique".to_owned(),
        ));
    }
    let mut builder = Builder::new(reused_schema_names);
    let mut roots = IndexMap::new();
    for reused_schema_name in reused_schema_names {
        let root = builder.traverse(source, reused_schema_name, false)?;
        roots.insert(reused_schema_name.clone(), root);
    }
    let root = builder.traverse(source, schema_name, true)?;
    roots.insert(schema_name.to_owned(), root);
    Ok(builder.finish(root, roots))
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

/// Serialize a nominal registry with reusable cross-schema model catalogs.
pub fn nominal_model_ir_with_reused_schemas_json(
    source: &StoreSource,
    schema_name: &str,
    reused_schema_names: &[String],
) -> Result<String, CompileError> {
    let registry =
        build_nominal_model_ir_with_reused_schemas(source, schema_name, reused_schema_names)?;
    serde_json::to_string_pretty(&registry)
        .map_err(|error| CompileError::Archive(error.to_string()))
}

#[derive(Debug, Default)]
struct Builder {
    models: Vec<NominalModel>,
    fields: Vec<NominalField>,
    next_slots: HashMap<ModelId, u32>,
    parents: Vec<Option<ModelId>>,
    models_by_path: HashMap<Vec<String>, ModelId>,
    reused_schema_names: HashSet<String>,
    current_schema_name: String,
    reuse_references: bool,
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
        let reused_model = self
            .reuse_references
            .then(|| {
                reusable_model_reference(
                    occurrence,
                    &self.current_schema_name,
                    &self.reused_schema_names,
                )
            })
            .flatten()
            .map(reference_path)
            .transpose()?
            .map(|path| {
                self.models_by_path.get(&path).copied().ok_or_else(|| {
                    CompileError::Archive(format!(
                        "reusable schema model '{}' was not generated before use",
                        path.join("/")
                    ))
                })
            })
            .transpose()?;
        let model = if let Some(model) = reused_model {
            Some(model)
        } else if let Some(kind) = model_kind(occurrence) {
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
            self.models_by_path.insert(occurrence.path().to_vec(), id);
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
            let next_slot = self.next_slots.entry(parent).or_default();
            let slot = SlotId(*next_slot);
            *next_slot = next_slot.checked_add(1).ok_or_else(|| {
                CompileError::Archive(format!(
                    "nominal model {} field count exceeds u32",
                    parent.0
                ))
            })?;
            self.fields.push(NominalField {
                id: FieldId(u32::try_from(self.fields.len()).map_err(|error| {
                    CompileError::Archive(format!("nominal field count exceeds u32: {error}"))
                })?),
                slot,
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
        Ok(if reused_model.is_some() {
            TraversalControl::SkipChildren
        } else {
            TraversalControl::Descend
        })
    }

    fn leave(&mut self, _occurrence: &SchemaOccurrence<'_>) -> Result<(), Self::Error> {
        let _ = self.parents.pop();
        Ok(())
    }
}

impl Builder {
    fn new(reused_schema_names: &[String]) -> Self {
        Self {
            reused_schema_names: reused_schema_names.iter().cloned().collect(),
            ..Self::default()
        }
    }

    fn traverse(
        &mut self,
        source: &StoreSource,
        schema_name: &str,
        reuse_references: bool,
    ) -> Result<ModelId, CompileError> {
        schema_name.clone_into(&mut self.current_schema_name);
        self.reuse_references = reuse_references;
        let traverser = SchemaTraverser::compile(source, schema_name)?;
        traverser.traverse(self)?;
        self.models_by_path
            .get(&vec![schema_name.to_owned()])
            .copied()
            .ok_or_else(|| {
                CompileError::Archive(format!(
                    "typed view generation requires collection root '{schema_name}'"
                ))
            })
    }

    fn finish(self, root: ModelId, roots: IndexMap<String, ModelId>) -> NominalModelIr {
        let registry_hash = registry_hash(root, &self.models, &self.fields);
        NominalModelIr {
            root,
            roots,
            models: self.models,
            fields: self.fields,
            registry_hash,
        }
    }
}

fn reusable_model_reference<'a>(
    occurrence: &'a SchemaOccurrence<'a>,
    current_schema_name: &str,
    reused_schema_names: &HashSet<String>,
) -> Option<&'a str> {
    let reusable_collection = match occurrence.schema_id() {
        SchemaId::Dict(_) => true,
        SchemaId::List(_) => occurrence
            .list()
            .is_some_and(|list| list.primary_key.is_some() && !list.allow_duplicate_primary_key),
        SchemaId::Bool(_) | SchemaId::Int(_) | SchemaId::Str(_) => false,
    };
    reusable_collection.then(|| {
        occurrence
            .pure_references()
            .iter()
            .copied()
            .find(|reference| {
                reference.split_once('#').is_some_and(|(schema_name, _)| {
                    schema_name != current_schema_name && reused_schema_names.contains(schema_name)
                }) && !reference.contains("/$defs/")
            })
    })?
}

fn reference_path(reference: &str) -> Result<Vec<String>, CompileError> {
    let (schema_name, path) = reference.split_once('#').ok_or_else(|| {
        CompileError::Archive(format!(
            "reusable schema reference '{reference}' is invalid"
        ))
    })?;
    let mut components = vec![schema_name.to_owned()];
    components.extend(
        path.split('/')
            .filter(|component| !component.is_empty())
            .map(ToOwned::to_owned),
    );
    Ok(components)
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
        state.update(&field.slot.0.to_le_bytes());
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
        assert_eq!(
            ir.fields
                .iter()
                .map(|field| (field.parent, field.slot))
                .collect::<Vec<_>>(),
            [
                (ModelId(0), SlotId(0)),
                (ModelId(1), SlotId(0)),
                (ModelId(0), SlotId(1)),
                (ModelId(2), SlotId(0)),
            ]
        );
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
    fn pure_cross_schema_references_reuse_generated_models() {
        let source = StoreSource::from_json(
            r#"{"external":{"type":"dict","keys":{"shared":{"type":"dict","keys":{"name":{"type":"str"}}}}},"root":{"type":"dict","keys":{"value":{"type":"dict","$ref":"external#/keys/shared"}}}}"#,
        )
        .expect("valid source");
        let ir =
            build_nominal_model_ir_with_reused_schemas(&source, "root", &["external".to_owned()])
                .expect("valid registry");

        assert_eq!(ir.models.len(), 3);
        assert_eq!(ir.fields.len(), 3);
        assert_eq!(ir.roots["external"], ModelId(0));
        assert_eq!(ir.root, ModelId(2));
        assert_eq!(ir.fields[2].target, FieldTarget::Model(ModelId(1)));
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
