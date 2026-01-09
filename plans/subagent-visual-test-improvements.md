# Subagent Visual Test Improvements

This document describes improvements needed for the subagent visual tests in `crates/zed/src/visual_test_runner.rs`.

## Background

The subagent UI displays collapsed cards within agent thread messages. When a subagent is spawned, it appears as a card showing:
- Status icon (spinning arrow for running, checkmark for completed)
- Label describing the task
- File change stats: "N files changed +X -Y" with colored additions/deletions

The visual tests capture screenshots of these UI elements to ensure they render correctly.

## Running Visual Tests

### Commands

```bash
# Run visual tests (compares against baselines, fails if different)
cargo run -p zed --bin zed_visual_test_runner --features visual-tests

# Update baseline images (run when UI intentionally changes)
UPDATE_BASELINE=1 cargo run -p zed --bin zed_visual_test_runner --features visual-tests
```

**Important**: Never use `--release` flag - it causes the build to be killed on some systems.

### Output Locations

- **Test output**: `target/visual_tests/` - Screenshots from the most recent test run
- **Baselines**: `crates/zed/test_fixtures/visual_tests/` - Committed baseline images

### Verifying Screenshots

1. Run tests with `UPDATE_BASELINE=1` to generate new screenshots
2. Open images in `target/visual_tests/` to inspect visually
3. Compare against mockups to ensure UI matches design intent
4. If correct, commit the updated baselines in `crates/zed/test_fixtures/visual_tests/`

## Current Issues

Looking at the current screenshots compared to the mockups, several issues need fixing:

### 1. Subagent Card Width

**Problem**: The subagent card spans the entire width of the thread area. It should have horizontal margins like other tool call cards.

**Location**: The card is rendered in `crates/agent_ui/src/acp/thread_view.rs` in the `render_subagent_collapsed` function.

### 2. Status Icon Not Reflecting Actual Status

**Problem**: The subagent collapsed card always shows a blue circle icon regardless of the tool call status (pending, in-progress, completed, failed). It should show:
- Spinning arrow for running/in-progress
- Green checkmark for completed
- Red X for failed/canceled

**Location**: `crates/agent_ui/src/acp/thread_view.rs` in `render_subagent_collapsed` function.

**Current code** (around line 3340):
```rust
.child(
    Icon::new(IconName::Circle)
        .size(IconSize::Small)
        .color(Color::Info),
)
```

**How to fix**: Check `tool_call.status` and render different icons:
```rust
let is_running = matches!(
    tool_call.status,
    ToolCallStatus::Pending | ToolCallStatus::InProgress
);
let is_completed = matches!(tool_call.status, ToolCallStatus::Completed);
let is_failed = matches!(
    tool_call.status,
    ToolCallStatus::Failed | ToolCallStatus::Canceled | ToolCallStatus::Rejected
);

// Then use .when() clauses to render appropriate icons:
.when(is_running, |this| {
    this.child(
        Icon::new(IconName::ArrowCircle)
            .size(IconSize::XSmall)
            .color(Color::Info)
            .with_rotate_animation(2),
    )
})
.when(is_completed, |this| {
    this.child(
        Icon::new(IconName::Check)
            .size(IconSize::XSmall)
            .color(Color::Success),
    )
})
.when(is_failed, |this| {
    this.child(
        Icon::new(IconName::Close)
            .size(IconSize::XSmall)
            .color(Color::Error),
    )
})
```

**Note**: Until this is fixed, the visual tests will show the same blue dot icon for all status states. The tests correctly exercise the production code, but won't visually distinguish between running and completed states.

### 3. Subagent Card Width

**Problem**: The subagent card spans the entire width of the thread area. It should have horizontal margins like other tool call cards.

**Location**: The card is rendered in `crates/agent_ui/src/acp/thread_view.rs` in the `render_subagent_collapsed` function.

**How to investigate**:
```bash
# Find the rendering function
grep -n "fn render_subagent_collapsed" crates/agent_ui/src/acp/thread_view.rs
```

Look at how other tool call cards are rendered (search for `render_tool_call` or similar) to see what padding/margin styles they use. The subagent collapsed card likely needs `.mx_2()` or similar margin styling to match.

### 4. Multiple Stacked Subagent Cards

**Problem**: The test only shows one subagent card. The mockups show multiple subagent cards stacked vertically within a single assistant message.

**Location**: `crates/zed/src/visual_test_runner.rs` in the `run_subagent_visual_tests` function.

**Current code** (around line 1051):
```rust
connection.set_next_prompt_updates(vec![
    acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new(
        "I'll help you refactor...".into(),
    )),
    acp::SessionUpdate::ToolCall(
        acp::ToolCall::new("subagent", "Implement settings navigation refactor")
            .kind(acp::ToolKind::Other)
            .status(acp::ToolCallStatus::InProgress),
    ),
]);
```

**How to fix**: Add multiple `ToolCall` updates, each representing a different subagent. Each needs:
- A unique tool call ID (generated automatically by `ToolCall::new`)
- A descriptive label
- Its own `ActionLog` with file modifications for diff stats

Example structure:
```rust
connection.set_next_prompt_updates(vec![
    acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new(
        "I'll delegate this to multiple subagents:".into(),
    )),
    acp::SessionUpdate::ToolCall(
        acp::ToolCall::new("subagent", "Add concept of agent orchestrator")
            .kind(acp::ToolKind::Other)
            .status(acp::ToolCallStatus::Completed),
    ),
    acp::SessionUpdate::ToolCall(
        acp::ToolCall::new("subagent", "Wire up capacity for tool delegation")
            .kind(acp::ToolKind::Other)
            .status(acp::ToolCallStatus::InProgress),
    ),
]);
```

Then for each tool call, you need to:
1. Get the tool call ID from the thread entries
2. Create a subagent thread with its own ActionLog
3. Set up file modifications in that ActionLog
4. Attach the subagent thread via `ToolCallUpdateSubagentThread`

### 5. Mixed Completed/In-Progress States

**Problem**: Need a test showing multiple subagents where some are completed and some are in-progress.

**How to fix**: When creating multiple tool calls (as above), set different statuses:
- `acp::ToolCallStatus::Completed` for finished subagents
- `acp::ToolCallStatus::InProgress` for running subagents

The status can also be updated after creation:
```rust
thread.update(cx, |thread, cx| {
    thread.handle_session_update(
        acp::SessionUpdate::ToolCallUpdate(acp::ToolCallUpdate::new(
            tool_call_id.clone(),
            acp::ToolCallUpdateFields::new().status(acp::ToolCallStatus::Completed),
        )),
        cx,
    )
})??;
```

### 6. Agent Response After Completed Subagents

**Problem**: Need to show agent response text appearing after all subagents complete.

**How to fix**: After setting all subagent statuses to completed, add another agent message chunk:
```rust
// After updating all tool calls to Completed status...
thread.update(cx, |thread, cx| {
    thread.handle_session_update(
        acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new(
            "All refactoring tasks are complete. The settings panel has been reorganized.".into(),
        )),
        cx,
    )
})??;
```

### 7. New Test: All Completed Portal View

**Problem**: Current portal test shows running state. Need separate test for completed state.

**How to fix**: After `run_visual_test("subagent_portal_running", ...)`, add:
1. Update all tool call statuses to `Completed`
2. Expand one of the tool calls
3. Call `run_visual_test("subagent_portal_completed", ...)`

## File Modification Setup

Setting up diff stats for subagents is complex due to async race conditions. The current approach:

1. **Create test files** on disk with original content
2. **Open buffer** from the project
3. **Edit buffer** with new content
4. **Register with ActionLog** using `buffer_created()` (not `buffer_read()` - this avoids race conditions)
5. **Set base text directly** on the BufferDiff using `set_base_text()`

Key code pattern (from current implementation):
```rust
// Capture original content
let original_content: Arc<str> = 
    buffer.read_with(cx, |buffer, _| Arc::from(buffer.text().as_str()))?;

// Edit the buffer
buffer.update(cx, |buffer, cx| {
    buffer.edit([(0..buffer.len(), new_content)], None, cx);
})?;

// Register with action log
subagent_action_log.update(cx, |action_log, cx| {
    action_log.buffer_created(buffer.clone(), cx);
})?;

// Set base_text directly on the diff
let buffer_snapshot = buffer.read_with(cx, |buffer, _| buffer.text_snapshot())?;
let diff_entity = subagent_action_log.read_with(cx, |log, cx| {
    log.tracked_buffers_for_debug(cx)
        .find(|(b, _)| *b == &buffer)
        .map(|(_, tracked)| tracked.diff().clone())
})?;

if let Some(diff) = diff_entity {
    let receiver = diff.update(cx, |diff, cx| {
        diff.set_base_text(Some(original_content), None, buffer_snapshot, cx)
    })?;
    receiver.await.ok();
}
```

For multiple subagents, each needs its own:
- `ActionLog` instance
- Set of modified buffers
- Different file paths (to avoid conflicts)

## Test Structure Overview

The subagent tests are in `run_subagent_visual_tests()` function. Current flow:

1. Create temp directory with test files
2. Create project and add worktree
3. Set up stub connection with programmed responses
4. Create workspace and agent panel
5. Send user message to trigger response
6. Get tool call ID and create subagent thread
7. Set up file modifications in ActionLog
8. Attach subagent thread to tool call
9. Capture screenshots at various states

## Key Types and Their Locations

| Type | Location | Purpose |
|------|----------|---------|
| `AcpThread` | `crates/acp_thread/src/acp_thread.rs` | Thread holding conversation entries |
| `ActionLog` | `crates/action_log/src/action_log.rs` | Tracks buffer edits for diff stats |
| `AcpThreadView` | `crates/agent_ui/src/acp/thread_view.rs` | UI rendering of thread |
| `StubAgentConnection` | `crates/acp_thread/src/connection.rs` | Test double for agent connections |
| `BufferDiff` | `crates/buffer_diff/src/buffer_diff.rs` | Computes diffs for diff stats |

## Debugging Tips

### Check if diffs are computed correctly
Add logging in the test:
```rust
let changed_count = action_log.read_with(cx, |log, cx| {
    let changed = log.changed_buffers(cx);
    log::info!("Action log has {} changed buffers", changed.len());
    for (buffer, diff) in &changed {
        let snapshot = buffer.read(cx).snapshot();
        let diff_snapshot = diff.read(cx).snapshot(cx);
        let hunk_count = diff_snapshot.hunks(&snapshot).count();
        log::info!("  Buffer has {} hunks", hunk_count);
    }
    changed.len()
})?;
```

### Wait for async operations
The diff computation is async. Use multiple refresh/timer cycles:
```rust
for _ in 0..3 {
    cx.refresh()?;
    cx.background_executor()
        .timer(std::time::Duration::from_millis(100))
        .await;
}
```

### Window size
The agent panel window size is controlled by `agent_panel_window_size()` at the top of the file:
```rust
fn agent_panel_window_size() -> Size<Pixels> {
    Size {
        width: px(500.0),
        height: px(450.0),
    }
}
```

## Recommended Implementation Order

1. **Fix status icon rendering** - This is a bug in `thread_view.rs` - icons should reflect status
2. **Fix card width** - This is a rendering issue in `thread_view.rs`, independent of tests
3. **Add multiple subagent cards** - Modify test to create multiple tool calls with subagent threads
4. **Add mixed status test** - Set different statuses on different subagents
5. **Add completed portal test** - New test scenario
6. **Add response after completion** - Add agent message after subagents complete

## Testing Your Changes

After each change:
1. Run `UPDATE_BASELINE=1 cargo run -p zed --bin zed_visual_test_runner --features visual-tests`
2. Open `target/visual_tests/subagent_*.png` files
3. Compare visually against mockups
4. If correct, commit updated baselines