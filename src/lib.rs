use std::collections::HashMap;

use pyo3::prelude::*;

#[pyclass]
pub struct RCacheManager {
    _raw_guilds: HashMap::<u64, Py<PyObject>>
}

#[pymethods]
impl RCacheManager {
    #[new]
    fn new() -> Self {
        Self {
            _raw_guilds: HashMap::new()
        }
    }

    /// Cached Guilds Count
    #[pyo3(text_signature = "($self, /)")]
    fn guilds_count<'a>(&self, _py: Python<'a>) -> PyResult<usize> {
        Ok(
            self._raw_guilds.len()
        )
    }
}

#[pymodule]
fn melisa_rcache(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<RCacheManager>()?;
    Ok(())
}