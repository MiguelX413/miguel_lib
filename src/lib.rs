mod interval;
mod span;

use crate::interval::Interval;
use crate::span::Span;
use pyo3::prelude::*;

/// Returns a list of the UTF-8 indices of disjoint matches, from start to end.
#[pyfunction]
fn match_indices(string: &str, substring: &str) -> Vec<usize> {
    let mut byte_index = 0;
    let mut len = 0;
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
    let mut byte_index = 0;
    let mut len = 0;
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
    let mut byte_index = 0;
    let mut len = 0;
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
    let mut byte_index = 0;
    let mut len = 0;
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
    m.add_class::<Interval>()?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
