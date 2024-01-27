Zed can be configured via a simple JSON file located at `~/.config/zed/keymap.json`.

### Predefined keymaps

We have a growing collection of pre-defined keymaps in our [keymaps repository](https://github.com/zed-industries/keymaps).

### Custom key bindings

#### Accessing custom key bindings

You can open `keymap.json` via `CMD + K, CMD + S`, the command palette, or the `Zed > Settings > Open Key Bindings` application menu item.

#### Adding a custom key binding

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

### All key bindings

#### Global

| **Command**                      | **Target**     | **Default Shortcut**           |
| -------------------------------- | -------------- | ------------------------------ |
| Open recent                      | Branches       | `Alt + Command + B`            |
| Toggle focus                     | Collab Panel   | `Command + Shift + C`          |
| Toggle inlay hints               | Editor         | `Control + :`                  |
| Cancel                           | Menu           | `Control + C`                  |
| Cancel                           | Menu           | `Escape`                       |
| Confirm                          | Menu           | `Enter`                        |
| Secondary confirm                | Menu           | `Command + Enter`              |
| Select first                     | Menu           | `Command + Up`                 |
| Select first                     | Menu           | `Page Up`                      |
| Select first                     | Menu           | `Shift + Page Down`            |
| Select first                     | Menu           | `Shift + Page Up`              |
| Select last                      | Menu           | `Command + Down`               |
| Select last                      | Menu           | `Page Down`                    |
| Select next                      | Menu           | `Control + N`                  |
| Select next                      | Menu           | `Down`                         |
| Select prev                      | Menu           | `Control + P`                  |
| Select prev                      | Menu           | `Up`                           |
| Show context menu                | Menu           | `Control + Enter`              |
| Activate next item               | Pane           | `Alt + Command + Right`        |
| Activate next item               | Pane           | `Command + }`                  |
| Activate prev item               | Pane           | `Alt + Command + Left`         |
| Activate prev item               | Pane           | `Command + {`                  |
| Close active item                | Pane           | `Command + W`                  |
| Close all items                  | Pane           | `Command + K, Command + W`     |
| Close clean items                | Pane           | `Command + K, U`               |
| Close inactive items             | Pane           | `Alt + Command + T`            |
| Open recent                      | Projects       | `Alt + Command + O`            |
| Toggle focus                     | Terminal Panel | ``Control + ` ``               |
| Activate pane in direction down  | Workspace      | `Command + K, Command + Down`  |
| Activate pane in direction left  | Workspace      | `Command + K, Command + Left`  |
| Activate pane in direction right | Workspace      | `Command + K, Command + Right` |
| Activate pane in direction up    | Workspace      | `Command + K, Command + Up`    |
| Close inactive tabs and panes    | Workspace      | `Control + Alt + Command + W`  |
| Close window                     | Workspace      | `Command + Shift + W`          |
| Follow next collaborator         | Workspace      | `Control + Alt + Command + F`  |
| New file                         | Workspace      | `Command + N`                  |
| New terminal                     | Workspace      | `Control + ~`                  |
| New window                       | Workspace      | `Command + Shift + N`          |
| Open                             | Workspace      | `Command + O`                  |
| Save                             | Workspace      | `Command + S`                  |
| Save as                          | Workspace      | `Command + Shift + S`          |
| Swap pane in direction           | Workspace      | `Command + K, Shift + Down`    |
| Swap pane in direction           | Workspace      | `Command + K, Shift + Left`    |
| Swap pane in direction           | Workspace      | `Command + K, Shift + Right`   |
| Swap pane in direction           | Workspace      | `Command + K, Shift + Up`      |
| Toggle zoom                      | Workspace      | `Shift + Escape`               |
| Debug elements                   | Zed            | `Command + Alt + I`            |
| Decrease buffer font size        | Zed            | `Command + `                   |
| Hide                             | Zed            | `Command + H`                  |
| Hide others                      | Zed            | `Alt + Command + H`            |
| Increase buffer font size        | Zed            | `Command + +`                  |
| Increase buffer font size        | Zed            | `Command + =`                  |
| Minimize                         | Zed            | `Command + M`                  |
| Open settings                    | Zed            | `Command + ,`                  |
| Quit                             | Zed            | `Command + Q`                  |
| Reset buffer font size           | Zed            | `Command + 0`                  |
| Toggle full screen               | Zed            | `Control + Command + F`        |

#### Editor

| **Command**                      | **Target** | **Default Shortcut**                 |
| -------------------------------- | ---------- | ------------------------------------ |
| Inline assist                    | Assistant  | `Control + Enter`                    |
| Add selection above              | Editor     | `Command + Alt + Up`                 |
| Add selection above              | Editor     | `Command + Control + P`              |
| Add selection below              | Editor     | `Command + Alt + Down`               |
| Add selection below              | Editor     | `Command + Control + N`              |
| Backspace                        | Editor     | `Backspace`                          |
| Backspace                        | Editor     | `Control + H`                        |
| Backspace                        | Editor     | `Shift + Backspace`                  |
| Cancel                           | Editor     | `Escape`                             |
| Confirm code action              | Editor     | `Enter`                              |
| Confirm completion               | Editor     | `Enter`                              |
| Confirm completion               | Editor     | `Tab`                                |
| Confirm rename                   | Editor     | `Enter`                              |
| Context menu first               | Editor     | `Page Up`                            |
| Context menu last                | Editor     | `Page Down`                          |
| Context menu next                | Editor     | `Control + N`                        |
| Context menu next                | Editor     | `Down`                               |
| Context menu prev                | Editor     | `Control + P`                        |
| Context menu prev                | Editor     | `Up`                                 |
| Copy                             | Editor     | `Command + C`                        |
| Cut                              | Editor     | `Command + X`                        |
| Cut to end of line               | Editor     | `Control + K`                        |
| Delete                           | Editor     | `Control + D`                        |
| Delete                           | Editor     | `Delete`                             |
| Delete line                      | Editor     | `Control + Shift + K`                |
| Delete to beginning of line      | Editor     | `Command + Backspace`                |
| Delete to end of line            | Editor     | `Command + Delete`                   |
| Delete to next subword end       | Editor     | `Control + Alt + D`                  |
| Delete to next subword end       | Editor     | `Control + Alt + Delete`             |
| Delete to next word end          | Editor     | `Alt + D`                            |
| Delete to next word end          | Editor     | `Alt + Delete`                       |
| Delete to previous subword start | Editor     | `Control + Alt + Backspace`          |
| Delete to previous subword start | Editor     | `Control + Alt + H`                  |
| Delete to previous word start    | Editor     | `Alt + Backspace`                    |
| Delete to previous word start    | Editor     | `Alt + H`                            |
| Duplicate line                   | Editor     | `Command + Shift + D`                |
| Find all references              | Editor     | `Alt + Shift + F12`                  |
| Fold                             | Editor     | `Alt + Command + [`                  |
| Format                           | Editor     | `Command + Shift + I`                |
| Go to definition                 | Editor     | `F12`                                |
| Go to definition split           | Editor     | `Alt + F12`                          |
| Go to diagnostic                 | Editor     | `F8`                                 |
| Go to hunk                       | Editor     | `Command + F8`                       |
| Go to prev diagnostic            | Editor     | `Shift + F8`                         |
| Go to prev hunk                  | Editor     | `Command + Shift + F8`               |
| Go to type definition            | Editor     | `Command + F12`                      |
| Go to type definition split      | Editor     | `Alt + Command + F12`                |
| Hover                            | Editor     | `Command + K, Command + I`           |
| Indent                           | Editor     | `Command + ]`                        |
| Join lines                       | Editor     | `Control + J`                        |
| Move down                        | Editor     | `Control + N`                        |
| Move down                        | Editor     | `Down`                               |
| Move left                        | Editor     | `Control + B`                        |
| Move left                        | Editor     | `Left`                               |
| Move line down                   | Editor     | `Control + Command + Down`           |
| Move line up                     | Editor     | `Control + Command + Up`             |
| Move page down                   | Editor     | `Control + V`                        |
| Move page down                   | Editor     | `Shift + Page Down`                  |
| Move page up                     | Editor     | `Alt + V`                            |
| Move page up                     | Editor     | `Shift + Page Up`                    |
| Move right                       | Editor     | `Control + F`                        |
| Move right                       | Editor     | `Right`                              |
| Move to beginning                | Editor     | `Command + Up`                       |
| Move to beginning of line        | Editor     | `Command + Left`                     |
| Move to beginning of line        | Editor     | `Control + A`                        |
| Move to beginning of line        | Editor     | `Home`                               |
| Move to enclosing bracket        | Editor     | `Control + M`                        |
| Move to end                      | Editor     | `Command + Down`                     |
| Move to end of line              | Editor     | `Command + Right`                    |
| Move to end of line              | Editor     | `Control + E`                        |
| Move to end of line              | Editor     | `End`                                |
| Move to end of paragraph         | Editor     | `Control + Down`                     |
| Move to next subword end         | Editor     | `Control + Alt + F`                  |
| Move to next subword end         | Editor     | `Control + Alt + Right`              |
| Move to next word end            | Editor     | `Alt + F`                            |
| Move to next word end            | Editor     | `Alt + Right`                        |
| Move to previous subword start   | Editor     | `Control + Alt + B`                  |
| Move to previous subword start   | Editor     | `Control + Alt + Left`               |
| Move to previous word start      | Editor     | `Alt + B`                            |
| Move to previous word start      | Editor     | `Alt + Left`                         |
| Move to start of paragraph       | Editor     | `Control + Up`                       |
| Move up                          | Editor     | `Control + P`                        |
| Move up                          | Editor     | `Up`                                 |
| Next screen                      | Editor     | `Control + L`                        |
| Open excerpts                    | Editor     | `Alt + Enter`                        |
| Outdent                          | Editor     | `Command + [`                        |
| Page down                        | Editor     | `Page Down`                          |
| Page up                          | Editor     | `Page Up`                            |
| Paste                            | Editor     | `Command + V`                        |
| Redo                             | Editor     | `Command + Shift + Z`                |
| Redo selection                   | Editor     | `Command + Shift + U`                |
| Rename                           | Editor     | `F2`                                 |
| Reveal in finder                 | Editor     | `Alt + Command + R`                  |
| Select all                       | Editor     | `Command + A`                        |
| Select all matches               | Editor     | `Command + Shift + L`                |
| Select down                      | Editor     | `Control + Shift + N`                |
| Select down                      | Editor     | `Shift + Down`                       |
| Select larger syntax node        | Editor     | `Alt + Up`                           |
| Select left                      | Editor     | `Control + Shift + B`                |
| Select left                      | Editor     | `Shift + Left`                       |
| Select line                      | Editor     | `Command + L`                        |
| Select next                      | Editor     | `Command + D`                        |
| Select next                      | Editor     | `Command + K, Command + D`           |
| Select previous                  | Editor     | `Command + K, Control + Command + D` |
| Select previous                  | Editor     | `Control + Command + D`              |
| Select right                     | Editor     | `Control + Shift + F`                |
| Select right                     | Editor     | `Shift + Right`                      |
| Select smaller syntax node       | Editor     | `Alt + Down`                         |
| Select to beginning              | Editor     | `Command + Shift + Up`               |
| Select to beginning of line      | Editor     | `Command + Shift + Left`             |
| Select to beginning of line      | Editor     | `Control + Shift + A`                |
| Select to beginning of line      | Editor     | `Shift + Home`                       |
| Select to end                    | Editor     | `Command + Shift + Down`             |
| Select to end of line            | Editor     | `Command + Shift + Right`            |
| Select to end of line            | Editor     | `Control + Shift + E`                |
| Select to end of line            | Editor     | `Shift + End`                        |
| Select to end of paragraph       | Editor     | `Control + Shift + Down`             |
| Select to next subword end       | Editor     | `Control + Alt + Shift + F`          |
| Select to next subword end       | Editor     | `Control + Alt + Shift + Right`      |
| Select to next word end          | Editor     | `Alt + Shift + F`                    |
| Select to next word end          | Editor     | `Alt + Shift + Right`                |
| Select to previous subword start | Editor     | `Control + Alt + Shift + B`          |
| Select to previous subword start | Editor     | `Control + Alt + Shift + Left`       |
| Select to previous word start    | Editor     | `Alt + Shift + B`                    |
| Select to previous word start    | Editor     | `Alt + Shift + Left`                 |
| Select to start of paragraph     | Editor     | `Control + Shift + Up`               |
| Select up                        | Editor     | `Control + Shift + P`                |
| Select up                        | Editor     | `Shift + Up`                         |
| Show character palette           | Editor     | `Control + Command + Space`          |
| Show completions                 | Editor     | `Control + Space`                    |
| Tab                              | Editor     | `Tab`                                |
| Tab prev                         | Editor     | `Shift + Tab`                        |
| Toggle code actions              | Editor     | `Command + .`                        |
| Toggle comments                  | Editor     | `Command + /`                        |
| Transpose                        | Editor     | `Control + T`                        |
| Undo                             | Editor     | `Command + Z`                        |
| Undo selection                   | Editor     | `Command + U`                        |
| Unfold lines                     | Editor     | `Alt + Command + ]`                  |

#### Editor (Full Only)

| **Command**         | **Target**    | **Default Shortcut**      |
| ------------------- | ------------- | ------------------------- |
| Quote selection     | Assistant     | `Command + >`             |
| Deploy              | Buffer Search | `Command + E`             |
| Deploy              | Buffer Search | `Command + F`             |
| Next suggestion     | Copilot       | `Alt + ]`                 |
| Previous suggestion | Copilot       | `Alt + [`                 |
| Suggest             | Copilot       | `Alt + \`                 |
| Newline             | Editor        | `Enter`                   |
| Newline             | Editor        | `Shift + Enter`           |
| Newline above       | Editor        | `Command + Shift + Enter` |
| Newline below       | Editor        | `Command + Enter`         |
| Toggle soft wrap    | Editor        | `Alt + Z`                 |
| Toggle              | Go To Line    | `Control + G`             |
| Toggle              | Outline       | `Command + Shift + O`     |

#### Editor (Auto Height Only)

| **Command**   | **Target** | **Default Shortcut**      |
| ------------- | ---------- | ------------------------- |
| Newline       | Editor     | `Control + Enter`         |
| Newline below | Editor     | `Control + Shift + Enter` |

#### Pane

| **Command**            | **Target**     | **Default Shortcut**  |
| ---------------------- | -------------- | --------------------- |
| Activate item 1        | Pane           | `Control + 1`         |
| Activate item 2        | Pane           | `Control + 2`         |
| Activate item 3        | Pane           | `Control + 3`         |
| Activate item 4        | Pane           | `Control + 4`         |
| Activate item 5        | Pane           | `Control + 5`         |
| Activate item 6        | Pane           | `Control + 6`         |
| Activate item 7        | Pane           | `Control + 7`         |
| Activate item 8        | Pane           | `Control + 8`         |
| Activate item 9        | Pane           | `Control + 9`         |
| Activate last item     | Pane           | `Control + 0`         |
| Go back                | Pane           | `Control + `          |
| Go forward             | Pane           | `Control + _`         |
| Reopen closed item     | Pane           | `Command + Shift + T` |
| Split down             | Pane           | `Command + K, Down`   |
| Split left             | Pane           | `Command + K, Left`   |
| Split right            | Pane           | `Command + K, Right`  |
| Split up               | Pane           | `Command + K, Up`     |
| Toggle filters         | Project Search | `Alt + Command + F`   |
| Toggle focus           | Project Search | `Command + F`         |
| Toggle focus           | Project Search | `Command + Shift + F` |
| Activate regex mode    | Search         | `Alt + Command + G`   |
| Activate semantic mode | Search         | `Alt + Command + S`   |
| Activate text mode     | Search         | `Alt + Command + X`   |
| Cycle mode             | Search         | `Alt + Tab`           |
| Select all matches     | Search         | `Alt + Enter`         |
| Select next match      | Search         | `Command + G`         |
| Select prev match      | Search         | `Command + Shift + G` |
| Toggle case sensitive  | Search         | `Alt + Command + C`   |
| Toggle replace         | Search         | `Command + Shift + H` |
| Toggle whole word      | Search         | `Alt + Command + W`   |

#### Buffer Search Bar

| **Command**            | **Target**    | **Default Shortcut** |
| ---------------------- | ------------- | -------------------- |
| Dismiss                | Buffer Search | `Escape`             |
| Focus editor           | Buffer Search | `Tab`                |
| Cycle mode             | Search        | `Alt + Tab`          |
| Next history query     | Search        | `Down`               |
| Previous history query | Search        | `Up`                 |
| Replace all            | Search        | `Command + Enter`    |
| Replace next           | Search        | `Enter`              |
| Select all matches     | Search        | `Alt + Enter`        |
| Select next match      | Search        | `Enter`              |
| Select prev match      | Search        | `Shift + Enter`      |

#### Workspace

| **Command**        | **Target**        | **Default Shortcut**       |
| ------------------ | ----------------- | -------------------------- |
| Toggle focus       | Assistant         | `Command + ?`              |
| Toggle             | Command Palette   | `Command + Shift + P`      |
| Deploy             | Diagnostics       | `Command + Shift + M`      |
| Toggle             | File Finder       | `Command + P`              |
| Toggle             | Language Selector | `Command + K, M`           |
| Toggle focus       | Project Panel     | `Command + Shift + E`      |
| Toggle             | Project Symbols   | `Command + T`              |
| Toggle             | Theme Selector    | `Command + K, Command + T` |
| Activate pane 1    | Workspace         | `Command + 1`              |
| Activate pane 2    | Workspace         | `Command + 2`              |
| Activate pane 3    | Workspace         | `Command + 3`              |
| Activate pane 4    | Workspace         | `Command + 4`              |
| Activate pane 5    | Workspace         | `Command + 5`              |
| Activate pane 6    | Workspace         | `Command + 6`              |
| Activate pane 7    | Workspace         | `Command + 7`              |
| Activate pane 8    | Workspace         | `Command + 8`              |
| Activate pane 9    | Workspace         | `Command + 9`              |
| Close all docks    | Workspace         | `Alt + Command + Y`        |
| New search         | Workspace         | `Command + Shift + F`      |
| Save all           | Workspace         | `Command + Alt + S`        |
| Toggle bottom dock | Workspace         | `Command + J`              |
| Toggle left dock   | Workspace         | `Command + B`              |
| Toggle right dock  | Workspace         | `Command + R`              |
| Open keymap        | Zed               | `Command + K, Command + S` |

#### Project Panel

| **Command**             | **Target**    | **Default Shortcut**        |
| ----------------------- | ------------- | --------------------------- |
| Collapse selected entry | Project Panel | `Left`                      |
| Copy                    | Project Panel | `Command + C`               |
| Copy path               | Project Panel | `Command + Alt + C`         |
| Copy relative path      | Project Panel | `Alt + Command + Shift + C` |
| Cut                     | Project Panel | `Command + X`               |
| Delete                  | Project Panel | `Backspace`                 |
| Expand selected entry   | Project Panel | `Right`                     |
| New directory           | Project Panel | `Alt + Command + N`         |
| New file                | Project Panel | `Command + N`               |
| New search in directory | Project Panel | `Alt + Shift + F`           |
| Open                    | Project Panel | `Space`                     |
| Paste                   | Project Panel | `Command + V`               |
| Rename                  | Project Panel | `Enter`                     |
| Rename                  | Project Panel | `F2`                        |
| Reveal in finder        | Project Panel | `Alt + Command + R`         |

#### Project Search Bar

| **Command**            | **Target**     | **Default Shortcut**  |
| ---------------------- | -------------- | --------------------- |
| Search in new          | Project Search | `Command + Enter`     |
| Toggle focus           | Project Search | `Escape`              |
| Activate regex mode    | Search         | `Alt + Command + G`   |
| Activate semantic mode | Search         | `Alt + Command + S`   |
| Activate text mode     | Search         | `Alt + Command + X`   |
| Cycle mode             | Search         | `Alt + Tab`           |
| Next history query     | Search         | `Down`                |
| Previous history query | Search         | `Up`                  |
| Replace all            | Search         | `Command + Enter`     |
| Replace next           | Search         | `Enter`               |
| Toggle replace         | Search         | `Command + Shift + H` |

#### Terminal

| **Command**                 | **Target** | **Default Shortcut**        |
| --------------------------- | ---------- | --------------------------- |
| Clear                       | Terminal   | `Command + K`               |
| Copy                        | Terminal   | `Command + C`               |
| Delete line                 | Terminal   | `Command + Backspace`       |
| Move to beginning of line   | Terminal   | `Command + Left`            |
| Move to end of line         | Terminal   | `Command + Right`           |
| Move to next word end       | Terminal   | `Alt + Right`               |
| Move to previous word start | Terminal   | `Alt + Left`                |
| Paste                       | Terminal   | `Command + V`               |
| Show character palette      | Terminal   | `Control + Command + Space` |

#### Assistant Editor

| **Command**        | **Target** | **Default Shortcut** |
| ------------------ | ---------- | -------------------- |
| Assist             | Assistant  | `Command + Enter`    |
| Cycle message role | Assistant  | `Control + R`        |
| Quote selection    | Assistant  | `Command + >`        |
| Split              | Assistant  | `Shift + Enter`      |
| Save               | Workspace  | `Command + S`        |
