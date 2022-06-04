use std::cmp::max;

use pyo3::prelude::*;

/// Merge intervals overlapping in a list
#[pyfunction]
fn merge_intervals(mut intervals: Vec<(isize, isize)>) -> Vec<(isize, isize)> {
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

/// Returns the UTF-16 length of a string.
#[pyfunction]
fn utf16len(string: &str) -> usize {
    let mut length = 0;
    string.chars().for_each(|char| length += char.len_utf16());
    return length;
}

/// Interval class.
#[pyclass]
struct Interval {
    _intervals: Vec<(isize, isize)>,
}

#[pymethods]
impl Interval {
    #[new]
    fn new(interval_list: Option<Vec<(isize, isize)>>) -> Self {
        match interval_list {
            Some(f) => Interval { _intervals: f },
            None => Interval { _intervals: vec![] }
        }
    }
    fn __contains__(&self, item: isize) -> bool {
        for &interval in self._intervals.iter() {
            if interval.0 <= item && item <= interval.1 {
                return true;
            }
        }
        return false;
    }
    fn __repr__(&self) -> String {
        let mut intervals: Vec<String> = vec![];
        self._intervals.iter().for_each(|&f| intervals.push(format!("({}, {})", f.0, f.1)));
        return format!("Interval([{}])", intervals.join(", "));
    }
    fn __str__(&self) -> String {
        let mut intervals: Vec<String> = vec![];
        self._intervals.iter().for_each(|&f| intervals.push(format!("[{}, {}]", f.0, f.1)));
        return format!("({})", intervals.join(" ∪ "));
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
