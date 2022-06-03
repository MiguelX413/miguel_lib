use pyo3::prelude::*;

/// Interval class.
#[pyclass]
struct Interval {
    _intervals: Vec<(isize, isize)>,
}

#[pymethods]
impl Interval {
    #[new]
    fn new(value: Vec<(isize, isize)>) -> Self {
        Interval { _intervals: value }
    }
    fn __repr__(&self) -> String {
        let mut intervals: Vec<String> = vec![];
        self._intervals.iter().for_each(|&f| intervals.push(format!("[{}, {}]", f.0, f.1)));
        return format!("<Interval {}>", intervals.join(", "));
    }
}


/// Returns the UTF-16 length of a string.
#[pyfunction]
fn utf16len(string: &str) -> usize {
    let mut length = 0;
    for char in string.chars() {
        length += char.len_utf16();
    }
    return length;
}

/// A Python module implemented in Rust.
#[pymodule]
fn miguel_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(utf16len, m)?)?;
    m.add_class::<Interval>()?;
    Ok(())
}
