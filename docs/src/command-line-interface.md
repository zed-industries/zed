# Command-line Interface

Zed has a CLI, on Linux this should come with the distribution's Zed package (binary name can vary from distribution to distribution, `zed` will be used later for brevity).
For macOS, the CLI comes in the same package with the editor binary, and could be installed into the system with the `cli: install` Zed command which will create a symlink to the `/usr/local/bin/zed`.
It can also be built from source out of the `cli` crate in this repository.

Use `zed --help` to see the full list of capabilities.
General highlights:

- Opening another empty Zed window: `zed`

- Opening a file or directory in Zed: `zed /path/to/entry` (use `-n` to open in the new window)

- Reading from stdin: `ps axf | zed -`

- Starting Zed with logs in the terminal: `zed --foreground`

- Uninstalling Zed and all its related files: `zed --uninstall`

## Terminal CLI

The Terminal CLI enables programmatic control of Zed's integrated terminals, allowing external tools, scripts, and AI agents to create, manage, and interact with terminal sessions.

> **Note:** This feature is opt-in and disabled by default. Enable it in Settings > Terminal > CLI, or add `"terminal": { "cli_enabled": true }` to your settings.

### Environment Variables

Each terminal receives the following environment variable:

- `ZED_TERM_ID` - The unique entity ID of the terminal, which can be used to reference it in CLI commands when enabled.

### Commands

All terminal commands use the format: `zed terminal <command> [options]`

#### create

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

**Examples:**
```sh
# Create a terminal in a specific directory
zed terminal create --cwd /path/to/project

# Run a specific command
zed terminal create --command python --args script.py

# Create with custom title and environment
zed terminal create --title "Build Server" --env PORT=3000
```

#### send

Send text input to a terminal.

```sh
zed terminal send <terminal> <text>
```

The `<terminal>` argument can be an entity ID or a terminal title.

**Examples:**
```sh
zed terminal send 12345 "npm run build"
zed terminal send "Build Server" "exit"
```

#### key

Send a special key to a terminal.

```sh
zed terminal key <terminal> <key>
```

Keys use the same format as Zed keybindings. Modifiers (`ctrl`, `alt`, `shift`) can be combined with `-`. Examples: `enter`, `ctrl-c`, `alt-f`, `ctrl-shift-up`.

**Examples:**
```sh
zed terminal key 12345 enter
zed terminal key 12345 ctrl-c
```

#### read

Read the current screen content of a terminal.

```sh
zed terminal read <terminal>
```

Returns JSON with `lines` (array of strings), `cursor_row`, and `cursor_col`.

#### list

List all terminals with their entity IDs and titles.

```sh
zed terminal list
```

Returns JSON with workspaces containing terminal arrays.

#### cwd

Get the current working directory of a terminal.

```sh
zed terminal cwd <terminal>
```

#### idle

Check if a terminal is idle (no running foreground process).

```sh
zed terminal idle <terminal>
```

Returns JSON with `idle: true/false`.

#### close

Close a terminal.

```sh
zed terminal close <terminal>
```

#### split

Split a terminal pane in a given direction.

```sh
zed terminal split <terminal> [--direction <direction>] [--title <title>]
```

Directions: `up`, `down`, `left`, `right` (default: `right`)

**Example:**
```sh
zed terminal split 12345 --direction right --title "Tests"
```

#### layout

Get the terminal panel layout tree or reorganize terminals.

```sh
# Get current layout (returns JSON)
zed terminal layout

# Reorganize into a layout mode
zed terminal layout --tile-vertical    # Side-by-side columns
zed terminal layout --tile-horizontal  # Stacked rows
zed terminal layout --consolidate      # All in one pane as tabs
```

#### focus

Focus a specific terminal.

```sh
zed terminal focus <terminal>
```

#### title

Set or clear the title override for a terminal.

```sh
zed terminal title <terminal> [--set <title>]
```

If `--set` is omitted, clears any title override.

#### move

Move a terminal to another pane.

```sh
zed terminal move <terminal> --to-pane-of <other-terminal>
```

### Example: Automated Build Pipeline

```sh
#!/bin/bash
# Create terminals for a development workflow

# Create build terminal
zed terminal create --title "Build" --cwd ~/project
BUILD_ID=$(zed terminal list | jq -r '.workspaces[].terminals[] | select(.title=="Build") | .entity_id')

# Create test terminal in same pane
zed terminal create --title "Tests" --in-pane-of "$BUILD_ID" --no-activate

# Split for logs
zed terminal split "$BUILD_ID" --direction right --title "Logs"

# Start build
zed terminal send "Build" "npm run watch"
zed terminal key "Build" enter
```
