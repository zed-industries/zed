# Configuring Zed

## Folder-specific settings

Folder-specific settings are used to override Zed's global settings for files within a specific directory in the project panel. To get started, create a `.zed` subdirectory and add a `settings.json` within it. It should be noted that folder-specific settings don't need to live only a project's root, but can be defined at multiple levels in the project hierarchy. In setups like this, Zed will find the configuration nearest to the file you are working in and apply those settings to it. In most cases, this level of flexibility won't be needed and a single configuration for all files in a project is all that is required; the `Zed > Settings > Open Local Settings` menu action is built for this case. Running this action will look for a `.zed/settings.json` file at the root of the first top-level directory in your project panel. If it does not exist, it will create it.

The following global settings can be overridden with a folder-specific configuration:

- `copilot`
- `enable_language_server`
- `ensure_final_newline_on_save`
- `format_on_save`
- `formatter`
- `hard_tabs`
- `language_overrides`
- `preferred_line_length`
- `remove_trailing_whitespace_on_save`
- `soft_wrap`
- `tab_size`
- `show_copilot_suggestions`
- `show_whitespaces`

_See the Global settings section for details about these settings_

## Global settings

To get started with editing Zed's global settings, open `~/.config/zed/settings.json` via `⌘` + `,`, the command palette (`zed: open settings`), or the `Zed > Settings > Open Settings` application menu item.

Here are all the currently available settings.

## Active Pane Magnification

- Description: Scale by which to zoom the active pane. When set to `1.0`, the active pane has the same size as others, but when set to a larger value, the active pane takes up more space.
- Setting: `active_pane_magnification`
- Default: `1.0`

**Options**

`float` values

## Autosave

- Description: When to automatically save edited buffers.
- Setting: `autosave`
- Default: `off`

**Options**

1. To disable autosave, set it to `off`

```json
{
  "autosave": "off"
}
```

2. To autosave when focus changes, use `on_focus_change`:

```json
{
  "autosave": "on_focus_change"
}
```

3. To autosave when the active window changes, use `on_window_change`:

```json
{
  "autosave": "on_window_change"
}
```

4. To autosave after an inactivity period, use `after_delay`:

```json
{
  "autosave": {
    "after_delay": {
      "milliseconds": 1000
    }
  }
}
```

## Auto Update

- Description: Whether or not to automatically check for updates.
- Setting: `auto_update`
- Default: `true`

**Options**

`boolean` values

## Buffer Font Family

- Description: The name of a font to use for rendering text in the editor.
- Setting: `buffer_font_family`
- Default: `Zed Mono`

**Options**

The name of any font family installed on the user's system

## Buffer Font Features

- Description: The OpenType features to enable for text in the editor.
- Setting: `buffer_font_features`
- Default: `null`

**Options**

Zed supports a subset of OpenType features that can be enabled or disabled for a given buffer or terminal font. The following [OpenType features](https://en.wikipedia.org/wiki/List_of_typographic_features) can be enabled or disabled too: `calt`, `case`, `cpsp`, `frac`, `liga`, `onum`, `ordn`, `pnum`, `ss01`, `ss02`, `ss03`, `ss04`, `ss05`, `ss06`, `ss07`, `ss08`, `ss09`, `ss10`, `ss11`, `ss12`, `ss13`, `ss14`, `ss15`, `ss16`, `ss17`, `ss18`, `ss19`, `ss20`, `subs`, `sups`, `swsh`, `titl`, `tnum`, `zero`.

For example, to disable ligatures for a given font you can add the following to your settings:

```json
{
  "buffer_font_features": {
    "calt": false
  }
}
```

## Buffer Font Size

- Description: The default font size for text in the editor.
- Setting: `buffer_font_size`
- Default: `15`

**Options**

`integer` values

## Confirm Quit

- Description: Whether or not to prompt the user to confirm before closing the application.
- Setting: `confirm_quit`
- Default: `false`

**Options**

`boolean` values

## Copilot

- Description: Copilot-specific settings.
- Setting: `copilot`
- Default:

```json
"copilot": {
  "disabled_globs": [
    ".env"
  ]
}
```

**Options**

### Disabled Globs

- Description: The set of glob patterns for which Copilot should be disabled in any matching file.
- Setting: `disabled_globs`
- Default: [".env"]

**Options**

List of `string` values

## Cursor Blink

- Description: Whether or not the cursor blinks.
- Setting: `cursor_blink`
- Default: `true`

**Options**

`boolean` values

## Default Dock Anchor

- Description: The default anchor for new docks.
- Setting: `default_dock_anchor`
- Default: `bottom`

**Options**

1. Position the dock attached to the bottom of the workspace: `bottom`
2. Position the dock to the right of the workspace like a side panel: `right`
3. Position the dock full screen over the entire workspace: `expanded`

## Editor Toolbar

- Description: Whether or not to show various elements in the editor toolbar.
- Setting: `toolbar`
- Default:

```json
"toolbar": {
  "breadcrumbs": true,
  "quick_actions": true
},
```

**Options**

Each option controls displaying of a particular toolbar element. If all elements are hidden, the editor toolbar is not displayed.

## Enable Language Server

- Description: Whether or not to use language servers to provide code intelligence.
- Setting: `enable_language_server`
- Default: `true`

**Options**

`boolean` values

## Ensure Final Newline On Save

- Description: Whether or not to ensure there's a single newline at the end of a buffer when saving it.
- Setting: `ensure_final_newline_on_save`
- Default: `true`

**Options**

`boolean` values

## LSP

- Description: Configuration for language servers.
- Setting: `lsp`
- Default: `null`

**Options**

The following settings can be overridden for specific language servers:

- `initialization_options`

To override settings for a language, add an entry for that language server's name to the `lsp` value. Example:

```json
"lsp": {
  "rust-analyzer": {
    "initialization_options": {
      "check": {
        "command": "clippy" // rust-analyzer.check.command (default: "check")
      }
    }
  }
}
```

## Format On Save

- Description: Whether or not to perform a buffer format before saving.
- Setting: `format_on_save`
- Default: `on`

**Options**

1. `on`, enables format on save obeying `formatter` setting:

```json
{
  "format_on_save": "on"
}
```

2. `off`, disables format on save:

```json
{
  "format_on_save": "off"
}
```

## Formatter

- Description: How to perform a buffer format.
- Setting: `formatter`
- Default: `language_server`

**Options**

1. To use the current language server, use `"language_server"`:

```json
{
  "formatter": "language_server"
}
```

2. Or to use an external command, use `"external"`. Specify the name of the formatting program to run, and an array of arguments to pass to the program. The buffer's text will be passed to the program on stdin, and the formatted output should be written to stdout. For example, the following command would strip trailing spaces using [`sed(1)`](https://linux.die.net/man/1/sed):

```json
{
  "formatter": {
    "external": {
      "command": "sed",
      "arguments": ["-e", "s/ *$//"]
    }
  }
}
```

## Auto close

- Description: Whether or not to automatically type closing characters for you.
- Setting: `use_autoclose`
- Default: `true`

**Options**

`boolean` values

## Git

- Description: Configuration for git-related features.
- Setting: `git`
- Default:

```json
"git": {
  "git_gutter": "tracked_files"
},
```

### Git Gutter

- Description: Whether or not to show the git gutter.
- Setting: `git_gutter`
- Default: `tracked_files`

**Options**

1. Show git gutter in tracked files

```json
{
  "git_gutter": "tracked_files"
}
```

2. Hide git gutter

```json
{
  "git_gutter": "hide"
}
```

## Hard Tabs

- Description: Whether to indent lines using tab characters or multiple spaces.
- Setting: `hard_tabs`
- Default: `false`

**Options**

`boolean` values

## Hover Popover Enabled

- Description: Whether or not to show the informational hover box when moving the mouse over symbols in the editor.
- Setting: `hover_popover_enabled`
- Default: `true`

**Options**

`boolean` values

## Inlay hints

- Description: Configuration for displaying extra text with hints in the editor.
- Setting: `inlay_hints`
- Default:

```json
"inlay_hints": {
  "enabled": false,
  "show_type_hints": true,
  "show_parameter_hints": true,
  "show_other_hints": true,
  "edit_debounce_ms": 700,
  "scroll_debounce_ms": 50
}
```

**Options**

Inlay hints querying consists of two parts: editor (client) and LSP server.
With the inlay settings above are changed to enable the hints, editor will start to query certain types of hints and react on LSP hint refresh request from the server.
At this point, the server may or may not return hints depending on its implementation, further configuration might be needed, refer to the corresponding LSP server documentation.

The following languages have inlay hints preconfigured by Zed:

- [Go](https://docs.zed.dev/languages/go)
- [Rust](https://docs.zed.dev/languages/rust)
- [Svelte](https://docs.zed.dev/languages/svelte)
- [Typescript](https://docs.zed.dev/languages/typescript)

Use the `lsp` section for the server configuration. Examples are provided in the corresponding language documentation.

Hints are not instantly queried in Zed, two kinds of debounces are used, either may be set to 0 to be disabled.
Settings-related hint updates are not debounced.

## Journal

- Description: Configuration for the journal.
- Setting: `journal`
- Default:

```json
"journal": {
  "path": "~",
  "hour_format": "hour12"
}
```

### Path

- Description: The path of the directory where journal entries are stored.
- Setting: `path`
- Default: `~`

**Options**

`string` values

### Hour Format

- Description: The format to use for displaying hours in the journal.
- Setting: `hour_format`
- Default: `hour12`

**Options**

1. 12-hour format:

```json
{
  "hour_format": "hour12"
}
```

2. 24-hour format:

```json
{
  "hour_format": "hour24"
}
```

## Language Overrides

- Description: Configuration overrides for specific languages.
- Setting: `language_overrides`
- Default: `null`

**Options**

To override settings for a language, add an entry for that languages name to the `language_overrides` value. Example:

```json
"language_overrides": {
  "C": {
    "format_on_save": "off",
    "preferred_line_length": 64,
    "soft_wrap": "preferred_line_length"
  },
  "JSON": {
    "tab_size": 4
  }
}
```

The following settings can be overridden for each specific language:

- `enable_language_server`
- `ensure_final_newline_on_save`
- `format_on_save`
- `formatter`
- `hard_tabs`
- `preferred_line_length`
- `remove_trailing_whitespace_on_save`
- `show_copilot_suggestions`
- `show_whitespaces`
- `soft_wrap`
- `tab_size`

These values take in the same options as the root-level settings with the same name.

## Preferred Line Length

- Description: The column at which to soft-wrap lines, for buffers where soft-wrap is enabled.
- Setting: `preferred_line_length`
- Default: `80`

**Options**

`integer` values

## Projects Online By Default

- Description: Whether or not to show the online projects view by default.
- Setting: `projects_online_by_default`
- Default: `true`

**Options**

`boolean` values

## Remove Trailing Whitespace On Save

- Description: Whether or not to remove any trailing whitespace from lines of a buffer before saving it.
- Setting: `remove_trailing_whitespace_on_save`
- Default: `true`

**Options**

`boolean` values

## Semantic Index

- Description: Settings related to semantic index.
- Setting: `semantic_index`
- Default:

```json
"semantic_index": {
  "enabled": false
},
```

### Enabled

- Description: Whether or not to display the `Semantic` mode in project search.
- Setting: `enabled`
- Default: `true`

**Options**

`boolean` values

## Show Call Status Icon

- Description: Whether or not to show the call status icon in the status bar.
- Setting: `show_call_status_icon`
- Default: `true`

**Options**

`boolean` values

## Show Completions On Input

- Description: Whether or not to show completions as you type.
- Setting: `show_completions_on_input`
- Default: `true`

**Options**

`boolean` values

## Show Completion Documentation

- Description: Whether to display inline and alongside documentation for items in the completions menu.
- Setting: `show_completion_documentation`
- Default: `true`

**Options**

`boolean` values

## Completion Documentation Debounce Delay

- Description: The debounce delay before re-querying the language server for completion documentation when not included in original completion list.
- Setting: `completion_documentation_secondary_query_debounce`
- Default: `300` ms

**Options**

`integer` values

## Show Copilot Suggestions

- Description: Whether or not to show Copilot suggestions as you type or wait for a `copilot::Toggle`.
- Setting: `show_copilot_suggestions`
- Default: `true`

**Options**

`boolean` values

## Show Whitespaces

- Description: Whether or not to show render whitespace characters in the editor.
- Setting: `show_whitespaces`
- Default: `selection`

**Options**

1. `all`
2. `selection`
3. `none`

## Soft Wrap

- Description: Whether or not to automatically wrap lines of text to fit editor / preferred width.
- Setting: `soft_wrap`
- Default: `none`

**Options**

1. `editor_width`
2. `preferred_line_length`
3. `none`

## Tab Size

- Description: The number of spaces to use for each tab character.
- Setting: `tab_size`
- Default: `4`

**Options**

`integer` values

## Telemetry

- Description: Control what info is collected by Zed.
- Setting: `telemetry`
- Default:

```json
"telemetry": {
  "diagnostics": true,
  "metrics": true
},
```

**Options**

### Diagnostics

- Description: Setting for sending debug-related data, such as crash reports.
- Setting: `diagnostics`
- Default: `true`

**Options**

`boolean` values

### Metrics

- Description: Setting for sending anonymized usage data, such what languages you're using Zed with.
- Setting: `metrics`
- Default: `true`

**Options**

`boolean` values

## Terminal

- Description: Configuration for the terminal.
- Setting: `terminal`
- Default:

```json
"terminal": {
  "alternate_scroll": "off",
  "blinking": "terminal_controlled",
  "copy_on_select": false,
  "env": {},
  "font_family": null,
  "font_features": null,
  "font_size": null,
  "option_as_meta": false,
  "shell": {},
  "toolbar": {
    "title": true
  },
  "working_directory": "current_project_directory"
}
```

### Alternate Scroll

- Description: Set whether Alternate Scroll mode (DECSET code: `?1007`) is active by default. Alternate Scroll mode converts mouse scroll events into up / down key presses when in the alternate screen (e.g. when running applications like vim or less). The terminal can still set and unset this mode with ANSI escape codes.
- Setting: `alternate_scroll`
- Default: `off`

**Options**

1. Default alternate scroll mode to on

```json
{
  "alternate_scroll": "on"
}
```

2. Default alternate scroll mode to off

```json
{
  "alternate_scroll": "off"
}
```

### Blinking

- Description: Set the cursor blinking behavior in the terminal
- Setting: `blinking`
- Default: `terminal_controlled`

**Options**

1. Never blink the cursor, ignore the terminal mode

```json
{
  "blinking": "off"
}
```

2. Default the cursor blink to off, but allow the terminal to turn blinking on

```json
{
  "blinking": "terminal_controlled"
}
```

3. Always blink the cursor, ignore the terminal mode

```json
"blinking": "on",
```

### Copy On Select

- Description: Whether or not selecting text in the terminal will automatically copy to the system clipboard.
- Setting: `copy_on_select`
- Default: `false`

**Options**

`boolean` values

### Env

- Description: Any key-value pairs added to this object will be added to the terminal's environment. Keys must be unique, use `:` to separate multiple values in a single variable
- Setting: `env`
- Default: `{}`

**Example**

```json
"env": {
  "ZED": "1",
  "KEY": "value1:value2"
}
```

### Font Size

- Description: What font size to use for the terminal. When not set defaults to matching the editor's font size
- Setting: `font_size`
- Default: `null`

**Options**

`integer` values

### Font Family

- Description: What font to use for the terminal. When not set, defaults to matching the editor's font.
- Setting: `font_family`
- Default: `null`

**Options**

The name of any font family installed on the user's system

### Font Features

- Description: What font features to use for the terminal. When not set, defaults to matching the editor's font features.
- Setting: `font_features`
- Default: `null`

**Options**

See Buffer Font Features

### Option As Meta

- Description: Re-interprets the option keys to act like a 'meta' key, like in Emacs.
- Setting: `option_as_meta`
- Default: `true`

**Options**

`boolean` values

### Shell

- Description: What shell to use when launching the terminal.
- Setting: `shell`
- Default: `system`

**Options**

1. Use the system's default terminal configuration (usually the `/etc/passwd` file).

```json
{
  "shell": "system"
}
```

2. A program to launch:

```json
"shell": {
    "program": "sh"
}
```

3. A program with arguments:

```json
"shell": {
  "with_arguments": {
    "program": "/bin/bash",
    "args": ["--login"]
  }
}
```

## Terminal Toolbar

- Description: Whether or not to show various elements in the terminal toolbar. It only affects terminals placed in the editor pane.
- Setting: `toolbar`
- Default:

```json
"toolbar": {
  "title": true,
},
```

**Options**

At the moment, only the `title` option is available, it controls displaying of the terminal title that can be changed via `PROMPT_COMMAND`. If the title is hidden, the terminal toolbar is not displayed.

### Working Directory

- Description: What working directory to use when launching the terminal.
- Setting: `working_directory`
- Default: `"current_project_directory"`

**Options**

1. Use the current file's project directory. Will Fallback to the first project directory strategy if unsuccessful

```json
{
  "working_directory": "current_project_directory"
}
```

2. Use the first project in this workspace's directory. Will fallback to using this platform's home directory.

```json
{
  "working_directory": "first_project_directory"
}
```

3. Always use this platform's home directory (if we can find it)

```json
{
  "working_directory": "always_home"
}
```

4. Always use a specific directory. This value will be shell expanded. If this path is not a valid directory the terminal will default to this platform's home directory.

```json
"working_directory": {
  "always": {
    "directory": "~/zed/projects/"
  }
}
```

## Theme

- Description: The theme setting can be specified in two forms - either as the name of a theme or as an object containing the `mode`, `dark`, and `light` themes for the Zed UI.
- Setting: `theme`
- Default: `One Dark`

### Theme Object

- Description: Specify the theme using an object that includes the `mode`, `dark`, and `light` themes.
- Setting: `theme`
- Default:

```json
"theme": {
  "mode": "dark",
  "dark": "One Dark",
  "light": "One Light"
},
```

### Mode

- Description: Specify theme mode.
- Setting: `mode`
- Default: `dark`

**Options**

1. Set the theme to dark mode

```json
{
  "mode": "dark"
}
```

2. Set the theme to light mode

```json
{
  "mode": "light"
}
```

3. Set the theme to system mode

```json
{
  "mode": "system"
}
```

### Dark

- Description: The name of the dark Zed theme to use for the UI.
- Setting: `dark`
- Default: `One Dark`

**Options**

Run the `theme selector: toggle` action in the command palette to see a current list of valid themes names.

### Light

- Description: The name of the light Zed theme to use for the UI.
- Setting: `light`
- Default: `One Light`

**Options**

Run the `theme selector: toggle` action in the command palette to see a current list of valid themes names.

## Vim

- Description: Whether or not to enable vim mode (work in progress).
- Setting: `vim_mode`
- Default: `false`

## Project Panel

- Description: Customise project panel
- Setting: `project_panel`
- Default:

```json
"project_panel": {
  "dock": "left",
  "git_status": true,
  "default_width": "N/A - width in pixels"
},
```

### Dock

- Description: Control the position of the dock
- Setting: `dock`
- Default: `left`

**Options**

1. Default dock position to left

```json
{
  "dock": "left"
}
```

2. Default dock position to right

```json
{
  "dock": "right"
}
```

### Git Status

- Description: Indicates newly created and updated files
- Setting: `git_status`
- Default: `true`

1. Default enable git status

```json
{
  "git_status": true
}
```

2. Default disable git status

```json
{
  "git_status": false
}
```

### Default Width

- Description: Customise default width taken by project panel
- Setting: `default_width`
- Default: N/A width in pixels (eg: 420)

**Options**

`boolean` values

## An example configuration:

```json
// ~/.config/zed/settings.json
{
  "theme": "cave-light",
  "tab_size": 2,
  "preferred_line_length": 80,
  "soft_wrap": "none",

  "buffer_font_size": 18,
  "buffer_font_family": "Zed Mono",

  "autosave": "on_focus_change",
  "format_on_save": "off",
  "vim_mode": false,
  "projects_online_by_default": true,
  "terminal": {
    "font_family": "FiraCode Nerd Font Mono",
    "blinking": "off"
  },
  "language_overrides": {
    "C": {
      "format_on_save": "language_server",
      "preferred_line_length": 64,
      "soft_wrap": "preferred_line_length"
    }
  }
}
```
