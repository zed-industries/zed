# Subagent UI Implementation Plan

## Current State

The subagent tool is functionally working - it can spawn multiple subagents in parallel, they execute their tasks, and return combined results. However, the UI is completely broken:

### What's Currently Wrong

1. **No visual feedback during execution**: The UI shows nothing while subagents are running. Users only see results after all subagents complete.

2. **Single collapsed card for multiple subagents**: When multiple subagents run in parallel, they're shown as a single "2 subagents" tool call instead of individual cards for each subagent.

3. **Wrong expanded view**: When expanded, the tool call shows generic "Input:" and "Output:" sections instead of the proper subagent UI with:
   - Individual subagent cards that can be expanded/collapsed
   - Status indicators (running spinner, completed checkmark, failed X)
   - File change stats (N files changed, +X -Y lines)
   - The ability to view each subagent's conversation thread

4. **Missing `update_subagent_thread()` calls**: The code that would attach the subagent's `AcpThread` to the tool call content was removed during refactoring for multiple subagents.

## Architecture Overview

### Key Components

1. **`SubagentTool`** (`crates/agent/src/tools/subagent_tool.rs`)
   - Implements `AgentTool` trait
   - Takes `SubagentToolInput` with array of `SubagentConfig` items
   - Spawns multiple subagent `Thread` entities in parallel
   - Each subagent has its own `AcpThread` for display purposes

2. **`Thread`** (`crates/agent/src/thread.rs`)
   - The agent's conversation thread
   - `new_subagent()` creates a thread for a subagent with inherited tools
   - `ToolCallEventStream` has `update_subagent_thread()` to attach display thread

3. **`AcpThread`** (`crates/acp_thread/src/acp_thread.rs`)
   - Display-layer thread representation
   - `ToolCallContent::SubagentThread(Entity<AcpThread>)` - content type for subagent threads
   - `ToolCallUpdateSubagentThread` - update type to attach thread to tool call

4. **`AcpThreadView`** (`crates/agent_ui/src/acp/thread_view.rs`)
   - Renders the conversation thread UI
   - `render_tool_call()` - renders individual tool calls
   - `render_tool_call_content()` - renders content including `SubagentThread`
   - Currently returns `Empty` for `SubagentThread` content

### Data Flow

```
SubagentTool::run()
    → Creates Entity<Thread> for each subagent
    → Creates Entity<AcpThread> for each subagent (display layer)
    → Should call event_stream.update_subagent_thread() for each
    → ToolCallUpdateSubagentThread flows to AcpThread::update_tool_call()
    → AcpThread stores ToolCallContent::SubagentThread
    → AcpThreadView::render_tool_call_content() renders it
```

## Required Implementation

### 1. Support Multiple SubagentThreads Per Tool Call

The current `ToolCallContent::SubagentThread` only holds a single `Entity<AcpThread>`. This must be changed to support multiple subagents.

**Option A**: Add a new content type `ToolCallContent::SubagentThreads(Vec<Entity<AcpThread>>)`

**Option B**: Allow multiple `SubagentThread` items in the `content` Vec

The implementation must support:
- Adding subagent threads one at a time as they're spawned
- Displaying all subagents in the tool call's expanded view
- Each subagent having independent expand/collapse state

### 2. Send Updates During Subagent Execution

Currently, no UI updates are sent while subagents run. The implementation must:

1. Call `event_stream.update_subagent_thread()` for each subagent immediately after creating its `AcpThread`
2. Ensure the UI updates in real-time as subagents are spawned
3. Show running state (spinner) for each subagent while it executes

### 3. Implement Subagent Card Rendering

In `AcpThreadView::render_tool_call_content()`, the `SubagentThread` match arm currently returns `Empty`. This must be replaced with proper rendering.

**Collapsed Card Requirements**:
- Status icon on left (blue spinner for running, green check for completed, red X for failed)
- Label text from the subagent's `label` field
- Em-dash separator
- File change stats: "N files changed" in muted text
- "+X" in green, "-Y" in red (line additions/deletions)
- Chevron to expand/collapse

**Expanded View Requirements**:
- Header with Zed Agent icon and label
- "Expand Subagent" button to open full thread in a new view/portal
- Collapse chevron
- Summary of what the subagent accomplished
- Scrollable preview of the subagent's conversation (optional)

### 4. Compute Diff Stats

Implement `compute_subagent_diff_stats()` function that:
- Reads the subagent's `ActionLog` 
- Calls `action_log.changed_buffers(cx)` to get modified buffers
- Counts files changed, lines added, lines removed from diff hunks
- Returns `SubagentDiffStats { files_changed, lines_added, lines_removed }`

### 5. Individual Expand/Collapse State

Each subagent card must have independent expand/collapse state:
- Add tracking (e.g., `expanded_subagents: HashSet<SubagentId>` or similar)
- Clicking a subagent card toggles only that card
- The parent tool call's expand/collapse is separate from individual subagents

## Visual Testing Requirements

**Visual tests are MANDATORY for this implementation.** All UI work must be verified through visual tests before being considered complete.

### Running Visual Tests

```bash
# Run visual tests and compare against baselines
cargo run -p zed --bin zed_visual_test_runner --features visual-tests --release

# Update/create baseline screenshots
UPDATE_BASELINE=1 cargo run -p zed --bin zed_visual_test_runner --features visual-tests --release
```

Screenshots are saved to `target/visual_tests/`.

### Required Visual Test Scenarios

Each scenario must have a visual test that captures screenshots:

1. **Single subagent running**: One subagent in progress with spinner
2. **Single subagent completed**: One subagent done with checkmark and diff stats
3. **Multiple subagents running**: 2-3 subagents all in progress
4. **Multiple subagents mixed state**: Some completed, some still running
5. **All subagents completed**: All done, showing individual summaries
6. **Subagent with file changes**: Shows "+X -Y" diff stats
7. **Expanded subagent card**: Individual subagent expanded showing details
8. **Failed subagent**: Shows error state with X icon

### How to Write Visual Tests

Visual tests use `StubAgentConnection` to provide pre-programmed responses:

```rust
let connection = StubAgentConnection::new();

// Create a subagent tool call
let tool_call = acp::ToolCall::new("subagent-1", "Researching alternatives")
    .kind(acp::ToolKind::Other)
    .meta(acp::Meta::from_iter([("tool_name".into(), "subagent".into())]))
    .status(acp::ToolCallStatus::InProgress);

connection.set_next_prompt_updates(vec![acp::SessionUpdate::ToolCall(tool_call)]);
```

To show file change stats, you must:
1. Create real files in a temp directory
2. Open them as buffers
3. Register with `ActionLog` via `buffer_read()`
4. Make edits to the buffer
5. Mark as edited via `buffer_edited()`

See `crates/zed/src/visual_test_runner.rs` and `plans/subagents/visual-tests.md` for detailed examples.

## Implementation Order

1. **First**: Modify data structures to support multiple subagent threads per tool call
2. **Second**: Re-add `update_subagent_thread()` calls in `SubagentTool::run()` for each subagent
3. **Third**: Implement basic collapsed card rendering with status icon and label
4. **Fourth**: Add expand/collapse functionality for individual subagent cards  
5. **Fifth**: Implement diff stats computation and display
6. **Sixth**: Implement expanded view with full details
7. **Throughout**: Write visual tests for each feature as it's implemented

## Files to Modify

- `crates/acp_thread/src/acp_thread.rs` - Add support for multiple subagent threads
- `crates/agent/src/tools/subagent_tool.rs` - Re-add `update_subagent_thread()` calls
- `crates/agent/src/thread.rs` - May need modifications to `ToolCallEventStream`
- `crates/agent_ui/src/acp/thread_view.rs` - Implement subagent card rendering
- `crates/zed/src/visual_test_runner.rs` - Add visual test scenarios

## Non-Negotiable Requirements

1. **No "for now" solutions** - Every feature must be fully implemented
2. **No simplified versions** - The UI must match the mockups exactly
3. **No skipped features** - All listed requirements are mandatory
4. **Visual tests required** - Every UI change must have corresponding visual tests
5. **No temporary workarounds** - If something is hard, solve it properly

## Reference: Mockup Description

The mockup shows:
- Multiple subagent cards stacked vertically within a single tool call
- Each card has: status icon, label, file change stats, expand chevron
- Cards can be individually expanded to show the subagent's work
- "Expand Subagent" button opens a portal/full view of that subagent's thread
- Running subagents show a spinning icon
- Completed subagents show checkmark and diff stats in git-style coloring

## Questions for Clarification

[To be filled in after review]
```

---

I have questions about this implementation plan:

1. **Multiple subagent UI layout**: Should each subagent be rendered as a completely separate tool call card in the thread (like how multiple parallel tool calls appear separately), or should they all be grouped under a single "Subagents" parent card with nested cards inside?

2. **"Expand Subagent" behavior**: When clicking "Expand Subagent", should it:
   - Open a new panel/pane with the full subagent thread?
   - Expand inline within the current thread view?
   - Open a modal/portal overlay?

3. **Real-time updates**: Should we show streaming text from each subagent as it runs (like we do for the main agent), or just show a spinner until the subagent completes?

4. **The mockup you showed**: The mockup image shows "Expand Subagent" button - is that from actual working code somewhere, or is that a design mockup? If it's working code, where can I find it?

5. **Subagent identification**: How should individual subagents be identified in the UI state tracking? By their `AcpThread` entity ID, by the label string, or by some other identifier?

6. **Error handling display**: If one subagent fails while others succeed, how should that be displayed? Should the failed one show an error message inline, or should there be a separate error state?

7. **File change stats scope**: Should the diff stats show changes made by just that one subagent, or cumulative changes across all subagents in the tool call?