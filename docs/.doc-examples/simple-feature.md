<!--
  GOLD STANDARD EXAMPLE: Simple Feature / Overview Documentation

  This example demonstrates concise documentation for a feature overview
  or navigation guide.

  Key patterns to note:
  - Anchor IDs on all sections
  - Brief opening paragraph explains what this covers
  - Each section is concise (1-2 paragraphs max)
  - Links to detailed docs for each feature
  - Quick reference table at the end
  - Uses {#kb ...} syntax for all keybindings
-->

---

title: Finding and Navigating Code - Zed
description: Navigate your codebase in Zed with file finder, project search, go to definition, symbol search, and the command palette.

---

# Finding & Navigating

Zed provides several ways to move around your codebase quickly. Here's an overview of the main navigation tools.

## Command Palette {#command-palette}

The Command Palette ({#kb command_palette::Toggle}) is your gateway to almost everything in Zed. Type a few characters to filter commands, then press Enter to execute.

[Learn more about the Command Palette →](./command-palette.md)

## File Finder {#file-finder}

Open any file in your project with {#kb file_finder::Toggle}. Type part of the filename or path to narrow results.

## Project Search {#project-search}

Search across all files with {#kb pane::DeploySearch}. Results appear in a [multibuffer](./multibuffers.md), letting you edit matches in place.

## Go to Definition {#go-to-definition}

Jump to where a symbol is defined with {#kb editor::GoToDefinition} (or `Cmd+Click` / `Ctrl+Click`). If there are multiple definitions, they open in a multibuffer.

## Go to Symbol {#go-to-symbol}

- **Current file:** {#kb outline::Toggle} opens an outline of symbols in the active file
- **Entire project:** {#kb project_symbols::Toggle} searches symbols across all files

## Outline Panel {#outline-panel}

The Outline Panel ({#kb outline_panel::ToggleFocus}) shows a persistent tree view of symbols in the current file. It's especially useful with [multibuffers](./multibuffers.md) for navigating search results or diagnostics.

[Learn more about the Outline Panel →](./outline-panel.md)

## Tab Switcher {#tab-switcher}

Quickly switch between open tabs with {#kb tab_switcher::Toggle}. Tabs are sorted by recent use—keep holding Ctrl and press Tab to cycle through them.

[Learn more about the Tab Switcher →](./tab-switcher.md)

## Quick Reference {#quick-reference}

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

## See Also {#see-also}

- [Command Palette](./command-palette.md) — Full command palette documentation
- [Multibuffers](./multibuffers.md) — Edit multiple files simultaneously
- [Outline Panel](./outline-panel.md) — Symbol tree view
- [Tab Switcher](./tab-switcher.md) — Switch between open files
