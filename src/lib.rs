use pyo3::exceptions::{PyStopIteration, PyValueError};

use pyo3::prelude::*;

#[pyclass]
pub struct ChunksIter {
    chunk_size: usize,
    iter: Py<PyAny>,
    complete: bool,
}

#[pymethods]
impl ChunksIter {
    #[new]
    fn py_new(py: Python, iter: Py<PyAny>, chunk_size: usize) -> PyResult<Self> {
        if chunk_size < 1 {
            return Err(PyValueError::new_err("chunk_size cannot be 0 or lower"));
        }
        Ok(Self {
            chunk_size,
            iter: iter.call_method0(py, "__iter__")?,
            complete: false,
        })
    }
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(mut slf: PyRefMut<'_, Self>, py: Python) -> PyResult<Option<Vec<PyObject>>> {
        if slf.complete {
            return Ok(None);
        }

        let output = (0..slf.chunk_size)
            .into_iter()
            .filter_map(|_| {
                if slf.complete {
                    return None;
                }
                match slf.iter.call_method0(py, "__next__") {
                    Ok(ok) => {
                        if ok.is_none(py) {
                            slf.complete = true;
                            return None;
                        }
                        Some(Ok(ok))
                    }
                    Err(err) => {
                        if err.is_instance_of::<PyStopIteration>(py) {
                            slf.complete = true;
                            return None;
                        }
                        Some(Err(err))
                    }
                }
            })
            .collect::<PyResult<Vec<PyObject>>>()?;

        if output.is_empty() {
            Ok(None)
        } else {
            Ok(Some(output))
        }
    }
}

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

/// Random crap I like to use.
#[pymodule]
fn miguel_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(match_indices, m)?)?;
    m.add_function(wrap_pyfunction!(match_utf16_indices, m)?)?;
    m.add_function(wrap_pyfunction!(match_byte_indices, m)?)?;
    m.add_function(wrap_pyfunction!(rmatch_indices, m)?)?;
    m.add_function(wrap_pyfunction!(rmatch_utf16_indices, m)?)?;
    m.add_function(wrap_pyfunction!(rmatch_byte_indices, m)?)?;
    m.add_function(wrap_pyfunction!(utf16len, m)?)?;
    m.add_class::<ChunksIter>()?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
