use desiccant_props::{Error, Solution, lithium_chloride_solubility};

fn close(actual: f64, expected: f64, relative: f64) {
    assert!(
        (actual - expected).abs() <= relative * expected.abs(),
        "actual={actual}, expected={expected}, relative={relative}"
    );
}

#[test]
fn kac_matches_stratified_tank_reference() -> Result<(), Error> {
    let s = Solution::potassium_acetate(0.50)?;
    let t = 313.15;
    for (actual, expected) in [
        (s.density(t)?, 1262.40829825),
        (s.specific_heat(t)?, 2712.57869475),
        (s.conductivity(t)?, 0.4974521755),
        (s.sensible_enthalpy(t)?, 107046.53542333332),
        (s.viscosity(t)?, 0.0028843959926759615),
        (s.vapor_pressure(t)?, 3803.644663200722),
        (s.water_diffusivity(t)?, 9.644352220218658e-10),
    ] {
        close(actual, expected, 1e-10);
    }
    Ok(())
}

#[test]
fn licl_matches_matlab_formulas_with_coolprop_reference_water() -> Result<(), Error> {
    for (t, x, rho, k, mu, pressure) in [
        (298.15, 0.20, 1114.52826, 0.56165146, 0.00194305, 2186.57924),
        (313.15, 0.35, 1209.45415, 0.56091708, 0.00367002, 2270.75015),
        (
            353.15,
            0.42,
            1234.85677,
            0.59429631,
            0.00322452,
            10175.29020,
        ),
    ] {
        let s = Solution::lithium_chloride(x)?;
        close(s.density(t)?, rho, 0.0002);
        close(s.conductivity(t)?, k, 0.001);
        close(s.viscosity(t)?, mu, 0.001);
        close(s.vapor_pressure(t)?, pressure, 0.0002);
    }
    Ok(())
}

#[test]
fn sensible_enthalpy_derivative_and_inverse() -> Result<(), Error> {
    for s in [
        Solution::potassium_acetate(0.50)?,
        Solution::lithium_chloride(0.25)?,
        Solution::lithium_chloride(0.35)?,
    ] {
        let t = 313.15;
        let h = s.sensible_enthalpy(t)?;
        close(
            s.temperature_from_sensible_enthalpy(h, 300.0, 330.0)?,
            t,
            1e-10,
        );
        let slope = (s.sensible_enthalpy(t + 0.01)? - s.sensible_enthalpy(t - 0.01)?) / 0.02;
        close(slope, s.specific_heat(t)?, 1e-7);
        assert!(
            s.temperature_from_sensible_enthalpy(h, 315.0, 330.0)
                .is_err()
        );
    }
    Ok(())
}

#[test]
fn out_of_domain_and_unsupported_states_fail_explicitly() -> Result<(), Error> {
    assert!(Solution::potassium_acetate(f64::NAN).is_err());
    let kac = Solution::potassium_acetate(0.20)?;
    assert!(kac.density(25.0).is_err());
    assert!(matches!(
        kac.vapor_pressure(298.15),
        Err(Error::InvalidResult(_))
    ));
    assert!(matches!(
        kac.heat_of_dilution(313.15),
        Err(Error::Unsupported(_))
    ));
    let licl = Solution::lithium_chloride(0.42)?;
    assert!(matches!(
        licl.density(273.15),
        Err(Error::AboveSolubility { .. })
    ));
    assert!(lithium_chloride_solubility(313.15)? > licl.mass_fraction());
    assert!(licl.density(313.15).is_ok());
    Ok(())
}
