# Diff Review Button Visual Test Fixes

This document describes the issues with the diff review button visual tests and provides detailed guidance on how to fix them.

## Background

The diff review button is a "+" button that appears in the gutter of the editor when viewing uncommitted changes in the Project Diff view. When clicked, it allows users to add a review. The button should:

1. **Normal state**: Show a white "+" icon on a dark gray rounded rectangle background
2. **Hover state**: Same as normal, but with a subtle light border around the button
3. **Tooltip**: Show "Add Review" tooltip when hovered

### Relevant Files

- `crates/editor/src/element.rs` - Contains `layout_diff_review_button()` which creates the button
- `crates/zed/src/visual_test_runner.rs` - Contains `run_diff_review_visual_tests()` which runs the visual tests
- `crates/gpui/src/window.rs` - Contains `simulate_mouse_move()` for hover simulation
- `target/visual_tests/` - Output directory for test screenshots

### Reference Mockup

The button should look like the mockup where:
- The "+" button has a dark gray/charcoal background with rounded corners
- The icon is white
- On hover, a subtle white/light border appears around the button
- The "Add Review" tooltip appears below the button when hovering

---

## Current Issues

### Issue 1: `diff_review_button_enabled.png` Shows Empty State

**What's happening**: The screenshot shows "No uncommitted changes" and "Remote up to date" instead of the actual diff view with code and the "+" button.

**Root cause**: The test is likely capturing the screenshot before the diff content has fully loaded, or the git repository setup in the test isn't creating actual uncommitted changes that the diff view can detect.

**Location**: `crates/zed/src/visual_test_runner.rs` in `run_diff_review_visual_tests()`

**How to fix**:
1. Check that the test properly creates uncommitted changes:
   - The test creates a file, commits it, then modifies it
   - Ensure the modification happens AFTER the commit
   - Add longer wait times for git status detection

2. Verify the ProjectDiff view is actually showing content:
   ```rust
   // After deploying ProjectDiff, wait longer for it to detect changes
   cx.background_executor()
       .timer(std::time::Duration::from_millis(1000))  // Increase from 500ms
       .await;
   ```

3. Add verification that the diff view has content before taking screenshot:
   ```rust
   // Could add a check like:
   workspace_window.update(cx, |workspace, window, cx| {
       // Verify the editor has content
   })?;
   ```

### Issue 2: "+" Button Not Visible in Screenshots

**What's happening**: In `diff_review_button_tooltip.png`, there's a gray rectangle in the gutter on line 4, but no "+" icon is visible inside it.

**Root cause**: The button is being rendered but the icon might not be showing due to:
1. Icon size/color issues with the current styling
2. The wrapper div might be obscuring the IconButton
3. z-index or layering issues

**Location**: `crates/editor/src/element.rs` around line 3060-3085

**Current code structure**:
```rust
let button = IconButton::new("diff_review_button", IconName::Plus)
    .icon_size(IconSize::XSmall)
    .size(ui::ButtonSize::Compact)
    .style(ButtonStyle::Subtle)
    .tooltip(Tooltip::text("Add Review"))
    .into_any_element();

let button = div()
    .rounded_sm()
    .bg(element_bg)
    .border_1()
    .border_color(gpui::transparent_black())
    .hover(move |style| style.border_color(border_color))
    .child(button)
    .into_any_element();
```

**How to fix**:
1. The wrapper div might need explicit sizing to not collapse:
   ```rust
   let button = div()
       .flex()
       .items_center()
       .justify_center()
       // ... rest of styling
   ```

2. Check if `ButtonStyle::Subtle` has a transparent background that's being overridden improperly by the wrapper

3. Consider using a single element approach instead of wrapping:
   ```rust
   // Instead of wrapping, customize the IconButton directly
   let button = IconButton::new("diff_review_button", IconName::Plus)
       .icon_size(IconSize::XSmall)
       .size(ui::ButtonSize::Compact)
       .style(ButtonStyle::Filled)  // Use Filled for visible background
       .tooltip(Tooltip::text("Add Review"));
   ```

4. If using the wrapper approach, ensure the IconButton isn't also setting its own background that conflicts

### Issue 3: Tooltip Not Appearing

**What's happening**: The "Add Review" tooltip is not visible in the tooltip test screenshot.

**Root cause**: Elements rendered via `prepaint_as_root()` (used by `prepaint_gutter_button()`) don't fully participate in the hover/tooltip system during visual tests. The tooltip delay timer may not be advancing properly.

**Location**: 
- `crates/editor/src/element.rs` - `prepaint_gutter_button()` function
- `crates/zed/src/visual_test_runner.rs` - tooltip test code

**How to fix**:

1. **Investigate prepaint_as_root behavior**: The `prepaint_gutter_button()` function uses `button.prepaint_as_root()` which positions elements outside the normal layout flow. This may bypass tooltip registration.

2. **Check if tooltip works in real app**: Before spending more time on the test, verify the tooltip actually works when running Zed normally. If it works there, the issue is test-specific.

3. **Alternative testing approach**: Instead of trying to capture the tooltip visually, consider:
   - Testing that the tooltip is configured (unit test)
   - Manual verification that tooltip works in real app
   - Document that visual tooltip testing is a known limitation

4. **Timer advancement**: Ensure the GPUI background executor is properly advancing timers:
   ```rust
   // The tooltip has a 500ms delay (TOOLTIP_SHOW_DELAY in div.rs)
   // Make sure we're waiting long enough and the timer is advancing
   cx.background_executor()
       .timer(std::time::Duration::from_millis(700))
       .await;
   ```

### Issue 4: Hover Border Not Showing

**What's happening**: The button should show a subtle border when hovered, but it doesn't appear in screenshots.

**Root cause**: Same as tooltip issue - hover state changes aren't being triggered properly for elements rendered via `prepaint_as_root()`.

**Location**: `crates/editor/src/element.rs` in the button styling code

**How to fix**:
1. Verify hover works in real app first
2. The `.hover()` modifier on the wrapper div may not work for prepainted elements
3. Consider if hover styling should be on the IconButton itself rather than a wrapper

### Issue 5: `diff_review_button_disabled.png` Shows Button

**What's happening**: The disabled test screenshot looks identical to the tooltip screenshot, suggesting the button is visible when it should be hidden (feature flag disabled).

**Root cause**: Either:
1. The feature flag isn't being properly disabled between tests
2. The screenshots are being taken in the wrong order
3. The window isn't being refreshed after flag change

**Location**: `crates/zed/src/visual_test_runner.rs`

**How to fix**:
1. Verify flag is actually disabled:
   ```rust
   cx.update(|cx| {
       cx.update_flags(false, vec![]);  // This disables all flags
   })?;
   ```

2. Add a window refresh and wait after changing flags:
   ```rust
   cx.update_window(workspace_window.into(), |_view, window: &mut Window, _cx| {
       window.refresh();
   })?;
   
   cx.background_executor()
       .timer(std::time::Duration::from_millis(300))
       .await;
   ```

3. Consider creating a fresh window for the disabled test rather than reusing

### Issue 6: Test Content Changed

**What's happening**: The file content in screenshots shows different text than the original working screenshots (e.g., "// Modified content with changes" with word-diff highlighting).

**Root cause**: The test file content was modified, or there's inconsistency in how the test files are being created.

**Location**: `crates/zed/src/visual_test_runner.rs` in the test file creation code

**How to fix**:
1. Review the test file content:
   ```rust
   // Original content that gets committed
   let original_content = "// Original content\n";
   
   // Modified content that creates the diff
   let modified_content = r#"import { ScrollArea } from 'components';
   import { ButtonAlt, Tooltip } from 'ui';
   import { Message, FileEdit } from 'types';
   import { AiPaneTabContext } from 'context';
   "#;
   ```

2. Ensure the content matches what you expect to see in the screenshots

3. The word-diff highlighting (red/green backgrounds on specific words) suggests inline diff mode is enabled - verify this is the intended display mode

---

## Testing Strategy

### Step 1: Verify Real App Behavior
Before fixing tests, manually verify in the real Zed app:
1. Open a project with uncommitted changes
2. Open the Project Diff view (Uncommitted Changes tab)
3. Verify the "+" button appears on the last line
4. Hover over it - verify tooltip appears and border shows
5. Click it - verify intended action occurs

### Step 2: Fix Test Setup
1. Ensure git repo is properly created with real uncommitted changes
2. Add sufficient wait times for async operations
3. Verify diff view is populated before taking screenshots

### Step 3: Fix Button Rendering
1. Simplify the button code - try without the wrapper div first
2. If wrapper is needed, ensure proper sizing and layout
3. Test icon visibility separately from hover/tooltip

### Step 4: Document Limitations
If tooltip/hover testing proves too difficult due to `prepaint_as_root()` limitations:
1. Document this as a known limitation
2. Add unit tests for tooltip configuration
3. Rely on manual testing for visual hover effects

---

## Code Snippets for Reference

### The Feature Flag Check
```rust
// In layout_diff_review_button()
if !cx.has_flag::<DiffReviewFeatureFlag>() {
    return None;
}

let show_diff_review_button = self.editor.read(cx).show_diff_review_button();
if !show_diff_review_button {
    return None;
}
```

### The Button Positioning
```rust
// The button is placed at the last row of the document
let max_point = snapshot.display_snapshot.max_point();
let last_row = max_point.row();

// Then positioned via prepaint_gutter_button()
let button = prepaint_gutter_button(
    button,
    last_row,
    line_height,
    gutter_dimensions,
    scroll_position,
    gutter_hitbox,
    display_hunks,
    window,
    cx,
);
```

### Mouse Simulation for Hover
```rust
// In Window (crates/gpui/src/window.rs)
#[cfg(any(test, feature = "test-support"))]
pub fn simulate_mouse_move(&mut self, position: Point<Pixels>, cx: &mut App) {
    let event = PlatformInput::MouseMove(MouseMoveEvent {
        position,
        modifiers: self.modifiers,
        pressed_button: None,
    });
    let _ = self.dispatch_event(event, cx);
}
```

---

## Debugging Tips

1. **Add println! statements** in the visual test runner to verify state at each step

2. **Check the worktree errors** - The test output shows errors like "error scanning directory" which may indicate the temp directory is being cleaned up too early

3. **Increase timeouts** - Many async operations may need more time to complete

4. **Simplify first** - Get the basic "enabled" screenshot working before tackling hover/tooltip

5. **Compare with working tests** - Look at how `run_agent_thread_view_test()` works since it successfully captures UI

---

## Success Criteria

When fixed, the screenshots should show:

1. **diff_review_button_enabled.png**: 
   - Full diff view with file header (thread-view.tsx)
   - Code content with diff highlighting
   - "+" button visible in gutter on last row with dark background and white icon

2. **diff_review_button_tooltip.png**:
   - Same as enabled, plus:
   - Subtle border around the "+" button (hover state)
   - "Add Review" tooltip visible near the button

3. **diff_review_button_disabled.png**:
   - Same diff view content
   - NO "+" button visible (feature flag disabled)

4. **diff_review_button_regular_editor.png**:
   - Regular editor (not diff view)
   - NO "+" button visible (only shows in diff view)