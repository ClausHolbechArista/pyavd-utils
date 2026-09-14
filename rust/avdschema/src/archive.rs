// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Validated, read-only access to an archived compiled schema store.

#![allow(
    clippy::mem_forget,
    reason = "self_cell internally uses mem::forget to safely construct its self-referential owner"
)]

#[cfg(feature = "dump_load_files")]
use std::path::Path;
use std::sync::OnceLock;

use fancy_regex::Regex;
use fancy_regex::RegexBuilder;
#[cfg(all(feature = "mmap", not(target_family = "wasm")))]
use mmap_guard::FileData;
use ordermap::OrderMap;
use rkyv::rancor::Error as RkyvError;
use self_cell::self_cell;

use crate::DynamicKeyOverrides;
use crate::Load as _;
use crate::SchemaDataMapping;
use crate::SchemaDataSequence as _;
use crate::SchemaDataValue as _;
use crate::StoreSource;
use crate::compiled::ARCHIVE_FORMAT_VERSION;
use crate::compiled::ARCHIVE_HEADER_LENGTH;
use crate::compiled::ARCHIVE_MAGIC;
use crate::compiled::ArchivedBoolSchema;
use crate::compiled::ArchivedCompiledDeprecation;
use crate::compiled::ArchivedCompiledDocumentationOptions;
use crate::compiled::ArchivedCompiledStore;
use crate::compiled::ArchivedCompiledStringFormat;
use crate::compiled::ArchivedCompiledValue;
use crate::compiled::ArchivedDictSchema;
use crate::compiled::ArchivedIntSchema;
use crate::compiled::ArchivedListSchema;
use crate::compiled::ArchivedSchemaId;
use crate::compiled::ArchivedStrSchema;
use crate::compiled::CompiledStore;
use crate::compiled::SchemaId;

enum ArchiveBytes {
    #[cfg(all(feature = "mmap", not(target_family = "wasm")))]
    Mapped(FileData),
    Owned(rkyv::util::AlignedVec),
}

impl AsRef<[u8]> for ArchiveBytes {
    fn as_ref(&self) -> &[u8] {
        match self {
            #[cfg(all(feature = "mmap", not(target_family = "wasm")))]
            Self::Mapped(data) => data.as_ref(),
            Self::Owned(data) => data.as_slice(),
        }
    }
}

struct ArchiveRoot<'a>(&'a ArchivedCompiledStore);

self_cell!(
    struct ArchiveCell {
        owner: ArchiveBytes,

        #[covariant]
        dependent: ArchiveRoot,
    }
);

/// Immutable schema archive plus process-local derived caches.
pub struct Store {
    archive: ArchiveCell,
    compiled_patterns: Vec<OnceLock<Result<Regex, String>>>,
}

impl std::fmt::Debug for Store {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Store")
            .field("roots", &self.archived().roots.len())
            .field("compiled_patterns", &self.compiled_patterns.len())
            .finish_non_exhaustive()
    }
}

impl Store {
    /// Load a JSON schema source and compile it into process-owned archived bytes.
    pub fn from_json(json: &str) -> Result<Self, StoreError> {
        let source = StoreSource::from_json(json)
            .map_err(|error| StoreError::InvalidSource(error.to_string()))?;
        Self::compile(&source)
    }

    /// Load gzip-compressed JSON schema source and compile it into process-owned archived bytes.
    #[cfg(feature = "gzip")]
    pub fn from_gz_bytes(bytes: &[u8]) -> Result<Self, StoreError> {
        let source = StoreSource::from_gz_bytes(bytes)
            .map_err(|error| StoreError::InvalidSource(error.to_string()))?;
        Self::compile(&source)
    }

    /// Memory-map and validate a compiled schema archive.
    #[cfg(all(feature = "mmap", not(target_family = "wasm")))]
    pub fn from_file(path: &Path) -> Result<Self, StoreError> {
        let bytes = mmap_guard::map_file(path)?;
        Self::from_bytes(ArchiveBytes::Mapped(bytes))
    }

    /// Compile a raw schema store into process-owned archived bytes.
    pub fn compile(source: &StoreSource) -> Result<Self, StoreError> {
        let bytes = CompiledStore::compile(source)?.to_bytes()?;
        Self::from_bytes(ArchiveBytes::Owned(bytes))
    }

    /// Compile one named root and its reachable schema nodes into owned bytes.
    pub fn compile_schema(source: &StoreSource, schema_name: &str) -> Result<Self, StoreError> {
        let bytes = CompiledStore::compile_schema(source, schema_name)?.to_bytes()?;
        Self::from_bytes(ArchiveBytes::Owned(bytes))
    }

    /// Compile a source store and atomically write its archived runtime representation.
    #[cfg(feature = "dump_load_files")]
    pub fn compile_to_file(source: &StoreSource, destination: &Path) -> Result<(), StoreError> {
        CompiledStore::compile_to_file(source, destination)?;
        Ok(())
    }

    fn from_bytes(bytes: ArchiveBytes) -> Result<Self, StoreError> {
        let archive = ArchiveCell::try_new(bytes, |bytes| {
            validate_header(bytes.as_ref())?;
            rkyv::access::<ArchivedCompiledStore, RkyvError>(bytes.as_ref())
                .map_err(|error| StoreError::InvalidArchive(error.to_string()))
                .and_then(|store| {
                    validate_integrity(store)?;
                    Ok(ArchiveRoot(store))
                })
        })?;
        let pattern_count = archive.borrow_dependent().0.strings.len();
        Ok(Self {
            archive,
            compiled_patterns: std::iter::repeat_with(OnceLock::new)
                .take(pattern_count)
                .collect(),
        })
    }

    /// Return the root schema view for a schema name, including AVD aliases.
    pub fn get(&self, schema_name: &str) -> Option<SchemaView<'_>> {
        let archived = self.archived();
        let id = archived.roots.get(schema_name).or_else(|| {
            let alias = match schema_name {
                "eos_designs" => "avd_design",
                "eos_cli_config_gen" => "eos_config",
                "avd_design" => "eos_designs",
                "eos_config" => "eos_cli_config_gen",
                _ => return None,
            };
            archived.roots.get(alias)
        })?;
        Some(
            SchemaCursor {
                store: self,
                id: native_schema_id(*id),
            }
            .view(),
        )
    }

    /// Return the primary key for the list schema at a static data path.
    ///
    /// Numeric path components traverse list items. This intentionally only
    /// supports static keys, matching the existing EOS config helper contract.
    pub fn get_list_primary_key(
        &self,
        schema_name: &str,
        data_path: &[String],
    ) -> Result<Option<&str>, SchemaPathError> {
        let empty_data = serde_json::Value::Object(serde_json::Map::new());
        let Some(view) = self.get_schema_from_path(schema_name, data_path, &empty_data, None)?
        else {
            return Ok(None);
        };
        Ok(match view {
            SchemaView::List(schema) => schema.primary_key(),
            SchemaView::Bool(_) | SchemaView::Int(_) | SchemaView::Str(_) | SchemaView::Dict(_) => {
                None
            }
        })
    }

    /// Return the effective schema covering a data path.
    ///
    /// Dynamic keys are resolved at the root dictionary from the supplied
    /// data and optional caller overrides. Nested dynamic keys are not
    /// supported, matching the existing path-helper contract.
    pub fn get_schema_from_path<'store, 'input, V>(
        &'store self,
        schema_name: &str,
        data_path: &[String],
        data_value: V,
        dynamic_key_overrides: Option<&DynamicKeyOverrides>,
    ) -> Result<Option<SchemaView<'store>>, SchemaPathError>
    where
        V: crate::SchemaDataValue<'input>,
    {
        let Some(mut view) = self.get(schema_name) else {
            return Err(SchemaPathError::InvalidSchemaName(schema_name.to_owned()));
        };
        let mut path = data_path.iter();
        let Some(root_key) = path.next() else {
            return Ok(Some(view));
        };
        let SchemaView::Dict(root_schema) = view else {
            return Err(SchemaPathError::SchemaNotDict);
        };
        let input = data_value
            .as_mapping()
            .ok_or(SchemaPathError::ValueNotADict)?;
        view = if let Some(static_schema) = root_schema.key(root_key) {
            static_schema
        } else {
            let dynamic_keys = resolve_dynamic_keys(root_schema, input, dynamic_key_overrides);
            let Some(dynamic_schema) = dynamic_keys.get(root_key).copied() else {
                return Ok(None);
            };
            dynamic_schema
        };

        for component in path {
            view = match view {
                SchemaView::Dict(schema) => {
                    let Some(child) = schema.key(component) else {
                        return Ok(None);
                    };
                    child
                }
                SchemaView::List(schema) if component.parse::<usize>().is_ok() => {
                    let Some(items) = schema.items() else {
                        return Ok(None);
                    };
                    items
                }
                SchemaView::List(_) => return Ok(None),
                SchemaView::Bool(_) | SchemaView::Int(_) | SchemaView::Str(_) => {
                    return Err(SchemaPathError::InvalidTraversal);
                }
            };
        }
        Ok(Some(view))
    }

    fn archived(&self) -> &ArchivedCompiledStore {
        self.archive.borrow_dependent().0
    }

    fn pattern(&self, index: u32, pattern: &str) -> Result<&Regex, &str> {
        let Some(cell) = usize::try_from(index)
            .ok()
            .and_then(|index| self.compiled_patterns.get(index))
        else {
            return Err("string schema index is outside the pattern cache");
        };
        cell.get_or_init(|| {
            RegexBuilder::new(format!("^(?:{pattern})$").as_str())
                // Keep the Perl classes `\d`, `\s`, and `\w` enabled with ASCII semantics.
                // This disables their Unicode expansion and properties such as `\p{Greek}`.
                .unicode_mode(false)
                .build()
                .map_err(|error| error.to_string())
        })
        .as_ref()
        .map_err(String::as_str)
    }
}

#[derive(Clone, Debug, derive_more::Display)]
pub enum SchemaPathError {
    #[display("Schema name '{_0}' not found in the schema store")]
    InvalidSchemaName(String),
    #[display("Root schema is not a dictionary")]
    SchemaNotDict,
    #[display("Root data is not a dictionary")]
    ValueNotADict,
    #[display("Data path cannot be traversed through this schema node")]
    InvalidTraversal,
}

/// Resolve the concrete keys covered by an effective dictionary's dynamic keys.
///
/// Input values take precedence over schema defaults. Caller overrides are
/// applied last. Removed dynamic-key schemas are excluded.
pub fn resolve_dynamic_keys<'store, 'input, M>(
    schema: DictView<'store>,
    input: M,
    overrides: Option<&DynamicKeyOverrides>,
) -> OrderMap<String, SchemaView<'store>>
where
    M: SchemaDataMapping<'input>,
{
    let mut resolved = OrderMap::new();
    for (path, dynamic_schema) in schema.dynamic_keys() {
        if dynamic_schema
            .deprecation()
            .is_some_and(DeprecationView::removed)
        {
            continue;
        }
        let values = dynamic_values_at_path(path, input).or_else(|| {
            schema
                .default_dynamic_keys(path)
                .map(|values| values.map(ToOwned::to_owned).collect())
        });
        for key in values.into_iter().flatten() {
            resolved.insert(key, dynamic_schema);
        }
    }
    if let Some(overrides) = overrides {
        for (concrete_key, path) in overrides {
            let Some(dynamic_schema) = schema.dynamic_key(path) else {
                continue;
            };
            if dynamic_schema
                .deprecation()
                .is_some_and(DeprecationView::removed)
            {
                continue;
            }
            resolved.insert(concrete_key.clone(), dynamic_schema);
        }
    }
    resolved
}

fn dynamic_values_at_path<'input, M>(key_path: &str, input: M) -> Option<Vec<String>>
where
    M: SchemaDataMapping<'input>,
{
    let mut path = key_path.split('.');
    path.next()
        .and_then(|root_key| input.get(root_key).map(|value| (root_key, value)))
        .map(|(root_key, value)| {
            value
                .walk(path, Some(&mut vec![root_key.to_owned()]))
                .into_values()
                .flat_map(|value| {
                    if let Some(string) = value.as_str() {
                        return vec![string.to_owned()];
                    }
                    value
                        .as_sequence()
                        .map(|items| {
                            items
                                .iter()
                                .filter_map(|item| item.as_str().map(ToOwned::to_owned))
                                .collect()
                        })
                        .unwrap_or_default()
                })
                .collect()
        })
}

/// Internal identifier of one schema node inside an archive.
#[derive(Clone, Copy, Debug)]
struct SchemaCursor<'a> {
    store: &'a Store,
    id: SchemaId,
}

impl<'a> SchemaCursor<'a> {
    fn view(self) -> SchemaView<'a> {
        match self.id {
            SchemaId::Bool(index) => SchemaView::Bool(BoolView {
                schema: table_get(&self.store.archived().bools, index),
            }),
            SchemaId::Int(index) => SchemaView::Int(IntView {
                schema: table_get(&self.store.archived().ints, index),
            }),
            SchemaId::Str(index) => SchemaView::Str(StrView {
                cursor: self,
                schema: table_get(&self.store.archived().strings, index),
            }),
            SchemaId::List(index) => SchemaView::List(ListView {
                cursor: self,
                schema: table_get(&self.store.archived().lists, index),
            }),
            SchemaId::Dict(index) => SchemaView::Dict(DictView {
                cursor: self,
                schema: table_get(&self.store.archived().dicts, index),
            }),
        }
    }
}

/// Typed borrowed view of one schema node in a compiled store.
#[derive(Clone, Copy, Debug)]
pub enum SchemaView<'a> {
    Bool(BoolView<'a>),
    Int(IntView<'a>),
    Str(StrView<'a>),
    List(ListView<'a>),
    Dict(DictView<'a>),
}

impl<'a> SchemaView<'a> {
    pub fn common(self) -> CommonView<'a> {
        match self {
            Self::Bool(view) => view.common(),
            Self::Int(view) => view.common(),
            Self::Str(view) => view.common(),
            Self::List(view) => view.common(),
            Self::Dict(view) => view.common(),
        }
    }

    pub fn required(self) -> bool {
        self.common().required()
    }

    pub fn deprecation(self) -> Option<DeprecationView<'a>> {
        self.common().deprecation()
    }

    pub fn default(self) -> Option<SchemaValueView<'a>> {
        self.common().default()
    }

    pub fn display_name(self) -> Option<&'a str> {
        self.common().display_name()
    }

    pub fn description(self) -> Option<&'a str> {
        self.common().description()
    }

    pub fn documentation_options(self) -> Option<DocumentationOptionsView<'a>> {
        self.common().documentation_options()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CommonView<'a>(&'a crate::compiled::ArchivedCommon);

impl<'a> CommonView<'a> {
    pub fn required(self) -> bool {
        self.0.required
    }

    pub fn deprecation(self) -> Option<DeprecationView<'a>> {
        self.0.deprecation.as_ref().map(DeprecationView)
    }

    pub fn default(self) -> Option<SchemaValueView<'a>> {
        self.0.default.as_ref().map(SchemaValueView::from)
    }

    pub fn display_name(self) -> Option<&'a str> {
        self.0.display_name.as_ref().map(AsRef::as_ref)
    }

    pub fn description(self) -> Option<&'a str> {
        self.0.description.as_ref().map(AsRef::as_ref)
    }

    pub fn documentation_options(self) -> Option<DocumentationOptionsView<'a>> {
        self.0
            .documentation_options
            .as_ref()
            .map(DocumentationOptionsView)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DocumentationOptionsView<'a>(&'a ArchivedCompiledDocumentationOptions);

impl<'a> DocumentationOptionsView<'a> {
    pub fn table(self) -> Option<&'a str> {
        self.0.table.as_ref().map(AsRef::as_ref)
    }

    pub fn hide_keys(self) -> bool {
        self.0.hide_keys
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SchemaValueView<'a> {
    Null,
    Bool(bool),
    I64(i64),
    U64(u64),
    F64(f64),
    String(&'a str),
    List(SchemaListValueView<'a>),
    Object(SchemaObjectValueView<'a>),
}

#[derive(Clone, Copy, Debug)]
pub struct SchemaListValueView<'a>(&'a rkyv::vec::ArchivedVec<ArchivedCompiledValue>);

impl<'a> SchemaListValueView<'a> {
    pub fn iter(self) -> impl Iterator<Item = SchemaValueView<'a>> {
        self.0.iter().map(SchemaValueView::from)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SchemaObjectValueView<'a>(
    &'a rkyv::vec::ArchivedVec<
        rkyv::tuple::ArchivedTuple2<rkyv::string::ArchivedString, ArchivedCompiledValue>,
    >,
);

impl<'a> SchemaObjectValueView<'a> {
    pub fn iter(self) -> impl Iterator<Item = (&'a str, SchemaValueView<'a>)> {
        self.0
            .iter()
            .map(|entry| (entry.0.as_ref(), SchemaValueView::from(&entry.1)))
    }
}

impl<'a> From<&'a ArchivedCompiledValue> for SchemaValueView<'a> {
    fn from(value: &'a ArchivedCompiledValue) -> Self {
        match value {
            ArchivedCompiledValue::Null => Self::Null,
            ArchivedCompiledValue::Bool(value) => Self::Bool(*value),
            ArchivedCompiledValue::I64(value) => Self::I64(value.to_native()),
            ArchivedCompiledValue::U64(value) => Self::U64(value.to_native()),
            ArchivedCompiledValue::F64(value) => Self::F64(f64::from_bits(value.to_native())),
            ArchivedCompiledValue::String(value) => Self::String(value.as_ref()),
            ArchivedCompiledValue::List(values) => Self::List(SchemaListValueView(values)),
            ArchivedCompiledValue::Object(values) => Self::Object(SchemaObjectValueView(values)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StringFormatView {
    Cidr,
    Ip,
    IpPool,
    Ipv4,
    Ipv4Cidr,
    Ipv4Pool,
    Ipv6,
    Ipv6Cidr,
    Ipv6Pool,
    Mac,
}

impl From<ArchivedCompiledStringFormat> for StringFormatView {
    fn from(value: ArchivedCompiledStringFormat) -> Self {
        match value {
            ArchivedCompiledStringFormat::Cidr => Self::Cidr,
            ArchivedCompiledStringFormat::Ip => Self::Ip,
            ArchivedCompiledStringFormat::IpPool => Self::IpPool,
            ArchivedCompiledStringFormat::Ipv4 => Self::Ipv4,
            ArchivedCompiledStringFormat::Ipv4Cidr => Self::Ipv4Cidr,
            ArchivedCompiledStringFormat::Ipv4Pool => Self::Ipv4Pool,
            ArchivedCompiledStringFormat::Ipv6 => Self::Ipv6,
            ArchivedCompiledStringFormat::Ipv6Cidr => Self::Ipv6Cidr,
            ArchivedCompiledStringFormat::Ipv6Pool => Self::Ipv6Pool,
            ArchivedCompiledStringFormat::Mac => Self::Mac,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DeprecationView<'a>(&'a ArchivedCompiledDeprecation);

impl<'a> DeprecationView<'a> {
    pub fn warning(self) -> bool {
        self.0.warning
    }
    pub fn removed(self) -> bool {
        self.0.removed
    }
    pub fn allow_with_new_key(self) -> bool {
        self.0.allow_with_new_key
    }
    pub fn new_key(self) -> Option<&'a str> {
        self.0.new_key.as_ref().map(AsRef::as_ref)
    }
    pub fn remove_in_version(self) -> Option<&'a str> {
        self.0.remove_in_version.as_ref().map(AsRef::as_ref)
    }
    pub fn remove_after_date(self) -> Option<&'a str> {
        self.0.remove_after_date.as_ref().map(AsRef::as_ref)
    }
    pub fn url(self) -> Option<&'a str> {
        self.0.url.as_ref().map(AsRef::as_ref)
    }
    pub fn upgrade_handler(self) -> Option<&'a str> {
        self.0.upgrade_handler.as_ref().map(AsRef::as_ref)
    }
}

macro_rules! scalar_view {
    ($name:ident, $schema:ty) => {
        #[derive(Clone, Copy, Debug)]
        pub struct $name<'a> {
            schema: &'a $schema,
        }

        impl<'a> $name<'a> {
            pub fn common(self) -> CommonView<'a> {
                CommonView(&self.schema.common)
            }
        }
    };
}

scalar_view!(BoolView, ArchivedBoolSchema);
scalar_view!(IntView, ArchivedIntSchema);

#[derive(Clone, Copy, Debug)]
pub struct StrView<'a> {
    cursor: SchemaCursor<'a>,
    schema: &'a ArchivedStrSchema,
}

impl<'a> StrView<'a> {
    pub fn common(self) -> CommonView<'a> {
        CommonView(&self.schema.common)
    }
}

impl<'a> IntView<'a> {
    pub fn min(self) -> Option<i64> {
        self.schema.min.as_ref().map(|value| value.to_native())
    }
    pub fn max(self) -> Option<i64> {
        self.schema.max.as_ref().map(|value| value.to_native())
    }
    pub fn valid_values(self) -> Option<impl Iterator<Item = i64> + 'a> {
        self.schema
            .valid_values
            .as_ref()
            .map(|values| values.iter().map(|value| value.to_native()))
    }
    pub fn dynamic_valid_values(self) -> Option<impl Iterator<Item = &'a str> + 'a> {
        self.schema
            .dynamic_valid_values
            .as_ref()
            .map(|values| values.iter().map(AsRef::as_ref))
    }
    pub fn convert_types(self) -> Option<impl Iterator<Item = &'a str> + 'a> {
        self.schema
            .convert_types
            .as_ref()
            .map(|values| values.iter().map(AsRef::as_ref))
    }
}

impl<'a> StrView<'a> {
    pub fn convert_to_lower_case(self) -> bool {
        self.schema.convert_to_lower_case
    }
    pub fn min_length(self) -> Option<u64> {
        self.schema
            .min_length
            .as_ref()
            .map(|value| value.to_native())
    }
    pub fn max_length(self) -> Option<u64> {
        self.schema
            .max_length
            .as_ref()
            .map(|value| value.to_native())
    }
    pub fn pattern(self) -> Option<&'a str> {
        self.schema.pattern.as_ref().map(AsRef::as_ref)
    }
    pub fn valid_values(self) -> Option<impl Iterator<Item = &'a str> + 'a> {
        self.schema
            .valid_values
            .as_ref()
            .map(|values| values.iter().map(AsRef::as_ref))
    }
    pub fn dynamic_valid_values(self) -> Option<impl Iterator<Item = &'a str> + 'a> {
        self.schema
            .dynamic_valid_values
            .as_ref()
            .map(|values| values.iter().map(AsRef::as_ref))
    }
    pub fn convert_types(self) -> Option<impl Iterator<Item = &'a str> + 'a> {
        self.schema
            .convert_types
            .as_ref()
            .map(|values| values.iter().map(AsRef::as_ref))
    }
    pub fn format(self) -> Option<StringFormatView> {
        self.schema.format.as_ref().map(|format| (*format).into())
    }
    pub fn compiled_pattern(self) -> Option<Result<&'a Regex, &'a str>> {
        let pattern = self.pattern()?;
        let SchemaId::Str(index) = self.cursor.id else {
            return None;
        };
        Some(self.cursor.store.pattern(index, pattern))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ListView<'a> {
    cursor: SchemaCursor<'a>,
    schema: &'a ArchivedListSchema,
}

impl<'a> ListView<'a> {
    pub fn common(self) -> CommonView<'a> {
        CommonView(&self.schema.common)
    }
    pub fn items(self) -> Option<SchemaView<'a>> {
        self.schema.items.as_ref().map(|id| {
            SchemaCursor {
                store: self.cursor.store,
                id: native_schema_id(*id),
            }
            .view()
        })
    }
    pub fn min_length(self) -> Option<u64> {
        self.schema
            .min_length
            .as_ref()
            .map(|value| value.to_native())
    }
    pub fn max_length(self) -> Option<u64> {
        self.schema
            .max_length
            .as_ref()
            .map(|value| value.to_native())
    }
    pub fn primary_key(self) -> Option<&'a str> {
        self.schema.primary_key.as_ref().map(AsRef::as_ref)
    }
    pub fn unique_keys(self) -> Option<impl Iterator<Item = &'a str> + 'a> {
        self.schema
            .unique_keys
            .as_ref()
            .map(|values| values.iter().map(AsRef::as_ref))
    }
    pub fn allow_duplicate_primary_key(self) -> bool {
        self.schema.allow_duplicate_primary_key
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DictView<'a> {
    cursor: SchemaCursor<'a>,
    schema: &'a ArchivedDictSchema,
}

impl<'a> DictView<'a> {
    pub fn common(self) -> CommonView<'a> {
        CommonView(&self.schema.common)
    }
    pub fn key(self, key: &str) -> Option<SchemaView<'a>> {
        self.schema.keys.get(key).map(|id| {
            SchemaCursor {
                store: self.cursor.store,
                id: native_schema_id(*id),
            }
            .view()
        })
    }
    pub fn dynamic_key(self, path: &str) -> Option<SchemaView<'a>> {
        self.schema.dynamic_keys.get(path).map(|id| {
            SchemaCursor {
                store: self.cursor.store,
                id: native_schema_id(*id),
            }
            .view()
        })
    }
    pub fn keys(self) -> impl Iterator<Item = (&'a str, SchemaView<'a>)> + 'a {
        self.schema.keys.iter().map(|(key, id)| {
            let view = SchemaCursor {
                store: self.cursor.store,
                id: native_schema_id(*id),
            }
            .view();
            (key.as_ref(), view)
        })
    }
    pub fn dynamic_keys(self) -> impl Iterator<Item = (&'a str, SchemaView<'a>)> + 'a {
        self.schema.dynamic_keys.iter().map(|(key, id)| {
            let view = SchemaCursor {
                store: self.cursor.store,
                id: native_schema_id(*id),
            }
            .view();
            (key.as_ref(), view)
        })
    }
    pub fn default_dynamic_keys(self, path: &str) -> Option<impl Iterator<Item = &'a str> + 'a> {
        self.schema
            .default_dynamic_keys
            .get(path)
            .map(|values| values.iter().map(AsRef::as_ref))
    }
    pub fn allow_other_keys(self) -> bool {
        self.schema.allow_other_keys
    }
    pub fn begin_relaxed_validation(self) -> bool {
        self.schema.begin_relaxed_validation
    }
    pub fn has_schema_keys(self) -> bool {
        !self.schema.keys.is_empty() || !self.schema.dynamic_keys.is_empty()
    }
}

fn native_schema_id(id: ArchivedSchemaId) -> SchemaId {
    match id {
        ArchivedSchemaId::Bool(index) => SchemaId::Bool(index.to_native()),
        ArchivedSchemaId::Int(index) => SchemaId::Int(index.to_native()),
        ArchivedSchemaId::Str(index) => SchemaId::Str(index.to_native()),
        ArchivedSchemaId::List(index) => SchemaId::List(index.to_native()),
        ArchivedSchemaId::Dict(index) => SchemaId::Dict(index.to_native()),
    }
}

fn table_get<T>(table: &[T], index: u32) -> &T {
    match usize::try_from(index)
        .ok()
        .and_then(|index| table.get(index))
    {
        Some(value) => value,
        None => invalid_schema_id(),
    }
}

#[allow(
    clippy::panic,
    reason = "archive integrity is validated before any SchemaView can be created"
)]
fn invalid_schema_id() -> ! {
    panic!("validated schema archive contains an invalid schema id")
}

#[derive(Debug, derive_more::Display)]
pub enum StoreError {
    #[display("Input is not a compiled AVD schema archive")]
    NotArchive,
    #[display("Unsupported compiled schema archive version {found}; expected {expected}")]
    UnsupportedVersion {
        found: u32,
        expected: u32,
    },
    #[display("Invalid compiled schema archive: {_0}")]
    InvalidArchive(String),
    #[display("Invalid schema source: {_0}")]
    InvalidSource(String),
    Io(std::io::Error),
    Compile(crate::compiled::CompileError),
}

impl From<std::io::Error> for StoreError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<crate::compiled::CompileError> for StoreError {
    fn from(error: crate::compiled::CompileError) -> Self {
        Self::Compile(error)
    }
}

fn validate_header(bytes: &[u8]) -> Result<(), StoreError> {
    if bytes.get(..ARCHIVE_MAGIC.len()) != Some(ARCHIVE_MAGIC) {
        return Err(StoreError::NotArchive);
    }
    if bytes.len() < ARCHIVE_HEADER_LENGTH {
        return Err(StoreError::InvalidArchive(
            "archive header is truncated".to_owned(),
        ));
    }
    let version_bytes: [u8; 4] = bytes
        .get(ARCHIVE_MAGIC.len()..ARCHIVE_MAGIC.len() + 4)
        .and_then(|bytes| bytes.try_into().ok())
        .ok_or(StoreError::NotArchive)?;
    let found = u32::from_le_bytes(version_bytes);
    if found != ARCHIVE_FORMAT_VERSION {
        return Err(StoreError::UnsupportedVersion {
            found,
            expected: ARCHIVE_FORMAT_VERSION,
        });
    }
    Ok(())
}

fn validate_integrity(store: &ArchivedCompiledStore) -> Result<(), StoreError> {
    for id in store.roots.values() {
        validate_schema_id(store, *id)?;
    }
    for schema in store.lists.iter() {
        if let Some(id) = schema.items.as_ref() {
            validate_schema_id(store, *id)?;
        }
    }
    for schema in store.dicts.iter() {
        for id in schema.keys.values().chain(schema.dynamic_keys.values()) {
            validate_schema_id(store, *id)?;
        }
    }
    Ok(())
}

fn validate_schema_id(
    store: &ArchivedCompiledStore,
    id: ArchivedSchemaId,
) -> Result<(), StoreError> {
    let valid = match id {
        ArchivedSchemaId::Bool(index) => {
            usize::try_from(index.to_native()).is_ok_and(|index| index < store.bools.len())
        }
        ArchivedSchemaId::Int(index) => {
            usize::try_from(index.to_native()).is_ok_and(|index| index < store.ints.len())
        }
        ArchivedSchemaId::Str(index) => {
            usize::try_from(index.to_native()).is_ok_and(|index| index < store.strings.len())
        }
        ArchivedSchemaId::List(index) => {
            usize::try_from(index.to_native()).is_ok_and(|index| index < store.lists.len())
        }
        ArchivedSchemaId::Dict(index) => {
            usize::try_from(index.to_native()).is_ok_and(|index| index < store.dicts.len())
        }
    };
    if valid {
        Ok(())
    } else {
        Err(StoreError::InvalidArchive(format!(
            "schema id {id:?} is outside its typed table"
        )))
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write as _;

    use indexmap::IndexMap;
    use serde_json::json;

    use super::*;
    use crate::compiled::CompiledStore;

    fn metadata_store() -> Store {
        Store::from_json(
            &json!({
                "base_str": {
                    "type": "str",
                    "default": "base",
                    "description": "base description",
                    "required": true,
                    "min_length": 2,
                    "max_length": 20,
                    "pattern": "[a-z]+",
                    "format": "mac",
                    "valid_values": ["base", "local"],
                    "dynamic_valid_values": ["choices.names"],
                    "convert_types": ["int"],
                    "documentation_options": {"table": "base table"},
                    "deprecation": {
                        "warning": true,
                        "new_key": "replacement",
                        "allow_with_new_key": true,
                        "removed": false,
                        "remove_in_version": "6.0.0",
                        "remove_after_date": "2030-01-01",
                        "url": "https://example.test",
                        "upgrade_handler": "rename"
                    }
                },
                "base_dict": {
                    "type": "dict",
                    "allow_other_keys": true,
                    "keys": {"inherited": {"type": "bool"}}
                },
                "test": {
                    "type": "dict",
                    "default": {"enabled": true, "names": ["one", "two"]},
                    "display_name": "Test schema",
                    "documentation_options": {"table": "root", "hide_keys": true},
                    "keys": {
                        "name": {
                            "type": "str",
                            "$ref": "base_str#",
                            "default": "local",
                            "display_name": "Name"
                        },
                        "count": {
                            "type": "int",
                            "min": 1,
                            "max": 10,
                            "valid_values": [1, 2],
                            "dynamic_valid_values": ["counts"],
                            "convert_types": ["str"]
                        },
                        "items": {
                            "type": "list",
                            "min_length": 1,
                            "max_length": 4,
                            "primary_key": "id",
                            "unique_keys": ["name"],
                            "allow_duplicate_primary_key": true,
                            "items": {"type": "dict", "keys": {"id": {"type": "str"}}}
                        },
                        "relaxed": {
                            "type": "dict",
                            "$ref": "base_dict#",
                            "relaxed_validation": true,
                            "keys": {"local": {"type": "bool"}}
                        }
                    },
                    "dynamic_keys": {"names": {"type": "bool"}}
                }
            })
            .to_string(),
        )
        .expect("metadata schema should compile")
    }

    #[test]
    fn compiled_patterns_preserve_full_match_and_ascii_contract() {
        let store = Store::from_json(
            &json!({
                "alternation": {"type": "str", "pattern": "foo|bar"},
                "perl_classes": {"type": "str", "pattern": r"\d+\s+\d+"},
                "lookahead": {"type": "str", "pattern": "(?=[a-z])(?=.*[0-9])[a-z0-9]+"},
                "variable_lookbehind": {"type": "str", "pattern": "(?<=a+)b"},
                "unicode_property": {"type": "str", "pattern": r"\p{Greek}+"}
            })
            .to_string(),
        )
        .expect("pattern schemas should compile");

        let Some(SchemaView::Str(alternation)) = store.get("alternation") else {
            panic!("alternation schema should compile as a string")
        };
        let alternation = alternation.compiled_pattern().unwrap().unwrap();
        assert!(alternation.is_match("foo").unwrap());
        assert!(alternation.is_match("bar").unwrap());
        assert!(!alternation.is_match("foobar").unwrap());

        for name in ["perl_classes", "lookahead", "variable_lookbehind"] {
            let Some(SchemaView::Str(schema)) = store.get(name) else {
                panic!("{name} schema should compile as a string")
            };
            assert!(schema.compiled_pattern().unwrap().is_ok());
        }

        let Some(SchemaView::Str(unicode_property)) = store.get("unicode_property") else {
            panic!("unicode_property schema should compile as a string")
        };
        assert!(unicode_property.compiled_pattern().unwrap().is_err());
    }

    #[test]
    fn views_expose_effective_metadata() {
        let store = metadata_store();
        let SchemaView::Dict(root) = store.get("test").expect("test root should exist") else {
            panic!("test root should be a dict")
        };
        assert_eq!(root.common().display_name(), Some("Test schema"));
        let docs = root
            .common()
            .documentation_options()
            .expect("root documentation options should exist");
        assert_eq!(docs.table(), Some("root"));
        assert!(docs.hide_keys());
        let SchemaValueView::Object(default) =
            root.common().default().expect("default should exist")
        else {
            panic!("root default should be an object")
        };
        assert_eq!(default.iter().count(), 2);

        let SchemaView::Str(name) = root.key("name").expect("name should exist") else {
            panic!("name should be a string")
        };
        assert!(matches!(
            name.common().default(),
            Some(SchemaValueView::String("local"))
        ));
        assert_eq!(name.common().display_name(), Some("Name"));
        assert_eq!(name.common().description(), Some("base description"));
        assert!(name.common().required());
        assert_eq!(name.min_length(), Some(2));
        assert_eq!(name.max_length(), Some(20));
        assert_eq!(name.pattern(), Some("[a-z]+"));
        assert_eq!(name.format(), Some(StringFormatView::Mac));
        assert_eq!(
            name.valid_values().unwrap().collect::<Vec<_>>(),
            ["base", "local"]
        );
        assert_eq!(
            name.dynamic_valid_values().unwrap().collect::<Vec<_>>(),
            ["choices.names"]
        );
        assert_eq!(name.convert_types().unwrap().collect::<Vec<_>>(), ["int"]);
        let deprecation = name
            .common()
            .deprecation()
            .expect("deprecation should exist");
        assert!(deprecation.warning());
        assert!(deprecation.allow_with_new_key());
        assert!(!deprecation.removed());
        assert_eq!(deprecation.new_key(), Some("replacement"));
        assert_eq!(deprecation.remove_in_version(), Some("6.0.0"));
        assert_eq!(deprecation.remove_after_date(), Some("2030-01-01"));
        assert_eq!(deprecation.url(), Some("https://example.test"));
        assert_eq!(deprecation.upgrade_handler(), Some("rename"));

        let SchemaView::Int(count) = root.key("count").expect("count should exist") else {
            panic!("count should be an integer")
        };
        assert_eq!(count.min(), Some(1));
        assert_eq!(count.max(), Some(10));
        assert_eq!(count.valid_values().unwrap().collect::<Vec<_>>(), [1, 2]);
        assert_eq!(
            count.dynamic_valid_values().unwrap().collect::<Vec<_>>(),
            ["counts"]
        );
        assert_eq!(count.convert_types().unwrap().collect::<Vec<_>>(), ["str"]);

        let SchemaView::List(items) = root.key("items").expect("items should exist") else {
            panic!("items should be a list")
        };
        assert_eq!(items.min_length(), Some(1));
        assert_eq!(items.max_length(), Some(4));
        assert_eq!(items.primary_key(), Some("id"));
        assert_eq!(items.unique_keys().unwrap().collect::<Vec<_>>(), ["name"]);
        assert!(items.allow_duplicate_primary_key());
        assert!(matches!(items.items(), Some(SchemaView::Dict(_))));

        let SchemaView::Dict(relaxed) = root.key("relaxed").expect("relaxed should exist") else {
            panic!("relaxed should be a dict")
        };
        assert!(relaxed.allow_other_keys());
        assert!(relaxed.begin_relaxed_validation());
        assert!(relaxed.key("inherited").is_some());
        assert!(relaxed.key("local").is_some());
    }

    #[test]
    fn path_navigation_resolves_dynamic_keys_and_static_precedence() {
        let store = metadata_store();
        let data = json!({"names": ["dynamic_name"], "name": "configured"});
        assert!(matches!(
            store
                .get_schema_from_path("test", &["dynamic_name".into()], &data, None)
                .unwrap(),
            Some(SchemaView::Bool(_))
        ));
        assert!(matches!(
            store
                .get_schema_from_path("test", &["name".into()], &data, None)
                .unwrap(),
            Some(SchemaView::Str(_))
        ));
        let overrides = DynamicKeyOverrides::from_iter([("forced".into(), "names".into())]);
        assert!(matches!(
            store
                .get_schema_from_path("test", &["forced".into()], &data, Some(&overrides))
                .unwrap(),
            Some(SchemaView::Bool(_))
        ));
    }

    #[test]
    fn constructors_reject_non_archives_versions_and_invalid_ids() {
        assert!(matches!(
            Store::from_bytes(ArchiveBytes::Owned(rkyv::util::AlignedVec::new())),
            Err(StoreError::NotArchive)
        ));

        let mut truncated = rkyv::util::AlignedVec::new();
        truncated.extend_from_slice(ARCHIVE_MAGIC);
        assert!(matches!(
            Store::from_bytes(ArchiveBytes::Owned(truncated)),
            Err(StoreError::InvalidArchive(_))
        ));

        let mut versioned = CompiledStore::default().to_bytes().unwrap();
        versioned[ARCHIVE_MAGIC.len()..ARCHIVE_MAGIC.len() + 4]
            .copy_from_slice(&(ARCHIVE_FORMAT_VERSION + 1).to_le_bytes());
        assert!(matches!(
            Store::from_bytes(ArchiveBytes::Owned(versioned)),
            Err(StoreError::UnsupportedVersion { .. })
        ));

        let invalid = CompiledStore {
            roots: IndexMap::from_iter([("invalid".into(), SchemaId::Dict(0))]),
            ..Default::default()
        }
        .to_bytes()
        .unwrap();
        assert!(matches!(
            Store::from_bytes(ArchiveBytes::Owned(invalid)),
            Err(StoreError::InvalidArchive(_))
        ));
    }

    #[cfg(feature = "gzip")]
    #[test]
    fn gzip_source_constructor_compiles_runtime_store() {
        let source = json!({"test": {"type": "bool"}}).to_string();
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
        encoder.write_all(source.as_bytes()).unwrap();
        let bytes = encoder.finish().unwrap();
        let store = Store::from_gz_bytes(&bytes).unwrap();
        assert!(matches!(store.get("test"), Some(SchemaView::Bool(_))));
    }

    #[cfg(all(feature = "mmap", not(target_family = "wasm")))]
    #[test]
    fn file_constructor_maps_archives() {
        let source = crate::utils::test_utils::get_tmp_file(&format!(
            "archive-source-{}.json",
            std::process::id()
        ));
        let archive = crate::utils::test_utils::get_tmp_file(&format!(
            "archive-runtime-{}.rkyv",
            std::process::id()
        ));
        std::fs::write(&source, r#"{"test":{"type":"bool"}}"#).unwrap();

        let source_model = StoreSource::from_file(Some(&source)).unwrap();
        Store::compile_to_file(&source_model, &archive).unwrap();
        let mapped_store = Store::from_file(&archive).unwrap();
        assert!(matches!(
            mapped_store.get("test"),
            Some(SchemaView::Bool(_))
        ));

        std::fs::remove_file(source).unwrap();
        std::fs::remove_file(archive).unwrap();
    }
}
