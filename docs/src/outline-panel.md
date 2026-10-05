---
title: Outline Panel - Zed
description: Navigate code structure with Zed's outline panel. View symbols, jump to definitions, and browse file outlines.
---

# Outline Panel

In addition to the modal outline (`cmd-shift-o`), Zed offers an outline panel. The outline panel can be deployed via `cmd-shift-b` ({#action outline_panel::ToggleFocus} via the command palette), or by clicking the `Outline Panel` button in the status bar.

When viewing a "singleton" buffer (i.e., a single file on a tab), the outline panel works similarly to that of the outline modal－it displays the outline of the current buffer's symbols. Each symbol entry shows its type prefix (such as "struct", "fn", "mod", "impl") along with the symbol name, helping you quickly identify what kind of symbol you're looking at. Clicking on an entry allows you to jump to the associated section in the file. The outline view will also automatically scroll to the section associated with the current cursor position within the file.

![Using the outline panel in a singleton buffer](https://zed.dev/img/outline-panel/singleton.png)

## Navigation {#navigation}

Single-click a row to select it and scroll the editor without expanding or collapsing it, including folded files and symbols.
Double-click a row to navigate and focus the editor, except for folder and excerpt rows, which keep focus in the outline panel.

Click an entry's expand/collapse arrow to fold or unfold it without navigating to it.
Clicking a file or folder icon navigates like clicking its label; it does not toggle expansion.

For folder arrows, set `outline_panel.folder_indicator` to `"chevron"` or `"both"`.
The `"icon"` setting hides folder arrows, but you can still use {#action outline_panel::ExpandSelectedEntry} and {#action outline_panel::CollapseSelectedEntry}.

In a multibuffer, clicking a folder row scrolls to the first file beneath that occurrence of the folder and leaves the folder selected.
This also works for collapsed folders and compacted folder paths.
When symbols are hidden in a multibuffer, file rows have no expand/collapse arrows because they have no children in the panel.
Folder arrows remain available, and clicking any row still navigates without changing its folded state.

## Usage with multibuffers

File rows follow the order of the files in the multibuffer.
If files from a folder appear in separate parts of the multibuffer, the outline panel repeats that folder's path group to preserve their order.

When a multibuffer includes deleted project files, they remain grouped under their original paths with struck-through file names.
Folder rows group files by path and are not struck through, even when the folders no longer exist on disk.

The outline panel truly excels when used with multi-buffers. Here are some examples of its versatility:

### Project Search Results

Get an overview of search results across your project.

![Using the outline panel in a project search multi-buffer](https://zed.dev/img/outline-panel/project-search.png)

### Project Diagnostics

View a summary of all errors and warnings reported by the language server.

![Using the outline panel while viewing project diagnostics multi-buffer](https://zed.dev/img/outline-panel/project-diagnostics.png)

### Find All References

Quickly navigate through all references when using the {#action editor::FindAllReferences} action.

![Using the outline panel while viewing `find all references` multi-buffer](https://zed.dev/img/outline-panel/find-all-references.png)

The outline view provides a great way to quickly navigate to specific parts of your code and helps you maintain context when working with large result sets in multi-buffers.
