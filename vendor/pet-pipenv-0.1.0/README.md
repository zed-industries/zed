# Pipenv

## Notes

- Where are pipenv envionments located?
  - Pipenv environments are generally located in the `WORKON_HOME`, `~/.venvs` directory.
  - Locations are computed from the `WORKON_HOME` environment variable.
  - However there are other commonly known locations, found in https://github.com/pypa/pipenv/blob/main/pipenv/utils/shell.py#L184
  - And then there are other locations found in documentation on the pipenv https://pipenv.pypa.io/en/latest/virtualenv.html
  - Note: Its possible that `WORKON_HOME` variable is not accessible for whatever reason (e.g. its setup in a shell script that is not sourced). Hence its safer to look for all known locations.
- `.project`
  - A Python environment is `pipenv` enviornment if:
    - A Pytohn envioronment contains a `.project` (this contains the path to the project that the environment is associated with.)
    - The project directory contains a `Pipfile`
- Version
  - Follow the symlink of the Python file and identify the Pthon install location
    - Extract the version of Python from the `patchlevel.h` file from the entry `#define PY_VERSION`
    - These files are located in `<sys prefix>/include/patchlevel.h` or `<sys prefix>/Headers/patchlevel.h`
  - On Windows => Extract the version from `pyvenv.cfg` if the python exe and the `pyvenv.cfg` file were created at the same time (max 60s difference).
    - Its possible for the Python environment used to create virtual envs to change.
    - E.g. assume we create a virtual env using Python 3.9, now `pyvenv.cfg` would contain the version 3.9.0.
    - Now assume we upgrade Python 3.9 to Python 3.10, now the `pyvenv.cfg` file still contains 3.9.0.
    - However the python executable in the virtual env would point to the same original path and now we have pyhon 3.10 exe there.
- Find is not implemented for this locator
  - This is because all environments are located in known locations, hence there is no need to search for them.
  - For each environment the callers will invoke the `try_from` method to see if the environment is a `pipenv` environment.
