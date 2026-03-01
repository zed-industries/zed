# Undo Tree Panel

A GPUI sidebar panel that visualises the full branch-preserving undo history of the active editor buffer. The equivalent of Neovim's `undotree` plugin or Emacs's `undo-tree-mode`.

Zed's CRDT already preserves every branch of edit history, but there is no built-in way to *see* the tree or jump directly to an arbitrary node. This crate adds the visual panel, and also introduces `g-`/`g+` (Vim) for chronological timeline walking across branches.

## How it works

The panel is stateless: it stores only display-layer data (a flat list of `DisplayRow`s and the selected index). The source of truth is always the buffer's history. On every relevant editor event (edit, undo, redo), it snapshots the history and rebuilds the display.

There are two rendering modes:

- **Singleton buffers** get the full branching tree view. The panel calls `text::Buffer::undo_tree_snapshot()` to clone the `UndoTree` from `History`, then flattens it into display rows. Navigation dispatches `language::Buffer::goto_transaction`.

- **Multibuffers** (search results, project-wide find-replace) get a linear undo/redo timeline via `MultiBuffer::undo_history_snapshot()`, since multibuffer history has no tree structure. Navigation issues repeated `undo()`/`redo()` calls to reach the target depth.

## Tree visualisation

The flattening algorithm produces Unicode box-drawing output using the buffer's monospace font:

```
○  initial (3/7)
│
○  hello world · 3s ago
├─╮
│ ○  brave · 5s ago
│
@  delete · just now
│
~  (3 hidden)
│
○  refactor · 12s ago
```

- `○` regular node, `@` current node, `~` collapsed chain (≥4 single-child nodes folded)
- Active path uses `text` colour, inactive branches use `text_muted`, current node uses `text_accent`
- Folds never hide the current node

Node labels are extracted from CRDT operations: transaction edit text is whitespace-normalised, truncated at 24 chars, and shown with a relative timestamp. Multibuffer labels additionally include affected file names.

## Event subscription

| Source          | Event                                             | Response        |
| --------------- | ------------------------------------------------- | --------------- |
| `Workspace`     | `ActiveItemChanged`                               | Swap editor ref |
| Active `Editor` | `Edited`, `TransactionUndone`, `TransactionBegun` | Rebuild display |

## Future work

- Keyboard navigation (`j`/`k`, `Enter`, `[`/`]` for chrono walk)
- Cherry-pick and drop (applying or reversing a specific node's edit)
- Preview mode (show a node's buffer state without navigating to it)
- Diff preview per node
- Persistence across editor restarts