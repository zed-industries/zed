# Windows Registry

## Notes

- Looks for all installations under `HKLM/Software/Python` & `HKCU/Software/Python`
- The registry contains information about the Python installations (prefix, version, display name, etc)
- If a conda installation if found, pass that directory to the conda locator to get all conda environments.

```rust
for company of [PythonCore, ContinuumAnalytics]:
    for key in [HKLM, HKCU]:
        for installed_version in `<key>/Software/Python/<company>`
            // installed_version are values like 3.12, 3.10, 3.9, etc
            install_key = `<key>/Software/Python/<company>/<installed_version>InstallPath`
            env_path = `install_key/(Default)`
            exe = `install_key/(ExecutablePath)`

            if this is a conda install:
                Pass this directory to the conda locator to get all conda environments.
                continue

            if `exe` exists on disc:
                version = `install_key/(Version)` // SysVersion contains only first 2 parts of version
                display_name = `install_key/(DisplayName)`
                👍 track this environment

```
