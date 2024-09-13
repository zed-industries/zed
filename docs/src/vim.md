# Vim Mode

Zed includes a Vim emulation layer known as "vim mode". On this page, you will learn how to turn Zed's vim mode on or off, what tools and commands Zed provides to help you navigate and edit your code, and generally how to make the most of vim mode in Zed.

> **Note**: This page focuses on exploring the features offered by vim mode in Zed. If you're looking to configure vim mode instead, head to the page [Configuring Vim Mode](vim-configuration.md).

## Zed's vim mode design

Vim mode tries to offer a familiar experience to Vim users: it replicates the behavior of motions and commands precisely when it makes sense and uses Zed-specific functionality to provide an editing experience that "just works" without requiring configuration on your part.

This includes support for semantic navigation, multiple cursors, or other features usually provided by plugins like surrounding text.

So, Zed's vim mode does not replicate Vim one-to-one, but it meshes Vim's modal design with Zed's modern features to provide a more fluid experience. It's also [configurable](vim-configuration.md), so you can add your own key bindings or override the defaults.

### Core differences

There are four types of features in vim mode that use Zed's core functionality, leading to some differences in behavior:

1. **Motions**: vim mode uses Zed's semantic parsing to tune the behavior of motions per language. For example, in Rust, jumping to matching bracket with `%` works with the pipe character `|`. In Javascript, `w` considers `$` to be a word character.
2. **Visual block selections**: vim mode uses Zed's multiple cursor to emulate visual block selections, making block selections a lot more flexible. For example, anything you insert after a block selection updates on every line in real-time, and you can adding or remove cursors anytime.
3. **Macros**: vim mode uses Zed's recording system for vim macros. So, you can capture and replay more complex actions, like autocompletion.
4. **Search and replace**: vim mode uses Zed's search system, so, the syntax for regular expressions is slightly different compared to Vim. Head to [Regex differences](#regex-differences) for details.

> **Note:** The foundations of Zed's vim mode should already cover many use cases, and we're always looking to improve it. If you find missing features that you rely on in your workflow, please [file an issue](https://github.com/zed-industries/zed/issues).

## Enabling and disabling vim mode

When you first open Zed, a checkbox will appear on the welcome screen, allowing you to enable vim mode. If you missed this, you can toggle vim mode on or off anytime by opening the command palette and using the workspace command `toggle vim mode`.

## Zed-specific features

Zed is built on a modern foundation that (among other things) uses tree-sitter and language servers to understand the content of the file you're editing and supports multiple cursors out of the box.

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

| Command                   | Default Shortcut |
| ------------------------- | ---------------- |
| Go to next git change     | `] c`            |
| Go to previous git change | `[ c`            |

### Treesitter

Treesitter is a powerful tool that Zed uses to understand the structure of your code. These commands help you navigate your code semantically.

| Command                      | Default Shortcut |
| ---------------------------- | ---------------- |
| Select a smaller syntax node | `] x`            |
| Select a larger syntax node  | `[ x`            |

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

### Ex commands

These ex commands open Zed's various panels and windows.

| Command                      | Default Shortcut |
| ---------------------------- | ---------------- |
| Open the project panel       | `:E[xplore]`     |
| Open the collaboration panel | `:C[ollab]`      |
| Open the chat panel          | `:Ch[at]`        |
| Open the AI panel            | `:A[I]`          |
| Open the notifications panel | `:No[tif]`       |
| Open the feedback window     | `:fe[edback]`    |
| Open the diagnostics window  | `:cl[ist]`       |
| Open the terminal            | `:te[rm]`        |
| Open the extensions window   | `:Ext[ensions]`  |

## Command palette

Vim mode allows you to enable Zed’s command palette with `:`. This means that you can use vim's command palette to run any action that Zed supports.

Additionally vim mode contains a number of aliases for popular vim commands to ensure that muscle memory works. For example `:w<enter>` will save the file.

We do not (yet) emulate the full power of vim’s command line, in particular we we do not support arguments to commands yet. Please reach out on [GitHub](https://github.com/zed-industries/zed) as you find things that are missing from the command palette.

As mentioned above, one thing to be aware of is that the regex engine is slightly different from vim's in `:%s/a/b`.

Currently supported vim-specific commands:

```
# window management
:w[rite][!], :wq[!], :q[uit][!], :wa[ll][!], :wqa[ll][!], :qa[ll][!], :[e]x[it][!], :up[date]
    to save/close tab(s) and pane(s) (no filename is supported yet)
:cq
    to quit completely.
:vs[plit], :sp[lit]
    to split vertically/horizontally (no filename is supported yet)
:new, :vne[w]
    to create a new file in a new pane above or to the left
:tabedit, :tabnew
    to create a new file in a new tab.
:tabn[ext], :tabp[rev]
    to go to previous/next tabs
:tabc[lose]
    to close the current tab

# navigating diagnostics
:cn[ext], :cp[rev], :ln[ext], :lp[rev]
    to go to the next/prev diagnostics
:cc, :ll
    to open the errors page

# handling git diff
:dif[fupdate]
    to view the diff under the cursor ("d o" in normal mode)
:rev[ert]
    to revert the diff under the cursor ("d p" in normal mode)

# jump to position
:<number>
    to jump to a line number
:$
    to jump to the end of the file
:/foo and :?foo
    to jump to next/prev line matching foo

# replacement (/g is always assumed and Zed uses different regex syntax to vim)
:[range]s/foo/bar/
  to replace instances of foo with bar

# editing
:j[oin]
    to join the current line
:d[elete][l][p]
    to delete the current line
:s[ort] [i]
    to sort the current selection (with i, case-insensitively)
:y[ank]
```

As any Zed command is available, you may find that it's helpful to remember mnemonics that run the correct command. For example:

```
:diffs  Toggle all Hunk [Diffs]
:cpp    [C]o[p]y [P]ath to file
:crp    [C]opy [r]elative [P]ath
:reveal [Reveal] in finder
:zlog   Open [Z]ed Log
:clank  [C]ancel [lan]guage server work[k]
```

## Supported plugins

Zed's vim mode includes some features that are usually provided by very popular plugins in the Vim ecosystem:

- You can surround text objects with `ys` (yank surround), change surrounding with `cs`, and delete surrounding with `ds`.
- You can comment and uncomment selections with `gc` in visual mode and `gcc` in normal mode.
- The project panel supports many shortcuts modeled after the Vim plugin `netrw`: navigation with `hjkl`, open file with `o`, open file in a new tab with `t`, etc.
- You can add key bindings to your keymap to navigate "camelCase" names.

## Regex differences

Zed uses a different regular expression engine from Vim. This means that you will have to use a different syntax in some cases. Here are the most common differences:

- **Capture groups**: Vim uses `\(` and `\)` to represent capture groups, in Zed these are `(` and `)`. On the flip side, in Vim, `(` and `)` represent literal parentheses, but in Zed these must be escaped to `\(` and `\)`.
- **Matches**: When replacing, Vim uses the backslash character followed by a number to represent a matched capture group. For example, `\1`. Zed uses the dollar sign instead. So, when in Vim you use `\0` to represent the entire match, in Zed the syntax is `$0` instead. Same for numbered capture groups: `\1` in Vim is `$1` in Zed.
- **Global option**: By default, in Vim, regex searches only match the first occurrence on a line, and you append `/g` at the end of your query to find all matches. In Zed, regex searches are global by default.
- **Case sensitivity**: Vim uses `/i` to indicate a case-insensitive search. In Zed you can either write `(?i)` at the start of the pattern or toggle case-sensitivity with the shortcut {#kb search::ToggleCaseSensitive}.

> **Note**: To help with the transition, the command palette will fix parentheses and replace groups for you when you write a Vim-style substitute command, `:%s//`. So, Zed will convert `%s:/\(a\)(b)/\1/` into a search for "(a)\(b\)" and a replacement of "$1".

> **Note**: To see the entire regular expression syntax supported by Zed, see the [regex crate documentation](https://docs.rs/regex/latest/regex/#syntax).
