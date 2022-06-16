use pyo3::exceptions::PyValueError;
use std::cmp::max;

use pyo3::prelude::*;
use pyo3::types::PyTuple;

fn merge_sub_spans(sub_spans: &mut Vec<(i32, i32)>) {
    sub_spans.sort_by_key(|&a| a.0);
    let mut index: usize = 0;
    for i in 1..sub_spans.len() {
        if sub_spans[index].1 >= sub_spans[i].0 {
            sub_spans[index].1 = max(sub_spans[index].1, sub_spans[i].1);
        } else {
            index += 1;
            sub_spans[index] = sub_spans[i];
        }
    }
    sub_spans.truncate(index + 1);
}

/// Returns a list of the UTF-8 indices of disjoint matches, from start to end.
#[pyfunction]
fn match_indices(string: &str, substring: &str) -> Vec<usize> {
    let mut byte_index: usize = 0;
    let mut len: usize = 0;
    string
        .match_indices(substring)
        .map(|f| {
            len += string[byte_index..f.0].chars().count();
            byte_index = f.0;
            len
        })
        .collect::<Vec<usize>>()
}

/// Returns a list of the UTF-16 indices of disjoint matches, from start to end.
#[pyfunction]
fn match_utf16_indices(string: &str, substring: &str) -> Vec<usize> {
    let mut byte_index: usize = 0;
    let mut len: usize = 0;
    string
        .match_indices(substring)
        .map(|f| {
            len += utf16len(&string[byte_index..f.0]);
            byte_index = f.0;
            len
        })
        .collect::<Vec<usize>>()
}

/// Returns a list of the byte indices of disjoint matches, from start to end.
#[pyfunction]
fn match_byte_indices(string: &str, substring: &str) -> Vec<usize> {
    string
        .match_indices(substring)
        .map(|f| f.0)
        .collect::<Vec<usize>>()
}

/// Returns a list of the UTF-8 indices of disjoint matches, from end to start.
#[pyfunction]
fn rmatch_indices(string: &str, substring: &str) -> Vec<usize> {
    let mut byte_index: usize = 0;
    let mut len: usize = 0;
    let mut output = string
        .rmatch_indices(substring)
        .collect::<Vec<(usize, &str)>>()
        .iter()
        .rev()
        .map(|f| {
            len += string[byte_index..f.0].chars().count();
            byte_index = f.0;
            len
        })
        .collect::<Vec<usize>>();
    output.reverse();
    output
}

/// Returns a list of the UTF-16 indices of disjoint matches, from end to start.
#[pyfunction]
fn rmatch_utf16_indices(string: &str, substring: &str) -> Vec<usize> {
    let mut byte_index: usize = 0;
    let mut len: usize = 0;
    let mut output = string
        .rmatch_indices(substring)
        .collect::<Vec<(usize, &str)>>()
        .iter()
        .rev()
        .map(|f| {
            len += utf16len(&string[byte_index..f.0]);
            byte_index = f.0;
            len
        })
        .collect::<Vec<usize>>();
    output.reverse();
    output
}

/// Returns a list of the byte indices of disjoint matches, from end to start.
#[pyfunction]
fn rmatch_byte_indices(string: &str, substring: &str) -> Vec<usize> {
    string
        .rmatch_indices(substring)
        .map(|f| f.0)
        .collect::<Vec<usize>>()
}

/// A function that returns the UTF-16 length of a string.
#[pyfunction]
fn utf16len(string: &str) -> usize {
    string.chars().map(|char| char.len_utf16()).sum()
}

/// A class used to represent spans.
#[pyclass]
struct Span {
    sub_spans: Vec<(i32, i32)>,
}

#[pymethods]
impl Span {
    #[new]
    fn py_new(sub_spans: Option<Vec<(i32, i32)>>) -> PyResult<Self> {
        match sub_spans {
            Some(mut f) => {
                if f.iter().any(|&subspan| subspan.0 > subspan.1) {
                    Err(PyValueError::new_err(
                        "Start point of sub-span cannot be greater than its end point",
                    ))
                } else {
                    merge_sub_spans(&mut f);
                    Ok(Span { sub_spans: f })
                }
            }
            None => Ok(Span { sub_spans: vec![] }),
        }
    }
    #[args(other = "*")]
    fn union(&self, other: &PyTuple) -> PyResult<Span> {
        let mut output = self.clone();
        output.union_update(other)?;
        Ok(output)
    }
    #[args(other = "*")]
    fn union_update(&mut self, other: &PyTuple) -> PyResult<()> {
        let inputs: Vec<Span> = other.extract()?;
        self.sub_spans
            .extend(inputs.iter().flat_map(|f| &f.sub_spans));
        if !inputs.is_empty() {
            merge_sub_spans(&mut self.sub_spans);
        }
        Ok(())
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
    fn __or__(&self, other: &Span) -> Span {
        let mut output = self.clone();
        output.__ior__(other);
        output
    }
    fn __ior__(&mut self, other: &Span) {
        self.sub_spans.append(&mut other.sub_spans.clone());
        merge_sub_spans(&mut self.sub_spans);
    }
}

impl Clone for Span {
    fn clone(&self) -> Span {
        Span {
            sub_spans: self.sub_spans.clone(),
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn miguel_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(match_indices, m)?)?;
    m.add_function(wrap_pyfunction!(match_utf16_indices, m)?)?;
    m.add_function(wrap_pyfunction!(match_byte_indices, m)?)?;
    m.add_function(wrap_pyfunction!(rmatch_indices, m)?)?;
    m.add_function(wrap_pyfunction!(rmatch_utf16_indices, m)?)?;
    m.add_function(wrap_pyfunction!(rmatch_byte_indices, m)?)?;
    m.add_function(wrap_pyfunction!(utf16len, m)?)?;
    m.add_class::<Span>()?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
