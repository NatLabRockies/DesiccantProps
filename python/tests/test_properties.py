import unittest

from desiccant_props import Solution, lithium_chloride_solubility


class PropertyTests(unittest.TestCase):
    def test_kac_wheel_round_trip(self):
        solution = Solution.potassium_acetate(0.50)
        temperature = 313.15
        self.assertAlmostEqual(solution.vapor_pressure(temperature), 3803.6446632, places=5)
        enthalpy = solution.sensible_enthalpy(temperature)
        self.assertAlmostEqual(
            solution.temperature_from_sensible_enthalpy(enthalpy, 300, 330),
            temperature,
            places=7,
        )

    def test_licl_phase_limit_error(self):
        solution = Solution.lithium_chloride(0.42)
        self.assertGreater(lithium_chloride_solubility(313.15), solution.mass_fraction)
        with self.assertRaisesRegex(ValueError, "exceeds solubility"):
            solution.density(273.15)
