# Pyenv

## Notes

- Where is pyenv located?
  - For windows its generally `~/.pyenv/pyenv-win` else `~/.pyenv`
- `versions` sub directory for `pyenv` contains all Python versions installed using pyenv
- On windows, if the directory ends with `-win32`, then its a 32bit Windows Python installation.
- Version
  - Extract the version of Python from the `patchlevel.h` file from the entry `#define PY_VERSION`
  - These files are located in `<sys prefix>/include/patchlevel.h` or `<sys prefix>/Headers/patchlevel.h`
