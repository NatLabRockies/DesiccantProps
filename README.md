# DesiccantProps

[![CI: build, tests and formatting](https://github.com/NatLabRockies/DesiccantProps/actions/workflows/pypi.yml/badge.svg?branch=main)](https://github.com/NatLabRockies/DesiccantProps/actions/workflows/pypi.yml)
[![PyPI](https://img.shields.io/pypi/v/desiccant-props)](https://pypi.org/project/desiccant-props/)
[![crates.io](https://img.shields.io/crates/v/desiccant-props)](https://crates.io/crates/desiccant-props)

Thermophysical properties of aqueous potassium acetate (KAc) and lithium
chloride (LiCl) for HVAC simulation. The Python package wraps the same Rust
implementation.

## Installation

For Rust, add the crate to your project:

```sh
cargo add desiccant-props
```

For Python 3.10 or later, install from PyPI once the package is published:

```sh
python -m pip install desiccant-props
```

Before the PyPI release, you can install from a local checkout with Rust
installed:

```sh
python -m pip install .
```

## Usage

```rust
use desiccant_props::Solution;

let kac = Solution::potassium_acetate(0.50)?;
let pressure_pa = kac.vapor_pressure(313.15)?;
# Ok::<(), desiccant_props::Error>(())
```

```python
from desiccant_props import Solution

kac = Solution.potassium_acetate(0.50)
pressure_pa = kac.vapor_pressure(313.15)
```

Temperature is in kelvin, composition is solute mass fraction, and outputs
are in SI units. The API returns errors for inputs outside its supported
domain. Sensible enthalpy is referenced to 273.15 K at **each** composition;
it is not a solution enthalpy model for cross-composition energy balances.

See the [user guide](https://desiccantprops.readthedocs.io/), [Rust API](https://docs.rs/desiccant-props),
and [local build and release instructions](docs/releasing.rst). BSD-3-Clause.

## Citation

Please cite the paper for the correlation you use:

- **Potassium acetate (KAc):** Pokharel, K., Foster, K. E. O., Rippy, K.,
  Votava, J., Bailey, T., Nixon, S., and Kozubal, E. (2025).
  “Material properties of aqueous potassium acetate solutions for use in
  liquid desiccant air conditioning.” *Progress in Engineering Science*,
  2(4), 100149. [doi:10.1016/j.pes.2025.100149](https://doi.org/10.1016/j.pes.2025.100149)
- **Lithium chloride (LiCl):** Conde, M. R. (2004). “Properties of aqueous
  solutions of lithium and calcium chlorides: formulations for use in air
  conditioning equipment design.” *International Journal of Thermal Sciences*,
  43(4), 367–382. [doi:10.1016/j.ijthermalsci.2003.09.003](https://doi.org/10.1016/j.ijthermalsci.2003.09.003)
