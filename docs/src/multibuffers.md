# Multibuffers

One of the superpowers Zed gives you is the ability to edit multiple files simultaneously. When combined with multiple cursors, this makes wide-ranging refactors significantly faster.

## Editing in a multibuffer

Editing a multibuffer is the same as editing a normal file. Changes you make will be reflected in the open copies of that file in the rest of the editor, and you can save all files with `editor: Save` (bound to `cmd-s` on macOS, `ctrl-s` on Windows/Linux, or `:w` in Vim mode).

When in a multibuffer, it is often useful to use multiple cursors to edit every file simultaneously. If you want to edit a few instances, you can select them with the mouse (`option-click` on macOS, `alt-click` on Window/Linux) or the keyboard. `cmd-d` on macOS, `ctrl-d` on Windows/Linux, or `gl` in Vim mode will select the next match of the word under the cursor.

When you want to edit all matches you can select them by running the `editor: Select All Matches` command (`cmd-shift-l` on macOS, `ctrl-shift-l` on Windows/Linux, or `g a` in Vim mode).

## Project search

To start a search run the `pane: Toggle Search` command (`cmd-shift-f` on macOS, `ctrl-shift-f` on Windows/Linux, or `g/` in Vim mode). After the search has completed, the results will be shown in a new multibuffer. There will be one excerpt for each matching line across the whole project.

## Diagnostics

If you have a language server installed, the diagnostics pane can show you all errors across your project. You can open it by clicking on the icon in the status bar, or running the `diagnostcs: Deploy` command` ('cmd-shift-m` on macOS, `ctrl-shift-m` on Windows/Linux, or `:clist` in Vim mode).

## Find References

If you have a language server installed, you can find all references to the symbol under the cursor with the `editor: Find References` command (`cmd-click` on macOS, `ctrl-click` on Windows/Linux, or `g A` in Vim mode.

Depending on your language server, commands like `editor: Go To Definition` and `editor: Go To Type Definition` will also open a multibuffer if there are multiple possible definitions.
