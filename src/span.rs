use pyo3::basic::CompareOp;
use pyo3::exceptions::PyValueError;
use std::cmp::max;

use pyo3::prelude::*;
use pyo3::types::PyTuple;

fn merge_sub_spans(sub_spans: &mut Vec<(i32, i32)>) {
    sub_spans.sort_by_key(|&a| a.0);
    let mut index = 0;
    for i in 1..sub_spans.len() {
        if sub_spans[index].1 >= sub_spans[i].0 - 1 {
            sub_spans[index].1 = max(sub_spans[index].1, sub_spans[i].1);
        } else {
            index += 1;
            sub_spans[index] = sub_spans[i];
        }
    }
    sub_spans.truncate(index + 1);
}

/// A class used to represent spans.
#[pyclass]
pub(crate) struct Span {
    sub_spans: Vec<(i32, i32)>,
}

#[pymethods]
impl Span {
    #[new]
    fn py_new(sub_spans: Option<Vec<(i32, i32)>>) -> PyResult<Self> {
        match sub_spans {
            Some(mut f) => {
                for sub_span in &f {
                    if sub_span.0 > sub_span.1 {
                        return Err(PyValueError::new_err(
                            "Start point of sub-span cannot be greater than its end point",
                        ));
                    }
                }

                merge_sub_spans(&mut f);
                Ok(Span { sub_spans: f })
            }
            None => Ok(Span { sub_spans: vec![] }),
        }
    }
    /// Return a shallow copy of a Span.
    fn copy(&self) -> Self {
        self.clone()
    }
    /// Returns True if two Spans do not overlap.
    fn isdisjoint(&self, other: &Self) -> bool {
        let mut sub_spans = self.sub_spans.clone();
        sub_spans.extend(other.sub_spans.iter());
        sub_spans.sort_by_key(|&a| a.0);
        let mut index = 0;
        for i in 1..sub_spans.len() {
            if sub_spans[index].1 >= sub_spans[i].0 {
                return false;
            } else {
                index += 1;
                sub_spans[index] = sub_spans[i];
            }
        }
        true
    }
    /// Return True if other contains this Span, else False.
    fn issubset(&self, other: &Self) -> bool {
        let mut temp = other.clone();
        temp.sub_spans.extend(self.sub_spans.iter());
        merge_sub_spans(&mut temp.sub_spans);
        other.sub_spans == temp.sub_spans
    }
    /// Return True if this Span contains other, else False.
    fn issuperset(&self, other: &Self) -> bool {
        other.issubset(self)
    }
    #[args(others = "*")]
    fn union(&self, others: &PyTuple) -> PyResult<Span> {
        let mut output = self.clone();
        output.union_update(others)?;
        Ok(output)
    }
    #[args(others = "*")]
    fn union_update(&mut self, others: &PyTuple) -> PyResult<()> {
        let inputs: Vec<Span> = others.extract()?;
        self.sub_spans
            .extend(inputs.iter().flat_map(|f| &f.sub_spans));
        if !inputs.is_empty() {
            merge_sub_spans(&mut self.sub_spans);
        }
        Ok(())
    }
    fn __or__(&self, other: &Self) -> Span {
        let mut output = self.clone();
        output.__ior__(other);
        output
    }
    fn __ior__(&mut self, other: &Self) {
        self.sub_spans.extend(other.sub_spans.iter());
        merge_sub_spans(&mut self.sub_spans);
    }
    fn __contains__(&self, item: i32) -> bool {
        self.sub_spans.iter().any(|&f| f.0 <= item && item <= f.1)
    }
    fn __repr__(&self) -> String {
        format!(
            "Span([{}])",
            self.sub_spans
                .iter()
                .map(|&f| format!("({}, {})", f.0, f.1))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
    fn __str__(&self) -> String {
        format!(
            "({})",
            self.sub_spans
                .iter()
                .map(|&f| format!("[{}, {}]", f.0, f.1))
                .collect::<Vec<String>>()
                .join(" ∪ ")
        )
    }
    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Eq => self.sub_spans == other.sub_spans,
            CompareOp::Ne => self.sub_spans != other.sub_spans,
            CompareOp::Lt => self.issubset(other) && (self.sub_spans != other.sub_spans),
            CompareOp::Le => self.issubset(other),
            CompareOp::Gt => self.issuperset(other) && (self.sub_spans != other.sub_spans),
            CompareOp::Ge => self.issuperset(other),
        }
    }
    #[classattr]
    #[allow(non_upper_case_globals)]
    const __hash__: Option<PyObject> = None;
}

impl Clone for Span {
    fn clone(&self) -> Span {
        Span {
            sub_spans: self.sub_spans.clone(),
        }
    }
}
