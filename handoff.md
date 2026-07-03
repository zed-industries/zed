# Handoff: Git Graph toggleable columns review (PR #59850)

## Context
- Upstream PR: https://github.com/zed-industries/zed/pull/59850 ("git_ui: make git
  graph columns toggleable", by @RemcoSmitsDev).
- Review/iteration branch: `git-graph-columns-review`, based on the PR head
  `98ce42e663`. Follow-up commit: `3af0fb8a1c`.
- This branch addresses the review feedback **except** the resize-with-hidden-columns
  bug fix, which is intentionally left for the next agent (see "TODO" below). Two
  tests already encode the expected behavior and are **failing on purpose**.

## Build / test
- Build: `cargo check -p ui -p git_ui`
- Tests: `cargo test -p ui data_table::tests::`
  - Expected today: `31 passed; 2 failed`. The 2 failures are the intentional specs:
    - `drag_with_hidden_columns::drag_boundary_follows_cursor_with_hidden_column`
    - `drag_with_hidden_columns::drag_does_not_resize_hidden_neighbor`
- Lint: `./script/clippy -p ui -p git_ui` (clean as of `3af0fb8a1c`).

## What is DONE on this branch (commit 3af0fb8a1c)
1. **Column visibility mask moved out of the resize state.** It now lives on the view
   as `GitGraph.column_visibility: TableRow<bool>`
   (`crates/git_ui/src/git_graph.rs`), not on `RedistributableColumnsState`. Filtering
   no longer requires a resizable table.
2. **Redistribution deduplicated** into shared helpers in
   `crates/ui/src/components/redistributable_columns.rs`:
   - `redistribute_hidden_widths(widths, hidden)`
   - `redistribute_hidden_fractions(fractions, hidden)`
   - `is_column_hidden(hidden, idx)`
   `RedistributableColumnsState::widths_to_render` / `preview_fractions` are back to
   their pre-PR shape (no embedded filtering).
3. **Hidden columns are not rendered** (cells/headers skipped via `column_is_visible`
   in `data_table.rs`) and **keep their real stored width** instead of a fabricated
   `0.0`, so re-showing a column restores its previous size. Width state is never
   mutated by filtering.
4. **Last-visible-column menu entry is disabled** (not a silent no-op) via new
   `ContextMenu::toggleable_entry_disabled_when`
   (`crates/ui/src/components/context_menu.rs`), used in
   `GitGraph::deploy_header_context_menu`.
5. **Panic-safe header slice**: the `table_filter` slice now uses a checked
   `get(..)` with a fallback (`git_graph.rs`).
6. **Test seam**: extracted a behavior-preserving pure `compute_drag_preview` from
   `RedistributableColumnsState::on_drag_move` so the drag math is unit-testable.

## TODO (next agent — the actual fix)
### 1. Make column resizing redistribution/visibility-aware  (PRIMARY)
The two failing tests specify the bug. Root cause: the resize dividers are laid out
from the **redistributed** (visible-only) widths
(`redistribute_hidden_widths(widths_to_render(), hidden)`), but the drag math in
`RedistributableColumnsState::compute_drag_preview` / `on_drag_move`
(`crates/ui/src/components/redistributable_columns.rs`) still works on the **raw
committed widths** and is unaware of which columns are hidden. Two symptoms:
- **Position mismatch**: `col_position` is summed from un-redistributed widths, so the
  cursor->boundary mapping is wrong once any column is hidden.
  (Test: `drag_boundary_follows_cursor_with_hidden_column`.)
- **Propagation through a hidden neighbor**: `drag_column_handle` pushes the diff to
  `col_idx + 1` in *original* index space; if that neighbor is hidden it should skip
  to the next *visible* column.
  (Test: `drag_does_not_resize_hidden_neighbor`.)

Suggested approach:
- Thread the `hidden` mask into `compute_drag_preview` (and
  `drag_column_handle` / `propagate_resize_diff` as needed) and do the geometry in the
  redistributed/visible-column space; skip hidden columns for neighbor propagation.
- IMPORTANT plumbing note: the mask now lives on `GitGraph`, not on
  `RedistributableColumnsState`, so `on_drag_move` (a method on the state, invoked via
  `bind_redistributable_columns`) does NOT currently receive it. You'll need to get the
  mask into the drag path — e.g. pass it through `bind_redistributable_columns` /
  `render_redistributable_columns_resize_handles` (git_graph already passes
  `Some(&self.column_visibility)` to the latter) or via the drag payload.
- Update the two tests to pass the `hidden` mask into `compute_drag_preview` (they
  currently call the pre-fix signature and assert the *desired* result); they should
  go green once the math is visibility-aware.

### 2. (Deferred) Persistence of column visibility
Intentionally skipped. Can be added later as a settings-backed bitmap; the mask is a
`TableRow<bool>` on `GitGraph`, so serializing/restoring it is straightforward.

## Key files
- `crates/git_ui/src/git_graph.rs` — `column_visibility` field, `toggle_column_visibility`,
  `deploy_header_context_menu`, `preview_column_fractions`, `graph_viewport_width`,
  header render + resize-handle wiring.
- `crates/ui/src/components/redistributable_columns.rs` — redistribution helpers,
  `compute_drag_preview`, `on_drag_move`, `render_redistributable_columns_resize_handles`.
- `crates/ui/src/components/data_table.rs` — `column_filter` wiring, `column_is_visible`.
- `crates/ui/src/components/context_menu.rs` — `toggleable_entry_disabled_when`.
- `crates/ui/src/components/data_table/tests.rs` — `mod drag_with_hidden_columns`
  (failing specs) and `mod column_filter` (helper tests).
