---
title: Helix Mode - Zed
description: Helix-style keybindings and modal editing in Zed. Selection-first editing built on top of Vim mode.
---

# Helix Mode

_Work in progress. Not all Helix keybindings are implemented yet._

Zed's Helix mode is an emulation layer that brings Helix-style keybindings and modal editing to Zed. It builds upon Zed's [Vim mode](./vim.md), so much of the core functionality is shared. Enabling `helix_mode` will also enable `vim_mode`.

For a guide on Vim-related features that are also available in Helix mode, please refer to our [Vim mode documentation](./vim.md).

To check the current status of Helix mode, or to request a missing Helix feature, see the ["Are we Helix yet?" discussion](https://github.com/zed-industries/zed/discussions/33580).

For a detailed list of Helix's default keybindings, please visit the [official Helix documentation](https://docs.helix-editor.com/keymap.html).

## Core differences

Any text object that works with `m i` or `m a` also works with `]` and `[`, so for example `] (` selects the next pair of parentheses after the cursor.

In Helix mode, some text object keys follow Helix's meanings instead of Vim's: `m i m` and `m a m` select the closest surrounding pair of any kind (brackets, quotes, backticks, or vertical bars), `t` is the type/class text object, `c` is the comment text object, and `x` is the (X)HTML element text object.

Like Helix, the closest surrounding pair (`m i m`, `m a m`, `m d m`, `m r m`) is matched using the language's syntax tree, so delimiters inside string literals or comments don't count as pairs. In buffers without a language, no pair is matched.
