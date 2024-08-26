# Configuring Zed

Zed is designed to be configured: we want to fit your workflow and preferences exactly. We provide default settings that are designed to be a comfortable starting point for as many people as possible, but we hope you will enjoy tweaking it to make it feel incredible.

In addition to the settings described here, you may also want to change your [theme](./themes.md), configure your [key bindings](./key-bindings.md), set up [tasks](./tasks.md) or install [extensions](https://github.com/zed-industries/extensions).

## Settings files

<!--
TBD: Settings files. Rewrite with "remote settings" in mind (e.g. `local settings` on the remote host).
Consider renaming `zed: Open Local Settings` to `zed: Open Project Settings`.

TBD: Add settings documentation about how settings are merged as overlays. E.g. project>local>default. Note how settings that are maps are merged, but settings that are arrays are replaced and must include the defaults.
-->

Your settings file can be opened with {#kb zed::OpenSettings}. By default it is located at `~/.config/zed/settings.json`, though if you have XDG_CONFIG_HOME in your environment on Linux it will be at `$XDG_CONFIG_HOME/zed/settings.json` instead.

This configuration is merged with any local configuration inside your projects. You can open the project settings by running {#action zed::OpenLocalSettings} from the command palette. This will create a `.zed` directory containing`.zed/settings.json`.

Although most projects will only need one settings file at the root, you can add more local settings files for subdirectories as needed. Not all settings can be set in local files, just those that impact the behavior of the editor and language tooling. For example you can set `tab_size`, `formatter` etc. but not `theme`, `vim_mode` and similar.

The syntax for configuration files is a super-set of JSON that allows `//` comments.

## Default settings

You can find the default settings for your current Zed by running {#action zed::OpenDefaultSettings} from the command palette.

Extensions that provide language servers may also provide default settings for those language servers.

# Settings

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

1. To disable autosave, set it to `off`:

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
- Default: `Zed Plex Mono`

**Options**

The name of any font family installed on the user's system

## Buffer Font Features

- Description: The OpenType features to enable for text in the editor.
- Setting: `buffer_font_features`
- Default: `null`

**Options**

Zed supports all OpenType features that can be enabled or disabled for a given buffer or terminal font, as well as setting values for font features.

For example, to disable font ligatures, add the following to your settings:

```json
{
  "buffer_font_features": {
    "calt": false
  }
}
```

You can also set other OpenType features, like setting `cv01` to `7`:

```json
{
  "buffer_font_features": {
    "cv01": 7
  }
}
```

## Buffer Font Size

- Description: The default font size for text in the editor.
- Setting: `buffer_font_size`
- Default: `15`

**Options**

`integer` values

## Buffer Font Weight

- Description: The default font weight for text in the editor.
- Setting: `buffer_font_weight`
- Default: `400`

**Options**

`integer` values between `100` and `900`

## Buffer Line Height

- Description: The default line height for text in the editor.
- Setting: `buffer_line_height`
- Default: `"comfortable"`

**Options**

`"standard"`, `"comfortable"` or `{"custom": float}` (`1` is very compact, `2` very loose)

## Confirm Quit

- Description: Whether or not to prompt the user to confirm before closing the application.
- Setting: `confirm_quit`
- Default: `false`

**Options**

`boolean` values

## Centered Layout

- Description: Configuration for the centered layout mode.
- Setting: `centered_layout`
- Default:

```json
"centered_layout": {
  "left_padding": 0.2,
  "right_padding": 0.2,
}
```

**Options**

The `left_padding` and `right_padding` options define the relative width of the
left and right padding of the central pane from the workspace when the centered layout mode is activated. Valid values range is from `0` to `0.4`.

## Direnv Integration

- Description: Settings for [direnv](https://direnv.net/) integration. Requires `direnv` to be installed. `direnv` integration currently only means that the environment variables set by a `direnv` configuration can be used to detect some language servers in `$PATH` instead of installing them.
- Setting: `load_direnv`
- Default:

```json
"load_direnv": "shell_hook"
```

**Options**
There are two options to choose from:

1. `shell_hook`: Use the shell hook to load direnv. This relies on direnv to activate upon entering the directory. Supports POSIX shells and fish.
2. `direct`: Use `direnv export json` to load direnv. This will load direnv directly without relying on the shell hook and might cause some inconsistencies. This allows direnv to work with any shell.

## Inline Completions

- Description: Settings for inline completions.
- Setting: `inline_completions`
- Default:

```json
"inline_completions": {
  "disabled_globs": [
    ".env"
  ]
}
```

**Options**

### Disabled Globs

- Description: A list of globs representing files that inline completions should be disabled for.
- Setting: `disabled_globs`
- Default: `[".env"]`

**Options**

List of `string` values

## Current Line Highlight

- Description: How to highlight the current line in the editor.
- Setting: `current_line_highlight`
- Default: `all`

**Options**

1. Don't highlight the current line:

```json
"current_line_highlight": "none"
```

2. Highlight the gutter area:

```json
"current_line_highlight": "gutter"
```

3. Highlight the editor area:

```json
"current_line_highlight": "line"
```

4. Highlight the full line:

```json
"current_line_highlight": "all"
```

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

## Editor Scrollbar

- Description: Whether or not to show the editor scrollbar and various elements in it.
- Setting: `scrollbar`
- Default:

```json
"scrollbar": {
  "show": "auto",
  "cursors": true,
  "git_diff": true,
  "search_results": true,
  "selected_symbol": true,
  "diagnostics": true
},
```

### Show Mode

- Description: When to show the editor scrollbar.
- Setting: `show`
- Default: `auto`

**Options**

1. Show the scrollbar if there's important information or follow the system's configured behavior:

```json
"scrollbar": {
  "show": "auto"
}
```

2. Match the system's configured behavior:

```json
"scrollbar": {
  "show": "system"
}
```

3. Always show the scrollbar:

```json
"scrollbar": {
  "show": "always"
}
```

4. Never show the scrollbar:

```json
"scrollbar": {
  "show": "never"
}
```

### Cursor Indicators

- Description: Whether to show cursor positions in the scrollbar.
- Setting: `cursors`
- Default: `true`

**Options**

`boolean` values

### Git Diff Indicators

- Description: Whether to show git diff indicators in the scrollbar.
- Setting: `git_diff`
- Default: `true`

**Options**

`boolean` values

### Search Results Indicators

- Description: Whether to show buffer search results in the scrollbar.
- Setting: `search_results`
- Default: `true`

**Options**

`boolean` values

### Selected Symbols Indicators

- Description: Whether to show selected symbol occurrences in the scrollbar.
- Setting: `selected_symbol`
- Default: `true`

**Options**

`boolean` values

### Diagnostics

- Description: Whether to show diagnostic indicators in the scrollbar.
- Setting: `diagnostics`
- Default: `true`

**Options**

`boolean` values

## Editor Tab Bar

- Description: Settings related to the editor's tab bar.
- Settings: `tab_bar`
- Default:

```json
"tab_bar": {
  "show": true,
  "show_nav_history_buttons": true
}
```

### Show

- Description: Whether or not to show the tab bar in the editor.
- Setting: `show`
- Default: `true`

**Options**

`boolean` values

### Navigation History Buttons

- Description: Whether or not to show the navigation history buttons.
- Setting: `show_nav_history_buttons`
- Default: `true`

**Options**

`boolean` values

## Editor Tabs

- Description: Configuration for the editor tabs.
- Setting: `tabs`
- Default:

```json
"tabs": {
  "close_position": "right",
  "file_icons": false,
  "git_status": false
},
```

### Close Position

- Description: Where to display close button within a tab.
- Setting: `close_position`
- Default: `right`

**Options**

1. Display the close button on the right:

```json
{
  "close_position": "right"
}
```

2. Display the close button on the left:

```json
{
  "close_position": "left"
}
```

### File Icons

- Description: Whether to show the file icon for a tab.
- Setting: `file_icons`
- Default: `false`

### Git Status

- Description: Whether or not to show Git file status in tab.
- Setting: `git_status`
- Default: `false`

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
- Default: `auto`

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

3. Or to use code actions provided by the connected language servers, use `"code_actions"`:

```json
{
  "formatter": {
    "code_actions": {
      // Use ESLint's --fix:
      "source.fixAll.eslint": true,
      // Organize imports on save:
      "source.organizeImports": true
    }
  }
}
```

4. Or to use multiple formatters consecutively, use an array of formatters:

```json
{
  "formatter": [
    {"language_server": {"name": "rust-analyzer"}},
    {"external": {
      "command": "sed",
      "arguments": ["-e", "s/ *$//"]
    }
  ]
}
```

Here `rust-analyzer` will be used first to format the code, followed by a call of sed.
If any of the formatters fails, the subsequent ones will still be executed.

## Code Actions On Format

- Description: The code actions to perform with the primary language server when formatting the buffer.
- Setting: `code_actions_on_format`
- Default: `{}`, except for Go it's `{ "source.organizeImports": true }`

**Examples**

<!--
TBD: Add Python Ruff source.organizeImports example
-->

1. Organize imports on format in TypeScript and TSX buffers:

```json
{
  "languages": {
    "TypeScript": {
      "code_actions_on_format": {
        "source.organizeImports": true
      }
    },
    "TSX": {
      "code_actions_on_format": {
        "source.organizeImports": true
      }
    }
  }
}
```

2. Run ESLint `fixAll` code action when formatting:

```json
{
  "languages": {
    "JavaScript": {
      "code_actions_on_format": {
        "source.fixAll.eslint": true
      }
    }
  }
}
```

3. Run only a single ESLint rule when using `fixAll`:

```json
{
  "languages": {
    "JavaScript": {
      "code_actions_on_format": {
        "source.fixAll.eslint": true
      }
    }
  },
  "lsp": {
    "eslint": {
      "settings": {
        "codeActionOnSave": {
          "rules": ["import/order"]
        }
      }
    }
  }
}
```

## Auto close

- Description: Whether to automatically add matching closing characters when typing opening parenthesis, bracket, brace, single or double quote characters.
- Setting: `use_autoclose`
- Default: `true`

**Options**

`boolean` values

## Always Treat Brackets As Autoclosed

- Description: Controls how the editor handles the autoclosed characters.
- Setting: `always_treat_brackets_as_autoclosed`
- Default: `false`

**Options**

`boolean` values

**Example**

If the setting is set to `true`:

1. Enter in the editor: `)))`
2. Move the cursor to the start: `^)))`
3. Enter again: `)))`

The result is still `)))` and not `))))))`, which is what it would be by default.

## File Types

- Setting: `file_types`
- Description: Configure how Zed selects a language for a file based on its filename or extension. Supports glob entries.
- Default: `{}`

**Examples**

To interpret all `.c` files as C++, files called `MyLockFile` as TOML and files starting with `Dockerfile` as Dockerfile:

```json
{
  "file_types": {
    "C++": ["c"],
    "TOML": ["MyLockFile"],
    "Dockerfile": ["Dockerfile*"]
  }
}
```

## Git

- Description: Configuration for git-related features.
- Setting: `git`
- Default:

```json
{
  "git": {
    "git_gutter": "tracked_files",
    "inline_blame": {
      "enabled": true
    }
  }
}
```

### Git Gutter

- Description: Whether or not to show the git gutter.
- Setting: `git_gutter`
- Default: `tracked_files`

**Options**

1. Show git gutter in tracked files

```json
{
  "git": {
    "git_gutter": "tracked_files"
  }
}
```

2. Hide git gutter

```json
{
  "git": {
    "git_gutter": "hide"
  }
}
```

### Indent Guides

- Description: Configuration related to indent guides. Indent guides can be configured separately for each language.
- Setting: `indent_guides`
- Default:

```json
{
  "indent_guides": {
    "enabled": true,
    "line_width": 1,
    "active_line_width": 1,
    "coloring": "fixed",
    "background_coloring": "disabled"
  }
}
```

**Options**

1. Disable indent guides

```json
{
  "indent_guides": {
    "enabled": false
  }
}
```

2. Enable indent guides for a specific language.

```json
{
  "languages": {
    "Python": {
      "indent_guides": {
        "enabled": true
      }
    }
  }
}
```

3. Enable indent aware coloring ("rainbow indentation").
   The colors that are used for different indentation levels are defined in the theme (theme key: `accents`). They can be customized by using theme overrides.

```json
{
  "indent_guides": {
    "enabled": true,
    "coloring": "indent_aware"
  }
}
```

4. Enable indent aware background coloring ("rainbow indentation").
   The colors that are used for different indentation levels are defined in the theme (theme key: `accents`). They can be customized by using theme overrides.

```json
{
  "indent_guides": {
    "enabled": true,
    "coloring": "indent_aware",
    "background_coloring": "indent_aware"
  }
}
```

### Inline Git Blame

- Description: Whether or not to show git blame information inline, on the currently focused line.
- Setting: `inline_blame`
- Default:

```json
{
  "git": {
    "inline_blame": {
      "enabled": true
    }
  }
}
```

**Options**

1. Disable inline git blame:

```json
{
  "git": {
    "inline_blame": {
      "enabled": false
    }
  }
}
```

2. Only show inline git blame after a delay (that starts after cursor stops moving):

```json
{
  "git": {
    "inline_blame": {
      "enabled": false,
      "delay_ms": 500
    }
  }
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

## Languages

- Description: Configuration for specific languages.
- Setting: `languages`
- Default: `null`

**Options**

To override settings for a language, add an entry for that languages name to the `languages` value. Example:

```json
"languages": {
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
- `show_inline_completions`
- `show_whitespaces`
- `soft_wrap`
- `tab_size`
- `use_autoclose`
- `always_treat_brackets_as_autoclosed`

These values take in the same options as the root-level settings with the same name.

## Preview tabs

- Description:
  Preview tabs allow you to open files in preview mode, where they close automatically when you switch to another file unless you explicitly pin them. This is useful for quickly viewing files without cluttering your workspace. Preview tabs display their file names in italics. \
   There are several ways to convert a preview tab into a regular tab:

  - Double-clicking on the file
  - Double-clicking on the tab header
  - Using the `project_panel::OpenPermanent` action
  - Editing the file
  - Dragging the file to a different pane

- Setting: `preview_tabs`
- Default:

```json
"preview_tabs": {
  "enabled": true,
  "enable_preview_from_file_finder": false,
  "enable_preview_from_code_navigation": false,
}
```

### Enable preview from file finder

- Description: Determines whether to open files in preview mode when selected from the file finder.
- Setting: `enable_preview_from_file_finder`
- Default: `false`

**Options**

`boolean` values

### Enable preview from code navigation

- Description: Determines whether a preview tab gets replaced when code navigation is used to navigate away from the tab.
- Setting: `enable_preview_from_code_navigation`
- Default: `false`

**Options**

`boolean` values

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

## Show Inline Completions

- Description: Whether to show inline completions as you type or manually by triggering `editor::ShowInlineCompletion`.
- Setting: `show_inline_completions`
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
4. `boundary`

## Soft Wrap

- Description: Whether or not to automatically wrap lines of text to fit editor / preferred width.
- Setting: `soft_wrap`
- Default: `prefer_line`

**Options**

1. `none` to stop the soft-wrapping
2. `prefer_line` to avoid wrapping generally, unless the line is too long
3. `editor_width` to wrap lines that overflow the editor width
4. `preferred_line_length` to wrap lines that overflow `preferred_line_length` config value

## Wrap Guides (Vertical Rulers)

- Description: Where to display vertical rulers as wrap-guides. Disable by setting `show_wrap_guides` to `false`.
- Setting: `wrap_guides`
- Default: []

**Options**

List of `integer` column numbers

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
{
  "terminal": {
    "alternate_scroll": "off",
    "blinking": "terminal_controlled",
    "copy_on_select": false,
    "dock": "bottom",
    "env": {},
    "font_family": null,
    "font_features": null,
    "font_size": null,
    "line_height": "comfortable",
    "option_as_meta": true,
    "button": false,
    "shell": {},
    "toolbar": {
      "title": true
    },
    "working_directory": "current_project_directory"
  }
}
```

### Terminal: Dock

- Description: Control the position of the dock
- Setting: `dock`
- Default: `bottom`

**Options**

`"bottom"`, `"left"` or `"right"`

### Terminal: Alternate Scroll

- Description: Set whether Alternate Scroll mode (DECSET code: `?1007`) is active by default. Alternate Scroll mode converts mouse scroll events into up / down key presses when in the alternate screen (e.g. when running applications like vim or less). The terminal can still set and unset this mode with ANSI escape codes.
- Setting: `alternate_scroll`
- Default: `off`

**Options**

1. Default alternate scroll mode to on

```json
{
  "terminal": {
    "alternate_scroll": "on"
  }
}
```

2. Default alternate scroll mode to off

```json
{
  "terminal": {
    "alternate_scroll": "off"
  }
}
```

### Terminal: Blinking

- Description: Set the cursor blinking behavior in the terminal
- Setting: `blinking`
- Default: `terminal_controlled`

**Options**

1. Never blink the cursor, ignore the terminal mode

```json
{
  "terminal": {
    "blinking": "off"
  }
}
```

2. Default the cursor blink to off, but allow the terminal to turn blinking on

```json
{
  "terminal": {
    "blinking": "terminal_controlled"
  }
}
```

3. Always blink the cursor, ignore the terminal mode

```json
{
  "terminal": {
    "blinking": "on"
  }
}
```

### Terminal: Copy On Select

- Description: Whether or not selecting text in the terminal will automatically copy to the system clipboard.
- Setting: `copy_on_select`
- Default: `false`

**Options**

`boolean` values

**Example**

```json
{
  "terminal": {
    "copy_on_select": true
  }
}
```

### Terminal: Env

- Description: Any key-value pairs added to this object will be added to the terminal's environment. Keys must be unique, use `:` to separate multiple values in a single variable
- Setting: `env`
- Default: `{}`

**Example**

```json
{
  "terminal": {
    "env": {
      "ZED": "1",
      "KEY": "value1:value2"
    }
  }
}
```

### Terminal: Font Size

- Description: What font size to use for the terminal. When not set defaults to matching the editor's font size
- Setting: `font_size`
- Default: `null`

**Options**

`integer` values

```json
{
  "terminal": {
    "font_size": 15
  }
}
```

### Terminal: Font Family

- Description: What font to use for the terminal. When not set, defaults to matching the editor's font.
- Setting: `font_family`
- Default: `null`

**Options**

The name of any font family installed on the user's system

```json
{
  "terminal": {
    "font_family": "Berkeley Mono"
  }
}
```

### Terminal: Font Features

- Description: What font features to use for the terminal. When not set, defaults to matching the editor's font features.
- Setting: `font_features`
- Default: `null`

**Options**

See Buffer Font Features

```jsonc
{
  "terminal": {
    "font_features": {
      "calt": false,
      // See Buffer Font Features for more features
    },
  },
}
```

### Terminal: Line Height

- Description: Set the terminal's line height.
- Setting: `line_height`
- Default: `comfortable`

**Options**

1. Use a line height that's `comfortable` for reading, 1.618. (default)

```jsonc
{
  "terminal": {
    "line_height": "comfortable",
  },
}
```

2. Use a `standard` line height, 1.3. This option is useful for TUIs, particularly if they use box characters

```jsonc
{
  "terminal": {
    "line_height": "standard",
  },
}
```

3.  Use a custom line height.

```jsonc
{
  "terminal": {
    "line_height": {
      "custom": 2,
    },
  },
}
```

### Terminal: Option As Meta

- Description: Re-interprets the option keys to act like a 'meta' key, like in Emacs.
- Setting: `option_as_meta`
- Default: `true`

**Options**

`boolean` values

```json
{
  "terminal": {
    "option_as_meta": true
  }
}
```

### Terminal: Shell

- Description: What shell to use when launching the terminal.
- Setting: `shell`
- Default: `system`

**Options**

1. Use the system's default terminal configuration (usually the `/etc/passwd` file).

```json
{
  "terminal": {
    "shell": "system"
  }
}
```

2. A program to launch:

```json
{
  "terminal": {
    "shell": {
      "program": "sh"
    }
  }
}
```

3. A program with arguments:

```json
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

## Terminal: Toolbar

- Description: Whether or not to show various elements in the terminal toolbar. It only affects terminals placed in the editor pane.
- Setting: `toolbar`
- Default:

```json
{
  "terminal": {
    "toolbar": {
      "title": true
    }
  }
}
```

**Options**

At the moment, only the `title` option is available, it controls displaying of the terminal title that can be changed via `PROMPT_COMMAND`. If the title is hidden, the terminal toolbar is not displayed.

### Terminal: Button

- Description: Control to show or hide the terminal button in the status bar
- Setting: `button`
- Default: `true`

**Options**

`boolean` values

```json
{
  "terminal": {
    "button": false
  }
}
```

### Terminal: Working Directory

- Description: What working directory to use when launching the terminal.
- Setting: `working_directory`
- Default: `"current_project_directory"`

**Options**

1. Use the current file's project directory. Will Fallback to the first project directory strategy if unsuccessful

```json
{
  "terminal": {
    "working_directory": "current_project_directory"
  }
}
```

2. Use the first project in this workspace's directory. Will fallback to using this platform's home directory.

```json
{
  "terminal": {
    "working_directory": "first_project_directory"
  }
}
```

3. Always use this platform's home directory (if we can find it)

```json
{
  "terminal": {
    "working_directory": "always_home"
  }
}
```

4. Always use a specific directory. This value will be shell expanded. If this path is not a valid directory the terminal will default to this platform's home directory.

```json
{
  "terminal": {
    "working_directory": {
      "always": {
        "directory": "~/zed/projects/"
      }
    }
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
  "mode": "system",
  "dark": "One Dark",
  "light": "One Light"
},
```

### Mode

- Description: Specify theme mode.
- Setting: `mode`
- Default: `system`

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

- Description: Customize project panel
- Setting: `project_panel`
- Default:

```json
{
  "project_panel": {
    "button": true,
    "default_width": 240,
    "dock": "left",
    "file_icons": true,
    "folder_icons": true,
    "git_status": true,
    "indent_size": 20,
    "auto_reveal_entries": true,
    "auto_fold_dirs": true,
    "scrollbar": {
      "show": "always"
    }
  }
}
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

**Options**

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

- Description: Customize default width taken by project panel
- Setting: `default_width`
- Default: N/A width in pixels (eg: 420)

**Options**

`boolean` values

### Auto Reveal Entries

- Description: Whether to reveal it in the project panel automatically, when a corresponding project entry becomes active. Gitignored entries are never auto revealed.
- Setting: `auto_reveal_entries`
- Default: `true`

**Options**

1. Enable auto reveal entries

```json
{
  "auto_reveal_entries": true
}
```

2. Disable auto reveal entries

```json
{
  "auto_reveal_entries": false
}
```

### Auto Fold Dirs

- Description: Whether to fold directories automatically when directory has only one directory inside.
- Setting: `auto_fold_dirs`
- Default: `true`

**Options**

1. Enable auto fold dirs

```json
{
  "auto_fold_dirs": true
}
```

2. Disable auto fold dirs

```json
{
  "auto_fold_dirs": false
}
```

### Indent Size

- Description: Amount of indentation (in pixels) for nested items.
- Setting: `indent_size`
- Default: `20`

### Scrollbar

- Description: Scrollbar related settings. Possible values: "always", "never".
- Setting: `scrollbar`
- Default:

```json
"scrollbar": {
    "show": "always"
}
```

**Options**

1. Show scrollbar in project panel

```json
{
  "scrollbar": {
    "show": "always"
  }
}
```

2. Hide scrollbar in project panel

```json
{
  "scrollbar": {
    "show": "never"
  }
}
```

## Assistant Panel

- Description: Customize assistant panel
- Setting: `assistant`
- Default:

```json
"assistant": {
  "enabled": true,
  "button": true,
  "dock": "right",
  "default_width": 640,
  "default_height": 320,
  "provider": "openai",
  "version": "1",
},
```

## Outline Panel

- Description: Customize outline Panel
- Setting: `outline_panel`
- Default:

```json
"outline_panel": {
  "button": true,
  "default_width": 240,
  "dock": "left",
  "file_icons": true,
  "folder_icons": true,
  "git_status": true,
  "indent_size": 20,
  "auto_reveal_entries": true,
  "auto_fold_dirs": true,
}
```

## Calls

- Description: Customize behavior when participating in a call
- Setting: `calls`
- Default:

```json
"calls": {
  // Join calls with the microphone live by default
  "mute_on_join": false,
  // Share your project when you are the first to join a channel
  "share_on_join": false
},
```

## Unnecessary Code Fade

- Description: How much to fade out unused code.
- Setting: `unnecessary_code_fade`
- Default: `0.3`

**Options**

Float values between `0.0` and `0.9`, where:

- `0.0` means no fading (unused code looks the same as used code)
- `0.9` means maximum fading (unused code is very faint but still visible)

**Example**

```json
{
  "unnecessary_code_fade": 0.5
}
```

## An example configuration:

```json
// ~/.config/zed/settings.json
{
  "theme": "cave-light",
  "tab_size": 2,
  "preferred_line_length": 80,
  "soft_wrap": "none",

  "buffer_font_size": 18,
  "buffer_font_family": "Zed Plex Mono",

  "autosave": "on_focus_change",
  "format_on_save": "off",
  "vim_mode": false,
  "projects_online_by_default": true,
  "terminal": {
    "font_family": "FiraCode Nerd Font Mono",
    "blinking": "off"
  },
  "languages": {
    "C": {
      "format_on_save": "language_server",
      "preferred_line_length": 64,
      "soft_wrap": "preferred_line_length"
    }
  }
}
```
