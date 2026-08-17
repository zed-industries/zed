# Python from Mac XCode

## Notes

- Look for Python in such as:
  - /Applications/Xcode.app/Contents/Developer/usr/bin/python3
  - /Applications/Xcode_15.0.1.app/Contents/Developer/usr/bin/python3 (such paths are on CI, see here https://github.com/microsoft/python-environment-tools/issues/38)
- Sometimes, `/usr/bin/python3` can be a copy of Python for the above locations.
  - Unfortunately these are not symlinks, but we need to `spawn` the process to get the actual `sys.executable` information.
- Version
  - If we spawn such as `/usr/bin/python3`, then we have the version (for at least one of the Python versions).
  - Extract the version of Python from the `patchlevel.h` file from the entry `#define PY_VERSION`
  - These files are located in `<sys prefix>/include/patchlevel.h` or `<sys prefix>/Headers/patchlevel.h`

## Known Issues

- `/usr/bin/python3` can be a copy of Python for Python from one of the above locations.
  Unfortunately these are not symlinks, but we need to `spawn` the process to get the actual `sys.executable` information.
- `find` will never return python installed in XCode.
  - The assumption is that if users are using Python installed in XCode, then it will be in `/usr/bin/python3`.
  - I.e. its very unlikely users will be using Python by looking for it in `/Applications/Xcode.app/Contents/Developer/usr/bin/python3`.
  - If this is not true, then we can always search for Python in such directories and list them.
