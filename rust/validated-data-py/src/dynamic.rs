// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Grouped, typed access to concrete dynamic keys without relocating their stored payloads.
//!
//! A generated `_DynamicKeys` namespace has one descriptor per selector. Access resolves only
//! that group's concrete names. Entries and iterators keep the original owner; `.value` wraps
//! the original payload using its generated target, rather than copying a synthetic document.

use crate::descriptors::{NativeDict, ResolvedTarget, wrap_type};
use crate::{FieldBinding, ModelBinding, ViewHandle};
use pyo3::exceptions::PyKeyError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyModule, PyTuple, PyType};
use pyo3::{PyTraverseError, PyVisit};
use validated_data::FieldRelation;

fn concrete_keys(parent: &ViewHandle, field: &FieldBinding) -> Vec<String> {
    let mut keys = parent.dynamic_names(field);
    let mut seen = std::collections::HashSet::new();
    keys.retain(|key| {
        !parent.is_static_key(key)
            && parent.raw_key(key).is_some_and(|value| !value.is_null())
            && seen.insert(key.clone())
    });
    keys
}

/// Descriptor returning a grouped namespace over the containing dictionary.
#[pyclass(frozen)]
#[derive(Debug)]
pub(crate) struct DynamicNamespaceField(pub Py<PyType>);

#[pymethods]
impl DynamicNamespaceField {
    fn __get__(
        slf: PyRef<'_, Self>,
        instance: Option<&Bound<'_, PyAny>>,
        _owner: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Py<PyAny>> {
        let py = slf.py();
        let Some(instance) = instance.filter(|value| !value.is_none()) else {
            return Ok(slf.into_pyobject(py)?.unbind().into_any());
        };
        let instance: PyRef<'_, NativeDict> = instance.extract()?;
        wrap_type(slf.0.bind(slf.py()), instance.handle().0.clone())
    }
    #[allow(
        clippy::unused_self,
        reason = "The Python descriptor protocol requires an instance receiver."
    )]
    fn __set__(&self, _instance: &Bound<'_, PyAny>, _value: &Bound<'_, PyAny>) -> PyResult<()> {
        Err(pyo3::exceptions::PyAttributeError::new_err(
            "dynamic keys are immutable",
        ))
    }
    #[allow(
        clippy::needless_pass_by_value,
        reason = "The PyO3 GC protocol accepts PyVisit by value."
    )]
    fn __traverse__(&self, visit: PyVisit<'_>) -> Result<(), PyTraverseError> {
        visit.call(&self.0)
    }
}

/// Cached group conversion and selector metadata, shared by its entries and iterators.
#[pyclass(frozen)]
#[derive(Debug)]
pub(crate) struct DynamicGroupField {
    field: &'static FieldBinding,
    target: ResolvedTarget,
}

impl DynamicGroupField {
    pub(crate) fn new(field: &'static FieldBinding, target: ResolvedTarget) -> Self {
        Self { field, target }
    }
}

#[allow(
    clippy::multiple_inherent_impl,
    reason = "PyO3 methods need their own impl block apart from Rust-only construction helpers."
)]
#[pymethods]
impl DynamicGroupField {
    fn __get__(
        slf: PyRef<'_, Self>,
        instance: Option<&Bound<'_, PyAny>>,
        _owner: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Py<PyAny>> {
        let py = slf.py();
        let Some(instance) = instance.filter(|value| !value.is_none()) else {
            return Ok(slf.into_pyobject(py)?.unbind().into_any());
        };
        let parent: PyRef<'_, NativeDict> = instance.extract()?;
        let parent = parent.handle().0.clone();
        let keys = concrete_keys(&parent, slf.field);
        let group = slf.into_pyobject(py)?.unbind();
        Ok(Py::new(
            py,
            DynamicCollection {
                parent,
                keys,
                group,
            },
        )?
        .into_any())
    }
    #[allow(
        clippy::unused_self,
        reason = "The Python descriptor protocol requires an instance receiver."
    )]
    fn __set__(&self, _instance: &Bound<'_, PyAny>, _value: &Bound<'_, PyAny>) -> PyResult<()> {
        Err(pyo3::exceptions::PyAttributeError::new_err(
            "dynamic groups are immutable",
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

#[pyclass(frozen, name = "_DynamicEntry")]
#[derive(Debug)]
pub(crate) struct DynamicEntry {
    key: String,
    value: ViewHandle,
    group: Py<DynamicGroupField>,
}

#[pymethods]
impl DynamicEntry {
    #[getter]
    fn key(&self) -> &str {
        &self.key
    }
    #[getter]
    fn value(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        self.group
            .borrow(py)
            .target
            .convert(py, Some(self.value.clone()))
    }
    #[allow(
        clippy::needless_pass_by_value,
        reason = "The PyO3 GC protocol accepts PyVisit by value."
    )]
    fn __traverse__(&self, visit: PyVisit<'_>) -> Result<(), PyTraverseError> {
        visit.call(&self.group)
    }
}

#[pyclass(frozen, name = "_DynamicCollection")]
#[derive(Debug)]
pub(crate) struct DynamicCollection {
    parent: ViewHandle,
    keys: Vec<String>,
    group: Py<DynamicGroupField>,
}

impl DynamicCollection {
    fn entry(&self, py: Python<'_>, key: &str) -> PyResult<Option<Py<DynamicEntry>>> {
        if !self.keys.iter().any(|candidate| candidate == key) {
            return Ok(None);
        }
        let value = self
            .parent
            .raw_key(key)
            .ok_or_else(|| PyKeyError::new_err(key.to_owned()))?;
        Py::new(
            py,
            DynamicEntry {
                key: key.to_owned(),
                value,
                group: self.group.clone_ref(py),
            },
        )
        .map(Some)
    }
    fn iterator(slf: PyRef<'_, Self>, kind: Iteration) -> PyResult<DynamicIterator> {
        let py = slf.py();
        Ok(DynamicIterator {
            collection: slf.into_pyobject(py)?.unbind(),
            index: 0,
            kind,
        })
    }
}

#[allow(
    clippy::multiple_inherent_impl,
    reason = "PyO3 methods need their own impl block apart from Rust-only collection helpers."
)]
#[pymethods]
impl DynamicCollection {
    fn __len__(&self) -> usize {
        self.keys.len()
    }
    fn __contains__(&self, key: &str) -> bool {
        self.keys.iter().any(|candidate| candidate == key)
    }
    fn __getitem__(&self, py: Python<'_>, key: &str) -> PyResult<Py<DynamicEntry>> {
        self.entry(py, key)?
            .ok_or_else(|| PyKeyError::new_err(key.to_owned()))
    }
    #[pyo3(signature = (key, *args, **kwargs))]
    fn get(
        &self,
        py: Python<'_>,
        key: &str,
        args: &Bound<'_, PyTuple>,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Py<PyAny>> {
        let default = crate::collections::get_default(args, kwargs)?;
        Ok(self.entry(py, key)?.map_or_else(
            || default.unwrap_or_else(|| self.group.borrow(py).target.undefined(py)),
            Py::into_any,
        ))
    }
    fn __iter__(slf: PyRef<'_, Self>) -> PyResult<DynamicIterator> {
        Self::iterator(slf, Iteration::Values)
    }
    fn values(slf: PyRef<'_, Self>) -> PyResult<DynamicIterator> {
        Self::iterator(slf, Iteration::Values)
    }
    fn keys(slf: PyRef<'_, Self>) -> PyResult<DynamicIterator> {
        Self::iterator(slf, Iteration::Keys)
    }
    fn items(slf: PyRef<'_, Self>) -> PyResult<DynamicIterator> {
        Self::iterator(slf, Iteration::Items)
    }
    #[allow(
        clippy::needless_pass_by_value,
        reason = "The PyO3 GC protocol accepts PyVisit by value."
    )]
    fn __traverse__(&self, visit: PyVisit<'_>) -> Result<(), PyTraverseError> {
        visit.call(&self.group)
    }
}

#[derive(Clone, Copy, Debug)]
enum Iteration {
    Keys,
    Values,
    Items,
}

#[pyclass]
#[derive(Debug)]
pub(crate) struct DynamicIterator {
    collection: Py<DynamicCollection>,
    index: usize,
    kind: Iteration,
}

#[pymethods]
impl DynamicIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(&mut self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        let collection = self.collection.borrow(py);
        let Some(key) = collection.keys.get(self.index) else {
            return Ok(None);
        };
        self.index += 1;
        if matches!(self.kind, Iteration::Keys) {
            return Ok(Some(key.into_pyobject(py)?.unbind().into_any()));
        }
        let entry = collection.__getitem__(py, key)?.into_any();
        if matches!(self.kind, Iteration::Values) {
            return Ok(Some(entry));
        }
        Ok(Some(
            PyTuple::new(py, [key.into_pyobject(py)?.unbind().into_any(), entry])?
                .unbind()
                .into_any(),
        ))
    }
    #[allow(
        clippy::needless_pass_by_value,
        reason = "The PyO3 GC protocol accepts PyVisit by value."
    )]
    fn __traverse__(&self, visit: PyVisit<'_>) -> Result<(), PyTraverseError> {
        visit.call(&self.collection)
    }
}

/// Install descriptors on an already-attached namespace and its containing model.
pub(crate) fn install_namespace(
    module: &Bound<'_, PyModule>,
    model: &'static ModelBinding,
    class: &Bound<'_, PyType>,
    namespace: &Bound<'_, PyType>,
    opaque: &Bound<'_, PyType>,
    undefined: &Bound<'_, PyAny>,
) -> PyResult<()> {
    let py = module.py();
    namespace.setattr(
        "_model_binding",
        Py::new(py, crate::descriptors::ModelInfo(model))?,
    )?;
    let mut names = std::collections::HashSet::new();
    for field in model.fields {
        let FieldRelation::DynamicKey(_) = field.descriptor.relation else {
            continue;
        };
        let name = field.name;
        if !names.insert(name) || namespace.hasattr(name)? {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "conflicting dynamic group {name}"
            )));
        }
        namespace.setattr(
            name,
            Py::new(
                py,
                DynamicGroupField::new(
                    field,
                    ResolvedTarget::new(module, field.target, None, opaque, undefined)?,
                ),
            )?,
        )?;
    }
    class.setattr(
        "_dynamic_keys",
        Py::new(py, DynamicNamespaceField(namespace.clone().unbind()))?,
    )?;
    Ok(())
}
