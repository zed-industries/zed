# Vim Mode

Zed includes a vim emulation layer known as "vim mode". This document aims to describe how it works, and how to make the most out of it.

## Philosophy

Vim mode in Zed is supposed to primarily "do what you expect": it mostly tries to copy vim exactly, but will use Zed-specific functionality when available to make things smoother.

This means Zed will never be 100% Vim compatible, but should be 100% Vim familiar! We expect that our Vim mode already copes with 90% of your workflow, and we'd like to keep improving it. If you find things that you can’t yet do in Vim mode, but which you rely on in your current workflow, please [file an issue](https://github.com/zed-industries/zed/issues).

## Zed-specific features

Zed is built on a modern foundation that (among other things) uses tree-sitter and language servers to understand the content of the file you're editing, and supports multiple cursors out of the box.

Vim mode has several "core Zed" key bindings, that will help you make the most of Zed's specific feature set.

```
# Language server
g d     Go to definition
g D     Go to declaration
g y     Go to type definition
g I     Go to implementation

c d     Rename (change definition)
g A     Go to All references to the current word

g s   Find symbol in current file
g S   Find symbol in entire project

g ]   Go to next diagnostic
g [   Go to previous diagnostic
] d   Go to next diagnostic
[ d   Go to previous diagnostic
g h   Show inline error (hover)
g .   Open the code actions menu

# Git
] c   Go to next git change
[ c   Go to previous git change

# Treesitter
] x   Select a smaller syntax node
[ x   Select a larger syntax node

# Multi cursor
g l   Add a visual selection for the next copy of the current word
g L   The same, but backwards
g >   Skip latest word selection, and add next.
g <   The same, but backwards
g a   Add a visual selection for every copy of the current word

# Pane management
g /        Open a project-wide search
g <space>  Open the current search excerpt
<ctrl-w> <space>  Open the current search excerpt in a split
<ctrl-w> g d      Go to definition in a split
<ctrl-w> g D      Go to type definition in a split

# Insert mode
ctrl-x ctrl-o  Open the completion menu
ctrl-x ctrl-c  Request GitHub Copilot suggestion (if configured)
ctrl-x ctrl-a  Open the inline AI assistant (if configured)
ctrl-x ctrl-l  Open the code actions menu
ctrl-x ctrl-z  Hides all suggestions

# Ex commands
:E[xplore]    Open the project panel
:C[ollab]     Open the collaboration panel
:Ch[at]       Open the chat panel
:A[I]         Open the AI panel
:No[tif]      Open the notifications panel
:fe[edback]   Open the feedback window
:cl[ist]      Open the diagnostics window
:te[rm]       Open the terminal
:Ext[ensions] Open the extensions window
```

Vim mode uses Zed to define concepts like "brackets" (for the `%` key) and "words" (for motions like `w` and `e`). This does lead to some differences, but they are mostly positive. For example `%` considers `|` to be a bracket in languages like Rust; and `w` considers `$` to be a word-character in languages like Javascript.

Vim mode emulates visual block mode using Zed's multiple cursor support. This again leads to some differences, but is much more powerful.

Vim's macro support (`q` and `@`) is implemented using Zed's actions. This lets us support recording and replaying of autocompleted code, etc. Unlike Vim, Zed does not re-use the yank registers for recording macros, they are two separate namespaces.

Finally, Vim mode's search and replace functionality is backed by Zed's. This means that the pattern syntax is slightly different, see the section on [Regex differences](#regex-differences) for details.

## Custom key bindings

You can edit your personal key bindings with `:keymap`.
For vim-specific shortcuts, you may find the following template a good place to start.

> **Note:** We made some breaking changes in Zed version `0.145.0`. For older versions, see [the previous version of this document](https://github.com/zed-industries/zed/blob/c67aeaa9c58619a58708722ac7d7a78c75c29336/docs/src/vim.md#L90).

```json
[
  {
    "context": "VimControl && !menu",
    "bindings": {
      // put key-bindings here if you want them to work in normal & visual mode
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

If you would like to emulate vim's `map` (`nmap` etc.) commands you can bind to the [`workspace::SendKeystrokes`](./key-bindings.md#remapping-keys) action in the correct context.

You can see the bindings that are enabled by default in vim mode [here](https://github.com/zed-industries/zed/blob/main/assets/keymaps/vim.json).

#### Contexts

Zed's keyboard bindings are evaluated only when the `"context"` matches the location you are in on the screen. Locations are nested, so when you're editing you're in the `"Workspace"` location is at the top, containing a `"Pane"` which contains an `"Editor"`. Contexts are matched only on one level at a time. So it is possible to combine `Editor && vim_mode == normal`, but `Workspace && vim_mode == normal` will never match because we set the vim context at the `Editor` level.

Vim mode adds several contexts to the `Editor`:

- `vim_mode` is similar to, but not identical to, the current mode. It starts as one of `normal`, `visual`, `insert` or `replace` (depending on your mode). If you are mid-way through typing a sequence, `vim_mode` will be either `waiting` if it's waiting for an arbitrary key (for example after typing `f` or `t`), or `operator` if it's waiting for another binding to trigger (for example after typing `c` or `d`).
- `vim_operator` is set to `none` unless `vim_mode == operator` in which case it is set to the current operator's default keybinding (for example after typing `d`, `vim_operator == d`).
- `"VimControl"` indicates that vim keybindings should work. It is currently an alias for `vim_mode == normal || vim_mode == visual || vim_mode == operator`, but the definition may change over time.

### Restoring some sense of normality

If you're using Vim mode on Linux or Windows, you may find that it has overridden keybindings
that you can't live without. You can restore them to their defaults by copying these into your keymap:

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

## Command palette

Vim mode allows you to enable Zed’s command palette with `:`. This means that you can use vim's command palette to run any action that Zed supports.

Additionally vim mode contains a number of aliases for popular vim commands to ensure that muscle memory works. For example `:w<enter>` will save the file.

We do not (yet) emulate the full power of vim’s command line, in particular we special case specific patterns instead of using vim's range selection syntax, and we do not support arguments to commands yet. Please reach out on [GitHub](https://github.com/zed-industries/zed) as you find things that are missing from the command palette.

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

# jump to position
:<number>
    to jump to a line number
:$
    to jump to the end of the file
:/foo and :?foo
    to jump to next/prev line matching foo

# replacement (/g is always assumed and Zed uses different regex syntax to vim)
:%s/foo/bar/
  to replace instances of foo with bar
:X,Ys/foo/bar/
    to limit replacement between line X and Y
    other ranges are not yet implemented

# editing
:j[oin]
    to join the current line (no range is yet supported)
:d[elete][l][p]
    to delete the current line (no range is yet supported)
:s[ort] [i]
    to sort the current selection (with i, case-insensitively)
```

As any Zed command is available, you may find that it's helpful to remember mnemonics that run the correct command. For example:

```
:diff   Toggle Hunk [Diff]
:diffs  Toggle all Hunk [Diffs]
:revert Revert Selected Hunks
:cpp    [C]o[p]y [P]ath to file
:crp    [C]opy [r]elative [P]ath
:reveal [Reveal] in finder
:zlog   Open [Z]ed Log
```

## Settings

Vim mode is not enabled by default. To enable Vim mode, you need to add the following configuration to your settings file:

```json
{
  "vim_mode": true
}
```

Alternatively, you can enable Vim mode by running the `toggle vim mode` command from the command palette.

Some vim settings are available to modify the default vim behavior:

```json
{
  "vim": {
    // "always": use system clipboard when no register is specified
    // "never": don't use system clipboard unless "+ or "* is specified
    // "on_yank": use system clipboard for yank operations when no register is specified
    "use_system_clipboard": "always",
    // Let `f` and `t` motions extend across multiple lines
    "use_multiline_find": true,
    // Let `f` and `t` motions match case insensitively if the target is lowercase
    "use_smartcase_find": true,
    // Use relative line numbers in normal mode, absolute in insert mode
    // c.f. https://github.com/jeffkreeftmeijer/vim-numbertoggle
    "toggle_relative_line_numbers": true,
    // Add custom digraphs (e.g. ctrl-k f z will insert a zombie emoji)
    "custom_digraphs": {
      "fz": "🧟‍♀️"
    }
  }
}
```

There are also a few Zed settings that you may also enjoy if you use vim mode:

```json
{
  // disable cursor blink
  "cursor_blink": false,
  // use relative line numbers
  "relative_line_numbers": true,
  // hide the scroll bar
  "scrollbar": { "show": "never" },
  // prevent the buffer from scrolling beyond the last line
  "scroll_beyond_last_line": "off",
  // allow cursor to reach edges of screen
  "vertical_scroll_margin": 0,
  "gutter": {
    // disable line numbers completely:
    "line_numbers": false
  },
  "command_aliases": {
    "W": "w",
    "Wq": "wq",
    "Q": "q"
  }
}
```

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

## Supported plugins

Zed has nascent support for some Vim plugins:

- From `vim-surround`, `ys`, `cs` and `ds` work. Though you cannot add new HTML tags yet.
- From `vim-commentary`, `gc` in visual mode and `gcc` in normal mode. Though you cannot operate on arbitrary objects yet.
- From `netrw`, most keybindings are supported in the project panel.
- From `vim-spider`/`CamelCaseMotion` you can use subword motions as described above.

## Regex differences

Zed uses a different regular expression engine from Vim. This means that you will have to use a different syntax for some things.

Notably:

- Vim uses `\(` and `\)` to represent capture groups, in Zed these are `(` and `)`.
- On the flip side, `(` and `)` represent literal parentheses, but in Zed these must be escaped to `\(` and `\)`.
- When replacing, Vim uses `\0` to represent the entire match, in Zed this is `$0`, same for numbered capture groups `\1` -> `$1`.
- Vim uses `/g` to indicate "all matches on one line", in Zed this is implied
- Vim uses `/i` to indicate "case-insensitive", in Zed you can either use `(?i)` at the start of the pattern or toggle case-sensitivity with `cmd-option-c`.

To help with the transition, the command palette will fix parentheses and replace groups for you when you run `:%s//`. So `%s:/\(a\)(b)/\1/` will be converted into a search for "(a)\(b\)" and a replacement of "$1".

For the full syntax supported by Zed's regex engine see the [regex crate documentation](https://docs.rs/regex/latest/regex/#syntax).
