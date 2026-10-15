// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Attribute-derived binding metadata and interpreter-local Python type registration.
//!
//! The data-view declaration supplies both the Rust model descriptor and its Python exposure.
//! Registration follows only exposed relationships from the selected root, allocates all ordinary
//! types before contextual key identities, attaches the public class hierarchy, and finally installs
//! cached native field descriptors. Qualified names describe exposure, not additional data models.
//! No schema traversal or per-field generated Python function is needed during value access.

use std::collections::BTreeMap;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyModule, PyType};
use validated_data::{ArchiveModel, FieldDescriptor, ModelDescriptor, ModelKind};

use crate::collections::{CollectionInfo, NativeIndexedList, NativeList};
use crate::descriptors::{NativeDict, NativeField, ResolvedTarget};

/// Conversion to perform after reading a field or collection item.
#[derive(Clone, Copy, Debug)]
pub enum Target {
    /// Materialize a native Python scalar using its checked archive representation.
    Scalar,
    /// Wrap a collection using the referenced model's interpreter-local Python class.
    ///
    /// A function reference avoids eagerly expanding child metadata and permits cyclic type
    /// relationships in catalogs. It returns static metadata, never a borrowed data view.
    Model(fn() -> &'static ModelBinding),
    /// Wrap a relaxed payload without exposing descendant properties.
    Opaque,
}

/// One Python property, paired with its original Rust field descriptor.
#[derive(Clone, Copy, Debug)]
pub struct FieldBinding {
    /// Python property name, including keyword escaping.
    pub name: &'static str,
    /// Field descriptor emitted by the same declaration as the typed Rust accessor.
    pub descriptor: &'static FieldDescriptor,
    /// Conversion used when the field is present and non-null.
    pub target: Target,
}

/// Python exposure of one nominal model, derived alongside its typed Rust view.
#[derive(Clone, Copy, Debug)]
pub struct ModelBinding {
    /// Public Python class path relative to its module, such as `AVDDesign.DevicesItemBase`.
    ///
    /// Dotted paths attach existing classes to their enclosing types. Missing ancestors are
    /// namespace-only classes for a referenced subtree, not additional archive-backed views.
    pub name: &'static str,
    /// Original model shape, field relationships, and primary-key slots.
    pub descriptor: &'static ModelDescriptor,
    /// Statically exposed dictionary fields; dynamic relationships have no named property.
    pub fields: &'static [FieldBinding],
    /// Conversion for list items; absent for dictionaries and untyped lists.
    pub item: Option<Target>,
    /// Qualified contextual item class granting this list's primary-key presence guarantees.
    ///
    /// The class inherits the ordinary item class without copying its descriptors. Duplicate-key
    /// lists also use contextual items, but retain positional rather than unique-key operations.
    pub keyed_item_name: Option<&'static str>,
}

/// A data view whose declaration also describes its Python exposure.
///
/// Implemented by `#[data_view(python(name = "..."))]`. The core view crate does not depend on
/// `PyO3`; only declarations opting into Python bindings refer to this trait and crate.
pub trait PythonModel: ArchiveModel {
    /// Static binding metadata derived from the model declaration.
    const BINDING: ModelBinding;

    /// Return the prepared metadata without constructing or borrowing a data object.
    fn binding() -> &'static ModelBinding {
        &Self::BINDING
    }
}

fn reachable_models(
    root: &'static ModelBinding,
) -> PyResult<BTreeMap<&'static str, &'static ModelBinding>> {
    let mut models: BTreeMap<&'static str, &'static ModelBinding> = BTreeMap::new();
    let mut pending = vec![root];
    while let Some(model) = pending.pop() {
        if let Some(previous) = models.insert(model.name, model) {
            if previous.descriptor.identity != model.descriptor.identity {
                return Err(PyValueError::new_err(format!(
                    "conflicting Python model name: {}",
                    model.name
                )));
            }
            continue;
        }
        for target in model
            .fields
            .iter()
            .map(|field| field.target)
            .chain(model.item)
        {
            if let Target::Model(child) = target {
                pending.push(child());
            }
        }
    }
    Ok(models)
}

/// Resolve one public class path when preparing a descriptor or wrapping a root.
///
/// Field targets cache the returned type, so nested names incur no repeated lookup during access.
pub(crate) fn resolve_python_type<'py>(
    module: &Bound<'py, PyModule>,
    name: &str,
) -> PyResult<Bound<'py, PyType>> {
    let mut value = module.clone().into_any();
    for component in name.split('.') {
        value = value.getattr(component)?;
    }
    Ok(value.cast_into::<PyType>()?)
}

/// Prepare a class's Python identity before attaching fields or enclosing namespaces.
fn class_namespace<'py>(
    py: Python<'py>,
    module_name: &str,
    qualified_name: &str,
) -> PyResult<Bound<'py, PyDict>> {
    let namespace = PyDict::new(py);
    namespace.set_item("__module__", module_name)?;
    namespace.set_item("__qualname__", qualified_name)?;
    namespace.set_item("__slots__", ())?;
    Ok(namespace)
}

/// Attach canonical classes to their enclosing types without creating another model instance.
///
/// Partial cross-schema references may expose a leaf without its enclosing data models.
/// Missing ancestors are namespace-only classes. The temporary type index is discarded after
/// registration; the public hierarchy and cached descriptors own the interpreter-local types.
fn attach_python_hierarchy<'py>(
    module: &Bound<'py, PyModule>,
    module_name: &str,
    type_factory: &Bound<'py, PyAny>,
    types: &mut BTreeMap<String, Bound<'py, PyType>>,
) -> PyResult<()> {
    let py = module.py();
    let model_names: Vec<_> = types.keys().cloned().collect();
    for name in model_names {
        let mut prefix = String::new();
        for component in name.split('.').take(name.split('.').count() - 1) {
            if !prefix.is_empty() {
                prefix.push('.');
            }
            prefix.push_str(component);
            if !types.contains_key(&prefix) {
                let namespace = class_namespace(py, module_name, &prefix)?;
                let class = type_factory
                    .call1((component, (), namespace))?
                    .cast_into::<PyType>()?;
                types.insert(prefix.clone(), class);
            }
        }
    }
    for (name, class) in types.iter() {
        if let Some((parent, local_name)) = name.rsplit_once('.') {
            let parent = types.get(parent).ok_or_else(|| {
                PyValueError::new_err(format!("missing Python namespace: {parent}"))
            })?;
            parent.setattr(local_name, class)?;
        } else {
            if module.hasattr(name)? {
                return Err(PyValueError::new_err(format!(
                    "conflicting Python module attribute: {name}"
                )));
            }
            module.add(name, class)?;
        }
    }
    Ok(())
}

/// Install the Python-visible models reachable from one generated root.
///
/// Shared `PyO3` classes implement dictionary/list operations and property access. Registration
/// creates nominal `CPython` subclasses and descriptors from the prepared metadata; relaxed and
/// dynamic-only descendants are not registered. Child types and undefined values are cached in
/// interpreter-local, garbage-collected objects, not global Rust state.
/// The module also exposes `_Model`, `_Collection`, `_List`, and `_IndexedList` aliases to the native
/// bases for generated stub inheritance. Their generic parameters describe static typing only;
/// registration does not create additional wrapper classes or runtime generic specializations.
///
/// The caller must structurally check the archive with the matching Rust model before returning
/// a root wrapper. Registration describes access; it does not establish validation guarantees.
pub fn install_models<Model: PythonModel>(
    module: &Bound<'_, PyModule>,
    opaque: &Bound<'_, PyType>,
    undefined: &Bound<'_, PyAny>,
) -> PyResult<()> {
    install_root(module, Model::binding(), opaque, undefined)
}

pub(crate) fn install_root(
    module: &Bound<'_, PyModule>,
    root: &'static ModelBinding,
    opaque: &Bound<'_, PyType>,
    undefined: &Bound<'_, PyAny>,
) -> PyResult<()> {
    let models = reachable_models(root)?;
    let py = module.py();
    let type_factory = py.import("builtins")?.getattr("type")?;
    let sequence = py.import("collections.abc")?.getattr("Sequence")?;
    let module_name: String = module.name()?.extract()?;
    // Stub inheritance names the existing native bases, parameterized with each schema's
    // field/item types. Export aliases, not new classes or runtime typing wrappers.
    module.add("_Model", py.get_type::<NativeDict>())?;
    module.add("_Collection", py.get_type::<NativeList>())?;
    module.add("_List", py.get_type::<NativeList>())?;
    module.add("_IndexedList", py.get_type::<NativeIndexedList>())?;
    let mut types = BTreeMap::new();
    // Create every ordinary type first so schema order and forward references do not matter.
    for model in models.values() {
        let base = match model.descriptor.kind {
            ModelKind::Dict => py.get_type::<NativeDict>(),
            ModelKind::List if model.descriptor.indexed => py.get_type::<NativeIndexedList>(),
            ModelKind::List => py.get_type::<NativeList>(),
        };
        let namespace = class_namespace(py, &module_name, model.name)?;
        // Structural equality must not coexist with the inherited identity hash.
        namespace.set_item("__hash__", py.None())?;
        let local_name = model.name.rsplit('.').next().unwrap_or(model.name);
        let class = type_factory
            .call1((local_name, (base,), namespace))?
            .cast_into::<PyType>()?;
        if model.descriptor.kind == ModelKind::List && !model.descriptor.indexed {
            // Register only positional generated types, not their shared NativeList base:
            // keyed types also inherit that base but have incompatible indexing semantics.
            sequence.call_method1("register", (&class,))?;
        }
        types.insert(model.name.to_owned(), class);
    }
    for model in models.values() {
        if let Some(name) = model.keyed_item_name {
            if types.contains_key(name) {
                return Err(PyValueError::new_err(format!(
                    "conflicting contextual item name: {name}"
                )));
            }
            let Some(Target::Model(item)) = model.item else {
                return Err(PyValueError::new_err(
                    "primary-key items require a Python dictionary model",
                ));
            };
            let namespace = class_namespace(py, &module_name, name)?;
            let base = types
                .get(item().name)
                .ok_or_else(|| {
                    PyValueError::new_err(format!("missing Python item model: {}", item().name))
                })?
                .clone();
            let local_name = name.rsplit('.').next().unwrap_or(name);
            types.insert(
                name.to_owned(),
                type_factory
                    .call1((local_name, (base,), namespace))?
                    .cast_into::<PyType>()?,
            );
        }
    }
    attach_python_hierarchy(module, &module_name, &type_factory, &mut types)?;
    for model in models.values() {
        let class = resolve_python_type(module, model.name)?;
        if model.descriptor.kind == ModelKind::Dict {
            for field in model.fields {
                if class.hasattr(field.name)? {
                    return Err(PyValueError::new_err(format!(
                        "conflicting Python property: {}.{}",
                        model.name, field.name
                    )));
                }
                class.setattr(
                    field.name,
                    Py::new(
                        py,
                        NativeField::new(
                            field.descriptor.id,
                            ResolvedTarget::new(module, field.target, None, opaque, undefined)?,
                        ),
                    )?,
                )?;
            }
        } else {
            class.setattr(
                "_binding",
                Py::new(
                    py,
                    CollectionInfo::new(
                        ResolvedTarget::new(
                            module,
                            model.item.unwrap_or(Target::Scalar),
                            model.keyed_item_name,
                            opaque,
                            undefined,
                        )?,
                        model.descriptor.primary_key_fields.to_vec(),
                    ),
                )?,
            )?;
        }
    }
    Ok(())
}
