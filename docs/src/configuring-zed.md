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

Styling settings applied to the active pane.

### Magnification

- Description: Scale by which to zoom the active pane. When set to `1.0`, the active pane has the same size as others, but when set to a larger value, the active pane takes up more space.
- Setting: `magnification`
- Default: `1.0`

### Border size

- Description: Size of the border surrounding the active pane. When set to 0, the active pane doesn't have any border. The border is drawn inset.
- Setting: `border_size`
- Default: `0.0`

### Inactive Opacity

- Description: Opacity of inactive panels. When set to 1.0, the inactive panes have the same opacity as the active one. If set to 0, the inactive panes content will not be visible at all. Values are clamped to the [0.0, 1.0] range.
- Setting: `inactive_opacity`
- Default: `1.0`

**Options**

`float` values

## Auto Install extensions

- Description: Define extensions to be autoinstalled or never be installed.
- Setting: `auto_install_extension`
- Default: `{"html": true}`

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

- Description: Settings for [direnv](https://direnv.net/) integration. Requires `direnv` to be installed.
  `direnv` integration make it possible to use the environment variables set by a `direnv` configuration to detect some language servers in `$PATH` instead of installing them.
  It also allows for those environment variables to be used in tasks.
- Setting: `load_direnv`
- Default:

```json
"load_direnv": "direct"
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

## Inline Completions Disabled in

- Description: A list of language scopes in which inline completions should be disabled.
- Setting: `inline_completions_disabled_in`
- Default: `[]`

**Options**

List of `string` values

1. Don't show inline completions in comments:

```json
"disabled_in": ["comment"]
```

2. Don't show inline completions in strings and comments:

```json
"disabled_in": ["comment", "string"]
```

3. Only in Go, don't show inline completions in strings and comments:

```json
{
  "languages": {
    "Go": {
      "inline_completions_disabled_in": ["comment", "string"]
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
  "git_status": false,
  "activate_on_close": "history"
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

2. Activate the neighbour tab (prefers the right one, if present):

```json
{
  "activate_on_close": "neighbour"
}
```

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

## File Scan Exclusions

- Setting: `file_scan_exclusions`
- Description: Configure how Add filename or directory globs that will be excluded by Zed entirely. They will be skipped during file scans, file searches and hidden from project file tree.
- Default:

```json
"file_scan_exclusions": [
  "**/.git",
  "**/.svn",
  "**/.hg",
  "**/CVS",
  "**/.DS_Store",
  "**/Thumbs.db",
  "**/.classpath",
  "**/.settings"
],
```

Note, specifying `file_scan_exclusions` in settings.json will override the defaults (shown above). If you are looking to exclude additional items you will need to include all the default values in your settings.

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

### Modal Max Width

- Description: Max-width of the file finder modal. It can take one of these values: `small`, `medium`, `large`, `xlarge`, and `full`.
- Setting: `max_modal_width`
- Default: `small`

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
- Default: `none`

**Options**

1. `none` to avoid wrapping generally, unless the line is too long
2. `prefer_line` (deprecated, same as `none`)
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
    "button": false,
    "shell": {},
    "toolbar": {
      "breadcrumbs": true
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
        "directories": [".venv", "venv"],
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
    "file_icons": true,
    "folder_icons": true,
    "git_status": true,
    "indent_size": 20,
    "indent_guides": true,
    "auto_reveal_entries": true,
    "auto_fold_dirs": true,
    "scrollbar": {
      "show": null
    },
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

### Indent Guides: Show

- Description: Whether to show indent guides in the project panel. Possible values: "always", "never".
- Setting: `indent_guides`

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
- Default: `null`
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
