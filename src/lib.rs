use pyo3::prelude::*;

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
    Ok(())
}
