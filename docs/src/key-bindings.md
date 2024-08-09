# Key bindings

Zed can be configured via a simple JSON file located at `~/.config/zed/keymap.json`.

## Predefined keymaps

We have a growing collection of pre-defined keymaps in [zed repository's keymaps folder](https://github.com/zed-industries/zed/tree/main/assets/keymaps). Our current keymaps include:

- Atom
- JetBrains
- SublimeText
- TextMate
- VSCode (default)

These keymaps can be set via the `base_keymap` setting in your `settings.json` file. Additionally, if you'd like to work from a clean slate, you can provide `"None"` to the setting.

## Custom key bindings

### Accessing custom key bindings

You can open `keymap.json` via `⌘` + `K`, `⌘` + `S`, the command palette, or the `Zed > Settings > Open Key Bindings` application menu item.

### Adding a custom key binding

To customize key bindings, specify a context and the list of bindings to set. Re-mapping an existing binding will clobber the existing binding in favor of the custom one.

An example of adding a set of custom key bindings:

```json
[
  {
    "context": "Editor",
    "bindings": {
      "ctrl-w": "editor::SelectLargerSyntaxNode",
      "ctrl-shift-W": "editor::SelectSmallerSyntaxNode",
      "ctrl-c": "editor::Cancel"
    }
  }
]
```

You can see more examples in Zed's [`default.json`](https://github.com/zed-industries/zed/blob/main/assets/keymaps/default-macos.json)

_There are some key bindings that can't be overridden; we are working on an issue surrounding this._

### Keybinding syntax

Zed has the ability to match against not just a single keypress, but a sequence of keys typed in order. Each key in the `"bindings"` map is a sequence of keypresses separated with a space.

Each key press is a sequence of modifiers followed by a key. The modifiers are:

- `ctrl-` The control key
- `cmd-`, `win-` or `super-` for the platform modifier (Command on macOS, Windows key on Windows, and the Super key on Linux).
- `alt-` for alt (option on macOS)
- `shift-` The shift key
- `fn-` The function key

The keys can be any single unicode codepoint that your keyboard generates (for example `a`, `0`, `£` or `ç`), or any named key (`tab`, `f1`, `shift`, or `cmd`).

A few examples:

```
 "bindings": {
   "cmd-k cmd-s": "zed::OpenKeymap", // matches ⌘-k then ⌘-s
   "space e": "editor::Complete", // type space then e
   "ç": "editor::Complete", // matches ⌥-c
   "shift shift": "file_finder::Toggle", // matches pressing and releasing shift twice
 }
```

The `shift-` modifier can only be used in combination with a letter to indicate the uppercase version. For example `shift-g` matches typing `G`. Although on many keyboards shift is used to type punctuation characters like `(`, the keypress is not considered to be modified and so `shift-(` does not match.

The `alt-` modifier can be used on many layouts to generate a different key. For example on macOS US keyboard the combination `alt-c` types `ç`. You can match against either in your keymap file, though by convention Zed spells this combination as `alt-c`.

It is possible to match against typing a modifier key on its own. For example `shift shift` can be used to implement JetBrains search everywhere shortcut. In this case the binding happens on key release instead of key press.

### Remapping keys

A common request is to be able to map from one sequence of keys to another. As of Zed 0.124.0 you can do this with the `workspace::SendKeystrokes` action.

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
- - Other examples of asynchronous things are: communicating with a language server, changing the language of a buffer, anything that hits the network.
- There is a limit of 100 simulated keys at a time, this is to avoid accidental infinite recursion if you trigger SendKeystrokes again inside your bindings.

The argument to `SendKeystrokes` is a space-separated list of keystrokes (using the same syntax as above). Due to the way that keystrokes are parsed, any segment that is not recognized as a keypress will be sent verbatim to the currently focused input field.

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
See the [tasks documentation](/docs/tasks#custom-keybindings-for-tasks) for more.

### All key bindings

#### Global

| **Command**               | **Target**   | **Default Shortcut**    |
| ------------------------- | ------------ | ----------------------- |
| Toggle focus              | Collab Panel | `⌘ + Shift + C`         |
| Toggle inlay hints        | Editor       | `Control + :`           |
| Cancel                    | Menu         | `Control + C`           |
| Cancel                    | Menu         | `Control + Escape`      |
| Cancel                    | Menu         | `Escape`                |
| Cancel                    | Menu         | `⌘ + Escape`            |
| Confirm                   | Menu         | `Enter`                 |
| Secondary confirm         | Menu         | `Control + Enter`       |
| Secondary confirm         | Menu         | `⌘ + Enter`             |
| Select first              | Menu         | `Page Up`               |
| Select first              | Menu         | `Shift + Page Down`     |
| Select first              | Menu         | `Shift + Page Up`       |
| Select first              | Menu         | `⌘ + Up`                |
| Select last               | Menu         | `Page Down`             |
| Select last               | Menu         | `⌘ + Down`              |
| Select next               | Menu         | `Control + N`           |
| Select next               | Menu         | `Down`                  |
| Select prev               | Menu         | `Control + P`           |
| Select prev               | Menu         | `Up`                    |
| Confirm input             | Picker       | `Alt + Enter`           |
| Confirm input             | Picker       | `⌘ + Alt + Enter`       |
| Use selected query        | Picker       | `Shift + Enter`         |
| Close window              | Workspace    | `⌘ + Shift + W`         |
| Follow next collaborator  | Workspace    | `Control + Alt + ⌘ + F` |
| Open                      | Workspace    | `⌘ + O`                 |
| Toggle zoom               | Workspace    | `Shift + Escape`        |
| Debug elements            | Zed          | `⌘ + Alt + I`           |
| Decrease buffer font size | Zed          | `⌘ + -`                 |
| Hide                      | Zed          | `⌘ + H`                 |
| Hide others               | Zed          | `Alt + ⌘ + H`           |
| Increase buffer font size | Zed          | `⌘ + +`                 |
| Increase buffer font size | Zed          | `⌘ + =`                 |
| Minimize                  | Zed          | `⌘ + M`                 |
| Open settings             | Zed          | `⌘ + ,`                 |
| Quit                      | Zed          | `⌘ + Q`                 |
| Reset buffer font size    | Zed          | `⌘ + 0`                 |
| Toggle full screen        | Zed          | `Control + ⌘ + F`       |

#### Editor

| **Command**                      | **Target** | **Default Shortcut**            |
| -------------------------------- | ---------- | ------------------------------- |
| Add selection above              | Editor     | `⌘ + Alt + Up`                  |
| Add selection above              | Editor     | `⌘ + Control + P`               |
| Add selection below              | Editor     | `⌘ + Alt + Down`                |
| Add selection below              | Editor     | `⌘ + Control + N`               |
| Backspace                        | Editor     | `Backspace`                     |
| Backspace                        | Editor     | `Control + H`                   |
| Backspace                        | Editor     | `Shift + Backspace`             |
| Cancel                           | Editor     | `Escape`                        |
| Confirm code action              | Editor     | `Enter`                         |
| Confirm completion               | Editor     | `Enter`                         |
| Confirm completion               | Editor     | `Tab`                           |
| Confirm rename                   | Editor     | `Enter`                         |
| Context menu first               | Editor     | `Page Up`                       |
| Context menu last                | Editor     | `Page Down`                     |
| Context menu next                | Editor     | `Control + N`                   |
| Context menu next                | Editor     | `Down`                          |
| Context menu prev                | Editor     | `Control + P`                   |
| Context menu prev                | Editor     | `Up`                            |
| Copy                             | Editor     | `⌘ + C`                         |
| Cut                              | Editor     | `⌘ + X`                         |
| Cut to end of line               | Editor     | `Control + K`                   |
| Delete                           | Editor     | `Control + D`                   |
| Delete                           | Editor     | `Delete`                        |
| Delete line                      | Editor     | `⌘ + Shift + K`                 |
| Delete to beginning of line      | Editor     | `⌘ + Backspace`                 |
| Delete to end of line            | Editor     | `⌘ + Delete`                    |
| Delete to next subword end       | Editor     | `Control + Alt + D`             |
| Delete to next subword end       | Editor     | `Control + Alt + Delete`        |
| Delete to next word end          | Editor     | `Alt + D`                       |
| Delete to next word end          | Editor     | `Alt + Delete`                  |
| Delete to previous subword start | Editor     | `Control + Alt + Backspace`     |
| Delete to previous subword start | Editor     | `Control + Alt + H`             |
| Delete to previous word start    | Editor     | `Alt + Backspace`               |
| Delete to previous word start    | Editor     | `Alt + H`                       |
| Delete to previous word start    | Editor     | `Control + W`                   |
| Display cursor names             | Editor     | `Control + ⌘ + C`               |
| Duplicate line down              | Editor     | `Alt + Shift + Down`            |
| Duplicate line up                | Editor     | `Alt + Shift + Up`              |
| Find all references              | Editor     | `Alt + Shift + F12`             |
| Fold                             | Editor     | `Alt + ⌘ + [`                   |
| Format                           | Editor     | `⌘ + Shift + I`                 |
| Go to definition                 | Editor     | `F12`                           |
| Go to definition split           | Editor     | `Alt + F12`                     |
| Go to declaration                | Editor     | `Ctrl + F12`                    |
| Go to declaration split          | Editor     | `Alt + Ctrl + F12`              |
| Go to diagnostic                 | Editor     | `F8`                            |
| Go to implementation             | Editor     | `Shift + F12`                   |
| Go to prev diagnostic            | Editor     | `Shift + F8`                    |
| Go to type definition            | Editor     | `⌘ + F12`                       |
| Go to type definition split      | Editor     | `Alt + ⌘ + F12`                 |
| Hover                            | Editor     | `⌘ + K, ⌘ + I`                  |
| Indent                           | Editor     | `⌘ + ]`                         |
| Join lines                       | Editor     | `Control + J`                   |
| Move down                        | Editor     | `Control + N`                   |
| Move down                        | Editor     | `Down`                          |
| Move left                        | Editor     | `Control + B`                   |
| Move left                        | Editor     | `Left`                          |
| Move line down                   | Editor     | `Alt + Down`                    |
| Move line up                     | Editor     | `Alt + Up`                      |
| Move page down                   | Editor     | `Control + V`                   |
| Move page down                   | Editor     | `Shift + Page Down`             |
| Move page up                     | Editor     | `Alt + V`                       |
| Move page up                     | Editor     | `Shift + Page Up`               |
| Move right                       | Editor     | `Control + F`                   |
| Move right                       | Editor     | `Right`                         |
| Move to beginning                | Editor     | `⌘ + Up`                        |
| Move to beginning of line        | Editor     | `Control + A`                   |
| Move to beginning of line        | Editor     | `Home`                          |
| Move to beginning of line        | Editor     | `⌘ + Left`                      |
| Move to enclosing bracket        | Editor     | `Control + M`                   |
| Move to end                      | Editor     | `⌘ + Down`                      |
| Move to end of line              | Editor     | `Control + E`                   |
| Move to end of line              | Editor     | `End`                           |
| Move to end of line              | Editor     | `⌘ + Right`                     |
| Move to end of paragraph         | Editor     | `Control + Down`                |
| Move to next subword end         | Editor     | `Control + Alt + F`             |
| Move to next subword end         | Editor     | `Control + Alt + Right`         |
| Move to next word end            | Editor     | `Alt + F`                       |
| Move to next word end            | Editor     | `Alt + Right`                   |
| Move to previous subword start   | Editor     | `Control + Alt + B`             |
| Move to previous subword start   | Editor     | `Control + Alt + Left`          |
| Move to previous word start      | Editor     | `Alt + B`                       |
| Move to previous word start      | Editor     | `Alt + Left`                    |
| Move to start of paragraph       | Editor     | `Control + Up`                  |
| Move up                          | Editor     | `Control + P`                   |
| Move up                          | Editor     | `Up`                            |
| Next screen                      | Editor     | `Control + L`                   |
| Outdent                          | Editor     | `⌘ + [`                         |
| Page down                        | Editor     | `Page Down`                     |
| Page up                          | Editor     | `Page Up`                       |
| Paste                            | Editor     | `⌘ + V`                         |
| Redo                             | Editor     | `⌘ + Shift + Z`                 |
| Redo selection                   | Editor     | `⌘ + Shift + U`                 |
| Rename                           | Editor     | `F2`                            |
| Reveal in File Manager           | Editor     | `Alt + ⌘ + R`                   |
| Toggle hunk diff                 | Editor     | `⌘ + '`                         |
| Expand all hunk diffs            | Editor     | `⌘ + "`                         |
| Revert selected hunks            | Editor     | `⌘ + Alt + Z`                   |
| Select all                       | Editor     | `⌘ + A`                         |
| Select all matches               | Editor     | `⌘ + Shift + L`                 |
| Select down                      | Editor     | `Control + Shift + N`           |
| Select down                      | Editor     | `Shift + Down`                  |
| Select larger syntax node        | Editor     | `Control + Shift + Right`       |
| Select left                      | Editor     | `Control + Shift + B`           |
| Select left                      | Editor     | `Shift + Left`                  |
| Select line                      | Editor     | `⌘ + L`                         |
| Select next                      | Editor     | `⌘ + D`                         |
| Select next                      | Editor     | `⌘ + K, ⌘ + D`                  |
| Select previous                  | Editor     | `Control + ⌘ + D`               |
| Select previous                  | Editor     | `⌘ + K, Control + ⌘ + D`        |
| Select right                     | Editor     | `Control + Shift + F`           |
| Select right                     | Editor     | `Shift + Right`                 |
| Select smaller syntax node       | Editor     | `Control + Shift + Left`        |
| Select to beginning              | Editor     | `⌘ + Shift + Up`                |
| Select to beginning of line      | Editor     | `Control + Shift + A`           |
| Select to beginning of line      | Editor     | `Shift + Home`                  |
| Select to beginning of line      | Editor     | `⌘ + Shift + Left`              |
| Select to end                    | Editor     | `⌘ + Shift + Down`              |
| Select to end of line            | Editor     | `Control + Shift + E`           |
| Select to end of line            | Editor     | `Shift + End`                   |
| Select to end of line            | Editor     | `⌘ + Shift + Right`             |
| Select to end of paragraph       | Editor     | `Control + Shift + Down`        |
| Select to next subword end       | Editor     | `Control + Alt + Shift + F`     |
| Select to next subword end       | Editor     | `Control + Alt + Shift + Right` |
| Select to next word end          | Editor     | `Alt + Shift + F`               |
| Select to next word end          | Editor     | `Alt + Shift + Right`           |
| Select to previous subword start | Editor     | `Control + Alt + Shift + B`     |
| Select to previous subword start | Editor     | `Control + Alt + Shift + Left`  |
| Select to previous word start    | Editor     | `Alt + Shift + B`               |
| Select to previous word start    | Editor     | `Alt + Shift + Left`            |
| Select to start of paragraph     | Editor     | `Control + Shift + Up`          |
| Select up                        | Editor     | `Control + Shift + P`           |
| Select up                        | Editor     | `Shift + Up`                    |
| Show character palette           | Editor     | `Control + ⌘ + Space`           |
| Show completions                 | Editor     | `Control + Space`               |
| Show inline completion           | Editor     | `Alt + \`                       |
| Tab                              | Editor     | `Tab`                           |
| Tab prev                         | Editor     | `Shift + Tab`                   |
| Toggle code actions              | Editor     | `⌘ + .`                         |
| Toggle comments                  | Editor     | `⌘ + /`                         |
| Toggle git blame                 | Editor     | `⌘ + Alt + G, B`                |
| Toggle line numbers              | Editor     | `⌘ + ;`                         |
| Transpose                        | Editor     | `Control + T`                   |
| Undo                             | Editor     | `⌘ + Z`                         |
| Undo selection                   | Editor     | `⌘ + U`                         |
| Unfold lines                     | Editor     | `Alt + ⌘ + ]`                   |

#### Editor (Full Only)

| **Command**                      | **Target**    | **Default Shortcut** |
| -------------------------------- | ------------- | -------------------- |
| Inline assist                    | Assistant     | `Control + Enter`    |
| Quote selection                  | Assistant     | `⌘ + >`              |
| Deploy                           | Buffer Search | `⌘ + Alt + F`        |
| Deploy                           | Buffer Search | `⌘ + E`              |
| Deploy                           | Buffer Search | `⌘ + F`              |
| Accept partial inline completion | Editor        | `Alt + Right`        |
| Go to hunk                       | Editor        | `⌘ + F8`             |
| Go to prev hunk                  | Editor        | `⌘ + Shift + F8`     |
| Newline                          | Editor        | `Enter`              |
| Newline                          | Editor        | `Shift + Enter`      |
| Newline above                    | Editor        | `⌘ + Shift + Enter`  |
| Newline below                    | Editor        | `⌘ + Enter`          |
| Next inline completion           | Editor        | `Alt + ]`            |
| Open excerpts                    | Editor        | `Alt + Enter`        |
| Open excerpts split              | Editor        | `⌘ + K, Enter`       |
| Previous inline completion       | Editor        | `Alt + [`            |
| Toggle soft wrap                 | Editor        | `Alt + Z`            |
| Toggle                           | Go To Line    | `Control + G`        |
| Toggle                           | Outline       | `⌘ + Shift + O`      |

#### Editor (Auto Height Only)

| **Command**   | **Target** | **Default Shortcut**      |
| ------------- | ---------- | ------------------------- |
| Newline       | Editor     | `Control + Enter`         |
| Newline       | Editor     | `Shift + Enter`           |
| Newline below | Editor     | `Control + Shift + Enter` |

#### Pane

| **Command**                   | **Target**     | **Default Shortcut**    |
| ----------------------------- | -------------- | ----------------------- |
| Activate item 1               | Pane           | `Control + 1`           |
| Activate item 2               | Pane           | `Control + 2`           |
| Activate item 3               | Pane           | `Control + 3`           |
| Activate item 4               | Pane           | `Control + 4`           |
| Activate item 5               | Pane           | `Control + 5`           |
| Activate item 6               | Pane           | `Control + 6`           |
| Activate item 7               | Pane           | `Control + 7`           |
| Activate item 8               | Pane           | `Control + 8`           |
| Activate item 9               | Pane           | `Control + 9`           |
| Activate last item            | Pane           | `Control + 0`           |
| Activate next item            | Pane           | `Alt + ⌘ + Right`       |
| Activate next item            | Pane           | `⌘ + }`                 |
| Activate prev item            | Pane           | `Alt + ⌘ + Left`        |
| Activate prev item            | Pane           | `⌘ + {`                 |
| Swap item to left             | Pane           | `⌘ + Shift + Page Up`   |
| Swap item to right            | Pane           | `⌘ + Shift + Page Down` |
| Close active item             | Pane           | `⌘ + W`                 |
| Close all items               | Pane           | `⌘ + K, ⌘ + W`          |
| Close clean items             | Pane           | `⌘ + K, U`              |
| Close inactive items          | Pane           | `Alt + ⌘ + T`           |
| Go back                       | Pane           | `Control + -`           |
| Go forward                    | Pane           | `Control + _`           |
| Reopen closed item            | Pane           | `⌘ + Shift + T`         |
| Split down                    | Pane           | `⌘ + K, Down`           |
| Split left                    | Pane           | `⌘ + K, Left`           |
| Split right                   | Pane           | `⌘ + K, Right`          |
| Split up                      | Pane           | `⌘ + K, Up`             |
| Toggle filters                | Project Search | `Alt + ⌘ + F`           |
| Toggle focus                  | Project Search | `⌘ + F`                 |
| Toggle focus                  | Project Search | `⌘ + Shift + F`         |
| Activate regex mode           | Search         | `Alt + ⌘ + G`           |
| Activate text mode            | Search         | `Alt + ⌘ + X`           |
| Cycle mode                    | Search         | `Alt + Tab`             |
| Select all matches            | Search         | `Alt + Enter`           |
| Select next match             | Search         | `⌘ + G`                 |
| Select prev match             | Search         | `⌘ + Shift + G`         |
| Toggle case sensitive         | Search         | `Alt + ⌘ + C`           |
| Toggle replace                | Search         | `⌘ + Shift + H`         |
| Toggle whole word             | Search         | `Alt + ⌘ + W`           |
| Close inactive tabs and panes | Workspace      | `Control + Alt + ⌘ + W` |

#### Buffer Search Bar

| **Command**            | **Target**    | **Default Shortcut** |
| ---------------------- | ------------- | -------------------- |
| Dismiss                | Buffer Search | `Escape`             |
| Focus editor           | Buffer Search | `Tab`                |
| Cycle mode             | Search        | `Alt + Tab`          |
| Focus search           | Search        | `⌘ + F`              |
| Next history query     | Search        | `Down`               |
| Previous history query | Search        | `Up`                 |
| Replace all            | Search        | `⌘ + Enter`          |
| Replace next           | Search        | `Enter`              |
| Select all matches     | Search        | `Alt + Enter`        |
| Select next match      | Search        | `Enter`              |
| Select prev match      | Search        | `Shift + Enter`      |
| Toggle replace         | Search        | `⌘ + Alt + F`        |

#### Workspace

| **Command**                      | **Target**        | **Default Shortcut**    |
| -------------------------------- | ----------------- | ----------------------- |
| Toggle focus                     | Assistant         | `⌘ + ?`                 |
| Open recent                      | Branches          | `Alt + ⌘ + B`           |
| Toggle                           | Command Palette   | `⌘ + Shift + P`         |
| Deploy                           | Diagnostics       | `⌘ + Shift + M`         |
| Toggle                           | File Finder       | `⌘ + P`                 |
| Toggle                           | Language Selector | `⌘ + K, M`              |
| Deploy search                    | Pane              | `⌘ + Shift + F`         |
| Deploy search                    | Pane              | `⌘ + Shift + H`         |
| Toggle focus                     | Project Panel     | `⌘ + Shift + E`         |
| Toggle                           | Project Symbols   | `⌘ + T`                 |
| Open recent                      | Projects          | `Alt + ⌘ + O`           |
| Toggle                           | Tab Switcher      | `Control + Shift + Tab` |
| Toggle                           | Tab Switcher      | `Control + Tab`         |
| Rerun                            | Task              | `Alt + T`               |
| Spawn                            | Task              | `Alt + Shift + T`       |
| Toggle focus                     | Terminal Panel    | ``Control + ` ``        |
| Toggle                           | Theme Selector    | `⌘ + K, ⌘ + T`          |
| Activate pane 1                  | Workspace         | `⌘ + 1`                 |
| Activate pane 2                  | Workspace         | `⌘ + 2`                 |
| Activate pane 3                  | Workspace         | `⌘ + 3`                 |
| Activate pane 4                  | Workspace         | `⌘ + 4`                 |
| Activate pane 5                  | Workspace         | `⌘ + 5`                 |
| Activate pane 6                  | Workspace         | `⌘ + 6`                 |
| Activate pane 7                  | Workspace         | `⌘ + 7`                 |
| Activate pane 8                  | Workspace         | `⌘ + 8`                 |
| Activate pane 9                  | Workspace         | `⌘ + 9`                 |
| Activate pane in direction down  | Workspace         | `⌘ + K, ⌘ + Down`       |
| Activate pane in direction left  | Workspace         | `⌘ + K, ⌘ + Left`       |
| Activate pane in direction right | Workspace         | `⌘ + K, ⌘ + Right`      |
| Activate pane in direction up    | Workspace         | `⌘ + K, ⌘ + Up`         |
| Close all docks                  | Workspace         | `Alt + ⌘ + Y`           |
| New file                         | Workspace         | `⌘ + N`                 |
| New terminal                     | Workspace         | `Control + ~`           |
| New window                       | Workspace         | `⌘ + Shift + N`         |
| Save                             | Workspace         | `⌘ + S`                 |
| Save all                         | Workspace         | `⌘ + Alt + S`           |
| Save as                          | Workspace         | `⌘ + Shift + S`         |
| Save without format              | Workspace         | `⌘ + K, S`              |
| Swap pane in direction           | Workspace         | `⌘ + K, Shift + Down`   |
| Swap pane in direction           | Workspace         | `⌘ + K, Shift + Left`   |
| Swap pane in direction           | Workspace         | `⌘ + K, Shift + Right`  |
| Swap pane in direction           | Workspace         | `⌘ + K, Shift + Up`     |
| Toggle bottom dock               | Workspace         | `⌘ + J`                 |
| Toggle left dock                 | Workspace         | `⌘ + B`                 |
| Toggle right dock                | Workspace         | `⌘ + R`                 |
| Unfollow                         | Workspace         | `Escape`                |
| Open keymap                      | Zed               | `⌘ + K, ⌘ + S`          |

#### Project Panel

| **Command**             | **Target**    | **Default Shortcut**  |
| ----------------------- | ------------- | --------------------- |
| Collapse selected entry | Project Panel | `Left`                |
| Copy                    | Project Panel | `⌘ + C`               |
| Copy path               | Project Panel | `⌘ + Alt + C`         |
| Copy relative path      | Project Panel | `Alt + ⌘ + Shift + C` |
| Cut                     | Project Panel | `⌘ + X`               |
| Delete                  | Project Panel | `Backspace`           |
| Delete                  | Project Panel | `Delete`              |
| Delete                  | Project Panel | `⌘ + Backspace`       |
| Delete                  | Project Panel | `⌘ + Delete`          |
| Expand selected entry   | Project Panel | `Right`               |
| New directory           | Project Panel | `Alt + ⌘ + N`         |
| New file                | Project Panel | `⌘ + N`               |
| New search in directory | Project Panel | `Alt + Shift + F`     |
| Open                    | Project Panel | `Space`               |
| Paste                   | Project Panel | `⌘ + V`               |
| Rename                  | Project Panel | `Enter`               |
| Rename                  | Project Panel | `F2`                  |
| Reveal in File Manager  | Project Panel | `Alt + ⌘ + R`         |

#### Project Search Bar

| **Command**            | **Target**     | **Default Shortcut** |
| ---------------------- | -------------- | -------------------- |
| Search in new          | Project Search | `⌘ + Enter`          |
| Toggle focus           | Project Search | `Escape`             |
| Activate regex mode    | Search         | `Alt + ⌘ + G`        |
| Activate text mode     | Search         | `Alt + ⌘ + X`        |
| Cycle mode             | Search         | `Alt + Tab`          |
| Focus search           | Search         | `⌘ + Shift + F`      |
| Next history query     | Search         | `Down`               |
| Previous history query | Search         | `Up`                 |
| Replace all            | Search         | `⌘ + Enter`          |
| Replace next           | Search         | `Enter`              |
| Toggle replace         | Search         | `⌘ + Shift + H`      |

#### Terminal

| **Command**                 | **Target** | **Default Shortcut**  |
| --------------------------- | ---------- | --------------------- |
| Clear                       | Terminal   | `⌘ + K`               |
| Copy                        | Terminal   | `⌘ + C`               |
| Delete line                 | Terminal   | `⌘ + Backspace`       |
| Move to beginning of line   | Terminal   | `⌘ + Left`            |
| Move to end of line         | Terminal   | `⌘ + Right`           |
| Move to next word end       | Terminal   | `Alt + Right`         |
| Move to previous word start | Terminal   | `Alt + Left`          |
| Paste                       | Terminal   | `⌘ + V`               |
| Show character palette      | Terminal   | `Control + ⌘ + Space` |

#### Assistant Editor

| **Command**        | **Target** | **Default Shortcut** |
| ------------------ | ---------- | -------------------- |
| Assist             | Assistant  | `⌘ + Enter`          |
| Cycle message role | Assistant  | `Control + R`        |
| Quote selection    | Assistant  | `⌘ + >`              |
| Split              | Assistant  | `Shift + Enter`      |
| Save               | Workspace  | `⌘ + S`              |
