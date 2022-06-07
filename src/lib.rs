use std::cmp::max;

use pyo3::prelude::*;

/// A function that merges overlapping intervals in a sequence.
#[pyfunction]
fn merge_intervals(mut intervals: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
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
    return intervals;
}

/// A function that returns the UTF-16 length of a string.
#[pyfunction]
fn utf16len(string: &str) -> usize {
    return string.chars().map(|char| char.len_utf16()).sum();
}

/// A class used to represent intervals.
#[pyclass]
struct Interval {
    _intervals: Vec<(i32, i32)>,
}

#[pymethods]
impl Interval {
    #[new]
    fn new(interval_list: Option<Vec<(i32, i32)>>) -> Self {
        match interval_list {
            Some(f) => Interval { _intervals: f },
            None => Interval { _intervals: vec![] }
        }
    }
    fn __contains__(&self, item: i32) -> bool {
        return self._intervals.iter().any(|&f| f.0 <= item && item <= f.1);
    }
    fn __repr__(&self) -> String {
        return format!("Interval([{}])", self._intervals.iter().map(|&f| format!("({}, {})", f.0, f.1)).collect::<Vec<String>>().join(", "));
    }
    fn __str__(&self) -> String {
        return format!("({})", self._intervals.iter().map(|&f| format!("[{}, {}]", f.0, f.1)).collect::<Vec<String>>().join(" ∪ "));
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn miguel_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(merge_intervals, m)?)?;
    m.add_function(wrap_pyfunction!(utf16len, m)?)?;
    m.add_class::<Interval>()?;
    Ok(())
}
