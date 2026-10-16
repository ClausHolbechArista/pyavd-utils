// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Owner-preserving Python values over supplied data, schema defaults, and empty collections.
//!
//! This is an access layer, not a second host document. Default collections retain a schema
//! owner and a small navigation path into its archived default; no default is inserted into data.
//! Raw Rust data views continue to observe only supplied values for inheritance and validation.

use crate::{FieldBinding, ModelBinding, Target};
use avdschema::{
    SchemaDataMapping, SchemaDataSequence, SchemaDataValue, SchemaId, SchemaValueView, Store,
};
use std::sync::Arc;
use validated_data::{FieldRelation, ModelKind, PrimaryKeyValue, ValueHandle};

#[derive(Clone, Debug)]
enum Step {
    Key(String),
    Item(usize),
}

#[derive(Clone, Debug)]
enum Source {
    Data(ValueHandle),
    Default { schema: SchemaId, path: Vec<Step> },
    Empty(ModelKind),
}

/// An immutable value retaining its data and, when needed, its compiled schema owner.
///
/// Python access may use defaults, but presence helpers inspect the backing source before that
/// fallback. Cloning this handle copies ownership tokens, never the host document or a default.
#[derive(Clone, Debug)]
pub struct ViewHandle {
    source: Source,
    schemas: Option<Arc<Store>>,
    model: Option<&'static ModelBinding>,
}

impl From<ValueHandle> for ViewHandle {
    fn from(value: ValueHandle) -> Self {
        Self {
            source: Source::Data(value),
            schemas: None,
            model: None,
        }
    }
}

impl ViewHandle {
    pub(crate) fn dynamic_names(&self, field: &FieldBinding) -> Vec<String> {
        let FieldRelation::DynamicKey(path) = field.descriptor.relation else {
            return Vec::new();
        };
        let Some(store) = &self.schemas else {
            return Vec::new();
        };
        let Some(avdschema::SchemaView::Dict(schema)) = self
            .model
            .and_then(|model| model.schema_id)
            .and_then(|id| store.schema(id))
        else {
            return Vec::new();
        };
        if schema
            .dynamic_key(path)
            .and_then(avdschema::SchemaView::deprecation)
            .is_some_and(avdschema::DeprecationView::removed)
        {
            return Vec::new();
        }
        let input = InputMapping(self.input());
        let mut names =
            avdschema::resolve_dynamic_key_names(schema, path, input).unwrap_or_default();
        // Validation assigns an overlapping concrete key to the last non-removed selector.
        // Expose it only through that group's target, never through a conflicting model.
        for (later_path, later_schema) in schema
            .dynamic_keys()
            .skip_while(|(name, _)| *name != path)
            .skip(1)
        {
            if later_schema
                .deprecation()
                .is_some_and(avdschema::DeprecationView::removed)
            {
                continue;
            }
            let later =
                avdschema::resolve_dynamic_key_names(schema, later_path, input).unwrap_or_default();
            names.retain(|name| !later.contains(name));
        }
        names
    }

    fn input(&self) -> InputValue<'_> {
        match &self.source {
            Source::Data(value) => InputValue::Data(value.view()),
            _ => self
                .schema_value()
                .map_or(InputValue::Empty, InputValue::Schema),
        }
    }

    pub(crate) fn is_static_key(&self, key: &str) -> bool {
        self.model.is_some_and(|model| model.fields.iter().any(|field| matches!(field.descriptor.relation, FieldRelation::Key(name) if name == key)))
    }

    pub(crate) fn with_schemas(mut self, schemas: Arc<Store>) -> Self {
        self.schemas = Some(schemas);
        self
    }

    pub(crate) fn with_model(mut self, model: &'static ModelBinding) -> Self {
        self.model = Some(model);
        self
    }

    fn schema_value(&self) -> Option<SchemaValueView<'_>> {
        let Source::Default { schema, path } = &self.source else {
            return None;
        };
        let mut value = self.schemas.as_ref()?.schema(*schema)?.default()?;
        for step in path {
            value = match (value, step) {
                (SchemaValueView::Object(values), Step::Key(key)) => values.get(key)?,
                (SchemaValueView::List(values), Step::Item(index)) => values.get(*index)?,
                _ => return None,
            };
        }
        Some(value)
    }

    fn child(&self, source: Source) -> Self {
        Self {
            source,
            schemas: self.schemas.clone(),
            model: None,
        }
    }

    /// Read a supplied concrete key, without applying that field's schema default.
    pub(crate) fn raw_key(&self, key: &str) -> Option<Self> {
        match &self.source {
            Source::Data(value) => value
                .as_dict()?
                .key(key)
                .map(|value| self.child(Source::Data(value))),
            Source::Default { schema, path } => {
                let SchemaValueView::Object(values) = self.schema_value()? else {
                    return None;
                };
                values.get(key)?;
                let mut path = path.clone();
                path.push(Step::Key(key.to_owned()));
                Some(self.child(Source::Default {
                    schema: *schema,
                    path,
                }))
            }
            Source::Empty(_) => None,
        }
    }

    pub(crate) fn field_binding(&self, name: &str) -> Option<&'static FieldBinding> {
        self.model?.fields.iter().find(|field| {
            field.name == name && matches!(field.descriptor.relation, FieldRelation::Key(_))
        })
    }

    pub(crate) fn field(&self, binding: &FieldBinding) -> Option<Self> {
        let FieldRelation::Key(key) = binding.descriptor.relation else {
            return None;
        };
        if let Some(value) = self.raw_key(key) {
            return Some(value);
        }
        if let Some(schema) = binding.schema_id
            && self
                .schemas
                .as_ref()
                .and_then(|store| store.schema(schema))
                .and_then(avdschema::SchemaView::default)
                .is_some()
        {
            return Some(self.child(Source::Default {
                schema,
                path: Vec::new(),
            }));
        }
        match binding.target {
            Target::Model(model) => Some(self.child(Source::Empty(model().descriptor.kind))),
            Target::Opaque => {
                let kind = match self.schemas.as_ref()?.schema(binding.schema_id?)? {
                    avdschema::SchemaView::Dict(_) => ModelKind::Dict,
                    avdschema::SchemaView::List(_) => ModelKind::List,
                    _ => return None,
                };
                Some(self.child(Source::Empty(kind)))
            }
            Target::Scalar => None,
        }
    }

    pub(crate) fn is_defined(&self, binding: &FieldBinding) -> bool {
        let FieldRelation::Key(key) = binding.descriptor.relation else {
            return false;
        };
        matches!(self.source, Source::Data(_))
            && self.raw_key(key).is_some_and(|value| !value.is_null())
    }

    pub(crate) fn is_default(&self, binding: &FieldBinding) -> bool {
        let FieldRelation::Key(key) = binding.descriptor.relation else {
            return false;
        };
        if self.raw_key(key).is_some() {
            return !matches!(self.source, Source::Data(_));
        }
        self.field(binding).is_some()
    }

    /// Whether this value is explicitly null, without applying defaults.
    pub fn is_null(&self) -> bool {
        match &self.source {
            Source::Data(value) => value.is_null(),
            _ => matches!(self.schema_value(), Some(SchemaValueView::Null)),
        }
    }

    /// Borrow text from the host archive or the schema archive.
    pub fn as_str(&self) -> Option<&str> {
        match &self.source {
            Source::Data(value) => value.as_str(),
            _ => match self.schema_value()? {
                SchemaValueView::String(value) => Some(value),
                _ => None,
            },
        }
    }

    /// Read a signed integer without Python materialization.
    pub fn as_i64(&self) -> Option<i64> {
        match &self.source {
            Source::Data(value) => value.as_i64(),
            _ => match self.schema_value()? {
                SchemaValueView::I64(value) => Some(value),
                SchemaValueView::U64(value) => i64::try_from(value).ok(),
                _ => None,
            },
        }
    }

    /// Read a nonnegative integer, including unsigned schema defaults, without materialization.
    pub fn as_u64(&self) -> Option<u64> {
        match &self.source {
            Source::Data(value) => value.as_u64(),
            _ => match self.schema_value()? {
                SchemaValueView::U64(value) => Some(value),
                SchemaValueView::I64(value) => u64::try_from(value).ok(),
                _ => None,
            },
        }
    }

    /// Read a boolean without Python materialization.
    pub fn as_bool(&self) -> Option<bool> {
        match &self.source {
            Source::Data(value) => value.as_bool(),
            _ => match self.schema_value()? {
                SchemaValueView::Bool(value) => Some(value),
                _ => None,
            },
        }
    }

    fn kind(&self) -> Option<ModelKind> {
        match &self.source {
            Source::Data(value) if value.as_dict().is_some() => Some(ModelKind::Dict),
            Source::Data(value) if value.as_list().is_some() => Some(ModelKind::List),
            Source::Empty(kind) => Some(*kind),
            _ => match self.schema_value()? {
                SchemaValueView::Object(_) => Some(ModelKind::Dict),
                SchemaValueView::List(_) => Some(ModelKind::List),
                _ => None,
            },
        }
    }

    pub(crate) fn as_dict(&self) -> Option<DictHandle> {
        (self.kind() == Some(ModelKind::Dict)).then(|| DictHandle(self.clone()))
    }

    pub(crate) fn as_list(&self) -> Option<ListHandle> {
        (self.kind() == Some(ModelKind::List)).then(|| ListHandle(self.clone()))
    }

    /// Whether the backing collection has no entries; default field lookup does not populate it.
    pub fn is_empty(&self) -> bool {
        if let Source::Data(value) = &self.source {
            return value.is_empty();
        }
        match self.kind() {
            Some(ModelKind::Dict) => self.keys().is_empty(),
            Some(ModelKind::List) => self.as_list().is_none_or(|list| list.is_empty()),
            None => self.is_null(),
        }
    }

    fn keys(&self) -> Vec<String> {
        match &self.source {
            Source::Data(value) => value
                .as_dict()
                .map(|dict| dict.keys().map(str::to_owned).collect())
                .unwrap_or_default(),
            _ => match self.schema_value() {
                Some(SchemaValueView::Object(values)) => {
                    values.iter().map(|(key, _)| key.to_owned()).collect()
                }
                _ => Vec::new(),
            },
        }
    }

    pub(crate) fn content_eq(&self, other: &Self) -> bool {
        if let (Source::Data(left), Source::Data(right)) = (&self.source, &other.source) {
            return left.content_eq(right);
        }
        if self.kind() != other.kind() {
            return false;
        }
        match self.kind() {
            Some(ModelKind::Dict) => {
                let keys = self.keys();
                keys.len() == other.keys().len()
                    && keys.iter().all(|key| {
                        self.raw_key(key).zip(other.raw_key(key)).is_some_and(
                            |(left_value, right_value)| left_value.content_eq(&right_value),
                        )
                    })
            }
            Some(ModelKind::List) => {
                let (Some(left_list), Some(right_list)) = (self.as_list(), other.as_list()) else {
                    return false;
                };
                left_list.len() == right_list.len()
                    && (0..left_list.len()).all(|index| {
                        left_list.get(index).zip(right_list.get(index)).is_some_and(
                            |(left_item, right_item)| left_item.content_eq(&right_item),
                        )
                    })
            }
            None => {
                self.is_null() && other.is_null()
                    || self
                        .as_bool()
                        .zip(other.as_bool())
                        .is_some_and(|(left_value, right_value)| left_value == right_value)
                    || self
                        .as_i64()
                        .zip(other.as_i64())
                        .is_some_and(|(left_value, right_value)| left_value == right_value)
                    || self
                        .as_u64()
                        .zip(other.as_u64())
                        .is_some_and(|(left_value, right_value)| left_value == right_value)
                    || self
                        .as_str()
                        .zip(other.as_str())
                        .is_some_and(|(left_value, right_value)| left_value == right_value)
            }
        }
    }

    /// Explicitly materialize the backing payload for a legacy serialization boundary.
    ///
    /// Only fields present in that payload are serialized; accessor defaults are not injected.
    pub fn to_json(&self) -> String {
        if let Source::Data(value) = &self.source {
            return value.to_json();
        }
        self.to_value().to_string()
    }

    fn to_value(&self) -> serde_json::Value {
        if let Some(value) = self.as_u64() {
            return value.into();
        }
        if let Some(value) = self.as_bool() {
            return value.into();
        }
        if let Some(value) = self.as_i64() {
            return value.into();
        }
        if let Some(value) = self.as_str() {
            return value.into();
        }
        match self.kind() {
            Some(ModelKind::Dict) => serde_json::Value::Object(
                self.keys()
                    .into_iter()
                    .filter_map(|key| self.raw_key(&key).map(|value| (key, value.to_value())))
                    .collect(),
            ),
            Some(ModelKind::List) => serde_json::Value::Array(
                self.as_list()
                    .into_iter()
                    .flat_map(|list| (0..list.len()).filter_map(move |i| list.get(i)))
                    .map(|value| value.to_value())
                    .collect(),
            ),
            None => serde_json::Value::Null,
        }
    }
}

/// Borrowed adapters let the shared schema selector resolver inspect either archive directly.
/// They intentionally expose raw data, so supplied empty/null selectors suppress defaults.
#[derive(Clone, Copy)]
enum InputValue<'a> {
    Data(validated_data::ValueView<'a>),
    Schema(SchemaValueView<'a>),
    Empty,
}

#[derive(Clone, Copy)]
struct InputMapping<'a>(InputValue<'a>);

#[derive(Clone, Copy)]
struct InputSequence<'a>(InputValue<'a>);

impl<'a> SchemaDataValue<'a> for InputValue<'a> {
    type Mapping = InputMapping<'a>;
    type Sequence = InputSequence<'a>;
    fn as_mapping(self) -> Option<Self::Mapping> {
        match self {
            Self::Data(value) if value.as_dict().is_some() => Some(InputMapping(self)),
            Self::Schema(SchemaValueView::Object(_)) | Self::Empty => Some(InputMapping(self)),
            _ => None,
        }
    }
    fn as_sequence(self) -> Option<Self::Sequence> {
        match self {
            Self::Data(value) if value.as_list().is_some() => Some(InputSequence(self)),
            Self::Schema(SchemaValueView::List(_)) => Some(InputSequence(self)),
            _ => None,
        }
    }
    fn as_str(self) -> Option<&'a str> {
        match self {
            Self::Data(value) => value.as_str(),
            Self::Schema(SchemaValueView::String(value)) => Some(value),
            _ => None,
        }
    }
}

impl<'a> SchemaDataMapping<'a> for InputMapping<'a> {
    type Value = InputValue<'a>;
    type Keys = Box<dyn Iterator<Item = std::borrow::Cow<'a, str>> + 'a>;
    fn get(&self, key: &str) -> Option<Self::Value> {
        match self.0 {
            InputValue::Data(value) => value.as_dict()?.key(key).map(InputValue::Data),
            InputValue::Schema(SchemaValueView::Object(values)) => {
                values.get(key).map(InputValue::Schema)
            }
            _ => None,
        }
    }
    fn keys(&self) -> Self::Keys {
        match self.0 {
            InputValue::Data(value) => Box::new(
                value
                    .as_dict()
                    .into_iter()
                    .flat_map(validated_data::DictView::entries)
                    .map(|(key, _)| std::borrow::Cow::Borrowed(key)),
            ),
            InputValue::Schema(SchemaValueView::Object(values)) => Box::new(
                values
                    .iter()
                    .map(|(key, _)| std::borrow::Cow::Borrowed(key)),
            ),
            _ => Box::new(std::iter::empty()),
        }
    }
}

impl<'a> SchemaDataSequence<'a> for InputSequence<'a> {
    type Value = InputValue<'a>;
    type Iter = Box<dyn Iterator<Item = InputValue<'a>> + 'a>;
    fn iter(&self) -> Self::Iter {
        match self.0 {
            InputValue::Data(value) => Box::new(
                value
                    .as_list()
                    .into_iter()
                    .flat_map(|list| (0..list.len()).filter_map(move |i| list.get(i)))
                    .map(InputValue::Data),
            ),
            InputValue::Schema(SchemaValueView::List(values)) => {
                Box::new(values.iter().map(InputValue::Schema))
            }
            _ => Box::new(std::iter::empty()),
        }
    }
}

/// Dictionary access shared by host-backed and schema-backed native models.
#[derive(Clone, Debug)]
pub(crate) struct DictHandle(pub ViewHandle);

impl DictHandle {
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub(crate) fn content_eq(&self, other: &Self) -> bool {
        self.0.content_eq(&other.0)
    }
    pub(crate) fn field(&self, slot: u32) -> Option<ViewHandle> {
        if let Some(binding) = self.0.model.and_then(|model| {
            model
                .fields
                .iter()
                .find(|field| field.descriptor.id == slot)
        }) {
            return self.0.field(binding);
        }
        match &self.0.source {
            Source::Data(value) => value
                .as_dict()?
                .field(slot)
                .map(|value| self.0.child(Source::Data(value))),
            _ => None,
        }
    }
}

/// List access over archived host values or a borrowed schema default.
#[derive(Clone, Debug)]
pub(crate) struct ListHandle(ViewHandle);

impl ListHandle {
    pub(crate) fn content_eq(&self, other: &Self) -> bool {
        self.0.content_eq(&other.0)
    }
    pub(crate) fn len(&self) -> usize {
        match &self.0.source {
            Source::Data(value) => value.as_list().map_or(0, |list| list.len()),
            _ => match self.0.schema_value() {
                Some(SchemaValueView::List(values)) => values.len(),
                _ => 0,
            },
        }
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub(crate) fn get(&self, index: usize) -> Option<ViewHandle> {
        let mut value = match &self.0.source {
            Source::Data(value) => self.0.child(Source::Data(value.as_list()?.get(index)?)),
            Source::Default { schema, path } if index < self.len() => {
                let mut path = path.clone();
                path.push(Step::Item(index));
                self.0.child(Source::Default {
                    schema: *schema,
                    path,
                })
            }
            _ => return None,
        };
        if let Some(Target::Model(model)) = self.0.model.and_then(|model| model.item) {
            value = value.with_model(model());
        }
        Some(value)
    }
    pub(crate) fn get_by_primary_key(&self, keys: &[PrimaryKeyValue<'_>]) -> Option<ViewHandle> {
        let model = self.0.model?;
        // Host archives use their prepared key index. Defaults and dynamic payloads retain
        // their original representation and are searched without building another data store.
        if let Source::Data(value) = &self.0.source
            && let Some(value) = value.as_list()?.get_by_primary_key(keys)
        {
            let value = self.0.child(Source::Data(value));
            return Some(match model.item {
                Some(Target::Model(model)) => value.with_model(model()),
                _ => value,
            });
        }
        (0..self.len())
            .filter_map(|index| self.get(index))
            .find(|item| {
                keys.len() == model.descriptor.primary_key_fields.len()
                    && keys
                        .iter()
                        .zip(model.descriptor.primary_key_fields)
                        .all(|(key, slot)| {
                            let Some(value) = item.as_dict().and_then(|dict| dict.field(*slot))
                            else {
                                return false;
                            };
                            match key {
                                PrimaryKeyValue::Str(key) => value.as_str() == Some(*key),
                                PrimaryKeyValue::Int(key) => value.as_i64() == Some(*key),
                                PrimaryKeyValue::Bool(key) => value.as_bool() == Some(*key),
                            }
                        })
            })
    }
}
