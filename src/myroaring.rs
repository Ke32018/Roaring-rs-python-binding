use std::fmt;

use pyo3::{pyclass, pymethods};
use roaring::RoaringBitmap;


#[pyclass(name = "RoaringBitmap")]
pub struct PyRoaringBitmap {
    inner: RoaringBitmap,
}

#[pymethods]
impl PyRoaringBitmap {
    #[new]
    pub fn new() -> Self {
        let map = RoaringBitmap::new();
        Self {inner: map}
    }

    /// Adds a value to the set.
    /// Returns whether the value was absent from the set.
    pub fn insert(&mut self, v: u32) -> bool {
        self.inner.insert(v)  
    }

    /// Removes a value from the set. Returns true if the value was present in the set.
    pub fn remove(&mut self, v: u32) -> bool {
        self.inner.remove(v)
    }

    /// Returns true if the set has no elements in common with other. This is equivalent to checking for an empty intersection.
    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.inner.is_disjoint(&other.inner)
    }

    /// Returns true if this set is a subset of other.
    pub fn is_subset(&self, other: &Self) -> bool {
        self.inner.is_subset(&other.inner)
    }

    /// Returns true if this set is a superset of other.
    pub fn is_superset(&self, other: &Self) -> bool {
        self.inner.is_superset(&other.inner)
    }

    pub fn __repr__(&self) -> String {
        format!("{}", self)
    }
}

impl fmt::Display for PyRoaringBitmap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}