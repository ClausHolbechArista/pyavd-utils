// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

#![allow(
    clippy::as_conversions,
    clippy::indexing_slicing,
    reason = "SchemaId table indices are produced and validated by CompiledStore"
)]

use indexmap::IndexMap;

use crate::CompileError;
use crate::SchemaDiagnostic;
use crate::StoreSource;
use crate::any::SourceSchema;
use crate::compiled::CompiledStore;
use crate::compiled::SchemaId;
use crate::resolve::resolve_ref::resolve_ref;

/// Stable identifier of one use-site in a [`SchemaGraph`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OccurrenceId(usize);

/// Children of one schema occurrence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OccurrenceKind {
    /// Scalar schema without child occurrences.
    Scalar,
    /// List schema and its optional item occurrence.
    List { items: Option<OccurrenceId> },
    /// Dictionary schema and its ordered key occurrences.
    Dict {
        keys: IndexMap<String, OccurrenceId>,
        dynamic_keys: IndexMap<String, OccurrenceId>,
    },
}

/// One schema use-site paired with its effective interned schema node.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Occurrence {
    path: Vec<String>,
    schema_id: SchemaId,
    declared_reference: Option<String>,
    retained_model_reference: Option<String>,
    kind: OccurrenceKind,
}

impl Occurrence {
    /// Absolute source-schema path for this use-site.
    pub fn path(&self) -> &[String] {
        &self.path
    }

    /// Effective structural identity in the compiled schema arena.
    pub fn schema_id(&self) -> SchemaId {
        self.schema_id
    }

    /// `$ref` written at this use-site, before inheritance.
    pub fn declared_reference(&self) -> Option<&str> {
        self.declared_reference.as_deref()
    }

    /// Cross-schema model reference retained by the current Python generator.
    pub fn retained_model_reference(&self) -> Option<&str> {
        self.retained_model_reference.as_deref()
    }

    /// Typed child relationships for this use-site.
    pub fn kind(&self) -> &OccurrenceKind {
        &self.kind
    }
}

/// Compiled schema arena paired with a distinct node for every schema use-site.
#[derive(Debug)]
pub struct SchemaGraph {
    compiled: CompiledStore,
    occurrences: Vec<Occurrence>,
    root: OccurrenceId,
}

impl SchemaGraph {
    /// Compile one named schema and retain its ordered occurrence graph.
    pub fn compile(store: &StoreSource, schema_name: &str) -> Result<Self, CompileError> {
        let compiled = CompiledStore::compile_schema(store, schema_name)?;
        let root_schema = store
            .get(schema_name)
            .map_err(|error| SchemaDiagnostic::Reference {
                schema_path: vec![schema_name.to_owned()],
                reference: format!("{schema_name}#"),
                error: error.into(),
            })?;
        let root_schema_id =
            *compiled
                .roots
                .get(schema_name)
                .ok_or_else(|| SchemaDiagnostic::StructuralCycle {
                    schema_path: vec![schema_name.to_owned()],
                })?;
        let mut builder = GraphBuilder {
            store,
            compiled,
            occurrences: Vec::new(),
            model_schema_name: schema_name,
        };
        let root = builder.build_occurrence(
            &[root_schema],
            root_schema_id,
            vec![schema_name.to_owned()],
        )?;
        Ok(Self {
            compiled: builder.compiled,
            occurrences: builder.occurrences,
            root,
        })
    }

    /// Root occurrence.
    pub fn root(&self) -> OccurrenceId {
        self.root
    }

    pub(crate) fn compiled(&self) -> &CompiledStore {
        &self.compiled
    }

    /// Look up an occurrence by identifier.
    pub fn occurrence(&self, id: OccurrenceId) -> Option<&Occurrence> {
        self.occurrences.get(id.0)
    }

    pub(crate) fn occurrence_internal(&self, id: OccurrenceId) -> &Occurrence {
        &self.occurrences[id.0]
    }

    /// Iterate over all occurrences in depth-first source order.
    pub fn occurrences(&self) -> impl Iterator<Item = (OccurrenceId, &Occurrence)> {
        self.occurrences
            .iter()
            .enumerate()
            .map(|(index, occurrence)| (OccurrenceId(index), occurrence))
    }
}

struct GraphBuilder<'a> {
    store: &'a StoreSource,
    compiled: CompiledStore,
    occurrences: Vec<Occurrence>,
    model_schema_name: &'a str,
}

impl GraphBuilder<'_> {
    fn build_occurrence(
        &mut self,
        declared_layers: &[&SourceSchema],
        schema_id: SchemaId,
        path: Vec<String>,
    ) -> Result<OccurrenceId, CompileError> {
        let layers = expand_layers(self.store, declared_layers, &path)?;
        let declared = declared_layers.first().copied();
        let declared_reference = declared.and_then(schema_ref).map(ToOwned::to_owned);
        let retained_model_reference = layers
            .iter()
            .copied()
            .take_while(|schema| is_pure_reference(schema))
            .find(|schema| retain_model_reference(schema, &layers, self.model_schema_name))
            .and_then(schema_ref)
            .map(ToOwned::to_owned);

        let id = OccurrenceId(self.occurrences.len());
        self.occurrences.push(Occurrence {
            path: path.clone(),
            schema_id,
            declared_reference,
            retained_model_reference,
            kind: OccurrenceKind::Scalar,
        });

        let kind = match schema_id {
            SchemaId::Bool(_) | SchemaId::Int(_) | SchemaId::Str(_) => OccurrenceKind::Scalar,
            SchemaId::List(index) => {
                let list = &self.compiled.lists[index as usize];
                let item_layers = layers
                    .iter()
                    .filter_map(|schema| match schema {
                        SourceSchema::List(schema) => schema.items.as_deref(),
                        _ => None,
                    })
                    .collect::<Vec<_>>();
                let items = match (list.items, item_layers.is_empty()) {
                    (Some(item_schema_id), false) => Some(self.build_occurrence(
                        &item_layers,
                        item_schema_id,
                        path_with(&path, "items"),
                    )?),
                    _ => None,
                };
                OccurrenceKind::List { items }
            }
            SchemaId::Dict(index) => {
                let key_ids = self.compiled.dicts[index as usize].keys.clone();
                let dynamic_key_ids = self.compiled.dicts[index as usize].dynamic_keys.clone();
                let dict_maps = layers.iter().filter_map(|schema| match schema {
                    SourceSchema::Dict(schema) => schema.keys.as_ref(),
                    _ => None,
                });
                let mut child_layers: IndexMap<&str, Vec<&SourceSchema>> = IndexMap::new();
                for map in dict_maps {
                    for (name, schema) in map {
                        child_layers.entry(name).or_default().push(schema);
                    }
                }
                let mut keys = IndexMap::new();
                for (name, child_schema_id) in key_ids {
                    let key_layers = child_layers.get(name.as_str()).ok_or_else(|| {
                        SchemaDiagnostic::StructuralCycle {
                            schema_path: path_with(&path, &name),
                        }
                    })?;
                    let child = self.build_occurrence(
                        key_layers,
                        child_schema_id,
                        path_with(&path_with(&path, "keys"), &name),
                    )?;
                    keys.insert(name, child);
                }
                let dynamic_dict_maps = layers.iter().filter_map(|schema| match schema {
                    SourceSchema::Dict(schema) => schema.dynamic_keys.as_ref(),
                    _ => None,
                });
                let mut dynamic_child_layers: IndexMap<&str, Vec<&SourceSchema>> = IndexMap::new();
                for map in dynamic_dict_maps {
                    for (name, schema) in map {
                        dynamic_child_layers.entry(name).or_default().push(schema);
                    }
                }
                let mut dynamic_keys = IndexMap::new();
                for (name, child_schema_id) in dynamic_key_ids {
                    let dynamic_key_layers =
                        dynamic_child_layers.get(name.as_str()).ok_or_else(|| {
                            SchemaDiagnostic::StructuralCycle {
                                schema_path: path_with(&path, &name),
                            }
                        })?;
                    let child = self.build_occurrence(
                        dynamic_key_layers,
                        child_schema_id,
                        path_with(&path_with(&path, "dynamic_keys"), &name),
                    )?;
                    dynamic_keys.insert(name, child);
                }
                OccurrenceKind::Dict { keys, dynamic_keys }
            }
        };
        self.occurrences[id.0].kind = kind;
        Ok(id)
    }
}

fn expand_layers<'a>(
    store: &'a StoreSource,
    declared_layers: &[&'a SourceSchema],
    path: &[String],
) -> Result<Vec<&'a SourceSchema>, CompileError> {
    let Some(first) = declared_layers.first().copied() else {
        return Err(SchemaDiagnostic::StructuralCycle {
            schema_path: path.to_vec(),
        }
        .into());
    };
    let mut result = Vec::new();
    for declared in declared_layers {
        let mut layer = *declared;
        let mut chain = Vec::new();
        loop {
            if schema_type(first) != schema_type(layer) {
                return Err(SchemaDiagnostic::TypeMismatch {
                    expected: schema_type(first),
                    found: schema_type(layer),
                }
                .into());
            }
            chain.push(std::ptr::from_ref(layer));
            result.push(layer);
            let Some(reference) = schema_ref(layer) else {
                break;
            };
            layer = resolve_ref(reference, store).map_err(|error| SchemaDiagnostic::Reference {
                schema_path: path.to_vec(),
                reference: reference.to_owned(),
                error,
            })?;
            if chain.contains(&std::ptr::from_ref(layer)) {
                return Err(SchemaDiagnostic::ReferenceCycle {
                    schema_path: path.to_vec(),
                    reference: reference.to_owned(),
                }
                .into());
            }
        }
    }
    Ok(result)
}

fn retain_model_reference(
    declared: &SourceSchema,
    effective_layers: &[&SourceSchema],
    model_schema_name: &str,
) -> bool {
    let Some(reference) = schema_ref(declared) else {
        return false;
    };
    let foreign_reference = reference
        .split_once('#')
        .is_some_and(|(schema_name, _)| schema_name != model_schema_name);
    if !foreign_reference || reference.contains("/$defs/") || !is_pure_reference(declared) {
        return false;
    }
    match declared {
        SourceSchema::Dict(_) => true,
        SourceSchema::List(_) => {
            effective_layers
                .iter()
                .find_map(|schema| match schema {
                    SourceSchema::List(schema) => schema.primary_key.as_ref(),
                    _ => None,
                })
                .is_some()
                && !effective_layers
                    .iter()
                    .find_map(|schema| match schema {
                        SourceSchema::List(schema) => schema.allow_duplicate_primary_key,
                        _ => None,
                    })
                    .unwrap_or_default()
        }
        _ => false,
    }
}

fn is_pure_reference(schema: &SourceSchema) -> bool {
    match schema {
        SourceSchema::Bool(schema) => base_is_pure(&schema.base),
        SourceSchema::Int(schema) => {
            base_is_pure(&schema.base)
                && schema.min.is_none()
                && schema.max.is_none()
                && schema.valid_values.valid_values.is_none()
                && schema.valid_values.dynamic_valid_values.is_none()
                && schema.convert_types.convert_types.is_none()
        }
        SourceSchema::Str(schema) => {
            base_is_pure(&schema.base)
                && schema.convert_to_lower_case.is_none()
                && schema.format.is_none()
                && schema.min_length.is_none()
                && schema.max_length.is_none()
                && schema.pattern.is_none()
                && schema.valid_values.valid_values.is_none()
                && schema.valid_values.dynamic_valid_values.is_none()
                && schema.convert_types.convert_types.is_none()
        }
        SourceSchema::List(schema) => {
            base_is_pure(&schema.base)
                && schema.items.is_none()
                && schema.min_length.is_none()
                && schema.max_length.is_none()
                && schema.primary_key.is_none()
                && schema.unique_keys.is_none()
                && schema.allow_duplicate_primary_key.is_none()
        }
        SourceSchema::Dict(schema) => {
            base_is_pure(&schema.base)
                && schema.keys.is_none()
                && schema.dynamic_keys.is_none()
                && schema.allow_other_keys.is_none()
                && schema.schema_defs.is_none()
                && schema.schema_id.is_none()
                && schema.schema_schema.is_none()
        }
    }
}

fn base_is_pure<T: crate::base::DataValue>(base: &crate::base::Base<T>) -> bool {
    base.default.is_none()
        && base.display_name.is_none()
        && base.required.is_none()
        && base.schema_ref.is_some()
}

fn schema_ref(schema: &SourceSchema) -> Option<&str> {
    match schema {
        SourceSchema::Bool(schema) => schema.base.schema_ref.as_deref(),
        SourceSchema::Int(schema) => schema.base.schema_ref.as_deref(),
        SourceSchema::Str(schema) => schema.base.schema_ref.as_deref(),
        SourceSchema::List(schema) => schema.base.schema_ref.as_deref(),
        SourceSchema::Dict(schema) => schema.base.schema_ref.as_deref(),
    }
}

fn schema_type(schema: &SourceSchema) -> &'static str {
    match schema {
        SourceSchema::Bool(_) => "bool",
        SourceSchema::Int(_) => "int",
        SourceSchema::Str(_) => "str",
        SourceSchema::List(_) => "list",
        SourceSchema::Dict(_) => "dict",
    }
}

fn path_with(path: &[String], element: &str) -> Vec<String> {
    let mut result = path.to_vec();
    result.push(element.to_owned());
    result
}

#[cfg(test)]
mod tests {
    use crate::Load as _;
    use crate::OccurrenceKind;
    use crate::SchemaGraph;
    use crate::StoreSource;

    #[test]
    fn graph_preserves_occurrences_and_reference_provenance() {
        let store = StoreSource::from_json(
            r#"{
                "shared": {
                    "type": "dict",
                    "keys": {"value": {"type": "str"}}
                },
                "shared_list": {
                    "type": "list",
                    "primary_key": "name",
                    "items": {
                        "type": "dict",
                        "keys": {"name": {"type": "str"}}
                    }
                },
                "model": {
                    "type": "dict",
                    "dynamic_keys": {
                        "selectors.names": {"type": "str"}
                    },
                    "keys": {
                        "first": {"type": "dict", "$ref": "shared#"},
                        "second": {"type": "dict", "$ref": "shared#"},
                        "required": {"type": "dict", "$ref": "shared#", "required": true},
                        "indexed": {"type": "list", "$ref": "shared_list#"}
                    }
                }
            }"#,
        )
        .unwrap();

        let graph = SchemaGraph::compile(&store, "model").unwrap();
        let root = graph.occurrence(graph.root()).unwrap();
        assert!(matches!(root.kind(), OccurrenceKind::Dict { .. }));
        if let OccurrenceKind::Dict { keys, dynamic_keys } = root.kind() {
            let first_id = *keys.get("first").unwrap();
            let second_id = *keys.get("second").unwrap();
            let first = graph.occurrence(first_id).unwrap();
            let second = graph.occurrence(second_id).unwrap();

            assert_ne!(first_id, second_id);
            assert_eq!(first.schema_id(), second.schema_id());
            assert_eq!(first.declared_reference(), Some("shared#"));
            assert_eq!(first.retained_model_reference(), Some("shared#"));
            assert_eq!(first.path(), ["model", "keys", "first"]);
            assert_eq!(
                graph
                    .occurrence(*keys.get("required").unwrap())
                    .unwrap()
                    .retained_model_reference(),
                None
            );
            assert_eq!(
                graph
                    .occurrence(*keys.get("indexed").unwrap())
                    .unwrap()
                    .retained_model_reference(),
                Some("shared_list#")
            );
            let dynamic = graph
                .occurrence(*dynamic_keys.get("selectors.names").unwrap())
                .unwrap();
            assert_eq!(dynamic.path(), ["model", "dynamic_keys", "selectors.names"]);
        }
    }
}
