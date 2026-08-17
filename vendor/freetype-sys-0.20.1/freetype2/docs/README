After saying `make refdoc' or `make refdoc-venv' the `reference/' directory
contains the FreeType API reference.  You need Python >= 3.5 and pip to make
this target.

There are two ways to generate the documentation:

1. Using `make refdoc':

    - Ensure `python' and `pip' are available.
    - Install pip package `docwriter' with `pip install --user docwriter'.
    - Make target with `make refdoc'.
    - This target can be run offline once required packages are installed.

2. Using `make refdoc-venv' (requires internet access):

    - Ensure `python', `pip' and Python package `virtualenv' are available.
    - Make target with `make refdoc-venv'.
    - This may or may not require internet access every time depending on
    pip and system caching.

Some troubleshooting tips:

* Regularly run `pip install --upgrade docwriter' to check for updates which
may include bug fixes.

* `Docwriter' does not support Python 2.  Ensure that Python >= 3.5 is
installed and available as `python3'/`python'.

* Ensure that `docwriter' is installed in the same Python target that
`make refdoc' uses (python3/python).

* If none of this works, send a mail to `freetype-devel@nongnu.org' or file
an issue at `https://github.com/freetype/docwriter/issues'.
