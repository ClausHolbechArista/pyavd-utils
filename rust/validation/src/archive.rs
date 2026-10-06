// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Validated single-host data archives.
//!
//! Validation and coercion happen before publication. Successful data is flattened into value and
//! relationship tables, written atomically, and later memory-mapped. Generated code supplies the
//! nominal model registry; compatibility is exact rather than negotiated.

#![allow(
    clippy::mem_forget,
    reason = "self_cell uses mem::forget internally to construct a self-referential mmap owner"
)]

use std::collections::HashMap;
use std::io::Write as _;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

use avdschema::Store;
use mmap_guard::FileData;
use rkyv::Archive;
use rkyv::Deserialize;
use rkyv::Serialize;
use rkyv::rancor::Error as RkyvError;
use self_cell::self_cell;
use serde_json::Value;

use crate::Configuration;
use crate::StoreValidateError;
use crate::StoreValidateInput as _;
use crate::ValidationResult;
use crate::feedback::InputDiagnostic;

const MAGIC: &[u8; 8] = b"AVDDATA\0";
const FORMAT_VERSION: u32 = 2;
const HEADER_LENGTH: usize = 16;
/// Validation semantics used by typed validated-data archives.
///
/// Policy 1 returns coerced data and coercion diagnostics and enables EOS Config key warnings for AVD Design roots.
pub const VALIDATION_POLICY_ID: u32 = 1;
static TEMPORARY_FILE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// Generated relationship kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldRelation {
    /// Statically named dictionary key.
    Key(&'static str),
    /// Dynamic dictionary key schema. Concrete keys remain data-dependent.
    DynamicKey(&'static str),
    /// Sequence item.
    Item,
}

/// Model identity emitted by a schema generator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModelDescriptor {
    /// Registry-local identifier.
    pub id: u32,
}
/// One generated field or list-item relationship.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldDescriptor {
    /// Registry-local slot identifier.
    pub id: u32,
    /// Model containing the relationship.
    pub parent_model: u32,
    /// Relationship to the parent model.
    pub relation: FieldRelation,
    /// Nominal child model for collection values.
    pub target_model: Option<u32>,
}
/// Ordered field slots forming the primary key for one indexed-list model.
///
/// Current schemas emit one component. The slice keeps archive and generated-code contracts ready
/// for composite keys without exposing a single-field assumption.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PrimaryKeyDescriptor {
    /// Indexed-list model containing the items.
    pub model: u32,
    /// Ordered field slots on each item model.
    pub field_slots: &'static [u32],
}
/// Static generated registry paired with an archive.
#[derive(Clone, Copy, Debug)]
pub struct ModelRegistry {
    /// Root model identifier.
    pub root_model: u32,
    /// Hash emitted from the nominal schema IR.
    pub hash: [u8; 32],
    /// Field and item relationships.
    pub fields: &'static [FieldDescriptor],
    /// Indexed-list primary-key definitions.
    pub primary_keys: &'static [PrimaryKeyDescriptor],
}

impl ModelRegistry {
    fn field(self, parent: u32, key: &str) -> Option<FieldDescriptor> {
        self.fields.iter().copied().find(|field| {
            field.parent_model == parent
                && matches!(field.relation, FieldRelation::Key(field_key) if field_key == key)
        })
    }

    fn item(self, parent: u32) -> Option<FieldDescriptor> {
        self.fields
            .iter()
            .copied()
            .find(|field| field.parent_model == parent && field.relation == FieldRelation::Item)
    }

    fn primary_key(self, model: u32) -> Option<PrimaryKeyDescriptor> {
        self.primary_keys
            .iter()
            .copied()
            .find(|descriptor| descriptor.model == model)
    }
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
) -> Result<ArchivePublication, ArchiveError> {
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
        ArchiveError::Build("successful validation did not return coerced data".to_owned())
    })?;
    let archive = Builder::new(registry).build(&value, schemas.archive_hash())?;
    let archived = rkyv::to_bytes::<RkyvError>(&archive)
        .map_err(|error| ArchiveError::Build(error.to_string()))?;
    let mut bytes = rkyv::util::AlignedVec::<16>::with_capacity(HEADER_LENGTH + archived.len());
    bytes.extend_from_slice(MAGIC);
    bytes.extend_from_slice(&FORMAT_VERSION.to_le_bytes());
    bytes.extend_from_slice(&VALIDATION_POLICY_ID.to_le_bytes());
    bytes.extend_from_slice(archived.as_slice());
    write_atomically(destination, bytes.as_slice())?;
    publication.published = true;
    Ok(publication)
}

/// Identifier of a value node in one archive.
#[derive(Archive, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[rkyv(derive(Clone, Copy, Debug, PartialEq, Eq))]
pub struct ValueId(u32);

/// Identifier of an interned string.
#[derive(Archive, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[rkyv(derive(Clone, Copy, Debug, PartialEq, Eq))]
pub struct StringId(u32);

/// Flat value representation. Collections refer to contiguous relationship-table ranges.
#[derive(Archive, Serialize, Deserialize, Clone, Debug)]
#[rkyv(derive(Debug))]
enum ValueNode {
    Null,
    Bool(bool),
    I64(i64),
    U64(u64),
    String(StringId),
    List {
        start: u32,
        len: u32,
        model: u32,
        primary_key_start: u32,
        primary_key_len: u32,
    },
    Dict {
        start: u32,
        len: u32,
        model: u32,
    },
}

#[derive(Archive, Serialize, Deserialize, Clone, Debug)]
#[rkyv(derive(Debug))]
struct MapSlot {
    key: StringId,
    field_slot: Option<u32>,
    value: ValueId,
}

#[derive(Archive, Serialize, Deserialize, Clone, Debug)]
#[rkyv(derive(Debug))]
struct SequenceSlot {
    field_slot: Option<u32>,
    value: ValueId,
}

/// One scalar component of an indexed-list primary key.
#[derive(Archive, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[rkyv(derive(Debug, PartialEq, Eq))]
enum PrimaryKeyPart {
    Bool(bool),
    I64(i64),
    U64(u64),
    String(StringId),
}

#[derive(Archive, Serialize, Deserialize, Clone, Debug)]
#[rkyv(derive(Debug))]
struct PrimaryKeySlot {
    parts_start: u32,
    parts_len: u32,
    value: ValueId,
}

#[derive(Archive, Serialize, Deserialize, Clone, Debug)]
#[rkyv(derive(Debug))]
struct ValidatedArchive {
    schema_hash: [u8; 32],
    registry_hash: [u8; 32],
    policy_id: u32,
    root: ValueId,
    values: Vec<ValueNode>,
    map_slots: Vec<MapSlot>,
    sequence_slots: Vec<SequenceSlot>,
    primary_key_parts: Vec<PrimaryKeyPart>,
    primary_key_slots: Vec<PrimaryKeySlot>,
    strings: Vec<String>,
}

struct Builder {
    registry: ModelRegistry,
    values: Vec<ValueNode>,
    map_slots: Vec<MapSlot>,
    sequence_slots: Vec<SequenceSlot>,
    primary_key_parts: Vec<PrimaryKeyPart>,
    primary_key_slots: Vec<PrimaryKeySlot>,
    strings: Vec<String>,
    string_ids: HashMap<String, StringId>,
}

impl Builder {
    fn new(registry: ModelRegistry) -> Self {
        Self {
            registry,
            values: Vec::new(),
            map_slots: Vec::new(),
            sequence_slots: Vec::new(),
            primary_key_parts: Vec::new(),
            primary_key_slots: Vec::new(),
            strings: Vec::new(),
            string_ids: HashMap::new(),
        }
    }

    fn build(
        mut self,
        value: &Value,
        schema_hash: [u8; 32],
    ) -> Result<ValidatedArchive, ArchiveError> {
        let root = self.push_value(value, Some(self.registry.root_model))?;
        Ok(ValidatedArchive {
            schema_hash,
            registry_hash: self.registry.hash,
            policy_id: VALIDATION_POLICY_ID,
            root,
            values: self.values,
            map_slots: self.map_slots,
            sequence_slots: self.sequence_slots,
            primary_key_parts: self.primary_key_parts,
            primary_key_slots: self.primary_key_slots,
            strings: self.strings,
        })
    }

    fn push_value(&mut self, value: &Value, model: Option<u32>) -> Result<ValueId, ArchiveError> {
        let id = ValueId(to_u32(self.values.len(), "value")?);
        self.values.push(ValueNode::Null);
        let node = match value {
            Value::Null => ValueNode::Null,
            Value::Bool(value) => ValueNode::Bool(*value),
            Value::Number(value) if value.is_i64() => ValueNode::I64(
                value
                    .as_i64()
                    .ok_or_else(|| ArchiveError::Build("invalid i64".to_owned()))?,
            ),
            Value::Number(value) if value.is_u64() => ValueNode::U64(
                value
                    .as_u64()
                    .ok_or_else(|| ArchiveError::Build("invalid u64".to_owned()))?,
            ),
            Value::Number(_) => {
                return Err(ArchiveError::Build(
                    "floating-point values are not supported".to_owned(),
                ));
            }
            Value::String(value) => ValueNode::String(self.intern(value)?),
            Value::Array(items) => {
                let parent = model.unwrap_or(u32::MAX);
                let descriptor = self.registry.item(parent);
                let primary_key = self.registry.primary_key(parent);
                let mut slots = Vec::with_capacity(items.len());
                let mut primary_keys = Vec::with_capacity(items.len());
                for item in items {
                    let child =
                        self.push_value(item, descriptor.and_then(|field| field.target_model))?;
                    slots.push(SequenceSlot {
                        field_slot: descriptor.map(|field| field.id),
                        value: child,
                    });
                    if let Some(primary_key) = primary_key {
                        primary_keys.push((
                            self.primary_key_parts(child, primary_key.field_slots)?,
                            child,
                        ));
                    }
                }
                let start = to_u32(self.sequence_slots.len(), "sequence slot")?;
                self.sequence_slots.extend(slots);
                let primary_key_start = to_u32(self.primary_key_slots.len(), "primary key slot")?;
                for (key_parts, item_value) in primary_keys {
                    let parts_start = to_u32(self.primary_key_parts.len(), "primary key part")?;
                    let parts_len = to_u32(key_parts.len(), "primary key part count")?;
                    self.primary_key_parts.extend(key_parts);
                    self.primary_key_slots.push(PrimaryKeySlot {
                        parts_start,
                        parts_len,
                        value: item_value,
                    });
                }
                ValueNode::List {
                    start,
                    len: to_u32(items.len(), "sequence length")?,
                    model: parent,
                    primary_key_start,
                    primary_key_len: to_u32(self.primary_key_slots.len(), "primary key slot")?
                        .checked_sub(primary_key_start)
                        .ok_or_else(|| {
                            ArchiveError::Build("invalid primary key slot range".to_owned())
                        })?,
                }
            }
            Value::Object(items) => {
                let parent = model.unwrap_or(u32::MAX);
                let mut slots = Vec::with_capacity(items.len());
                for (key, item) in items {
                    let descriptor = self.registry.field(parent, key);
                    let child =
                        self.push_value(item, descriptor.and_then(|field| field.target_model))?;
                    let key = self.intern(key)?;
                    slots.push(MapSlot {
                        key,
                        field_slot: descriptor.map(|field| field.id),
                        value: child,
                    });
                }
                let start = to_u32(self.map_slots.len(), "map slot")?;
                self.map_slots.extend(slots);
                ValueNode::Dict {
                    start,
                    len: to_u32(items.len(), "map length")?,
                    model: parent,
                }
            }
        };
        let index = usize::try_from(id.0)
            .map_err(|error| ArchiveError::Build(format!("invalid value id: {error}")))?;
        let target = self.values.get_mut(index).ok_or_else(|| {
            ArchiveError::Build(format!("value id {index} is outside the builder table"))
        })?;
        *target = node;
        Ok(id)
    }

    fn primary_key_parts(
        &self,
        value: ValueId,
        field_slots: &[u32],
    ) -> Result<Vec<PrimaryKeyPart>, ArchiveError> {
        let value_index = usize::try_from(value.0).map_err(|error| {
            ArchiveError::Build(format!("invalid indexed-list item id: {error}"))
        })?;
        let ValueNode::Dict { start, len, .. } = self.values.get(value_index).ok_or_else(|| {
            ArchiveError::Build(format!("indexed-list item {value_index} is missing"))
        })?
        else {
            return Err(ArchiveError::Build(
                "indexed-list item is not a dictionary".to_owned(),
            ));
        };
        let start = usize::try_from(*start).map_err(|error| {
            ArchiveError::Build(format!("invalid indexed-list item start: {error}"))
        })?;
        let len = usize::try_from(*len).map_err(|error| {
            ArchiveError::Build(format!("invalid indexed-list item length: {error}"))
        })?;
        let slots = self
            .map_slots
            .get(
                start..start.checked_add(len).ok_or_else(|| {
                    ArchiveError::Build("indexed-list item range overflows".to_owned())
                })?,
            )
            .ok_or_else(|| ArchiveError::Build("indexed-list item range is invalid".to_owned()))?;
        field_slots
            .iter()
            .map(|field_slot| {
                let slot = slots
                    .iter()
                    .find(|slot| slot.field_slot == Some(*field_slot))
                    .ok_or_else(|| {
                        ArchiveError::Build(format!(
                            "indexed-list item is missing primary-key field slot {field_slot}"
                        ))
                    })?;
                let index = usize::try_from(slot.value.0).map_err(|error| {
                    ArchiveError::Build(format!("invalid primary-key value id: {error}"))
                })?;
                match self.values.get(index) {
                    Some(ValueNode::Bool(node_value)) => Ok(PrimaryKeyPart::Bool(*node_value)),
                    Some(ValueNode::I64(node_value)) => Ok(PrimaryKeyPart::I64(*node_value)),
                    Some(ValueNode::U64(node_value)) => Ok(PrimaryKeyPart::U64(*node_value)),
                    Some(ValueNode::String(node_value)) => Ok(PrimaryKeyPart::String(*node_value)),
                    Some(_) => Err(ArchiveError::Build(format!(
                        "primary-key field slot {field_slot} is not a scalar"
                    ))),
                    None => Err(ArchiveError::Build(format!(
                        "primary-key field slot {field_slot} references a missing value"
                    ))),
                }
            })
            .collect()
    }

    fn intern(&mut self, value: &str) -> Result<StringId, ArchiveError> {
        if let Some(id) = self.string_ids.get(value) {
            return Ok(*id);
        }
        let id = StringId(to_u32(self.strings.len(), "string")?);
        self.strings.push(value.to_owned());
        self.string_ids.insert(value.to_owned(), id);
        Ok(id)
    }
}

fn to_u32(value: usize, subject: &str) -> Result<u32, ArchiveError> {
    u32::try_from(value)
        .map_err(|error| ArchiveError::Build(format!("{subject} count exceeds u32: {error}")))
}

struct ArchiveRoot<'a>(&'a ArchivedValidatedArchive);
self_cell! {
    struct ArchiveCell {
        owner: FileData,
        #[covariant]
        dependent: ArchiveRoot,
    }
}

/// Immutable memory-mapped validated data.
pub struct DataStore {
    archive: ArchiveCell,
}

impl std::fmt::Debug for DataStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DataStore").finish_non_exhaustive()
    }
}

impl DataStore {
    /// Memory-map an archive and require exact schema, registry, and policy compatibility.
    pub fn from_file(
        path: &Path,
        schema_hash: [u8; 32],
        registry_hash: [u8; 32],
    ) -> Result<Self, ArchiveError> {
        let data = mmap_guard::map_file(path)?;
        let archive = ArchiveCell::try_new(data, |bytes| {
            validate_header(bytes.as_ref())?;
            let root = rkyv::access::<ArchivedValidatedArchive, RkyvError>(bytes.as_ref())
                .map_err(|error| ArchiveError::Invalid(error.to_string()))?;
            if root.schema_hash != schema_hash {
                return Err(ArchiveError::Compatibility(
                    "schema archive hash differs".to_owned(),
                ));
            }
            if root.registry_hash != registry_hash {
                return Err(ArchiveError::Compatibility(
                    "generated model registry hash differs".to_owned(),
                ));
            }
            if root.policy_id.to_native() != VALIDATION_POLICY_ID {
                return Err(ArchiveError::Compatibility(
                    "validation policy differs".to_owned(),
                ));
            }
            validate_integrity(root)?;
            Ok(ArchiveRoot(root))
        })?;
        Ok(Self { archive })
    }

    /// Return the root value without materializing it.
    pub fn root(&self) -> ValueView<'_> {
        self.value(self.archived().root)
    }

    /// Return an owned root handle suitable for objects that must outlive a Rust borrow, such as
    /// Python view wrappers.
    pub fn root_handle(self: &Arc<Self>) -> ValueHandle {
        ValueHandle {
            store: Arc::clone(self),
            id: self.archived().root.0.to_native(),
        }
    }

    fn archived(&self) -> &ArchivedValidatedArchive {
        self.archive.borrow_dependent().0
    }

    fn value(&self, id: ArchivedValueId) -> ValueView<'_> {
        ValueView {
            store: self,
            id: id.0.to_native(),
        }
    }
}

/// Owned handle to one immutable archived value.
///
/// Cloning a handle retains the memory-mapped store; it does not materialize the value.
#[derive(Clone, Debug)]
pub struct ValueHandle {
    store: Arc<DataStore>,
    id: u32,
}

impl ValueHandle {
    fn view(&self) -> ValueView<'_> {
        ValueView {
            store: &self.store,
            id: self.id,
        }
    }

    /// Return a dictionary handle when this value is a dictionary.
    pub fn as_dict(&self) -> Option<DictHandle> {
        self.view().as_dict().map(|_| DictHandle(self.clone()))
    }

    /// Return a list handle when this value is a list.
    pub fn as_list(&self) -> Option<ListHandle> {
        self.view().as_list().map(|_| ListHandle(self.clone()))
    }

    /// Borrow a string scalar.
    pub fn as_str(&self) -> Option<&str> {
        self.view().as_str()
    }

    /// Read an integer scalar.
    pub fn as_i64(&self) -> Option<i64> {
        self.view().as_i64()
    }

    /// Read a boolean scalar.
    pub fn as_bool(&self) -> Option<bool> {
        self.view().as_bool()
    }

    /// Return whether the value is explicit null.
    pub fn is_null(&self) -> bool {
        self.view().is_null()
    }
}

/// Borrowed zero-copy view of one archived value.
#[derive(Clone, Copy, Debug)]
pub struct ValueView<'a> {
    store: &'a DataStore,
    id: u32,
}

impl<'a> ValueView<'a> {
    #[allow(
        clippy::indexing_slicing,
        reason = "all value IDs are checked before DataStore construction"
    )]
    fn node(self) -> &'a ArchivedValueNode {
        &self.store.archived().values[usize::try_from(self.id).unwrap_or(usize::MAX)]
    }
    /// Return a dictionary view when this value is a dictionary.
    pub fn as_dict(self) -> Option<DictView<'a>> {
        matches!(self.node(), ArchivedValueNode::Dict { .. }).then_some(DictView(self))
    }
    /// Return a list view when this value is a list.
    pub fn as_list(self) -> Option<ListView<'a>> {
        matches!(self.node(), ArchivedValueNode::List { .. }).then_some(ListView(self))
    }
    /// Borrow a string scalar.
    pub fn as_str(self) -> Option<&'a str> {
        let ArchivedValueNode::String(id) = self.node() else {
            return None;
        };
        self.store
            .archived()
            .strings
            .get(usize::try_from(id.0.to_native()).ok()?)
            .map(AsRef::as_ref)
    }
    /// Read a signed integer scalar.
    pub fn as_i64(self) -> Option<i64> {
        match self.node() {
            ArchivedValueNode::I64(value) => Some(value.to_native()),
            ArchivedValueNode::U64(value) => i64::try_from(value.to_native()).ok(),
            _ => None,
        }
    }
    /// Read a boolean scalar.
    pub fn as_bool(self) -> Option<bool> {
        let ArchivedValueNode::Bool(value) = self.node() else {
            return None;
        };
        Some(*value)
    }
    /// Return whether the archived value is explicit null.
    pub fn is_null(self) -> bool {
        matches!(self.node(), ArchivedValueNode::Null)
    }
}

/// Borrowed dictionary view with slot- and key-based lookup.
#[derive(Clone, Copy, Debug)]
pub struct DictView<'a>(ValueView<'a>);

impl<'a> DictView<'a> {
    fn range(self) -> (usize, usize) {
        let ArchivedValueNode::Dict { start, len, .. } = self.0.node() else {
            return (0, 0);
        };
        (
            usize::try_from(start.to_native()).unwrap_or(0),
            usize::try_from(len.to_native()).unwrap_or(0),
        )
    }
    /// Look up a statically generated field slot.
    pub fn field(self, slot: u32) -> Option<ValueView<'a>> {
        let (start, len) = self.range();
        self.0
            .store
            .archived()
            .map_slots
            .get(start..start.checked_add(len)?)?
            .iter()
            .find_map(|entry| {
                (entry.field_slot.as_ref().map(|value| value.to_native()) == Some(slot))
                    .then(|| self.0.store.value(entry.value))
            })
    }
    /// Look up a concrete key, including dynamic and extra keys.
    pub fn key(self, key: &str) -> Option<ValueView<'a>> {
        let (start, len) = self.range();
        self.0
            .store
            .archived()
            .map_slots
            .get(start..start.checked_add(len)?)?
            .iter()
            .find_map(|entry| {
                let stored = self
                    .0
                    .store
                    .archived()
                    .strings
                    .get(usize::try_from(entry.key.0.to_native()).ok()?)?;
                (stored.as_ref() == key).then(|| self.0.store.value(entry.value))
            })
    }
}

/// Owned dictionary handle retaining its memory-mapped store.
#[derive(Clone, Debug)]
pub struct DictHandle(ValueHandle);

impl DictHandle {
    /// Look up a statically generated field slot.
    pub fn field(&self, slot: u32) -> Option<ValueHandle> {
        self.0
            .view()
            .as_dict()?
            .field(slot)
            .map(|value| ValueHandle {
                store: Arc::clone(&self.0.store),
                id: value.id,
            })
    }

    /// Look up a concrete dictionary key.
    pub fn key(&self, key: &str) -> Option<ValueHandle> {
        self.0.view().as_dict()?.key(key).map(|value| ValueHandle {
            store: Arc::clone(&self.0.store),
            id: value.id,
        })
    }
}

/// One borrowed scalar component used for indexed-list lookup.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrimaryKeyValue<'a> {
    Bool(bool),
    Int(i64),
    Str(&'a str),
}

/// Borrowed sequence view.
#[derive(Clone, Copy, Debug)]
pub struct ListView<'a>(ValueView<'a>);

impl<'a> ListView<'a> {
    fn range(self) -> (usize, usize) {
        let ArchivedValueNode::List { start, len, .. } = self.0.node() else {
            return (0, 0);
        };
        (
            usize::try_from(start.to_native()).unwrap_or(0),
            usize::try_from(len.to_native()).unwrap_or(0),
        )
    }

    fn primary_key_range(self) -> (usize, usize) {
        let ArchivedValueNode::List {
            primary_key_start,
            primary_key_len,
            ..
        } = self.0.node()
        else {
            return (0, 0);
        };
        (
            usize::try_from(primary_key_start.to_native()).unwrap_or(0),
            usize::try_from(primary_key_len.to_native()).unwrap_or(0),
        )
    }
    /// Number of archived items.
    pub fn len(self) -> usize {
        self.range().1
    }
    /// Return whether the sequence is empty.
    pub fn is_empty(self) -> bool {
        self.len() == 0
    }
    /// Return one item by position.
    pub fn get(self, index: usize) -> Option<ValueView<'a>> {
        let (start, len) = self.range();
        (index < len)
            .then(|| self.0.store.archived().sequence_slots.get(start + index))
            .flatten()
            .map(|entry| self.0.store.value(entry.value))
    }

    /// Look up an indexed-list item by its ordered primary-key components.
    pub fn get_by_primary_key(self, key: &[PrimaryKeyValue<'_>]) -> Option<ValueView<'a>> {
        let (start, len) = self.primary_key_range();
        self.0
            .store
            .archived()
            .primary_key_slots
            .get(start..start.checked_add(len)?)?
            .iter()
            .find_map(|entry| {
                self.primary_key_matches(entry, key)
                    .then(|| self.0.store.value(entry.value))
            })
    }

    fn primary_key_matches(
        self,
        entry: &ArchivedPrimaryKeySlot,
        key: &[PrimaryKeyValue<'_>],
    ) -> bool {
        let Ok(start) = usize::try_from(entry.parts_start.to_native()) else {
            return false;
        };
        let Ok(len) = usize::try_from(entry.parts_len.to_native()) else {
            return false;
        };
        let Some(parts) = start
            .checked_add(len)
            .and_then(|end| self.0.store.archived().primary_key_parts.get(start..end))
        else {
            return false;
        };
        parts.len() == key.len()
            && parts.iter().zip(key).all(|(stored_part, requested_part)| {
                match (stored_part, requested_part) {
                    (ArchivedPrimaryKeyPart::Bool(stored), PrimaryKeyValue::Bool(requested)) => {
                        stored == requested
                    }
                    (ArchivedPrimaryKeyPart::I64(stored), PrimaryKeyValue::Int(requested)) => {
                        stored.to_native() == *requested
                    }
                    (ArchivedPrimaryKeyPart::U64(stored), PrimaryKeyValue::Int(requested)) => {
                        u64::try_from(*requested)
                            .is_ok_and(|converted| stored.to_native() == converted)
                    }
                    (
                        ArchivedPrimaryKeyPart::String(stored_id),
                        PrimaryKeyValue::Str(requested),
                    ) => self
                        .0
                        .store
                        .archived()
                        .strings
                        .get(usize::try_from(stored_id.0.to_native()).unwrap_or(usize::MAX))
                        .is_some_and(|stored| stored.as_ref() == *requested),
                    _ => false,
                }
            })
    }
}

/// Owned list handle retaining its memory-mapped store.
#[derive(Clone, Debug)]
pub struct ListHandle(ValueHandle);

impl ListHandle {
    /// Number of archived items.
    pub fn len(&self) -> usize {
        self.0.view().as_list().map_or(0, ListView::len)
    }

    /// Return whether the list is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return one item by position.
    pub fn get(&self, index: usize) -> Option<ValueHandle> {
        self.0
            .view()
            .as_list()?
            .get(index)
            .map(|value| ValueHandle {
                store: Arc::clone(&self.0.store),
                id: value.id,
            })
    }

    /// Look up an indexed-list item by its ordered primary-key components.
    pub fn get_by_primary_key(&self, key: &[PrimaryKeyValue<'_>]) -> Option<ValueHandle> {
        self.0
            .view()
            .as_list()?
            .get_by_primary_key(key)
            .map(|value| ValueHandle {
                store: Arc::clone(&self.0.store),
                id: value.id,
            })
    }
}

fn validate_header(bytes: &[u8]) -> Result<(), ArchiveError> {
    if bytes.get(..MAGIC.len()) != Some(MAGIC) {
        return Err(ArchiveError::Invalid(
            "missing validated-data magic".to_owned(),
        ));
    }
    let version = bytes
        .get(8..12)
        .and_then(|bytes| bytes.try_into().ok())
        .map(u32::from_le_bytes)
        .ok_or_else(|| ArchiveError::Invalid("truncated header".to_owned()))?;
    if version != FORMAT_VERSION {
        return Err(ArchiveError::Compatibility(format!(
            "archive version {version} is unsupported"
        )));
    }
    let policy = bytes
        .get(12..16)
        .and_then(|bytes| bytes.try_into().ok())
        .map(u32::from_le_bytes)
        .ok_or_else(|| ArchiveError::Invalid("truncated header".to_owned()))?;
    if policy != VALIDATION_POLICY_ID {
        return Err(ArchiveError::Compatibility(format!(
            "validation policy {policy} is unsupported"
        )));
    }
    Ok(())
}

fn validate_integrity(root: &ArchivedValidatedArchive) -> Result<(), ArchiveError> {
    let values = root.values.len();
    if usize::try_from(root.root.0.to_native()).map_or(true, |id| id >= values) {
        return Err(ArchiveError::Invalid(
            "root value id is outside the value table".to_owned(),
        ));
    }
    for value in root.values.iter() {
        let valid = match value {
            ArchivedValueNode::String(id) => {
                usize::try_from(id.0.to_native()).is_ok_and(|id| id < root.strings.len())
            }
            ArchivedValueNode::List {
                start,
                len,
                primary_key_start,
                primary_key_len,
                ..
            } => {
                range_fits(
                    start.to_native(),
                    len.to_native(),
                    root.sequence_slots.len(),
                ) && range_fits(
                    primary_key_start.to_native(),
                    primary_key_len.to_native(),
                    root.primary_key_slots.len(),
                )
            }
            ArchivedValueNode::Dict { start, len, .. } => {
                range_fits(start.to_native(), len.to_native(), root.map_slots.len())
            }
            ArchivedValueNode::Null
            | ArchivedValueNode::Bool(_)
            | ArchivedValueNode::I64(_)
            | ArchivedValueNode::U64(_) => true,
        };
        if !valid {
            return Err(ArchiveError::Invalid(
                "value node contains an invalid table reference".to_owned(),
            ));
        }
    }
    for slot in root.map_slots.iter() {
        if usize::try_from(slot.value.0.to_native()).map_or(true, |id| id >= values)
            || usize::try_from(slot.key.0.to_native()).map_or(true, |id| id >= root.strings.len())
        {
            return Err(ArchiveError::Invalid(
                "map slot contains an invalid id".to_owned(),
            ));
        }
    }
    for slot in root.sequence_slots.iter() {
        if usize::try_from(slot.value.0.to_native()).map_or(true, |id| id >= values) {
            return Err(ArchiveError::Invalid(
                "sequence slot contains an invalid value id".to_owned(),
            ));
        }
    }
    for slot in root.primary_key_slots.iter() {
        if usize::try_from(slot.value.0.to_native()).map_or(true, |id| id >= values)
            || !range_fits(
                slot.parts_start.to_native(),
                slot.parts_len.to_native(),
                root.primary_key_parts.len(),
            )
        {
            return Err(ArchiveError::Invalid(
                "primary key slot contains an invalid id or range".to_owned(),
            ));
        }
    }
    for part in root.primary_key_parts.iter() {
        if let ArchivedPrimaryKeyPart::String(id) = part
            && usize::try_from(id.0.to_native()).map_or(true, |id| id >= root.strings.len())
        {
            return Err(ArchiveError::Invalid(
                "primary key part contains an invalid string id".to_owned(),
            ));
        }
    }
    Ok(())
}

fn range_fits(start: u32, len: u32, table_len: usize) -> bool {
    usize::try_from(start)
        .ok()
        .zip(usize::try_from(len).ok())
        .and_then(|(start, len)| start.checked_add(len))
        .is_some_and(|end| end <= table_len)
}

fn write_atomically(destination: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let parent = destination.parent().unwrap_or_else(|| Path::new("."));
    let name = destination.file_name().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "destination has no file name",
        )
    })?;
    for _ in 0..100 {
        let sequence = TEMPORARY_FILE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let temporary = parent.join(format!(
            ".{}.{}.{}.tmp",
            name.to_string_lossy(),
            std::process::id(),
            sequence
        ));
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
        {
            Ok(mut file) => {
                let result = file
                    .write_all(bytes)
                    .and_then(|()| file.flush())
                    .and_then(|()| {
                        drop(file);
                        std::fs::rename(&temporary, destination)
                    });
                if result.is_err() {
                    let _ = std::fs::remove_file(&temporary);
                }
                return result;
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(error),
        }
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::AlreadyExists,
        "unable to reserve temporary archive",
    ))
}
#[allow(
    clippy::module_name_repetitions,
    reason = "The qualified public error name is explicit at call sites"
)]
/// Failure to validate, build, publish, or open a validated-data archive.
#[derive(Debug, derive_more::Display)]
pub enum ArchiveError {
    /// Schema validation setup failed.
    #[display("Validation failed: {_0}")]
    Validation(StoreValidateError),
    /// Archive construction or serialization failed.
    #[display("Could not build validated-data archive: {_0}")]
    Build(String),
    /// Archive bytes or identifiers are invalid.
    #[display("Invalid validated-data archive: {_0}")]
    Invalid(String),
    /// Archive does not match the required schema, registry, or policy.
    #[display("Incompatible validated-data archive: {_0}")]
    Compatibility(String),
    /// Filesystem operation failed.
    #[display("Archive I/O failed: {_0}")]
    Io(std::io::Error),
}
impl From<StoreValidateError> for ArchiveError {
    fn from(value: StoreValidateError) -> Self {
        Self::Validation(value)
    }
}
impl From<std::io::Error> for ArchiveError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

#[cfg(test)]
mod tests {
    use avdschema::Load as _;
    use avdschema::StoreSource;

    use super::*;

    const FIELDS: &[FieldDescriptor] = &[
        FieldDescriptor {
            id: 0,
            parent_model: 0,
            relation: FieldRelation::Key("name"),
            target_model: None,
        },
        FieldDescriptor {
            id: 1,
            parent_model: 0,
            relation: FieldRelation::Key("items"),
            target_model: Some(1),
        },
        FieldDescriptor {
            id: 2,
            parent_model: 1,
            relation: FieldRelation::Item,
            target_model: Some(2),
        },
        FieldDescriptor {
            id: 3,
            parent_model: 2,
            relation: FieldRelation::Key("enabled"),
            target_model: None,
        },
        FieldDescriptor {
            id: 4,
            parent_model: 2,
            relation: FieldRelation::Key("name"),
            target_model: None,
        },
    ];
    const PRIMARY_KEY_FIELDS: &[u32] = &[4];
    const PRIMARY_KEYS: &[PrimaryKeyDescriptor] = &[PrimaryKeyDescriptor {
        model: 1,
        field_slots: PRIMARY_KEY_FIELDS,
    }];
    const REGISTRY: ModelRegistry = ModelRegistry {
        root_model: 0,
        hash: [7; 32],
        fields: FIELDS,
        primary_keys: PRIMARY_KEYS,
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
            DataStore::from_file(&destination, schemas.archive_hash(), REGISTRY.hash)
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
            indexed_item.field(4).and_then(ValueView::as_str),
            Some("Ethernet1")
        );
        assert_eq!(
            items
                .get(0)
                .and_then(ValueView::as_dict)
                .and_then(|item| item.field(3))
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
                .field(4)
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
    fn rejects_invalid_value_reference_without_map_slots() {
        let directory = tempfile::tempdir().expect("temporary directory");
        let destination = directory.path().join("invalid.rkyv");
        let schemas = schemas();
        let archive = ValidatedArchive {
            schema_hash: schemas.archive_hash(),
            registry_hash: REGISTRY.hash,
            policy_id: VALIDATION_POLICY_ID,
            root: ValueId(0),
            values: vec![ValueNode::String(StringId(0))],
            map_slots: Vec::new(),
            sequence_slots: Vec::new(),
            primary_key_parts: Vec::new(),
            primary_key_slots: Vec::new(),
            strings: Vec::new(),
        };
        let archived = rkyv::to_bytes::<RkyvError>(&archive).expect("serializable archive");
        let mut bytes = Vec::with_capacity(HEADER_LENGTH + archived.len());
        bytes.extend_from_slice(MAGIC);
        bytes.extend_from_slice(&FORMAT_VERSION.to_le_bytes());
        bytes.extend_from_slice(&VALIDATION_POLICY_ID.to_le_bytes());
        bytes.extend_from_slice(archived.as_slice());
        std::fs::write(&destination, bytes).expect("write invalid archive");

        let error = DataStore::from_file(&destination, schemas.archive_hash(), REGISTRY.hash)
            .expect_err("invalid string id must be rejected");

        assert!(matches!(
            error,
            ArchiveError::Invalid(message)
                if message == "value node contains an invalid table reference"
        ));
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
