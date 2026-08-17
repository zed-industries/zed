# Python from Mac Command Line Tools

## Notes

- Look for Python in
  - /Library/Developer/CommandLineTools/usr/bin
  - /Library/Developer/CommandLineTools/Library/Frameworks/Python3.framework/Versions
- Sometimes, `/usr/bin/python3` can be a copy of Python for the above locations.
  - Unfortunately these are not symlinks, but we need to `spawn` the process to get the actual `sys.executable` information.
- `/Library/Developer/CommandLineTools/usr/bin/python?` are generally symlinks to the Python executable in `/Library/Developer/CommandLineTools/Library/Frameworks/Python3.framework/Versions/<version>/bin/python?`.
- Version
  - If we spawn such as `/usr/bin/python3`, then we have the version (for at least one of the Python versions).
  - Extract the version of Python from the `patchlevel.h` file from the entry `#define PY_VERSION`
  - These files are located in `<sys prefix>/include/patchlevel.h` or `<sys prefix>/Headers/patchlevel.h`

## Known Issues

- `/usr/bin/python3` can be a copy of Python for Python from one of the above locations.
  Unfortunately these are not symlinks, but we need to `spawn` the process to get the actual `sys.executable` information.
