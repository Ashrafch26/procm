/// functionalities that used in python
use pyo3::prelude::*;

mod engine;
mod json;
mod kahn;
mod logs;
mod process;
mod task_manager;

#[pyfunction]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[pymodule]
fn pyrust(m: &Bound<'_ , PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    Ok(())
}
