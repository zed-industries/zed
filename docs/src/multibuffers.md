# Multibuffers

One of the superpowers Zed gives you is the ability to edit multiple files simultaneously. When combined with multiple cursors, this makes wide-ranging refactors significantly faster.

## Editing in a multibuffer

<div class="video" style="position: relative; padding-top: 71.71314741035857%;">
  <iframe
    src="https://customer-snccc0j9v3kfzkif.cloudflarestream.com/bda0a6584c19f4b39e58a263c0ae4358/iframe?muted=true&preload=true&loop=true&autoplay=true&poster=https%3A%2F%2Fcustomer-snccc0j9v3kfzkif.cloudflarestream.com%2Fbda0a6584c19f4b39e58a263c0ae4358%2Fthumbnails%2Fthumbnail.jpg%3Ftime%3D%26height%3D600&controls=false"
    style="border: none; position: absolute; top: 0; left: 0; height: 100%; width: 100%;"
    allow="accelerometer; gyroscope; autoplay; encrypted-media; picture-in-picture;"
    allowfullscreen="true"
  ></iframe>
</div>

Editing a multibuffer is the same as editing a normal file. Changes you make will be reflected in the open copies of that file in the rest of the editor, and you can save all files with `editor: Save` (bound to `cmd-s` on macOS, `ctrl-s` on Windows/Linux, or `:w` in Vim mode).

When in a multibuffer, it is often useful to use multiple cursors to edit every file simultaneously. If you want to edit a few instances, you can select them with the mouse (`option-click` on macOS, `alt-click` on Window/Linux) or the keyboard. `cmd-d` on macOS, `ctrl-d` on Windows/Linux, or `gl` in Vim mode will select the next match of the word under the cursor.

When you want to edit all matches you can select them by running the `editor: Select All Matches` command (`cmd-shift-l` on macOS, `ctrl-shift-l` on Windows/Linux, or `g a` in Vim mode).

## Navigating to the Source File

While you can easily edit files in a multibuffer, navigating directly to the source file is often beneficial. You can accomplish this by clicking on any of the divider lines between excerpts or by placing your cursor in an excerpt and executing the `editor: open excerpts` command. It’s key to note that if multiple cursors are being used, the command will open the source file positioned under each cursor within the multibuffer.

Additionally, if you prefer to use the mouse and would like to double-click on an excerpt to open it, you can enable this functionality with the setting: `"double_click_in_multibuffer": "open"`.

## Project search

To start a search run the `pane: Toggle Search` command (`cmd-shift-f` on macOS, `ctrl-shift-f` on Windows/Linux, or `g/` in Vim mode). After the search has completed, the results will be shown in a new multibuffer. There will be one excerpt for each matching line across the whole project.

## Diagnostics

If you have a language server installed, the diagnostics pane can show you all errors across your project. You can open it by clicking on the icon in the status bar, or running the `diagnostics: Deploy` command (`cmd-shift-m` on macOS, `ctrl-shift-m` on Windows/Linux, or `:clist` in Vim mode).

## Find References

If you have a language server installed, you can find all references to the symbol under the cursor with the `editor: Find References` command (`cmd-click` on macOS, `ctrl-click` on Windows/Linux, or `g A` in Vim mode.

Depending on your language server, commands like `editor: Go To Definition` and `editor: Go To Type Definition` will also open a multibuffer if there are multiple possible definitions.
