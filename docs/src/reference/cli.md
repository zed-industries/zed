# CLI Reference

Zed includes a command-line interface (CLI) for opening files and directories, integrating with other tools, and controlling Zed from scripts.

## Installation

**macOS:** Run the `cli: install` command from the command palette ({#kb command_palette::Toggle}) to install the `zed` CLI to `/usr/local/bin/zed`.

**Linux:** The CLI is included with Zed packages. The binary name may vary by distribution (commonly `zed` or `zeditor`).

**Windows:** The CLI is included with Zed. Add Zed's installation directory to your PATH, or use the full path to `zed.exe`.

## Usage

```sh
zed [OPTIONS] [PATHS]...
```

## Opening Files and Directories

Open a file:

```sh
zed myfile.txt
```

Open a directory as a workspace:

```sh
zed ~/projects/myproject
```

Open multiple files or directories:

```sh
zed file1.txt file2.txt ~/projects/myproject
```

Open a file at a specific line and column:

```sh
zed myfile.txt:42        # Open at line 42
zed myfile.txt:42:10     # Open at line 42, column 10
```

## Options

### `-w`, `--wait`

Wait for all opened files to be closed before the CLI exits. When opening a directory, waits until the window is closed.

This is useful for integrating Zed with tools that expect an editor to block until editing is complete (e.g., `git commit`):

```sh
export EDITOR="zed --wait"
git commit  # Opens Zed and waits for you to close the commit message file
```

### `-n`, `--new`

Open paths in a new workspace window, even if the paths are already open in an existing window:

```sh
zed -n ~/projects/myproject
```

### `-a`, `--add`

Add paths to the currently focused workspace instead of opening a new window:

```sh
zed -a newfile.txt
```

### `-r`, `--reuse`

Reuse an existing window, replacing its current workspace with the new paths:

```sh
zed -r ~/projects/different-project
```

### `--diff <OLD_PATH> <NEW_PATH>`

Open a diff view comparing two files. Can be specified multiple times:

```sh
zed --diff file1.txt file2.txt
zed --diff old.rs new.rs --diff old2.rs new2.rs
```

### `--foreground`

Run Zed in the foreground, keeping the terminal attached. Useful for debugging:

```sh
zed --foreground
```

### `--user-data-dir <DIR>`

Use a custom directory for all user data (database, extensions, logs) instead of the default location:

```sh
zed --user-data-dir ~/.zed-custom
```

Default locations:

- **macOS:** `~/Library/Application Support/Zed`
- **Linux:** `$XDG_DATA_HOME/zed` (typically `~/.local/share/zed`)
- **Windows:** `%LOCALAPPDATA%\Zed`

### `-v`, `--version`

Print Zed's version and exit:

```sh
zed --version
```

### `--uninstall`

Uninstall Zed and remove all related files (macOS and Linux only):

```sh
zed --uninstall
```

### `--zed <PATH>`

Specify a custom path to the Zed application or binary:

```sh
zed --zed /path/to/Zed.app myfile.txt
```

## Reading from Standard Input

Read content from stdin by passing `-` as the path:

```sh
echo "Hello, World!" | zed -
cat myfile.txt | zed -
ps aux | zed -
```

This creates a temporary file with the stdin content and opens it in Zed.

## URL Handling

The CLI can open `zed://`, `http://`, and `https://` URLs:

```sh
zed zed://settings
zed https://github.com/zed-industries/zed
```

## Using Zed as Your Default Editor

Set Zed as your default editor for Git and other tools:

```sh
export EDITOR="zed --wait"
export VISUAL="zed --wait"
```

Add these lines to your shell configuration file (e.g., `~/.bashrc`, `~/.zshrc`).

## macOS: Switching Release Channels

On macOS, you can launch a specific release channel by passing the channel name as the first argument:

```sh
zed --stable myfile.txt
zed --preview myfile.txt
zed --nightly myfile.txt
```

## WSL Integration (Windows)

On Windows, the CLI supports opening paths from WSL distributions. This is handled automatically when launching Zed from within WSL.

## Exit Codes

| Code | Meaning                           |
| ---- | --------------------------------- |
| `0`  | Success                           |
| `1`  | Error (details printed to stderr) |

When using `--wait`, the exit code reflects whether the files were saved before closing.

## Terminal Subcommands

The `zed terminal` subcommands enable programmatic control of Zed's integrated terminals, allowing external tools, scripts, and AI agents to create, manage, and interact with terminal sessions.

> **Security:** This feature is opt-in and disabled by default. Enable it by adding `"terminal": { "cli_enabled": true }` to your Zed settings (Settings → Open Settings or {#kb zed::OpenSettings}). When enabled, any process running as your user can control terminals—this grants the same access you already have but through a programmatic interface.

### Environment Variables

Each terminal receives an environment variable that scripts can use for self-reference:

- `ZED_TERM_ID` — The entity ID of the terminal (session-scoped, not persistent across Zed restarts)

### Commands

All terminal commands use the format: `zed terminal <command> [options]`

#### `zed terminal create`

Create a new terminal in the active workspace.

```sh
zed terminal create [options]
```

| Option | Description |
|--------|-------------|
| `--cwd <path>` | Working directory for the terminal |
| `--command <cmd>` | Command to run instead of default shell |
| `--args <args>...` | Arguments for the command (use `--` before args starting with `-`) |
| `--env <KEY=VALUE>...` | Environment variables to set |
| `--title <title>` | Title override for the terminal tab |
| `--in-pane-of <id>` | Create as a tab in the pane containing terminal with this ID |
| `--no-activate` | Create as a background tab (don't focus) |

Examples:

```sh
# Create a terminal in a specific directory
zed terminal create --cwd /path/to/project

# Run a specific command
zed terminal create --command python --args script.py

# Create with custom title and environment
zed terminal create --title "Build Server" --env PORT=3000
```

#### `zed terminal send`

Send text input to a terminal.

```sh
zed terminal send <id> <text>
```

#### `zed terminal key`

Send a keystroke to a terminal. Uses Zed's keybinding format (e.g., `enter`, `ctrl-c`, `alt-x`).

```sh
zed terminal key <id> <key>
```

Examples:

```sh
zed terminal key 12345 enter
zed terminal key 12345 ctrl-c
```

#### `zed terminal read`

Read the current screen content of a terminal. Returns JSON with the visible buffer content.

```sh
zed terminal read <id>
```

#### `zed terminal list`

List all terminals with their entity IDs and metadata.

```sh
zed terminal list
```

Returns JSON with a `workspaces` array containing terminal information. The terminal you're running from (if any) is marked with `you_are_here: true`.

#### `zed terminal cwd`

Get the current working directory of a terminal.

```sh
zed terminal cwd <id>
```

#### `zed terminal idle`

Check if a terminal is idle (no running foreground process).

```sh
zed terminal idle <id>
```

Returns JSON with `idle: true/false`.

#### `zed terminal close`

Close a terminal.

```sh
zed terminal close <id>
```

#### `zed terminal split`

Split a terminal pane in a given direction, creating a new terminal.

```sh
zed terminal split <id> --direction <direction>
```

Directions: `up`, `down`, `left`, `right` (default: `right`)

#### `zed terminal layout`

Get the terminal panel layout tree or reorganize terminals.

```sh
# Get current layout (returns JSON)
zed terminal layout

# Reorganize into a layout mode
zed terminal layout --tile-vertical    # Side-by-side columns
zed terminal layout --tile-horizontal  # Stacked rows
zed terminal layout --consolidate      # All in one pane as tabs
```

#### `zed terminal focus`

Focus a specific terminal.

```sh
zed terminal focus <id>
```

#### `zed terminal title`

Set or clear the title override for a terminal.

```sh
zed terminal title <id> --set <title>  # Set title
zed terminal title <id>                 # Clear title override
```

#### `zed terminal move`

Move a terminal to another pane.

```sh
zed terminal move <id> --to-pane-of <other-id>
```
