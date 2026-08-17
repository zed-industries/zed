# WinPython Locator

This crate provides support for detecting [WinPython](https://winpython.github.io/) environments.

## Detection Strategy

WinPython environments are identified by looking for:

1. **Marker files**: `.winpython` or `winpython.ini` file in parent directories
2. **Directory naming pattern**: Parent directory matching patterns like `WPy64-*`, `WPy32-*`, or `WPy-*`
3. **Python folder naming**: The Python installation folder typically follows the pattern `python-X.Y.Z.amd64` or `python-X.Y.Z`

## Typical WinPython Directory Structure

```
WPy64-31300/                          # Top-level WinPython directory
├── .winpython                        # Marker file (may also be winpython.ini)
├── python-3.13.0.amd64/              # Python installation
│   ├── python.exe
│   ├── pythonw.exe
│   ├── Scripts/
│   └── Lib/
├── scripts/                          # WinPython-specific scripts
│   ├── env.bat
│   └── WinPython Command Prompt.exe
├── settings/                         # Settings directory
└── notebooks/                        # Optional Jupyter notebooks
```

## Platform Support

This locator only works on Windows, as WinPython is a Windows-only distribution.

## Search Paths

By default, the locator only looks for WinPython installations under
`%USERPROFILE%\WinPython`. Earlier versions also scanned drive roots
(`C:\`, `D:\`, `E:\`), `Program Files`, `Downloads`, `Desktop`, and
`Documents` on every refresh — those scans were a Windows Defender
hot-spot and inflated refresh latency, so they have been removed.

If your WinPython installation lives elsewhere, point the locator at it
with the `WINPYTHON_HOME` environment variable. The value can be a
single path or a `;`-separated list of paths. Each entry can either:

- _be_ a WinPython installation (e.g. `D:\WPy64-31300`), or
- _contain_ one or more WinPython installations (e.g. `D:\python-tools`
  with `D:\python-tools\WPy64-31300` inside).

Examples:

```
set WINPYTHON_HOME=D:\WPy64-31300
set WINPYTHON_HOME=D:\WPy64-31300;E:\portable-python
```
