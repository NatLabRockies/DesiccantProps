use crate::{Error, Solution, lithium_chloride_solubility};
use pyo3::{exceptions::PyValueError, prelude::*};

fn py_error(error: Error) -> PyErr {
    PyValueError::new_err(error.to_string())
}

/// Aqueous desiccant at a fixed solute mass fraction, with SI property methods.
#[pyclass(name = "Solution", module = "desiccant_props", frozen)]
struct PySolution(Solution);

#[pymethods]
impl PySolution {
    /// Create a potassium acetate solution.
    #[staticmethod]
    fn potassium_acetate(mass_fraction: f64) -> PyResult<Self> {
        Solution::potassium_acetate(mass_fraction)
            .map(Self)
            .map_err(py_error)
    }

    /// Create a lithium chloride solution.
    #[staticmethod]
    fn lithium_chloride(mass_fraction: f64) -> PyResult<Self> {
        Solution::lithium_chloride(mass_fraction)
            .map(Self)
            .map_err(py_error)
    }

    #[getter]
    fn mass_fraction(&self) -> f64 {
        self.0.mass_fraction()
    }

    fn temperature_limits(&self) -> (f64, f64) {
        self.0.temperature_limits()
    }

    fn density(&self, temperature_k: f64) -> PyResult<f64> {
        self.0.density(temperature_k).map_err(py_error)
    }

    fn specific_heat(&self, temperature_k: f64) -> PyResult<f64> {
        self.0.specific_heat(temperature_k).map_err(py_error)
    }

    fn viscosity(&self, temperature_k: f64) -> PyResult<f64> {
        self.0.viscosity(temperature_k).map_err(py_error)
    }

    fn conductivity(&self, temperature_k: f64) -> PyResult<f64> {
        self.0.conductivity(temperature_k).map_err(py_error)
    }

    fn water_diffusivity(&self, temperature_k: f64) -> PyResult<f64> {
        self.0.water_diffusivity(temperature_k).map_err(py_error)
    }

    fn vapor_pressure(&self, temperature_k: f64) -> PyResult<f64> {
        self.0.vapor_pressure(temperature_k).map_err(py_error)
    }

    fn water_activity(&self, temperature_k: f64) -> PyResult<f64> {
        self.0.water_activity(temperature_k).map_err(py_error)
    }

    fn sensible_enthalpy(&self, temperature_k: f64) -> PyResult<f64> {
        self.0.sensible_enthalpy(temperature_k).map_err(py_error)
    }

    fn temperature_from_sensible_enthalpy(
        &self,
        enthalpy: f64,
        min_k: f64,
        max_k: f64,
    ) -> PyResult<f64> {
        self.0
            .temperature_from_sensible_enthalpy(enthalpy, min_k, max_k)
            .map_err(py_error)
    }

    fn heat_of_dilution(&self, temperature_k: f64) -> PyResult<f64> {
        self.0.heat_of_dilution(temperature_k).map_err(py_error)
    }
}

#[pyfunction(name = "lithium_chloride_solubility")]
fn py_lithium_chloride_solubility(temperature_k: f64) -> PyResult<f64> {
    lithium_chloride_solubility(temperature_k).map_err(py_error)
}

#[pymodule]
fn _native(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PySolution>()?;
    module.add_function(wrap_pyfunction!(py_lithium_chloride_solubility, module)?)
}
