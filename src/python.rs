use pyo3::prelude::*;

#[pyfunction]
fn stub() -> String {
	crate::stub()
}

#[pymodule(name="theodolite")]
fn module_root(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(stub, m)?)
}
