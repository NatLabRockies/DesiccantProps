// Conde (2004), DOI 10.1016/j.ijthermalsci.2003.09.003.
// Port of the NatLabRockies FY25_FIA MATLAB LiCl routines.
use vle_steam::{SteamError, SteamState};

pub fn activity(t: f64, x: f64) -> f64 {
    let a = 2.0 - (1.0 + (x / 0.28).powf(4.3)).powf(0.60);
    let b = (1.0 + (x / 0.21).powf(5.10)).powf(0.49) - 1.0;
    (1.0 - (1.0 + (x / 0.362).powf(-4.75)).powf(-0.40) - 0.03 * (-(x - 0.1).powi(2) / 0.005).exp())
        * (a + b * t / 647.14)
}

fn water_density(t: f64, p_kpa: f64) -> Result<f64, SteamError> {
    Ok(SteamState::tp(t + 0.003, p_kpa)?.rho)
}

fn density_at(t: f64, x: f64, p_kpa: f64) -> Result<f64, SteamError> {
    let c = x / (1.0 - x);
    Ok(water_density(t, p_kpa)? * (1.0 + c * (0.540966 + c * (-0.303792 + c * 0.100791))))
}

pub fn density(t: f64, x: f64) -> Result<f64, SteamError> {
    density_at(t, x, 101.325)
}

pub fn conductivity(t: f64, x: f64) -> Result<f64, SteamError> {
    let base = vle_steam::thermal_conductivity(t + 0.003, 131.325)?;
    Ok(base - (0.0108958 - 0.0117882 * x) * x * density_at(t, x, 85.0)? / 42.4)
}

pub fn viscosity(t: f64, x: f64) -> Result<f64, SteamError> {
    let c = x / (1.0 - x).powf(1.0 / 0.6);
    let rate = 0.090481 * c.powf(3.6) + 1.390262 * c + 0.675875 * c * 647.14 / t - 0.583517 * c * c;
    Ok(vle_steam::viscosity(t + 0.003, 85.0)? * rate.exp())
}

pub fn diffusivity(t: f64, x: f64) -> Result<f64, SteamError> {
    let molar_mass = 18.015268;
    let critical_volume = 1e6 * molar_mass / 322.0;
    let volume = 1e6 * molar_mass / water_density(t, 101.325)?;
    let viscosity = vle_steam::viscosity(t + 0.003, 101.325)?;
    let base = 0.11353e-16 * 8314.0 * t / viscosity * critical_volume.powf(2.0 / 3.0) / volume;
    Ok(base * (1.0 - (1.0 + (x.sqrt() / 0.52).powf(-4.92)).powf(-0.56)))
}

fn concentration_term(x: f64) -> f64 {
    if x > 0.31 {
        0.12825 + 0.62934 * x
    } else {
        x * (1.43980 + x * (-1.24317 - 0.12070 * x))
    }
}

const TERMS: [(f64, f64); 3] = [(58.5225, 0.02), (-105.6343, 0.04), (47.7948, 0.06)];

pub fn specific_heat(t: f64, x: f64) -> f64 {
    let theta = t / 228.0 - 1.0;
    let correction: f64 = TERMS.iter().map(|(a, b)| a * theta.powf(*b)).sum();
    4184.0 * (1.0 - concentration_term(x) * correction)
}

pub fn sensible_enthalpy(t: f64, x: f64) -> f64 {
    let theta = t / 228.0 - 1.0;
    let reference = 273.15_f64 / 228.0 - 1.0;
    let correction: f64 = TERMS
        .iter()
        .map(|(a, b)| a * 228.0 * (theta.powf(b + 1.0) - reference.powf(b + 1.0)) / (b + 1.0))
        .sum();
    4184.0 * (t - 273.15 - concentration_term(x) * correction)
}

pub fn heat_of_dilution(t: f64, x: f64) -> f64 {
    let ratio = x / (0.6 - x) / 0.845;
    1000.0 * (169.105 + 457.85 * t / 647.096) * (1.0 + ratio.powf(-1.965)).powf(-2.265)
}

pub fn solubility(t: f64) -> f64 {
    let theta = t / 647.14;
    let (a, b, c): (f64, f64, f64) = if t <= 292.25 {
        (-0.31522, 2.88248, -2.62433)
    } else {
        (-1.31231, 6.17767, -5.03479)
    };
    (-b + (b * b - 4.0 * c * (a - theta)).sqrt()) / (2.0 * c)
}
