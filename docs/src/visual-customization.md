# Visual Customization

Various aspects of Zed's visual layout can be configured via Zed settings.json which you can access via {#action zed::OpenSettings} ({#kb zed::OpenSettings}).

See [Configuring Zed](./configuring-zed.md) for additional information and other non-visual settings.

## Themes

Use may install zed extensions providing [Themes](./themes.md) and [Icon Themes](./icon-themes.md) via {#action zed::Extensions} from the command palette or menu.

You can preview/choose amongst your installed themes and icon themes with {#action theme_selector::Toggle} ({#kb theme_selector::Toggle}) and ({#action icon_theme_selector::Toggle}) which will modify the following settings:

```json
{
  "theme": "One Dark",
  "icon_theme": "Zed (Default)"
}
```

If you would like to use distinct themes for light mode/dark mode that can be set with:

```json
{
  "theme": {
    "dark": "One Dark"
    "light": "One Light",
    // Mode to use (dark, light) or "system" to follow the OS's light/dark mode (default)
    "mode": "system",
  },
  "icon_theme": {
    "dark": "Zed (Default)"
    "light": "Zed (Default)",
    // Mode to use (dark, light) or "system" to follow the OS's light/dark mode (default)
    "mode": "system",
  }
}
```

## Fonts

```json
  // UI Font. Use ".SystemUIFont" to use the default system font (SF Pro on macOS),
  // or ".ZedSans" for the bundled default (currently IBM Plex)
  "ui_font_family": ".SystemUIFont",
  "ui_font_weight": 400, // Font weight in standard CSS units from 100 to 900.
  "ui_font_size": 16,

  // Buffer Font - Used by editor buffers
  // use ".ZedMono" for the bundled default monospace (currently Lilex)
  "buffer_font_family": "Berkeley Mono", // Font name for editor buffers
  "buffer_font_size": 15,                 // Font size for editor buffers
  "buffer_font_weight": 400,              // Font weight in CSS units [100-900]
  // Line height "comfortable" (1.618), "standard" (1.3) or custom: `{ "custom": 2 }`
  "buffer_line_height": "comfortable",

  // Terminal Font Settings
  "terminal": {
    "font_family": "",
    "font_size": 15,
    // Terminal line height: comfortable (1.618), standard(1.3) or `{ "custom": 2 }`
    "line_height": "comfortable",
  },

  // Agent Panel Font Settings
  "agent_font_size": 15
```

### Font ligatures

By default Zed enable font ligatures which will visually combines certain adjacent characters.

For example `=>` will be displayed as `→` and `!=` will be `≠`. This is purely cosmetic and the individual characters remain unchanged.

To disable this behavior use:

```json
{
  "buffer_font_features": {
    "calt": false // Disable ligatures
  }
}
```

### Status Bar

```json
{
  // Whether to show full labels in line indicator or short ones
  //   - `short`: "2 s, 15 l, 32 c"
  //   - `long`: "2 selections, 15 lines, 32 characters"
  "line_indicator_format": "long"

  // Individual status bar icons can be hidden:
  // "project_panel": {"button": false },
  // "outline_panel": {"button": false },
  // "collaboration_panel": {"button": false },
  // "git_panel": {"button": false },
  // "notification_panel": {"button": false },
  // "agent": {"button": false },
  // "debugger": {"button": false },
  // "diagnostics": {"button": false },
  // "search": {"button": false },
}
```

### Titlebar

```json
  // Control which items are shown/hidden in the title bar
  "title_bar": {
    "show": "always",               // When to show: always | never | hide_in_full_screen
    "show_branch_icon": false,      // Show/hide branch icon beside branch switcher
    "show_branch_name": true,       // Show/hide branch name
    "show_project_items": true,     // Show/hide project host and name
    "show_onboarding_banner": true, // Show/hide onboarding banners
    "show_user_picture": true,      // Show/hide user avatar
    "show_sign_in": true,           // Show/hide sign-in button
    "show_menus": false             // Show/hide menus
  },
```

## Workspace

```json
{
  // Force usage of Zed build in path prompts (file and directory pickers)
  // instead of OS native pickers (false).
  "use_system_path_prompts": true,
  // Force usage of Zed built in confirmation prompts ("Do you want to save?")
  // instead of OS native prompts (false). On linux this is ignored (always false).
  "use_system_prompts": true,

  // Whether to use the system provided dialogs for Open and Save As (true) or
  // Zed's built-in keyboard-first pickers (false)
  "use_system_path_prompts": true,

  // Active pane styling settings.
  "active_pane_modifiers": {
    // Inset border size of the active pane, in pixels.
    "border_size": 0.0,
    // Opacity of the inactive panes. 0 means transparent, 1 means opaque.
    "inactive_opacity": 1.0
  },

  // Layout mode of the bottom dock: contained, full, left_aligned, right_aligned
  "bottom_dock_layout": "contained",

  // Whether to resize all the panels in a dock when resizing the dock.
  // Can be a combination of "left", "right" and "bottom".
  "resize_all_panels_in_dock": ["left"]
}
```

<!--
TBD: Centered layout related settings
```json
    "centered_layout": {
    // The relative width of the left padding of the central pane from the
    // workspace when the centered layout is used.
    "left_padding": 0.2,
    // The relative width of the right padding of the central pane from the
    // workspace when the centered layout is used.
    "right_padding": 0.2
    },
```
-->

## Editor

```json
  // Whether the cursor blinks in the editor.
  "cursor_blink": true,

  // Cursor shape for the default editor: bar, block, underline, hollow
  "cursor_shape": null,

  // Highlight the current line in the editor: none, gutter, line, all
  "current_line_highlight": "all",

  // When does the mouse cursor hide: never, on_typing, on_typing_and_movement
  "hide_mouse": "on_typing_and_movement",

  // Whether to highlight all occurrences of the selected text in an editor.
  "selection_highlight": true,

  // Visually show tabs and spaces  (none, all, selection, boundary, trailing)
  "show_whitespaces": "selection",
  "whitespace_map": { // Which characters to show when `show_whitespaces` enabled
    "space": "•",
    "tab": "→"
  },

  "unnecessary_code_fade": 0.3, // How much to fade out unused code.

  // Hide the values of in variables from visual display in private files
  "redact_private_values": false,

  // Soft-wrap and rulers
  "soft_wrap": "none",          // none, editor_width, preferred_line_length, bounded
  "preferred_line_length": 80,  // Column to soft-wrap
  "show_wrap_guides": true,     // Show/hide wrap guides (vertical rulers)
  "wrap_guides": [],            // Where to position wrap_guides (character counts)

  // Gutter Settings
  "gutter": {
    "line_numbers": true,         // Show/hide line numbers in the gutter.
    "runnables": true,            // Show/hide runnables buttons in the gutter.
    "breakpoints": true,          // Show/hide show breakpoints in the gutter.
    "folds": true,                // Show/hide show fold buttons in the gutter.
    "min_line_number_digits": 4   // Reserve space for N digit line numbers
  },
  "relative_line_numbers": false, // Show relative line numbers in gutter

  // Indent guides
  "indent_guides": {
    "enabled": true,
    "line_width": 1,                  // Width of guides in pixels [1-10]
    "active_line_width": 1,           // Width of active guide in pixels [1-10]
    "coloring": "fixed",              // disabled, fixed, indent_aware
    "background_coloring": "disabled" // disabled, indent_aware
  }
```

### Git Blame {#editor-blame}

```json
  "git": {
    "inline_blame": {
      "enabled": true,             // Show/hide inline blame
      "delay": 0,                  // Show after delay (ms)
      "min_column": 0,             // Minimum column to inline display blame
      "padding": 7,                // Padding between code and inline blame (em)
      "show_commit_summary": false // Show/hide commit summary
    },
    "hunk_style": "staged_hollow"  // staged_hollow, unstaged_hollow
  }
```

### Editor Toolbar

```json
  // Editor toolbar related settings
  "toolbar": {
    "breadcrumbs": true, // Whether to show breadcrumbs.
    "quick_actions": true, // Whether to show quick action buttons.
    "selections_menu": true, // Whether to show the Selections menu
    "agent_review": true, // Whether to show agent review buttons
    "code_actions": false // Whether to show code action buttons
  }
```

### Editor Scrollbar and Minimap {#editor-scrollbar}

```json
  // Scrollbar related settings
  "scrollbar": {
    // When to show the scrollbar in the editor (auto, system, always, never)
    "show": "auto",
    "cursors": true,          // Show cursor positions in the scrollbar.
    "git_diff": true,         // Show git diff indicators in the scrollbar.
    "search_results": true,   // Show buffer search results in the scrollbar.
    "selected_text": true,    // Show selected text occurrences in the scrollbar.
    "selected_symbol": true,  // Show selected symbol occurrences in the scrollbar.
    "diagnostics": "all",     // Show diagnostics (none, error, warning, information, all)
    "axes": {
      "horizontal": true,     // Show/hide the horizontal scrollbar
      "vertical": true        // Show/hide the vertical scrollbar
    }
  },

  // Minimap related settings
  "minimap": {
    "show": "never",                // When to show (auto, always, never)
    "display_in": "active_editor",  // Where to show (active_editor, all_editor)
    "thumb": "always",              // When to show thumb (always, hover)
    "thumb_border": "left_open",    // Thumb border (left_open, right_open, full, none)
    "max_width_columns": 80,        // Maximum width of minimap
    "current_line_highlight": null  // Highlight current line (null, line, gutter)
  },

  // Control Editor scroll beyond the last line: off, one_page, vertical_scroll_margin
  "scroll_beyond_last_line": "one_page",
  // Lines to keep above/below the cursor when scrolling with the keyboard
  "vertical_scroll_margin": 3,
  // The number of characters to keep on either side when scrolling with the mouse
  "horizontal_scroll_margin": 5,
  // Scroll sensitivity multiplier
  "scroll_sensitivity": 1.0,
  // Scroll sensitivity multiplier for fast scrolling (hold alt while scrolling)
  "fast_scroll_sensitivity": 4.0,
```

### Editor Tabs

```json
  // Maximum number of tabs per pane. Unset for unlimited.
  "max_tabs": null,

  // Customize the tab bar appearance
  "tab_bar": {
    "show": true,                     // Show/hide the tab bar
    "show_nav_history_buttons": true, // Show/hide history buttons on tab bar
    "show_tab_bar_buttons": true      // Show hide buttons (new, split, zoom)
  },
  "tabs": {
    "git_status": false,              // Color to show git status
    "close_position": "right",        // Close button position (left, right, hidden)
    "show_close_button": "hover",     // Close button shown (hover, always, hidden)
    "file_icons": false,              // Icon showing file type
    // Show diagnostics in file icon (off, errors, all). Requires file_icons=true
    "show_diagnostics": "off"
  }
```

### Status Bar

```json
  "status_bar": {
    // Show/hide a button that displays the active buffer's language.
    // Clicking the button brings up the language selector.
    // Defaults to true.
    "active_language_button": true,
    // Show/hide a button that displays the cursor's position.
    // Clicking the button brings up an input for jumping to a line and column.
    // Defaults to true.
    "cursor_position_button": true,
  },
  "global_lsp_settings": {
    // Show/hide the LSP button in the status bar.
    // Activity from the LSP is still shown.
    // Button is not shown if "enable_language_server" if false.
    "button": true
  },
```

### Multibuffer

```json
{
  // The default number of lines to expand excerpts in the multibuffer by.
  "expand_excerpt_lines": 5,
  // The default number of lines of context provided for excerpts in the multibuffer by.
  "excerpt_context_lines": 2
}
```

### Editor Completions, Snippets, Actions, Diagnostics {#editor-lsp}

```json
  "snippet_sort_order": "inline",        // Snippets completions: top, inline, bottom, none
  "show_completions_on_input": true,     // Show completions while typing
  "show_completion_documentation": true, // Show documentation in completions
  "auto_signature_help": false,          // Show method signatures inside parentheses

  // Whether to show the signature help after completion or a bracket pair inserted.
  // If `auto_signature_help` is enabled, this setting will be treated as enabled also.
  "show_signature_help_after_edits": false,

  // Whether to show code action button at start of buffer line.
  "inline_code_actions": true,

  // Which level to use to filter out diagnostics displayed in the editor:
  "diagnostics_max_severity": null,      // off, error, warning, info, hint, null (all)

  // How to render LSP `textDocument/documentColor` colors in the editor.
  "lsp_document_colors": "inlay",        // none, inlay, border, background
```

### Edit Predictions {#editor-ai}

```json
  "edit_predictions": {
    "mode": "eager",                // Automatically show (eager) or hold-alt (subtle)
    "enabled_in_text_threads": true // Show/hide predictions in agent text threads
  },
  "show_edit_predictions": true     // Show/hide predictions in editor
```

### Editor Inlay Hints

```json
{
  "inlay_hints": {
    "enabled": false,
    // Toggle certain types of hints on and off, all switched on by default.
    "show_type_hints": true,
    "show_parameter_hints": true,
    "show_other_hints": true,

    // Whether to show a background for inlay hints (theme `hint.background`)
    "show_background": false, //

    // Time to wait after editing before requesting hints (0 to disable debounce)
    "edit_debounce_ms": 700,
    // Time to wait after scrolling before requesting hints (0 to disable debounce)
    "scroll_debounce_ms": 50,

    // A set of modifiers which, when pressed, will toggle the visibility of inlay hints.
    "toggle_on_modifiers_press": {
      "control": false,
      "shift": false,
      "alt": false,
      "platform": false,
      "function": false
    }
  }
}
```

## File Finder

```json
  // File Finder Settings
  "file_finder": {
    "file_icons": true,         // Show/hide file icons
    "modal_max_width": "small", // Horizontal size: small, medium, large, xlarge, full
    "git_status": true,         // Show the git status for each entry
    "include_ignored": null     // gitignored files in results: true, false, null
  },
```

## Project Panel

Project panel can be shown/hidden with {#action project_panel::ToggleFocus} ({#kb project_panel::ToggleFocus}) or with {#action pane::RevealInProjectPanel} ({#kb pane::RevealInProjectPanel}).

```json
  // Project Panel Settings
  "project_panel": {
    "button": true,                 // Show/hide button in the status bar
    "default_width": 240,           // Default panel width
    "dock": "left",                 // Position of the dock (left, right)
    "entry_spacing": "comfortable", // Vertical spacing (comfortable, standard)
    "file_icons": true,             // Show/hide file icons
    "folder_icons": true,           // Show/hide folder icons
    "git_status": true,             // Indicate new/updated files
    "indent_size": 20,              // Pixels for each successive indent
    "auto_reveal_entries": true,    // Show file in panel when activating its buffer
    "auto_fold_dirs": true,         // Fold dirs with single subdir
    "sticky_scroll": true,          // Stick parent directories at top of the project panel.
    "drag_and_drop": true,          // Whether drag and drop is enabled
    "scrollbar": {                  // Project panel scrollbar settings
      "show": null                  // Show/hide: (auto, system, always, never)
    },
    "show_diagnostics": "all",      //
    // Settings related to indent guides in the project panel.
    "indent_guides": {
      // When to show indent guides in the project panel. (always, never)
      "show": "always"
    },
    // Whether to hide the root entry when only one folder is open in the window.
    "hide_root": false
  }.
```

## Agent Panel

```json
  "agent": {
    "version": "2",
    "enabled": true,        // Enable/disable the agent
    "button": true,         // Show/hide the icon in the status bar
    "dock": "right",        // Where to dock: left, right, bottom
    "default_width": 640,   // Default width (left/right docked)
    "default_height": 320,  // Default height (bottom docked)
  },
  "agent_font_size": 16
```

See [Zed AI Documentation](./ai/overview.md) for additional non-visual AI settings.

## Terminal Panel

```json
  // Terminal Panel Settings
  "terminal": {
    "dock": "bottom",                   // Where to dock: left, right, bottom
    "button": true,                     // Show/hide status bar icon
    "default_width": 640,               // Default width (left/right docked)
    "default_height": 320,              // Default height (bottom docked)

    // Set the cursor blinking behavior in the terminal (on, off, terminal_controlled)
    "blinking": "terminal_controlled",
    // Default cursor shape for the terminal cursor (block, bar, underline, hollow)
    "cursor_shape": "block",

    // Environment variables to add to terminal's process environment
    "env": {
      // "KEY": "value"
    },

    // Terminal scrollbar
    "scrollbar": {
      "show": null                       // Show/hide: (auto, system, always, never)
    },
    // Terminal Font Settings
    "font_family": "Fira Code",
    "font_size": 15,
    "font_weight": 400,
    // Terminal line height: comfortable (1.618), standard(1.3) or `{ "custom": 2 }`
    "line_height": "comfortable",

    "max_scroll_history_lines": 10000,   // Scrollback history (0=disable, max=100000)
  }
```

See [Terminal settings](./configuring-zed.md#terminal) for additional non-visual customization options.

### Other Panels

```json
  // Git Panel
  "git_panel": {
    "button": true,               // Show/hide status bar icon
    "dock": "left",               // Where to dock: left, right
    "default_width": 360,         // Default width of the git panel.
    "status_style": "icon",       // label_color, icon
    "sort_by_path": false,        // Sort by path (false) or status (true)
    "scrollbar": {
      "show": null                // Show/hide: (auto, system, always, never)
    }
  },

  // Debugger Panel
  "debugger": {
    "dock": "bottom",             // Where to dock: left, right, bottom
    "button": true                // Show/hide status bar icon
  },

  // Outline Panel
  "outline_panel": {
    "button": true,               // Show/hide status bar icon
    "default_width": 300,         // Default width of the git panel
    "dock": "left",               // Where to dock: left, right
    "file_icons": true,           // Show/hide file_icons
    "folder_icons": true,         // Show file_icons (true), chevrons (false) for dirs
    "git_status": true,           // Show git status
    "indent_size": 20,            // Indentation for nested items (pixels)
    "indent_guides": {
      "show": "always"            // Show indent guides (always, never)
    },
    "auto_reveal_entries": true,  // Show file in panel when activating its buffer
    "auto_fold_dirs": true,       // Fold dirs with single subdir
    "scrollbar": {                // Project panel scrollbar settings
      "show": null                // Show/hide: (auto, system, always, never)
    }
  }
```

## Collaboration Panels

```json
{
  // Collaboration Panel
  "collaboration_panel": {
    "button": true,               // Show/hide status bar icon
    "dock": "left",               // Where to dock: left, right
    "default_width": 240          // Default width of the collaboration panel.
  },
  "show_call_status_icon": true,  // Shown call status in the OS status bar.

  // Notification Panel
  "notification_panel": {
    // Whether to show the notification panel button in the status bar.
    "button": true,
    // Where to dock the notification panel. Can be 'left' or 'right'.
    "dock": "right",
    // Default width of the notification panel.
    "default_width": 380
  }
```
