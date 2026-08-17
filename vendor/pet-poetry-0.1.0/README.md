# Poetry

## Notes

- Where are poetry envionments located?
  - The paths were determined by inspecting the source for Poetry code.
- `config.toml` is the global Poetry config file
- `poetry.toml` is the local Poetry config/project file
- Using the `config.toml` and known global locations, enumerate all known Poetry Environments
  - Given a project directory (workspace folder), compute the hash for that directory
  - Look for an environment with the same hash.
- Given a project directory (workspace folder), determine the local poetry config (`poetry.toml`)
  - Check the setting `virtualenvs.in-project` and `POETRY_VIRTUALENVS_IN_PROJECT`
  - Based on the above value any existing `.venv` directory in the project directory will be treated as a Poetry environment.
- Version
  - Follow the symlink of the Python file and identify the Pthon install location
    - Extract the version of Python from the `patchlevel.h` file from the entry `#define PY_VERSION`
    - These files are located in `<sys prefix>/include/patchlevel.h` or `<sys prefix>/Headers/patchlevel.h`
  - On Windows => Extract the version from `pyvenv.cfg` if the python exe and the `pyvenv.cfg` file were created at the same time (max 60s difference).
    - Its possible for the Python environment used to create virtual envs to change.
    - E.g. assume we create a virtual env using Python 3.9, now `pyvenv.cfg` would contain the version 3.9.0.
    - Now assume we upgrade Python 3.9 to Python 3.10, now the `pyvenv.cfg` file still contains 3.9.0.
    - However the python executable in the virtual env would point to the same original path and now we have pyhon 3.10 exe there.
