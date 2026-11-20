Models and limits
=================

Potassium acetate
-----------------

Based on Pokharel et al., *Material properties of aqueous potassium acetate
solutions for use in liquid desiccant air conditioning*,
`doi:10.1016/j.pes.2025.100149 <https://doi.org/10.1016/j.pes.2025.100149>`_.
Coefficients, including the conductivity fit, are taken from the lab's
stratified-tank Rust implementation (source blob
``56aa5792786da7d170065f46fb874da2063478d8``). The enthalpy inverse solves
the integral directly and does not use the original extrapolation fallback.

The initial software guard is 0.10–0.70 solute mass fraction and
293.15–353.15 K. These guardrails are not experimental validity claims for
every property across the entire rectangle. The current model does not
implement a KAc crystallization boundary or heat of dilution. Verify
single-phase conditions and the supporting data range for your application.

The supplied vapor-pressure fit gives approximately 3.80 kPa at 50 wt%
and 40 °C; the article abstract reports 3.1 kPa at that point. The port
preserves the supplied fit rather than refitting it. Reconcile this
difference against the supplementary data before relying on this model
for design. At some dilute states the supplied fit exceeds pure-water
saturation pressure; both vapor pressure and water activity return an
error at those states.

Lithium chloride
----------------

Based on Conde, M. R. (2004), *Properties of aqueous solutions of lithium and
calcium chlorides: formulations for use in air conditioning equipment
design*, `doi:10.1016/j.ijthermalsci.2003.09.003
<https://doi.org/10.1016/j.ijthermalsci.2003.09.003>`_, as implemented in the
lab's `FY25 MATLAB LiCl routines
<https://github.com/NatLabRockies/Desiccant_AC_EDDR_TES/tree/main/FY25_FIA/%24FluidPropertyLibrary/LD_LiCl>`_.

The initial software guard is 0–0.50 solute mass fraction and
273.15–363.15 K, further restricted by the LiCl precipitation limit.
``lithium_chloride_solubility`` returns the salt precipitation line, not
the ice boundary. The pure-water reference properties use IAPWS correlations
through ``vle-steam`` instead of CoolProp. Density, viscosity, conductivity
and diffusivity preserve the MATLAB routines' reference pressures and
0.003 K water-property offset, so results are expected to be close, not
bitwise identical. At zero LiCl, the water activity is set to the pure-water
value of one.
