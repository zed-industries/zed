# Configuring vim mode

Zed includes a mode that provides modal editing features modeled after the popular text editor Vim.

> **Note**: This page focuses on configuration options. If you're new to vim mode, the [Vim Mode](vim.md) page provides an overview of the modal editing features Zed provides.

In this guide, you'll learn how to:

- Enable or disable vim mode.
- Customize key bindings.
- Change vim mode settings.
- Useful optional key bindings to add to your configuration for faster navigation.

## Enabling and disabling vim mode

When you first open Zed, a checkbox will appear on the welcome screen, allowing you to enable vim mode. You can also toggle vim mode on or off anytime by opening the command palette and using the workspace command `toggle vim mode`.

This command adds or removes the following property from your user settings:

```json
{
  "vim_mode": true
}
```

## Customizing key bindings

You can edit your personal key bindings with `:keymap`. For Vim-specific shortcuts, you may find the following template a good place to start.

```json
[
  {
    "context": "VimControl && !menu",
    "bindings": {
      // put key-bindings here if you want them to work in normal & visual mode
    }
  },
  {
    "context": "vim_mode == normal && !menu",
    "bindings": {
      // "shift-y": ["workspace::SendKeystrokes", "y $"] // use nvim's Y behavior
    }
  },
  {
    "context": "vim_mode == insert",
    "bindings": {
      // "j k": "vim::NormalBefore" // remap jk in insert mode to escape.
    }
  },
  {
    "context": "EmptyPane || SharedScreen",
    "bindings": {
      // put key-bindings here (in addition to above) if you want them to
      // work when no editor exists
      // "space f": "file_finder::Toggle"
    }
  }
]
```

If you would like to emulate Vim's `map` (`nmap` etc.) commands you can bind to the [`workspace::SendKeystrokes`](./key-bindings.md#remapping-keys) action in the correct context.

You can see the bindings that are enabled by default in vim mode [here](https://github.com/zed-industries/zed/blob/main/assets/keymaps/vim.json).

If you want to navigate between the editor and docks (terminal, project panel, AI assistant, ...) just like you navigate between splits you can use the following key bindings:

```json
{
  "context": "Dock",
  "bindings": {
    "ctrl-w h": ["workspace::ActivatePaneInDirection", "Left"],
    "ctrl-w l": ["workspace::ActivatePaneInDirection", "Right"],
    "ctrl-w k": ["workspace::ActivatePaneInDirection", "Up"],
    "ctrl-w j": ["workspace::ActivatePaneInDirection", "Down"]
    // ... or other keybindings
  }
}
```

Subword motion is not enabled by default. To enable it, add these bindings to your keymap.

```json
[
  {
    "context": "VimControl && !menu && vim_mode != operator",
    "bindings": {
      "w": "vim::NextSubwordStart",
      "b": "vim::PreviousSubwordStart",
      "e": "vim::NextSubwordEnd",
      "g e": "vim::PreviousSubwordEnd"
    }
  }
]
```

Surrounding the selection in visual mode is also not enabled by default (`shift-s` normally behaves like `c`). To enable it, add the following to your keymap.

```json
{
  "context": "vim_mode == visual",
  "bindings": {
    "shift-s": [
      "vim::PushOperator",
      {
        "AddSurrounds": {}
      }
    ]
  }
}
```

### Restoring common text editing keybindings

If you're using vim mode on Linux or Windows, you may find it overrides keybindings you can't live without: <kbd>Ctrl</kbd>+<kbd>v</kbd> to copy, <kbd>Ctrl</kbd>+<kbd>f</kbd> to search, etc. You can restore them by copying this data into your keymap:

```json
{
  "context": "Editor && !menu",
  "bindings": {
    "ctrl-c": "editor::Copy",          // vim default: return to normal mode
    "ctrl-x": "editor::Cut",           // vim default: decrement
    "ctrl-v": "editor::Paste",         // vim default: visual block mode
    "ctrl-y": "editor::Undo",          // vim default: line up
    "ctrl-f": "buffer_search::Deploy", // vim default: page down
    "ctrl-o": "workspace::Open",       // vim default: go back
    "ctrl-a": "editor::SelectAll",     // vim default: increment
  }
},
```

## Changing vim mode settings

You can change the following settings to modify vim mode's behavior:

| Property                     | Description                                                                                                                                                                                   | Default Value |
| ---------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------- |
| use_system_clipboard         | Determines how system clipboard is used:<br><ul><li>"always": use for all operations</li><li>"never": only use when explicitly specified</li><li>"on_yank": use for yank operations</li></ul> | "always"      |
| use_multiline_find           | If `true`, `f` and `t` motions extend across multiple lines.                                                                                                                                  | false         |
| use_smartcase_find           | If `true`, `f` and `t` motions are case-insensitive when the target letter is lowercase.                                                                                                      | false         |
| toggle_relative_line_numbers | If `true`, line numbers are relative in normal mode and absolute in insert mode, giving you the best of both options.                                                                         | false         |
| custom_digraphs              | An object that allows you to add custom digraphs. Read below for an example.                                                                                                                  | {}            |

Here's an example of adding a digraph for the zombie emoji. This allows you to type `ctrl-k f z` to insert a zombie emoji. You can add as many digraphs as you like.

```json
{
  "vim": {
    "custom_digraphs": {
      "fz": "🧟‍♀️"
    }
  }
}
```

Here's an example of these settings changed:

```json
{
  "vim": {
    "use_system_clipboard": "never",
    "use_multiline_find": true,
    "use_smartcase_find": true,
    "toggle_relative_line_numbers": true,
    "custom_digraphs": {
      "fz": "🧟‍♀️"
    }
  }
}
```

## Useful core Zed settings for vim mode

Here are a few general Zed settings that can help you fine-tune your Vim experience:

| Property                | Description                                                                                                                                                   | Default Value          |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------- |
| cursor_blink            | If `true`, the cursor blinks.                                                                                                                                 | `true`                 |
| relative_line_numbers   | If `true`, line numbers in the left gutter are relative to the cursor.                                                                                        | `true`                 |
| scrollbar               | Object that controls the scrollbar display. Set to `{ "show": "never" }` to hide the scroll bar.                                                              | `{ "show": "always" }` |
| scroll_beyond_last_line | If set to `"one_page"`, allows scrolling up to one page beyond the last line. Set to `"off"` to prevent this behavior.                                        | `"one_page"`           |
| vertical_scroll_margin  | The number of lines to keep above or below the cursor when scrolling. Set to `0` to allow the cursor to go up to the edges of the screen vertically.          | `3`                    |
| gutter.line_numbers     | Controls the display of line numbers in the gutter. Set the `"line_numbers"` property to `false` to hide line numbers.                                        | `true`                 |
| command_aliases         | Object that defines aliases for commands in the command palette. You can use it to define shortcut names for commands you use often. Read below for examples. | `{}`                   |

> **Note**: the `command_aliases` setting is represented as a single object value in the table for brevity. in practice, it defines multiple aliases: `w` for `w`, `wq` for `wq`, and `q` for `q`.

Here's an example of these settings changed:

```json
{
  // Disable cursor blink
  "cursor_blink": false,
  // Use relative line numbers
  "relative_line_numbers": true,
  // Hide the scroll bar
  "scrollbar": { "show": "never" },
  // Prevent the buffer from scrolling beyond the last line
  "scroll_beyond_last_line": "off",
  // Allow cursor to reach edges of screen
  "vertical_scroll_margin": 0,
  "gutter": {
    // Disable line numbers completely:
    "line_numbers": false
  },
  "command_aliases": {
    "W": "w",
    "Wq": "wq",
    "Q": "q"
  }
}
```
