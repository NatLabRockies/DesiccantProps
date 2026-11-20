Using the library
=================

After publication, install with ``cargo add desiccant-props`` or
``pip install desiccant-props``. See :doc:`releasing` for local builds.

.. code-block:: python

   from desiccant_props import Solution, lithium_chloride_solubility

   t = 313.15  # K
   kac = Solution.potassium_acetate(0.50)
   licl = Solution.lithium_chloride(0.35)
   print(kac.vapor_pressure(t))
   print(licl.water_activity(t))
   print(lithium_chloride_solubility(t))

.. code-block:: rust

   use desiccant_props::{Error, Solution};

   fn main() -> Result<(), Error> {
       let solution = Solution::lithium_chloride(0.35)?;
       let h = solution.sensible_enthalpy(313.15)?;
       let temperature = solution.temperature_from_sensible_enthalpy(h, 300.0, 330.0)?;
       println!("{temperature} K");
       Ok(())
   }

Both interfaces expose the same operations. Temperatures and bracket
endpoints are in kelvin; mass fractions are dimensionless (``0.35`` means
35 wt%). Use the `Rust API reference <https://docs.rs/desiccant-props>`_
for signatures. Rust returns ``Result``; Python raises ``ValueError``.

.. list-table::
   :header-rows: 1

   * - Method
     - Output
   * - ``density``
     - kg/m³
   * - ``specific_heat``
     - J/(kg K)
   * - ``viscosity``
     - Pa s
   * - ``conductivity``
     - W/(m K)
   * - ``water_diffusivity``
     - m²/s
   * - ``vapor_pressure``
     - Pa, equilibrium water vapor pressure
   * - ``water_activity``
     - Ratio of equilibrium vapor pressure to pure-water saturation pressure
   * - ``sensible_enthalpy``
     - J/kg of solution
   * - ``heat_of_dilution``
     - J/kg of transferred water (LiCl only)

For variable-concentration simulations, construct a solution at each stream
concentration and conserve salt and water in the equipment model. Sensible
enthalpy integrates specific heat at fixed composition with a zero at
273.15 K for each composition. It does not include heat of mixing and is not
a common enthalpy datum for absorber/desorber balances. The LiCl differential
heat of dilution is exposed separately, not added to bulk solution enthalpy.
