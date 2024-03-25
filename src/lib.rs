//! This is the documentation for rateslib-rs

#[cfg(test)]
mod tests;

// type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
// type GenericResult<T> = Result<T, GenericError>;

use ndarray::Array1;
use pyo3::prelude::*;

pub mod dual;
use dual::dual1::Dual;
use dual::dual2::Dual2;
use dual::linalg_py::{dsolve1_py, dsolve2_py, fdsolve1_py, fdsolve2_py};

pub mod splines;
use splines::spline_py::{PPSplineF64, PPSplineDual, PPSplineDual2};

#[pymodule]
fn rateslibrs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Dual>()?;
    m.add_class::<Dual2>()?;
    m.add_class::<PPSplineF64>()?;
    m.add_class::<PPSplineDual>()?;
    m.add_class::<PPSplineDual2>()?;

    // m.add_class::<PPSpline<f64>>()?;
    m.add_function(wrap_pyfunction!(dsolve1_py, m)?)?;
    m.add_function(wrap_pyfunction!(dsolve2_py, m)?)?;
    m.add_function(wrap_pyfunction!(fdsolve1_py, m)?)?;
    m.add_function(wrap_pyfunction!(fdsolve2_py, m)?)?;
    Ok(())
}
