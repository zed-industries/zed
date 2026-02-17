# Edit Prediction Tabstop Selections

## Status: ✅ COMPLETED (with Tab navigation fix)

All implementation steps have been completed and tested. The feature is ready for review.

## Problem
Zeta 2 model outputs `<|selection_start|>`/`<|user_cursor|>` marker pairs to indicate
uncertain regions in predictions (e.g. guessed variable names). Currently these markers leak
through as literal text in the editor because they're not parsed.

## Goal
Parse these markers into selection ranges and surface them as snippet-style tabstops that the
user can Tab through after accepting a prediction.

## Example model output
```
for <|selection_start|>item<|user_cursor|> in <|selection_start|>collection<|user_cursor|> {
    <|user_cursor|>
}
```
This should produce 3 tabstops: "item" (selected), "collection" (selected), empty cursor in body.

## Implementation Plan

### ✅ 1. Add `SELECTION_START_MARKER` to `zeta_prompt`
- File: `crates/zeta_prompt/src/zeta_prompt.rs`
- Added `pub const SELECTION_START_MARKER: &str = "<|selection_start|>";` next to `CURSOR_MARKER`

### ✅ 2. Add `PredictedSelection` to `edit_prediction_types`
- File: `crates/edit_prediction_types/src/edit_prediction_types.rs`
- Added `PredictedSelection` struct with `start: PredictedCursorPosition, end: PredictedCursorPosition`
- Added `tabstop_selections: Vec<PredictedSelection>` to `EditPrediction::Local`

### ✅ 3. Extract marker ranges in `zeta2.rs`
- File: `crates/edit_prediction/src/zeta2.rs`
- Implemented `extract_selections_and_cursor()` function
- Returns `(stripped_text, Vec<Range<usize>>, Option<usize>)` — selections + first cursor offset
- Added 10 unit tests for marker extraction

### ✅ 4. Map selection ranges through the diff to `PredictedSelection`s
- File: `crates/edit_prediction/src/zeta2.rs`
- Implemented `map_offset_in_new_text_to_predicted_position()` helper
- Implemented `compute_edits_cursor_and_selections()` that combines cursor and selection mapping

### ✅ 5. Thread selections through the prediction pipeline
- `crates/edit_prediction/src/prediction.rs`: Added `tabstop_selections` to `EditPrediction` struct and `EditPredictionResult::new`
- `crates/edit_prediction/src/zed_edit_prediction_delegate.rs`: Passes through `tabstop_selections`
- Updated all other providers (mercury, ollama, sweep_ai, zeta1, copilot) with empty `tabstop_selections`
- Updated test files with new field

### ✅ 6. Accept with tabstops in the editor
- File: `crates/editor/src/editor.rs`
- Added `tabstop_selections: Vec<(Anchor, usize, Anchor, usize)>` to `EditPrediction::Edit`
- Updated `update_visible_edit_prediction()` to map `PredictedSelection`s to editor anchors
- Updated `accept_partial_edit_prediction()` for `Full` granularity:
  - Builds tabstop ranges after applying edits
  - Selects the first tabstop if present
  - Pushes `SnippetState` onto `snippet_stack` for Tab/Shift-Tab navigation

## Files Modified
- `crates/zeta_prompt/src/zeta_prompt.rs` - Added SELECTION_START_MARKER constant
- `crates/edit_prediction_types/src/edit_prediction_types.rs` - Added PredictedSelection struct and tabstop_selections field
- `crates/edit_prediction/src/zeta2.rs` - Core marker extraction and mapping logic + tests
- `crates/edit_prediction/src/prediction.rs` - Added tabstop_selections to EditPrediction
- `crates/edit_prediction/src/zed_edit_prediction_delegate.rs` - Thread through tabstop_selections
- `crates/edit_prediction/src/mercury.rs` - Empty tabstop_selections
- `crates/edit_prediction/src/ollama.rs` - Empty tabstop_selections
- `crates/edit_prediction/src/sweep_ai.rs` - Empty tabstop_selections
- `crates/edit_prediction/src/zeta1.rs` - Empty tabstop_selections
- `crates/edit_prediction/src/edit_prediction_tests.rs` - Updated test struct
- `crates/copilot/src/copilot_edit_prediction_delegate.rs` - Empty tabstop_selections
- `crates/editor/src/editor.rs` - Editor-side tabstop handling + exit tabstop fix
- `crates/editor/src/edit_prediction_tests.rs` - Added tabstop_selections to test helpers
- `crates/editor/src/editor_tests.rs` - Added tabstop_selections to test helpers

## Tests
All 89 tests in edit_prediction pass, including 10 new tests for `extract_selections_and_cursor`:
- `test_extract_selections_and_cursor_basic`
- `test_extract_selections_and_cursor_multiple_selections`
- `test_extract_selections_and_cursor_with_standalone_cursor`
- `test_extract_selections_and_cursor_only_cursor`
- `test_extract_selections_and_cursor_no_markers`
- `test_extract_selections_and_cursor_backwards_pair`
- `test_extract_selections_and_cursor_orphaned_selection_start`
- `test_extract_selections_and_cursor_multiple_standalone_cursors`
- `test_extract_selections_full_example`

## Tab Navigation Fix

**Bug**: After accepting a prediction with tabstop selections, pressing Tab indented the line
instead of navigating to the next tabstop or collapsing the selection.

**Root cause**: The `tabstop_ranges` only contained the selection tabstops (e.g., `["multiply"]`)
but no final "exit" tabstop. With only 1 range at `active_index: 0`,
`move_to_snippet_tabstop(Bias::Right)` checked `0 + 1 < 1` → false, pushed the snippet back,
and returned false. `tab()` then fell through to indentation (non-empty selection → indent).

**Fix**: Added a final exit tabstop at `cursor_target` (the predicted cursor position) to the
end of `tabstop_ranges`, mirroring how regular snippets use `$0`. Now Tab on the last selection
advances to the exit tabstop (collapsing the cursor), and the snippet is done.

## Key references
- Snippet tabstop push: `editor.rs` `insert_snippet` L10483 → `self.snippet_stack.push(SnippetState { ... })`
- Tab/shift-tab navigation: `move_to_snippet_tabstop` L10571
- Marker extraction reference: `edit_prediction_cli/src/format_prompt.rs` `extract_selections` L296
- Cursor position resolution after edit: `editor.rs` L8008-8015
- `PredictedCursorPosition` resolve pattern: anchor.to_offset + offset