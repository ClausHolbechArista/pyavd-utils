// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Native property descriptors, scalar conversion, and archive-owner-preserving wrappers.

use crate::values::{DictHandle, ViewHandle as ValueHandle};
use pyo3::basic::CompareOp;
use pyo3::exceptions::{PyAttributeError, PyTypeError};
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyInt, PyModule, PyString, PyType};
use pyo3::{PyTraverseError, PyVisit};
use std::sync::Arc;

use crate::Target;

/// Private owner-preserving constructor argument shared by native model wrappers.
#[pyclass(name = "_ValueHandle", frozen, skip_from_py_object)]
#[derive(Clone, Debug)]
pub struct PyValueHandle(pub ValueHandle);

/// Interpreter-local attachment of generated dictionary metadata to its Python class.
#[pyclass(frozen)]
#[derive(Debug)]
pub(crate) struct ModelInfo(pub &'static crate::ModelBinding);

/// Shared dictionary backing for all generated native Python model types.
#[pyclass(subclass, frozen, skip_from_py_object)]
#[derive(Debug)]
pub(crate) struct NativeDict(DictHandle);

impl NativeDict {
    pub(crate) fn handle(&self) -> &DictHandle {
        &self.0
    }
}

#[pymethods]
#[allow(
    clippy::multiple_inherent_impl,
    reason = "PyO3 separates native methods from Rust helpers."
)]
impl NativeDict {
    #[new]
    #[classmethod]
    #[allow(
        clippy::needless_pass_by_value,
        reason = "PyO3 extracts constructor handles as PyRef values."
    )]
    fn new(class: &Bound<'_, PyType>, handle: PyRef<'_, PyValueHandle>) -> PyResult<Self> {
        let binding = class
            .getattr("_model_binding")?
            .extract::<PyRef<'_, ModelInfo>>()?
            .0;
        handle
            .0
            .clone()
            .with_model(binding)
            .as_dict()
            .map(Self)
            .ok_or_else(|| PyTypeError::new_err("value is not a dictionary"))
    }

    fn __bool__(&self) -> bool {
        !self.0.is_empty()
    }

    /// Whether the backing data supplies a non-null field, independently of accessor defaults.
    fn _is_defined(&self, name: &str) -> PyResult<bool> {
        let field = self
            .0
            .0
            .field_binding(name)
            .ok_or_else(|| PyAttributeError::new_err(name.to_owned()))?;
        Ok(self.0.0.is_defined(field))
    }

    /// Whether the field uses a schema default or an implicit empty collection fallback.
    fn _is_default(&self, name: &str) -> PyResult<bool> {
        let field = self
            .0
            .0
            .field_binding(name)
            .ok_or_else(|| PyAttributeError::new_err(name.to_owned()))?;
        Ok(self.0.0.is_default(field))
    }

    fn __richcmp__(
        slf: PyRef<'_, Self>,
        other: &Bound<'_, PyAny>,
        op: CompareOp,
    ) -> PyResult<Py<PyAny>> {
        let py = slf.py();
        if !matches!(op, CompareOp::Eq | CompareOp::Ne) {
            return Ok(py.NotImplemented());
        }
        let handle = slf.0.clone();
        let instance = slf.into_pyobject(py)?;
        if !instance.get_type().is(other.get_type()) {
            return Ok(py.NotImplemented());
        }
        let other: PyRef<'_, Self> = other.extract()?;
        equality_result(py, op, handle.content_eq(&other.0))
    }
}

/// Convert a same-model structural comparison into Python's equality result.
pub(crate) fn equality_result(py: Python<'_>, op: CompareOp, equal: bool) -> PyResult<Py<PyAny>> {
    let result = match op {
        CompareOp::Eq => equal,
        CompareOp::Ne => !equal,
        _ => return Ok(py.NotImplemented()),
    };
    Ok(result.into_pyobject(py)?.to_owned().unbind().into_any())
}

/// Cached interpreter-local conversion objects. All references are visited by owning classes.
#[derive(Debug)]
pub(crate) struct ResolvedTarget {
    class: Option<Py<PyType>>,
    undefined: Py<PyAny>,
    model: Option<&'static crate::ModelBinding>,
}

impl ResolvedTarget {
    /// Compare a sequence item without allocating a wrapper for another native dictionary or
    /// a built-in scalar. Custom Python comparators still use the normal rich-comparison protocol.
    pub(crate) fn matches(
        &self,
        py: Python<'_>,
        value: ValueHandle,
        other: &Bound<'_, PyAny>,
    ) -> PyResult<bool> {
        if let Some(class) = &self.class {
            if other.get_type().is(class.bind(py))
                && let Ok(other) = other.extract::<PyRef<'_, NativeDict>>()
                && let Some(value) = value.as_dict()
            {
                return Ok(value.content_eq(other.handle()));
            }
        } else {
            if other.get_type().is(py.get_type::<PyString>())
                && let Some(value) = value.as_str()
            {
                // Python permits unpaired surrogates, whereas the archive stores UTF-8.
                // Fall back to Python comparison if the query has no UTF-8 representation.
                if let Ok(other) = other.cast::<PyString>()?.to_str() {
                    return Ok(value == other);
                }
            }
            if (other.get_type().is(py.get_type::<PyInt>())
                || other.get_type().is(py.get_type::<PyBool>()))
                && let Some(value) = value.as_i64().or_else(|| value.as_bool().map(i64::from))
            {
                return Ok(other.extract::<i64>().ok() == Some(value));
            }
            if value.is_null() && other.is_none() {
                return Ok(true);
            }
        }
        self.convert(py, Some(value))?.bind(py).eq(other)
    }

    pub(crate) fn undefined(&self, py: Python<'_>) -> Py<PyAny> {
        self.undefined.clone_ref(py)
    }
    pub(crate) fn new(
        module: &Bound<'_, PyModule>,
        target: Target,
        contextual_name: Option<&str>,
        opaque: &Bound<'_, PyType>,
        undefined: &Bound<'_, PyAny>,
    ) -> PyResult<Self> {
        let class = match target {
            Target::Scalar => None,
            Target::Model(model) => Some(
                crate::catalog::resolve_python_type(
                    module,
                    contextual_name.unwrap_or(model().name),
                )?
                .unbind(),
            ),
            Target::Opaque => Some(opaque.clone().unbind()),
        };
        Ok(Self {
            class,
            undefined: undefined.clone().unbind(),
            model: match target {
                Target::Model(model) => Some(model()),
                _ => None,
            },
        })
    }

    pub(crate) fn convert(
        &self,
        py: Python<'_>,
        value: Option<ValueHandle>,
    ) -> PyResult<Py<PyAny>> {
        let Some(value) = value else {
            return Ok(py.None());
        };
        if value.is_null() {
            return Ok(py.None());
        }
        let value = match self.model {
            Some(model) => value.with_model(model),
            None => value,
        };
        if let Some(class) = &self.class {
            return wrap_type(class.bind(py), value);
        }
        if let Some(scalar) = value.as_bool() {
            return Ok(scalar.into_pyobject(py)?.to_owned().unbind().into_any());
        }
        if let Some(scalar) = value.as_i64() {
            return Ok(scalar.into_pyobject(py)?.unbind().into_any());
        }
        if let Some(scalar) = value.as_u64() {
            return Ok(scalar.into_pyobject(py)?.unbind().into_any());
        }
        if let Some(scalar) = value.as_str() {
            return Ok(PyString::new(py, scalar).unbind().into_any());
        }
        Err(PyTypeError::new_err(
            "collection has no generated Python target",
        ))
    }

    pub(crate) fn traverse(&self, visit: &PyVisit<'_>) -> Result<(), PyTraverseError> {
        if let Some(class) = &self.class {
            visit.call(class)?;
        }
        visit.call(&self.undefined)
    }
}

/// Native read-only descriptor for a statically slotted dictionary field.
#[pyclass(frozen)]
#[derive(Debug)]
pub(crate) struct NativeField {
    binding: &'static crate::FieldBinding,
    target: ResolvedTarget,
}

impl NativeField {
    pub(crate) fn new(binding: &'static crate::FieldBinding, target: ResolvedTarget) -> Self {
        Self { binding, target }
    }
}

#[pymethods]
#[allow(
    clippy::multiple_inherent_impl,
    reason = "PyO3 separates native methods from Rust helpers."
)]
impl NativeField {
    fn __get__(
        slf: PyRef<'_, Self>,
        instance: Option<&Bound<'_, PyAny>>,
        _owner: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Py<PyAny>> {
        let Some(instance) = instance.filter(|value| !value.is_none()) else {
            let py = slf.py();
            return Ok(slf.into_pyobject(py)?.unbind().into_any());
        };
        let instance: PyRef<'_, NativeDict> = instance.extract()?;
        slf.target
            .convert(slf.py(), instance.handle().0.field(slf.binding))
    }

    #[allow(
        clippy::unused_self,
        reason = "The Python descriptor protocol requires an instance receiver."
    )]
    fn __set__(&self, _instance: &Bound<'_, PyAny>, _value: &Bound<'_, PyAny>) -> PyResult<()> {
        Err(PyAttributeError::new_err("data-view fields are immutable"))
    }

    #[allow(
        clippy::needless_pass_by_value,
        reason = "The PyO3 GC protocol accepts PyVisit by value."
    )]
    fn __traverse__(&self, visit: PyVisit<'_>) -> Result<(), PyTraverseError> {
        self.target.traverse(&visit)
    }
}

pub(crate) fn wrap_type(class: &Bound<'_, PyType>, value: ValueHandle) -> PyResult<Py<PyAny>> {
    let handle = Py::new(class.py(), PyValueHandle(value))?;
    Ok(class.call1((handle,))?.unbind())
}

/// Wrap a structurally checked root or child handle in a qualified native model class.
pub fn wrap_named(
    module: &Bound<'_, PyModule>,
    name: &str,
    value: data_views::ValueHandle,
) -> PyResult<Py<PyAny>> {
    wrap_type(
        &crate::catalog::resolve_python_type(module, name)?,
        value.into(),
    )
}

/// Wrap checked host data with borrowed schema-default access.
///
/// The caller must open the data using this schema's fingerprint and the generated registry,
/// verify that the schema matches the generated code's schema fingerprint, and structurally
/// check its root before calling. Descendants retain both archive owners.
pub fn wrap_with_schema(
    module: &Bound<'_, PyModule>,
    name: &str,
    value: data_views::ValueHandle,
    schemas: Arc<avdschema::Store>,
) -> PyResult<Py<PyAny>> {
    wrap_type(
        &crate::catalog::resolve_python_type(module, name)?,
        ValueHandle::from(value).with_schemas(schemas),
    )
}
