# Configuring Zed

Zed is designed to be configured: we want to fit your workflow and preferences exactly. We provide default settings that are designed to be a comfortable starting point for as many people as possible, but we hope you will enjoy tweaking it to make it feel incredible.

In addition to the settings described here, you may also want to change your [theme](./themes.md), configure your [key bindings](./key-bindings.md), set up [tasks](./tasks.md) or install [extensions](https://github.com/zed-industries/extensions).

## Settings Editor

You can browse through many of the supported settings via the Settings Editor, which can be opened with the {#kb zed::OpenSettings} keybinding, or through the `zed: open settings` action in the command palette. Through it, you can customize your local, user settings as well as project settings.

> Note that not all settings that Zed supports are available through the Settings Editor yet.
> Some more intricate ones, such as language formatters, can only be changed through the JSON settings file {#kb zed::OpenSettingsFile}.

## User Settings File

<!--
TBD: Settings files. Rewrite with "remote settings" in mind (e.g. `local settings` on the remote host).
Consider renaming `zed: Open Local Settings` to `zed: Open Project Settings`.

TBD: Add settings documentation about how settings are merged as overlays. E.g. project>local>default. Note how settings that are maps are merged, but settings that are arrays are replaced and must include the defaults.
-->

Your settings JSON file can be opened with {#kb zed::OpenSettingsFile}.
By default it is located at `~/.config/zed/settings.json`, though if you have `XDG_CONFIG_HOME` in your environment on Linux it will be at `$XDG_CONFIG_HOME/zed/settings.json` instead.

Whatever you have added to your user settings file gets merged with any local configuration inside your projects.

### Default Settings

In the Settings Editor, the values you see set are the default ones.
You can also verify them in JSON by running {#action zed::OpenDefaultSettings} from the command palette.

Extensions that provide language servers may also provide default settings for those language servers.

## Project Settings File

Similarly to user files, you can open your project settings file by running {#action zed::OpenProjectSettings} from the command palette.
This will create a `.zed` directory containing`.zed/settings.json`.

Although most projects will only need one settings file at the root, you can add more local settings files for subdirectories as needed.
Not all settings can be set in local files, just those that impact the behavior of the editor and language tooling.
For example you can set `tab_size`, `formatter` etc. but not `theme`, `vim_mode` and similar.

The syntax for configuration files is a super-set of JSON that allows `//` comments.

## Per-release Channel Overrides

Zed reads the same `settings.json` across all release channels (Stable, Preview or Nightly).
However, you can scope overrides to a specific channel by adding top-level `stable`, `preview`, `nightly` or `dev` objects.
They are merged into the base configuration with settings from these keys taking precedence upon launching the specified build. For example:

```json [settings]
{
  "theme": "sunset",
  "vim_mode": false,
  "nightly": {
    "theme": "cave-light",
    "vim_mode": true
  },
  "preview": {
    "theme": "zed-dark"
  }
}
```

With this configuration, Stable keeps all base preferences, Preview switches to `zed-dark`, and Nightly enables Vim mode with a different theme.

Changing settings in the Settings Editorwill always apply the change across all channels.

# Settings

Find below an extensive run-through of many supported settings by Zed.

## Active Pane Modifiers

- Description: Styling settings applied to the active pane.
- Setting: `active_pane_modifiers`
- Default:

```json [settings]
{
  "active_pane_modifiers": {
    "border_size": 0.0,
    "inactive_opacity": 1.0
  }
}
```

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

- Description: Control the layout of the bottom dock, relative to the left and right docks.
- Setting: `bottom_dock_layout`
- Default: `"contained"`

**Options**

1. Contain the bottom dock, giving the full height of the window to the left and right docks.

```json [settings]
{
  "bottom_dock_layout": "contained"
}
```

2. Give the bottom dock the full width of the window, truncating the left and right docks.

```json [settings]
{
  "bottom_dock_layout": "full"
}
```

3. Left align the bottom dock, truncating the left dock and giving the right dock the full height of the window.

```json [settings]
{
  "bottom_dock_layout": "left_aligned"
}
```

4. Right align the bottom dock, giving the left dock the full height of the window and truncating the right dock.

```json [settings]
{
  "bottom_dock_layout": "right_aligned"
}
```

## Agent Font Size

- Description: The font size for text in the agent panel. Inherits the UI font size if unset.
- Setting: `agent_font_size`
- Default: `null`

**Options**

`integer` values from `6` to `100` pixels (inclusive)

## Allow Rewrap

- Description: Controls where the {#action editor::Rewrap} action is allowed in the current language scope
- Setting: `allow_rewrap`
- Default: `"in_comments"`

**Options**

1. Allow rewrap in comments only:

```json [settings]
{
  "allow_rewrap": "in_comments"
}
```

2. Allow rewrap in selections only:

```json [settings]
{
  "allow_rewrap": "in_selections"
}
```

3. Allow rewrap anywhere:

```json [settings]
{
  "allow_rewrap": "anywhere"
}
```

Note: This setting has no effect in Vim mode, as rewrap is already allowed everywhere.

## Auto Indent

- Description: Whether indentation should be adjusted based on the context whilst typing. This can be specified on a per-language basis.
- Setting: `auto_indent`
- Default: `true`

**Options**

`boolean` values

## Auto Indent On Paste

- Description: Whether indentation of pasted content should be adjusted based on the context
- Setting: `auto_indent_on_paste`
- Default: `true`

**Options**

`boolean` values

## Auto Install extensions

- Description: Define extensions to be autoinstalled or never be installed.
- Setting: `auto_install_extensions`
- Default: `{ "html": true }`

**Options**

You can find the names of your currently installed extensions by listing the subfolders under the [extension installation location](./extensions/installing-extensions.md#installation-location):

On macOS:

```sh
ls ~/Library/Application\ Support/Zed/extensions/installed/
```

On Linux:

```sh
ls ~/.local/share/zed/extensions/installed
```

Define extensions which should be installed (`true`) or never installed (`false`).

```json [settings]
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

```json [settings]
{
  "autosave": "off"
}
```

2. To autosave when focus changes, use `on_focus_change`:

```json [settings]
{
  "autosave": "on_focus_change"
}
```

3. To autosave when the active window changes, use `on_window_change`:

```json [settings]
{
  "autosave": "on_window_change"
}
```

4. To autosave after an inactivity period, use `after_delay`:

```json [settings]
{
  "autosave": {
    "after_delay": {
      "milliseconds": 1000
    }
  }
}
```

Note that a save will be triggered when an unsaved tab is closed, even if this is earlier than the configured inactivity period.

## Autoscroll on Clicks

- Description: Whether to scroll when clicking near the edge of the visible text area.
- Setting: `autoscroll_on_clicks`
- Default: `false`

**Options**

`boolean` values

## Auto Signature Help

- Description: Show method signatures in the editor, when inside parentheses
- Setting: `auto_signature_help`
- Default: `false`

**Options**

`boolean` values

### Show Signature Help After Edits

- Description: Whether to show the signature help after completion or a bracket pair inserted. If `auto_signature_help` is enabled, this setting will be treated as enabled also.
- Setting: `show_signature_help_after_edits`
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

1. VS Code

```json [settings]
{
  "base_keymap": "VSCode"
}
```

2. Atom

```json [settings]
{
  "base_keymap": "Atom"
}
```

3. JetBrains

```json [settings]
{
  "base_keymap": "JetBrains"
}
```

4. None

```json [settings]
{
  "base_keymap": "None"
}
```

5. Sublime Text

```json [settings]
{
  "base_keymap": "SublimeText"
}
```

6. TextMate

```json [settings]
{
  "base_keymap": "TextMate"
}
```

## Buffer Font Family

- Description: The name of a font to use for rendering text in the editor.
- Setting: `buffer_font_family`
- Default: `.ZedMono`. This currently aliases to [Lilex](https://lilex.myrt.co).

**Options**

The name of any font family installed on the user's system, or `".ZedMono"`.

## Buffer Font Features

- Description: The OpenType features to enable for text in the editor.
- Setting: `buffer_font_features`
- Default: `null`
- Platform: macOS and Windows.

**Options**

Zed supports all OpenType features that can be enabled or disabled for a given buffer or terminal font, as well as setting values for font features.

For example, to disable font ligatures, add the following to your settings:

```json [settings]
{
  "buffer_font_features": {
    "calt": false
  }
}
```

You can also set other OpenType features, like setting `cv01` to `7`:

```json [settings]
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

```json [settings]
{
  "buffer_font_fallbacks": ["Nerd Font"]
}
```

## Buffer Font Size

- Description: The default font size for text in the editor.
- Setting: `buffer_font_size`
- Default: `15`

**Options**

A font size from `6` to `100` pixels (inclusive)

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

## Centered Layout

- Description: Configuration for the centered layout mode.
- Setting: `centered_layout`
- Default:

```json [settings]
"centered_layout": {
  "left_padding": 0.2,
  "right_padding": 0.2,
}
```

**Options**

The `left_padding` and `right_padding` options define the relative width of the
left and right padding of the central pane from the workspace when the centered layout mode is activated. Valid values range is from `0` to `0.4`.

## Close on File Delete

- Description: Whether to automatically close editor tabs when their corresponding files are deleted from disk.
- Setting: `close_on_file_delete`
- Default: `false`

**Options**

`boolean` values

When enabled, this setting will automatically close tabs for files that have been deleted from the file system. This is particularly useful for workflows involving temporary or scratch files that are frequently created and deleted. When disabled (default), deleted files remain open with a strikethrough through their tab title.

Note: Dirty files (files with unsaved changes) will not be automatically closed even when this setting is enabled, ensuring you don't lose unsaved work.

## Confirm Quit

- Description: Whether or not to prompt the user to confirm before closing the application.
- Setting: `confirm_quit`
- Default: `false`

**Options**

`boolean` values

## Diagnostics Max Severity

- Description: Which level to use to filter out diagnostics displayed in the editor
- Setting: `diagnostics_max_severity`
- Default: `null`

**Options**

1. Allow all diagnostics (default):

```json [settings]
{
  "diagnostics_max_severity": "all"
}
```

2. Show only errors:

```json [settings]
{
  "diagnostics_max_severity": "error"
}
```

3. Show errors and warnings:

```json [settings]
{
  "diagnostics_max_severity": "warning"
}
```

4. Show errors, warnings, and information:

```json [settings]
{
  "diagnostics_max_severity": "info"
}
```

5. Show all including hints:

```json [settings]
{
  "diagnostics_max_severity": "hint"
}
```

## Disable AI

- Description: Whether to disable all AI features in Zed
- Setting: `disable_ai`
- Default: `false`

**Options**

`boolean` values

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

## Double Click In Multibuffer

- Description: What to do when multibuffer is double clicked in some of its excerpts (parts of singleton buffers)
- Setting: `double_click_in_multibuffer`
- Default: `"select"`

**Options**

1. Behave as a regular buffer and select the whole word (default):

```json [settings]
{
  "double_click_in_multibuffer": "select"
}
```

2. Open the excerpt clicked as a new buffer in the new tab:

```json [settings]
{
  "double_click_in_multibuffer": "open"
}
```

For the case of "open", regular selection behavior can be achieved by holding `alt` when double clicking.

## Drop Target Size

- Description: Relative size of the drop target in the editor that will open dropped file as a split pane (0-0.5). For example, 0.25 means if you drop onto the top/bottom quarter of the pane a new vertical split will be used, if you drop onto the left/right quarter of the pane a new horizontal split will be used.
- Setting: `drop_target_size`
- Default: `0.2`

**Options**

`float` values between `0` and `0.5`

## Edit Predictions

- Description: Settings for edit predictions.
- Setting: `edit_predictions`
- Default:

```json [settings]
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

```json [settings]
"disabled_in": ["comment"]
```

2. Don't show edit predictions in strings and comments:

```json [settings]
"disabled_in": ["comment", "string"]
```

3. Only in Go, don't show edit predictions in strings and comments:

```json [settings]
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

```json [settings]
"current_line_highlight": "none"
```

2. Highlight the gutter area:

```json [settings]
"current_line_highlight": "gutter"
```

3. Highlight the editor area:

```json [settings]
"current_line_highlight": "line"
```

4. Highlight the full line:

```json [settings]
"current_line_highlight": "all"
```

## Selection Highlight

- Description: Whether to highlight all occurrences of the selected text in an editor.
- Setting: `selection_highlight`
- Default: `true`

## Rounded Selection

- Description: Whether the text selection should have rounded corners.
- Setting: `rounded_selection`
- Default: `true`

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

```json [settings]
"cursor_shape": "bar"
```

2. A block that surrounds the following character:

```json [settings]
"cursor_shape": "block"
```

3. An underline / underscore that runs along the following character:

```json [settings]
"cursor_shape": "underline"
```

4. An box drawn around the following character:

```json [settings]
"cursor_shape": "hollow"
```

## Gutter

- Description: Settings for the editor gutter
- Setting: `gutter`
- Default:

```json [settings]
{
  "gutter": {
    "line_numbers": true,
    "runnables": true,
    "breakpoints": true,
    "folds": true,
    "min_line_number_digits": 4
  }
}
```

**Options**

- `line_numbers`: Whether to show line numbers in the gutter
- `runnables`: Whether to show runnable buttons in the gutter
- `breakpoints`: Whether to show breakpoints in the gutter
- `folds`: Whether to show fold buttons in the gutter
- `min_line_number_digits`: Minimum number of characters to reserve space for in the gutter

## Hide Mouse

- Description: Determines when the mouse cursor should be hidden in an editor or input box.
- Setting: `hide_mouse`
- Default: `on_typing_and_movement`

**Options**

1. Never hide the mouse cursor:

```json [settings]
"hide_mouse": "never"
```

2. Hide only when typing:

```json [settings]
"hide_mouse": "on_typing"
```

3. Hide on both typing and cursor movement:

```json [settings]
"hide_mouse": "on_typing_and_movement"
```

## Snippet Sort Order

- Description: Determines how snippets are sorted relative to other completion items.
- Setting: `snippet_sort_order`
- Default: `inline`

**Options**

1. Place snippets at the top of the completion list:

```json [settings]
"snippet_sort_order": "top"
```

2. Place snippets normally without any preference:

```json [settings]
"snippet_sort_order": "inline"
```

3. Place snippets at the bottom of the completion list:

```json [settings]
"snippet_sort_order": "bottom"
```

4. Do not show snippets in the completion list at all:

```json [settings]
"snippet_sort_order": "none"
```

## Editor Scrollbar

- Description: Whether or not to show the editor scrollbar and various elements in it.
- Setting: `scrollbar`
- Default:

```json [settings]
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

```json [settings]
"scrollbar": {
  "show": "auto"
}
```

2. Match the system's configured behavior:

```json [settings]
"scrollbar": {
  "show": "system"
}
```

3. Always show the scrollbar:

```json [settings]
"scrollbar": {
  "show": "always"
}
```

4. Never show the scrollbar:

```json [settings]
"scrollbar": {
  "show": "never"
}
```

### Cursor Indicators

- Description: Whether to show cursor positions in the scrollbar.
- Setting: `cursors`
- Default: `true`

Cursor indicators appear as small marks on the scrollbar showing where other collaborators' cursors are positioned in the file.

**Options**

`boolean` values

### Git Diff Indicators

- Description: Whether to show git diff indicators in the scrollbar.
- Setting: `git_diff`
- Default: `true`

Git diff indicators appear as colored marks showing lines that have been added, modified, or deleted compared to the git HEAD.

**Options**

`boolean` values

### Search Results Indicators

- Description: Whether to show buffer search results in the scrollbar.
- Setting: `search_results`
- Default: `true`

Search result indicators appear as marks showing all locations in the file where your current search query matches.

**Options**

`boolean` values

### Selected Text Indicators

- Description: Whether to show selected text occurrences in the scrollbar.
- Setting: `selected_text`
- Default: `true`

Selected text indicators appear as marks showing all occurrences of the currently selected text throughout the file.

**Options**

`boolean` values

### Selected Symbols Indicators

- Description: Whether to show selected symbol occurrences in the scrollbar.
- Setting: `selected_symbol`
- Default: `true`

Selected symbol indicators appear as marks showing all occurrences of the currently selected symbol (like a function or variable name) throughout the file.

**Options**

`boolean` values

### Diagnostics

- Description: Which diagnostic indicators to show in the scrollbar.
- Setting: `diagnostics`
- Default: `all`

Diagnostic indicators appear as colored marks showing errors, warnings, and other language server diagnostics at their corresponding line positions in the file.

**Options**

1. Show all diagnostics:

```json [settings]
{
  "show_diagnostics": "all"
}
```

2. Do not show any diagnostics:

```json [settings]
{
  "show_diagnostics": "off"
}
```

3. Show only errors:

```json [settings]
{
  "show_diagnostics": "error"
}
```

4. Show only errors and warnings:

```json [settings]
{
  "show_diagnostics": "warning"
}
```

5. Show only errors, warnings, and information:

```json [settings]
{
  "show_diagnostics": "info"
}
```

### Axes

- Description: Forcefully enable or disable the scrollbar for each axis
- Setting: `axes`
- Default:

```json [settings]
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

```json [settings]
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

```json [settings]
{
  "show": "always"
}
```

2. Show the minimap if the editor's scrollbars are visible:

```json [settings]
{
  "show": "auto"
}
```

3. Never show the minimap:

```json [settings]
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

```json [settings]
{
  "thumb": "hover"
}
```

2. Always show the minimap thumb:

```json [settings]
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

```json [settings]
{
  "thumb_border": "full"
}
```

2. Display a border on all sides except the left side:

```json [settings]
{
  "thumb_border": "left_open"
}
```

3. Display a border on all sides except the right side:

```json [settings]
{
  "thumb_border": "right_open"
}
```

4. Display a border only on the left side:

```json [settings]
{
  "thumb_border": "left_only"
}
```

5. Display the thumb without any border:

```json [settings]
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

```json [settings]
{
  "minimap": {
    "current_line_highlight": null
  }
}
```

2. Highlight the current line in the minimap:

```json [settings]
{
  "minimap": {
    "current_line_highlight": "line"
  }
}
```

or

```json [settings]
{
  "minimap": {
    "current_line_highlight": "all"
  }
}
```

3. Do not highlight the current line in the minimap:

```json [settings]
{
  "minimap": {
    "current_line_highlight": "gutter"
  }
}
```

or

```json [settings]
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

```json [settings]
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

```json [settings]
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

```json [settings]
{
  "close_position": "right"
}
```

2. Display the close button on the left:

```json [settings]
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

```json [settings]
{
  "activate_on_close": "history"
}
```

2. Activate the right neighbour tab if present:

```json [settings]
{
  "activate_on_close": "neighbour"
}
```

3. Activate the left neighbour tab if present:

```json [settings]
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

```json [settings]
{
  "show_close_button": "hover"
}
```

2. Show it persistently:

```json [settings]
{
  "show_close_button": "always"
}
```

3. Never show it, even if hovering it:

```json [settings]
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

```json [settings]
{
  "show_diagnostics": "off"
}
```

2. Only mark files with errors:

```json [settings]
{
  "show_diagnostics": "errors"
}
```

3. Mark files with errors and warnings:

```json [settings]
{
  "show_diagnostics": "all"
}
```

### Show Inline Code Actions

- Description: Whether to show code action button at start of buffer line.
- Setting: `inline_code_actions`
- Default: `true`

**Options**

`boolean` values

### Drag And Drop Selection

- Description: Whether to allow drag and drop text selection in buffer. `delay` is the milliseconds that must elapse before drag and drop is allowed. Otherwise, a new text selection is created.
- Setting: `drag_and_drop_selection`
- Default:

```json [settings]
"drag_and_drop_selection": {
  "enabled": true,
  "delay": 300
}
```

## Editor Toolbar

- Description: Whether or not to show various elements in the editor toolbar.
- Setting: `toolbar`
- Default:

```json [settings]
"toolbar": {
  "breadcrumbs": true,
  "quick_actions": true,
  "selections_menu": true,
  "agent_review": true,
  "code_actions": false
},
```

**Options**

Each option controls displaying of a particular toolbar element. If all elements are hidden, the editor toolbar is not displayed.

## Use System Tabs

- Description: Whether to allow windows to tab together based on the user’s tabbing preference (macOS only).
- Setting: `use_system_window_tabs`
- Default: `false`

**Options**

This setting enables integration with macOS’s native window tabbing feature. When set to `true`, Zed windows can be grouped together as tabs in a single macOS window, following the system-wide tabbing preferences set by the user (such as "Always", "In Full Screen", or "Never"). This setting is only available on macOS.

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

## Expand Excerpt Lines

- Description: The default number of lines to expand excerpts in the multibuffer by
- Setting: `expand_excerpt_lines`
- Default: `5`

**Options**

Positive `integer` values

## Excerpt Context Lines

- Description: The number of lines of context to provide when showing excerpts in the multibuffer.
- Setting: `excerpt_context_lines`
- Default: `2`

**Options**

Positive `integer` value between 1 and 32. Values outside of this range will be clamped to this range.

## Extend Comment On Newline

- Description: Whether to start a new line with a comment when a previous line is a comment as well.
- Setting: `extend_comment_on_newline`
- Default: `true`

**Options**

`boolean` values

## Status Bar

- Description: Control various elements in the status bar. Note that some items in the status bar have their own settings set elsewhere.
- Setting: `status_bar`
- Default:

```json [settings]
"status_bar": {
  "active_language_button": true,
  "cursor_position_button": true,
  "line_endings_button": false
},
```

There is an experimental setting that completely hides the status bar. This causes major usability problems (you will be unable to use many of Zed's features), but is provided for those who value screen real-estate above all else.

```json
"status_bar": {
  "experimental.show": false
}
```

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

```json [settings]
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

```json [settings]
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

## Global LSP Settings

- Description: Configuration for global LSP settings that apply to all language servers
- Setting: `global_lsp_settings`
- Default:

```json [settings]
{
  "global_lsp_settings": {
    "button": true
  }
}
```

**Options**

- `button`: Whether to show the LSP status button in the status bar

## LSP Highlight Debounce

- Description: The debounce delay in milliseconds before querying highlights from the language server based on the current cursor location.
- Setting: `lsp_highlight_debounce`
- Default: `75`

**Options**

`integer` values representing milliseconds

## Features

- Description: Features that can be globally enabled or disabled
- Setting: `features`
- Default:

```json [settings]
{
  "features": {
    "edit_prediction_provider": "zed"
  }
}
```

### Edit Prediction Provider

- Description: Which edit prediction provider to use
- Setting: `edit_prediction_provider`
- Default: `"zed"`

**Options**

1. Use Zeta as the edit prediction provider:

```json [settings]
{
  "features": {
    "edit_prediction_provider": "zed"
  }
}
```

2. Use Copilot as the edit prediction provider:

```json [settings]
{
  "features": {
    "edit_prediction_provider": "copilot"
  }
}
```

3. Use Supermaven as the edit prediction provider:

```json [settings]
{
  "features": {
    "edit_prediction_provider": "supermaven"
  }
}
```

4. Turn off edit predictions across all providers

```json [settings]
{
  "features": {
    "edit_prediction_provider": "none"
  }
}
```

## Format On Save

- Description: Whether or not to perform a buffer format before saving.
- Setting: `format_on_save`
- Default: `on`

**Options**

1. `on`, enables format on save obeying `formatter` setting:

```json [settings]
{
  "format_on_save": "on"
}
```

2. `off`, disables format on save:

```json [settings]
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

```json [settings]
{
  "formatter": "language_server"
}
```

2. Or to use an external command, use `"external"`. Specify the name of the formatting program to run, and an array of arguments to pass to the program. The buffer's text will be passed to the program on stdin, and the formatted output should be written to stdout. For example, the following command would strip trailing spaces using [`sed(1)`](https://linux.die.net/man/1/sed):

```json [settings]
{
  "formatter": {
    "external": {
      "command": "sed",
      "arguments": ["-e", "s/ *$//"]
    }
  }
}
```

3. External formatters may optionally include a `{buffer_path}` placeholder which at runtime will include the path of the buffer being formatted. Formatters operate by receiving file content via standard input, reformatting it and then outputting it to standard output and so normally don't know the filename of what they are formatting. Tools like Prettier support receiving the file path via a command line argument which can then used to impact formatting decisions.

WARNING: `{buffer_path}` should not be used to direct your formatter to read from a filename. Your formatter should only read from standard input and should not read or write files directly.

```json [settings]
  "formatter": {
    "external": {
      "command": "prettier",
      "arguments": ["--stdin-filepath", "{buffer_path}"]
    }
  }
```

4. Or to use code actions provided by the connected language servers, use `"code_actions"`:

```json [settings]
{
  "formatter": [
    // Use ESLint's --fix:
    { "code_action": "source.fixAll.eslint" },
    // Organize imports on save:
    { "code_action": "source.organizeImports" }
  ]
}
```

5. Or to use multiple formatters consecutively, use an array of formatters:

```json [settings]
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

```json [settings]
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

```json [settings]
"file_scan_inclusions": [".env*"],
```

## File Types

- Setting: `file_types`
- Description: Configure how Zed selects a language for a file based on its filename or extension. Supports glob entries.
- Default:

```json [settings]
"file_types": {
  "JSONC": ["**/.zed/**/*.json", "**/zed/**/*.json", "**/Zed/**/*.json", "**/.vscode/**/*.json"],
  "Shell Script": [".env.*"]
}
```

**Examples**

To interpret all `.c` files as C++, files called `MyLockFile` as TOML and files starting with `Dockerfile` as Dockerfile:

```json [settings]
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

```json [settings]
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

```json [settings]
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

```json [settings]
{
  "diagnostics": {
    "inline": {
      "enabled": true
    }
  }
}
```

2. Delay diagnostic updates until some time after the last diagnostic update.

```json [settings]
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

```json [settings]
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

```json [settings]
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

```json [settings]
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

```json [settings]
{
  "git": {
    "git_gutter": "tracked_files",
    "inline_blame": {
      "enabled": true
    },
    "branch_picker": {
      "show_author_name": true
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

```json [settings]
{
  "git": {
    "git_gutter": "tracked_files"
  }
}
```

2. Hide git gutter

```json [settings]
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

```json [settings]
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

```json [settings]
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

```json [settings]
{
  "git": {
    "inline_blame": {
      "enabled": false
    }
  }
}
```

2. Only show inline git blame after a delay (that starts after cursor stops moving):

```json [settings]
{
  "git": {
    "inline_blame": {
      "delay_ms": 500
    }
  }
}
```

3. Show a commit summary next to the commit date and author:

```json [settings]
{
  "git": {
    "inline_blame": {
      "show_commit_summary": true
    }
  }
}
```

4. Use this as the minimum column at which to display inline blame information:

```json [settings]
{
  "git": {
    "inline_blame": {
      "min_column": 80
    }
  }
}
```

5. Set the padding between the end of the line and the inline blame hint, in ems:

```json [settings]
{
  "git": {
    "inline_blame": {
      "padding": 10
    }
  }
}
```

### Branch Picker

- Description: Configuration related to the branch picker.
- Setting: `branch_picker`
- Default:

```json [settings]
{
  "git": {
    "branch_picker": {
      "show_author_name": false
    }
  }
}
```

**Options**

1. Show the author name in the branch picker:

```json [settings]
{
  "git": {
    "branch_picker": {
      "show_author_name": true
    }
  }
}
```

### Hunk Style

- Description: What styling we should use for the diff hunks.
- Setting: `hunk_style`
- Default:

```json [settings]
{
  "git": {
    "hunk_style": "staged_hollow"
  }
}
```

**Options**

1. Show the staged hunks faded out and with a border:

```json [settings]
{
  "git": {
    "hunk_style": "staged_hollow"
  }
}
```

2. Show unstaged hunks faded out and with a border:

```json [settings]
{
  "git": {
    "hunk_style": "unstaged_hollow"
  }
}
```

## Go to Definition Fallback

- Description: What to do when the {#action editor::GoToDefinition} action fails to find a definition
- Setting: `go_to_definition_fallback`
- Default: `"find_all_references"`

**Options**

1. Do nothing:

```json [settings]
{
  "go_to_definition_fallback": "none"
}
```

2. Find references for the same symbol (default):

```json [settings]
{
  "go_to_definition_fallback": "find_all_references"
}
```

## Hard Tabs

- Description: Whether to indent lines using tab characters or multiple spaces.
- Setting: `hard_tabs`
- Default: `false`

**Options**

`boolean` values

## Helix Mode

- Description: Whether or not to enable Helix mode. Enabling `helix_mode` also enables `vim_mode`. See the [Helix documentation](./helix.md) for more details.
- Setting: `helix_mode`
- Default: `false`

**Options**

`boolean` values

## Indent Guides

- Description: Configuration related to indent guides. Indent guides can be configured separately for each language.
- Setting: `indent_guides`
- Default:

```json [settings]
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

```json [settings]
{
  "indent_guides": {
    "enabled": false
  }
}
```

2. Enable indent guides for a specific language.

```json [settings]
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

```json [settings]
{
  "indent_guides": {
    "enabled": true,
    "coloring": "indent_aware"
  }
}
```

4. Enable indent aware background coloring ("rainbow indentation").
   The colors that are used for different indentation levels are defined in the theme (theme key: `accents`). They can be customized by using theme overrides.

```json [settings]
{
  "indent_guides": {
    "enabled": true,
    "coloring": "indent_aware",
    "background_coloring": "indent_aware"
  }
}
```

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

```json [settings]
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

```json [settings]
{
  "mode": "dark"
}
```

2. Set the icon theme to light mode

```json [settings]
{
  "mode": "light"
}
```

3. Set the icon theme to system mode

```json [settings]
{
  "mode": "system"
}
```

### Dark

- Description: The name of the dark icon theme.
- Setting: `dark`
- Default: `Zed (Default)`

**Options**

Run the {#action icon_theme_selector::Toggle} action in the command palette to see a current list of valid icon themes names.

### Light

- Description: The name of the light icon theme.
- Setting: `light`
- Default: `Zed (Default)`

**Options**

Run the {#action icon_theme_selector::Toggle} action in the command palette to see a current list of valid icon themes names.

## Image Viewer

- Description: Settings for image viewer functionality
- Setting: `image_viewer`
- Default:

```json [settings]
{
  "image_viewer": {
    "unit": "binary"
  }
}
```

**Options**

### Unit

- Description: The unit for image file sizes
- Setting: `unit`
- Default: `"binary"`

**Options**

1. Use binary units (KiB, MiB):

```json [settings]
{
  "image_viewer": {
    "unit": "binary"
  }
}
```

2. Use decimal units (KB, MB):

```json [settings]
{
  "image_viewer": {
    "unit": "decimal"
  }
}
```

## Inlay hints

- Description: Configuration for displaying extra text with hints in the editor.
- Setting: `inlay_hints`
- Default:

```json [settings]
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
- [TypeScript](https://docs.zed.dev/languages/typescript)

Use the `lsp` section for the server configuration. Examples are provided in the corresponding language documentation.

Hints are not instantly queried in Zed, two kinds of debounces are used, either may be set to 0 to be disabled.
Settings-related hint updates are not debounced.

All possible config values for `toggle_on_modifiers_press` are:

```json [settings]
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

```json [settings]
"journal": {
  "path": "~",
  "hour_format": "hour12"
}

```

### Path

- Description: The path of the directory where journal entries are stored. If an invalid path is specified, the journal will fall back to using `~` (the home directory).
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

```json [settings]
{
  "hour_format": "hour12"
}
```

2. 24-hour format:

```json [settings]
{
  "hour_format": "hour24"
}
```

## JSX Tag Auto Close

- Description: Whether to automatically close JSX tags
- Setting: `jsx_tag_auto_close`
- Default:

```json [settings]
{
  "jsx_tag_auto_close": {
    "enabled": true
  }
}
```

**Options**

- `enabled`: Whether to enable automatic JSX tag closing

## Languages

- Description: Configuration for specific languages.
- Setting: `languages`
- Default: `null`

**Options**

To override settings for a language, add an entry for that languages name to the `languages` value. Example:

```json [settings]
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
- [`whitespace_map`](#whitespace-map)
- [`soft_wrap`](#soft-wrap)
- [`tab_size`](#tab-size)
- [`use_autoclose`](#use-autoclose)
- [`always_treat_brackets_as_autoclosed`](#always-treat-brackets-as-autoclosed)

These values take in the same options as the root-level settings with the same name.

## Language Models

- Description: Configuration for language model providers
- Setting: `language_models`
- Default:

```json [settings]
{
  "language_models": {
    "anthropic": {
      "api_url": "https://api.anthropic.com"
    },
    "google": {
      "api_url": "https://generativelanguage.googleapis.com"
    },
    "ollama": {
      "api_url": "http://localhost:11434"
    },
    "openai": {
      "api_url": "https://api.openai.com/v1"
    }
  }
}
```

**Options**

Configuration for various AI model providers including API URLs and authentication settings.

## Line Indicator Format

- Description: Format for line indicator in the status bar
- Setting: `line_indicator_format`
- Default: `"short"`

**Options**

1. Short format:

```json [settings]
{
  "line_indicator_format": "short"
}
```

2. Long format:

```json [settings]
{
  "line_indicator_format": "long"
}
```

## Linked Edits

- Description: Whether to perform linked edits of associated ranges, if the language server supports it. For example, when editing opening `<html>` tag, the contents of the closing `</html>` tag will be edited as well.
- Setting: `linked_edits`
- Default: `true`

**Options**

`boolean` values

## LSP Document Colors

- Description: Whether to show document color information from the language server
- Setting: `lsp_document_colors`
- Default: `true`

**Options**

`boolean` values

## Max Tabs

- Description: Maximum number of tabs to show in the tab bar
- Setting: `max_tabs`
- Default: `null`

**Options**

Positive `integer` values or `null` for unlimited tabs

## Middle Click Paste (Linux only)

- Description: Enable middle-click paste on Linux
- Setting: `middle_click_paste`
- Default: `true`

**Options**

`boolean` values

## Multi Cursor Modifier

- Description: Determines the modifier to be used to add multiple cursors with the mouse. The open hover link mouse gestures will adapt such that it do not conflict with the multicursor modifier.
- Setting: `multi_cursor_modifier`
- Default: `alt`

**Options**

1. Maps to `Alt` on Linux and Windows and to `Option` on macOS:

```json [settings]
{
  "multi_cursor_modifier": "alt"
}
```

2. Maps `Control` on Linux and Windows and to `Command` on macOS:

```json [settings]
{
  "multi_cursor_modifier": "cmd_or_ctrl" // alias: "cmd", "ctrl"
}
```

## Node

- Description: Configuration for Node.js integration
- Setting: `node`
- Default:

```json [settings]
{
  "node": {
    "ignore_system_version": false,
    "path": null,
    "npm_path": null
  }
}
```

**Options**

- `ignore_system_version`: Whether to ignore the system Node.js version
- `path`: Custom path to Node.js binary
- `npm_path`: Custom path to npm binary

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

By default no proxy will be used, or Zed will attempt to retrieve proxy settings from environment variables, such as `http_proxy`, `HTTP_PROXY`, `https_proxy`, `HTTPS_PROXY`, `all_proxy`, `ALL_PROXY`, `no_proxy` and `NO_PROXY`.

For example, to set an `http` proxy, add the following to your settings:

```json [settings]
{
  "proxy": "http://127.0.0.1:10809"
}
```

Or to set a `socks5` proxy:

```json [settings]
{
  "proxy": "socks5h://localhost:10808"
}
```

If you wish to exclude certain hosts from using the proxy, set the `NO_PROXY` environment variable. This accepts a comma-separated list of hostnames, host suffixes, IPv4/IPv6 addresses or blocks that should not use the proxy. For example if your environment included `NO_PROXY="google.com, 192.168.1.0/24"` all hosts in `192.168.1.*`, `google.com` and `*.google.com` would bypass the proxy. See [reqwest NoProxy docs](https://docs.rs/reqwest/latest/reqwest/struct.NoProxy.html#method.from_string) for more.

## On Last Window Closed

- Description: What to do when the last window is closed
- Setting: `on_last_window_closed`
- Default: `"platform_default"`

**Options**

1. Use platform default behavior:

```json [settings]
{
  "on_last_window_closed": "platform_default"
}
```

2. Always quit the application:

```json [settings]
{
  "on_last_window_closed": "quit_app"
}
```

## Profiles

- Description: Configuration profiles that can be applied on top of existing settings
- Setting: `profiles`
- Default: `{}`

**Options**

Configuration object for defining settings profiles. Example:

```json [settings]
{
  "profiles": {
    "presentation": {
      "buffer_font_size": 20,
      "ui_font_size": 18,
      "theme": "One Light"
    }
  }
}
```

## Preview tabs

- Description:
  Preview tabs allow you to open files in preview mode, where they close automatically when you switch to another file unless you explicitly pin them. This is useful for quickly viewing files without cluttering your workspace. Preview tabs display their file names in italics. \
   There are several ways to convert a preview tab into a regular tab:

  - Double-clicking on the file
  - Double-clicking on the tab header
  - Using the {#action project_panel::OpenPermanent} action
  - Editing the file
  - Dragging the file to a different pane

- Setting: `preview_tabs`
- Default:

```json [settings]
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

## Pane Split Direction Horizontal

- Description: The direction that you want to split panes horizontally
- Setting: `pane_split_direction_horizontal`
- Default: `"up"`

**Options**

1. Split upward:

```json [settings]
{
  "pane_split_direction_horizontal": "up"
}
```

2. Split downward:

```json [settings]
{
  "pane_split_direction_horizontal": "down"
}
```

## Pane Split Direction Vertical

- Description: The direction that you want to split panes vertically
- Setting: `pane_split_direction_vertical`
- Default: `"left"`

**Options**

1. Split to the left:

```json [settings]
{
  "pane_split_direction_vertical": "left"
}
```

2. Split to the right:

```json [settings]
{
  "pane_split_direction_vertical": "right"
}
```

## Preferred Line Length

- Description: The column at which to soft-wrap lines, for buffers where soft-wrap is enabled.
- Setting: `preferred_line_length`
- Default: `80`

**Options**

`integer` values

## Private Files

- Description: Globs to match against file paths to determine if a file is private
- Setting: `private_files`
- Default: `["**/.env*", "**/*.pem", "**/*.key", "**/*.cert", "**/*.crt", "**/secrets.yml"]`

**Options**

List of `string` glob patterns

## Projects Online By Default

- Description: Whether or not to show the online projects view by default.
- Setting: `projects_online_by_default`
- Default: `true`

**Options**

`boolean` values

## Read SSH Config

- Description: Whether to read SSH configuration files
- Setting: `read_ssh_config`
- Default: `true`

**Options**

`boolean` values

## Redact Private Values

- Description: Hide the values of variables from visual display in private files
- Setting: `redact_private_values`
- Default: `false`

**Options**

`boolean` values

## Relative Line Numbers

- Description: Whether to show relative line numbers in the gutter
- Setting: `relative_line_numbers`
- Default: `"disabled"`

**Options**

1. Show relative line numbers in the gutter whilst counting wrapped lines as one line:

```json [settings]
{
  "relative_line_numbers": "enabled"
}
```

2. Show relative line numbers in the gutter, including wrapped lines in the counting:

```json [settings]
{
  "relative_line_numbers": "wrapped"
}
```

2. Do not use relative line numbers:

```json [settings]
{
  "relative_line_numbers": "disabled"
}
```

## Remove Trailing Whitespace On Save

- Description: Whether or not to remove any trailing whitespace from lines of a buffer before saving it.
- Setting: `remove_trailing_whitespace_on_save`
- Default: `true`

**Options**

`boolean` values

## Resize All Panels In Dock

- Description: Whether to resize all the panels in a dock when resizing the dock. Can be a combination of "left", "right" and "bottom".
- Setting: `resize_all_panels_in_dock`
- Default: `["left"]`

**Options**

List of strings containing any combination of:

- `"left"`: Resize left dock panels together
- `"right"`: Resize right dock panels together
- `"bottom"`: Resize bottom dock panels together

## Restore on File Reopen

- Description: Whether to attempt to restore previous file's state when opening it again. The state is stored per pane.
- Setting: `restore_on_file_reopen`
- Default: `true`

**Options**

`boolean` values

## Restore on Startup

- Description: Controls session restoration on startup.
- Setting: `restore_on_startup`
- Default: `last_session`

**Options**

1. Restore all workspaces that were open when quitting Zed:

```json [settings]
{
  "restore_on_startup": "last_session"
}
```

2. Restore the workspace that was closed last:

```json [settings]
{
  "restore_on_startup": "last_workspace"
}
```

3. Always start with an empty editor:

```json [settings]
{
  "restore_on_startup": "none"
}
```

## Scroll Beyond Last Line

- Description: Whether the editor will scroll beyond the last line
- Setting: `scroll_beyond_last_line`
- Default: `"one_page"`

**Options**

1. Scroll one page beyond the last line by one page:

```json [settings]
{
  "scroll_beyond_last_line": "one_page"
}
```

2. The editor will scroll beyond the last line by the same amount of lines as `vertical_scroll_margin`:

```json [settings]
{
  "scroll_beyond_last_line": "vertical_scroll_margin"
}
```

3. The editor will not scroll beyond the last line:

```json [settings]
{
  "scroll_beyond_last_line": "off"
}
```

**Options**

`boolean` values

## Scroll Sensitivity

- Description: Scroll sensitivity multiplier. This multiplier is applied to both the horizontal and vertical delta values while scrolling.
- Setting: `scroll_sensitivity`
- Default: `1.0`

**Options**

Positive `float` values

### Fast Scroll Sensitivity

- Description: Scroll sensitivity multiplier for fast scrolling. This multiplier is applied to both the horizontal and vertical delta values while scrolling. Fast scrolling happens when a user holds the alt or option key while scrolling.
- Setting: `fast_scroll_sensitivity`
- Default: `4.0`

**Options**

Positive `float` values

### Horizontal Scroll Margin

- Description: The number of characters to keep on either side when scrolling with the mouse
- Setting: `horizontal_scroll_margin`
- Default: `5`

**Options**

Non-negative `integer` values

### Vertical Scroll Margin

- Description: The number of lines to keep above/below the cursor when scrolling with the keyboard
- Setting: `vertical_scroll_margin`
- Default: `3`

**Options**

Non-negative `integer` values

## Search

- Description: Search options to enable by default when opening new project and buffer searches.
- Setting: `search`
- Default:

```json [settings]
"search": {
  "button": true,
  "whole_word": false,
  "case_sensitive": false,
  "include_ignored": false,
  "regex": false,
  "center_on_match": false
},
```

### Button

- Description: Whether to show the project search button in the status bar.
- Setting: `button`
- Default: `true`

### Whole Word

- Description: Whether to only match on whole words.
- Setting: `whole_word`
- Default: `false`

### Case Sensitive

- Description: Whether to match case sensitively. This setting affects both
  searches and editor actions like "Select Next Occurrence", "Select Previous
  Occurrence", and "Select All Occurrences".
- Setting: `case_sensitive`
- Default: `false`

### Include Ignore

- Description: Whether to include gitignored files in search results.
- Setting: `include_ignored`
- Default: `false`

### Regex

- Description: Whether to interpret the search query as a regular expression.
- Setting: `regex`
- Default: `false`

### Center On Match

- Description: Whether to center the cursor on each search match when navigating.
- Setting: `center_on_match`
- Default: `false`

## Search Wrap

- Description: If `search_wrap` is disabled, search result do not wrap around the end of the file
- Setting: `search_wrap`
- Default: `true`

## Center on Match

- Description: If `center_on_match` is enabled, the editor will center the cursor on the current match when searching.
- Setting: `center_on_match`
- Default: `false`

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

```json [settings]
{
  "completions": {
    "words": "fallback",
    "words_min_length": 3,
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

### Min Words Query Length

- Description: Minimum number of characters required to automatically trigger word-based completions.
  Before that value, it's still possible to trigger the words-based completion manually with the corresponding editor command.
- Setting: `words_min_length`
- Default: `3`

**Options**

Positive integer values

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

## Whitespace Map

- Description: Specify the characters used to render whitespace when show_whitespaces is enabled.
- Setting: `whitespace_map`
- Default:

```json [settings]
{
  "whitespace_map": {
    "space": "•",
    "tab": "→"
  }
}
```

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

## Show Wrap Guides

- Description: Whether to show wrap guides (vertical rulers) in the editor. Setting this to true will show a guide at the 'preferred_line_length' value if 'soft_wrap' is set to 'preferred_line_length', and will show any additional guides as specified by the 'wrap_guides' setting.
- Setting: `show_wrap_guides`
- Default: `true`

**Options**

`boolean` values

## Use On Type Format

- Description: Whether to use additional LSP queries to format (and amend) the code after every "trigger" symbol input, defined by LSP server capabilities
- Setting: `use_on_type_format`
- Default: `true`

**Options**

`boolean` values

## Use Auto Surround

- Description: Whether to automatically surround selected text when typing opening parenthesis, bracket, brace, single or double quote characters. For example, when you select text and type '(', Zed will surround the text with ().
- Setting: `use_auto_surround`
- Default: `true`

**Options**

`boolean` values

## Use System Path Prompts

- Description: Whether to use the system provided dialogs for Open and Save As. When set to false, Zed will use the built-in keyboard-first pickers.
- Setting: `use_system_path_prompts`
- Default: `true`

**Options**

`boolean` values

## Use System Prompts

- Description: Whether to use the system provided dialogs for prompts, such as confirmation prompts. When set to false, Zed will use its built-in prompts. Note that on Linux, this option is ignored and Zed will always use the built-in prompts.
- Setting: `use_system_prompts`
- Default: `true`

**Options**

`boolean` values

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

## Tasks

- Description: Configuration for tasks that can be run within Zed
- Setting: `tasks`
- Default:

```json [settings]
{
  "tasks": {
    "variables": {},
    "enabled": true,
    "prefer_lsp": false
  }
}
```

**Options**

- `variables`: Custom variables for task configuration
- `enabled`: Whether tasks are enabled
- `prefer_lsp`: Whether to prefer LSP-provided tasks over Zed language extension ones

## Telemetry

- Description: Control what info is collected by Zed.
- Setting: `telemetry`
- Default:

```json [settings]
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

```json [settings]
{
  "terminal": {
    "alternate_scroll": "off",
    "blinking": "terminal_controlled",
    "copy_on_select": false,
    "keep_selection_on_copy": true,
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
    "minimum_contrast": 45,
    "option_as_meta": false,
    "button": true,
    "shell": "system",
    "scroll_multiplier": 3.0,
    "toolbar": {
      "breadcrumbs": false
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

```json [settings]
{
  "terminal": {
    "alternate_scroll": "off"
  }
}
```

2. Default alternate scroll mode to on

```json [settings]
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

```json [settings]
{
  "terminal": {
    "blinking": "off"
  }
}
```

2. Default the cursor blink to off, but allow the terminal to turn blinking on

```json [settings]
{
  "terminal": {
    "blinking": "terminal_controlled"
  }
}
```

3. Always blink the cursor, ignore the terminal mode

```json [settings]
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

```json [settings]
{
  "terminal": {
    "copy_on_select": true
  }
}
```

### Terminal: Cursor Shape

- Description: Controls the visual shape of the cursor in the terminal. When not explicitly set, it defaults to a block shape.
- Setting: `cursor_shape`
- Default: `null` (defaults to block)

**Options**

1. A block that surrounds the following character

```json [settings]
{
  "terminal": {
    "cursor_shape": "block"
  }
}
```

2. A vertical bar

```json [settings]
{
  "terminal": {
    "cursor_shape": "bar"
  }
}
```

3. An underline / underscore that runs along the following character

```json [settings]
{
  "terminal": {
    "cursor_shape": "underline"
  }
}
```

4. A box drawn around the following character

```json [settings]
{
  "terminal": {
    "cursor_shape": "hollow"
  }
}
```

### Terminal: Keep Selection On Copy

- Description: Whether or not to keep the selection in the terminal after copying text.
- Setting: `keep_selection_on_copy`
- Default: `true`

**Options**

`boolean` values

**Example**

```json [settings]
{
  "terminal": {
    "keep_selection_on_copy": false
  }
}
```

### Terminal: Env

- Description: Any key-value pairs added to this object will be added to the terminal's environment. Keys must be unique, use `:` to separate multiple values in a single variable
- Setting: `env`
- Default: `{}`

**Example**

```json [settings]
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

```json [settings]
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

```json [settings]
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

```json [settings]
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
- Default: `standard`

**Options**

1. Use a line height that's `comfortable` for reading, 1.618.

```json [settings]
{
  "terminal": {
    "line_height": "comfortable"
  }
}
```

2. Use a `standard` line height, 1.3. This option is useful for TUIs, particularly if they use box characters. (default)

```json [settings]
{
  "terminal": {
    "line_height": "standard"
  }
}
```

3.  Use a custom line height.

```json [settings]
{
  "terminal": {
    "line_height": {
      "custom": 2
    }
  }
}
```

### Terminal: Minimum Contrast

- Description: Controls the minimum contrast between foreground and background colors in the terminal. Uses the APCA (Accessible Perceptual Contrast Algorithm) for color adjustments. Set this to 0 to disable this feature.
- Setting: `minimum_contrast`
- Default: `45`

**Options**

`integer` values from 0 to 106. Common recommended values:

- `0`: No contrast adjustment
- `45`: Minimum for large fluent text (default)
- `60`: Minimum for other content text
- `75`: Minimum for body text
- `90`: Preferred for body text

```json [settings]
{
  "terminal": {
    "minimum_contrast": 45
  }
}
```

### Terminal: Option As Meta

- Description: Re-interprets the option keys to act like a 'meta' key, like in Emacs.
- Setting: `option_as_meta`
- Default: `false`

**Options**

`boolean` values

```json [settings]
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

```json [settings]
{
  "terminal": {
    "shell": "system"
  }
}
```

2. A program to launch:

```json [settings]
{
  "terminal": {
    "shell": {
      "program": "sh"
    }
  }
}
```

3. A program with arguments:

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

## Terminal: Detect Virtual Environments {#terminal-detect_venv}

- Description: Activate the [Python Virtual Environment](https://docs.python.org/3/library/venv.html), if one is found, in the terminal's working directory (as resolved by the working_directory and automatically activating the virtual environment.
- Setting: `detect_venv`
- Default:

```json [settings]
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

```json [settings]
{
  "terminal": {
    "detect_venv": "off"
  }
}
```

### Terminal: Scroll Multiplier

- Description: The multiplier for scrolling speed in the terminal when using mouse wheel or trackpad.
- Setting: `scroll_multiplier`
- Default: `1.0`

**Options**

Positive floating point values. Values less than or equal to 0 will be clamped to a minimum of 0.01.

**Example**

```json
{
  "terminal": {
    "scroll_multiplier": 5.0
  }
}
```

## Terminal: Toolbar

- Description: Whether or not to show various elements in the terminal toolbar.
- Setting: `toolbar`
- Default:

```json [settings]
{
  "terminal": {
    "toolbar": {
      "breadcrumbs": false
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

```json [settings]
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

```json [settings]
{
  "terminal": {
    "working_directory": "current_project_directory"
  }
}
```

2. Use the first project in this workspace's directory. Will fallback to using this platform's home directory.

```json [settings]
{
  "terminal": {
    "working_directory": "first_project_directory"
  }
}
```

3. Always use this platform's home directory (if we can find it)

```json [settings]
{
  "terminal": {
    "working_directory": "always_home"
  }
}
```

4. Always use a specific directory. This value will be shell expanded. If this path is not a valid directory the terminal will default to this platform's home directory.

```json [settings]
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

## REPL

- Description: Repl settings.
- Setting: `repl`
- Default:

```json [settings]
"repl": {
  // Maximum number of columns to keep in REPL's scrollback buffer.
  // Clamped with [20, 512] range.
  "max_columns": 128,
  // Maximum number of lines to keep in REPL's scrollback buffer.
  // Clamped with [4, 256] range.
  "max_lines": 32
},
```

## Theme

- Description: The theme setting can be specified in two forms - either as the name of a theme or as an object containing the `mode`, `dark`, and `light` themes for the Zed UI.
- Setting: `theme`
- Default: `One Dark`

### Theme Object

- Description: Specify the theme using an object that includes the `mode`, `dark`, and `light` themes.
- Setting: `theme`
- Default:

```json [settings]
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

```json [settings]
{
  "mode": "dark"
}
```

2. Set the theme to light mode

```json [settings]
{
  "mode": "light"
}
```

3. Set the theme to system mode

```json [settings]
{
  "mode": "system"
}
```

### Dark

- Description: The name of the dark Zed theme to use for the UI.
- Setting: `dark`
- Default: `One Dark`

**Options**

Run the {#action theme_selector::Toggle} action in the command palette to see a current list of valid themes names.

### Light

- Description: The name of the light Zed theme to use for the UI.
- Setting: `light`
- Default: `One Light`

**Options**

Run the {#action theme_selector::Toggle} action in the command palette to see a current list of valid themes names.

## Title Bar

- Description: Whether or not to show various elements in the title bar
- Setting: `title_bar`
- Default:

```json [settings]
"title_bar": {
  "show_branch_icon": false,
  "show_branch_name": true,
  "show_project_items": true,
  "show_onboarding_banner": true,
  "show_user_picture": true,
  "show_sign_in": true,
  "show_menus": false
}
```

**Options**

- `show_branch_icon`: Whether to show the branch icon beside branch switcher in the titlebar
- `show_branch_name`: Whether to show the branch name button in the titlebar
- `show_project_items`: Whether to show the project host and name in the titlebar
- `show_onboarding_banner`: Whether to show onboarding banners in the titlebar
- `show_user_picture`: Whether to show user picture in the titlebar
- `show_sign_in`: Whether to show the sign in button in the titlebar
- `show_menus`: Whether to show the menus in the titlebar

## Vim

- Description: Whether or not to enable vim mode.
- Setting: `vim_mode`
- Default: `false`

## When Closing With No Tabs

- Description: Whether the window should be closed when using 'close active item' on a window with no tabs
- Setting: `when_closing_with_no_tabs`
- Default: `"platform_default"`

**Options**

1. Use platform default behavior:

```json [settings]
{
  "when_closing_with_no_tabs": "platform_default"
}
```

2. Always close the window:

```json [settings]
{
  "when_closing_with_no_tabs": "close_window"
}
```

3. Never close the window:

```json [settings]
{
  "when_closing_with_no_tabs": "keep_window_open"
}
```

## Project Panel

- Description: Customize project panel
- Setting: `project_panel`
- Default:

```json [settings]
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
    "drag_and_drop": true,
    "scrollbar": {
      "show": null
    },
    "sticky_scroll": true,
    "show_diagnostics": "all",
    "indent_guides": {
      "show": "always"
    },
    "sort_mode": "directories_first",
    "hide_root": false,
    "hide_hidden": false,
    "starts_open": true,
    "auto_open": {
      "on_create": true,
      "on_paste": true,
      "on_drop": true
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

```json [settings]
{
  "dock": "left"
}
```

2. Default dock position to right

```json [settings]
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

```json [settings]
{
  "entry_spacing": "comfortable"
}
```

2. Standard entry spacing

```json [settings]
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

```json [settings]
{
  "git_status": true
}
```

2. Default disable git status

```json [settings]
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

```json [settings]
{
  "auto_reveal_entries": true
}
```

2. Disable auto reveal entries

```json [settings]
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

```json [settings]
{
  "auto_fold_dirs": true
}
```

2. Disable auto fold dirs

```json [settings]
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

```json [settings]
"indent_guides": {
  "show": "always"
}
```

**Options**

1. Show indent guides in the project panel

```json [settings]
{
  "indent_guides": {
    "show": "always"
  }
}
```

2. Hide indent guides in the project panel

```json [settings]
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

```json [settings]
"scrollbar": {
  "show": null
}
```

**Options**

1. Show scrollbar in the project panel

```json [settings]
{
  "scrollbar": {
    "show": "always"
  }
}
```

2. Hide scrollbar in the project panel

```json [settings]
{
  "scrollbar": {
    "show": "never"
  }
}
```

### Sort Mode

- Description: Sort order for entries in the project panel
- Setting: `sort_mode`
- Default: `directories_first`

**Options**

1. Show directories first, then files

```json [settings]
{
  "sort_mode": "directories_first"
}
```

2. Mix directories and files together

```json [settings]
{
  "sort_mode": "mixed"
}
```

3. Show files first, then directories

```json [settings]
{
  "sort_mode": "files_first"
}
```

### Auto Open

- Description: Control whether files are opened automatically after different creation flows in the project panel.
- Setting: `auto_open`
- Default:

```json [settings]
"auto_open": {
  "on_create": true,
  "on_paste": true,
  "on_drop": true
}
```

**Options**

- `on_create`: Whether to automatically open newly created files in the editor.
- `on_paste`: Whether to automatically open files after pasting or duplicating them.
- `on_drop`: Whether to automatically open files dropped from external sources.

## Agent

Visit [the Configuration page](./ai/configuration.md) under the AI section to learn more about all the agent-related settings.

## Collaboration Panel

- Description: Customizations for the collaboration panel.
- Setting: `collaboration_panel`
- Default:

```json [settings]
{
  "collaboration_panel": {
    "button": true,
    "dock": "left",
    "default_width": 240
  }
}
```

**Options**

- `button`: Whether to show the collaboration panel button in the status bar
- `dock`: Where to dock the collaboration panel. Can be `left` or `right`
- `default_width`: Default width of the collaboration panel

## Debugger

- Description: Configuration for debugger panel and settings
- Setting: `debugger`
- Default:

```json [settings]
{
  "debugger": {
    "stepping_granularity": "line",
    "save_breakpoints": true,
    "dock": "bottom",
    "button": true
  }
}
```

See the [debugger page](./debugger.md) for more information about debugging support within Zed.

## Git Panel

- Description: Setting to customize the behavior of the git panel.
- Setting: `git_panel`
- Default:

```json [settings]
{
  "git_panel": {
    "button": true,
    "dock": "left",
    "default_width": 360,
    "status_style": "icon",
    "fallback_branch_name": "main",
    "sort_by_path": false,
    "collapse_untracked_diff": false,
    "scrollbar": {
      "show": null
    }
  }
}
```

**Options**

- `button`: Whether to show the git panel button in the status bar
- `dock`: Where to dock the git panel. Can be `left` or `right`
- `default_width`: Default width of the git panel
- `status_style`: How to display git status. Can be `label_color` or `icon`
- `fallback_branch_name`: What branch name to use if `init.defaultBranch` is not set
- `sort_by_path`: Whether to sort entries in the panel by path or by status (the default)
- `collapse_untracked_diff`: Whether to collapse untracked files in the diff panel
- `scrollbar`: When to show the scrollbar in the git panel

## Outline Panel

- Description: Customize outline Panel
- Setting: `outline_panel`
- Default:

```json [settings]
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

```json [settings]
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

```json [settings]
{
  "unnecessary_code_fade": 0.5
}
```

## UI Font Family

- Description: The name of the font to use for text in the UI.
- Setting: `ui_font_family`
- Default: `.ZedSans`. This currently aliases to [IBM Plex](https://www.ibm.com/plex/).

**Options**

The name of any font family installed on the system, `".ZedSans"` to use the Zed-provided default, or `".SystemUIFont"` to use the system's default UI font (on macOS and Windows).

## UI Font Features

- Description: The OpenType features to enable for text in the UI.
- Setting: `ui_font_features`
- Default:

```json [settings]
"ui_font_features": {
  "calt": false
}
```

- Platform: macOS and Windows.

**Options**

Zed supports all OpenType features that can be enabled or disabled for a given UI font, as well as setting values for font features.

For example, to disable font ligatures, add the following to your settings:

```json [settings]
{
  "ui_font_features": {
    "calt": false
  }
}
```

You can also set other OpenType features, like setting `cv01` to `7`:

```json [settings]
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

```json [settings]
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

## Settings Profiles

- Description: Configure any number of settings profiles that are temporarily applied on top of your existing user settings when selected from `settings profile selector: toggle`.
- Setting: `profiles`
- Default: `{}`

In your `settings.json` file, add the `profiles` object.
Each key within this object is the name of a settings profile, and each value is an object that can include any of Zed's settings.

Example:

```json [settings]
"profiles": {
  "Presenting (Dark)": {
    "agent_buffer_font_size": 18.0,
    "buffer_font_size": 18.0,
    "theme": "One Dark",
    "ui_font_size": 18.0
  },
  "Presenting (Light)": {
    "agent_buffer_font_size": 18.0,
    "buffer_font_size": 18.0,
    "theme": "One Light",
    "ui_font_size": 18.0
  },
  "Writing": {
    "agent_buffer_font_size": 15.0,
    "buffer_font_size": 15.0,
    "theme": "Catppuccin Frappé - No Italics",
    "ui_font_size": 15.0,
    "tab_bar": { "show": false },
    "toolbar": { "breadcrumbs": false }
  }
}
```

To preview and enable a settings profile, open the command palette via {#kb command_palette::Toggle} and search for `settings profile selector: toggle`.

## An example configuration:

```json [settings]
// ~/.config/zed/settings.json
{
  "theme": "cave-light",
  "tab_size": 2,
  "preferred_line_length": 80,
  "soft_wrap": "none",

  "buffer_font_size": 18,
  "buffer_font_family": ".ZedMono",

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
      "format_on_save": "on",
      "formatter": "language_server",
      "preferred_line_length": 64,
      "soft_wrap": "preferred_line_length"
    }
  }
}
```
