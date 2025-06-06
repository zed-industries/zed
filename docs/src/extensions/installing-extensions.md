# Installing Extensions

You can search for extensions by launching the CodeOrbit Extension Gallery by pressing `cmd-shift-x` (macOS) or `ctrl-shift-x` (Linux), opening the command palette and selecting `CodeOrbit: extensions` or by selecting "CodeOrbit > Extensions" from the menu bar.

Here you can view the extensions that you currently have installed or search and install new ones.

## Installation Location

- On macOS, extensions are installed in `~/Library/Application Support/CodeOrbit/extensions`.
- On Linux, they are installed in either `$XDG_DATA_HOME/CodeOrbit/extensions` or `~/.local/share/CodeOrbit/extensions`.

This directory contains two subdirectories:

- `installed`, which contains the source code for each extension.
- `work` which contains files created by the extension itself, such as downloaded language servers.

## Auto installing

To automate extension installation/uninstallation see the docs for [auto_install_extensions](../configuring-CodeOrbit.md#auto-install-extensions).
