use pyo3::prelude::*;
use libhello::adder;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// Summ tow ints using native call
#[pyfunction]
fn native_adder(a: i64, b: i64) -> PyResult<i64> {
    Ok(adder(a, b))
}

/// A Python module implemented in Rust.
#[pymodule]
fn pylibhello(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(native_adder, m)?)?;
    Ok(())
}