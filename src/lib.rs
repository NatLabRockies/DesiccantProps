#![doc = include_str!("../README.md")]

mod kac;
mod licl;
#[cfg(feature = "python")]
mod python;

use std::fmt;

/// Failure to construct a solution or evaluate a property.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// An input is not finite or is outside the supported interval.
    OutOfRange {
        /// Name of the input.
        input: &'static str,
        /// Requested value.
        value: f64,
        /// Inclusive lower bound.
        min: f64,
        /// Inclusive upper bound.
        max: f64,
    },
    /// The LiCl mass fraction exceeds the modeled solubility.
    AboveSolubility {
        /// Requested solute mass fraction.
        mass_fraction: f64,
        /// Maximum dissolved solute mass fraction.
        limit: f64,
    },
    /// The selected solution model does not provide this property.
    Unsupported(&'static str),
    /// Failure evaluating the reference water properties.
    Water(String),
    /// The correlation returned a nonphysical result.
    InvalidResult(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfRange {
                input,
                value,
                min,
                max,
            } => {
                write!(f, "{input} {value} is outside [{min}, {max}]")
            }
            Self::AboveSolubility {
                mass_fraction,
                limit,
            } => {
                write!(
                    f,
                    "LiCl mass fraction {mass_fraction} exceeds solubility {limit}"
                )
            }
            Self::Unsupported(name) => write!(f, "{name} is not available for this solution"),
            Self::Water(message) => write!(f, "reference water properties: {message}"),
            Self::InvalidResult(name) => {
                write!(f, "{name} correlation returned a nonphysical result")
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<vle_steam::SteamError> for Error {
    fn from(error: vle_steam::SteamError) -> Self {
        Self::Water(error.to_string())
    }
}

fn bounds(input: &'static str, value: f64, min: f64, max: f64) -> Result<(), Error> {
    if (min..=max).contains(&value) {
        Ok(())
    } else {
        Err(Error::OutOfRange {
            input,
            value,
            min,
            max,
        })
    }
}

fn positive(name: &'static str, value: f64) -> Result<f64, Error> {
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(Error::InvalidResult(name))
    }
}

#[derive(Clone, Copy, Debug)]
enum Model {
    KAc,
    LiCl,
}

/// Aqueous desiccant at a fixed solute mass fraction.
///
/// Create a new value at each concentration when modeling water transfer.
/// All temperature arguments are in kelvin; all outputs use SI units.
#[derive(Clone, Copy, Debug)]
pub struct Solution {
    model: Model,
    x: f64,
}

impl Solution {
    /// Construct potassium acetate solution with solute mass fraction 0.10–0.70.
    pub fn potassium_acetate(mass_fraction: f64) -> Result<Self, Error> {
        bounds("KAc mass fraction", mass_fraction, 0.10, 0.70)?;
        Ok(Self {
            model: Model::KAc,
            x: mass_fraction,
        })
    }

    /// Construct lithium chloride solution with solute mass fraction 0–0.50.
    ///
    /// Each property call also checks the solubility limit at its temperature.
    pub fn lithium_chloride(mass_fraction: f64) -> Result<Self, Error> {
        bounds("LiCl mass fraction", mass_fraction, 0.0, 0.50)?;
        Ok(Self {
            model: Model::LiCl,
            x: mass_fraction,
        })
    }

    /// Solute mass fraction (kg of salt per kg of solution).
    pub fn mass_fraction(self) -> f64 {
        self.x
    }

    /// Inclusive software temperature guards in K, before any phase checks.
    pub fn temperature_limits(self) -> (f64, f64) {
        match self.model {
            Model::KAc => (293.15, 353.15),
            Model::LiCl => (273.15, 363.15),
        }
    }

    fn check(self, t: f64) -> Result<(), Error> {
        let (min, max) = self.temperature_limits();
        bounds("temperature (K)", t, min, max)?;
        if matches!(self.model, Model::LiCl) {
            let limit = licl::solubility(t);
            if self.x > limit {
                return Err(Error::AboveSolubility {
                    mass_fraction: self.x,
                    limit,
                });
            }
        }
        Ok(())
    }

    /// Liquid density in kg/m³.
    pub fn density(self, temperature_k: f64) -> Result<f64, Error> {
        self.check(temperature_k)?;
        positive(
            "density",
            match self.model {
                Model::KAc => kac::density(temperature_k, self.x),
                Model::LiCl => licl::density(temperature_k, self.x)?,
            },
        )
    }

    /// Specific heat capacity in J/(kg·K).
    pub fn specific_heat(self, temperature_k: f64) -> Result<f64, Error> {
        self.check(temperature_k)?;
        positive(
            "specific heat",
            match self.model {
                Model::KAc => kac::specific_heat(temperature_k, self.x),
                Model::LiCl => licl::specific_heat(temperature_k, self.x),
            },
        )
    }

    /// Dynamic viscosity in Pa·s.
    pub fn viscosity(self, temperature_k: f64) -> Result<f64, Error> {
        self.check(temperature_k)?;
        positive(
            "viscosity",
            match self.model {
                Model::KAc => kac::viscosity(temperature_k, self.x),
                Model::LiCl => licl::viscosity(temperature_k, self.x)?,
            },
        )
    }

    /// Thermal conductivity in W/(m·K).
    pub fn conductivity(self, temperature_k: f64) -> Result<f64, Error> {
        self.check(temperature_k)?;
        positive(
            "conductivity",
            match self.model {
                Model::KAc => kac::conductivity(temperature_k, self.x),
                Model::LiCl => licl::conductivity(temperature_k, self.x)?,
            },
        )
    }

    /// Water diffusivity in m²/s.
    pub fn water_diffusivity(self, temperature_k: f64) -> Result<f64, Error> {
        self.check(temperature_k)?;
        positive(
            "water diffusivity",
            match self.model {
                Model::KAc => kac::diffusivity(temperature_k, self.x),
                Model::LiCl => licl::diffusivity(temperature_k, self.x)?,
            },
        )
    }

    /// Equilibrium water vapor pressure above the solution in Pa.
    pub fn vapor_pressure(self, temperature_k: f64) -> Result<f64, Error> {
        self.check(temperature_k)?;
        let water_pressure = vle_steam::psat(temperature_k)? * 1000.0;
        Ok(self.activity_at(temperature_k, water_pressure)? * water_pressure)
    }

    /// Equilibrium water vapor pressure divided by pure-water saturation pressure.
    pub fn water_activity(self, temperature_k: f64) -> Result<f64, Error> {
        self.check(temperature_k)?;
        self.activity_at(temperature_k, vle_steam::psat(temperature_k)? * 1000.0)
    }

    fn activity_at(self, temperature_k: f64, water_pressure: f64) -> Result<f64, Error> {
        let value = match self.model {
            Model::KAc => kac::vapor_pressure(temperature_k, self.x) / water_pressure,
            Model::LiCl if self.x == 0.0 => 1.0,
            Model::LiCl => licl::activity(temperature_k, self.x),
        };
        if value > 0.0 && value <= 1.0 {
            Ok(value)
        } else {
            Err(Error::InvalidResult("water activity"))
        }
    }

    /// Sensible enthalpy in J/kg, integrated from 273.15 K at fixed composition.
    ///
    /// Not a solution enthalpy datum for streams with different compositions.
    pub fn sensible_enthalpy(self, temperature_k: f64) -> Result<f64, Error> {
        self.check(temperature_k)?;
        Ok(match self.model {
            Model::KAc => kac::sensible_enthalpy(temperature_k, self.x),
            Model::LiCl => licl::sensible_enthalpy(temperature_k, self.x),
        })
    }

    /// Recover temperature in K from fixed-composition sensible enthalpy in J/kg.
    ///
    /// The caller supplies a bracket inside the supported single-phase domain.
    pub fn temperature_from_sensible_enthalpy(
        self,
        enthalpy: f64,
        min_k: f64,
        max_k: f64,
    ) -> Result<f64, Error> {
        bounds(
            "upper bracket temperature (K)",
            max_k,
            min_k,
            self.temperature_limits().1,
        )?;
        let min_h = self.sensible_enthalpy(min_k)?;
        let max_h = self.sensible_enthalpy(max_k)?;
        bounds("sensible enthalpy (J/kg)", enthalpy, min_h, max_h)?;
        let (mut lo, mut hi) = (min_k, max_k);
        while hi - lo > 1e-8 {
            let mid = (lo + hi) / 2.0;
            if self.sensible_enthalpy(mid)? < enthalpy {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        Ok((lo + hi) / 2.0)
    }

    /// Conde's differential heat of dilution term in J/kg of transferred water.
    ///
    /// LiCl only; this is not bulk solution enthalpy.
    pub fn heat_of_dilution(self, temperature_k: f64) -> Result<f64, Error> {
        if !matches!(self.model, Model::LiCl) {
            return Err(Error::Unsupported("heat of dilution"));
        }
        self.check(temperature_k)?;
        Ok(licl::heat_of_dilution(temperature_k, self.x))
    }
}

/// Maximum dissolved LiCl mass fraction on the modeled salt precipitation line.
///
/// Accepts 273.15–363.15 K. Does not describe the ice line.
pub fn lithium_chloride_solubility(temperature_k: f64) -> Result<f64, Error> {
    bounds("temperature (K)", temperature_k, 273.15, 363.15)?;
    Ok(licl::solubility(temperature_k))
}
