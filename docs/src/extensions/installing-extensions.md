# Installing Extensions

You can search for extensions by launching the Zed Extension Gallery by pressing {#kb zed::Extensions} , opening the command palette and selecting {#action zed::Extensions} or by selecting "Zed > Extensions" from the menu bar.

Here you can view the extensions that you currently have installed or search and install new ones.

## Installation Location

- On macOS, extensions are installed in `~/Library/Application Support/Zed/extensions`.
- On Linux, they are installed in either `$XDG_DATA_HOME/zed/extensions` or `~/.local/share/zed/extensions`.
- On Windows, the directory is `%LOCALAPPDATA%\Zed\extensions`.

This directory contains two subdirectories:

- `installed`, which contains the source code for each extension.
- `work` which contains files created by the extension itself, such as downloaded language servers.

## Auto installing

To automate extension installation/uninstallation see the docs for [auto_install_extensions](../reference/all-settings.md#auto-install-extensions).
