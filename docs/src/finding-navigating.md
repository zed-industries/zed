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

## Text Finder

Quickly find any string in your project and open the file with {#kb project_search::OpenTextFinder}. Changed your mind and want a more detailed search with extra filters? Move to the project search using the button in the Actions menu in the right bottom corner.

## Project Search

Search across all files with {#kb pane::DeploySearch}. Type the query in the search field, then press Enter to run the search.

Results appear in a [multibuffer](./multibuffers.md), letting you edit matches in place.

### File Metadata Filters

Alongside the "Include" / "Exclude" [glob](./globs.md) filters, the filter row has a
metadata field that narrows the search by file size and modification time using
predicates borrowed from `find(1)`:

| Predicate | Meaning                                | Example                             |
| --------- | -------------------------------------- | ----------------------------------- |
| `-name`   | Case-sensitive glob on the file name   | `-name *.rs`                        |
| `-iname`  | Case-insensitive glob on the file name | `-iname *.RS`                       |
| `-size`   | File size                              | `-size 9` (larger than 9 KiB)       |
| `-mtime`  | Age in whole days                      | `-mtime -7` (modified in last week) |
| `-mmin`   | Age in whole minutes                   | `-mmin +30` (untouched for 30 min)  |

`+N` means "greater than N" and `-N` means "less than N". Predicates combine with
AND, so `-size 1 -mtime -7` matches files that are both larger than 1 KiB and
modified within the last week. Because predicates are whitespace-separated, a
glob cannot contain spaces.

`-size` departs from `find` twice, in favour of the common "show me the big
files" case: its default unit is **KiB**, and an unsigned value means **greater
than** rather than "equal to". So `-size 9` is the same as `-size +9k`, and
`-size +1` matches files over 1024 bytes. An explicit unit suffix still wins —
`c` (bytes), `b` (512-byte blocks), `k`, `M`, `G`. For `-mtime`/`-mmin` an
unsigned value keeps `find`'s "equal to" meaning.

`-name`/`-iname` overlap the "Include" glob field: they match the file name only,
where "Include" matches the whole path. The same syntax also drives the
[project panel's file filter](./project-panel.md#file-filter).

These are matched against metadata Zed's worktree scan already holds, so they
narrow the candidate set before any file is read.

> Note: the syntax is `find`-inspired, not `find`-compatible. `-size` counts
> bytes rather than 512-byte blocks by default and rounds down rather than up.

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

| Task               | Keybinding                           |
| ------------------ | ------------------------------------ |
| Command Palette    | {#kb command_palette::Toggle}        |
| Open file          | {#kb file_finder::Toggle}            |
| Project search     | {#kb pane::DeploySearch}             |
| Text search picker | {#kb project_search::OpenTextFinder} |
| Go to definition   | {#kb editor::GoToDefinition}         |
| Find references    | {#kb editor::FindAllReferences}      |
| Symbol in file     | {#kb outline::Toggle}                |
| Symbol in project  | {#kb project_symbols::Toggle}        |
| Outline Panel      | {#kb outline_panel::ToggleFocus}     |
| Tab Switcher       | {#kb tab_switcher::Toggle}           |
| Project Panel      | {#kb project_panel::ToggleFocus}     |
