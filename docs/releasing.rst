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

After confirming model validation and obtaining release approval, use the
GitHub Actions workflow for publishing to both registries.

Publishing to crates.io
-----------------------

Version ``0.1.0`` was published manually and cannot be uploaded again.
For later versions, sign in as a crate owner and configure a trusted publisher
under the ``desiccant-props`` settings on `crates.io
<https://crates.io/crates/desiccant-props/settings>`_:

* Repository: ``NatLabRockies/DesiccantProps``
* Workflow filename: ``pypi.yml``
* Environment: ``crates-io``

In the GitHub repository settings, create the ``crates-io`` environment and
restrict deployment to version tags (``v*``). Configure required reviewers if
available. GitHub authenticates using OIDC; no long-lived crates.io token is
needed. See the `crates.io trusted publishing announcement
<https://blog.rust-lang.org/2025/07/11/crates-io-development-update-2025-07/>`_.

Publishing to PyPI
------------------

Sign in to `PyPI <https://pypi.org/>`_ and add a `pending Trusted Publisher
<https://pypi.org/manage/account/publishing/>`_ if the project does not exist.
Use these values:

.. list-table::

   * - PyPI project name
     - ``desiccant-props``
   * - GitHub owner
     - ``NatLabRockies``
   * - Repository name
     - ``DesiccantProps``
   * - Workflow filename
     - ``pypi.yml``
   * - GitHub environment
     - ``pypi``

In the repository's settings, create the ``pypi`` environment and restrict
deployment to release tags (``v*``). Configure required reviewers if
available. GitHub authenticates to PyPI via OIDC; no PyPI API token is needed.

Push the workflow to ``main`` and verify the build-only CI run passes before
pushing the tag matching ``Cargo.toml`` (for example, ``v0.1.0``). The workflow
builds and tests wheels for CPython 3.10+ using the stable ABI on Linux
(manylinux 2.28), macOS, and Windows, for both x86-64 and ARM64, and tests the
source distribution, and runs the Rust checks. Only a pushed version tag can
publish, and only after all builds and tests pass. The crates.io publishing job
skips ``v0.1.0`` because it is already published; PyPI can still publish that
version. For a later release, update the version in ``Cargo.toml`` and
``Cargo.lock``, commit it, and push the matching tag.

The two registries publish independently: if one job fails after the other
succeeds, inspect the run and registry before retrying. Published versions
cannot be overwritten; publish the missing registry's artifacts from that same
tag, or use a new version for any changes.

Documentation hosting
---------------------

Build the Read the Docs site locally:

.. code-block:: sh

   python -m pip install -r docs/requirements.txt
   sphinx-build -W -b html docs docs/_build/html

Import the GitHub repository in Read the Docs; it picks up
``.readthedocs.yaml``. Sphinx does not need to import the native extension.
