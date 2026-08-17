# Virtualenvwrapper

## Notes

- They are regular Python environments created a specific location
- The location is defined in the `WORKON_HOME` environment variable.
- Else defaults to `~/.virtualenvs`
- They too have a have a `.project` file in the root of the environment
  This file contains the path to the project directory thats associated with this environment.
- They have a `.pyvenv.cfg` file in the root of the environment
  This file contains the version of Python used in this environment.
- A Python environment containing a `pyvenv.cfg` file is a virtual environment.
- Note: `poetry`, `pipenv`, `virtualenvwrapper`, etc all create virtual environments that have a `pyvenv.cfg` file.
  - Hence when determining whether an Environment is a `virtualenvwrapper` environment, we need to first if it is some other type like `poetry` or the like.
- Version
  - Follow the symlink of the Python file and identify the Pthon install location
    - Extract the version of Python from the `patchlevel.h` file from the entry `#define PY_VERSION`
    - These files are located in `<sys prefix>/include/patchlevel.h` or `<sys prefix>/Headers/patchlevel.h`
  - On Windows => Extract the version from `pyvenv.cfg` if the python exe and the `pyvenv.cfg` file were created at the same time (max 60s difference).
    - Its possible for the Python environment used to create virtual envs to change.
    - E.g. assume we create a virtual env using Python 3.9, now `pyvenv.cfg` would contain the version 3.9.0.
    - Now assume we upgrade Python 3.9 to Python 3.10, now the `pyvenv.cfg` file still contains 3.9.0.
    - However the python executable in the virtual env would point to the same original path and now we have pyhon 3.10 exe there.
