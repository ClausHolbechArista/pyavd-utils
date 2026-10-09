// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Native property descriptors, scalar conversion, and archive-owner-preserving wrappers.

use pyo3::exceptions::{PyAttributeError, PyTypeError};
use pyo3::prelude::*;
use pyo3::types::{PyModule, PyString, PyType};
use pyo3::{PyTraverseError, PyVisit};
use validated_data::{DictHandle, ValueHandle};

use crate::Target;

/// Private owner-preserving constructor argument shared by native model wrappers.
#[pyclass(name = "_ValueHandle", frozen, skip_from_py_object)]
#[derive(Clone, Debug)]
pub struct PyValueHandle(pub ValueHandle);

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
    #[allow(
        clippy::needless_pass_by_value,
        reason = "PyO3 extracts constructor handles as PyRef values."
    )]
    fn new(handle: PyRef<'_, PyValueHandle>) -> PyResult<Self> {
        handle
            .0
            .as_dict()
            .map(Self)
            .ok_or_else(|| PyTypeError::new_err("value is not a dictionary"))
    }
}

/// Cached interpreter-local conversion objects. All references are visited by owning classes.
#[derive(Debug)]
pub(crate) struct ResolvedTarget {
    class: Option<Py<PyType>>,
    undefined: Py<PyAny>,
}

impl ResolvedTarget {
    pub(crate) fn undefined(&self, py: Python<'_>) -> Py<PyAny> {
        self.undefined.clone_ref(py)
    }
    pub(crate) fn new(
        module: &Bound<'_, PyModule>,
        target: Target,
        opaque: &Bound<'_, PyType>,
        undefined: &Bound<'_, PyAny>,
    ) -> PyResult<Self> {
        let class = match target {
            Target::Scalar => None,
            Target::Model(name) => Some(module.getattr(name)?.cast_into::<PyType>()?.unbind()),
            Target::Opaque => Some(opaque.clone().unbind()),
        };
        Ok(Self {
            class,
            undefined: undefined.clone().unbind(),
        })
    }

    pub(crate) fn convert(
        &self,
        py: Python<'_>,
        value: Option<ValueHandle>,
    ) -> PyResult<Py<PyAny>> {
        let Some(value) = value else {
            return Ok(self.undefined.clone_ref(py));
        };
        if value.is_null() {
            return Ok(py.None());
        }
        if let Some(class) = &self.class {
            return wrap_type(class.bind(py), value);
        }
        if let Some(scalar) = value.as_bool() {
            return Ok(scalar.into_pyobject(py)?.to_owned().unbind().into_any());
        }
        if let Some(scalar) = value.as_i64() {
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
    slot: u32,
    target: ResolvedTarget,
}

impl NativeField {
    pub(crate) fn new(slot: u32, target: ResolvedTarget) -> Self {
        Self { slot, target }
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
            .convert(slf.py(), instance.handle().field(slf.slot))
    }

    #[allow(
        clippy::unused_self,
        reason = "The Python descriptor protocol requires an instance receiver."
    )]
    fn __set__(&self, _instance: &Bound<'_, PyAny>, _value: &Bound<'_, PyAny>) -> PyResult<()> {
        Err(PyAttributeError::new_err(
            "validated-data fields are immutable",
        ))
    }

    #[allow(
        clippy::needless_pass_by_value,
        reason = "The PyO3 GC protocol accepts PyVisit by value."
    )]
    fn __traverse__(&self, visit: PyVisit<'_>) -> Result<(), PyTraverseError> {
        self.target.traverse(&visit)
    }
}

fn wrap_type(class: &Bound<'_, PyType>, value: ValueHandle) -> PyResult<Py<PyAny>> {
    let handle = Py::new(class.py(), PyValueHandle(value))?;
    Ok(class.call1((handle,))?.unbind())
}

/// Wrap a structurally checked root or child handle in a named native model class.
pub fn wrap_named(
    module: &Bound<'_, PyModule>,
    name: &str,
    value: ValueHandle,
) -> PyResult<Py<PyAny>> {
    wrap_type(&module.getattr(name)?.cast_into::<PyType>()?, value)
}
