# Python from Python.org on Mac

## Notes

- Look for Python in
  - /Library/Frameworks/Python.framework/Versions/
- Sometimes, `/usr/local/bin/python?` can be a symlink to Python in one of the above locations.
  - Why `sometimes`, thats because any other installation can end up overwrite the symlink in `/usr/local/bin/python?` with something else.
- `/Library/Frameworks/Python.framework/Versions/Current/bin/python?` can be a symlink to Python in one of the above locations.
- Version
  - Extract the version of Python from the `patchlevel.h` file from the entry `#define PY_VERSION`
  - These files are located in `<sys prefix>/include/patchlevel.h` or `<sys prefix>/Headers/patchlevel.h`
