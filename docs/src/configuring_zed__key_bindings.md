Zed can be configured via a simple JSON file located at `~/.config/zed/keymap.json`.

## Predefined keymaps

We have a growing collection of pre-defined keymaps in [zed repository's keymaps folder](https://github.com/zed-industries/zed/tree/main/assets/keymaps).

A selection of base keymaps is available in the welcome screen under the "Choose a keymap" option.

Additionally, you can change the base keymap from the command palette - `⌘-Shift-P` - by selecting the "welcome: toggle base keymap selector" command.

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

You can see more examples in Zed's [`default.json`](https://zed.dev/ref/default.json)

_There are some key bindings that can't be overridden; we are working on an issue surrounding this._

## Special Keyboard Layouts

Some people have unique and custom keyboard layouts.

For example, [@TomPlanche](https://github.com/TomPlanche) having a [French keyboard](https%3A%2F%2Fcdn.shopify.com%2Fs%2Ffiles%2F1%2F0810%2F3669%2Ffiles%2Ffrench-azerty-mac-keyboard-layout-2021-keyshorts.png&f=1&nofb=1&ipt=f53a06c5e60a20b621082410aa699c8cceff269a11ff90b3b5a35c6124dbf827&ipo=images), had to type `Shift-Alt-(` in order to have a simple `[` so he made a simple layout with those 'rules':
`ù -> [`, `backtick -> ]`, `Alt-[ (where [ is the old ù) -> {`, `Alt-] -> }`.
But, it was impossible to take into account the `{` and `}` when he was typing so now, in order to ignore a binding, he can add `null` to the binding:

```json
[
  {
    "context": "Editor",
    "bindings": {
      "alt-[": null,
      "alt-]": null
    }
  }
]
```

## All key bindings

### Global

| **Command**                      | **Target**     | **Default Shortcut**          |
| -------------------------------- | -------------- | ----------------------------- |
| Open recent                      | Branches       | `Alt` + `⌘` + `B`             |
| Toggle focus                     | Collab Panel   | `⌘` + `Shift` + `C`           |
| Toggle inlay hints               | Editor         | `Control` + `:`               |
| Cancel                           | Menu           | `Control` + `C`               |
| Cancel                           | Menu           | `Escape`                      |
| Confirm                          | Menu           | `Enter`                       |
| Secondary confirm                | Menu           | `⌘` + `Enter`                 |
| Select first                     | Menu           | `⌘` + `Up`                    |
| Select first                     | Menu           | `Page Up`                     |
| Select first                     | Menu           | `Shift` + `Page Down`         |
| Select first                     | Menu           | `Shift` + `Page Up`           |
| Select last                      | Menu           | `⌘` + `Down`                  |
| Select last                      | Menu           | `Page Down`                   |
| Select next                      | Menu           | `Control` + `N`               |
| Select next                      | Menu           | `Down`                        |
| Select prev                      | Menu           | `Control` + `P`               |
| Select prev                      | Menu           | `Up`                          |
| Show context menu                | Menu           | `Control` + `Enter`           |
| Activate next item               | Pane           | `Alt` + `⌘` + `Right`         |
| Activate next item               | Pane           | `⌘` + `}`                     |
| Activate prev item               | Pane           | `Alt` + `⌘` + `Left`          |
| Activate prev item               | Pane           | `⌘` + `{`                     |
| Close active item                | Pane           | `⌘` + `W`                     |
| Close all items                  | Pane           | `⌘` + `K`, `⌘` + `W`          |
| Close clean items                | Pane           | `⌘` + `K`, `U`                |
| Close inactive items             | Pane           | `Alt` + `⌘` + `T`             |
| Open recent                      | Projects       | `Alt` + `⌘` + `O`             |
| Toggle focus                     | Terminal Panel | `Control` + `` ` ``           |
| Activate pane in direction down  | Workspace      | `⌘` + `K`, `⌘` + `Down`       |
| Activate pane in direction left  | Workspace      | `⌘` + `K`, `⌘` + `Left`       |
| Activate pane in direction right | Workspace      | `⌘` + `K`, `⌘` + `Right`      |
| Activate pane in direction up    | Workspace      | `⌘` + `K`, `⌘` + `Up`         |
| Close inactive tabs and panes    | Workspace      | `Control` + `Alt` + `⌘` + `W` |
| Close window                     | Workspace      | `⌘` + `Shift` + `W`           |
| Follow next collaborator         | Workspace      | `Control` + `Alt` + `⌘` + `F` |
| New file                         | Workspace      | `⌘` + `N`                     |
| New terminal                     | Workspace      | `Control` + `~`               |
| New window                       | Workspace      | `⌘` + `Shift` + `N`           |
| Open                             | Workspace      | `⌘` + `O`                     |
| Save                             | Workspace      | `⌘` + `S`                     |
| Save as                          | Workspace      | `⌘` + `Shift` + `S`           |
| Swap pane in direction           | Workspace      | `⌘` + `K`, `Shift` + `Down`   |
| Swap pane in direction           | Workspace      | `⌘` + `K`, `Shift` + `Left`   |
| Swap pane in direction           | Workspace      | `⌘` + `K`, `Shift` + `Right`  |
| Swap pane in direction           | Workspace      | `⌘` + `K`, `Shift` + `Up`     |
| Toggle zoom                      | Workspace      | `Shift` + `Escape`            |
| Debug elements                   | Zed            | `⌘` + `Alt` + `I`             |
| Decrease buffer font size        | Zed            | `⌘` + `` ` ``                 |
| Hide                             | Zed            | `⌘` + `H`                     |
| Hide others                      | Zed            | `Alt` + `⌘` + `H`             |
| Increase buffer font size        | Zed            | `⌘` + `+`                     |
| Increase buffer font size        | Zed            | `⌘` + `=`                     |
| Minimize                         | Zed            | `⌘` + `M`                     |
| Open settings                    | Zed            | `⌘` + `,`                     |
| Quit                             | Zed            | `⌘` + `Q`                     |
| Reset buffer font size           | Zed            | `⌘` + `0`                     |
| Toggle full screen               | Zed            | `Control` + `⌘` + `F`         |

### Editor

| **Command**                      | **Target** | **Default Shortcut**                  |
| -------------------------------- | ---------- | ------------------------------------- |
| Inline assist                    | Assistant  | `Control` + `Enter`                   |
| Add selection above              | Editor     | `⌘` + `Alt` + `Up`                    |
| Add selection above              | Editor     | `⌘` + `Control` + `P`                 |
| Add selection below              | Editor     | `⌘` + `Alt` + `Down`                  |
| Add selection below              | Editor     | `⌘` + `Control` + `N`                 |
| Backspace                        | Editor     | `Backspace`                           |
| Backspace                        | Editor     | `Control` + `H`                       |
| Backspace                        | Editor     | `Shift` + `Backspace`                 |
| Cancel                           | Editor     | `Escape`                              |
| Confirm code action              | Editor     | `Enter`                               |
| Confirm completion               | Editor     | `Enter`                               |
| Confirm completion               | Editor     | `Tab`                                 |
| Confirm rename                   | Editor     | `Enter`                               |
| Context menu first               | Editor     | `Page Up`                             |
| Context menu last                | Editor     | `Page Down`                           |
| Context menu next                | Editor     | `Control` + `N`                       |
| Context menu next                | Editor     | `Down`                                |
| Context menu prev                | Editor     | `Control` + `P`                       |
| Context menu prev                | Editor     | `Up`                                  |
| Copy                             | Editor     | `⌘` + `C`                             |
| Cut                              | Editor     | `⌘` + `X`                             |
| Cut to end of line               | Editor     | `Control` + `K`                       |
| Delete                           | Editor     | `Control` + `D`                       |
| Delete                           | Editor     | `Delete`                              |
| Delete line                      | Editor     | `Control` + `Shift` + `K`             |
| Delete to beginning of line      | Editor     | `⌘` + `Backspace`                     |
| Delete to end of line            | Editor     | `⌘` + `Delete`                        |
| Delete to next subword end       | Editor     | `Control` + `Alt` + `D`               |
| Delete to next subword end       | Editor     | `Control` + `Alt` + `Delete`          |
| Delete to next word end          | Editor     | `Alt` + `D`                           |
| Delete to next word end          | Editor     | `Alt` + `Delete`                      |
| Delete to previous subword start | Editor     | `Control` + `Alt` + `Backspace`       |
| Delete to previous subword start | Editor     | `Control` + `Alt` + `H`               |
| Delete to previous word start    | Editor     | `Alt` + `Backspace`                   |
| Delete to previous word start    | Editor     | `Alt` + `H`                           |
| Duplicate line                   | Editor     | `⌘` + `Shift` + `D`                   |
| Find all references              | Editor     | `Alt` + `Shift` + `F12`               |
| Fold                             | Editor     | `Alt` + `⌘` + `[`                     |
| Format                           | Editor     | `⌘` + `Shift` + `I`                   |
| Go to definition                 | Editor     | `F12`                                 |
| Go to definition split           | Editor     | `Alt` + `F12`                         |
| Go to diagnostic                 | Editor     | `F8`                                  |
| Go to hunk                       | Editor     | `⌘` + `F8`                            |
| Go to prev diagnostic            | Editor     | `Shift` + `F8`                        |
| Go to prev hunk                  | Editor     | `⌘` + `Shift` + `F8`                  |
| Go to type definition            | Editor     | `⌘` + `F12`                           |
| Go to type definition split      | Editor     | `Alt` + `⌘` + `F12`                   |
| Hover                            | Editor     | `⌘` + `K`, `⌘` + `I`                  |
| Indent                           | Editor     | `⌘` + `]`                             |
| Join lines                       | Editor     | `Control` + `J`                       |
| Move down                        | Editor     | `Control` + `N`                       |
| Move down                        | Editor     | `Down`                                |
| Move left                        | Editor     | `Control` + `B`                       |
| Move left                        | Editor     | `Left`                                |
| Move line down                   | Editor     | `Control` + `⌘` + `Down`              |
| Move line up                     | Editor     | `Control` + `⌘` + `Up`                |
| Move page down                   | Editor     | `Control` + `V`                       |
| Move page down                   | Editor     | `Shift` + `Page Down`                 |
| Move page up                     | Editor     | `Alt` + `V`                           |
| Move page up                     | Editor     | `Shift` + `Page Up`                   |
| Move right                       | Editor     | `Control` + `F`                       |
| Move right                       | Editor     | `Right`                               |
| Move to beginning                | Editor     | `⌘` + `Up`                            |
| Move to beginning of line        | Editor     | `⌘` + `Left`                          |
| Move to beginning of line        | Editor     | `Control` + `A`                       |
| Move to beginning of line        | Editor     | `Home`                                |
| Move to enclosing bracket        | Editor     | `Control` + `M`                       |
| Move to end                      | Editor     | `⌘` + `Down`                          |
| Move to end of line              | Editor     | `⌘` + `Right`                         |
| Move to end of line              | Editor     | `Control` + `E`                       |
| Move to end of line              | Editor     | `End`                                 |
| Move to end of paragraph         | Editor     | `Control` + `Down`                    |
| Move to next subword end         | Editor     | `Control` + `Alt` + `F`               |
| Move to next subword end         | Editor     | `Control` + `Alt` + `Right`           |
| Move to next word end            | Editor     | `Alt` + `F`                           |
| Move to next word end            | Editor     | `Alt` + `Right`                       |
| Move to previous subword start   | Editor     | `Control` + `Alt` + `B`               |
| Move to previous subword start   | Editor     | `Control` + `Alt` + `Left`            |
| Move to previous word start      | Editor     | `Alt` + `B`                           |
| Move to previous word start      | Editor     | `Alt` + `Left`                        |
| Move to start of paragraph       | Editor     | `Control` + `Up`                      |
| Move up                          | Editor     | `Control` + `P`                       |
| Move up                          | Editor     | `Up`                                  |
| Next screen                      | Editor     | `Control` + `L`                       |
| Open excerpts                    | Editor     | `Alt` + `Enter`                       |
| Outdent                          | Editor     | `⌘` + `[`                             |
| Page down                        | Editor     | `Page Down`                           |
| Page up                          | Editor     | `Page Up`                             |
| Paste                            | Editor     | `⌘` + `V`                             |
| Redo                             | Editor     | `⌘` + `Shift` + `Z`                   |
| Redo selection                   | Editor     | `⌘` + `Shift` + `U`                   |
| Rename                           | Editor     | `F2`                                  |
| Reveal in finder                 | Editor     | `Alt` + `⌘` + `R`                     |
| Select all                       | Editor     | `⌘` + `A`                             |
| Select all matches               | Editor     | `⌘` + `Shift` + `L`                   |
| Select down                      | Editor     | `Control` + `Shift` + `N`             |
| Select down                      | Editor     | `Shift` + `Down`                      |
| Select larger syntax node        | Editor     | `Alt` + `Up`                          |
| Select left                      | Editor     | `Control` + `Shift` + `B`             |
| Select left                      | Editor     | `Shift` + `Left`                      |
| Select line                      | Editor     | `⌘` + `L`                             |
| Select next                      | Editor     | `⌘` + `D`                             |
| Select next                      | Editor     | `⌘` + `K`, `⌘` + `D`                  |
| Select previous                  | Editor     | `⌘` + `K`, `Control` + `⌘` + `D`      |
| Select previous                  | Editor     | `Control` + `⌘` + `D`                 |
| Select right                     | Editor     | `Control` + `Shift` + `F`             |
| Select right                     | Editor     | `Shift` + `Right`                     |
| Select smaller syntax node       | Editor     | `Alt` + `Down`                        |
| Select to beginning              | Editor     | `⌘` + `Shift` + `Up`                  |
| Select to beginning of line      | Editor     | `⌘` + `Shift` + `Left`                |
| Select to beginning of line      | Editor     | `Control` + `Shift` + `A`             |
| Select to beginning of line      | Editor     | `Shift` + `Home`                      |
| Select to end                    | Editor     | `⌘` + `Shift` + `Down`                |
| Select to end of line            | Editor     | `⌘` + `Shift` + `Right`               |
| Select to end of line            | Editor     | `Control` + `Shift` + `E`             |
| Select to end of line            | Editor     | `Shift` + `End`                       |
| Select to end of paragraph       | Editor     | `Control` + `Shift` + `Down`          |
| Select to next subword end       | Editor     | `Control` + `Alt` + `Shift` + `F`     |
| Select to next subword end       | Editor     | `Control` + `Alt` + `Shift` + `Right` |
| Select to next word end          | Editor     | `Alt` + `Shift` + `F`                 |
| Select to next word end          | Editor     | `Alt` + `Shift` + `Right`             |
| Select to previous subword start | Editor     | `Control` + `Alt` + `Shift` + `B`     |
| Select to previous subword start | Editor     | `Control` + `Alt` + `Shift` + `Left`  |
| Select to previous word start    | Editor     | `Alt` + `Shift` + `B`                 |
| Select to previous word start    | Editor     | `Alt` + `Shift` + `Left`              |
| Select to start of paragraph     | Editor     | `Control` + `Shift` + `Up`            |
| Select up                        | Editor     | `Control` + `Shift` + `P`             |
| Select up                        | Editor     | `Shift` + `Up`                        |
| Show character palette           | Editor     | `Control` + `⌘` + `Space`             |
| Show completions                 | Editor     | `Control` + `Space`                   |
| Tab                              | Editor     | `Tab`                                 |
| Tab prev                         | Editor     | `Shift` + `Tab`                       |
| Toggle code actions              | Editor     | `⌘` + `.`                             |
| Toggle comments                  | Editor     | `⌘` + `/`                             |
| Transpose                        | Editor     | `Control` + `T`                       |
| Undo                             | Editor     | `⌘` + `Z`                             |
| Undo selection                   | Editor     | `⌘` + `U`                             |
| Unfold lines                     | Editor     | `Alt` + `⌘` + `]`                     |

### Editor (Full Only)

| **Command**         | **Target**    | **Default Shortcut**    |
| ------------------- | ------------- | ----------------------- |
| Quote selection     | Assistant     | `⌘` + `>`               |
| Deploy              | Buffer Search | `⌘` + `E`               |
| Deploy              | Buffer Search | `⌘` + `F`               |
| Next suggestion     | Copilot       | `Alt` + `]`             |
| Previous suggestion | Copilot       | `Alt` + `[`             |
| Suggest             | Copilot       | `Alt` + `\`             |
| Newline             | Editor        | `Enter`                 |
| Newline             | Editor        | `Shift` + `Enter`       |
| Newline above       | Editor        | `⌘` + `Shift` + `Enter` |
| Newline below       | Editor        | `⌘` + `Enter`           |
| Toggle soft wrap    | Editor        | `Alt` + `Z`             |
| Toggle              | Go To Line    | `Control` + `G`         |
| Toggle              | Outline       | `⌘` + `Shift` + `O`     |

### Editor (Auto Height Only)

| **Command**   | **Target** | **Default Shortcut**          |
| ------------- | ---------- | ----------------------------- |
| Newline       | Editor     | `Control` + `Enter`           |
| Newline below | Editor     | `Control` + `Shift` + `Enter` |

### Pane

| **Command**            | **Target**     | **Default Shortcut** |
| ---------------------- | -------------- | -------------------- |
| Activate item 1        | Pane           | `Control` + `1`      |
| Activate item 2        | Pane           | `Control` + `2`      |
| Activate item 3        | Pane           | `Control` + `3`      |
| Activate item 4        | Pane           | `Control` + `4`      |
| Activate item 5        | Pane           | `Control` + `5`      |
| Activate item 6        | Pane           | `Control` + `6`      |
| Activate item 7        | Pane           | `Control` + `7`      |
| Activate item 8        | Pane           | `Control` + `8`      |
| Activate item 9        | Pane           | `Control` + `9`      |
| Activate last item     | Pane           | `Control` + `0`      |
| Go back                | Pane           | `Control` + `-`      |
| Go forward             | Pane           | `Control` + `_`      |
| Reopen closed item     | Pane           | `⌘` + `Shift` + `T`  |
| Split down             | Pane           | `⌘` + `K`, `Down`    |
| Split left             | Pane           | `⌘` + `K`, `Left`    |
| Split right            | Pane           | `⌘` + `K`, `Right`   |
| Split up               | Pane           | `⌘` + `K`, `Up`      |
| Toggle filters         | Project Search | `Alt` + `⌘` + `F`    |
| Toggle focus           | Project Search | `⌘` + `F`            |
| Toggle focus           | Project Search | `⌘` + `Shift` + `F`  |
| Activate regex mode    | Search         | `Alt` + `⌘` + `G`    |
| Activate semantic mode | Search         | `Alt` + `⌘` + `S`    |
| Activate text mode     | Search         | `Alt` + `⌘` + `X`    |
| Cycle mode             | Search         | `Alt` + `Tab`        |
| Select all matches     | Search         | `Alt` + `Enter`      |
| Select next match      | Search         | `⌘` + `G`            |
| Select prev match      | Search         | `⌘` + `Shift` + `G`  |
| Toggle case sensitive  | Search         | `Alt` + `⌘` + `C`    |
| Toggle replace         | Search         | `⌘` + `Shift` + `H`  |
| Toggle whole word      | Search         | `Alt` + `⌘` + `W`    |

### Buffer Search Bar

| **Command**            | **Target**    | **Default Shortcut** |
| ---------------------- | ------------- | -------------------- |
| Dismiss                | Buffer Search | `Escape`             |
| Focus editor           | Buffer Search | `Tab`                |
| Cycle mode             | Search        | `Alt` + `Tab`        |
| Next history query     | Search        | `Down`               |
| Previous history query | Search        | `Up`                 |
| Replace all            | Search        | `Command + Enter`    |
| Replace next           | Search        | `Enter`              |
| Select all matches     | Search        | `Alt` + `Enter`      |
| Select next match      | Search        | `Enter`              |
| Select prev match      | Search        | `Shift` + `Enter`    |

### Workspace

| **Command**        | **Target**        | **Default Shortcut** |
| ------------------ | ----------------- | -------------------- |
| Toggle focus       | Assistant         | `⌘` + `?`            |
| Toggle             | Command Palette   | `⌘` + `Shift` + `P`  |
| Deploy             | Diagnostics       | `⌘` + `Shift` + `M`  |
| Toggle             | File Finder       | `⌘` + `P`            |
| Toggle             | Language Selector | `⌘` + `K`, `M`       |
| Toggle focus       | Project Panel     | `⌘` + `Shift` + `E`  |
| Toggle             | Project Symbols   | `⌘` + `T`            |
| Toggle             | Theme Selector    | `⌘` + `K`, `⌘` + `T` |
| Activate pane 1    | Workspace         | `⌘` + `1`            |
| Activate pane 2    | Workspace         | `⌘` + `2`            |
| Activate pane 3    | Workspace         | `⌘` + `3`            |
| Activate pane 4    | Workspace         | `⌘` + `4`            |
| Activate pane 5    | Workspace         | `⌘` + `5`            |
| Activate pane 6    | Workspace         | `⌘` + `6`            |
| Activate pane 7    | Workspace         | `⌘` + `7`            |
| Activate pane 8    | Workspace         | `⌘` + `8`            |
| Activate pane 9    | Workspace         | `⌘` + `9`            |
| Close all docks    | Workspace         | `Alt` + `⌘` + `Y`    |
| New search         | Workspace         | `⌘` + `Shift` + `F`  |
| Save all           | Workspace         | `⌘` + `Alt` + `S`    |
| Toggle bottom dock | Workspace         | `⌘` + `J`            |
| Toggle left dock   | Workspace         | `⌘` + `B`            |
| Toggle right dock  | Workspace         | `⌘` + `R`            |
| Open keymap        | Zed               | `⌘` + `K`, `⌘` + `S` |

### Project Panel

| **Command**             | **Target**    | **Default Shortcut**        |
| ----------------------- | ------------- | --------------------------- |
| Collapse selected entry | Project Panel | `Left`                      |
| Copy                    | Project Panel | `⌘` + `C`                   |
| Copy path               | Project Panel | `⌘` + `Alt` + `C`           |
| Copy relative path      | Project Panel | `Alt` + `⌘` + `Shift` + `C` |
| Cut                     | Project Panel | `⌘` + `X`                   |
| Delete                  | Project Panel | `Backspace`                 |
| Expand selected entry   | Project Panel | `Right`                     |
| New directory           | Project Panel | `Alt` + `⌘` + `N`           |
| New file                | Project Panel | `Command + N`               |
| New search in directory | Project Panel | `Alt` + `Shift` + `F`       |
| Open                    | Project Panel | `Space`                     |
| Paste                   | Project Panel | `⌘` + `V`                   |
| Rename                  | Project Panel | `Enter`                     |
| Rename                  | Project Panel | `F2`                        |
| Reveal in finder        | Project Panel | `Alt` + `⌘` + `R`           |

### Project Search Bar

| **Command**            | **Target**     | **Default Shortcut** |
| ---------------------- | -------------- | -------------------- |
| Search in new          | Project Search | `⌘` + `Enter`        |
| Toggle focus           | Project Search | `Escape`             |
| Activate regex mode    | Search         | `Alt` + `⌘` + `G`    |
| Activate semantic mode | Search         | `Alt` + `⌘` + `S`    |
| Activate text mode     | Search         | `Alt` + `⌘` + `X`    |
| Cycle mode             | Search         | `Alt` + `Tab`        |
| Next history query     | Search         | `Down`               |
| Previous history query | Search         | `Up`                 |
| Replace all            | Search         | `Command + Enter`    |
| Replace next           | Search         | `Enter`              |
| Toggle replace         | Search         | `⌘` + `Shift` + `H`  |

### Terminal

| **Command**                 | **Target** | **Default Shortcut**      |
| --------------------------- | ---------- | ------------------------- |
| Clear                       | Terminal   | `⌘` + `K`                 |
| Copy                        | Terminal   | `⌘` + `C`                 |
| Delete line                 | Terminal   | `⌘` + `Backspace`         |
| Move to beginning of line   | Terminal   | `⌘` + `Left`              |
| Move to end of line         | Terminal   | `⌘` + `Right`             |
| Move to next word end       | Terminal   | `Alt` + `Right`           |
| Move to previous word start | Terminal   | `Alt` + `Left`            |
| Paste                       | Terminal   | `⌘` + `V`                 |
| Show character palette      | Terminal   | `Control` + `⌘` + `Space` |

### Assistant Editor

| **Command**        | **Target** | **Default Shortcut** |
| ------------------ | ---------- | -------------------- |
| Assist             | Assistant  | `⌘` + `Enter`        |
| Cycle message role | Assistant  | `Control` + `R`      |
| Quote selection    | Assistant  | `⌘` + `>`            |
| Split              | Assistant  | `Shift` + `Enter`    |
| Save               | Workspace  | `⌘` + `S`            |
