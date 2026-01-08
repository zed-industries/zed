# Diff Review Button - Remaining Work

This document describes the remaining work needed to match the diff review button implementation to the mockup design.

## Background

The diff review button is a "+" button that appears in the gutter of the editor when viewing uncommitted changes in the Project Diff view. When hovered, it should show an "Add Review" tooltip.

### Current State

The button is now functional and visible in the visual tests:
- ✅ Button appears in the diff view when the `diff-review` feature flag is enabled
- ✅ Button is hidden when the feature flag is disabled
- ✅ Button is hidden in regular editors (only shows in diff view)
- ⚠️ Button uses text "+" instead of SVG icon (workaround for rendering limitation)
- ⚠️ Tooltip test doesn't show the tooltip (mouse position needs adjustment)

### Mockup Reference

The mockup shows:
- A "+" button with a dark gray/charcoal rounded rectangle background
- A white "+" icon inside
- On hover: a subtle light border appears around the button
- Tooltip: "Add Review" appears below the button when hovering

## Relevant Files

| File | Purpose |
|------|---------|
| `crates/editor/src/element.rs` | Contains `layout_diff_review_button()` which creates the button |
| `crates/zed/src/visual_test_runner.rs` | Contains `run_diff_review_visual_tests()` which runs the visual tests |
| `crates/zed/test_fixtures/visual_tests/` | Baseline images for visual tests |
| `target/visual_tests/` | Generated test screenshots (created when tests run) |

## Running the Visual Tests

### Basic Test Run

```bash
cd /path/to/zed
cargo -q run --package zed --bin zed_visual_test_runner --features="visual-tests"
```

This will:
1. Run all visual tests
2. Save screenshots to `target/visual_tests/`
3. Compare against baselines in `crates/zed/test_fixtures/visual_tests/`
4. Report pass/fail status

### Updating Baselines

When you've made intentional visual changes:

```bash
UPDATE_BASELINE=1 cargo -q run --package zed --bin zed_visual_test_runner --features="visual-tests"
```

This will update the baseline images to match the current output.

### Viewing Screenshots

After running tests, view the generated screenshots:

```bash
open target/visual_tests/diff_review_button_enabled.png
open target/visual_tests/diff_review_button_tooltip.png
open target/visual_tests/diff_review_button_disabled.png
open target/visual_tests/diff_review_button_regular_editor.png
```

Compare these against the mockup to identify differences.

## Remaining Issues

### Issue 1: Button Uses Text "+" Instead of SVG Icon

**Current behavior**: The button displays a text "+" character.

**Expected behavior**: The button should display the `IconName::Plus` SVG icon (same as mockup).

**Root cause**: SVG icons don't render when elements are painted via `prepaint_as_root()`. This is a limitation of how the editor paints gutter elements outside the normal element tree.

**Location**: `crates/editor/src/element.rs`, function `layout_diff_review_button()`, around line 3058-3080.

**Current code**:
```rust
let button = div()
    .id("diff_review_button")
    .flex()
    .items_center()
    .justify_center()
    .w(px(20.0))
    .h(px(20.0))
    .rounded(px(4.0))
    .bg(cx.theme().colors().element_background)
    .text_color(cx.theme().colors().text)
    .text_size(px(14.0))
    .font_weight(gpui::FontWeight::BOLD)
    .child("+")
    .tooltip(|window, cx| ui::Tooltip::text("Add Review")(window, cx))
    .into_any_element();
```

**Investigation needed**: 
1. Understand why SVG icons don't render in `prepaint_as_root()` context
2. Look at how breakpoint indicators render their SVG icons (they use the same `prepaint_gutter_button()` mechanism)
3. The breakpoint code is in `crates/editor/src/editor.rs`, function `render_breakpoint()` around line 8590

**Possible solutions**:
1. Find what's different between how breakpoints render icons vs our approach
2. Consider if there's a different rendering approach that would work
3. As a fallback, the text "+" is acceptable but not ideal

### Issue 2: Tooltip Not Appearing in Tests

**Current behavior**: The `diff_review_button_tooltip.png` screenshot looks identical to `diff_review_button_enabled.png` - no tooltip visible.

**Expected behavior**: The tooltip "Add Review" should be visible in the screenshot.

**Root cause**: The test hovers the mouse at a hardcoded position that may not match where the button actually is.

**Location**: `crates/zed/src/visual_test_runner.rs`, around line 1051.

**Current code**:
```rust
// Test 1b: Tooltip visible when hovering over the button
// The button is positioned in the gutter at the last row of the diff
let button_position = point(px(27.0), px(232.0));

// Simulate mouse move to hover over the button
cx.update_window(workspace_window.into(), |_, window, cx| {
    window.simulate_mouse_move(button_position, cx);
})?;
```

**How tooltips work**:
1. The button has `.tooltip(...)` which registers a tooltip builder
2. During `paint()`, `Interactivity::paint_mouse_listeners()` registers mouse event handlers
3. When mouse hovers over the button's hitbox, a timer starts
4. After `TOOLTIP_SHOW_DELAY` (500ms), the tooltip appears

**How to fix**:
1. Calculate the actual button position based on the layout:
   - Window size: 600x500 (see `window_size` in the test)
   - The button is on the last row of visible diff content (line 4)
   - Button is in the gutter area (left side, around x=10-30)
   - Y position depends on: title bar + tab bar + file header + line heights

2. Or, add debugging to print the button's actual position:
   ```rust
   // In layout_diff_review_button, after prepaint_gutter_button:
   eprintln!("Button position: {:?}", /* get position from hitbox */);
   ```

3. Update the hardcoded `button_position` to match the actual position

**Verification**: After fixing, the `diff_review_button_tooltip.png` should show:
- The same diff view as `enabled.png`
- Plus a tooltip near the button saying "Add Review"

### Issue 3: Button Styling Refinements

**Current vs Mockup differences**:

| Aspect | Current | Mockup |
|--------|---------|--------|
| Background | `element_background` (subtle gray) | Darker charcoal |
| Icon | Text "+" | SVG Plus icon |
| Hover state | No visible change | Light border appears |
| Size | 20x20px | Similar |
| Corners | 4px radius | Similar |

**To match the mockup more closely**:

1. **Darker background**: Try using a different theme color or a custom color:
   ```rust
   // Option 1: Use a darker theme color
   .bg(cx.theme().colors().surface_background)
   
   // Option 2: Use a custom darker color
   .bg(gpui::hsla(220.0/360.0, 0.1, 0.15, 1.0))
   ```

2. **Hover state with border**:
   ```rust
   let border_color = cx.theme().colors().border;
   
   div()
       // ... existing styles ...
       .border_1()
       .border_color(gpui::transparent_black())
       .hover(move |style| style.border_color(border_color))
   ```

## How to Approach the Work

### Step 1: Set Up Your Environment

1. Clone the repo and checkout the `diff-review-button` branch
2. Ensure you can build: `cargo build --package zed`
3. Run the visual tests to confirm they pass: `cargo -q run --package zed --bin zed_visual_test_runner --features="visual-tests"`

### Step 2: Understand the Current Implementation

1. Read `layout_diff_review_button()` in `crates/editor/src/element.rs`
2. Read how breakpoints render their icons in `render_breakpoint()` in `crates/editor/src/editor.rs`
3. Run the app and open a project with uncommitted changes to see the button in action

### Step 3: Fix the Tooltip Test

1. Add debug logging to find the button's actual position
2. Update the `button_position` in the test
3. Run the test and verify the tooltip appears
4. Update baselines: `UPDATE_BASELINE=1 cargo -q run ...`

### Step 4: Investigate SVG Icon Rendering

1. Compare the breakpoint icon rendering path to our button
2. Try using `IconButton` instead of `div()` with a text child
3. If icons still don't render, document the technical limitation

### Step 5: Refine Button Styling

1. Adjust background color to be darker
2. Add hover state with border
3. Test against the mockup
4. Update baselines

## Testing Checklist

After each change, verify:

- [ ] `diff_review_button_enabled.png` shows the button on line 4 with correct styling
- [ ] `diff_review_button_tooltip.png` shows the tooltip "Add Review"
- [ ] `diff_review_button_disabled.png` shows NO button (feature flag disabled)
- [ ] `diff_review_button_regular_editor.png` shows NO button (regular editor, not diff view)
- [ ] All visual tests pass: `cargo -q run --package zed --bin zed_visual_test_runner --features="visual-tests"`

## Additional Resources

- **GPUI Elements**: Look at `crates/gpui/src/elements/div.rs` for how elements handle tooltips
- **UI Components**: Look at `crates/ui/src/components/button/` for how IconButton works
- **Visual Test Framework**: `crates/zed/src/visual_test_runner.rs` contains all visual test infrastructure

## Questions?

If you get stuck:
1. Search the codebase for similar patterns (e.g., how other gutter elements work)
2. Look at the breakpoint implementation as a reference
3. Add `eprintln!()` debugging to understand the flow
4. Check the GPUI documentation in `crates/gpui/src/` for how elements are painted