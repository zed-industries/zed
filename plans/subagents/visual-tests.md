# Subagent Visual Tests Guide

This document explains how to write visual tests for the subagent UI, particularly tests that involve file modifications to show the diff stats feature.

## Background

The subagent UI displays a collapsed card showing:
- Status icon (spinning arrow for running, checkmark for completed, X for failed)
- Label from the subagent tool call
- File change stats: "N files changed +X -Y" with git-style coloring (green for additions, red for deletions)

The file change stats are computed from the subagent's `action_log`, which tracks buffer edits made during the subagent's execution.

## Project Structure

Key files involved:

```
crates/
├── agent_ui/src/acp/thread_view.rs    # UI rendering (render_subagent_collapsed, render_subagent_portal)
├── acp_thread/src/acp_thread.rs       # Thread data structures and SubagentThread content type
├── acp_thread/src/connection.rs       # StubAgentConnection for testing
├── action_log/src/action_log.rs       # Tracks file edits (changed_buffers method)
├── zed/src/visual_test_runner.rs      # Visual test runner binary
└── feature_flags/src/flags.rs         # SubagentsFeatureFlag
```

## Running Visual Tests

### Prerequisites

- macOS (visual tests use Metal for rendering)
- Screen Recording permission may be needed for some test scenarios

### Commands

```bash
# Run visual tests and generate screenshots (will fail if baselines don't exist)
cargo run -p zed --bin zed_visual_test_runner --features visual-tests --release

# Run and update/create baseline screenshots
UPDATE_BASELINE=1 cargo run -p zed --bin zed_visual_test_runner --features visual-tests --release
```

### Output Location

Screenshots are saved to `target/visual_tests/`:
- `subagent_collapsed_running.png` - Collapsed state with spinning icon
- `subagent_portal_running.png` - Expanded portal view
- `subagent_collapsed_completed.png` - Collapsed state with checkmark

## How the Current Tests Work

The existing subagent visual tests are in `crates/zed/src/visual_test_runner.rs` in the `run_subagent_visual_tests` function. Here's what they do:

1. Create a temporary project directory
2. Create a `StubAgentConnection` that returns a pre-programmed tool call
3. Open the Agent Panel with this stub connection
4. Send a message to trigger the tool call response
5. Create a subagent thread and attach it to the tool call
6. Capture screenshots in collapsed and expanded states

### Why File Stats Don't Show

The current tests don't show file change stats because:
1. The subagent thread is created with a fresh `ActionLog`
2. No actual file edits are performed
3. `action_log.changed_buffers()` returns an empty map

## Adding Tests with File Modifications

To show file change stats, you need to simulate the subagent making edits to files. Here's how:

### Step 1: Create Test Files

The test already creates a project directory. Add files that will be "modified":

```rust
// In run_subagent_visual_tests function, after creating project_path:
let test_file_path = project_path.join("src/settings.rs");
std::fs::create_dir_all(project_path.join("src"))?;
std::fs::write(&test_file_path, "// Original content\nfn old_function() {}\n")?;
```

### Step 2: Open the File in a Buffer

The `ActionLog` tracks edits to buffers that have been opened. You need to:

```rust
// After creating the subagent thread and action_log:
let subagent_action_log = cx.new(|_| action_log::ActionLog::new(project.clone()))?;

// Open the file as a buffer
let buffer = project.update(cx, |project, cx| {
    let path = project.project_path_for_absolute_path(&test_file_path, cx)?;
    anyhow::Ok(project.open_buffer(path, cx))
})??;
let buffer = buffer.await?;

// Register the buffer with the action log
subagent_action_log.update(cx, |action_log, cx| {
    action_log.buffer_read(buffer.clone(), cx);
})?;
```

### Step 3: Make Edits to the Buffer

Edit the buffer to create diff hunks:

```rust
// Make edits to the buffer
buffer.update(cx, |buffer, cx| {
    // Replace content - this creates a diff
    let new_content = "// Modified content\nfn new_function() {\n    println!(\"Hello\");\n}\n\nfn another_function() {}\n";
    buffer.edit([(0..buffer.len(), new_content)], None, cx);
})?;

// Mark the buffer as edited in the action log
subagent_action_log.update(cx, |action_log, cx| {
    action_log.buffer_edited(buffer.clone(), cx);
})?;
```

### Step 4: Create the Subagent Thread with the Modified ActionLog

```rust
let subagent_thread = cx.new(|cx| {
    acp_thread::AcpThread::new(
        "Settings Navigation Refactor",
        subagent_connection,
        project.clone(),
        subagent_action_log,  // This action_log now has tracked edits
        subagent_session_id,
        watch::Receiver::constant(
            acp::PromptCapabilities::new()
                .image(true)
                .audio(true)
                .embedded_context(true),
        ),
        cx,
    )
})?;
```

### Complete Example

Here's a more complete example showing how to create a test with file modifications:

```rust
async fn run_subagent_with_file_changes_test(
    app_state: Arc<AppState>,
    cx: &mut gpui::AsyncApp,
    update_baseline: bool,
) -> Result<TestResult> {
    use acp_thread::ToolCallUpdateSubagentThread;
    use agent_ui::AgentPanel;

    // Create project with test files
    let temp_dir = tempfile::tempdir()?;
    let project_path = temp_dir.path().join("project");
    std::fs::create_dir_all(project_path.join("src"))?;
    
    // Create original file content
    let settings_file = project_path.join("src/settings.rs");
    std::fs::write(&settings_file, 
        "fn old_settings() {\n    // old code\n}\n"
    )?;

    // Create project
    let project = cx.update(|cx| {
        project::Project::local(
            app_state.client.clone(),
            app_state.node_runtime.clone(),
            app_state.user_store.clone(),
            app_state.languages.clone(),
            app_state.fs.clone(),
            None,
            false,
            cx,
        )
    })?;

    // Add worktree
    let add_worktree_task = project.update(cx, |project, cx| {
        project.find_or_create_worktree(&project_path, true, cx)
    })?;
    add_worktree_task.await?;

    // Wait for worktree to scan
    cx.background_executor()
        .timer(std::time::Duration::from_millis(200))
        .await;

    // Create action log for subagent
    let subagent_action_log = cx.new(|_| action_log::ActionLog::new(project.clone()))?;

    // Open file as buffer
    let buffer_task = project.update(cx, |project, cx| {
        let worktree = project.worktrees(cx).next()?;
        let path = project::ProjectPath {
            worktree_id: worktree.read(cx).id(),
            path: Arc::from(std::path::Path::new("src/settings.rs")),
        };
        Some(project.open_buffer(path, cx))
    })?;
    
    if let Some(buffer_task) = buffer_task {
        let buffer = buffer_task.await?;
        
        // Register buffer with action log
        subagent_action_log.update(cx, |action_log, cx| {
            action_log.buffer_read(buffer.clone(), cx);
        })?;
        
        // Make edits
        buffer.update(cx, |buffer, cx| {
            let new_content = "fn new_settings() {\n    // refactored code\n    println!(\"Settings loaded\");\n}\n\nfn helper() {}\n";
            buffer.edit([(0..buffer.len(), new_content)], None, cx);
        })?;
        
        // Mark as edited
        subagent_action_log.update(cx, |action_log, cx| {
            action_log.buffer_edited(buffer.clone(), cx);
        })?;
    }

    // Now create the subagent thread with this action_log
    // ... rest of the test follows the same pattern as run_subagent_visual_tests
}
```

## How Diff Stats Are Computed

The `compute_subagent_diff_stats` function in `thread_view.rs` works like this:

```rust
fn compute_subagent_diff_stats(
    &self,
    subagent_thread: &Entity<AcpThread>,
    cx: &Context<Self>,
) -> SubagentDiffStats {
    let thread = subagent_thread.read(cx);
    let action_log = thread.action_log().read(cx);
    let changed_buffers = action_log.changed_buffers(cx);  // BTreeMap<Entity<Buffer>, Entity<BufferDiff>>
    let files_changed = changed_buffers.len();

    let mut lines_added = 0u32;
    let mut lines_removed = 0u32;

    for (buffer, diff_handle) in &changed_buffers {
        let buffer_snapshot = buffer.read(cx).snapshot();
        let diff_snapshot = diff_handle.read(cx).snapshot(cx);
        for hunk in diff_snapshot.hunks(&buffer_snapshot) {
            // Count new lines from the hunk range
            let new_lines = hunk.range.end.row.saturating_sub(hunk.range.start.row);
            lines_added += new_lines;
            
            // Count old lines from the base text
            if let Some(base_text_string) = diff_snapshot.base_text_string() {
                if hunk.diff_base_byte_range.end <= base_text_string.len() {
                    let old_text = &base_text_string[hunk.diff_base_byte_range.clone()];
                    let old_lines = old_text.lines().count() as u32;
                    lines_removed += old_lines;
                }
            }
        }
    }

    SubagentDiffStats { files_changed, lines_added, lines_removed }
}
```

## Comparing to Mockups

When reviewing screenshots, compare against the mockups which show:

1. **Collapsed state should have:**
   - Status icon on the left (blue spinner for running, green check for completed)
   - Label text (e.g., "Implement settings navigation refactor")
   - Em-dash separator
   - File stats: "N files changed" in muted text
   - "+X" in green, "-Y" in red

2. **Portal/expanded state should have:**
   - Header with Zed Agent icon
   - Label
   - "Expand Subagent" button
   - Collapse chevron (^)
   - Scrollable content area with thread messages

## Debugging Tips

### Check if ActionLog has changes

Add logging to verify the action log is tracking edits:

```rust
let changed_count = subagent_action_log.read_with(cx, |log, cx| {
    log.changed_buffers(cx).len()
})?;
log::info!("Action log has {} changed buffers", changed_count);
```

### Verify buffer diffs exist

```rust
for (buffer, diff) in action_log.changed_buffers(cx) {
    let snapshot = buffer.read(cx).snapshot();
    let diff_snapshot = diff.read(cx).snapshot(cx);
    let hunk_count = diff_snapshot.hunks(&snapshot).count();
    log::info!("Buffer has {} hunks", hunk_count);
}
```

### Common Issues

1. **Empty diff stats**: The buffer wasn't registered with `buffer_read` before editing
2. **No changed_buffers**: The buffer wasn't marked with `buffer_edited` after editing
3. **Wrong file path**: The worktree didn't scan the file, or the path is incorrect
4. **Timing issues**: Add delays after worktree operations to let scanning complete

## Adding New Visual Test Scenarios

To add a new test scenario:

1. Add a new async function following the pattern of `run_subagent_visual_tests`
2. Call it from `main()` in the test runner
3. Use `run_visual_test("your_test_name", ...)` to capture screenshots
4. Run with `UPDATE_BASELINE=1` to generate initial screenshots
5. Review the screenshots in `target/visual_tests/`
6. Iterate on the test until the output matches the expected design

## Reference: Key Types

- `AcpThread` - The thread entity that holds conversation entries
- `ToolCall` - A tool call entry in the thread
- `ToolCallContent::SubagentThread(Entity<AcpThread>)` - Content type for subagent threads
- `ActionLog` - Tracks buffer reads and edits
- `BufferDiff` - Computes diffs between buffer and base text
- `StubAgentConnection` - Test double for agent connections
- `SubagentDiffStats` - Struct holding files_changed, lines_added, lines_removed