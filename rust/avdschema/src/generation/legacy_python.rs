#![allow(
    clippy::as_conversions,
    clippy::indexing_slicing,
    clippy::manual_let_else,
    clippy::struct_excessive_bools,
    clippy::too_many_lines,
    clippy::unreachable,
    reason = "The renderer consumes compiler-validated schema IDs and uses explicit plans"
)]

use std::fmt::Write as _;

use crate::CompileError;
use crate::SchemaGraph;
use crate::StoreSource;
use crate::compiled::CompiledStore;
use crate::compiled::CompiledValue;
use crate::compiled::SchemaId;
use crate::generation::OccurrenceId;
use crate::generation::OccurrenceKind;

const HEADER: &str = "# Copyright (c) 2026 Arista Networks, Inc.\n\
# Use of this source code is governed by the Apache License 2.0\n\
# that can be found in the LICENSE file.\n\n\
from __future__ import annotations\n";

/// Error raised while planning or rendering a generated artifact.
#[derive(Debug, derive_more::Display)]
pub enum GenerationError {
    /// Schema compilation failed.
    #[display("{_0}")]
    Compile(CompileError),
    /// The selected generator requires a dictionary root.
    #[display(
        "Schema '{schema_name}' has type '{found}', but Python model generation requires a dictionary root"
    )]
    RootType {
        schema_name: String,
        found: &'static str,
    },
    /// A requested root key is not present in the selected schema.
    #[display("Root key '{root_key}' was not found in schema '{schema_name}'")]
    UnknownRootKey {
        schema_name: String,
        root_key: String,
    },
    /// The selected occurrence uses a feature outside the current experiment.
    #[display("Python model generation does not yet support {feature} at '{schema_path}'")]
    Unsupported {
        schema_path: String,
        feature: &'static str,
    },
}

impl From<CompileError> for GenerationError {
    fn from(value: CompileError) -> Self {
        Self::Compile(value)
    }
}

/// Generate the current nested Python model source for one named schema.
pub fn generate_python_models(
    store: &StoreSource,
    schema_name: &str,
) -> Result<String, GenerationError> {
    generate_python_models_projection(store, schema_name, &class_name(schema_name), &[])
}

/// Generate a current Python model containing only selected root keys.
///
/// An empty `root_keys` slice includes every root key.
pub fn generate_python_models_projection(
    store: &StoreSource,
    schema_name: &str,
    generated_class_name: &str,
    root_keys: &[String],
) -> Result<String, GenerationError> {
    let graph = SchemaGraph::compile(store, schema_name)?;
    let root_occurrence = graph.occurrence_internal(graph.root());
    let OccurrenceKind::Dict { keys, dynamic_keys } = root_occurrence.kind() else {
        return Err(GenerationError::RootType {
            schema_name: schema_name.to_owned(),
            found: runtime_type(root_occurrence.schema_id()),
        });
    };
    if !dynamic_keys.is_empty() {
        return Err(GenerationError::Unsupported {
            schema_path: root_occurrence.path().join("/"),
            feature: "dynamic keys",
        });
    }
    if let Some(root_key) = root_keys
        .iter()
        .find(|root_key| !keys.contains_key(root_key.as_str()))
    {
        return Err(GenerationError::UnknownRootKey {
            schema_name: schema_name.to_owned(),
            root_key: root_key.clone(),
        });
    }
    if matches!(generated_class_name, "EosDesigns" | "EosCliConfigGen")
        || generated_class_name.ends_with("Protocol")
    {
        return Err(GenerationError::Unsupported {
            schema_path: schema_name.to_owned(),
            feature: "root-specific base models",
        });
    }
    for (root_key, occurrence_id) in keys {
        if root_keys.is_empty() || root_keys.iter().any(|selected| selected == root_key) {
            validate_supported_occurrence(&graph, *occurrence_id)?;
        }
    }
    let root = build_node(
        &graph,
        graph.root(),
        generated_class_name,
        (!root_keys.is_empty()).then_some(root_keys),
    );
    let mut imports = ImportSet::default();
    root.collect_imports(&mut imports);
    let mut output = String::from(HEADER);
    render_imports(&mut output, &imports);
    output.push_str("\n\n");
    root.render(&mut output, 0);
    while output.ends_with("\n\n") {
        output.pop();
    }
    output.push('\n');
    Ok(output)
}

fn validate_supported_occurrence(
    graph: &SchemaGraph,
    occurrence_id: OccurrenceId,
) -> Result<(), GenerationError> {
    let occurrence = graph.occurrence_internal(occurrence_id);
    if occurrence.retained_model_reference().is_some() {
        return Ok(());
    }
    let unsupported = |feature| GenerationError::Unsupported {
        schema_path: occurrence.path().join("/"),
        feature,
    };
    match occurrence.schema_id() {
        SchemaId::Bool(_) | SchemaId::Int(_) | SchemaId::Str(_) => {}
        SchemaId::List(index) => {
            let schema = &graph.compiled().lists[index as usize];
            if schema.common.default.is_some() {
                return Err(unsupported("list defaults"));
            }
            if schema.primary_key.is_some() && schema.allow_duplicate_primary_key {
                return Err(unsupported("lists with duplicate primary keys"));
            }
            let OccurrenceKind::List { items } = occurrence.kind() else {
                return Err(unsupported("a list occurrence without list children"));
            };
            let Some(items) = items else {
                return Err(unsupported("lists without an item schema"));
            };
            let item_occurrence = graph.occurrence_internal(*items);
            if matches!(item_occurrence.schema_id(), SchemaId::List(_)) {
                return Err(unsupported("nested lists"));
            }
            match item_occurrence.schema_id() {
                SchemaId::Int(item_index)
                    if graph.compiled().ints[item_index as usize]
                        .valid_values
                        .is_some() =>
                {
                    return Err(unsupported("literal scalar list items"));
                }
                SchemaId::Str(item_index)
                    if graph.compiled().strings[item_index as usize]
                        .valid_values
                        .is_some() =>
                {
                    return Err(unsupported("literal scalar list items"));
                }
                _ => validate_supported_occurrence(graph, *items)?,
            }
        }
        SchemaId::Dict(index) => {
            let schema = &graph.compiled().dicts[index as usize];
            if schema.common.default.is_some() {
                return Err(unsupported("dictionary defaults"));
            }
            let OccurrenceKind::Dict { keys, dynamic_keys } = occurrence.kind() else {
                return Err(unsupported(
                    "a dictionary occurrence without dictionary children",
                ));
            };
            if !dynamic_keys.is_empty() {
                return Err(unsupported("dynamic keys"));
            }
            for (key, child) in keys {
                if !is_python_identifier(key) {
                    return Err(unsupported("Python field aliases"));
                }
                validate_supported_occurrence(graph, *child)?;
            }
        }
    }
    Ok(())
}

#[derive(Clone, Debug)]
enum ClassPlan {
    Model(ModelPlan),
    List(ListPlan),
    Literal(LiteralPlan),
}

impl ClassPlan {
    fn render(&self, output: &mut String, level: usize) {
        match self {
            Self::Model(plan) => plan.render(output, level),
            Self::List(plan) => plan.render(output, level),
            Self::Literal(plan) => plan.render(output, level),
        }
    }

    fn collect_imports(&self, imports: &mut ImportSet) {
        match self {
            Self::Model(plan) => plan.collect_imports(imports),
            Self::List(plan) => plan.collect_imports(imports),
            Self::Literal(_) => imports.literal = true,
        }
    }
}

#[derive(Clone, Debug)]
struct ModelPlan {
    name: String,
    base: String,
    classes: Vec<ClassPlan>,
    fields: Vec<FieldPlan>,
    allow_other_keys: bool,
}

impl ModelPlan {
    fn render(&self, output: &mut String, level: usize) {
        line(
            output,
            level,
            &format!("class {}({}):", self.name, self.base),
        );
        line(
            output,
            level + 1,
            &format!("\"\"\"Subclass of {}.\"\"\"", self.base),
        );
        if !self.classes.is_empty() {
            output.push('\n');
            for (index, class) in self.classes.iter().enumerate() {
                if index > 0 {
                    output.push('\n');
                }
                class.render(output, level + 1);
            }
        }
        if !self.fields.is_empty() {
            if !matches!(self.classes.last(), Some(ClassPlan::Literal(_))) {
                output.push('\n');
            }
            self.render_fields(output, level + 1);
            output.push('\n');
            self.render_init(output, level + 1);
        }
    }

    fn render_fields(&self, output: &mut String, level: usize) {
        line(output, level, "_fields: ClassVar[dict] = {");
        for (index, field) in self.fields.iter().enumerate() {
            let comma = if index + 1 == self.fields.len() {
                ""
            } else {
                ","
            };
            line(
                output,
                level + 1,
                &format!(
                    "\"{}\": {{\"type\": {}{}}}{}",
                    field.key,
                    field.runtime_type,
                    field
                        .default
                        .as_ref()
                        .map(|default| format!(", \"default\": {default}"))
                        .unwrap_or_default(),
                    comma
                ),
            );
        }
        line(output, level, "}");
        if self.allow_other_keys {
            line(output, level, "_allow_other_keys: ClassVar[bool] = True");
        }
        for field in &self.fields {
            line(
                output,
                level,
                &format!("{}: {}", field.name, field.annotation(false)),
            );
            if let Some(docstring) = field.docstring() {
                render_docstring(output, level, &docstring);
            }
        }
    }

    fn render_init(&self, output: &mut String, level: usize) {
        line(output, level, "if TYPE_CHECKING:");
        line(output, level + 1, "def __init__(");
        line(output, level + 2, "self,");
        line(output, level + 2, "*,");
        for (index, field) in self.fields.iter().enumerate() {
            let comma = if index + 1 == self.fields.len() {
                ""
            } else {
                ","
            };
            line(
                output,
                level + 2,
                &format!(
                    "{}: {} = Undefined{}",
                    field.name,
                    field.annotation(true),
                    comma
                ),
            );
        }
        line(output, level + 1, ")-> None:");
        line(output, level + 2, "\"\"\"");
        line(output, level + 2, &format!("{}.", self.name));
        output.push('\n');
        line(output, level + 2, &format!("Subclass of {}.", self.base));
        output.push('\n');
        line(output, level + 2, "Args:");
        for field in &self.fields {
            match field.description.as_deref() {
                Some(description) if description.contains('\n') => {
                    line(output, level + 3, &format!("{}:", field.name));
                    for wrapped in wrap_description(description, 100) {
                        let _ = writeln!(output, "{}   {wrapped}", "    ".repeat(level + 3));
                    }
                }
                Some(description) => {
                    line(output, level + 3, &format!("{}: {description}", field.name));
                }
                None => line(
                    output,
                    level + 3,
                    &format!("{}: {}", field.name, field.name),
                ),
            }
        }
        output.push('\n');
        line(output, level + 2, "\"\"\"");
    }

    fn collect_imports(&self, imports: &mut ImportSet) {
        imports.class_var = !self.fields.is_empty() || self.allow_other_keys;
        imports.avd_model = true;
        for class in &self.classes {
            class.collect_imports(imports);
        }
        for field in &self.fields {
            if let Some(reference) = &field.external_reference {
                if reference.starts_with("EosDesigns.") {
                    imports.eos_designs = true;
                } else if reference.starts_with("EosCliConfigGen.") {
                    imports.eos_cli = true;
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
struct ListPlan {
    name: String,
    base: String,
    item_type: String,
    primary_key: Option<String>,
    description: String,
}

impl ListPlan {
    fn render(&self, output: &mut String, level: usize) {
        line(
            output,
            level,
            &format!("class {}({}):", self.name, self.base),
        );
        render_docstring(output, level + 1, &self.description);
        if let Some(primary_key) = &self.primary_key {
            line(
                output,
                level + 1,
                &format!("_primary_key: ClassVar[str] = \"{primary_key}\""),
            );
        }
        output.push('\n');
        line(
            output,
            level,
            &format!("{}._item_type = {}", self.name, self.item_type),
        );
    }

    fn collect_imports(&self, imports: &mut ImportSet) {
        imports.class_var |= self.primary_key.is_some();
        if self.primary_key.is_some() {
            imports.avd_indexed_list = true;
        } else {
            imports.avd_list = true;
        }
    }
}

#[derive(Clone, Debug)]
struct LiteralPlan {
    name: String,
    values: Vec<String>,
}

impl LiteralPlan {
    fn render(&self, output: &mut String, level: usize) {
        line(
            output,
            level,
            &format!(
                "{}: TypeAlias = Literal[{}]",
                self.name,
                self.values.join(", ")
            ),
        );
    }
}

#[derive(Clone, Debug)]
struct FieldPlan {
    name: String,
    key: String,
    runtime_type: String,
    type_hint: String,
    optional: bool,
    default: Option<String>,
    description: Option<String>,
    external_reference: Option<String>,
}

impl FieldPlan {
    fn annotation(&self, include_undefined: bool) -> String {
        let mut types = vec![self.type_hint.clone()];
        if include_undefined {
            types.push("UndefinedType".to_owned());
        }
        if self.optional
            && self.default.is_none()
            && matches!(self.runtime_type.as_str(), "str" | "int" | "bool")
        {
            types.push("None".to_owned());
        }
        types.join(" | ")
    }

    fn docstring(&self) -> Option<String> {
        match (&self.description, &self.default) {
            (Some(description), Some(default)) => {
                Some(format!("{description}\n\nDefault value: `{default}`"))
            }
            (Some(description), None) => Some(description.clone()),
            (None, Some(default)) => Some(format!("Default value: `{default}`")),
            (None, None) => None,
        }
    }
}

fn build_node(
    graph: &SchemaGraph,
    occurrence_id: OccurrenceId,
    name: &str,
    root_keys: Option<&[String]>,
) -> ClassPlan {
    let occurrence = graph.occurrence_internal(occurrence_id);
    match occurrence.schema_id() {
        SchemaId::Dict(index) => {
            let schema = &graph.compiled().dicts[index as usize];
            let keys = match occurrence.kind() {
                OccurrenceKind::Dict { keys, .. } => keys,
                _ => unreachable!(),
            };
            let mut classes = Vec::new();
            let mut fields = Vec::new();
            for (key, child_id) in keys {
                if root_keys
                    .is_some_and(|root_keys| !root_keys.iter().any(|selected| selected == key))
                {
                    continue;
                }
                let child = graph.occurrence_internal(*child_id);
                if is_removed(graph.compiled(), child.schema_id()) {
                    continue;
                }
                let child_name = class_name(key);
                let is_primary_key = false;
                let (mut child_classes, field) =
                    build_field(graph, *child_id, key, &child_name, is_primary_key);
                classes.append(&mut child_classes);
                fields.push(field);
            }
            ClassPlan::Model(ModelPlan {
                name: name.to_owned(),
                base: "AvdModel".to_owned(),
                classes,
                fields,
                allow_other_keys: schema.allow_other_keys,
            })
        }
        _ => unreachable!(),
    }
}

fn build_field(
    graph: &SchemaGraph,
    occurrence_id: OccurrenceId,
    key: &str,
    generated_name: &str,
    primary_key: bool,
) -> (Vec<ClassPlan>, FieldPlan) {
    let occurrence = graph.occurrence_internal(occurrence_id);
    if let Some(reference) = occurrence.retained_model_reference() {
        let reference_name = class_name_from_ref(reference);
        let mut field = field_plan(
            graph,
            occurrence.schema_id(),
            key,
            &reference_name,
            &reference_name,
            primary_key,
            Some(reference_name.clone()),
        );
        field
            .description
            .clone_from(&common(graph.compiled(), occurrence.schema_id()).description);
        return (Vec::new(), field);
    }
    match occurrence.schema_id() {
        SchemaId::Bool(_) => (
            Vec::new(),
            field_plan(
                graph,
                occurrence.schema_id(),
                key,
                "bool",
                "bool",
                primary_key,
                None,
            ),
        ),
        SchemaId::Int(index) => {
            let schema = &graph.compiled().ints[index as usize];
            let classes = schema
                .valid_values
                .as_ref()
                .map_or_else(Vec::new, |values| {
                    vec![ClassPlan::Literal(LiteralPlan {
                        name: generated_name.to_owned(),
                        values: values.iter().map(ToString::to_string).collect(),
                    })]
                });
            let type_hint = if classes.is_empty() {
                "int"
            } else {
                generated_name
            };
            (
                classes,
                field_plan(
                    graph,
                    occurrence.schema_id(),
                    key,
                    "int",
                    type_hint,
                    primary_key,
                    None,
                ),
            )
        }
        SchemaId::Str(index) => {
            let schema = &graph.compiled().strings[index as usize];
            let classes = schema
                .valid_values
                .as_ref()
                .map_or_else(Vec::new, |values| {
                    vec![ClassPlan::Literal(LiteralPlan {
                        name: generated_name.to_owned(),
                        values: values.iter().map(|value| format!("\"{value}\"")).collect(),
                    })]
                });
            let type_hint = if classes.is_empty() {
                "str"
            } else {
                generated_name
            };
            (
                classes,
                field_plan(
                    graph,
                    occurrence.schema_id(),
                    key,
                    "str",
                    type_hint,
                    primary_key,
                    None,
                ),
            )
        }
        SchemaId::Dict(index) => {
            let schema = &graph.compiled().dicts[index as usize];
            if schema.keys.is_empty() {
                return (
                    Vec::new(),
                    field_plan(
                        graph,
                        occurrence.schema_id(),
                        key,
                        "dict",
                        "dict",
                        primary_key,
                        None,
                    ),
                );
            }
            let class = build_node(graph, occurrence_id, generated_name, None);
            (
                vec![class],
                field_plan(
                    graph,
                    occurrence.schema_id(),
                    key,
                    generated_name,
                    generated_name,
                    primary_key,
                    None,
                ),
            )
        }
        SchemaId::List(index) => {
            let schema = &graph.compiled().lists[index as usize];
            let items = match occurrence.kind() {
                OccurrenceKind::List { items } => *items,
                _ => None,
            };
            let item_name = match (schema.items, items) {
                (Some(SchemaId::Dict(_)), Some(item_id)) => {
                    let item_name = format!("{generated_name}Item");
                    let primary_key_name = schema.primary_key.as_deref();
                    let item = build_dict_item(graph, item_id, &item_name, primary_key_name);
                    let description =
                        list_description(schema.primary_key.as_deref(), &item_name, graph, item_id);
                    let list = list_plan(
                        generated_name,
                        &item_name,
                        schema.primary_key.as_deref(),
                        description.clone(),
                        graph,
                        item_id,
                    );
                    let mut field = field_plan(
                        graph,
                        occurrence.schema_id(),
                        key,
                        generated_name,
                        generated_name,
                        primary_key,
                        None,
                    );
                    field.description = Some(description);
                    return (vec![item, ClassPlan::List(list)], field);
                }
                (Some(SchemaId::Str(_)), _) => "str".to_owned(),
                (Some(SchemaId::Int(_)), _) => "int".to_owned(),
                (Some(SchemaId::Bool(_)), _) => "bool".to_owned(),
                _ => "Any".to_owned(),
            };
            let description = format!("Subclass of AvdList with `{item_name}` items.");
            let list = ListPlan {
                name: generated_name.to_owned(),
                base: format!("AvdList[{item_name}]"),
                item_type: item_name,
                primary_key: None,
                description: description.clone(),
            };
            let mut field = field_plan(
                graph,
                occurrence.schema_id(),
                key,
                generated_name,
                generated_name,
                primary_key,
                None,
            );
            field.description = Some(description);
            (vec![ClassPlan::List(list)], field)
        }
    }
}

fn build_dict_item(
    graph: &SchemaGraph,
    occurrence_id: OccurrenceId,
    name: &str,
    primary_key: Option<&str>,
) -> ClassPlan {
    let occurrence = graph.occurrence_internal(occurrence_id);
    let SchemaId::Dict(index) = occurrence.schema_id() else {
        unreachable!();
    };
    let schema = &graph.compiled().dicts[index as usize];
    let keys = match occurrence.kind() {
        OccurrenceKind::Dict { keys, .. } => keys,
        _ => unreachable!(),
    };
    let mut classes = Vec::new();
    let mut fields = Vec::new();
    for (key, child_id) in keys {
        let child = graph.occurrence_internal(*child_id);
        if is_removed(graph.compiled(), child.schema_id()) {
            continue;
        }
        let child_name = class_name(key);
        let (mut child_classes, field) = build_field(
            graph,
            *child_id,
            key,
            &child_name,
            primary_key == Some(key.as_str()),
        );
        classes.append(&mut child_classes);
        fields.push(field);
    }
    ClassPlan::Model(ModelPlan {
        name: name.to_owned(),
        base: "AvdModel".to_owned(),
        classes,
        fields,
        allow_other_keys: schema.allow_other_keys,
    })
}

fn list_plan(
    name: &str,
    item_name: &str,
    primary_key: Option<&str>,
    description: String,
    graph: &SchemaGraph,
    item_id: OccurrenceId,
) -> ListPlan {
    let primary_key_type = primary_key
        .and_then(
            |primary_key| match graph.occurrence_internal(item_id).kind() {
                OccurrenceKind::Dict { keys, .. } => keys.get(primary_key).copied(),
                _ => None,
            },
        )
        .map_or("str", |id| {
            runtime_type(graph.occurrence_internal(id).schema_id())
        });
    ListPlan {
        name: name.to_owned(),
        base: primary_key.map_or_else(
            || format!("AvdList[{item_name}]"),
            |_| format!("AvdIndexedList[{primary_key_type}, {item_name}]"),
        ),
        item_type: item_name.to_owned(),
        primary_key: primary_key.map(ToOwned::to_owned),
        description,
    }
}

fn list_description(
    primary_key: Option<&str>,
    item_name: &str,
    graph: &SchemaGraph,
    item_id: OccurrenceId,
) -> String {
    match primary_key {
        Some(primary_key) => {
            let primary_key_type = match graph.occurrence_internal(item_id).kind() {
                OccurrenceKind::Dict { keys, .. } => keys.get(primary_key).map_or("str", |id| {
                    runtime_type(graph.occurrence_internal(*id).schema_id())
                }),
                _ => "str",
            };
            format!(
                "Subclass of AvdIndexedList with `{item_name}` items. Primary key is `{primary_key}` (`{primary_key_type}`)."
            )
        }
        None => format!("Subclass of AvdList with `{item_name}` items."),
    }
}

fn field_plan(
    graph: &SchemaGraph,
    schema_id: SchemaId,
    key: &str,
    runtime_type: &str,
    type_hint: &str,
    primary_key: bool,
    external_reference: Option<String>,
) -> FieldPlan {
    let common = common(graph.compiled(), schema_id);
    let default = common.default.as_ref().map(render_default);
    let description = common.description.clone().or_else(|| {
        matches!(schema_id, SchemaId::Dict(_) | SchemaId::List(_))
            .then(|| auto_description(graph, schema_id, type_hint))
    });
    FieldPlan {
        name: key.to_owned(),
        key: key.to_owned(),
        runtime_type: runtime_type.to_owned(),
        type_hint: type_hint.to_owned(),
        optional: !common.required && !primary_key,
        default,
        description,
        external_reference,
    }
}

fn auto_description(graph: &SchemaGraph, schema_id: SchemaId, type_name: &str) -> String {
    match schema_id {
        SchemaId::Dict(_) => "Subclass of AvdModel.".to_owned(),
        SchemaId::List(index) => {
            let schema = &graph.compiled().lists[index as usize];
            if let Some(primary_key) = &schema.primary_key {
                format!(
                    "Subclass of AvdIndexedList with `{type_name}Item` items. Primary key is `{primary_key}` (`str`).",
                )
            } else {
                let item = schema.items.map_or("Any", runtime_type);
                format!("Subclass of AvdList with `{item}` items.")
            }
        }
        _ => String::new(),
    }
}

fn common(store: &CompiledStore, schema_id: SchemaId) -> &crate::compiled::Common {
    match schema_id {
        SchemaId::Bool(index) => &store.bools[index as usize].common,
        SchemaId::Int(index) => &store.ints[index as usize].common,
        SchemaId::Str(index) => &store.strings[index as usize].common,
        SchemaId::List(index) => &store.lists[index as usize].common,
        SchemaId::Dict(index) => &store.dicts[index as usize].common,
    }
}

fn is_removed(store: &CompiledStore, schema_id: SchemaId) -> bool {
    common(store, schema_id)
        .deprecation
        .as_ref()
        .is_some_and(|deprecation| deprecation.removed)
}

fn runtime_type(schema_id: SchemaId) -> &'static str {
    match schema_id {
        SchemaId::Bool(_) => "bool",
        SchemaId::Int(_) => "int",
        SchemaId::Str(_) => "str",
        SchemaId::List(_) => "list",
        SchemaId::Dict(_) => "dict",
    }
}

fn render_default(value: &CompiledValue) -> String {
    match value {
        CompiledValue::Null => "None".to_owned(),
        CompiledValue::Bool(value) => if *value { "True" } else { "False" }.to_owned(),
        CompiledValue::I64(value) => value.to_string(),
        CompiledValue::U64(value) => value.to_string(),
        CompiledValue::F64(bits) => f64::from_bits(*bits).to_string(),
        CompiledValue::String(value) => format!("\"{value}\""),
        CompiledValue::List(values) => format!(
            "[{}]",
            values
                .iter()
                .map(render_default)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        CompiledValue::Object(values) => format!(
            "{{{}}}",
            values
                .iter()
                .map(|(key, child)| format!("\"{key}\": {}", render_default(child)))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

#[derive(Default)]
struct ImportSet {
    class_var: bool,
    literal: bool,
    avd_model: bool,
    avd_list: bool,
    avd_indexed_list: bool,
    eos_designs: bool,
    eos_cli: bool,
}

fn render_imports(output: &mut String, imports: &ImportSet) {
    output.push('\n');
    if imports.eos_cli {
        output.push_str("from pyavd._eos_cli_config_gen.schema import EosCliConfigGen\n");
    }
    if imports.eos_designs {
        output.push_str("from pyavd._eos_designs.schema import EosDesigns\n");
    }
    if imports.class_var {
        output.push_str("from typing import ClassVar\n");
    }
    if imports.literal {
        output.push_str("from typing import Literal, TypeAlias\n");
    }
    output.push_str("from typing import TYPE_CHECKING\n");
    output.push('\n');
    if imports.avd_indexed_list || imports.avd_list || imports.avd_model {
        output.push('\n');
    }
    if imports.avd_indexed_list {
        output.push_str("from pyavd._schema.models.avd_indexed_list import AvdIndexedList\n");
    }
    if imports.avd_list {
        output.push_str("from pyavd._schema.models.avd_list import AvdList\n");
    }
    if imports.avd_model {
        output.push_str("from pyavd._schema.models.avd_model import AvdModel\n");
    }
    output.push_str("\nif TYPE_CHECKING:\n");
    output.push_str("    from pyavd._utils import Undefined, UndefinedType\n");
}

fn class_name(value: &str) -> String {
    value
        .split('_')
        .map(|element| {
            let mut chars = element.chars();
            chars
                .next()
                .map(|first| first.to_uppercase().collect::<String>() + chars.as_str())
                .unwrap_or_default()
        })
        .collect()
}

fn class_name_from_ref(reference: &str) -> String {
    let (schema_name, path) = reference.split_once('#').unwrap_or((reference, ""));
    let mut parts = vec![class_name(schema_name)];
    let elements = path.split('/').collect::<Vec<_>>();
    for (index, element) in elements.iter().enumerate() {
        if element.is_empty() || matches!(*element, "keys" | "items") {
            continue;
        }
        let suffix = (elements.get(index + 1) == Some(&"items")).then_some("_item");
        parts.push(class_name(&format!(
            "{element}{}",
            suffix.unwrap_or_default()
        )));
    }
    parts.join(".")
}

fn is_python_identifier(value: &str) -> bool {
    !matches!(
        value,
        "False"
            | "None"
            | "True"
            | "and"
            | "as"
            | "assert"
            | "async"
            | "await"
            | "break"
            | "class"
            | "continue"
            | "def"
            | "del"
            | "elif"
            | "else"
            | "except"
            | "finally"
            | "for"
            | "from"
            | "global"
            | "if"
            | "import"
            | "in"
            | "is"
            | "lambda"
            | "nonlocal"
            | "not"
            | "or"
            | "pass"
            | "raise"
            | "return"
            | "try"
            | "while"
            | "with"
            | "yield"
    ) && value.chars().all(|character| {
        character == '_' || character.is_ascii_lowercase() || character.is_ascii_digit()
    })
}

fn render_docstring(output: &mut String, level: usize, value: &str) {
    if value.contains('\n') {
        line(output, level, "\"\"\"");
        for wrapped in wrap_description(value, 100) {
            line(output, level, &wrapped);
        }
        line(output, level, "\"\"\"");
    } else {
        line(output, level, &format!("\"\"\"{value}\"\"\""));
    }
}

fn wrap_description(value: &str, width: usize) -> Vec<String> {
    let mut result = Vec::new();
    let normalized = value.replace("\n\n", "\n");
    let mut logical_length: usize = 0;
    let mut physical = String::new();
    for (line_index, source_line) in normalized.lines().enumerate() {
        if line_index > 0 {
            if !physical.is_empty() {
                result.push(std::mem::take(&mut physical));
            }
            logical_length = logical_length.saturating_add(1);
        }
        for word in source_line.split_whitespace() {
            let separator = usize::from(!physical.is_empty());
            if logical_length + separator + word.len() > width {
                if !physical.is_empty() {
                    result.push(std::mem::take(&mut physical));
                }
                logical_length = 0;
            }
            if !physical.is_empty() {
                physical.push(' ');
                logical_length += 1;
            }
            physical.push_str(word);
            logical_length += word.len();
        }
    }
    if !physical.is_empty() {
        result.push(physical);
    }
    result
}

fn line(output: &mut String, level: usize, value: &str) {
    let _ = writeln!(output, "{}{value}", "    ".repeat(level));
}
