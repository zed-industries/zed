# Diff Review Button - Remaining Work

This document describes the remaining work needed to complete the diff review button implementation.

## Background

The diff review button is a "+" button that appears in the gutter of the editor when viewing uncommitted changes in the Project Diff view. When hovered, it should show an "Add Review" tooltip. The button should behave like the breakpoint phantom indicator - appearing when the user hovers over the gutter.

## Current State (Completed)

The following items have been completed:

- ✅ Button appears in the diff view when the `diff-review` feature flag is enabled
- ✅ Button is hidden when the feature flag is disabled
- ✅ Button is hidden in regular editors (only shows in diff view)
- ✅ Button uses SVG `IconName::Plus` icon via `IconButton` (not text "+")
- ✅ Tooltip shows "Add Review" when hovering over the button
- ✅ Visual tests pass for enabled, disabled, tooltip, and regular editor cases
- ✅ Button positioned at left edge of gutter to avoid overlap with expand excerpt button

## Remaining Work

### Issue 1: Button Should Only Appear on Hover (Like Breakpoints)

**Current behavior**: The diff review button is always visible on the last row of the document.

**Expected behavior**: The button should only appear when the user hovers over the gutter, appearing on whichever row they're hovering over (just like the breakpoint phantom indicator).

**Reference implementation**: The breakpoint phantom indicator in `crates/editor/src/editor.rs`:
- `PhantomBreakpointIndicator` struct (line ~1016) tracks the hovered row and active state
- `gutter_breakpoint_indicator` field on `Editor` stores the indicator state
- `mouse_moved()` in `crates/editor/src/element.rs` (line ~1269) handles hover detection and creates the indicator
- There's a 200ms debounce before showing the indicator

**Implementation approach**:
1. Create a similar `PhantomDiffReviewIndicator` struct to track hover state
2. Add a `gutter_diff_review_indicator` field to `Editor`
3. Update `mouse_moved()` to detect gutter hover and create the indicator (when `show_diff_review_button` is true)
4. Update `layout_diff_review_button()` to use the indicator's row instead of `max_point.row()`

### Issue 2: Don't Show Button on Rows with Expand Excerpt Buttons

**Current behavior**: The diff review button can appear on the same row as an expand excerpt button, causing visual overlap.

**Expected behavior**: If a row has an expand excerpt button (indicated by `row_info.expand_info.is_some()`), don't show the diff review button on that row. Users can expand the excerpt first, then the diff review button will be available.

**Reference implementation**: The breakpoint code already does this check in `layout_breakpoints()` (line ~3007):
```rust
if row_infos
    .get((display_row.0.saturating_sub(range.start.0)) as usize)
    .is_some_and(|row_info| {
        row_info.expand_info.is_some()
            || row_info
                .diff_status
                .is_some_and(|status| status.is_deleted())
    })
{
    return None;
}
```

**Implementation approach**:
1. Pass `row_infos` to the diff review button layout function
2. Check if the hovered row has `expand_info` before showing the button
3. If `expand_info.is_some()`, don't show the diff review button

### Issue 3: Respect "Disable AI" Setting

**Current behavior**: The diff review button appears regardless of the "disable AI" setting.

**Expected behavior**: When `DisableAiSettings::get_global(cx).disable_ai` is `true`, the diff review button should never appear. The normal breakpoint hover behavior should work instead.

**Reference implementation**: See how other AI features check this setting:
- `crates/agent_ui/src/inline_assistant.rs` line ~61
- `crates/edit_prediction_ui/src/edit_prediction_button.rs` line ~90

**Implementation approach**:
1. In `layout_diff_review_button()`, check `DisableAiSettings::get_global(cx).disable_ai`
2. If AI is disabled, return `None` immediately
3. Add `project::DisableAiSettings` to the imports in `element.rs`

## Required Tests

### Unit Tests

Create unit tests in `crates/editor/src/editor_tests.rs` (or a new test file):

1. **`test_diff_review_button_shown_on_gutter_hover_without_expand`**
   - Set up a diff view with `show_diff_review_button = true`
   - Simulate hovering over a gutter row that does NOT have an expand button
   - Verify the diff review indicator is created for that row

2. **`test_diff_review_button_hidden_on_gutter_hover_with_expand`**
   - Set up a diff view with `show_diff_review_button = true`
   - Simulate hovering over a gutter row that HAS an expand button
   - Verify the diff review indicator is NOT created

3. **`test_diff_review_button_hidden_when_ai_disabled`**
   - Set `DisableAiSettings { disable_ai: true }`
   - Set up a diff view with `show_diff_review_button = true`
   - Simulate hovering over the gutter
   - Verify the diff review indicator is NOT created
   - Verify normal breakpoint indicator behavior still works

4. **`test_diff_review_button_shown_when_ai_enabled`**
   - Set `DisableAiSettings { disable_ai: false }`
   - Set up a diff view with `show_diff_review_button = true`
   - Simulate hovering over the gutter
   - Verify the diff review indicator IS created

### Visual Tests

Update visual tests in `crates/zed/src/visual_test_runner.rs`:

1. **`diff_review_button_hover_row_1`** (rename from `diff_review_button_enabled`)
   - Hover over line 1 in the gutter
   - Verify the "+" button appears on line 1

2. **`diff_review_button_hover_row_3`** (new test)
   - Hover over line 3 in the gutter
   - Verify the "+" button appears on line 3 (different from row 1)

3. **`diff_review_button_tooltip`** (keep existing)
   - Hover over the "+" button
   - Verify "Add Review" tooltip appears

4. **`diff_review_button_hidden_on_expand_row`** (new test)
   - Set up a diff where a row has an expand button
   - Hover over that row in the gutter
   - Verify the "+" button does NOT appear (only expand button visible)

5. **`diff_review_button_disabled`** (keep existing)
   - Feature flag disabled
   - Verify no button appears

6. **`diff_review_button_regular_editor`** (keep existing)
   - Regular editor (not diff view)
   - Verify no button appears

**Note**: Visual tests are NOT needed for the "disable AI" setting - unit tests are sufficient for that.

## Relevant Files

| File | Purpose |
|------|---------|
| `crates/editor/src/editor.rs` | Contains `PhantomBreakpointIndicator` as reference, and `Editor` struct |
| `crates/editor/src/element.rs` | Contains `layout_diff_review_button()`, `mouse_moved()`, and gutter rendering |
| `crates/editor/src/editor_tests.rs` | Unit tests for editor functionality |
| `crates/zed/src/visual_test_runner.rs` | Visual test infrastructure |
| `crates/project/src/project.rs` | Contains `DisableAiSettings` |
| `crates/multi_buffer/src/multi_buffer.rs` | Contains `RowInfo` and `expand_info` |

## Running Tests

### Unit Tests
```bash
cargo -q test --package editor test_diff_review
```

### Visual Tests
```bash
cargo -q run --package zed --bin zed_visual_test_runner --features="visual-tests"
```

### Update Visual Test Baselines
```bash
UPDATE_BASELINE=1 cargo -q run --package zed --bin zed_visual_test_runner --features="visual-tests"
```

## Implementation Order

1. **Phase 1**: Implement hover-based display (Issue 1)
   - Create `PhantomDiffReviewIndicator`
   - Update `mouse_moved()` to detect gutter hover for diff review
   - Update `layout_diff_review_button()` to use indicator row
   - Add visual tests for hovering different rows

2. **Phase 2**: Hide on expand rows (Issue 2)
   - Add `expand_info` check to diff review button layout
   - Add visual test for expand row behavior

3. **Phase 3**: Respect "Disable AI" setting (Issue 3)
   - Add `DisableAiSettings` check
   - Add unit tests for AI disabled/enabled states

4. **Phase 4**: Final cleanup
   - Remove any debug logging
   - Update all baselines
   - Verify all tests pass