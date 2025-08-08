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

Zed reads your keymap from `~/.config/zed/keymap.json`. You can open the file within Zed with {#action zed::OpenKeymap} from the command palette or to spawn the Zed Keymap Editor ({#action zed::OpenKeymapEditor}) use {#kb zed::OpenKeymapEditor}.

The file contains a JSON array of objects with `"bindings"`. If no `"context"` is set the bindings are always active. If it is set the binding is only active when the [context matches](#contexts).

Within each binding section a [key sequence](#keybinding-syntax) is mapped to an [action](#actions). If conflicts are detected they are resolved as [described below](#precedence).

If you are using a non-QWERTY, Latin-character keyboard, you may want to set `use_key_equivalents` to `true`. See [Non-QWERTY keyboards](#non-qwerty-keyboards) for more information.

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

The keys can be any single unicode codepoint that your keyboard generates (for example `a`, `0`, `£` or `ç`), or any named key (`tab`, `f1`, `shift`, or `cmd`). If you are using a non-Latin layout (e.g. Cyrillic), you can bind either to the cyrillic character, or the latin character that key generates with `cmd` pressed.

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

For example:

```
# in an editor, it might look like this:
Workspace os=macos keyboard_layout=com.apple.keylayout.QWERTY
  Pane
    Editor mode=full extension=md vim_mode=insert

# in the project panel
Workspace os=macos
  Dock
    ProjectPanel not_editing
```

Context expressions can contain the following syntax:

- `X && Y`, `X || Y` to and/or two conditions
- `!X` to check that a condition is false
- `(X)` for grouping
- `X > Y` to match if an ancestor in the tree matches X and this layer matches Y.

For example:

- `"context": "Editor"` - matches any editor (including inline inputs)
- `"context": "Editor && mode=full"` - matches the main editors used for editing code
- `"context": "!Editor && !Terminal"` - matches anywhere except where an Editor or Terminal is focused
- `"context": "os=macos > Editor"` - matches any editor on macOS.

It's worth noting that attributes are only available on the node they are defined on. This means that if you want to (for example) only enable a keybinding when the debugger is stopped in vim normal mode, you need to do `debugger_stopped > vim_mode == normal`.

Note: Before Zed v0.197.x, the ! operator only looked at one node at a time, and `>` meant "parent" not "ancestor". This meant that `!Editor` would match the context `Workspace > Pane > Editor`, because (confusingly) the Pane matches `!Editor`, and that `os=macos > Editor` did not match the context `Workspace > Pane > Editor` because of the intermediate `Pane` node.

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

Zed's support for non-QWERTY keyboards is still a work in progress.

If your keyboard can type the full ASCII ranges (DVORAK, COLEMAK, etc.) then shortcuts should work as you expect.

Otherwise, read on...

#### macOS

On Cyrillic, Hebrew, Armenian, and other keyboards that are mostly non-ASCII; macOS automatically maps keys to the ASCII range when `cmd` is held. Zed takes this a step further and it can always match key-presses against either the ASCII layout, or the real layout regardless of modifiers, and regardless of the `use_key_equivalents` setting. For example in Thai, pressing `ctrl-ๆ` will match bindings associated with `ctrl-q` or `ctrl-ๆ`

On keyboards that support extended Latin alphabets (French AZERTY, German QWERTZ, etc.) it is often not possible to type the entire ASCII range without `option`. This introduces an ambiguity, `option-2` produces `@`. To ensure that all the builtin keyboard shortcuts can still be typed on these keyboards we move key-bindings around. For example, shortcuts bound to `@` on QWERTY are moved to `"` on a Spanish layout. This mapping is based on the macOS system defaults and can be seen by running `dev: Open Key Context View` from the command palette.

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

### Linux

Since v0.196.0 on Linux if the key that you type doesn't produce an ASCII character then we use the QWERTY-layout equivalent key for keyboard shortcuts. This means that many shortcuts can be typed on many layouts.

We do not yet move shortcuts around to ensure that all the builtin shortcuts can be typed on every layout; so if there are some ASCII characters that cannot be typed, and your keyboard layout has different ASCII characters on the same keys as would be needed to type them, you may need to add custom key bindings to make this work. We do intend to fix this at some point, and help is very much wanted!

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
