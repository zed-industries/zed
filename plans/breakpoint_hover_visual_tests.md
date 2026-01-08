# Breakpoint Hover Visual Tests Implementation Guide

## ⚠️ CRITICAL: Success Criteria ⚠️

**READ THIS FIRST. READ IT AGAIN. DO NOT SKIP THIS SECTION.**

The ONLY way to evaluate success on this project is:

1. Run the visual tests to generate PNG screenshots
2. Open the generated PNGs from `target/visual_tests/`
3. Compare them visually to the reference mockups in this document
4. Assemble a list of problems (differences from the mockups)
5. **Success = the list of problems is empty**

**You are NOT done until:**
- You have regenerated the screenshots
- You have opened and looked at each PNG file
- You have compared each one to the corresponding mockup
- You have found ZERO problems

**Do NOT report success based on:**
- Tests passing (they pass even when broken)
- Debug output showing code paths are exercised
- Code that "should" work
- Anything other than the actual PNG files matching the mockups

## Goal

Create 3 visual tests that capture these states in a normal (non-git-diff) buffer:

1. **Gutter with line numbers, no breakpoint hover** - The baseline state
2. **Gutter with breakpoint hover indicator** - The gray circle that appears when hovering over a line in the gutter
3. **Gutter with breakpoint hover AND tooltip** - The full tooltip that says "Set breakpoint" with keybinding

## Reference Screenshots (MOCKUPS)

The expected visual states are shown below. Your generated PNGs MUST match these.

### State 1: No hover (baseline)
```
┌─────────┐
│       1 │
│       2 │
│       3 │
└─────────┘
```
Just line numbers in the gutter, no breakpoint indicator. This is the simplest case.

### State 2: Breakpoint hover indicator (circle only)
```
┌─────────┐
│  ●    1 │
│       2 │
│       3 │
└─────────┘
```
A gray/muted circle appears to the left of line 1, indicating a breakpoint can be set. The circle should be clearly visible - NOT the same as State 1.

### State 3: Breakpoint hover with tooltip
```
┌─────────────────────────────────┐
│  ●    1    # Diff Revi         │
│  ┌────────────────────────┐    │
│  │ Set breakpoint      f9 │    │
│  │ Right-click for more   │    │
│  │ options.               │    │
│  └────────────────────────┘    │
│       2                        │
│       3                        │
└────────────────────────────────┘
```
The tooltip appears showing "Set breakpoint" with the F9 keybinding and meta text.

## Current State (As of Last Session)

### What Exists
- Test function `run_breakpoint_hover_visual_tests` in `crates/zed/src/visual_test_runner.rs`
- Test is registered and runs as "Test 5: breakpoint_hover (3 variants)"
- Three PNG files are generated in `target/visual_tests/`:
  - `breakpoint_hover_none.png`
  - `breakpoint_hover_circle.png`
  - `breakpoint_hover_tooltip.png`

### The Problem
**All three screenshots look identical.** They all show a normal editor with no breakpoint indicator and no tooltip visible. The tests "pass" because they compare against baselines that were created with the same broken output.

### What Was Verified Working
Through extensive debug logging, we confirmed:
1. ✅ Mouse move events ARE being dispatched to the editor
2. ✅ `gutter_hovered` IS being set to `true`
3. ✅ The buffer HAS a file association (required for breakpoint indicators)
4. ✅ `PhantomBreakpointIndicator` IS being created
5. ✅ The 200ms timer IS firing and setting `is_active = true`
6. ✅ The phantom breakpoint IS being added to `breakpoint_rows` HashMap
7. ✅ `layout_breakpoints` IS processing 1 breakpoint
8. ✅ `prepaint_gutter_button` IS positioning the button at valid coordinates (e.g., `origin=(3.12px, 66.5px), size=(14px, 16px)`)
9. ✅ The paint phase IS painting 1 breakpoint

### What Is NOT Working
Despite all the above being true:
- ❌ The breakpoint indicator icon is NOT visible in the captured screenshot
- ❌ The tooltip is NOT visible in the captured screenshot

### Hypotheses for Why the Icon Doesn't Appear
1. **Color issue**: The phantom breakpoint uses `Color::Hint` which might be too dim or transparent against the dark theme background
2. **Icon rendering issue**: The icon might not be rendering correctly in the `render_to_image` capture
3. **Z-ordering issue**: Something might be painting over the indicator
4. **Icon asset issue**: The breakpoint icon asset might not be loading correctly in test context

### Hypotheses for Why the Tooltip Doesn't Appear
1. **Timing**: The tooltip might require additional event processing beyond `advance_clock`
2. **Focus**: Tooltips might require window/element focus that isn't present
3. **Capture timing**: The tooltip might be rendered in a separate layer not captured by `render_to_image`

## Key Code Locations

| File | Purpose |
|------|---------|
| `crates/zed/src/visual_test_runner.rs` | Visual test code - `run_breakpoint_hover_visual_tests` function |
| `crates/editor/src/element.rs` | Editor rendering, `mouse_moved` handler, breakpoint layout |
| `crates/editor/src/editor.rs` | `render_breakpoint()` function, `PhantomBreakpointIndicator` struct |
| `crates/gpui/src/elements/div.rs` | Tooltip implementation, `TOOLTIP_SHOW_DELAY` |

## How Breakpoint Hover Works (Code Flow)

### Step 1: Mouse Move Detection
In `element.rs`, the `mouse_moved` function:
```rust
let gutter_hovered = gutter_hitbox.bounds.contains(&event.position);
editor.set_gutter_hovered(gutter_hovered, cx);
```

### Step 2: Phantom Indicator Creation
If `gutter_hovered` is true and the buffer has a file:
```rust
let breakpoint_indicator = if gutter_hovered && !is_on_diff_review_button_row {
    // ... checks buffer has file ...
    Some(PhantomBreakpointIndicator {
        display_row: valid_point.row(),
        is_active: is_visible,  // false initially
        collides_with_existing_breakpoint: has_existing_breakpoint,
    })
}
```

### Step 3: Timer for Activation
A 200ms timer is spawned to set `is_active = true`:
```rust
editor.gutter_breakpoint_indicator.1.get_or_insert_with(|| {
    cx.spawn(async move |this, cx| {
        cx.background_executor()
            .timer(Duration::from_millis(200))
            .await;
        this.update(cx, |this, cx| {
            if let Some(indicator) = this.gutter_breakpoint_indicator.0.as_mut() {
                indicator.is_active = true;
                cx.notify();
            }
        }).ok();
    })
});
```

### Step 4: Adding to breakpoint_rows
During prepaint, active phantom breakpoints are added to `breakpoint_rows`:
```rust
if let Some(phantom_breakpoint) = &mut editor
    .gutter_breakpoint_indicator
    .0
    .filter(|phantom_breakpoint| phantom_breakpoint.is_active)
{
    breakpoint_rows.entry(phantom_breakpoint.display_row).or_insert_with(|| {
        // ... creates breakpoint entry ...
    });
}
```

### Step 5: Layout and Paint
`layout_breakpoints` creates button elements, and they're painted in the paint phase:
```rust
for breakpoint in layout.breakpoints.iter_mut() {
    breakpoint.paint(window, cx);
}
```

## Key Constants

```rust
// Tooltip delay (from gpui/src/elements/div.rs)
const TOOLTIP_SHOW_DELAY: Duration = Duration::from_millis(500);

// Breakpoint hover debounce (from editor/src/element.rs)
const BREAKPOINT_HOVER_DEBOUNCE: Duration = Duration::from_millis(200);
```

## Current Test Implementation

The current test does the following sequence:
1. Creates a temp project with a `.rs` file
2. Opens the file in a 300x200 window
3. Takes screenshot 1 (no hover)
4. Calls `window.draw()` to register mouse listeners
5. Simulates mouse move to gutter position `(30, 85)`
6. Advances clock 300ms (past 200ms debounce)
7. Calls `window.draw()` again
8. Takes screenshot 2 (should show circle)
9. Advances clock 600ms (past 500ms tooltip delay)
10. Calls `window.draw()` again  
11. Takes screenshot 3 (should show tooltip)

## Debugging Tips

### Adding Debug Output
If you need to trace execution, add `eprintln!` statements. Key places:
- `mouse_moved` in `element.rs` - to verify gutter hover detection
- `layout_breakpoints` in `element.rs` - to verify breakpoints being processed
- `prepaint_gutter_button` in `element.rs` - to see button positioning
- Paint phase in `element.rs` - to verify painting happens

### Checking Icon Color
The phantom breakpoint uses this color logic in `render_breakpoint`:
```rust
let color = if is_phantom {
    if collides_with_existing {
        Color::Custom(color.debugger_accent.blend(color.text.opacity(0.6)))
    } else {
        Color::Hint  // This is what phantom breakpoints use
    }
}
```

Try changing `Color::Hint` to something more visible like `Color::Error` to see if the icon renders at all.

### Verifying Element Position
The button is positioned at approximately:
- x: ~3 pixels from left edge of editor
- y: ~67 pixels from top of window (for line 1)
- size: 14x16 pixels

## Running the Tests

```bash
# Run all visual tests
cargo -q run --package zed --bin zed_visual_test_runner --features="visual-tests"

# Update baselines (creates new baseline images)
UPDATE_BASELINE=1 cargo -q run --package zed --bin zed_visual_test_runner --features="visual-tests"
```

Output files are in `target/visual_tests/`:
- `breakpoint_hover_none.png`
- `breakpoint_hover_circle.png`
- `breakpoint_hover_tooltip.png`

## Suggested Next Steps

1. **Investigate why the icon doesn't render visibly**
   - Try changing `Color::Hint` to `Color::Error` in `render_breakpoint` to see if ANY icon appears
   - Check if the icon asset is loading correctly
   - Try a much larger icon size

2. **Investigate the tooltip**
   - Check how other visual tests handle tooltips (if any do)
   - Verify the tooltip element is being created
   - Check if tooltips require special handling for `render_to_image`

3. **Consider simplifying the test**
   - Maybe test just the circle first, get that working, then tackle tooltip
   - Consider if there's a simpler way to trigger the hover state

4. **Check theme/color configuration**
   - Verify what `Color::Hint` resolves to in the test theme
   - Try explicitly setting a high-contrast color

## ⚠️ REMINDER: Success Criteria ⚠️

Before reporting this task as complete:

1. Run: `cargo -q run --package zed --bin zed_visual_test_runner --features="visual-tests"`
2. Open: `target/visual_tests/breakpoint_hover_none.png` - verify it shows just line numbers
3. Open: `target/visual_tests/breakpoint_hover_circle.png` - verify it shows a VISIBLE gray circle next to line 1
4. Open: `target/visual_tests/breakpoint_hover_tooltip.png` - verify it shows the circle AND a tooltip with "Set breakpoint f9"
5. Compare each to the mockups in this document
6. If ANY differences exist, you are NOT done

**The tests currently "pass" but produce incorrect output. Do not be fooled by passing tests.**