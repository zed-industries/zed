# Finding & Navigating

Zed provides several ways to move around your codebase quickly. Here's an overview of the main navigation tools.

## Command Palette

The Command Palette ({#kb command_palette::Toggle}) is your gateway to almost everything in Zed. Type a few characters to filter commands, then press Enter to execute.

[Learn more about the Command Palette →](./command-palette.md)

## File Finder

Open any file in your project with {#kb file_finder::Toggle}. Type part of the filename or path to narrow results.

## Project Search

Search across all files with {#kb pane::DeploySearch}. Results appear in a [multibuffer](./multibuffers.md), letting you edit matches in place.

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
