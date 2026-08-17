# Virtualenv

## Notes

- A Python environment containing a activation scripts such as `/bin/activate` or `Scripts/activate.bat` or the like is a virtual environment.
- Note: `poetry`, `pipenv`, `virtualenvwrapper`, etc all create virtual environments that have a `pyvenv.cfg` file.
  - Hence when determining whether an Environment is a `virtualenv` environment, we need to first if it is some other type like `poetry` or the like.
- Similarly an environment containing `pyvenv.cfg` is generally not considered a `virtualenv`, as it could be a `venv`, `poetry`, `pipenv`, etc.
- Version
  - Follow the symlink of the Python file and identify the Pthon install location
    - Extract the version of Python from the `patchlevel.h` file from the entry `#define PY_VERSION`
    - These files are located in `<sys prefix>/include/patchlevel.h` or `<sys prefix>/Headers/patchlevel.h`
  - On Windows => Extract the version from `pyvenv.cfg` if the python exe and the `pyvenv.cfg` file were created at the same time (max 60s difference).
    - Its possible for the Python environment used to create virtual envs to change.
    - E.g. assume we create a virtual env using Python 3.9, now `pyvenv.cfg` would contain the version 3.9.0.
    - Now assume we upgrade Python 3.9 to Python 3.10, now the `pyvenv.cfg` file still contains 3.9.0.
    - However the python executable in the virtual env would point to the same original path and now we have pyhon 3.10 exe there.
