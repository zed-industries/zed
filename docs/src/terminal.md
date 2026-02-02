# Terminal

Zed includes a built-in terminal emulator that supports multiple terminal instances, custom shells, and deep integration with the editor.

## Opening Terminals

| Action                  | macOS           | Linux/Windows   |
| ----------------------- | --------------- | --------------- |
| Toggle terminal panel   | `` Ctrl+` ``    | `` Ctrl+` ``    |
| Open new terminal       | `Ctrl+~`        | `Ctrl+~`        |
| Open terminal in center | Command palette | Command palette |

You can also open a terminal from the command palette with `terminal panel: toggle` or `workspace: new terminal`.

### Terminal Panel vs Center Terminal

Terminals can open in two locations:

- **Terminal Panel** — Docked at the bottom (default), left, or right of the workspace. Toggle with `` Ctrl+` ``.
- **Center Pane** — Opens as a regular tab alongside your files. Use `workspace: new center terminal` from the command palette.

## Working with Multiple Terminals

Create additional terminals with `Cmd+N` (macOS) or `Ctrl+N` (Linux/Windows) while focused in the terminal panel. Each terminal appears as a tab in the panel.

Split terminals horizontally with `Cmd+D` (macOS) or `Ctrl+Shift+5` (Linux/Windows).

## Configuring the Shell

By default, Zed uses your system's default shell (from `/etc/passwd` on Unix systems). To use a different shell:

```json [settings]
{
  "terminal": {
    "shell": {
      "program": "/bin/zsh"
    }
  }
}
```

To pass arguments to your shell:

```json [settings]
{
  "terminal": {
    "shell": {
      "with_arguments": {
        "program": "/bin/bash",
        "args": ["--login"]
      }
    }
  }
}
```

## Working Directory

Control where new terminals start:

| Value                                         | Behavior                                                                                                          |
| --------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| `"current_file_directory"`                    | Uses the current file's directory, falling back to the project directory, then the first project in the workspace |
| `"current_project_directory"`                 | Uses the current file's project directory (default)                                                               |
| `"first_project_directory"`                   | Uses the first project in your workspace                                                                          |
| `"always_home"`                               | Always starts in your home directory                                                                              |
| `{ "always": { "directory": "~/projects" } }` | Always starts in a specific directory                                                                             |

```json [settings]
{
  "terminal": {
    "working_directory": "first_project_directory"
  }
}
```

## Environment Variables

Add environment variables to all terminal sessions:

```json [settings]
{
  "terminal": {
    "env": {
      "EDITOR": "zed --wait",
      "MY_VAR": "value"
    }
  }
}
```

> **Tip:** Use `:` to separate multiple values in a single variable: `"PATH": "/custom/path:$PATH"`

### Python Virtual Environment Detection

Zed can automatically activate Python virtual environments when opening a terminal. By default, it searches for `.env`, `env`, `.venv`, and `venv` directories:

```json [settings]
{
  "terminal": {
    "detect_venv": {
      "on": {
        "directories": [".venv", "venv"],
        "activate_script": "default"
      }
    }
  }
}
```

The `activate_script` option supports `"default"`, `"csh"`, `"fish"`, and `"nushell"`.

To disable virtual environment detection:

```json [settings]
{
  "terminal": {
    "detect_venv": "off"
  }
}
```

## Fonts and Appearance

The terminal can use different fonts from the editor:

```json [settings]
{
  "terminal": {
    "font_family": "JetBrains Mono",
    "font_size": 14,
    "font_features": {
      "calt": false
    },
    "line_height": "comfortable"
  }
}
```

Line height options:

- `"comfortable"` — 1.618 ratio, good for reading (default)
- `"standard"` — 1.3 ratio, better for TUI applications with box-drawing characters
- `{ "custom": 1.5 }` — Custom ratio

### Cursor

Configure cursor appearance:

```json [settings]
{
  "terminal": {
    "cursor_shape": "bar",
    "blinking": "on"
  }
}
```

Cursor shapes: `"block"`, `"bar"`, `"underline"`, `"hollow"`

Blinking options: `"off"`, `"terminal_controlled"` (default), `"on"`

### Minimum Contrast

Zed adjusts terminal colors to maintain readability. The default value of `45` ensures text remains visible. Set to `0` to disable contrast adjustment and use exact theme colors:

```json [settings]
{
  "terminal": {
    "minimum_contrast": 0
  }
}
```

## Scrolling

Navigate terminal history with these keybindings:

| Action           | macOS                          | Linux/Windows    |
| ---------------- | ------------------------------ | ---------------- |
| Scroll page up   | `Shift+PageUp` or `Cmd+Up`     | `Shift+PageUp`   |
| Scroll page down | `Shift+PageDown` or `Cmd+Down` | `Shift+PageDown` |
| Scroll line up   | `Shift+Up`                     | `Shift+Up`       |
| Scroll line down | `Shift+Down`                   | `Shift+Down`     |
| Scroll to top    | `Shift+Home` or `Cmd+Home`     | `Shift+Home`     |
| Scroll to bottom | `Shift+End` or `Cmd+End`       | `Shift+End`      |

Adjust scroll speed with:

```json [settings]
{
  "terminal": {
    "scroll_multiplier": 3.0
  }
}
```

## Copy and Paste

| Action | macOS   | Linux/Windows  |
| ------ | ------- | -------------- |
| Copy   | `Cmd+C` | `Ctrl+Shift+C` |
| Paste  | `Cmd+V` | `Ctrl+Shift+V` |

### Copy on Select

Automatically copy selected text to the clipboard:

```json [settings]
{
  "terminal": {
    "copy_on_select": true
  }
}
```

### Keep Selection After Copy

By default, text stays selected after copying. To clear the selection:

```json [settings]
{
  "terminal": {
    "keep_selection_on_copy": false
  }
}
```

## Search

Search terminal content with `Cmd+F` (macOS) or `Ctrl+Shift+F` (Linux/Windows). This opens the same search bar used in the editor.

## Vi Mode

Toggle vi-style navigation in the terminal with `Ctrl+Shift+Space`. This allows you to navigate and select text using vi keybindings.

## Clear Terminal

Clear the terminal screen:

- macOS: `Cmd+K`
- Linux/Windows: `Ctrl+Shift+L`

## Option as Meta (macOS)

For Emacs users or applications that use Meta key combinations, enable Option as Meta:

```json [settings]
{
  "terminal": {
    "option_as_meta": true
  }
}
```

This reinterprets the Option key as Meta, allowing sequences like `Alt+X` to work correctly.

## Alternate Scroll Mode

When enabled, mouse scroll events are converted to arrow key presses in applications like `vim` or `less`:

```json [settings]
{
  "terminal": {
    "alternate_scroll": "on"
  }
}
```

## Path Hyperlinks

Zed detects file paths in terminal output and makes them clickable. `Cmd+Click` (macOS) or `Ctrl+Click` (Linux/Windows) opens the file in Zed, jumping to the line number if one is detected.

Common formats recognized:

- `src/main.rs:42` — Opens at line 42
- `src/main.rs:42:10` — Opens at line 42, column 10
- `File "script.py", line 10` — Python tracebacks

## Panel Configuration

### Dock Position

```json [settings]
{
  "terminal": {
    "dock": "bottom"
  }
}
```

Options: `"bottom"` (default), `"left"`, `"right"`

### Default Size

```json [settings]
{
  "terminal": {
    "default_width": 640,
    "default_height": 320
  }
}
```

### Terminal Button

Hide the terminal button in the status bar:

```json [settings]
{
  "terminal": {
    "button": false
  }
}
```

### Toolbar

Show the terminal title in a breadcrumb toolbar:

```json [settings]
{
  "terminal": {
    "toolbar": {
      "breadcrumbs": true
    }
  }
}
```

The title can be set by your shell using the escape sequence `\e]2;Title\007`.

## Integration with Tasks

The terminal integrates with Zed's [task system](./tasks.md). When you run a task, it executes in the terminal. Rerun the last task from a terminal with:

- macOS: `Cmd+Alt+R`
- Linux/Windows: `Ctrl+Shift+R` or `Alt+T`

## AI Assistance

Get help with terminal commands using inline assist:

- macOS: `Ctrl+Enter`
- Linux/Windows: `Ctrl+Enter` or `Ctrl+I`

This opens the AI assistant to help explain errors, suggest commands, or troubleshoot issues.

## Sending Text and Keystrokes

For advanced keybinding customization, you can send raw text or keystrokes to the terminal:

```json [keymap]
{
  "context": "Terminal",
  "bindings": {
    "alt-left": ["terminal::SendText", "\u001bb"],
    "ctrl-c": ["terminal::SendKeystroke", "ctrl-c"]
  }
}
```

## All Terminal Settings

For the complete list of terminal settings, see the [Terminal section in All Settings](./reference/all-settings.md#terminal).

## What's Next

- [Tasks](./tasks.md) — Run commands and scripts from Zed
- [REPL](./repl.md) — Interactive code execution
- [CLI Reference](./reference/cli.md) — Command-line interface for opening files in Zed
