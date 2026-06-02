---
title: Finding and Navigating Code - Zed
description: Navigate your codebase in Zed with file finder, project search, go to definition, symbol search, and the command palette.
---

# Finding & Navigating

Zed provides several ways to move around your codebase quickly. Here's an overview of the main navigation tools.

## Command Palette

The Command Palette ({#kb command_palette::Toggle}) is your gateway to almost everything in Zed. Type a few characters to filter commands, then press Enter to execute.

[Learn more about the Command Palette →](./command-palette.md)

## Project Panel

The Project Panel ({#kb project_panel::ToggleFocus}) shows a tree view of your workspace's files and directories. Browse, create, rename, move, and delete files without leaving the editor. It also surfaces git status and diagnostics at a glance.

[Learn more about the Project Panel →](./project-panel.md)

## File Finder

Open any file in your project with {#kb file_finder::Toggle}. Type part of the filename or path to narrow results.

### Ranking model

By default, file finder results are sorted purely by fuzzy match score. You can opt in to a richer ranking model that combines the fuzzy score with several user-shaped signals — recency, currently-open tabs, directory priority, and pinned files. All four are off by default, so adding the settings only changes behaviour for users who set them. Add any of the following to your `settings.json` to start tuning:

```json [settings]
{
  "file_finder": {
    // Weight (0.0–1.0) applied to recency. 0.0 disables it; 0.1 is a good
    // starting point. A freshly-visited file with a similar fuzzy score
    // will beat an older one.
    "recency_boost": 0.1,
    // Additive boost (0.0–1.0) for files currently open in any pane.
    "open_tab_boost": 0.05,
    // How the recency boost decays with the age of the last visit.
    // "linear" (default), "exponential", or "step".
    "recency_decay": "linear",
    // Number of days a file remains eligible for the recency boost.
    // Clamped to [1, 90].
    "recency_horizon_days": 7,
    // Path prefixes (must end with "/") that earn a small additive boost.
    "directory_priority": ["src/", "lib/"],
    // Path prefixes (must end with "/") that earn a small additive penalty.
    "directory_deprioritize": ["test/", "vendor/", "node_modules/"],
    // Glob patterns whose matches are pinned to the top of any query they match.
    "pinned_files": ["**/main.rs", "**/lib.rs"]
  }
}
```

Setting `recency_boost` and `open_tab_boost` to `0.0` (the default) restores the pure-fuzzy ranking introduced in PR #12103. The legacy `["file_finder::Toggle", { "separate_history": true }]` keybinding still works and is unaffected by these settings.

## Project Search

Search across all files with {#kb pane::DeploySearch}. Start typing in the search field to begin searching—results appear as you type.

Results appear in a [multibuffer](./multibuffers.md), letting you edit matches in place.

## Go to Definition

Jump to where a symbol is defined with {#kb editor::GoToDefinition} (or `Cmd+Click` / `Ctrl+Click`). If there are multiple definitions, they open in a multibuffer.

## Go to Symbol

- **Current file:** {#kb outline::Toggle} opens an outline of symbols in the active file
- **Entire project:** {#kb project_symbols::Toggle} searches symbols across all files

## Outline Panel

The Outline Panel ({#kb outline_panel::ToggleFocus}) shows a persistent tree view of symbols in the current file. It's especially useful with [multibuffers](./multibuffers.md) for navigating search results or diagnostics.

[Learn more about the Outline Panel →](./outline-panel.md)

## Tab Switcher

Quickly switch between open tabs with {#kb tab_switcher::Toggle}. Tabs are sorted by recent use—keep holding Ctrl and press Tab to cycle through them.

[Learn more about the Tab Switcher →](./tab-switcher.md)

## Quick Reference

| Task              | Keybinding                       |
| ----------------- | -------------------------------- |
| Command Palette   | {#kb command_palette::Toggle}    |
| Open file         | {#kb file_finder::Toggle}        |
| Project search    | {#kb pane::DeploySearch}         |
| Go to definition  | {#kb editor::GoToDefinition}     |
| Find references   | {#kb editor::FindAllReferences}  |
| Symbol in file    | {#kb outline::Toggle}            |
| Symbol in project | {#kb project_symbols::Toggle}    |
| Outline Panel     | {#kb outline_panel::ToggleFocus} |
| Tab Switcher      | {#kb tab_switcher::Toggle}       |
| Project Panel     | {#kb project_panel::ToggleFocus} |
