# Key bindings

Zed has a very customizable key binding system — you can tweak everything to work exactly how your fingers expect!

## Predefined keymaps

If you're used to a specific editor's defaults you can set a `base_keymap` in your [settings file](./configuring-zed.md). We currently have:

- VSCode (default)
- Atom
- Emacs (Beta)
- JetBrains
- SublimeText
- TextMate
- None (disables _all_ key bindings)

You can also enable `vim_mode`, which adds vim bindings too.

## User keymaps

Zed reads your keymap from `~/.zed/keymap.json` on MacOS (or `~/.config/zed/keymap.json` on Linux). You can open the file within Zed with {#kb zed::OpenKeymap}, or via `zed: Open Keymap` in the command palette.

The file contains a JSON array of objects with `"bindings"`. If no `"context"` is set the bindings are always active. If it is set the binding is only active when the [context matches](#contexts).

Within each binding section a [key sequence](#keybinding-syntax) is mapped to an [action](#actions). If conflicts are detected they are resolved as [described below](#precedence).

If you are using a non-QWERTY, Latin-character keyboard, you may want to set `use_layout_keys` to `true`. See [Non-QWERTY keyboards](#non-qwerty-keyboards) for more information.

For example:

```json
[
  {
    "bindings": {
      "ctrl-right": "editor::SelectLargerSyntaxNode",
      "ctrl-left": "editor::SelectSmallerSyntaxNode"
    }
  },
  {
    "context": "ProjectPanel && not_editing",
    "bindings": {
      "o": "project_panel::Open"
    }
  }
]
```

You can see all of Zed's default bindings in the default keymaps for [MacOS](https://github.com/zed-industries/zed/blob/main/assets/keymaps/default-macos.json) or [Linux](https://github.com/zed-industries/zed/blob/main/assets/keymaps/default-linux.json).

If you want to debug problems with custom keymaps you can use `dev: Open Key Context View` from the command palette. Please file [an issue](https://github.com/zed-industries/zed) if you run into something you think should work but isn't.

### Keybinding syntax

Zed has the ability to match against not just a single keypress, but a sequence of keys typed in order. Each key in the `"bindings"` map is a sequence of keypresses separated with a space.

Each keypress is a sequence of modifiers followed by a key. The modifiers are:

- `ctrl-` The control key
- `cmd-`, `win-` or `super-` for the platform modifier (Command on macOS, Windows key on Windows, and the Super key on Linux).
- `alt-` for alt (option on macOS)
- `shift-` The shift key
- `fn-` The function key
- `secondary-` Equivalent to `cmd` when Zed is running on macOS and `ctrl` when on Windows and Linux

The keys can be any single unicode codepoint that your keyboard generates (for example `a`, `0`, `£` or `ç`), or any named key (`tab`, `f1`, `shift`, or `cmd`). If you are using a non-Latin layout (e.g. Cyrillic), you can bind either to the cyrillic character, or the latin character that that key generates with `cmd` pressed.

A few examples:

```json
 "bindings": {
   "cmd-k cmd-s": "zed::OpenKeymap", // matches ⌘-k then ⌘-s
   "space e": "editor::Complete", // type space then e
   "ç": "editor::Complete", // matches ⌥-c
   "shift shift": "file_finder::Toggle", // matches pressing and releasing shift twice
 }
```

The `shift-` modifier can only be used in combination with a letter to indicate the uppercase version. For example `shift-g` matches typing `G`. Although on many keyboards shift is used to type punctuation characters like `(`, the keypress is not considered to be modified and so `shift-(` does not match.

The `alt-` modifier can be used on many layouts to generate a different key. For example on macOS US keyboard the combination `alt-c` types `ç`. You can match against either in your keymap file, though by convention Zed spells this combination as `alt-c`.

It is possible to match against typing a modifier key on its own. For example `shift shift` can be used to implement JetBrains search everywhere shortcut. In this case the binding happens on key release instead of keypress.

### Contexts

If a binding group has a `"context"` key it will be matched against the currently active contexts in Zed.

Zed's contexts make up a tree, with the root being `Workspace`. Workspaces contain Panes and Panels, and Panes contain Editors, etc. The easiest way to see what contexts are active at a given moment is the key context view, which you can get to with `dev: Open Key Context View` in the command palette.

Contexts can contain extra attributes in addition to the name, so that you can (for example) match only in markdown files with `"context": "Editor && extension==md"`. It's worth noting that you can only use attributes at the level they are defined.

For example:

```
# in an editor, it might look like this:
Workspace os=macos keyboard_layout=com.apple.keylayout.QWERTY
  Pane
    Editor mode=full extension=md inline_completion vim_mode=insert

# in the project panel
Workspace os=macos
  Dock
    ProjectPanel not_editing
```

Context expressions can contain the following syntax:

- `X && Y`, `X || Y` to and/or two conditions
- `!X` to negate a condition
- `(X)` for grouping
- `X > Y` to match if a parent in the tree matches X and this layer matches Y.

If you're using Vim mode, we have information on how [vim modes influence the context](./vim.md#contexts)

### Actions

Pretty much all of Zed's functionality is exposed as actions. Although there is
no explicitly documented list, you can find most of them by searching in the
command palette, by looking in the default keymaps for
[MacOS](https://github.com/zed-industries/zed/blob/main/assets/keymaps/default-macos.json)
or
[Linux](https://github.com/zed-industries/zed/blob/main/assets/keymaps/default-linux.json), or by using Zed's autocomplete in your keymap file.

Most actions do not require any arguments, and so you can bind them as strings: `"ctrl-a": "language_selector::Toggle"`. Some require a single argument, and must be bound as an array: `"cmd-1": ["workspace::ActivatePane", 0]`. Some actions require multiple arguments, and are bound as an array of a string and an object: `"ctrl-a": ["pane::DeploySearch", { "replace_enabled": true }]`.

### Precedence

When multiple keybindings have the same keystroke and are active at the same time, precedence is resolved in two ways:

- Bindings that match on lower nodes in the context tree win. This means that if you have a binding with a context of `Editor` it will take precedence over a binding with a context of `Workspace`. Bindings with no context match at the lowest level in the tree.
- If there are multiple bindings that match at the same level in the tree, then the binding defined later takes precedence. As user keybindings are loaded after system keybindings, this allows user bindings to take precedence over builtin keybindings.

The other kind of conflict that arises is when you have two bindings, one of which is a prefix of the other. For example if you have `"ctrl-w":"editor::DeleteToNextWordEnd"` and `"ctrl-w left":"editor::DeleteToEndOfLine"`.

When this happens, and both bindings are active in the current context, Zed will wait for 1 second after you type `ctrl-w` to see if you're about to type `left`. If you don't type anything, or if you type a different key, then `DeleteToNextWordEnd` will be triggered. If you do, then `DeleteToEndOfLine` will be triggered.

### Non-QWERTY keyboards

As of Zed 0.162.0, Zed has some support for non-QWERTY keyboards on macOS. Better support for non-QWERTY keyboards on Linux is planned.

There are roughly three categories of keyboard to consider:

Keyboards that support full ASCII (QWERTY, DVORAK, COLEMAK, etc.). On these keyboards bindings are resolved based on the character that would be generated by the key. So to type `cmd-[`, find the key labeled `[` and press it with command.

Keyboards that are mostly non-ASCII, but support full ASCII when the command key is pressed. For example Cyrillic keyboards, Armenian, Hebrew, etc. On these keyboards bindings are resolved based on the character that would be generated by typing the key with command pressed. So to type `ctrl-a`, find the key that generates `cmd-a`. For these keyboards, keyboard shortcuts are displayed in the app using their ASCII equivalents. If the ASCII-equivalents are not printed on your keyboard, you can use the macOS keyboard viewer and holding down the `cmd` key to find things (though often the ASCII equivalents are in a QWERTY layout).

Finally keyboards that support extended Latin alphabets (usually ISO keyboards) require the most support. For example French AZERTY, German QWERTZ, etc. On these keyboards it is often not possible to type the entire ASCII range without option. To ensure that shortcuts _can_ be typed without option, keyboard shortcuts are mapped to "key equivalents" in the same way as [macOS](). This mapping is defined per layout, and is a compromise between leaving keyboard shortcuts triggered by the same character they are defined with, keeping shortcuts in the same place as a QWERTY layout, and moving shortcuts out of the way of system shortcuts.

For example on a German QWERTZ keyboard, the `cmd->` shortcut is moved to `cmd-:` because `cmd->` is the system window switcher and this is where that shortcut is typed on a QWERTY keyboard. `cmd-+` stays the same because + is still typeable without option, and as a result, `cmd-[` and `cmd-]` become `cmd-ö` and `cmd-ä`, moving out of the way of the `+` key.

If you are defining shortcuts in your personal keymap, you can opt into the key equivalent mapping by setting `use_key_equivalents` to `true` in your keymap:

```json
[
  {
    "use_key_equivalents": true,
    "bindings": {
      "ctrl->": "editor::Indent" // parsed as ctrl-: when a German QWERTZ keyboard is active
    }
  }
]
```

## Tips and tricks

### Disabling a binding

If you'd like a given binding to do nothing in a given context you can use
`null` as the action. This is useful if you hit the keybinding by accident and
want to disable it, or if you want to type the character that would be typed by
the sequence, or if you want to disable multikey bindings starting with that key.

```json
[
  {
    "context": "Workspace",
    "bindings": {
      "cmd-r": null // cmd-r will do nothing when the Workspace context is active
    }
  }
]
```

A `null` binding follows the same precedence rules as normal actions. So disables all bindings that would match further up in the tree too. If you'd like a binding that matches further up in the tree to take precedence over a lower binding, you need to rebind it to the action you want in the context you want.

This is useful for preventing Zed from falling back to a default keybinding when the action you specified is conditional and propagates. For example, `buffer_search::DeployReplace` only triggers when the search bar is not in view. If the search bar is in view, it would propagate and trigger the default action set for that binding, such as opening the right dock. To prevent this from happening:

```json
[
  {
    "context": "Workspace",
    "bindings": {
      "cmd-r": null // cmd-r will do nothing when the search bar is in view
    }
  },
  {
    "context": "Workspace",
    "bindings": {
      "cmd-r": "buffer_search::DeployReplace" // cmd-r will deploy replace when the search bar is not in view
    }
  }
]
```

### Remapping keys

A common request is to be able to map from a single keystroke to a sequence. You can do this with the `workspace::SendKeystrokes` action.

```json
[
  {
    "bindings": {
      "alt-down": ["workspace::SendKeystrokes", "down down down down"],
      "cmd-alt-c": [
        "workspace::SendKeystrokes",
        "cmd-shift-p copy relative path enter"
      ],
      "cmd-alt-r": ["workspace::SendKeystrokes", "cmd-p README enter"]
    }
  },
  {
    "context": "Editor && vim_mode == insert",
    "bindings": {
      "j k": ["workspace::SendKeystrokes", "escape"]
    }
  }
]
```

There are some limitations to this, notably:

- Any asynchronous operation will not happen until after all your key bindings have been dispatched. For example this means that while you can use a binding to open a file (as in the `cmd-alt-r` example) you cannot send further keystrokes and hope to have them interpreted by the new view.
- Other examples of asynchronous things are: opening the command palette, communicating with a language server, changing the language of a buffer, anything that hits the network.
- There is a limit of 100 simulated keys at a time.

The argument to `SendKeystrokes` is a space-separated list of keystrokes (using the same syntax as above). Due to the way that keystrokes are parsed, any segment that is not recognized as a keypress will be sent verbatim to the currently focused input field.

If the argument to `SendKeystrokes` contains the binding used to trigger it, it will use the next-highest-precedence definition of that binding. This allows you to extend the default behavior of a key binding.

### Forward keys to terminal

If you're on Linux or Windows, you might find yourself wanting to forward key combinations to the built-in terminal instead of them being handled by Zed.

For example, `ctrl-n` creates a new tab in Zed on Linux. If you want to send `ctrl-n` to the built-in terminal when it's focused, add the following to your keymap:

```json
{
  "context": "Terminal",
  "bindings": {
    "ctrl-n": ["terminal::SendKeystroke", "ctrl-n"]
  }
}
```

### Task Key bindings

You can also bind keys to launch Zed Tasks defined in your tasks.json.
See the [tasks documentation](tasks.md#custom-keybindings-for-tasks) for more.
