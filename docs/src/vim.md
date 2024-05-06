# Vim Mode

Zed includes a vim emulation layer known as “vim mode”. This document aims to describe how it works, and how to make the most out of it.

## Philosophy

Vim mode in Zed is supposed to primarily "do what you expect": it mostly tries to copy vim exactly, but will use Zed-specific functionality when available to make things smoother.

This means Zed will never be 100% vim compatible, but should be 100% vim familiar! We expect that our vim mode already copes with 90% of your workflow, and we'd like to keep improving it. If you find things that you can’t yet do in vim mode, but which you rely on in your current workflow, please leave feedback in the editor itself (`:feedback`), or [file an issue](https://github.com/zed-industries/zed/issues).

## Zed-specific features

Zed is built on a modern foundation that (among other things) uses tree-sitter to understand the content of the file you're editing, and supports multiple cursors out of the box.

Vim mode has several "core Zed" key bindings, that will help you make the most of Zed's specific feature set.

```
# Normal mode
g d   Go to definition
g D   Go to type definition
c d   Rename (change definition)
g A   Go to All references to the current word

g <space>  Open the current search excerpt in its own tab

g s   Find symbol in current file
g S   Find symbol in entire project

g n   Add a visual selection for the next copy of the current word
g N   The same, but backwards
g >   Skip latest word selection, and add next.
g <   The same, but backwards
g a   Add a visual selection for every copy of the current word

g h   Show inline error (hover)

# Insert mode
ctrl-x ctrl-o  Open the completion menu
ctrl-x ctrl-c  Request GitHub Copilot suggestion (if configured)
ctrl-x ctrl-a  Open the inline AI assistant (if configured)
ctrl-x ctrl-l  Open the LSP code actions
ctrl-x ctrl-z  Hides all suggestions
```

Vim mode uses Zed to define concepts like "brackets" (for the `%` key) and "words" (for motions like `w` and `e`). This does lead to some differences, but they are mostly positive. For example `%` considers `|` to be a bracket in languages like Rust; and `w` considers `$` to be a word-character in languages like Javascript.

Vim mode emulates visual block mode using Zed's multiple cursor support. This again leads to some differences, but is much more powerful.

Finally, Vim mode's search and replace functionality is backed by Zed's. This means that the pattern syntax is slightly different, see the section on [Regex differences](#regex-differences) for details.

## Custom key bindings

Zed does not yet have an equivalent to vim’s `map` command to convert one set of keystrokes into another, however you can bind any sequence of keys to fire any Action documented in the [Key bindings documentation](https://zed.dev/docs/key-bindings).

You can edit your personal key bindings with `:keymap`.
For vim-specific shortcuts, you may find the following template a good place to start:

```json
[
  {
    "context": "Editor && VimControl && !VimWaiting && !menu",
    "bindings": {
      // put key-bindings here if you want them to work in normal & visual mode
    }
  },
  {
    "context": "Editor && vim_mode == normal && !VimWaiting && !menu",
    "bindings": {
      // put key-bindings here if you want them to work only in normal mode
    }
  },
  {
    "context": "Editor && vim_mode == visual && !VimWaiting && !menu",
    "bindings": {
      // visual, visual line & visual block modes
    }
  },
  {
    "context": "Editor && vim_mode == insert && !menu",
    "bindings": {
      // put key-bindings here if you want them to work in insert mode
    }
  }
]
```

You can see the bindings that are enabled by default in vim mode [here](https://zed.dev/ref/vim.json).

The details of the context are a little out of scope for this doc, but suffice to say that `menu` is true when a menu is open (e.g. the completions menu), `VimWaiting` is true after you type `f` or `t` when we’re waiting for a new key (and you probably don’t want bindings to happen). Please reach out on [GitHub](https://github.com/zed-industries/zed) if you want help making a key bindings work.

### Examples

Binding `jk` to exit insert mode and go to normal mode:

```
{
  "context": "Editor && vim_mode == insert && !menu",
  "bindings": {
    "j k": ["vim::SwitchMode", "Normal"]
  }
}
```

## Subword motion

Subword motion is not enabled by default. To enable it, add these bindings to your keymap.

```json
  {
    "context": "Editor && VimControl && !VimWaiting && !menu",
    "bindings": {
      "w": "vim::NextSubwordStart",
      "b": "vim::PreviousSubwordStart",
      "e": "vim::NextSubwordEnd",
      "g e": "vim::PreviousSubwordEnd"
    }
  },
```

## Command palette

Vim mode allows you to enable Zed’s command palette with `:`. This means that you can use vim's command palette to run any action that Zed supports.

Additionally vim mode contains a number of aliases for popular vim commands to ensure that muscle memory works. For example `:w<enter>` will save the file.

We do not (yet) emulate the full power of vim’s command line, in particular we special case specific patterns instead of using vim's range selection syntax, and we do not support arguments to commands yet. Please reach out on [GitHub](https://github.com/zed-industries/zed) as you find things that are missing from the command palette.

As mentioned above, one thing to be aware of is that the regex engine is slightly different from vim's in `:%s/a/b`.

Currently supported vim-specific commands (as of Zed 0.106):

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

# replacement
:%s/foo/bar/
    to replace instances of foo with bar (/g is always assumed, the range must always be %, and Zed uses different regex syntax to vim)

# editing
:j[oin]
    to join the current line (no range is yet supported)
:d[elete][l][p]
    to delete the current line (no range is yet supported)
:s[ort] [i]
    to sort the current selection (with i, case-insensitively)
```

## Vim settings

Some vim settings are available to modify the default vim behavior:

```json
{
  "vim": {
    // "always": use system clipboard
    // "never": don't use system clipboard
    // "on_yank": use system clipboard for yank operations
    "use_system_clipboard": "always",
    // Enable multi-line find for `f` and `t` motions
    "use_multiline_find": false,
    // Enable smartcase find for `f` and `t` motions
    "use_smartcase_find": false
  }
}
```

## Related settings

There are a few Zed settings that you may also enjoy if you use vim mode:

```json
{
  // disable cursor blink
  "cursor_blink": false,
  // use relative line numbers
  "relative_line_numbers": true,
  // hide the scroll bar
  "scrollbar": { "show": "never" }
}
```

## Regex differences

Zed uses a different regular expression engine from Vim. This means that you will have to use a different syntax for some things.

Notably:

- Vim uses `\(` and `\)` to represent capture groups, in Zed these are `(` and `)`.
- On the flip side, `(` and `)` represent literal parentheses, but in Zed these must be escaped to `\(` and `\)`.
- When replacing, Vim uses `\0` to represent the entire match, in Zed this is `$0`, same for numbered capture groups `\1` -> `$1`.
- Vim uses `\<` and `\>` to represent word boundaries, in Zed these are both handled by `\b`
- Vim uses `/g` to indicate "all matches on one line", in Zed this is implied
- Vim uses `/i` to indicate "case-insensitive", in Zed you can either use `(?i)` at the start of the pattern or toggle case-sensitivity with `cmd-option-c`.

To help with the transition, the command palette will fix parentheses and replace groups for you when you run `:%s//`. So `%s:/\(a\)(b)/\1/` will be converted into a search for "(a)\(b\)" and a replacement of "$1".

For the full syntax supported by Zed's regex engine see the [regex crate documentation](https://docs.rs/regex/latest/regex/#syntax).
