Building and releasing
======================

From the repository root, build and test the Rust crate:

.. code-block:: sh

   cargo fmt --check
   cargo test --locked
   cargo doc --locked --no-deps
   cargo package --locked

The default features do not require Python. docs.rs builds the public Rust
API automatically once the crate is published on crates.io.

Build a Python wheel and sdist using maturin. The package version comes
from ``Cargo.toml``. Use a supported CPython 3.10+ interpreter.

.. code-block:: sh

   python -m pip install maturin
   maturin build --release --locked
   maturin sdist

Install the wheel in a clean environment and run
``python -m unittest discover -s python/tests``. Before publishing, build
and test wheels on each advertised platform (use a manylinux build for
Linux). Validate the resulting distributions and configure your registry
credentials or trusted publisher separately. No publication is performed
by the build commands above.

After confirming the model validation and obtaining release approval, use
``cargo publish --locked`` for crates.io and upload the verified wheel and
sdist artifacts to PyPI (for example with ``twine upload``). Publish only
artifacts built from the intended release revision.

Build the Read the Docs site locally:

.. code-block:: sh

   python -m pip install -r docs/requirements.txt
   sphinx-build -W -b html docs docs/_build/html

Import the GitHub repository in Read the Docs; it picks up
``.readthedocs.yaml``. Sphinx does not need to import the native extension.
