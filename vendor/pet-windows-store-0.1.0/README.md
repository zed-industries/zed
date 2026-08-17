# Windows Store

## Known Issues

- Note possible to get the `version` information, hence not returned (env will need to be resolved)
- If there are multiple versions of Windows Store Python installed,
  none of the environments returned will contain the exes `.../WindowsApps/python.exe` or `.../WindowsApps/python3.exe`.
  This is becase we will need to spawn both of these exes to figure out the env it belongs to.
  For now, we will avoid that.
  Upon resolving `.../WindowsApps/python.exe` or `.../WindowsApps/python3.exe` we will return the right information.

```rust
for directory under `<home>/AppData/Local/Microsoft/WindowsApps`:
    if directory does not start with `PythonSoftwareFoundation.Python.`:
        continue

    if `python.exe` does not exists in the directory:
        continue

    app_model_key = `HKCU/Software/Classes/Local Settings/Software/Microsoft/Windows/CurrentVersion/AppModel`;
    package_name = `<app_model_key>/SystemAppData/<directory name>/Schemas/(PackageFullName)`
    key = `<app_model_key>/Repository/Packages/<package_name>`
    env_path = `<key>/(PackageRootFolder)`
    display_name = `<key>/(DisplayName)`

    // Get the first 2 parts of the version from the path
    // directory = \AppData\Local\Microsoft\WindowsApps\PythonSoftwareFoundation.Python.3.9_qbz5n2kfra8p0\python.exe
    // In this case first 2 parts are `3.9`
    // Now look for a file named `python3.9.exe` in the `WindowsApps` directory (parent directory)
    // If it exists, then use that as a symlink as well
    // As a result that exe will have a shorter path, hence thats what users will see
    exe = `python.exe` or `pythonX.Y.exe`

    // No way to get the full version information.
    👍 track this environment
```

## Notes

### Why will `/WindowsApps/python3.exe` & `/WindowsApps/python.exe` will never be returned as preferred exes

Assume we have Pythoon 3.10 and Python 3.12 installed from Windows Store.
Now we'll have the following exes in the `WindowsApps` directory:

- `/WindowsApps/python3.10.exe`
- `/WindowsApps/python3.12.exe`
- `/WindowsApps/python3.exe`
- `/WindowsApps/python.exe`.

However we will not know what Python3.exe and Python.exe point to.
The only way to determine this is by running the exe and checking the version.
But that will slow discovery, hence we will not spawn those and never return them either during a regular discovery.

### `/WindowsApps/python3.exe` & `/WindowsApps/python.exe` can get returned as symlinks

If user has just Python 3.10 installed, then `/WindowsApps/python3.exe` & `/WindowsApps/python3.10.exe` will be returned as symlinks.

Similarly, if caller of the API attempts to resolve either one of the above exes, then we'll end up spawning the exe and we get the fully qualified path such as the following:

- `C:\\Program Files\\WindowsApps\\PythonSoftwareFoundation.Python.3.10_3.10.3056.0_x64__qbz5n2kfra8p0\\python.exe`.

From here we know the enviroment details, and the original exe will be returned as a symlink.
