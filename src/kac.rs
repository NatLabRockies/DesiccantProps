// KAc fits from the approved stratified-tank implementation, based on
// Pokharel et al. (2025), DOI 10.1016/j.pes.2025.100149.

fn cubic(z: f64, a: f64, b: f64, c: f64, d: f64) -> f64 {
    ((a * z + b) * z + c) * z + d
}

pub fn density(t: f64, x: f64) -> f64 {
    let c = t - 273.15;
    let d = cubic(c, 2.59266e-8, -9.45856e-6, 0.000531834, 0.082511747);
    let e = cubic(c, -5.80973e-8, 1.65957e-5, -0.001396551, 0.528657673);
    let f = cubic(c, 2.49255e-8, -7.01397e-6, 0.0000525806, 0.999539253);
    ((d * x + e) * x + f) * 1000.0
}

fn cp_coefficients(x: f64) -> [f64; 4] {
    [
        cubic(x, 0.0000270879, -0.0000416783, 0.000019989, -0.0000029605),
        cubic(x, -0.002402515, 0.003623185, -0.00165879, 0.000214682),
        cubic(x, 0.005267041, -0.005500196, -0.004048008, 0.005052775),
        cubic(x, -3.678733888, 4.924863403, -4.066259526, 3.896593658),
    ]
}

pub fn specific_heat(t: f64, x: f64) -> f64 {
    let [a, b, c, d] = cp_coefficients(x);
    cubic(t - 273.15, a, b, c, d) * 1000.0
}

pub fn sensible_enthalpy(t: f64, x: f64) -> f64 {
    let [a, b, c, d] = cp_coefficients(x);
    let z = t - 273.15;
    z * cubic(z, a / 4.0, b / 3.0, c / 2.0, d) * 1000.0
}

pub fn conductivity(t: f64, x: f64) -> f64 {
    2.78165e-2 + 2.31072e-3 * t + 1.01253e-1 * x * x - 1.7695e-3 * t * x - 1.11004e-3 / x
}

pub fn vapor_pressure(t: f64, x: f64) -> f64 {
    let a = cubic(x, 46.5011, -63.5126, 27.62462, 14.34408);
    let b = cubic(x, -22162.0, 31092.19, -14890.3, -2804.45);
    (a + b / t).exp() * 1000.0
}

pub fn diffusivity(t: f64, x: f64) -> f64 {
    let a = (0.001201591 * t - 0.7095) * t + 106.49;
    let b = (-0.00111638 * t + 0.59) * t - 80.922;
    let c = (0.000428621 * t - 0.202) * t + 24.473;
    ((a * x + b) * x + c) * 1e-9
}

pub fn viscosity(t: f64, x: f64) -> f64 {
    let a = cubic(x, 0.434173202, -10.5717065, 5.984514, -13.1522);
    let b = cubic(x, 4768.625062, -624.812954, -47.6047, 1744.712);
    (a + b / t).exp()
}
