# Installing Extensions

You can search for extensions using the extensions page. On macOS, you can find the extensions page in the application menu under `Zed > Extensions`. You can also open it from the command palette with the `zed: extension` command.

On this page, you can view the extensions that you currently have installed, or search for new ones.

## Installation Location

- On macOS, extensions are installed in `~/Library/Application Support/Zed/extensions`.
- On Linux, they are installed in either `$XDG_DATA_HOME/zed/extensions` or `~/.local/share/zed/extensions`.

This directory contains two subdirectories:

- `installed`, which contains the source code for each extension.
- `work` which contains files created by the extension itself, such as downloaded language servers.
