#[cfg(test)]
mod tests;

use ndarray::Array1;
use pyo3::prelude::*;

pub mod dual;
use dual::dual1::Dual;
use dual::dual2::Dual2;
use dual::linalg_py::{dsolve1_py, dsolve2_py};

pub mod splines;
use splines::PPSpline;

#[pymodule]
fn rateslibrs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Dual>()?;
    m.add_class::<Dual2>()?;

    // m.add_class::<PPSpline<f64>>()?;
    m.add_function(wrap_pyfunction!(dsolve1_py, m)?)?;
    m.add_function(wrap_pyfunction!(dsolve2_py, m)?)?;
    Ok(())
}
