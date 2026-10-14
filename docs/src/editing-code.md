---
title: Editing Code in Zed
description: Core code editing features in Zed including multi-cursor, refactoring, code actions, and language server integration.
---

# Editing Code

Zed provides tools to help you write and modify code efficiently. This section covers the core editing features that work alongside your language server.

## What's in This Section

- **[Code Completions](./completions.md)** — Autocomplete from language servers and AI-powered edit predictions
- **[Snippets](./snippets.md)** — Insert reusable code templates with tab stops
- **[Formatting & Linting](./configuring-languages.md#formatting-and-linting)** — Configure automatic code formatting and linter integration
- **[Diagnostics & Quick Fixes](./diagnostics.md)** — View errors, warnings, and apply fixes from your language server
- **[Multibuffers](./multibuffers.md)** — Edit multiple files simultaneously with multiple cursors

## How These Features Work Together

When you're editing code, Zed combines input from multiple sources:

1. **Language servers** provide completions, diagnostics, and quick fixes based on your project's types and structure
2. **[Edit predictions](./ai/edit-prediction.md)** suggest multi-character or multi-line changes as you type
3. **Multibuffers** let you apply changes across files in one operation

For example, you might:

- Rename a function using your language server's rename refactor
- See the results in a multibuffer showing all affected files
- Use multiple cursors to make additional edits across all locations
- Get immediate diagnostic feedback if something breaks

## Hex Editing {#hex-editing}

Local binary files open in the hex editor by default when no specialized viewer,
such as the image viewer, handles them. Text files still open in the text editor.

To edit a local file as bytes, open it and run
{#action hex_editor::Open} from the command palette. You can also choose
**Open in Hex Editor** from a text file's tab context menu.
To choose a file directly, including binary files that cannot open as text,
run {#action hex_editor::OpenFile}.

The hex editor shows byte offsets, hexadecimal bytes, and editable ASCII cells.
Click a byte or use the arrow keys to move the cursor. Type two hexadecimal
digits to overwrite a byte. Switch to **Insert** to add bytes instead.
Typing at the end of the file appends bytes.

Hold `Shift` while moving the cursor or clicking to select bytes.
`Backspace` and `Delete` remove bytes. Copy and paste use hexadecimal text,
such as `00 FF 42`. Save, undo, and redo use the usual shortcuts.

Use the toolbar or command palette for these editing tools:

- **Find / Replace**, {#action hex_editor::Find} ({#kb hex_editor::Find}):
  choose **Hex**, **ASCII**, **UTF-8**, or **Number**. Hex accepts exact bytes
  such as `00 FF`, whole-byte wildcards (`??`), and nibble wildcards (`A?`,
  `?F`). Numeric search uses the selected integer or float type and byte order.
  **Next** and **Previous** select
  matches and wrap at the end of the file. **Replace** changes the selected
  match; **Replace All** changes non-overlapping matches in one undoable edit.
  Replacements can change the file size. Leave the replacement empty to delete
  matching bytes. Replacements use the selected format and cannot contain
  wildcards. Search patterns are limited to 4096 bytes.
- **Fill Selection**, {#action hex_editor::Fill} ({#kb hex_editor::Fill}):
  repeat a hexadecimal pattern across selected bytes without changing the file
  size. For example, `AA BB` fills five selected bytes with `AA BB AA BB AA`.
- **Go to Offset**, {#action hex_editor::GoToOffset}
  ({#kb hex_editor::GoToOffset}): jump to a decimal offset, such as `256`, or a
  hexadecimal offset prefixed with `0x`, such as `0x100`. The end-of-file offset
  lets you append bytes.
- **Select Range**, {#action hex_editor::SelectRange}
  ({#kb hex_editor::SelectRange}): enter a start offset and length. Switch
  **Length** to **End Offset** to select up to an exclusive end offset instead.
  Both fields accept decimal numbers or `0x`-prefixed hex. Select the range,
  then copy, delete, or fill it.
- **Inspect**, {#action hex_editor::Inspect} ({#kb hex_editor::Inspect}):
  read signed and unsigned 8-, 16-, 32-, and 64-bit integers and 32- and 64-bit
  floats. Switch between **Little Endian** and **Big Endian**. Choose a type,
  enter a numeric value, and choose **Apply** to overwrite existing bytes.
  The inspector starts at the selection's first byte or the cursor. A selected
  range limits the bytes it can read or overwrite. Click the **Bits 7..0**
  buttons to toggle individual bits in its first byte. Invalid or out-of-range
  values and edits that would extend past the inspected bytes are rejected.
- **Edit ASCII**, {#action hex_editor::ToggleAsciiMode}
  ({#kb hex_editor::ToggleAsciiMode}): switch between hex and ASCII input.
  Clicking an ASCII cell also switches to ASCII editing. Type ASCII characters
  to overwrite or insert bytes. Copy and paste use plain ASCII text in this
  mode, rather than hexadecimal text. Non-ASCII copy and paste are rejected;
  switch to hex editing to copy arbitrary bytes without losing data.
- **Save As**, {#action hex_editor::SaveAs} ({#kb hex_editor::SaveAs}):
  save the bytes to a new local file and continue editing that file.
  The original stays unchanged. Choose a path that does not already exist;
  hex Save As rejects existing destinations rather than overwriting them.
- **Read Only**, {#action hex_editor::ToggleReadOnly}: prevent byte edits,
  paste, deletion, replacement, fill, undo, redo, and saving to the current
  file. Save or undo pending changes before enabling it. Copy, search,
  inspection, bookmarks, and Save As remain available.
- **Bookmarks**, {#action hex_editor::Bookmarks}
  ({#kb hex_editor::Bookmarks}): name the selected range or cursor location,
  then jump back to it from the bookmark list. Locations follow insertions
  and deletions before them. Deleting either endpoint makes a bookmark
  unavailable until undo restores it. End-of-file bookmarks follow the end
  of the file. Bookmarks and annotations are stored in Zed's local database
  for each absolute file path and survive reopening or restarting Zed.
  Each file supports 256 entries.
- **Annotations**, {#action hex_editor::Annotations}
  ({#kb hex_editor::Annotations}): use the bookmark panel to attach a name
  and multiline notes to a range or cursor location. Choose **Edit** to
  change the name or notes, **Save Annotation** to apply them, **New** to
  start another entry, or **Use Selection** to move an entry to the current
  selection. Annotated bytes have an accent underline; hover to read the
  name and notes. Names support 256 bytes and notes support 16 KiB.
  Hover previews show up to four overlapping entries and the first 512
  characters of each note. Use **Edit** to read the full note.
- **Compare Files**, {#action hex_editor::CompareFiles}
  ({#kb hex_editor::CompareFiles}): choose a second local file to show its
  bytes beside the current buffer. Both sides are read-only during comparison.
  Highlights distinguish changed bytes, current-only bytes, and reference-only
  bytes. **Next Difference**, {#action hex_editor::NextDifference}
  ({#kb hex_editor::NextDifference}), and **Previous Difference**,
  {#action hex_editor::PreviousDifference}
  ({#kb hex_editor::PreviousDifference}), select differing regions and wrap.
  **Close Comparison** returns to editing without modifying either file.
  Comparison uses equal byte offsets, not insertion-aware alignment, and the
  reference is a snapshot. Choose the file again to refresh it. Scroll
  horizontally to see both sides in a narrow window.

Metadata does not change the binary file. Targets are anchored to its last
saved contents. Save the bytes before closing if a target includes newly
inserted bytes. Discarding edits restores targets on saved bytes; targets on
discarded inserted bytes become unavailable, but their names and notes remain.
If the file contents change outside Zed, reload keeps names and notes but marks
their targets unavailable rather than guessing new locations. Select a range
and use **Use Selection** to retarget them. Save As copies the metadata to the
new file while retaining the original file's metadata.

Press `Enter` in a tool's input field to find, replace, fill, jump, select, or apply
a numeric value, or add a bookmark. Press
`Enter` in the multiline notes field inserts a newline; use the save button to
apply notes. Press `Escape` or **Close** to return to the byte grid. All byte changes support undo
and redo and remain unsaved until you save the file.

Modified and inserted bytes use different highlights in both columns. Deleted
bytes have a marker at their former location and a count above the grid.
Unchanged bytes retain their normal appearance even when insertions or deletions
shift their offsets. Highlights compare against the last successful save or
reload and update when you undo or redo.

Hex editing supports local files up to 64 MiB. Save pending text edits before
opening the hex editor. If another editor changes the file on disk, saving
from the hex editor fails rather than overwriting those changes. Reload the
file before editing it again.

## Related Features

- [AI Features](./ai/overview.md) — Agentic editing, inline code transformations, and AI code completions
- [Configuring Languages](./configuring-languages.md) — Set up language servers for your project
- [Key Bindings](./key-bindings.md) — Customize keyboard shortcuts for editing commands
