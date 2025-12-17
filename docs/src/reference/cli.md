# CLI Reference

Zed includes a command-line interface for opening files, directories, and controlling the editor from your terminal.

## Installation

- **macOS**: Run `cli: install` from the command palette to create a symlink at `/usr/local/bin/zed`
- **Linux**: The CLI is included with your distribution's Zed package (binary name may vary)

## Usage

```sh
zed [OPTIONS] [PATHS...]
```

## Common Commands

| Command | Description |
|---------|-------------|
| `zed` | Open an empty Zed window |
| `zed .` | Open the current directory as a project |
| `zed /path/to/file` | Open a specific file |
| `zed /path/to/folder` | Open a folder as a project |
| `zed file1.rs file2.rs` | Open multiple files |
| `zed -n /path` | Open in a new window |
| `ps aux \| zed -` | Read from stdin |

## Options

| Option | Description |
|--------|-------------|
| `-w`, `--wait` | Wait for all given paths to be opened/closed before exiting |
| `-a`, `--add` | Add files to the currently open workspace |
| `-n`, `--new` | Create a new workspace |
| `-r`, `--reuse` | Reuse an existing window, replacing its workspace |
| `-` | Read from stdin |
| `--foreground` | Run Zed in the foreground (useful for debugging, shows all logs) |
| `--zed <PATH>` | Custom path to Zed.app or the zed binary |
| `--dev-server-token <TOKEN>` | Run Zed in dev-server mode with the given token |
| `--diff <OLD_PATH> <NEW_PATH>` | Open a diff view comparing two files (can be used multiple times) |
| `--user-data-dir <DIR>` | Set a custom directory for all user data (database, extensions, logs) |
| `--uninstall` | Uninstall Zed and all related files (Linux/macOS only) |
| `--version` | Print Zed's version and the app path |

## Opening Files at a Specific Line

You can open a file at a specific line and column:

```sh
zed /path/to/file:42      # Open at line 42
zed /path/to/file:42:10   # Open at line 42, column 10
```

## Examples

Open the current directory in Zed:

```sh
zed .
```

Open a file and wait for it to close (useful for Git commit messages):

```sh
GIT_EDITOR="zed -w" git commit
```

Pipe command output into Zed:

```sh
cat /var/log/system.log | zed -
```

Open multiple files in a new window:

```sh
zed -n src/main.rs src/lib.rs
```

Compare two files:

```sh
zed --diff old_version.rs new_version.rs
```

Add files to an existing workspace:

```sh
zed -a additional_file.rs
```
