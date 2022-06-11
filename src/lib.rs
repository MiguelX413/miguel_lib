use pyo3::exceptions::PyValueError;
use std::cmp::max;

use pyo3::prelude::*;
use pyo3::types::PyTuple;

fn merge_intervals(intervals: &mut Vec<(i32, i32)>) {
    intervals.sort_by_key(|&a| a.0);
    let mut index: usize = 0;
    for i in 1..intervals.len() {
        if intervals[index].1 >= intervals[i].0 {
            intervals[index].1 = max(intervals[index].1, intervals[i].1);
        } else {
            index += 1;
            intervals[index] = intervals[i];
        }
    }
    intervals.truncate(index + 1);
}

/// A function that returns the UTF-16 length of a string.
#[pyfunction]
fn utf16len(string: &str) -> usize {
    string.chars().map(|char| char.len_utf16()).sum()
}

/// A class used to represent intervals.
#[pyclass]
struct Interval {
    intervals: Vec<(i32, i32)>,
}

#[pymethods]
impl Interval {
    #[new]
    fn py_new(interval_list: Option<Vec<(i32, i32)>>) -> PyResult<Self> {
        match interval_list {
            Some(mut f) => {
                if f.iter().any(|&subinterval| subinterval.0 > subinterval.1) {
                    Err(PyValueError::new_err(
                        "Start point of sub-interval cannot be greater than its end point",
                    ))
                } else {
                    merge_intervals(&mut f);
                    Ok(Interval { intervals: f })
                }
            }
            None => Ok(Interval { intervals: vec![] }),
        }
    }
    #[args(other = "*")]
    fn union(&self, other: &PyTuple) -> PyResult<Interval> {
        let mut output = self.clone();
        output.union_update(other)?;
        Ok(output)
    }
    #[args(other = "*")]
    fn union_update(&mut self, other: &PyTuple) -> PyResult<()> {
        let inputs: Vec<Interval> = other.extract()?;
        inputs
            .iter()
            .for_each(|f| self.intervals.append(&mut f.intervals.clone()));
        if !inputs.is_empty() {
            merge_intervals(&mut self.intervals);
        }
        Ok(())
    }
    fn __contains__(&self, item: i32) -> bool {
        self.intervals.iter().any(|&f| f.0 <= item && item <= f.1)
    }
    fn __repr__(&self) -> String {
        format!(
            "Interval([{}])",
            self.intervals
                .iter()
                .map(|&f| format!("({}, {})", f.0, f.1))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
    fn __str__(&self) -> String {
        format!(
            "({})",
            self.intervals
                .iter()
                .map(|&f| format!("[{}, {}]", f.0, f.1))
                .collect::<Vec<String>>()
                .join(" ∪ ")
        )
    }
    fn __or__(&self, other: &Interval) -> Interval {
        let mut output = self.clone();
        output.__ior__(other);
        output
    }
    fn __ior__(&mut self, other: &Interval) {
        self.intervals.append(&mut other.intervals.clone());
        merge_intervals(&mut self.intervals);
    }
}

impl Clone for Interval {
    fn clone(&self) -> Interval {
        Interval {
            intervals: self.intervals.clone(),
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn miguel_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(utf16len, m)?)?;
    m.add_class::<Interval>()?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
