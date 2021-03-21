#[macro_use] mod macros;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
// use hashbrown::HashMap;

pub trait AnyExt {
    fn type_name(&self) -> &'static str;
}

impl<T> AnyExt for T {
    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}

fn test_with_nogil(
) -> () {
}

#[pyfunction]
fn test(
    py: Python,
) -> () {
    py.allow_threads(|| test_with_nogil())
}

/// A Python module implemented in Rust.
#[pymodule]
fn testlib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(test, m)?)?;
    Ok(())
}
