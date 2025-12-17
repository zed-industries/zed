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
| `-n`, `--new` | Open in a new window instead of reusing an existing one |
| `-w`, `--wait` | Wait for the file to be closed before returning |
| `-a`, `--add` | Add files to the current workspace |
| `-` | Read from stdin |
| `--foreground` | Start Zed with logs output to the terminal |
| `--dev-server-token <TOKEN>` | Start as a dev server with the given token |
| `--uninstall` | Uninstall Zed and all related files |
| `-h`, `--help` | Show help information |
| `-v`, `--version` | Show version information |

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
