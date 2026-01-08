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

## Current Test Output (as of January 8, 2025)

After running the visual tests, here's what each screenshot shows:

### 1. `diff_review_button_enabled.png`
**Status**: ❌ BROKEN - Shows empty state  
**What it shows**: "No uncommitted changes" / "Remote up to date" / "Close" button  
**Expected**: Diff view with file content and a "+" button in the gutter on the last row

### 2. `diff_review_button_tooltip.png`  
**Status**: ❌ BROKEN - Shows empty state (identical to enabled)  
**What it shows**: "No uncommitted changes" / "Remote up to date" / "Close" button  
**Expected**: Same as enabled, plus hover border on button and "Add Review" tooltip visible

### 3. `diff_review_button_disabled.png`
**Status**: ⚠️ SHOWS DIFF CONTENT (but feature flag is disabled)
**What it shows**: Full diff view with:
  - File header: "thread-view.tsx src/" with "Open File" button
  - Deleted line showing "// Original content" (red background)
  - Added line showing "// Modified content with changes" (green background)  
  - Lines 2-4 with import statements
  - Stage/Restore buttons
  - NO "+" button visible (correct - flag is disabled)
**Expected**: Same diff content with NO "+" button visible (feature flag disabled)
**Note**: This screenshot proves the diff system DOES work - git status just takes time to detect

### 4. `diff_review_button_regular_editor.png`
**Status**: ❌ BROKEN - Shows empty editor  
**What it shows**: Just title bar with "project main" and "Sign In" - no editor content  
**Expected**: Regular editor with the file content, but NO "+" button (only shows in diff view)

---

## Root Cause Analysis

### Issue 1: Git Status Detection Timing (PRIMARY ISSUE)

**The core problem**: Tests 1 and 2 (enabled/tooltip) run before git status detection completes. By the time test 3 (disabled) runs, the git status HAS been detected (which is why that screenshot shows diff content).

**Why this happens**: 
- The test creates a REAL git repository on disk using `std::process::Command::new("git")`
- Git status detection involves actual file system I/O and git operations
- `cx.advance_clock()` advances GPUI's simulated timer, but doesn't speed up real I/O
- `cx.run_until_parked()` only runs queued GPUI tasks, not blocking I/O

**Location**: `crates/zed/src/visual_test_runner.rs` in `run_diff_review_visual_tests()`

**Current code** (around line 943):
```rust
for _ in 0..10 {
    cx.advance_clock(Duration::from_millis(500));
    cx.run_until_parked();
}
```

This isn't sufficient because git status detection depends on:
1. Worktree file scanning (real I/O)
2. Git repository detection (real git operations)
3. Git status computation (real git operations)

### Issue 2: Regular Editor Not Loading File

**What's happening**: The `diff_review_button_regular_editor.png` shows an empty window - no file content.

**Likely cause**: The file open task may complete but the editor hasn't rendered, or there's an issue with the worktree access from the second window.

**Location**: Around line 1078-1140 in `visual_test_runner.rs`

### Issue 3: Test Order Creates Misleading Results

**Current flow**:
1. Test 1 (enabled): Take screenshot (git not ready → empty state)
2. Test 1b (tooltip): Take screenshot (git not ready → empty state)  
3. Close ProjectDiff, disable flags
4. Test 2 (disabled): Reopen ProjectDiff, take screenshot (git NOW ready → shows content)
5. Test 3 (regular): Open file in new window (doesn't load properly)

**The irony**: By the time we take the "disabled" screenshot, the diff IS working. We just took the "enabled" screenshots too early.

---

## Required Fix: Poll Until Diff Has Content

The fix is to **actively poll** until the ProjectDiff view reports having content, rather than using a fixed number of iterations.

### Step 1: Access the Editor to Check Content

Since `ProjectDiff.multibuffer` is private, we can use `act_as_type` to get the Editor, then check if its buffer has content:

```rust
// In visual_test_runner.rs, replace the fixed waiting loop with:
use editor::Editor;

// Poll until the diff view has content
let max_attempts = 200;  // ~20 seconds of simulated time
for attempt in 0..max_attempts {
    let has_content = workspace_window
        .update(cx, |workspace, _window, cx| {
            // Get the active item and check if it's an editor with content
            if let Some(item) = workspace.active_item(cx) {
                // Use act_as_type to get the Editor from ProjectDiff
                if let Some(editor_entity) = item.act_as_type::<Editor>(cx) {
                    let editor = editor_entity.downcast::<Editor>().unwrap();
                    let buffer = editor.read(cx).buffer().read(cx);
                    !buffer.is_empty()
                } else {
                    false
                }
            } else {
                false
            }
        })
        .unwrap_or(false);
    
    if has_content {
        eprintln!("Diff content detected after {} iterations", attempt);
        break;
    }
    
    cx.advance_clock(Duration::from_millis(100));
    cx.run_until_parked();
}
```

### Step 2: Add Import for Editor Type

At the top of `visual_test_runner.rs`, add:

```rust
#[cfg(target_os = "macos")]
use editor::Editor;
```

### Step 3: Fix Regular Editor Test Similarly

Apply similar polling for the regular editor test:

```rust
// After opening the file, poll until editor has content
for attempt in 0..50 {
    let has_editor_content = regular_window
        .update(cx, |workspace, _window, cx| {
            if let Some(item) = workspace.active_item(cx) {
                if let Some(editor_entity) = item.act_as_type::<Editor>(cx) {
                    let editor = editor_entity.downcast::<Editor>().unwrap();
                    !editor.read(cx).buffer().read(cx).is_empty()
                } else {
                    false
                }
            } else {
                false
            }
        })
        .unwrap_or(false);
    
    if has_editor_content {
        break;
    }
    
    cx.advance_clock(Duration::from_millis(100));
    cx.run_until_parked();
}
```

---

## Determinism Considerations

**Critical requirement**: Visual tests must be completely deterministic and cannot flake based on system timer, CPU load, etc.

**Current approach (polling with fixed max iterations)**: 
- Reasonably deterministic for most systems
- Could fail on extremely slow systems
- Not perfect but acceptable for now

**Better long-term solutions** (in order of preference):

1. **Mock the git layer in tests**: Use a fake git implementation that doesn't require real I/O. This would make tests instant and fully deterministic.

2. **Pre-populate the multibuffer**: Instead of relying on git detection, directly inject test content into the ProjectDiff's multibuffer. This bypasses all the async git machinery.

3. **Synchronous git operations for tests**: Add a test mode that forces git operations to complete synchronously before returning.

For now, the polling approach with a high iteration limit (200 iterations × 100ms = 20 seconds simulated time) should be reliable on any reasonable system, since we're advancing simulated time, not wall-clock time. The actual elapsed wall time depends on how fast the git operations complete.

---

## Success Criteria

When fixed, the screenshots should show:

1. **diff_review_button_enabled.png**: 
   - File header: "thread-view.tsx src/"
   - Diff content with Original/Modified lines  
   - "+" button visible in gutter area on the last row (row 4 with `import { AiPaneTabContext...`)
   - Button has dark background and white "+" icon

2. **diff_review_button_tooltip.png**:
   - Same as enabled, plus:
   - Subtle border around the "+" button (hover state)
   - "Add Review" tooltip visible near the button

3. **diff_review_button_disabled.png**:
   - Same diff view content  
   - NO "+" button visible anywhere (feature flag disabled)

4. **diff_review_button_regular_editor.png**:
   - Regular editor showing file content (import statements)
   - Line numbers visible
   - NO "+" button visible (regular editor, not diff view)

---

## Debugging Commands

To run the visual tests:
```bash
cargo -q run --package zed --bin zed_visual_test_runner --features="visual-tests"
```

To view the generated screenshots:
```bash
open target/visual_tests/diff_review_button_*.png
```

To add debugging output, insert `eprintln!` statements in the test runner:
```rust
eprintln!("Iteration {}: checking for content...", attempt);
```

---

## Notes on Tooltip Testing

Tooltips in GPUI have a built-in delay (`TOOLTIP_SHOW_DELAY` = 500ms). For the tooltip to appear:

1. Mouse must be moved to the button position via `window.simulate_mouse_move()`
2. Clock must be advanced past the tooltip delay
3. Window must be refreshed to trigger re-render

Current code does this correctly, but since the button itself isn't rendered (due to empty diff state), the tooltip can't appear.

**Important**: The button position is hardcoded as `point(px(27.0), px(232.0))`. Once the diff renders correctly, this position may need adjustment to match where the button actually appears. The button is rendered at the last row of the document in the gutter area.

If tooltips still don't work after fixing the button, investigate whether `prepaint_as_root()` elements properly participate in the hover/tooltip system. The fallback would be to document this as a known limitation and test tooltips manually.