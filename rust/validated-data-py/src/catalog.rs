// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Generated catalogs and two-phase Python type registration.

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyModule, PyType};

use crate::collections::{CollectionInfo, NativeIndexedList, NativeList};
use crate::descriptors::{NativeDict, NativeField, ResolvedTarget};

/// Conversion to perform after reading a field or collection item.
#[derive(Clone, Copy, Debug)]
pub enum Target {
    /// Materialize a native Python scalar.
    Scalar,
    /// Wrap a collection in the named generated class.
    Model(&'static str),
    /// Wrap a relaxed payload without exposing descendant properties.
    Opaque,
}

/// One statically named dictionary property.
#[derive(Clone, Copy, Debug)]
pub struct FieldBinding {
    /// Python property name, including keyword escaping.
    pub name: &'static str,
    /// Model-local archived field slot.
    pub slot: u32,
    /// Conversion used when the field is present and non-null.
    pub target: Target,
}

/// Python operations exposed by one nominal model.
#[derive(Clone, Copy, Debug)]
pub enum ModelShape {
    /// Dictionary with generated native descriptors.
    Dict(&'static [FieldBinding]),
    /// Positional collection; duplicate-key lists retain this shape.
    List(Target),
    /// Unique-key collection. Primary keys are read from these item slots.
    Indexed(Target, &'static [u32]),
    /// Contextual primary-key item identity, inheriting the original dictionary properties.
    Alias(&'static str),
}

/// One class in the generated Python binding catalog.
#[derive(Clone, Copy, Debug)]
pub struct ModelBinding {
    /// Public, module-local Python class name.
    pub name: &'static str,
    /// Dictionary, collection, or contextual identity operations.
    pub shape: ModelShape,
}

/// Declare a static model catalog backed by shared native descriptors and collection methods.
///
/// This macro emits metadata, not a separate `PyO3` implementation for every field. Named types
/// are created by [`install_models`] once per module initialization.
#[macro_export]
macro_rules! python_data_views {
    ($visibility:vis $name:ident { $( $class:literal => $kind:ident $body:tt; )* }) => {
        /// Generated Python model names and archive slot bindings.
        $visibility const $name: &[$crate::ModelBinding] = &[
            $( $crate::ModelBinding { name: $class, shape: $crate::python_data_views!(@shape $kind $body) }, )*
        ];
    };
    (@shape dict { $( $field:literal : $slot:literal => $target:expr ),* $(,)? }) => {
        $crate::ModelShape::Dict(&[ $( $crate::FieldBinding { name: $field, slot: $slot, target: $target }, )* ])
    };
    (@shape list ($target:expr)) => { $crate::ModelShape::List($target) };
    (@shape indexed ($target:expr, [$($slot:literal),* $(,)?])) => { $crate::ModelShape::Indexed($target, &[$($slot),*]) };
    (@shape alias ($base:literal)) => { $crate::ModelShape::Alias($base) };
}

/// Install every generated class and connect cached native descriptors to their child types.
///
/// The caller must open and structurally check the archive using the matching model registry
/// before returning a root wrapper. The catalog does not replace schema validation.
pub fn install_models(
    module: &Bound<'_, PyModule>,
    models: &[ModelBinding],
    opaque: &Bound<'_, PyType>,
    undefined: &Bound<'_, PyAny>,
) -> PyResult<()> {
    let py = module.py();
    let type_factory = py.import("builtins")?.getattr("type")?;
    let module_name: String = module.name()?.extract()?;
    // Plain classes are allocated first, so forward references do not depend on schema order.
    for model in models {
        let base = match model.shape {
            ModelShape::Dict(_) => py.get_type::<NativeDict>(),
            ModelShape::List(_) => py.get_type::<NativeList>(),
            ModelShape::Indexed(_, _) => py.get_type::<NativeIndexedList>(),
            ModelShape::Alias(_) => continue,
        };
        let namespace = PyDict::new(py);
        namespace.set_item("__module__", &module_name)?;
        namespace.set_item("__slots__", ())?;
        let class = type_factory.call1((model.name, (base,), namespace))?;
        module.add(model.name, class)?;
    }
    // Contextual items inherit a complete dictionary model, never another contextual item.
    for model in models {
        if let ModelShape::Alias(base) = model.shape {
            let namespace = PyDict::new(py);
            namespace.set_item("__module__", &module_name)?;
            namespace.set_item("__slots__", ())?;
            let class = type_factory.call1((model.name, (module.getattr(base)?,), namespace))?;
            module.add(model.name, class)?;
        }
    }
    for model in models {
        let class = module.getattr(model.name)?;
        match model.shape {
            ModelShape::Dict(fields) => {
                for field in fields {
                    class.setattr(
                        field.name,
                        Py::new(
                            py,
                            NativeField::new(
                                field.slot,
                                ResolvedTarget::new(module, field.target, opaque, undefined)?,
                            ),
                        )?,
                    )?;
                }
            }
            ModelShape::List(target) | ModelShape::Indexed(target, _) => {
                let keys = if let ModelShape::Indexed(_, keys) = model.shape {
                    keys
                } else {
                    &[]
                };
                class.setattr(
                    "_binding",
                    Py::new(
                        py,
                        CollectionInfo::new(
                            ResolvedTarget::new(module, target, opaque, undefined)?,
                            keys.to_vec(),
                        ),
                    )?,
                )?;
            }
            ModelShape::Alias(_) => {}
        }
    }
    Ok(())
}
