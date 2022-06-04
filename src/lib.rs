use pyo3::prelude::*;

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
    fn __repr__(&self) -> String {
        let mut intervals: Vec<String> = vec![];
        self._intervals.iter().for_each(|&f| intervals.push(format!("[{}, {}]", f.0, f.1)));
        return format!("<Interval {}>", intervals.join(", "));
    }
    fn __contains__(&self, item: isize) -> bool {
        for &interval in self._intervals.iter() {
            if interval.0 <= item && item <= interval.1 {
                return true;
            }
        }
        return false;
    }
}


/// Returns the UTF-16 length of a string.
#[pyfunction]
fn utf16len(string: &str) -> usize {
    let mut length = 0;
    string.chars().for_each(|char| length += char.len_utf16());
    return length;
}

/// A Python module implemented in Rust.
#[pymodule]
fn miguel_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(utf16len, m)?)?;
    m.add_class::<Interval>()?;
    Ok(())
}
