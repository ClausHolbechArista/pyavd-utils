// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Shared native positional, unique-key, and iterator implementations.
//!
//! Duplicate-key lists use only the positional implementation. Unique-key lists override indexing
//! with key lookup and add mapping-style helpers. Each iterator owns the archive and cached item
//! type independently of its source Python object.

use pyo3::exceptions::{PyIndexError, PyKeyError, PyTypeError};
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyInt, PyList, PySlice, PyString, PyTuple, PyType};
use pyo3::{PyTraverseError, PyVisit};
use validated_data::{ListHandle, PrimaryKeyValue};

use crate::descriptors::{PyValueHandle, ResolvedTarget};

#[pyclass(frozen)]
#[derive(Debug)]
pub(crate) struct CollectionInfo {
    target: ResolvedTarget,
    keys: Vec<u32>,
}

impl CollectionInfo {
    pub(crate) fn new(target: ResolvedTarget, keys: Vec<u32>) -> Self {
        Self { target, keys }
    }
}

#[pymethods]
#[allow(
    clippy::multiple_inherent_impl,
    reason = "PyO3 separates native methods from Rust helpers."
)]
impl CollectionInfo {
    #[allow(
        clippy::needless_pass_by_value,
        reason = "The PyO3 GC protocol accepts PyVisit by value."
    )]
    fn __traverse__(&self, visit: PyVisit<'_>) -> Result<(), PyTraverseError> {
        self.target.traverse(&visit)
    }
}

#[pyclass(subclass, frozen, skip_from_py_object)]
#[derive(Debug)]
pub(crate) struct NativeList {
    handle: ListHandle,
    info: Py<CollectionInfo>,
}

impl NativeList {
    fn construct(class: &Bound<'_, PyType>, handle: &PyValueHandle) -> PyResult<Self> {
        let handle = handle
            .0
            .as_list()
            .ok_or_else(|| PyTypeError::new_err("value is not a list"))?;
        let info = class.getattr("_binding")?.extract::<Py<CollectionInfo>>()?;
        Ok(Self { handle, info })
    }

    fn at(&self, py: Python<'_>, index: usize) -> PyResult<Py<PyAny>> {
        self.info
            .borrow(py)
            .target
            .convert(py, self.handle.get(index))
    }

    fn iterator(&self, py: Python<'_>, kind: Iteration) -> NativeIterator {
        NativeIterator {
            handle: self.handle.clone(),
            info: self.info.clone_ref(py),
            index: 0,
            kind,
        }
    }

    fn lookup(&self, py: Python<'_>, key: &Bound<'_, PyAny>) -> PyResult<Option<Py<PyAny>>> {
        let key = OwnedKey::extract(key)?;
        self.handle
            .get_by_primary_key(&[key.borrow()])
            .map(|value| self.info.borrow(py).target.convert(py, Some(value)))
            .transpose()
    }
}

#[pymethods]
#[allow(
    clippy::multiple_inherent_impl,
    reason = "PyO3 separates native methods from Rust helpers."
)]
impl NativeList {
    #[new]
    #[classmethod]
    #[allow(
        clippy::needless_pass_by_value,
        reason = "PyO3 extracts constructor handles as PyRef values."
    )]
    fn new(class: &Bound<'_, PyType>, handle: PyRef<'_, PyValueHandle>) -> PyResult<Self> {
        Self::construct(class, &handle)
    }

    fn __len__(&self) -> usize {
        self.handle.len()
    }

    fn __iter__(&self, py: Python<'_>) -> NativeIterator {
        self.iterator(py, Iteration::Values)
    }

    fn __getitem__(&self, py: Python<'_>, index: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
        let len = isize::try_from(self.handle.len())
            .map_err(|error| PyIndexError::new_err(error.to_string()))?;
        if let Ok(slice) = index.cast::<PySlice>() {
            let indices = slice.indices(len)?;
            let mut position = indices.start;
            let result = PyList::empty(py);
            for _ in 0..indices.slicelength {
                let offset = usize::try_from(position)
                    .map_err(|error| PyIndexError::new_err(error.to_string()))?;
                result.append(self.at(py, offset)?)?;
                position += indices.step;
            }
            return Ok(result.unbind().into_any());
        }
        let index: isize = index.extract()?;
        let normalized = if index < 0 { len + index } else { index };
        if !(0..len).contains(&normalized) {
            return Err(PyIndexError::new_err("list index out of range"));
        }
        self.at(
            py,
            usize::try_from(normalized)
                .map_err(|error| PyIndexError::new_err(error.to_string()))?,
        )
    }

    #[allow(
        clippy::needless_pass_by_value,
        reason = "The PyO3 GC protocol accepts PyVisit by value."
    )]
    fn __traverse__(&self, visit: PyVisit<'_>) -> Result<(), PyTraverseError> {
        visit.call(&self.info)
    }
}

#[pyclass(extends=NativeList, subclass, frozen, skip_from_py_object)]
#[derive(Debug)]
pub(crate) struct NativeIndexedList;

#[pymethods]
#[allow(
    clippy::multiple_inherent_impl,
    reason = "PyO3 separates native methods from Rust helpers."
)]
impl NativeIndexedList {
    #[new]
    #[classmethod]
    #[allow(
        clippy::needless_pass_by_value,
        reason = "PyO3 extracts constructor handles as PyRef values."
    )]
    fn new(
        class: &Bound<'_, PyType>,
        handle: PyRef<'_, PyValueHandle>,
    ) -> PyResult<PyClassInitializer<Self>> {
        Ok(PyClassInitializer::from(NativeList::construct(class, &handle)?).add_subclass(Self))
    }

    fn __getitem__(slf: PyRef<'_, Self>, key: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
        let py = slf.py();
        slf.into_super()
            .lookup(py, key)?
            .ok_or_else(|| PyKeyError::new_err(key.clone().unbind()))
    }

    fn __contains__(slf: PyRef<'_, Self>, key: &Bound<'_, PyAny>) -> PyResult<bool> {
        let key = OwnedKey::extract(key)?;
        Ok(slf
            .into_super()
            .handle
            .get_by_primary_key(&[key.borrow()])
            .is_some())
    }

    #[pyo3(signature = (key, *args, **kwargs))]
    fn get(
        slf: PyRef<'_, Self>,
        key: &Bound<'_, PyAny>,
        args: &Bound<'_, PyTuple>,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Py<PyAny>> {
        let py = slf.py();
        let base = slf.into_super();
        if args.len() > 1 {
            return Err(PyTypeError::new_err("get accepts at most one default"));
        }
        let keyword = if let Some(kwargs) = kwargs {
            let default = kwargs.get_item("default")?;
            if kwargs.len() != usize::from(default.is_some()) {
                return Err(PyTypeError::new_err("unexpected get keyword argument"));
            }
            default
        } else {
            None
        };
        if !args.is_empty() && keyword.is_some() {
            return Err(PyTypeError::new_err("multiple values for default"));
        }
        let default = if args.is_empty() {
            keyword.map(Bound::unbind)
        } else {
            Some(args.get_item(0)?.unbind())
        };
        Ok(base.lookup(py, key)?.unwrap_or_else(|| {
            default.unwrap_or_else(|| base.info.borrow(py).target.undefined(py))
        }))
    }

    fn keys(slf: PyRef<'_, Self>) -> NativeIterator {
        let py = slf.py();
        slf.into_super().iterator(py, Iteration::Keys)
    }

    fn values(slf: PyRef<'_, Self>) -> NativeIterator {
        let py = slf.py();
        slf.into_super().iterator(py, Iteration::Values)
    }

    fn items(slf: PyRef<'_, Self>) -> NativeIterator {
        let py = slf.py();
        slf.into_super().iterator(py, Iteration::Items)
    }
}

#[derive(Clone, Copy, Debug)]
enum Iteration {
    Values,
    Keys,
    Items,
}

#[pyclass]
#[derive(Debug)]
pub(crate) struct NativeIterator {
    handle: ListHandle,
    info: Py<CollectionInfo>,
    index: usize,
    kind: Iteration,
}

#[pymethods]
impl NativeIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&mut self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        let Some(item_value) = self.handle.get(self.index) else {
            return Ok(None);
        };
        self.index += 1;
        let info = self.info.borrow(py);
        if matches!(self.kind, Iteration::Values) {
            return info.target.convert(py, Some(item_value)).map(Some);
        }
        let dict = item_value
            .as_dict()
            .ok_or_else(|| PyTypeError::new_err("indexed item is not a dictionary"))?;
        let key_slot = info
            .keys
            .first()
            .ok_or_else(|| PyTypeError::new_err("indexed list has no primary key"))?;
        let key_value = dict
            .field(*key_slot)
            .ok_or_else(|| PyTypeError::new_err("primary key is absent"))?;
        let key = if let Some(string_key) = key_value.as_str() {
            PyString::new(py, string_key).unbind().into_any()
        } else if let Some(bool_key) = key_value.as_bool() {
            bool_key.into_pyobject(py)?.to_owned().unbind().into_any()
        } else if let Some(int_key) = key_value.as_i64() {
            int_key.into_pyobject(py)?.unbind().into_any()
        } else {
            return Err(PyTypeError::new_err("invalid primary key scalar"));
        };
        if matches!(self.kind, Iteration::Keys) {
            return Ok(Some(key));
        }
        let item = info.target.convert(py, Some(item_value))?;
        Ok(Some(PyTuple::new(py, [key, item])?.unbind().into_any()))
    }

    #[allow(
        clippy::needless_pass_by_value,
        reason = "The PyO3 GC protocol accepts PyVisit by value."
    )]
    fn __traverse__(&self, visit: PyVisit<'_>) -> Result<(), PyTraverseError> {
        visit.call(&self.info)
    }
}

#[derive(Debug)]
enum OwnedKey {
    Bool(bool),
    Int(i64),
    Str(String),
}

impl OwnedKey {
    fn extract(value: &Bound<'_, PyAny>) -> PyResult<Self> {
        if value.is_instance_of::<PyBool>() {
            return value.extract().map(Self::Bool);
        }
        if value.is_instance_of::<PyInt>() {
            return value.extract().map(Self::Int);
        }
        if value.is_instance_of::<PyString>() {
            return value.extract().map(Self::Str);
        }
        Err(PyTypeError::new_err(
            "primary-key components must be bool, int, or str",
        ))
    }

    fn borrow(&self) -> PrimaryKeyValue<'_> {
        match self {
            Self::Bool(value) => PrimaryKeyValue::Bool(*value),
            Self::Int(value) => PrimaryKeyValue::Int(*value),
            Self::Str(value) => PrimaryKeyValue::Str(value),
        }
    }
}
