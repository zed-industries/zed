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

This configuration is merged with any local configuration inside your projects. You can open the project settings by running {#action zed::OpenProjectSettings} from the command palette. This will create a `.zed` directory containing`.zed/settings.json`.

Although most projects will only need one settings file at the root, you can add more local settings files for subdirectories as needed. Not all settings can be set in local files, just those that impact the behavior of the editor and language tooling. For example you can set `tab_size`, `formatter` etc. but not `theme`, `vim_mode` and similar.

The syntax for configuration files is a super-set of JSON that allows `//` comments.

## Default settings

You can find the default settings for your current Zed by running {#action zed::OpenDefaultSettings} from the command palette.

Extensions that provide language servers may also provide default settings for those language servers.

# Settings

## Active Pane Modifiers

- Description: Styling settings applied to the active pane.
- Setting: `active_pane_modifiers`
- Default:

```json
{
  "active_pane_modifiers": {
    "magnification": 1.0,
    "border_size": 0.0,
    "inactive_opacity": 1.0
  }
}
```

### Magnification

- Description: Scale by which to zoom the active pane. When set to `1.0`, the active pane has the same size as others, but when set to a larger value, the active pane takes up more space.
- Setting: `magnification`
- Default: `1.0`

**Options**

`float` values

### Border size

- Description: Size of the border surrounding the active pane. When set to 0, the active pane doesn't have any border. The border is drawn inset.
- Setting: `border_size`
- Default: `0.0`

**Options**

Non-negative `float` values

### Inactive Opacity

- Description: Opacity of inactive panels. When set to 1.0, the inactive panes have the same opacity as the active one. If set to 0, the inactive panes content will not be visible at all. Values are clamped to the [0.0, 1.0] range.
- Setting: `inactive_opacity`
- Default: `1.0`

**Options**

`float` values

## Bottom Dock Layout

- Description: Control the layout of the bottom dock, relative to the left and right docks
- Setting: `bottom_dock_layout`
- Default: `"contained"`

**Options**

1. Contain the bottom dock, giving the full height of the window to the left and right docks

```json
{
  "bottom_dock_layout": "contained"
}
```

2. Give the bottom dock the full width of the window, truncating the left and right docks

```json
{
  "bottom_dock_layout": "full"
}
```

3. Left align the bottom dock, truncating the left dock and giving the right dock the full height of the window

```json
{
  "bottom_dock_layout": "left_aligned"
}
```

3. Right align the bottom dock, giving the left dock the full height of the window and truncating the right dock.

```json
{
  "bottom_dock_layout": "right_aligned"
}
```

## Auto Install extensions

- Description: Define extensions to be autoinstalled or never be installed.
- Setting: `auto_install_extension`
- Default: `{ "html": true }`

**Options**

You can find the names of your currently installed extensions by listing the subfolders under the [extension installation location](./extensions/installing-extensions#installation-location):

On MacOS:

```sh
ls ~/Library/Application\ Support/Zed/extensions/installed/
```

On Linux:

```sh
ls ~/.local/share/zed/extensions/installed
```

Define extensions which should be installed (`true`) or never installed (`false`).

```json
{
  "auto_install_extensions": {
    "html": true,
    "dockerfile": true,
    "docker-compose": false
  }
}
```

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

## Restore on Startup

- Description: Controls session restoration on startup.
- Setting: `restore_on_startup`
- Default: `last_session`

**Options**

1. Restore all workspaces that were open when quitting Zed:

```json
{
  "restore_on_startup": "last_session"
}
```

2. Restore the workspace that was closed last:

```json
{
  "restore_on_startup": "last_workspace"
}
```

3. Always start with an empty editor:

```json
{
  "restore_on_startup": "none"
}
```

## Autoscroll on Clicks

- Description: Whether to scroll when clicking near the edge of the visible text area.
- Setting: `autoscroll_on_clicks`
- Default: `false`

**Options**

`boolean` values

## Auto Update

- Description: Whether or not to automatically check for updates.
- Setting: `auto_update`
- Default: `true`

**Options**

`boolean` values

## Base Keymap

- Description: Base key bindings scheme. Base keymaps can be overridden with user keymaps.
- Setting: `base_keymap`
- Default: `VSCode`

**Options**

1. VSCode

```json
{
  "base_keymap": "VSCode"
}
```

2. Atom

```json
{
  "base_keymap": "Atom"
}
```

3. JetBrains

```json
{
  "base_keymap": "JetBrains"
}
```

4. None

```json
{
  "base_keymap": "None"
}
```

5. SublimeText

```json
{
  "base_keymap": "SublimeText"
}
```

6. TextMate

```json
{
  "base_keymap": "TextMate"
}
```

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
- Platform: macOS and Windows.

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

## Buffer Font Fallbacks

- Description: Set the buffer text's font fallbacks, this will be merged with the platform's default fallbacks.
- Setting: `buffer_font_fallbacks`
- Default: `null`
- Platform: macOS and Windows.

**Options**

For example, to use `Nerd Font` as a fallback, add the following to your settings:

```json
{
  "buffer_font_fallbacks": ["Nerd Font"]
}
```

## Buffer Font Size

- Description: The default font size for text in the editor.
- Setting: `buffer_font_size`
- Default: `15`

**Options**

`integer` values from `6` to `100` pixels (inclusive)

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

`"standard"`, `"comfortable"` or `{ "custom": float }` (`1` is compact, `2` is loose)

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

- Description: Settings for [direnv](https://direnv.net/) integration. Requires `direnv` to be installed.
  `direnv` integration make it possible to use the environment variables set by a `direnv` configuration to detect some language servers in `$PATH` instead of installing them.
  It also allows for those environment variables to be used in tasks.
- Setting: `load_direnv`
- Default: `"direct"`

**Options**

There are two options to choose from:

1. `shell_hook`: Use the shell hook to load direnv. This relies on direnv to activate upon entering the directory. Supports POSIX shells and fish.
2. `direct`: Use `direnv export json` to load direnv. This will load direnv directly without relying on the shell hook and might cause some inconsistencies. This allows direnv to work with any shell.

## Edit Predictions

- Description: Settings for edit predictions.
- Setting: `edit_predictions`
- Default:

```json
  "edit_predictions": {
    "disabled_globs": [
      "**/.env*",
      "**/*.pem",
      "**/*.key",
      "**/*.cert",
      "**/*.crt",
      "**/.dev.vars",
      "**/secrets.yml"
    ]
  }
```

**Options**

### Disabled Globs

- Description: A list of globs for which edit predictions should be disabled for. This list adds to a pre-existing, sensible default set of globs. Any additional ones you add are combined with them.
- Setting: `disabled_globs`
- Default: `["**/.env*", "**/*.pem", "**/*.key", "**/*.cert", "**/*.crt", "**/.dev.vars", "**/secrets.yml"]`

**Options**

List of `string` values.

## Edit Predictions Disabled in

- Description: A list of language scopes in which edit predictions should be disabled.
- Setting: `edit_predictions_disabled_in`
- Default: `[]`

**Options**

List of `string` values

1. Don't show edit predictions in comments:

```json
"disabled_in": ["comment"]
```

2. Don't show edit predictions in strings and comments:

```json
"disabled_in": ["comment", "string"]
```

3. Only in Go, don't show edit predictions in strings and comments:

```json
{
  "languages": {
    "Go": {
      "edit_predictions_disabled_in": ["comment", "string"]
    }
  }
}
```

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

## Selection Highlight

- Description: Whether to highlight all occurrences of the selected text in an editor.
- Setting: `selection_highlight`
- Default: `true`

## LSP Highlight Debounce

- Description: The debounce delay before querying highlights from the language server based on the current cursor location.
- Setting: `lsp_highlight_debounce`
- Default: `75`

## Cursor Blink

- Description: Whether or not the cursor blinks.
- Setting: `cursor_blink`
- Default: `true`

**Options**

`boolean` values

## Cursor Shape

- Description: Cursor shape for the default editor.
- Setting: `cursor_shape`
- Default: `bar`

**Options**

1. A vertical bar:

```json
"cursor_shape": "bar"
```

2. A block that surrounds the following character:

```json
"cursor_shape": "block"
```

3. An underline / underscore that runs along the following character:

```json
"cursor_shape": "underline"
```

4. An box drawn around the following character:

```json
"cursor_shape": "hollow"
```

## Hide Mouse

- Description: Determines when the mouse cursor should be hidden in an editor or input box.
- Setting: `hide_mouse`
- Default: `on_typing_and_movement`

**Options**

1. Never hide the mouse cursor:

```json
"hide_mouse": "never"
```

2. Hide only when typing:

```json
"hide_mouse": "on_typing"
```

3. Hide on both typing and cursor movement:

```json
"hide_mouse": "on_typing_and_movement"
```

## Snippet Sort Order

- Description: Determines how snippets are sorted relative to other completion items.
- Setting: `snippet_sort_order`
- Default: `inline`

**Options**

1. Place snippets at the top of the completion list:

```json
"snippet_sort_order": "top"
```

2. Place snippets normally without any preference:

```json
"snippet_sort_order": "inline"
```

3. Place snippets at the bottom of the completion list:

```json
"snippet_sort_order": "bottom"
```

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
  "selected_text": true,
  "selected_symbol": true,
  "diagnostics": "all",
  "axes": {
    "horizontal": true,
    "vertical": true,
  },
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

### Selected Text Indicators

- Description: Whether to show selected text occurrences in the scrollbar.
- Setting: `selected_text`
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

- Description: Which diagnostic indicators to show in the scrollbar.
- Setting: `diagnostics`
- Default: `all`

**Options**

1. Show all diagnostics:

```json
{
  "diagnostics": "all"
}
```

2. Do not show any diagnostics:

```json
{
  "diagnostics": "none"
}
```

3. Show only errors:

```json
{
  "diagnostics": "error"
}
```

4. Show only errors and warnings:

```json
{
  "diagnostics": "warning"
}
```

5. Show only errors, warnings, and information:

```json
{
  "diagnostics": "information"
}
```

### Axes

- Description: Forcefully enable or disable the scrollbar for each axis
- Setting: `axes`
- Default:

```json
"scrollbar": {
  "axes": {
    "horizontal": true,
    "vertical": true,
  },
}
```

#### Horizontal

- Description: When false, forcefully disables the horizontal scrollbar. Otherwise, obey other settings.
- Setting: `horizontal`
- Default: `true`

**Options**

`boolean` values

#### Vertical

- Description: When false, forcefully disables the vertical scrollbar. Otherwise, obey other settings.
- Setting: `vertical`
- Default: `true`

**Options**

`boolean` values

## Minimap

- Description: Settings related to the editor's minimap, which provides an overview of your document.
- Setting: `minimap`
- Default:

```json
{
  "minimap": {
    "show": "never",
    "thumb": "always",
    "thumb_border": "left_open",
    "current_line_highlight": null
  }
}
```

### Show Mode

- Description: When to show the minimap in the editor.
- Setting: `show`
- Default: `never`

**Options**

1. Always show the minimap:

```json
{
  "show": "always"
}
```

2. Show the minimap if the editor's scrollbars are visible:

```json
{
  "show": "auto"
}
```

3. Never show the minimap:

```json
{
  "show": "never"
}
```

### Thumb Display

- Description: When to show the minimap thumb (the visible editor area) in the minimap.
- Setting: `thumb`
- Default: `always`

**Options**

1. Show the minimap thumb when hovering over the minimap:

```json
{
  "thumb": "hover"
}
```

2. Always show the minimap thumb:

```json
{
  "thumb": "always"
}
```

### Thumb Border

- Description: How the minimap thumb border should look.
- Setting: `thumb_border`
- Default: `left_open`

**Options**

1. Display a border on all sides of the thumb:

```json
{
  "thumb_border": "full"
}
```

2. Display a border on all sides except the left side:

```json
{
  "thumb_border": "left_open"
}
```

3. Display a border on all sides except the right side:

```json
{
  "thumb_border": "right_open"
}
```

4. Display a border only on the left side:

```json
{
  "thumb_border": "left_only"
}
```

5. Display the thumb without any border:

```json
{
  "thumb_border": "none"
}
```

### Current Line Highlight

- Description: How to highlight the current line in the minimap.
- Setting: `current_line_highlight`
- Default: `null`

**Options**

1. Inherit the editor's current line highlight setting:

```json
{
  "minimap": {
    "current_line_highlight": null
  }
}
```

2. Highlight the current line in the minimap:

```json
{
  "minimap": {
    "current_line_highlight": "line"
  }
}
```

or

```json
{
  "minimap": {
    "current_line_highlight": "all"
  }
}
```

3. Do not highlight the current line in the minimap:

```json
{
  "minimap": {
    "current_line_highlight": "gutter"
  }
}
```

or

```json
{
  "minimap": {
    "current_line_highlight": "none"
  }
}
```

## Editor Tab Bar

- Description: Settings related to the editor's tab bar.
- Settings: `tab_bar`
- Default:

```json
"tab_bar": {
  "show": true,
  "show_nav_history_buttons": true,
  "show_tab_bar_buttons": true
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

### Tab Bar Buttons

- Description: Whether or not to show the tab bar buttons.
- Setting: `show_tab_bar_buttons`
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
  "git_status": false,
  "activate_on_close": "history",
  "show_close_button": "hover",
  "show_diagnostics": "off"
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

### Activate on close

- Description: What to do after closing the current tab.
- Setting: `activate_on_close`
- Default: `history`

**Options**

1.  Activate the tab that was open previously:

```json
{
  "activate_on_close": "history"
}
```

2. Activate the right neighbour tab if present:

```json
{
  "activate_on_close": "neighbour"
}
```

3. Activate the left neighbour tab if present:

```json
{
  "activate_on_close": "left_neighbour"
}
```

### Show close button

- Description: Controls the appearance behavior of the tab's close button.
- Setting: `show_close_button`
- Default: `hover`

**Options**

1.  Show it just upon hovering the tab:

```json
{
  "show_close_button": "hover"
}
```

2. Show it persistently:

```json
{
  "show_close_button": "always"
}
```

3. Never show it, even if hovering it:

```json
{
  "show_close_button": "hidden"
}
```

### Show Diagnostics

- Description: Whether to show diagnostics indicators in tabs. This setting only works when file icons are active and controls which files with diagnostic issues to mark.
- Setting: `show_diagnostics`
- Default: `off`

**Options**

1. Do not mark any files:

```json
{
  "show_diagnostics": "off"
}
```

2. Only mark files with errors:

```json
{
  "show_diagnostics": "errors"
}
```

3. Mark files with errors and warnings:

```json
{
  "show_diagnostics": "all"
}
```

## Editor Toolbar

- Description: Whether or not to show various elements in the editor toolbar.
- Setting: `toolbar`
- Default:

```json
"toolbar": {
  "breadcrumbs": true,
  "quick_actions": true,
  "selections_menu": true,
  "agent_review": true
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

- Description: Removes any lines containing only whitespace at the end of the file and ensures just one newline at the end.
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
- `settings`

To override configuration for a language server, add an entry for that language server's name to the `lsp` value.

Some options are passed via `initialization_options` to the language server. These are for options which must be specified at language server startup and when changed will require restarting the language server.

For example to pass the `check` option to `rust-analyzer`, use the following configuration:

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

While other options may be changed at a runtime and should be placed under `settings`:

```json
"lsp": {
  "yaml-language-server": {
    "settings": {
      "yaml": {
        "keyOrdering": true // Enforces alphabetical ordering of keys in maps
      }
    }
  }
}
```

## LSP Highlight Debounce

- Description: The debounce delay in milliseconds before querying highlights from the language server based on the current cursor location.
- Setting: `lsp_highlight_debounce`
- Default: `75`

**Options**

`integer` values representing milliseconds

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

3. External formatters may optionally include a `{buffer_path}` placeholder which at runtime will include the path of the buffer being formatted. Formatters operate by receiving file content via standard input, reformatting it and then outputting it to standard output and so normally don't know the filename of what they are formatting. Tools like prettier support receiving the file path via a command line argument which can then used to impact formatting decisions.

WARNING: `{buffer_path}` should not be used to direct your formatter to read from a filename. Your formatter should only read from standard input and should not read or write files directly.

```json
  "formatter": {
    "external": {
      "command": "prettier",
      "arguments": ["--stdin-filepath", "{buffer_path}"]
    }
  }
```

4. Or to use code actions provided by the connected language servers, use `"code_actions"`:

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

5. Or to use multiple formatters consecutively, use an array of formatters:

```json
{
  "formatter": [
    { "language_server": { "name": "rust-analyzer" } },
    {
      "external": {
        "command": "sed",
        "arguments": ["-e", "s/ *$//"]
      }
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

## File Scan Exclusions

- Setting: `file_scan_exclusions`
- Description: Files or globs of files that will be excluded by Zed entirely. They will be skipped during file scans, file searches, and not be displayed in the project file tree. Overrides `file_scan_inclusions`.
- Default:

```json
"file_scan_exclusions": [
  "**/.git",
  "**/.svn",
  "**/.hg",
  "**/.jj",
  "**/CVS",
  "**/.DS_Store",
  "**/Thumbs.db",
  "**/.classpath",
  "**/.settings"
],
```

Note, specifying `file_scan_exclusions` in settings.json will override the defaults (shown above). If you are looking to exclude additional items you will need to include all the default values in your settings.

## File Scan Inclusions

- Setting: `file_scan_inclusions`
- Description: Files or globs of files that will be included by Zed, even when ignored by git. This is useful for files that are not tracked by git, but are still important to your project. Note that globs that are overly broad can slow down Zed's file scanning. `file_scan_exclusions` takes precedence over these inclusions.
- Default:

```json
"file_scan_inclusions": [".env*"],
```

## File Types

- Setting: `file_types`
- Description: Configure how Zed selects a language for a file based on its filename or extension. Supports glob entries.
- Default:

```json
"file_types": {
  "JSONC": ["**/.zed/**/*.json", "**/zed/**/*.json", "**/Zed/**/*.json", "**/.vscode/**/*.json"],
  "Shell Script": [".env.*"]
}
```

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

## Diagnostics

- Description: Configuration for diagnostics-related features.
- Setting: `diagnostics`
- Default:

```json
{
  "diagnostics": {
    "include_warnings": true,
    "inline": {
      "enabled": false
    },
    "update_with_cursor": false,
    "primary_only": false,
    "use_rendered": false
  }
}
```

### Inline Diagnostics

- Description: Whether or not to show diagnostics information inline.
- Setting: `inline`
- Default:

```json
{
  "diagnostics": {
    "inline": {
      "enabled": false,
      "update_debounce_ms": 150,
      "padding": 4,
      "min_column": 0,
      "max_severity": null
    }
  }
}
```

**Options**

1. Enable inline diagnostics.

```json
{
  "diagnostics": {
    "inline": {
      "enabled": true
    }
  }
}
```

2. Delay diagnostic updates until some time after the last diagnostic update.

```json
{
  "diagnostics": {
    "inline": {
      "enabled": true,
      "update_debounce_ms": 150
    }
  }
}
```

3. Set padding between the end of the source line and the start of the diagnostic.

```json
{
  "diagnostics": {
    "inline": {
      "enabled": true,
      "padding": 4
    }
  }
}
```

4. Horizontally align inline diagnostics at the given column.

```json
{
  "diagnostics": {
    "inline": {
      "enabled": true,
      "min_column": 80
    }
  }
}
```

5. Show only warning and error diagnostics.

```json
{
  "diagnostics": {
    "inline": {
      "enabled": true,
      "max_severity": "warning"
    }
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
    },
    "hunk_style": "staged_hollow"
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

### Gutter Debounce

- Description: Sets the debounce threshold (in milliseconds) after which changes are reflected in the git gutter.
- Setting: `gutter_debounce`
- Default: `null`

**Options**

`integer` values representing milliseconds

Example:

```json
{
  "git": {
    "gutter_debounce": 100
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

### Hunk Style

- Description: What styling we should use for the diff hunks.
- Setting: `hunk_style`
- Default:

```json
{
  "git": {
    "hunk_style": "staged_hollow"
  }
}
```

**Options**

1. Show the staged hunks faded out and with a border:

```json
{
  "git": {
    "hunk_style": "staged_hollow"
  }
}
```

2. Show unstaged hunks faded out and with a border:

```json
{
  "git": {
    "hunk_style": "unstaged_hollow"
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
      "enabled": true,
      "delay_ms": 500
    }
  }
}
```

3. Show a commit summary next to the commit date and author:

```json
{
  "git": {
    "inline_blame": {
      "enabled": true,
      "show_commit_summary": true
    }
  }
}
```

4. Use this as the minimum column at which to display inline blame information:

```json
{
  "git": {
    "inline_blame": {
      "enabled": true,
      "min_column": 80
    }
  }
}
```

## Indent Guides

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

## Hover Popover Delay

- Description: Time to wait in milliseconds before showing the informational hover box.
- Setting: `hover_popover_delay`
- Default: `300`

**Options**

`integer` values representing milliseconds

## Icon Theme

- Description: The icon theme setting can be specified in two forms - either as the name of an icon theme or as an object containing the `mode`, `dark`, and `light` icon themes for files/folders inside Zed.
- Setting: `icon_theme`
- Default: `Zed (Default)`

### Icon Theme Object

- Description: Specify the icon theme using an object that includes the `mode`, `dark`, and `light`.
- Setting: `icon_theme`
- Default:

```json
"icon_theme": {
  "mode": "system",
  "dark": "Zed (Default)",
  "light": "Zed (Default)"
},
```

### Mode

- Description: Specify the icon theme mode.
- Setting: `mode`
- Default: `system`

**Options**

1. Set the icon theme to dark mode

```json
{
  "mode": "dark"
}
```

2. Set the icon theme to light mode

```json
{
  "mode": "light"
}
```

3. Set the icon theme to system mode

```json
{
  "mode": "system"
}
```

### Dark

- Description: The name of the dark icon theme.
- Setting: `dark`
- Default: `Zed (Default)`

**Options**

Run the `icon theme selector: toggle` action in the command palette to see a current list of valid icon themes names.

### Light

- Description: The name of the light icon theme.
- Setting: `light`
- Default: `Zed (Default)`

**Options**

Run the `icon theme selector: toggle` action in the command palette to see a current list of valid icon themes names.

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
  "show_background": false,
  "edit_debounce_ms": 700,
  "scroll_debounce_ms": 50,
  "toggle_on_modifiers_press": null
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

All possible config values for `toggle_on_modifiers_press` are:

```json
"inlay_hints": {
  "toggle_on_modifiers_press": {
    "control": true,
    "shift": true,
    "alt": true,
    "platform": true,
    "function": true
  }
}
```

Unspecified values have a `false` value, hints won't be toggled if all the modifiers are `false` or not all the modifiers are pressed.

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

- [`enable_language_server`](#enable-language-server)
- [`ensure_final_newline_on_save`](#ensure-final-newline-on-save)
- [`format_on_save`](#format-on-save)
- [`formatter`](#formatter)
- [`hard_tabs`](#hard-tabs)
- [`preferred_line_length`](#preferred-line-length)
- [`remove_trailing_whitespace_on_save`](#remove-trailing-whitespace-on-save)
- [`show_edit_predictions`](#show-edit-predictions)
- [`show_whitespaces`](#show-whitespaces)
- [`soft_wrap`](#soft-wrap)
- [`tab_size`](#tab-size)
- [`use_autoclose`](#use-autoclose)
- [`always_treat_brackets_as_autoclosed`](#always-treat-brackets-as-autoclosed)

These values take in the same options as the root-level settings with the same name.

## Network Proxy

- Description: Configure a network proxy for Zed.
- Setting: `proxy`
- Default: `null`

**Options**

The proxy setting must contain a URL to the proxy.

The following URI schemes are supported:

- `http`
- `https`
- `socks4` - SOCKS4 proxy with local DNS
- `socks4a` - SOCKS4 proxy with remote DNS
- `socks5` - SOCKS5 proxy with local DNS
- `socks5h` - SOCKS5 proxy with remote DNS

`http` will be used when no scheme is specified.

By default no proxy will be used, or Zed will attempt to retrieve proxy settings from environment variables, such as `http_proxy`, `HTTP_PROXY`, `https_proxy`, `HTTPS_PROXY`, `all_proxy`, `ALL_PROXY`.

For example, to set an `http` proxy, add the following to your settings:

```json
{
  "proxy": "http://127.0.0.1:10809"
}
```

Or to set a `socks5` proxy:

```json
{
  "proxy": "socks5h://localhost:10808"
}
```

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

## File Finder

### File Icons

- Description: Whether to show file icons in the file finder.
- Setting: `file_icons`
- Default: `true`

### Modal Max Width

- Description: Max-width of the file finder modal. It can take one of these values: `small`, `medium`, `large`, `xlarge`, and `full`.
- Setting: `modal_max_width`
- Default: `small`

### Skip Focus For Active In Search

- Description: Determines whether the file finder should skip focus for the active file in search results.
- Setting: `skip_focus_for_active_in_search`
- Default: `true`

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

## Search

- Description: Search options to enable by default when opening new project and buffer searches.
- Setting: `search`
- Default:

```json
"search": {
  "whole_word": false,
  "case_sensitive": false,
  "include_ignored": false,
  "regex": false
},
```

## Seed Search Query From Cursor

- Description: When to populate a new search's query based on the text under the cursor.
- Setting: `seed_search_query_from_cursor`
- Default: `always`

**Options**

1. `always` always populate the search query with the word under the cursor
2. `selection` only populate the search query when there is text selected
3. `never` never populate the search query

## Use Smartcase Search

- Description: When enabled, automatically adjusts search case sensitivity based on your query. If your search query contains any uppercase letters, the search becomes case-sensitive; if it contains only lowercase letters, the search becomes case-insensitive. \
  This applies to both in-file searches and project-wide searches.
- Setting: `use_smartcase_search`
- Default: `false`

**Options**

`boolean` values

Examples:

- Searching for "function" would match "function", "Function", "FUNCTION", etc.
- Searching for "Function" would only match "Function", not "function" or "FUNCTION"

## Show Call Status Icon

- Description: Whether or not to show the call status icon in the status bar.
- Setting: `show_call_status_icon`
- Default: `true`

**Options**

`boolean` values

## Completions

- Description: Controls how completions are processed for this language.
- Setting: `completions`
- Default:

```json
{
  "completions": {
    "words": "fallback",
    "lsp": true,
    "lsp_fetch_timeout_ms": 0,
    "lsp_insert_mode": "replace_suffix"
  }
}
```

### Words

- Description: Controls how words are completed. For large documents, not all words may be fetched for completion.
- Setting: `words`
- Default: `fallback`

**Options**

1. `enabled` - Always fetch document's words for completions along with LSP completions
2. `fallback` - Only if LSP response errors or times out, use document's words to show completions
3. `disabled` - Never fetch or complete document's words for completions (word-based completions can still be queried via a separate action)

### LSP

- Description: Whether to fetch LSP completions or not.
- Setting: `lsp`
- Default: `true`

**Options**

`boolean` values

### LSP Fetch Timeout (ms)

- Description: When fetching LSP completions, determines how long to wait for a response of a particular server. When set to 0, waits indefinitely.
- Setting: `lsp_fetch_timeout_ms`
- Default: `0`

**Options**

`integer` values representing milliseconds

### LSP Insert Mode

- Description: Controls what range to replace when accepting LSP completions.
- Setting: `lsp_insert_mode`
- Default: `replace_suffix`

**Options**

1. `insert` - Replaces text before the cursor, using the `insert` range described in the LSP specification
2. `replace` - Replaces text before and after the cursor, using the `replace` range described in the LSP specification
3. `replace_subsequence` - Behaves like `"replace"` if the text that would be replaced is a subsequence of the completion text, and like `"insert"` otherwise
4. `replace_suffix` - Behaves like `"replace"` if the text after the cursor is a suffix of the completion, and like `"insert"` otherwise

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

## Show Edit Predictions

- Description: Whether to show edit predictions as you type or manually by triggering `editor::ShowEditPrediction`.
- Setting: `show_edit_predictions`
- Default: `true`

**Options**

`boolean` values

## Show Whitespaces

- Description: Whether or not to render whitespace characters in the editor.
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
- Default: `none`

**Options**

1. `none` to avoid wrapping generally, unless the line is too long
2. `prefer_line` (deprecated, same as `none`)
3. `editor_width` to wrap lines that overflow the editor width
4. `preferred_line_length` to wrap lines that overflow `preferred_line_length` config value
5. `bounded` to wrap lines at the minimum of `editor_width` and `preferred_line_length`

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
    "default_width": 640,
    "default_height": 320,
    "detect_venv": {
      "on": {
        "directories": [".env", "env", ".venv", "venv"],
        "activate_script": "default"
      }
    },
    "env": {},
    "font_family": null,
    "font_features": null,
    "font_size": null,
    "line_height": "comfortable",
    "option_as_meta": false,
    "button": true,
    "shell": "system",
    "toolbar": {
      "breadcrumbs": true
    },
    "working_directory": "current_project_directory",
    "scrollbar": {
      "show": null
    }
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

1. Default alternate scroll mode to off

```json
{
  "terminal": {
    "alternate_scroll": "off"
  }
}
```

2. Default alternate scroll mode to on

```json
{
  "terminal": {
    "alternate_scroll": "on"
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
- Platform: macOS and Windows.

**Options**

See Buffer Font Features

```json
{
  "terminal": {
    "font_features": {
      "calt": false
      // See Buffer Font Features for more features
    }
  }
}
```

### Terminal: Line Height

- Description: Set the terminal's line height.
- Setting: `line_height`
- Default: `comfortable`

**Options**

1. Use a line height that's `comfortable` for reading, 1.618. (default)

```json
{
  "terminal": {
    "line_height": "comfortable"
  }
}
```

2. Use a `standard` line height, 1.3. This option is useful for TUIs, particularly if they use box characters

```json
{
  "terminal": {
    "line_height": "standard"
  }
}
```

3.  Use a custom line height.

```json
{
  "terminal": {
    "line_height": {
      "custom": 2
    }
  }
}
```

### Terminal: Option As Meta

- Description: Re-interprets the option keys to act like a 'meta' key, like in Emacs.
- Setting: `option_as_meta`
- Default: `false`

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

## Terminal: Detect Virtual Environments {#terminal-detect_venv}

- Description: Activate the [Python Virtual Environment](https://docs.python.org/3/library/venv.html), if one is found, in the terminal's working directory (as resolved by the working_directory and automatically activating the virtual environment.
- Setting: `detect_venv`
- Default:

```json
{
  "terminal": {
    "detect_venv": {
      "on": {
        // Default directories to search for virtual environments, relative
        // to the current working directory. We recommend overriding this
        // in your project's settings, rather than globally.
        "directories": [".env", "env", ".venv", "venv"],
        // Can also be `csh`, `fish`, and `nushell`
        "activate_script": "default"
      }
    }
  }
}
```

Disable with:

```json
{
  "terminal": {
    "detect_venv": "off"
  }
}
```

## Terminal: Toolbar

- Description: Whether or not to show various elements in the terminal toolbar.
- Setting: `toolbar`
- Default:

```json
{
  "terminal": {
    "toolbar": {
      "breadcrumbs": true
    }
  }
}
```

**Options**

At the moment, only the `breadcrumbs` option is available, it controls displaying of the terminal title that can be changed via `PROMPT_COMMAND`.

If the terminal title is empty, the breadcrumbs won't be shown.

The shell running in the terminal needs to be configured to emit the title.

Example command to set the title: `echo -e "\e]2;New Title\007";`

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
    "entry_spacing": "comfortable",
    "file_icons": true,
    "folder_icons": true,
    "git_status": true,
    "indent_size": 20,
    "auto_reveal_entries": true,
    "auto_fold_dirs": true,
    "scrollbar": {
      "show": null
    },
    "show_diagnostics": "all",
    "indent_guides": {
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

### Entry Spacing

- Description: Spacing between worktree entries
- Setting: `entry_spacing`
- Default: `comfortable`

**Options**

1. Comfortable entry spacing

```json
{
  "entry_spacing": "comfortable"
}
```

2. Standard entry spacing

```json
{
  "entry_spacing": "standard"
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
- Default: `240`

**Options**

`float` values

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

### Indent Guides: Show

- Description: Whether to show indent guides in the project panel.
- Setting: `indent_guides`
- Default:

```json
"indent_guides": {
  "show": "always"
}
```

**Options**

1. Show indent guides in the project panel

```json
{
  "indent_guides": {
    "show": "always"
  }
}
```

2. Hide indent guides in the project panel

```json
{
  "indent_guides": {
    "show": "never"
  }
}
```

### Scrollbar: Show

- Description: Whether to show a scrollbar in the project panel. Possible values: null, "auto", "system", "always", "never". Inherits editor settings when absent, see its description for more details.
- Setting: `scrollbar`
- Default:

```json
"scrollbar": {
  "show": null
}
```

**Options**

1. Show scrollbar in the project panel

```json
{
  "scrollbar": {
    "show": "always"
  }
}
```

2. Hide scrollbar in the project panel

```json
{
  "scrollbar": {
    "show": "never"
  }
}
```

## Agent

- Description: Customize agent behavior
- Setting: `agent`
- Default:

```json
"agent": {
  "version": "2",
  "enabled": true,
  "button": true,
  "dock": "right",
  "default_width": 640,
  "default_height": 320,
  "default_model": {
    "provider": "zed.dev",
    "model": "claude-3-7-sonnet-latest"
  },
  "editor_model": {
    "provider": "zed.dev",
    "model": "claude-3-7-sonnet-latest"
  },
  "single_file_review": true,
}
```

## Outline Panel

- Description: Customize outline Panel
- Setting: `outline_panel`
- Default:

```json
"outline_panel": {
  "button": true,
  "default_width": 300,
  "dock": "left",
  "file_icons": true,
  "folder_icons": true,
  "git_status": true,
  "indent_size": 20,
  "auto_reveal_entries": true,
  "auto_fold_dirs": true,
  "indent_guides": {
    "show": "always"
  },
  "scrollbar": {
    "show": null
  }
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

## UI Font Family

- Description: The name of the font to use for text in the UI.
- Setting: `ui_font_family`
- Default: `Zed Plex Sans`

**Options**

The name of any font family installed on the system.

## UI Font Features

- Description: The OpenType features to enable for text in the UI.
- Setting: `ui_font_features`
- Default:

```json
"ui_font_features": {
  "calt": false
}
```

- Platform: macOS and Windows.

**Options**

Zed supports all OpenType features that can be enabled or disabled for a given UI font, as well as setting values for font features.

For example, to disable font ligatures, add the following to your settings:

```json
{
  "ui_font_features": {
    "calt": false
  }
}
```

You can also set other OpenType features, like setting `cv01` to `7`:

```json
{
  "ui_font_features": {
    "cv01": 7
  }
}
```

## UI Font Fallbacks

- Description: The font fallbacks to use for text in the UI.
- Setting: `ui_font_fallbacks`
- Default: `null`
- Platform: macOS and Windows.

**Options**

For example, to use `Nerd Font` as a fallback, add the following to your settings:

```json
{
  "ui_font_fallbacks": ["Nerd Font"]
}
```

## UI Font Size

- Description: The default font size for text in the UI.
- Setting: `ui_font_size`
- Default: `16`

**Options**

`integer` values from `6` to `100` pixels (inclusive)

## UI Font Weight

- Description: The default font weight for text in the UI.
- Setting: `ui_font_weight`
- Default: `400`

**Options**

`integer` values between `100` and `900`

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
