# My workflow for macOS

To test modifications, I am only using `cargo run` (note that when `webrtc-sys` takes too long, use `Warp` for a faster download) to compile and start Zed in debug mode which is faster than building the release binaries.

Once, I am satisfied with a batch of changes, I install Zed into `/Applications/Zed Dev.app` with this:

```bash
./script/bundle-mac-without-licenses -l -o -i && \
rm -f "$HOME/.cargo/bin/zed" && \
ln -s "/Applications/Zed Dev.app/Contents/MacOS/cli" "$HOME/.cargo/bin/zed"
```

In `crates/zed/src/main.rs` and `crates/remote_server/src/server.rs`, I changed it so crashes in Dev builds (like above) will produce a `.dmp` + `.json` file in `~/Library/Logs/Zed/` unless `ZED_GENERATE_MINIDUMPS` is set to `false` or `0`.

## Development note

To have easier Zed's `main` branch merges, I am not really adding or modifying existing unit tests to my own functionality, so some are failing. I try to satisfy `./script/clippy`, though.

---

AI is heavily used for pretty much every feature implemented. I use a mix of those models, all free:
- Google's Gemini 3 Pro (https://aistudio.google.com/prompts/new_chat)
- OpenCode (which often has free models)
- Windsurf's free models (usually SWE)

For anything more complicated, I use the Architect + Editor pattern (see https://aider.chat/2024/09/26/architect.html) with Gemini 3 Pro being the Architect and the other models being the Editor.

## Sync this fork's main branch with Zed's main branch and merge into my custom dima branch

```bash
git checkout main && git pull zed main && git push && git checkout dima && git merge main
```

If there are merge conflicts, I resolve them via IntelliJ IDEA.

## Compare my changes with Zed's main branch

https://github.com/zed-industries/zed/compare/main...Dima-369:zed:dima

# Fork changes

## General/editor changes

- add many defaults in `project_settings.rs` to not crash on startup (not sure if that is only from my code)
- add `bundle-mac-without-licenses` which is faster than generating licenses, and skips the `sentry-cli` at end
- try to fix panic in `anchor_at_offset` when buffer has Umlaute, seems to work, no idea if my fix has other consequences
- fix crash "offset is greater than snapshot.len()" in `MultiBufferOffset::to_offset` by clamping out-of-bounds offsets to `snapshot.len()` instead of panicking. This happens when stale offsets (from a previous buffer state) are used after the buffer is edited and becomes shorter — common with async edits from agent/LSP format-on-save
- changed `fn do_copy(&self, strip_leading_indents: bool, cx: &mut Context<Self>) {` to only strip trailing newlines instead of leading indents
- lower `MIN_NAVIGATION_HISTORY_ROW_DELTA` to 3, from 10, as a test which seems fine
- opening a workspace which has no tabs initially, will trigger `workspace::NewFile` for proper editor focus. Before, there seems to be a bug where the project panel does not have proper focus
- improved the `go to next/previous diagnostic` action to always jump to errors first. Only if there are no errors, it jumps to warnings. Before, this was mixed
- moving up/down in outline panel does not wrap around anymore
- fixed that a large `vertical_scroll_margin` in `settings.json` to have a centered cursor jumps buffer scrolls around (https://github.com/zed-industries/zed/issues/42155)
- fixed that on entering the project search, there can be instances where visual mode is entered (https://github.com/zed-industries/zed/issues/43878)
- integrate file explorer modal from https://github.com/zed-industries/zed/pull/43961(Add file explorer modal, PR closed)
- integrated live refreshing project search from https://github.com/zed-industries/zed/pull/42889, enable in `settings.json` via `search > search_on_input`
- integrated smooth scroll from https://github.com/zed-industries/zed/pull/31671
- modified `compute_style_internal()` in `crates/gpui/src/elements/div.rs` to not apply the mouse hover style, since it clashes when one only uses the keyboard
  - I also unset the mouse hover background change on enabled `sticky_scroll`
- improved `outline::Toggle` to work in multi buffers, it shows the file headings only
- improve `editor::SelectLargerSyntaxNode` for inline code blocks in Markdown files (`foo bar`), so that it first extends the selection to the word inside the quotes, then the text inside the quotes and only then to the inner text plus the outer quotes
- improve `editor::AcceptNextWordEditPrediction` to not insert a sole space when a space is before a word in the suggestion. Now, it inserts both the space and the word
- styled the edit prediction "Jump to Edit" line popover to 50% opacity, removed "Jump to Edit" text, lowered padding, and decreased font size
- exclude unnamed/scratch buffers (tabs without a file) from project search results (`crates/project/src/project_search.rs`)
- patch `settings_changed()` in `crates/editor/src/editor.rs` to properly reload the buffer font family, so I can switch trivially between a monospace and proportional font (I am not sure why only my fork needs it, and `Zed.app` doesn't)

## Smooth animated cursor with trail (not in terminal)

Based on the `smooth-cursor` branch from <https://github.com/NVSRahul/zedmod> with several tweaks.
The animation code is pretty much the same as kitty's implementation, but it is still different because in Zed the cursor is instantly moved and just the trail animated.

- fixed that mouse clicks would not animate
- fixed bug that the character at the vim block cursor is not rendered until `smooth_time` is passed
- fixed that actions like `jump::Toggle`, `editor::GoToPreviousGlobalChange` or `pane::GoBack` would not animate (same for forward)
- added `large_jump_multiplier` (set to 1.0 to keep default behavior) which is potentially useful for large jumps to top/bottom of editor since I also have smooth scroll and the 2 animation systems 

Example `settings.json` configuration:

```json
"smooth_cursor": {
  "enabled": true,
  "trail": true,
  // same as kitty's defaut 400 and 100
  "smooth_time": 400,
  "leading_smooth_time": 100,
  "trail_opacity": 0.6,
  "trail_min_distance": 10, // set higher to avoid triggering on simple typing
  "large_jump_multiplier": 1.0, // disable the multiplier (was a test anyway, but with kitty's defaults it is not required)
},
```

## Motion system to make Zed feel fluid and intentional

**Experimental**. I will see if it causes annoying merge conflicts in the future and if I like keeping those animations personally.

I tested out the PR and I actually dislike all implemented animations apart from the dock animation, so I only took in that animation code from the PR. Everything else, the popover/dialog/modal opening/closing and the picker animation just make Zed feel sluggish.
I also removed the settings code for reduce motion, as I don't need it, and have it always enabled.

From <https://github.com/zed-industries/zed/pull/48295>

## Images

- on viewing an image, the `ImageViewer` key context is enabled, previously there was no context

## Go To Global Change Actions

Akin to the existing `editor::GoToPreviousChange` and `editor::GoToNextChange` actions, I implemented `editor::GoToPreviousGlobalChange` and `editor::GoToNextGlobalChange`.

It behaves like the JetBrains IDEs actions: `Last Edit Location` and `Next Edit Location`.

## Clipboard History Modal

- implement a filterable clipboard history model (opened via `clipboard_history_modal::ToggleClipboardHistory`) which keeps track of text clipboard actions like `editor::Copy`, `editor::Cut`, and `editor::CopyAll`. On confirming it pastes in the selected entry
  - inspired from `Choose Content to Paste` from JetBrains IDEs
  - in `crates/workspace/src/persistence.rs` there is own SQL table `clipboard_history`, so the recent entries is remembered across restarts

Every 500ms the system clipboard is monitored for any new changes, to react on external application changes.

## Open directory listing as a Editor (relatively basic file explorer)

This is inspired by `oil.nvim` for Neovim (https://github.com/stevearc/oil.nvim) or `vinegar` for Vim (https://github.com/tpope/vim-vinegar).

Take care to only modify the file names in the editor, not the top directory name or the newlines below the directory name, otherwise the logic on saving will not work.

You can empty a line to trash a file. It only works on macOS because it is using the `trash` CLI.

Deleting or adding lines is not supported, this is for file browsing and file renaming via usual editor keybindings.

### New actions

- `workspace::FileExplorerOpen` (the entry point)
- `workspace::FileExplorerOpenFile` (has a `close: bool` parameter to close the file explorer after opening the file, default is `true`)
- `workspace::FileExplorerNavigateToParentDirectory`
- `workspace::FileExplorerSaveModified` (show a confirmation dialog which lists all changes)
- `workspace::FileExplorerCreateFile` (show a modal to input a file name and open the newly created file; this also works outside this file explorer)
- `workspace::FileExplorerReload` (reload the current directory listing while preserving the cursor position on file name)

### Implementation

See `crates/editor/src/editor.rs`, search for `file_explorer` and related functions.

`FileIcon(usize)` was added to `pub enum InlayId` to display the SVG icon from the theme, same as the file icons from the file tabs.

`set_vim_insert_on_focus` was added to `editor.rs` to start in Vim insert mode when the editor is focused. I needed this for the `crates/editor/src/create_file_modal.rs` since I want to use Vim mode there, and start in insert mode instead of the default normal mode.

## Emoji Picker

Implement `emoji_picker_modal::ToggleEmojiPicker` which opens a modal and on picking an emoji, it is copied into the clipboard.

You modify the emojis in your `settings.json` like this in the root setting object:

```json
"emoji_picker": [
  "😄 smile",
  "😮 surprise",
  "😢 sad"
]
```

## Keyboard Editor

- allow `cmd-escape` to break out of the `Search by Keystroke` mode

## Keyboard Context

- allow mouse left click on the action name or context to copy into clipboard
- color the Last Keystroke action name to the same color as the `(match)`, `(low precedence)` and `(no match)` labels
  - on my smaller screen, I often do not see the end of a text row, and placing the color at front, helps immensely
- wrap the context labels, so they are always fully readable. I need this for my smaller screen

## Vim/Helix

- add `vim_visual` context which can be set to `normal`, `line` or `block` for more fine-grained keybindings
- disabled vim insert mode grouping (`crates/vim/src/vim.rs`) and made newlines always break undo grouping (`crates/editor/src/editor.rs`) for more fine-grained undo
- paste (`vim::Paste` and `editor::Paste`), newline (`editor::Newline`), newline below (`editor::NewlineBelow`) and newline above (`editor::NewlineAbove`) in vim mode now each create their own undo transaction, so undoing them no longer also undoes prior typed characters
- modified `vim/.../delete_motion.rs` so `vim::DeleteRight` at end of line stays on the newline character
- patch `clip_at_line_end()` in `crates/editor/src/display_map.rs` as a no-op when no editor selections are there, to have an always enabled `virtualedit=onemore` mode
  - this also allows mouse clicking beyond the end of the line where no characters are, to select the newline character in vim mode
- fix bug that when in vim visual line mode and cursor is on right newline character, that the line below is incorrectly copied on `editor::Copy`. This mostly happens in my own Zed config because I mixing `editor` and `vim` actions to ensure that I can move cursor on the right newline character, and usually not in proper Zed keybindings.

## Network

- add `"proxy_no_verify": true` support in `settings.json`

## JJ

- add `jjdescription` to `crates/languages/src/diff/config.toml > path_suffixes`  to change commit message description

## Git

- add `blame > git_blame_font_family` setting to specify the font family for the git blame view because I am using a proportional font and the blame view misaligns otherwise
- add `git::DiffWithCommit` from https://github.com/zed-industries/zed/pull/44467 and based on that code, `git::DiffWithBranch` is implemented
- add `({file count})` in the git panel to every directory, inspired by https://github.com/zed-industries/zed/pull/45846 (Improve Git Panel with TreeView, VSCode-style grouping, commit history, and auto-fetch)
- add `split_diff_font_decrease` setting to configure font size decrease for split diff view (default is 30%)
- split diff view uses 30% smaller font size than stacked view

### Project Diff Tab Changes

- make the title dynamic: if there are no files, it shows `No Changes`, otherwise it shows `Uncommitted Changes (1 file)` or `Uncommitted Changes (n files)`
- make the icon and text foreground dynamic when the tab is not selected with the same logic as the "Recent Branches" from `crates/title_bar/src/title_bar.rs`. See `fn tab_content(&self, params: TabContentParams, _window: &Window, cx: &App) -> AnyElement`

### Git Split View Navigation

- Added keyboard context to allow Git split view navigating. See `crates/editor/src/split.rs`. New context strings added: `editor_split_view`, `editor_is_split`, `editor_side_left`, and `editor_side_right`.

### Git Commit Modal

- move git commit modal to the right side instead of being centered, so it does not overlap the left git dock, which makes it impossible to see what files are staged on a small screen. One could lower the size of the git dock to make it fit, but then it is quite small
  - and use `text_accent` for its border color to be easier to see
- preset git commit message with "Update {file count} files" on multiple files and add every file name to the commit description as a list with a `-` prefix

## Zed CLI

Add `--stdin-cursor-at-end` flag to CLI to position cursor at end of buffer when reading from stdin instead of at start which I find more useful.

I developed and tested it like this:

```bash
timeout 15s bash -c 'cat README.md | target/debug/cli --zed target/debug/zed --stdin-cursor-at-end --foreground -' 2>&1 || echo "Timeout reached"
```

## Terminal

- remove abbreviated `cwd` display in terminal title
- add `terminal::OpenScrollbackBuffer` action to open the scrollback buffer in a new buffer. It positions the cursor at the end

### Vi Mode

- add little indicator at top of terminal view to display if Vi Mode is enabled
- modify Vi Mode keys to my custom Dvorak Programmer keyboard layout inspired by https://github.com/xahlee/xah-fly-keys
  - see diff in `crates/terminal/src/terminal.rs` compared to Zed's `main` branch for key changes
  - `enter` sets the cursor position for a selection
  - `escape` clears the selection if there is one, otherwise exits Vi Mode
- fix bug that in Vi Mode on key press, the terminal does not rerender, so the cursor position is not updated

## AI

- allow AI edit predictions in the following places:
  - Zed's `settings.json`
  - Zed's `keymap.json`
  - buffers without files like ones from `workspace: new file`
  - AI agent text threads

### Agent UI changes

**Warning**: The code around this is extremely brittle, I very often have large merge conflicts, so I revert affected files to the `main` version. Afterwards, I let AI patch in this functionality from previous working code, but it might miss things.

#### Concurrent Agent Tabs

Add concurrent agent tabs from 
<https://github.com/wzulfikar/zed/pull/8> (which was based on <https://github.com/zed-industries/zed/pull/42387>)

#### New Actions

- `agent::DismissOsNotifications` to dismiss the top right OS notification from Zed Agent. With multiple tabs, I feel that the notifications get stuck sometimes
- `agent::CloseActiveThreadTabOrDock`
- `agent::ActivateNextTab` / `agent::ActivatePreviousTab`
- `agent::TogglePlan` to toggle the plan of the current thread
- `agent::LaunchAcpAgent` which takes an external agent name and can be bound like this:
  - `"cmd-t": ["agent::LaunchAcpAgent", { "agent_name": "junie" }]`

These are missing in latest `dima` branch (I had them implemented at same point):

- `agent::DismissErrorNotification` / `agent::CopyErrorNotification`

#### Other (probably missing on latest `dima` branch)

- remove the opacity animation for the tabs when waiting for a response and instead display an accent color circle to indicate it's waiting for a response
- Zed Agent, External Agents and text thread title summaries are now generated on every AI message received
- change `agent::OpenActiveThreadAsMarkdown` to always open to end of buffer instead of start, and when there are more than 90k lines, open as `Plain Text` because Markdown lags hard for me, see `crates/agent_ui/src/acp/thread_view.rs`
- always allow all edits, otherwise it kepts asking for "Allow All Edits" every single time a new ACP thread is started which is just annoying. Note that it still asks for tool permissions
- fix: ACP sessions with session modes (e.g. Claude Code's brave/bypassPermissions mode) now respect the `always_allow_tool_actions` setting — previously `respect_always_allow_setting` was set to `false` when `session_modes` existed, causing tool permission prompts even with brave mode enabled
- show command output for `acp::ToolKind::Execute` always below the `Run Command` view in a plain text view to preserve newlines
  - I added `prepare_execute_tool_output_from_qwen()` to strip trailing and leading information for cleaner output
- allow `New From Summary` for ACP agents, instead of only for Zed Agent
- add `editor::NewTextThreadInEditor` action to create a new AI text thread as a standalone editor tab
  - add `agent::SendMessage` action to trigger sending the current message in a text thread editor, otherwise you can't send messages from the text thread editor tab via keyboad

#### Agent OS Notifications

See  `crates/agent_ui/src/ui/agent_notification.rs`.

- increase button size
- use vertical lines and display the agent tab name in the notification, if set

### Command palette

- the command palette sorting now sorts the same for `close work` and `work close`, and it does not search individual character matches anymore, like when you enter `bsp`, it would show `editor: backspace` before. I do not like that behavior, so I removed that
- change `command palette: toggle` to sort by recency instead of hit count
- remove `GlobalCommandPaletteInterceptor` usage which contains Vim things like `:delete, :edit, :help, :join, :quit, :sort, :write, :xit, :yank` because I do not use them. Apparently, this removes the ability to jump to a line via `:144`. I still removed this behavior because it is hard to sort those dynamic actions by recency in combination with the other real editor action commands.

## Project Panel

- renaming a file uses a vim compatible editor and starts in vim normal mode

## `keymap.json` changes

- add a JSON boolean key `highest_precedence` to the keymap dictionaries for tricky keybindings which otherwise require rewriting many other keybinding blocks
  - I originally implemented it for the vim mode support in project panel rename file editor to handle `Enter` in vim's insert mode

## Recent file and zoxide functionality

### `workspace::OpenRecentFile`

A new modal for recent file functionality which tracks every opened buffer in a new `persistence.rs` SQL table to quickly jump to a recent file (which can in turn open a new workspace).

## `projects::OpenRecentZoxide` for Zoxide (https://github.com/ajeetdsouza/zoxide)

A new modal which displays recent directories from the `zoxide` CLI binary.

It displays no footer and abbreviates paths to tildes.

`highlighted_label.rs` was adjusted for its filtering. Here `cmd+enter` is flipped, so by default, it always opens in a new window.

## New actions

- `Markdown::ScrollPageLittleDown` and `Markdown::ScrollPageLittleUp` which scroll a quarter of a page
- `workspace::NewFileFromClipboard` which pastes in the clipboard contents
  - the action supports setting an initial language like `"space n j": [ "workspace::NewFileFromClipboard", { "language": "json" } ],` in `keymap.json`
- `workspace::CopyFilePaths` which opens a picker to copy the file path to clipboard
- `workspace::MakeSinglePane` which closes all other panes except the active one
- `snippets::ReloadSnippets` because auto-reloading snippets is not working for me
- `editor::CopyAll` to copy entire buffer content to clipboard
- `editor::CountTokens` which counts the tokens in the current buffer using `o200k_base` via the `tiktoken` crate
- `editor::StopAllLanguageServers` which stops all language servers. It works like the bottom button in `Language Servers > Stop All Servers`
- `project_lsp_treesitter_symbol_search::Toggle` based on `search_everywhere::Toggle` from https://github.com/zed-industries/zed/pull/45720. I ripped out everything else except the symbol search. The reason this is better than the built-in `project_symbols::Toggle` is that it uses both Tree-sitter and LSP with indexing which is faster and more reliable.
- `editor::MoveToStartOfLargerSyntaxNode` from https://github.com/zed-industries/zed/pull/45331
- `buffer_search_modal::ToggleBufferSearch` which shows a modal to search the current buffer content (code is in `crates/search/src/buffer_search_modal.rs`) based on https://github.com/zed-industries/zed/pull/44530 (Add quick search modal). This is a basic implementation of Swiper from Emacs or `Snacks.picker.lines()` from Neovim. I tried matching every line with `nucleo`, but it was kinda slow, so it just split on spaces and then every line which has all words from the query is matched.
  - `ctrl-c` and `ctrl-t` can be used to insert history items into the search field
  - `ctrl-r` is to toggle between line (case-insensitive) and exact match (case-sensitive) mode
  - it also works in multi buffers, although the preview editor mixes lines

## Buffer Search

- never prefill the buffer search input field with the word under the cursor

## Hint jumping functionality

### `jump::Toggle` as a new action (this is for everything except multi buffers)

From https://github.com/tebben/zed/tree/feature/jump

With the following changes:

- modify key jump hints to my custom Dvorak Programmer keyboard layout
- implement multiple character jump hints
- set the opacity of the dialog to 50% to see hints below
- implement `jump::JumpToUrl` based on this code to jump to `http...` URLs
- note that it does not work in multi buffers, but it works to jump across panes of regular text editors

### `vim::HelixJumpToWord` as a new action (this is for multi buffers)

From https://github.com/zed-industries/zed/pull/43733

- improved UI to look like the `jump::Toggle` action
- removed the `helix > "jump_label_accent"` setting since the UI is now the same as `jump::Toggle`
- modified key jump hints to my custom Dvorak Programmer keyboard layout
- I am only using this is inside multi buffers, whereas `jump::Toggle` does not. And this also does not work to jump across editor panes
- note that escape does not work to break out of this mode, apparently. I have no idea how to adjust the code for it

## DeepL integration

There is this new action:  `zed::DeeplTranslate` which translates the current selection or the current line. It needs the `DEEPL_API_KEY` environment variable to be set. Bind like this:

```json
"space c g": [
  "zed::DeeplTranslate",
  {
    "source_lang": "EN",
    "target_lang": "DE",
  }
],
```

## File Finder modal, the `file_finder::Toggle` action

- `file_finder > modal_max_width=full` does not take full width anymore because it looks weird, but subtracts 128 pixels
- show scrollbar
- try to improve matching to use substring through `nucleo` crate. I dislike fuzzy matching which is annoying. Based on https://github.com/zed-industries/zed/pull/37123, but that had fuzzy matching
  - `nucleo` has its issues when you search in this repo for just `README`, it does not prioritize the root `README.md`, but other READMEs from other crates, but at least there is no weird fuzzy matching anymore

## UI changes

- use larger font size (`LabelSize::Default`) for the line/column and selection info in the bottom bar and use `text_accent` for it when a selection is active
- lower status bar height, see `impl Render for StatusBar`
- add scrollbar to `outline::Toggle`, `file_finder::Toggle` and `command_palette::Toggle` (why is it not shown in the first place?)
- lower `toolbar.rs` height to save space, same in `breadcrumbs.rs` (here no padding is set). This applies for terminals, as well
- lower `DEFAULT_TOAST_DURATION` from 10 to 5 seconds

### Custom Confirmation Modal

A new custom modal which bypasses the macOS native dialog to allow for easier keybindings and nicer UI.
This replaces Zed's unsaved changes modal.

See `crates/workspace/src/confirmation_dialog.rs`. The dismiss action is currently hardcoded to key `h`.

### Scrollbar

- hide horizontal scrollbar when soft wrap is enabled
- adjust scrollbar UI to look rounded and more native to macOS (idea is from this fork: https://github.com/notnotjake/zed)

## Tabs

- align the right slot of tabs (the directory) to the file name baseline, meaning in such cases: `README.md   db` (see diff in `crates/editor/src/items.rs`)
  - as recommended in `Refactoring UI > Baseline, not center`
- lower excessive tab height
- switch system tab background color from `title_bar_background` to `tab_bar_background`, so I can style active tabs far nicer because the default just uses a slightly different foreground color which is hard to spot
- improved `tab_switcher::ToggleAll` to not interfere with `pane::AlternateFile` (before just cycling through the tab list always tracked it for the alternate file action which is annoying)

### Vertical stacking tabs

The vertical tabs stack to next rows without scrollbars. Enable in `settings.json` with:

```json
"tab_bar": {
  "vertical_stacking": true
}
```

It places pinned tabs in an own row, separated to non-pinned tabs.

**Note:** Since it was too difficult to only render tab borders where exactly required, every tab now has a full border, so it looks a bit bold between tabs, but I don't mind. It looks better that way, instead of missing top borders in second row, for instance, when first row has pinned tabs.

# Original README

# Zed

[![Zed](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/zed-industries/zed/main/assets/badge/v0.json)](https://zed.dev)
[![CI](https://github.com/zed-industries/zed/actions/workflows/run_tests.yml/badge.svg)](https://github.com/zed-industries/zed/actions/workflows/run_tests.yml)

Welcome to Zed, a high-performance, multiplayer code editor from the creators of [Atom](https://github.com/atom/atom) and [Tree-sitter](https://github.com/tree-sitter/tree-sitter).

---

### Installation

On macOS, Linux, and Windows you can [download Zed directly](https://zed.dev/download) or install Zed via your local package manager ([macOS](https://zed.dev/docs/installation#macos)/[Linux](https://zed.dev/docs/linux#installing-via-a-package-manager)/[Windows](https://zed.dev/docs/windows#package-managers)).

Other platforms are not yet available:

- Web ([tracking discussion](https://github.com/zed-industries/zed/discussions/26195))

### Developing Zed

- [Building Zed for macOS](./docs/src/development/macos.md)
- [Building Zed for Linux](./docs/src/development/linux.md)
- [Building Zed for Windows](./docs/src/development/windows.md)

### Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for ways you can contribute to Zed.

Also... we're hiring! Check out our [jobs](https://zed.dev/jobs) page for open roles.

### Licensing

Zed source code is licensed primarily under GPL-3.0-or-later, with Apache-2.0 components where marked.

License information for third party dependencies must be correctly provided for CI to pass.

We use [`cargo-about`](https://github.com/EmbarkStudios/cargo-about) to automatically comply with open source licenses. If CI is failing, check the following:

- Is it showing a `no license specified` error for a crate you've created? If so, add `publish = false` under `[package]` in your crate's Cargo.toml.
- Is the error `failed to satisfy license requirements` for a dependency? If so, first determine what license the project has and whether this system is sufficient to comply with this license's requirements. If you're unsure, ask a lawyer. Once you've verified that this system is acceptable add the license's SPDX identifier to the `accepted` array in `script/licenses/zed-licenses.toml`.
- Is `cargo-about` unable to find the license for a dependency? If so, add a clarification field at the end of `script/licenses/zed-licenses.toml`, as specified in the [cargo-about book](https://embarkstudios.github.io/cargo-about/cli/generate/config.html#crate-configuration).

## Sponsorship

Zed is developed by **Zed Industries, Inc.**, a for-profit company.

If you’d like to financially support the project, you can do so via GitHub Sponsors.
Sponsorships go directly to Zed Industries and are used as general company revenue.
There are no perks or entitlements associated with sponsorship.
