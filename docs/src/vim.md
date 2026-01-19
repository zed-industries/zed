# Vim Mode

Zed includes a Vim emulation layer known as "vim mode". On this page, you will learn how to turn Zed's vim mode on or off, what tools and commands Zed provides to help you navigate and edit your code, and generally how to make the most of vim mode in Zed.

You'll learn how to:

- Understand the core differences between Zed's vim mode and traditional Vim
- Enable or disable vim mode
- Make the most of Zed-specific features within vim mode
- Customize vim mode key bindings
- Configure vim mode settings

Whether you're new to vim mode or an experienced Vim user looking to optimize your Zed experience, this guide will help you harness the full power of modal editing in Zed.

## Zed's vim mode design

Vim mode tries to offer a familiar experience to Vim users: it replicates the behavior of motions and commands precisely when it makes sense and uses Zed-specific functionality to provide an editing experience that "just works" without requiring configuration on your part.

This includes support for semantic navigation, multiple cursors, or other features usually provided by plugins like surrounding text.

So, Zed's vim mode does not replicate Vim one-to-one, but it meshes Vim's modal design with Zed's modern features to provide a more fluid experience. It's also configurable, so you can add your own key bindings or override the defaults.

### Core differences

There are four types of features in vim mode that use Zed's core functionality, leading to some differences in behavior:

1. **Motions**: vim mode uses Zed's semantic parsing to tune the behavior of motions per language. For example, in Rust, jumping to matching bracket with `%` works with the pipe character `|`. In JavaScript, `w` considers `$` to be a word character.
2. **Visual block selections**: vim mode uses Zed's multiple cursor to emulate visual block selections, making block selections a lot more flexible. For example, anything you insert after a block selection updates on every line in real-time, and you can add or remove cursors anytime.
3. **Macros**: vim mode uses Zed's recording system for vim macros. So, you can capture and replay more complex actions, like autocompletion.
4. **Search and replace**: vim mode uses Zed's search system, so, the syntax for regular expressions is slightly different compared to Vim. [Head to the Regex differences section](#regex-differences) for details.

> **Note:** The foundations of Zed's vim mode should already cover many use cases, and we're always looking to improve it. If you find missing features that you rely on in your workflow, please [file an issue on GitHub](https://github.com/zed-industries/zed/issues).

## Enabling and disabling vim mode

When you first open Zed, you'll see a checkbox on the welcome screen that allows you to enable vim mode.

If you missed this, you can toggle vim mode on or off anytime by opening the command palette and using the workspace command `toggle vim mode`.

> **Note**: This command toggles the following property in your user settings:
>
> ```json [settings]
> {
>   "vim_mode": true
> }
> ```

## Zed-specific features

Zed is built on a modern foundation that (among other things) uses Tree-sitter and language servers to understand the content of the file you're editing and supports multiple cursors out of the box.

Vim mode has several "core Zed" key bindings that will help you make the most of Zed's specific feature set.

### Language server

The following commands use the language server to help you navigate and refactor your code.

| Command                                  | Default Shortcut |
| ---------------------------------------- | ---------------- |
| Go to definition                         | `g d`            |
| Go to declaration                        | `g D`            |
| Go to type definition                    | `g y`            |
| Go to implementation                     | `g I`            |
| Rename (change definition)               | `c d`            |
| Go to All references to the current word | `g A`            |
| Find symbol in current file              | `g s`            |
| Find symbol in entire project            | `g S`            |
| Go to next diagnostic                    | `g ]` or `] d`   |
| Go to previous diagnostic                | `g [` or `[ d`   |
| Show inline error (hover)                | `g h`            |
| Open the code actions menu               | `g .`            |

### Git

| Command                         | Default Shortcut |
| ------------------------------- | ---------------- |
| Go to next git change           | `] c`            |
| Go to previous git change       | `[ c`            |
| Expand diff hunk                | `d o`            |
| Toggle staged                   | `d O`            |
| Stage and next (in diff view)   | `d u`            |
| Unstage and next (in diff view) | `d U`            |
| Restore change                  | `d p`            |

### Tree-sitter

Tree-sitter is a powerful tool that Zed uses to understand the structure of your code. Zed provides motions that change the current cursor position, and text objects that can be used as the target of actions.

| Command                         | Default Shortcut            |
| ------------------------------- | --------------------------- |
| Go to next/previous method      | `] m` / `[ m`               |
| Go to next/previous method end  | `] M` / `[ M`               |
| Go to next/previous section     | `] ]` / `[ [`               |
| Go to next/previous section end | `] [` / `[ ]`               |
| Go to next/previous comment     | `] /`, `] *` / `[ /`, `[ *` |
| Select a larger syntax node     | `[ x`                       |
| Select a smaller syntax node    | `] x`                       |

| Text Objects                                               | Default Shortcut |
| ---------------------------------------------------------- | ---------------- |
| Around a class, definition, etc.                           | `a c`            |
| Inside a class, definition, etc.                           | `i c`            |
| Around a function, method etc.                             | `a f`            |
| Inside a function, method, etc.                            | `i f`            |
| A comment                                                  | `g c`            |
| An argument, or list item, etc.                            | `i a`            |
| An argument, or list item, etc. (including trailing comma) | `a a`            |
| Around an HTML-like tag                                    | `a t`            |
| Inside an HTML-like tag                                    | `i t`            |
| The current indent level, and one line before and after    | `a I`            |
| The current indent level, and one line before              | `a i`            |
| The current indent level                                   | `i i`            |

Note that the definitions for the targets of the `[m` family of motions are the same as the
boundaries defined by `af`. The targets of the `[[` are the same as those defined by `ac`, though
if there are no classes, then functions are also used. Similarly `gc` is used to find `[ /`. `g c`

The definition of functions, classes and comments is language dependent, and support can be added
to extensions by adding a [`textobjects.scm`]. The definition of arguments and tags operates at
the Tree-sitter level, but looks for certain patterns in the parse tree and is not currently configurable
per language.

### Multi cursor

These commands help you manage multiple cursors in Zed.

| Command                                                      | Default Shortcut |
| ------------------------------------------------------------ | ---------------- |
| Add a cursor selecting the next copy of the current word     | `g l`            |
| Add a cursor selecting the previous copy of the current word | `g L`            |
| Skip latest word selection, and add next                     | `g >`            |
| Skip latest word selection, and add previous                 | `g <`            |
| Add a visual selection for every copy of the current word    | `g a`            |

### Pane management

These commands open new panes or jump to specific panes.

| Command                                    | Default Shortcut   |
| ------------------------------------------ | ------------------ |
| Open a project-wide search                 | `g /`              |
| Open the current search excerpt            | `g <space>`        |
| Open the current search excerpt in a split | `<ctrl-w> <space>` |
| Go to definition in a split                | `<ctrl-w> g d`     |
| Go to type definition in a split           | `<ctrl-w> g D`     |

### In insert mode

The following commands help you bring up Zed's completion menu, request a suggestion from GitHub Copilot, or open the inline AI assistant without leaving insert mode.

| Command                                                                      | Default Shortcut |
| ---------------------------------------------------------------------------- | ---------------- |
| Open the completion menu                                                     | `ctrl-x ctrl-o`  |
| Request GitHub Copilot suggestion (requires GitHub Copilot to be configured) | `ctrl-x ctrl-c`  |
| Open the inline AI assistant (requires a configured assistant)               | `ctrl-x ctrl-a`  |
| Open the code actions menu                                                   | `ctrl-x ctrl-l`  |
| Hides all suggestions                                                        | `ctrl-x ctrl-z`  |

### Supported plugins

Zed's vim mode includes some features that are usually provided by very popular plugins in the Vim ecosystem:

- You can surround text objects with `ys` (yank surround), change surrounding with `cs`, and delete surrounding with `ds`.
- You can comment and uncomment selections with `gc` in visual mode and `gcc` in normal mode.
- The project panel supports many shortcuts modeled after the Vim plugin `netrw`: navigation with `hjkl`, open file with `o`, open file in a new tab with `t`, etc.
- You can add key bindings to your keymap to navigate "camelCase" names. [Head down to the Optional key bindings](#optional-key-bindings) section to learn how.
- You can use `gR` to do [ReplaceWithRegister](https://github.com/vim-scripts/ReplaceWithRegister).
- You can use `cx` for [vim-exchange](https://github.com/tommcdo/vim-exchange) functionality. Note that it does not have a default binding in visual mode, but you can add one to your keymap (refer to the [optional key bindings](#optional-key-bindings) section).
- You can navigate to indent depths relative to your cursor with the [indent wise](https://github.com/jeetsukumaran/vim-indentwise) plugin `[-`, `]-`, `[+`, `]+`, `[=`, `]=`.
- You can select quoted text with AnyQuotes and bracketed text with AnyBrackets text objects. Zed also provides MiniQuotes and MiniBrackets which offer alternative selection behavior based on the [mini.ai](https://github.com/echasnovski/mini.nvim/blob/main/readmes/mini-ai.md) Neovim plugin. See the [Quote and Bracket text objects](#quote-and-bracket-text-objects) section below for details.
- You can configure AnyQuotes, AnyBrackets, MiniQuotes, and MiniBrackets text objects for selecting quoted and bracketed text using different selection strategies. See the [Any Bracket Functionality](#any-bracket-functionality) section below for details.

### Any Bracket Functionality

Zed offers two different strategies for selecting text surrounded by any quote, or any bracket. These text objects are **not enabled by default** and must be configured in your keymap to be used.

#### Included Characters

Each text object type works with specific characters:

| Text Object              | Characters                                                                             |
| ------------------------ | -------------------------------------------------------------------------------------- |
| AnyQuotes/MiniQuotes     | Single quote (`'`), Double quote (`"`), Backtick (`` ` ``)                             |
| AnyBrackets/MiniBrackets | Parentheses (`()`), Square brackets (`[]`), Curly braces (`{}`), Angle brackets (`<>`) |

Both "Any" and "Mini" variants work with the same character sets, but differ in their selection strategy.

#### AnyQuotes and AnyBrackets (Traditional Vim behavior)

These text objects implement traditional Vim behavior:

- **Selection priority**: Finds the innermost (closest) quotes or brackets first
- **Fallback mechanism**: If none are found, falls back to the current line
- **Character-based matching**: Focuses solely on open and close characters without considering syntax
- **Vanilla Vim similarity**: AnyBrackets matches the behavior of commands like `ci<`, `ci(`, etc., in vanilla Vim, including potential edge cases (like considering `>` in `=>` as a closing delimiter)

#### MiniQuotes and MiniBrackets (mini.ai behavior)

These text objects implement the behavior of the [mini.ai](https://github.com/echasnovski/mini.nvim/blob/main/readmes/mini-ai.md) Neovim plugin:

- **Selection priority**: Searches the current line first before expanding outward
- **Tree-sitter integration**: Uses Tree-sitter queries for more context-aware selections
- **Syntax-aware matching**: Can distinguish between actual brackets and similar characters in other contexts (like `>` in `=>`)

#### Choosing Between Approaches

- Use **AnyQuotes/AnyBrackets** if you:
  - Prefer traditional Vim behavior
  - Want consistent character-based selection prioritizing innermost delimiters
  - Need behavior that closely matches vanilla Vim's text objects

- Use **MiniQuotes/MiniBrackets** if you:
  - Prefer the mini.ai plugin behavior
  - Want more context-aware selections using Tree-sitter
  - Prefer current-line priority when searching

#### Example Configuration

To use these text objects, you need to add bindings to your keymap. Here's an example configuration that makes them available when using text object operators (`i` and `a`) or change-surrounds (`cs`):

```json [settings]
{
  "context": "vim_operator == a || vim_operator == i || vim_operator == cs",
  "bindings": {
    // Traditional Vim behavior
    "q": "vim::AnyQuotes",
    "b": "vim::AnyBrackets",

    // mini.ai plugin behavior
    "Q": "vim::MiniQuotes",
    "B": "vim::MiniBrackets"
  }
}
```

With this configuration, you can use commands like:

- `cib` - Change inside brackets using AnyBrackets behavior
- `ciB` - Change inside brackets using MiniBrackets behavior
- `ciq` - Change inside quotes using AnyQuotes behavior
- `ciQ` - Change inside quotes using MiniQuotes behavior

## Command palette

Vim mode allows you to open Zed's command palette with `:`. You can then type to access any usual Zed command. Additionally, vim mode adds aliases for popular Vim commands to ensure your muscle memory transfers to Zed. For example, you can write `:w` or `:write` to save the file.

Below, you'll find tables listing the commands you can use in the command palette. We put optional characters in square brackets to indicate that you can omit them.

> **Note**: We don't emulate the full power of Vim's command line yet. In particular, commands currently do not support arguments. Please [file issues on GitHub](https://github.com/zed-industries/zed) as you find things that are missing from the command palette.

### File and window management

This table shows commands for managing windows, tabs, and panes. As commands don't support arguments currently, you cannot specify a filename when saving or creating a new file.

| Command        | Description                                          |
| -------------- | ---------------------------------------------------- |
| `:w[rite][!]`  | Save the current file                                |
| `:wq[!]`       | Save the file and close the buffer                   |
| `:q[uit][!]`   | Close the buffer                                     |
| `:wa[ll][!]`   | Save all open files                                  |
| `:wqa[ll][!]`  | Save all open files and close all buffers            |
| `:qa[ll][!]`   | Close all buffers                                    |
| `:[e]x[it][!]` | Close the buffer                                     |
| `:up[date]`    | Save the current file                                |
| `:cq`          | Quit completely (close all running instances of Zed) |
| `:vs[plit]`    | Split the pane vertically                            |
| `:sp[lit]`     | Split the pane horizontally                          |
| `:new`         | Create a new file in a horizontal split              |
| `:vne[w]`      | Create a new file in a vertical split                |
| `:tabedit`     | Create a new file in a new tab                       |
| `:tabnew`      | Create a new file in a new tab                       |
| `:tabn[ext]`   | Go to the next tab                                   |
| `:tabp[rev]`   | Go to previous tab                                   |
| `:tabc[lose]`  | Close the current tab                                |
| `:ls`          | Show all buffers                                     |

> **Note:** The `!` character is used to force the command to execute without saving changes or prompting before overwriting a file.

### Ex commands

These ex commands open Zed's various panels and windows.

| Command                      | Default Shortcut |
| ---------------------------- | ---------------- |
| Open the project panel       | `:E[xplore]`     |
| Open the collaboration panel | `:C[ollab]`      |
| Open the chat panel          | `:Ch[at]`        |
| Open the AI panel            | `:A[I]`          |
| Open the git panel           | `:G[it]`         |
| Open the debug panel         | `:D[ebug]`       |
| Open the notifications panel | `:No[tif]`       |
| Open the feedback window     | `:fe[edback]`    |
| Open the diagnostics window  | `:cl[ist]`       |
| Open the terminal            | `:te[rm]`        |
| Open the extensions window   | `:Ext[ensions]`  |

### Navigating diagnostics

These commands navigate diagnostics.

| Command                  | Description                    |
| ------------------------ | ------------------------------ |
| `:cn[ext]` or `:ln[ext]` | Go to the next diagnostic      |
| `:cp[rev]` or `:lp[rev]` | Go to the previous diagnostics |
| `:cc` or `:ll`           | Open the errors page           |

### Git

These commands interact with the version control system git.

| Command         | Description                                             |
| --------------- | ------------------------------------------------------- |
| `:dif[fupdate]` | View the diff under the cursor (`d o` in normal mode)   |
| `:rev[ert]`     | Revert the diff under the cursor (`d p` in normal mode) |

### Jump

These commands jump to specific positions in the file.

| Command             | Description                         |
| ------------------- | ----------------------------------- |
| `:<number>`         | Jump to a line number               |
| `:$`                | Jump to the end of the file         |
| `:/foo` and `:?foo` | Jump to next/prev line matching foo |

### Replacement

This command replaces text. It emulates the substitute command in vim. The substitute command uses regular expressions, and Zed uses a slightly different syntax than vim. You can learn more about Zed's syntax below, [in the regex differences section](#regex-differences). Zed will replace only the first occurrence of the search pattern in the current line. To replace all matches append the `g` flag.

| Command                 | Description                       |
| ----------------------- | --------------------------------- |
| `:[range]s/foo/bar/[g]` | Replace instances of foo with bar |

### Editing

These commands help you edit text.

| Command           | Description                                             |
| ----------------- | ------------------------------------------------------- |
| `:j[oin]`         | Join the current line                                   |
| `:d[elete][l][p]` | Delete the current line                                 |
| `:s[ort] [i]`     | Sort the current selection (with i, case-insensitively) |
| `:y[ank]`         | Yank (copy) the current selection or line               |

### Set

These commands modify editor options locally for the current buffer.

| Command                         | Description                                                                                   |
| ------------------------------- | --------------------------------------------------------------------------------------------- |
| `:se[t] [no]wrap`               | Lines longer than the width of the window will wrap and displaying continues on the next line |
| `:se[t] [no]nu[mber]`           | Print the line number in front of each line                                                   |
| `:se[t] [no]r[elative]nu[mber]` | Changes the displayed number to be relative to the cursor                                     |
| `:se[t] [no]i[gnore]c[ase]`     | Controls whether the buffer and project search use case-sensitive matching                    |

### Command mnemonics

As any Zed command is available, you may find that it's helpful to remember mnemonics that run the correct command. For example:

- `:diffs` for "toggle all hunk diffs"
- `:cpp` for "copy path to file"
- `:crp` for "copy relative path"
- `:reveal` for "reveal in finder"
- `:zlog` for "open zed log"
- `:clank` for "cancel language server work"

## Customizing key bindings

In this section, we'll learn how to customize the key bindings of Zed's vim mode. You'll learn:

- How to select the correct context for your new key bindings.
- Useful contexts for vim mode key bindings.
- Common key bindings to customize for extra productivity.

### Selecting the correct context

Zed's key bindings are evaluated only when the `"context"` property matches your location in the editor. For example, if you add key bindings to the `"Editor"` context, they will only work when you're editing a file. If you add key bindings to the `"Workspace"` context, they will work everywhere in Zed. Here's an example of a key binding that saves when you're editing a file:

```json [settings]
{
  "context": "Editor",
  "bindings": {
    "ctrl-s": "file::Save"
  }
}
```

Contexts are nested, so when you're editing a file, the context is the `"Editor"` context, which is inside the `"Pane"` context, which is inside the `"Workspace"` context. That's why any key bindings you add to the `"Workspace"` context will work when you're editing a file. Here's an example:

```json [keymap]
// This key binding will work when you're editing a file. It comes built into Zed by default as the workspace: save command.
{
  "context": "Workspace",
  "bindings": {
    "ctrl-s": "workspace::Save"
  }
}
```

Contexts are expressions. They support boolean operators like `&&` (and) and `||` (or). For example, you can use the context `"Editor && vim_mode == normal"` to create key bindings that only work when you're editing a file _and_ you're in vim's normal mode.

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

Here's a template with useful vim mode contexts to help you customize your vim mode key bindings. You can copy it and integrate it into your user keymap.

```json [keymap]
[
  {
    "context": "VimControl && !menu",
    "bindings": {
      // Put key bindings here if you want them to work in normal & visual mode.
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
      // Put key bindings here (in addition to the context above) if you want them to
      // work when no editor exists.
      // "space f": "file_finder::Toggle"
    }
  }
]
```

> **Note**: If you would like to emulate Vim's `map` commands (`nmap`, etc.), you can use the action `workspace::SendKeystrokes` in the correct context.

### Optional key bindings

By default, you can navigate between the different files open in the editor with shortcuts like `ctrl+w` followed by one of `hjkl` to move to the left, down, up, or right, respectively.

But you cannot use the same shortcuts to move between all the editor docks (the terminal, project panel, assistant panel, ...). If you want to use the same shortcuts to navigate to the docks, you can add the following key bindings to your user keymap.

```json [settings]
{
  "context": "Dock",
  "bindings": {
    "ctrl-w h": "workspace::ActivatePaneLeft",
    "ctrl-w l": "workspace::ActivatePaneRight",
    "ctrl-w k": "workspace::ActivatePaneUp",
    "ctrl-w j": "workspace::ActivatePaneDown"
    // ... or other keybindings
  }
}
```

Subword motion, which allows you to navigate and select individual words in `camelCase` or `snake_case`, is not enabled by default. To enable it, add these bindings to your keymap.

```json [settings]
{
  "context": "VimControl && !menu && vim_mode != operator",
  "bindings": {
    "w": "vim::NextSubwordStart",
    "b": "vim::PreviousSubwordStart",
    "e": "vim::NextSubwordEnd",
    "g e": "vim::PreviousSubwordEnd"
  }
}
```

> Note: Operations like `dw` remain unaffected. If you would like operations to
> also use subword motion, remove `vim_mode != operator` from the `context`.

Vim mode comes with shortcuts to surround the selection in normal mode (`ys`), but it doesn't have a shortcut to add surrounds in visual mode. By default, `shift-s` substitutes the selection (erases the text and enters insert mode). To use `shift-s` to add surrounds in visual mode, you can add the following object to your keymap.

```json [settings]
{
  "context": "vim_mode == visual",
  "bindings": {
    "shift-s": "vim::PushAddSurrounds"
  }
}
```

In non-modal text editors, cursor navigation typically wraps when moving past line ends. Zed, however, handles this behavior exactly like Vim by default: the cursor stops at line boundaries. If you prefer your cursor to wrap between lines, override these keybindings:

```json [settings]
// In VimScript, this would look like this:
// set whichwrap+=<,>,[,],h,l
{
  "context": "VimControl && !menu",
  "bindings": {
    "left": "vim::WrappingLeft",
    "right": "vim::WrappingRight",
    "h": "vim::WrappingLeft",
    "l": "vim::WrappingRight"
  }
}
```

The [Sneak motion](https://github.com/justinmk/vim-sneak) feature allows for quick navigation to any two-character sequence in your text. You can enable it by adding the following keybindings to your keymap. By default, the `s` key is mapped to `vim::Substitute`. Adding these bindings will override that behavior, so ensure this change aligns with your workflow preferences.

```json [settings]
{
  "context": "vim_mode == normal || vim_mode == visual",
  "bindings": {
    "s": "vim::PushSneak",
    "shift-s": "vim::PushSneakBackward"
  }
}
```

The [vim-exchange](https://github.com/tommcdo/vim-exchange) feature does not have a default binding for visual mode, as the `shift-x` binding conflicts with the default `shift-x` binding for visual mode (`vim::VisualDeleteLine`). To assign the default vim-exchange binding, add the following keybinding to your keymap:

```json [settings]
{
  "context": "vim_mode == visual",
  "bindings": {
    "shift-x": "vim::Exchange"
  }
}
```

### Restoring common text editing and Zed keybindings

If you're using vim mode on Linux or Windows, you may find it overrides keybindings you can't live without: `ctrl+v` to paste, `ctrl+f` to search, etc. You can restore them by copying this data into your keymap:

```json [keymap]
{
  "context": "Editor && !menu",
  "bindings": {
    "ctrl-c": "editor::Copy",               // vim default: return to normal mode
    "ctrl-x": "editor::Cut",                // vim default: decrement
    "ctrl-v": "editor::Paste",              // vim default: visual block mode
    "ctrl-y": "editor::Undo",               // vim default: line up
    "ctrl-f": "buffer_search::Deploy",      // vim default: page down
    "ctrl-o": "workspace::Open",            // vim default: go back
    "ctrl-s": "workspace::Save",            // vim default: show signature
    "ctrl-a": "editor::SelectAll",          // vim default: increment
    "ctrl-b": "workspace::ToggleLeftDock"   // vim default: down
  }
},
```

## Changing vim mode settings

You can change the following settings to modify vim mode's behavior:

| Property                     | Description                                                                                                                                                                                   | Default Value |
| ---------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------- |
| default_mode                 | The default mode to start in. One of "normal", "insert", "replace", "visual", "visual_line", "visual_block", "helix_normal".                                                                  | "normal"      |
| use_system_clipboard         | Determines how system clipboard is used:<br><ul><li>"always": use for all operations</li><li>"never": only use when explicitly specified</li><li>"on_yank": use for yank operations</li></ul> | "always"      |
| use_multiline_find           | deprecated                                                                                                                                                                                    |
| use_smartcase_find           | If `true`, `f` and `t` motions are case-insensitive when the target letter is lowercase.                                                                                                      | false         |
| toggle_relative_line_numbers | If `true`, line numbers are relative in normal mode and absolute in insert mode, giving you the best of both options.                                                                         | false         |
| custom_digraphs              | An object that allows you to add custom digraphs. Read below for an example.                                                                                                                  | {}            |
| highlight_on_yank_duration   | The duration of the highlight animation(in ms). Set to `0` to disable                                                                                                                         | 200           |

Here's an example of adding a digraph for the zombie emoji. This allows you to type `ctrl-k f z` to insert a zombie emoji. You can add as many digraphs as you like.

```json [settings]
{
  "vim": {
    "custom_digraphs": {
      "fz": "🧟‍♀️"
    }
  }
}
```

Here's an example of these settings changed:

```json [settings]
{
  "vim": {
    "default_mode": "insert",
    "use_system_clipboard": "never",
    "use_smartcase_find": true,
    "relative_line_numbers": "enabled",
    "highlight_on_yank_duration": 50,
    "custom_digraphs": {
      "fz": "🧟‍♀️"
    }
  }
}
```

## Useful core Zed settings for vim mode

Here are a few general Zed settings that can help you fine-tune your Vim experience:

| Property                | Description                                                                                                                                                   | Default Value        |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------- |
| cursor_blink            | If `true`, the cursor blinks.                                                                                                                                 | `true`               |
| relative_line_numbers   | If `"enabled"`, line numbers in the left gutter are relative to the cursor. If `"wrapped"`, they also display for wrapped lines.                              | `"disabled"`         |
| scrollbar               | Object that controls the scrollbar display. Set to `{ "show": "never" }` to hide the scroll bar.                                                              | `{ "show": "auto" }` |
| scroll_beyond_last_line | If set to `"one_page"`, allows scrolling up to one page beyond the last line. Set to `"off"` to prevent this behavior.                                        | `"one_page"`         |
| vertical_scroll_margin  | The number of lines to keep above or below the cursor when scrolling. Set to `0` to allow the cursor to go up to the edges of the screen vertically.          | `3`                  |
| gutter.line_numbers     | Controls the display of line numbers in the gutter. Set the `"line_numbers"` property to `false` to hide line numbers.                                        | `true`               |
| command_aliases         | Object that defines aliases for commands in the command palette. You can use it to define shortcut names for commands you use often. Read below for examples. | `{}`                 |

Here's an example of these settings changed:

```json [settings]
{
  // Disable cursor blink
  "cursor_blink": false,
  // Use relative line numbers
  "relative_line_numbers": "enabled",
  // Hide the scroll bar
  "scrollbar": { "show": "never" },
  // Prevent the buffer from scrolling beyond the last line
  "scroll_beyond_last_line": "off",
  // Allow the cursor to reach the edges of the screen
  "vertical_scroll_margin": 0,
  "gutter": {
    // Disable line numbers completely
    "line_numbers": false
  },
  "command_aliases": {
    "W": "w",
    "Wq": "wq",
    "Q": "q"
  }
}
```

The `command_aliases` property is a single object that maps keys or key sequences to vim mode commands. The example above defines multiple aliases: `W` for `w`, `Wq` for `wq`, and `Q` for `q`.

## Regex differences

Zed uses a different regular expression engine from Vim. This means that you will have to use a different syntax in some cases. Here are the most common differences:

- **Capture groups**: Vim uses `\(` and `\)` to represent capture groups, in Zed these are `(` and `)`. On the flip side, in Vim, `(` and `)` represent literal parentheses, but in Zed these must be escaped to `\(` and `\)`.
- **Matches**: When replacing, Vim uses the backslash character followed by a number to represent a matched capture group. For example, `\1`. Zed uses the dollar sign instead. So, when in Vim you use `\0` to represent the entire match, in Zed the syntax is `$0` instead. Same for numbered capture groups: `\1` in Vim is `$1` in Zed.
- **Global option**: By default, in Vim, regex searches only match the first occurrence on a line, and you append `/g` at the end of your query to find all matches. In Zed, regex searches are global by default.
- **Case sensitivity**: Vim uses `/i` to indicate a case-insensitive search. In Zed you can either write `(?i)` at the start of the pattern or toggle case-sensitivity with the shortcut {#kb search::ToggleCaseSensitive}.

> **Note**: To help with the transition, the command palette will fix parentheses and replace groups for you when you write a Vim-style substitute command, `:%s//`. So, Zed will convert `%s:/\(a\)(b)/\1/` into a search for "(a)\(b\)" and a replacement of "$1".

For the full syntax supported by Zed's regex engine [see the regex crate documentation](https://docs.rs/regex/latest/regex/#syntax).
