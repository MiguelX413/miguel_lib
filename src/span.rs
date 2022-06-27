use pyo3::basic::CompareOp;
use pyo3::exceptions::PyValueError;
use std::cmp::{max, min};

use pyo3::prelude::*;
use pyo3::types::PyTuple;

fn merge_segments(segments: &mut Vec<(i32, i32)>) {
    segments.sort_by_key(|&a| a.0);
    let mut index = 0;
    for i in 1..segments.len() {
        if segments[index].1 >= segments[i].0 - 1 {
            segments[index].1 = max(segments[index].1, segments[i].1);
        } else {
            index += 1;
            segments[index] = segments[i];
        }
    }
    segments.truncate(index + 1);
}

/// A class used to represent spans.
#[pyclass]
pub(crate) struct Span {
    #[pyo3(get)]
    segments: Vec<(i32, i32)>,
}

#[pymethods]
impl Span {
    #[new]
    fn py_new(segments: Option<Vec<(i32, i32)>>) -> PyResult<Self> {
        match segments {
            Some(mut f) => {
                for segment in &f {
                    if segment.0 > segment.1 {
                        return Err(PyValueError::new_err(
                            "Start point of segment cannot be greater than its end point",
                        ));
                    }
                }

                merge_segments(&mut f);
                Ok(Self { segments: f })
            }
            None => Ok(Self { segments: vec![] }),
        }
    }
    /// Return a shallow copy of a Span.
    fn copy(&self) -> Self {
        self.clone()
    }
    #[args(others = "*")]
    fn intersection(&self, others: &PyTuple) -> PyResult<Self> {
        let mut output = self.clone();
        output.intersection_update(others)?;
        Ok(output)
    }
    #[args(others = "*")]
    fn intersection_update(&mut self, others: &PyTuple) -> PyResult<()> {
        others
            .extract::<Vec<Self>>()?
            .iter()
            .for_each(|input| self.__iand__(input));
        Ok(())
    }
    /// Returns True if two Spans do not overlap.
    fn isdisjoint(&self, other: &Self) -> bool {
        let mut segments = self.segments.clone();
        segments.extend(other.segments.iter());
        segments.sort_by_key(|&a| a.0);
        let mut index = 0;
        for i in 1..segments.len() {
            if segments[index].1 >= segments[i].0 {
                return false;
            } else {
                index += 1;
                segments[index] = segments[i];
            }
        }
        true
    }
    /// Return True if other contains this Span, else False.
    fn issubset(&self, other: &Self) -> bool {
        other.segments == other.__or__(self).segments
    }
    /// Return True if this Span contains other, else False.
    fn issuperset(&self, other: &Self) -> bool {
        other.issubset(self)
    }
    #[args(others = "*")]
    fn union(&self, others: &PyTuple) -> PyResult<Self> {
        let mut output = self.clone();
        output.union_update(others)?;
        Ok(output)
    }
    #[args(others = "*")]
    fn union_update(&mut self, others: &PyTuple) -> PyResult<()> {
        let inputs = others.extract::<Vec<Self>>()?;
        self.segments
            .extend(inputs.iter().flat_map(|f| &f.segments));
        if !inputs.is_empty() {
            merge_segments(&mut self.segments);
        }
        Ok(())
    }
    fn __or__(&self, other: &Self) -> Self {
        let mut output = self.clone();
        output.__ior__(other);
        output
    }
    fn __ior__(&mut self, other: &Self) {
        self.segments.extend(other.segments.iter());
        merge_segments(&mut self.segments);
    }
    fn __and__(&self, other: &Self) -> Self {
        let mut output = Self { segments: vec![] };
        let mut next_bound = 0;
        let mut bottom_bound;
        for &x in &self.segments {
            bottom_bound = next_bound;
            for y in bottom_bound..other.segments.len() {
                if x.1 < other.segments[y].0 {
                    break;
                } else {
                    let left = max(x.0, other.segments[y].0);
                    let right = min(x.1, other.segments[y].1);
                    if left <= right {
                        output.segments.push((left, right));
                    }
                    next_bound = y;
                }
            }
        }
        output
    }
    fn __iand__(&mut self, other: &Self) {
        self.segments = self.__and__(other).segments;
    }
    fn __contains__(&self, item: i32) -> bool {
        self.segments.iter().any(|&f| f.0 <= item && item <= f.1)
    }
    fn __repr__(&self) -> String {
        format!(
            "Span([{}])",
            self.segments
                .iter()
                .map(|&f| format!("({}, {})", f.0, f.1))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
    fn __str__(&self) -> String {
        if !self.segments.is_empty() {
            self.segments
                .iter()
                .map(|&f| format!("[{}, {}]", f.0, f.1))
                .collect::<Vec<String>>()
                .join(" ∪ ")
        } else {
            "∅".to_string()
        }
    }
    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Eq => self.segments == other.segments,
            CompareOp::Ne => self.segments != other.segments,
            CompareOp::Lt => self.issubset(other) && (self.segments != other.segments),
            CompareOp::Le => self.issubset(other),
            CompareOp::Gt => self.issuperset(other) && (self.segments != other.segments),
            CompareOp::Ge => self.issuperset(other),
        }
    }
    #[classattr]
    #[allow(non_upper_case_globals)]
    const __hash__: Option<PyObject> = None;
}

impl Clone for Span {
    fn clone(&self) -> Self {
        Self {
            segments: self.segments.clone(),
        }
    }
}
