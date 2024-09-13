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

In this section, we'll learn how to customize the key bindings of Zed's vim mode. You'll learn:

- How to select the correct context for your new key bindings.
- Useful contexts for vim mode key bindings.
- Common key bindings to customize for extra productivity.

> **Note**: You can find a complete list of vim mode's default key bindings in Zed's code repository: [vim mode default keymap](https://github.com/zed-industries/zed/blob/main/assets/keymaps/vim.json).

### Selecting the correct context

Zed's key bindings are evaluated only when the `"context"` property matches your location in the editor. For example, if you add key bindings to the `"Editor"` context, they will only work when you're editing a file. If you add key bindings to the `"Workspace"` context, they will work everywhere in Zed. Here's an example of a key binding that saves when you're editing a file:

```json
{
  "context": "Editor",
  "bindings": {
    "ctrl-s": "file::Save"
  }
}
```

> **Note**: You can edit your personal key bindings with the `:keymap` command ({#kb zed::OpenKeymap}).

Contexts are nested, so when you're editing a file, the context is the `"Editor"` context, which is inside the `"Pane"` context, which is inside the `"Workspace"` context. That's why any key bindings you add to the `"Workspace"` context will work when you're editing a file. Here's an example:

```json
// This key binding will work when you're editing a file. It comes built into Zed by default as the workspace: save command.
{
  "context": "Workspace",
  "bindings": {
    "ctrl-s": "file::Save"
  }
}
```

Contexts are expressions. They support boolean operators like `&&` (and) and `||` (or). For example, you can use the context `"Editor && vim_mode == normal"` to create key bindings that only work when you're editing a file *and* you're in vim's normal mode.

Vim mode adds several contexts to the `"Editor"` context:

| Operator             | Description                                                                                                                                                                        |
| -------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| VimControl           | Indicates that vim keybindings should work. Currently an alias for `vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator`, but the definition may change over time |
| vim_mode == normal   | Normal mode                                                                                                                                                                        |
| vim_mode == visual   | Visual mode                                                                                                                                                                        |
| vim_mode == insert   | Insert mode                                                                                                                                                                        |
| vim_mode == replace  | Replace mode                                                                                                                                                                       |
| vim_mode == waiting  | Waiting for an arbitrary key (e.g., after typing `f` or `t`)                                                                                                                       |
| vim_mode == operator | Waiting for another binding to trigger (e.g., after typing `c` or `d`)                                                                                                             |
| vim_operator         | Set to `none` unless `vim_mode == operator`, in which case it is set to the current operator's default keybinding (e.g., after typing `d`, `vim_operator == d`)                    |

> **Note**: Contexts are matched only on one level at a time. So it is possible to use the expression `"Editor && vim_mode == normal"`, but `"Workspace && vim_mode == normal"` will never match because we set the vim context at the `"Editor"` level.

### Useful contexts for vim mode key bindings

Here's a template with useful vim mode contexts to get started customizing your vim mode key bindings. You can copy it and integrate it into your user keymap.

```json
[
  {
    "context": "VimControl && !menu",
    "bindings": {
      // Put key-bindings here if you want them to work in normal & visual mode.
    }
  },
  {
    "context": "vim_mode == normal && !menu",
    "bindings": {
      // "shift-y": ["workspace::SendKeystrokes", "y $"] // Use neovim's yank behavior: yank to end of line.
    }
  },
  {
    "context": "vim_mode == insert",
    "bindings": {
      // "j k": "vim::NormalBefore" // In insert mode, make jk escape to normal mode.
    }
  },
  {
    "context": "EmptyPane || SharedScreen",
    "bindings": {
      // Put key-bindings here (in addition to above) if you want them to
      // work when no editor exists.
      // "space f": "file_finder::Toggle"
    }
  }
]
```

> **Note**: If you would like to emulate Vim's `map` commands (`nmap`, etc.), you can use the action [`workspace::SendKeystrokes`](./key-bindings.md#remapping-keys) in the correct context.

### Optional key bindings

By default, you can navigate between the different files open in the editor with shortcuts like `ctrl+w` followed by one of `hjkl` to move to the left, down, up, or right respectively.

But you cannot use the same shortcuts to move between all the editor docks (the terminal, project panel, assistant panel, ...). If you want to use the same shortcuts to navigate to the docks, you can add the following key bindings to your user key map.

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

Subword motion, which allows you to navigate and select individual words in camelCase or snake_case, is not enabled by default. To enable it, add these bindings to your keymap.

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

Vim mode comes with shortcuts to surround the selection in normal mode (`ys`), but it doesn't have a shortcut to add surrounds in visual mode. By default, `shift-s` substitutes the selection (erases the text and enters insert mode). To use `shift-s` to add surrounds in visual mode, you can add the following object to your keymap.

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

If you're using vim mode on Linux or Windows, you may find it overrides keybindings you can't live without: `ctrl+v` to copy, `ctrl+f` to search, etc. You can restore them by copying this data into your keymap:

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
