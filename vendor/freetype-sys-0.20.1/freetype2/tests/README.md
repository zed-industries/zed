# Unit and regression tests for the FreeType library

## Quick Start

### Download test fonts

Run the `tests/scripts/download-fonts.py` script, which will
download test fonts to the `tests/data/` directory first.

### Build the test programs

The tests are only built with the Meson build system, and
are disabled by default, enable the 'tests' option to compile
them, as in:

  meson setup out -Dtests=enabled
  meson compile -C out

### Run the test programs

  meson test -C out

