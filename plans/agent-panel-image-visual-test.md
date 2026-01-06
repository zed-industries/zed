# Visual Test Plan: Agent Panel Image Rendering

## 🎯 The Goal

We want a visual regression test that **catches bugs in how `read_file` displays images**. 

If someone changes the code in `ReadFileTool` or the UI rendering in `thread_view.rs`, this test should fail and show us visually what changed.

## ⚠️ Current Problem: The Test is Useless

**The current test in `crates/zed/src/visual_test_runner.rs` does NOT test the real code!**

Here's what it does now (WRONG):
1. Creates a `StubAgentConnection` 
2. Hard-codes a fake tool call response with pre-baked image data
3. Injects that directly into `AcpThread`
4. Takes a screenshot

**Why this is useless:** If you change how `ReadFileTool` produces its output (in `crates/agent/src/tools/read_file_tool.rs`), the test will still pass because it never runs that code! The test bypasses the entire tool execution pipeline.

## ✅ What We Actually Need

The test should:
1. Create a real project with a real image file
2. Actually run the real `ReadFileTool::run()` method
3. Let the tool produce its real output via `event_stream.update_fields()`
4. Have that real output flow through to `AcpThread` and render in the UI
5. Take a screenshot of the real rendered result

This way, if someone changes `ReadFileTool` or the UI rendering, the test will catch it.

## 📚 Architecture Background (For Newcomers)

Here's how the agent system works:

### The Two "Thread" Types
- **`Thread`** (in `crates/agent/src/thread.rs`) - Runs tools, talks to LLMs, produces events
- **`AcpThread`** (in `crates/acp_thread/src/acp_thread.rs`) - Receives events and stores data for UI rendering

### How Tools Work
1. `Thread` has registered tools (like `ReadFileTool`)
2. When a tool runs, it gets a `ToolCallEventStream`
3. The tool calls `event_stream.update_fields(...)` to send updates
4. Those updates become `ThreadEvent::ToolCallUpdate` events
5. Events flow to `AcpThread` via `handle_thread_events()` in `NativeAgentConnection`
6. `AcpThread` stores the data and the UI renders it

### The Key File Locations
- **Tool implementation:** `crates/agent/src/tools/read_file_tool.rs`
  - Lines 163-188: Image file handling (calls `event_stream.update_fields()`)
- **Event stream:** `crates/agent/src/thread.rs` 
  - `ToolCallEventStream::update_fields()` - sends updates
  - `ToolCallEventStream::test()` - creates a test event stream
- **UI rendering:** `crates/agent_ui/src/acp/thread_view.rs`
  - `render_image_output()` - renders images in tool call output
- **Current (broken) test:** `crates/zed/src/visual_test_runner.rs`
  - `run_agent_thread_view_test()` - the function that needs fixing

## 🔧 Implementation Plan

### Option A: Direct Tool Invocation (Recommended)

Run the real tool and capture its output:

```rust
// 1. Create a project with a real image file
let fs = FakeFs::new(cx.executor());
fs.insert_file("/project/test-image.png", EMBEDDED_TEST_IMAGE.to_vec()).await;
let project = Project::test(fs.clone(), ["/project"], cx).await;

// 2. Create the ReadFileTool (needs Thread, ActionLog)
let action_log = cx.new(|_| ActionLog::new(project.clone()));
// ... create Thread with project ...
let tool = Arc::new(ReadFileTool::new(thread.downgrade(), project.clone(), action_log));

// 3. Run the tool and capture events
let (event_stream, mut event_receiver) = ToolCallEventStream::test();
let input = ReadFileToolInput {
    path: "project/test-image.png".to_string(),
    start_line: None,
    end_line: None,
};
tool.run(input, event_stream, cx).await?;

// 4. Collect the ToolCallUpdateFields that the tool produced
let updates = event_receiver.collect_updates();

// 5. Create an AcpThread and inject the real tool output
// ... create AcpThread ...
acp_thread.update(cx, |thread, cx| {
    // First create the tool call entry
    thread.upsert_tool_call(initial_tool_call, cx)?;
    // Then update it with the real output from the tool
    for update in updates {
        thread.update_tool_call(update, cx)?;
    }
})?;

// 6. Render and screenshot
```

### Required Exports

The `agent` crate needs to export these for the visual test:
- `ReadFileTool` and `ReadFileToolInput`
- `ToolCallEventStream::test()` (already has `#[cfg(feature = "test-support")]`)
- `Thread` (to create the tool)

Check `crates/agent/src/lib.rs` and add exports if needed.

### Required Dependencies in `crates/zed/Cargo.toml`

The `visual-tests` feature needs:
```toml
"agent/test-support"  # For ToolCallEventStream::test() and tool exports
```

### Option B: Use NativeAgentConnection with Fake Model

Alternatively, use the full agent flow with a fake LLM:

1. Create `NativeAgentServer` with a `FakeLanguageModel`
2. Program the fake model to return a tool call for `read_file`
3. Let the real agent flow execute the tool
4. The tool runs, produces output, flows through to UI

This is more complex but tests more of the real code path.

## 📋 Step-by-Step Implementation Checklist

### Phase 1: Enable Tool Access
- [x] Add `agent/test-support` to `visual-tests` feature in `crates/zed/Cargo.toml`
- [x] Verify `ReadFileTool`, `ReadFileToolInput`, `ToolCallEventStream::test()` are exported
- [x] Added additional required features: `language_model/test-support`, `fs/test-support`, `action_log`

### Phase 2: Rewrite the Test
- [x] In `run_agent_thread_view_test()`, remove the fake stub response
- [x] Create a real temp directory with a real image file (FakeFs doesn't work in visual test runner)
- [x] Create the real `ReadFileTool` with Thread, ActionLog, etc.
- [x] Run the tool with `ToolCallEventStream::test()`
- [x] Capture the `ToolCallUpdateFields` it produces
- [x] Use the real tool output to populate the stub connection's response

### Phase 3: Verify It Works
- [x] Run `UPDATE_BASELINE=1 cargo run -p zed --bin visual_test_runner --features visual-tests`
- [x] Check the screenshot shows the real tool output
- [x] Intentionally break `read_file_tool.rs` (comment out `event_stream.update_fields`)
- [x] Verified the test fails with: "ReadFileTool did not produce any content - the tool is broken!"
- [x] Restored the code and verified test passes again

## 🧪 How to Verify the Test is Actually Testing Real Code

After implementing, do this sanity check:

1. In `crates/agent/src/tools/read_file_tool.rs`, comment out lines 181-185:
   ```rust
   // event_stream.update_fields(ToolCallUpdateFields::new().content(vec![
   //     acp::ToolCallContent::Content(acp::Content::new(acp::ContentBlock::Image(
   //         acp::ImageContent::new(language_model_image.source.clone(), "image/png"),
   //     ))),
   // ]));
   ```

2. Run the visual test - it should FAIL or produce a visibly different screenshot

3. Restore the code - test should pass again

If commenting out the real tool code doesn't affect the test, the test is still broken!

## 📁 Files Modified

| File | Change |
|------|--------|
| `crates/zed/Cargo.toml` | Added `agent/test-support`, `language_model/test-support`, `fs/test-support`, `action_log` to `visual-tests` feature |
| `crates/zed/src/visual_test_runner.rs` | Rewrote `run_agent_thread_view_test()` to run the real `ReadFileTool` and capture its output |

Note: No changes needed to `crates/agent/src/lib.rs` - all necessary exports were already public.

## ✅ Already Completed (Don't Redo These)

These changes have already been made and are working:

1. **`read_file` tool sends image content** - `crates/agent/src/tools/read_file_tool.rs` now calls `event_stream.update_fields()` with image content blocks (lines 181-185)

2. **UI renders images** - `crates/agent_ui/src/acp/thread_view.rs` has `render_image_output()` that shows dimensions ("512×512 PNG") and a "Go to File" button

3. **Image tool calls auto-expand** - The UI automatically expands tool calls that return images

4. **Visual test infrastructure exists** - The test runner, baseline comparison, etc. all work

The only thing broken is that the test doesn't actually run the real tool code!

## 🔗 Related Code References

- Tool implementation: [read_file_tool.rs](file:///Users/rtfeldman/code/zed5/crates/agent/src/tools/read_file_tool.rs)
- Event stream: [thread.rs lines 2501-2596](file:///Users/rtfeldman/code/zed5/crates/agent/src/thread.rs#L2501-L2596)
- UI rendering: [thread_view.rs render_image_output](file:///Users/rtfeldman/code/zed5/crates/agent_ui/src/acp/thread_view.rs#L3146-L3217)
- Current test: [visual_test_runner.rs run_agent_thread_view_test](file:///Users/rtfeldman/code/zed5/crates/zed/src/visual_test_runner.rs#L778-L943)
