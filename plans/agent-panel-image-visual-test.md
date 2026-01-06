# Visual Test Plan: Agent Panel Image Rendering

## Goal

Create a visual test that:
1. Renders a full Zed workspace window
2. Displays an AcpThreadView containing a tool call with an image output
3. Captures a screenshot showing the image rendered in context
4. Uses deterministic stubbed responses (no real model calls)

## Implementation Approach: Direct AcpThreadView with Stubbed Server

Rather than modifying `AgentPanel` to inject a stubbed server (which would require invasive cross-cutting changes), we will:

1. Create an `AcpThreadView` directly with a stubbed `AgentServer`
2. Wrap it in an `Item` and add it to the workspace's active pane
3. Program the stub connection with an image tool call response
4. Send a user message to trigger the response
5. Capture the screenshot

This approach reuses existing test patterns from `crates/agent_ui/src/acp/thread_view.rs` and requires **no changes to production code paths**.

## Detailed Implementation Steps

### Phase 1: Feature Flag and Dependency Setup

#### 1.1 Update `crates/zed/Cargo.toml`

Add the required test-support features to the `visual-tests` feature:

```toml
visual-tests = [
    "gpui/test-support",
    "gpui/screen-capture",
    "dep:image",
    "dep:semver",
    "dep:tempfile",
    "workspace/test-support",
    "project/test-support",
    "editor/test-support",
    "terminal_view/test-support",
    "image_viewer/test-support",
    "clock/test-support",
    # Add these:
    "acp_thread/test-support",
    "assistant_text_thread/test-support",
    "agent_ui/test-support",  # If needed for any exports
]
```

Also add these dependencies if not already present:

```toml
[dependencies]
acp_thread.workspace = true
assistant_text_thread.workspace = true
agent.workspace = true
agent_servers.workspace = true
agent_client_protocol.workspace = true
base64.workspace = true
```

#### 1.2 Create the `StubAgentServer` helper

In `crates/zed/src/visual_test_runner.rs`, add this helper module (or create a separate `visual_test_helpers.rs` file):

```rust
#[cfg(feature = "visual-tests")]
mod visual_test_helpers {
    use std::any::Any;
    use std::path::Path;
    use std::rc::Rc;
    use acp_thread::{AgentConnection, StubAgentConnection};
    use agent_servers::{AgentServer, AgentServerDelegate};
    use gpui::{App, SharedString, Task};
    
    #[derive(Clone)]
    pub struct StubAgentServer {
        connection: StubAgentConnection,
    }
    
    impl StubAgentServer {
        pub fn new(connection: StubAgentConnection) -> Self {
            Self { connection }
        }
    }
    
    impl AgentServer for StubAgentServer {
        fn logo(&self) -> ui::IconName {
            ui::IconName::Ai
        }
        
        fn name(&self) -> SharedString {
            "Visual Test Agent".into()
        }
        
        fn connect(
            &self,
            _root_dir: Option<&Path>,
            _delegate: AgentServerDelegate,
            _cx: &mut App,
        ) -> Task<gpui::Result<(Rc<dyn AgentConnection>, Option<task::SpawnInTerminal>)>> {
            Task::ready(Ok((Rc::new(self.connection.clone()), None)))
        }
        
        fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }
    }
}
```

### Phase 2: Initialization Updates

#### 2.1 Add required initialization in `visual_test_runner.rs`

Add these initializations to the `Application::new().run()` block, after the existing initializations:

```rust
// Existing initializations...
theme::init(theme::LoadThemes::JustBase, cx);
// ... etc ...

// Add these for agent thread support:
prompt_store::init(cx);
```

These are needed because:
- `prompt_store::init(cx)` - Required by `AcpThreadView` for slash command support

**Not required** for this stub-based test:
- `language_model::init` / `language_models::init` - Only needed for real model calls
- `acp_tools::init` - Only needed for real tool execution  
- `agent_ui::init` - Only needed for full AgentPanel wiring

### Phase 3: Test Implementation

#### 3.1 Create the `ThreadViewItem` wrapper

This wraps `AcpThreadView` as a workspace `Item`:

```rust
use agent_ui::acp::AcpThreadView;
use workspace::Item;
use gpui::{Entity, EventEmitter, Focusable, FocusHandle, Render, Window, Context, IntoElement};

struct ThreadViewItem(Entity<AcpThreadView>);

impl Item for ThreadViewItem {
    type Event = ();
    
    fn include_in_nav_history() -> bool {
        false
    }
    
    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Agent Thread".into()
    }
}

impl EventEmitter<()> for ThreadViewItem {}

impl Focusable for ThreadViewItem {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.0.read(cx).focus_handle(cx)
    }
}

impl Render for ThreadViewItem {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.0.clone().into_any_element()
    }
}
```

#### 3.2 Create the image test helper function

```rust
fn create_test_png_base64(width: u32, height: u32, color: [u8; 4]) -> String {
    use image::ImageEncoder as _;
    use image::EncodableLayout as _;
    
    let mut png_data = Vec::new();
    {
        let encoder = image::codecs::png::PngEncoder::new(&mut png_data);
        let mut pixels = Vec::with_capacity((width * height * 4) as usize);
        for _ in 0..(width * height) {
            pixels.extend_from_slice(&color);
        }
        encoder
            .write_image(&pixels, width, height, image::ExtendedColorType::Rgba8)
            .expect("Failed to encode PNG");
    }
    
    base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        png_data.as_bytes(),
    )
}
```

#### 3.3 Add the test case in the async block

After the existing tests, add:

```rust
// Run Test 3: Agent Thread with Image
println!("\n--- Test 3: agent_thread_with_image ---");

// 1. Create the stub connection with an image response
let connection = acp_thread::StubAgentConnection::new();
let red_png_base64 = create_test_png_base64(64, 64, [255, 0, 0, 255]);

connection.set_next_prompt_updates(vec![
    agent_client_protocol::SessionUpdate::ToolCall(
        agent_client_protocol::ToolCall::new("read_image", "Read image file")
            .kind(agent_client_protocol::ToolKind::Fetch)
            .status(agent_client_protocol::ToolCallStatus::Completed)
            .content(vec![agent_client_protocol::ToolCallContent::Content(
                agent_client_protocol::Content::new(
                    agent_client_protocol::ContentBlock::Image(
                        agent_client_protocol::ImageContent::new(red_png_base64, "image/png"),
                    ),
                ),
            )]),
    ),
]);

let stub_agent = std::rc::Rc::new(visual_test_helpers::StubAgentServer::new(connection.clone()));

// 2. Create the TextThreadStore and HistoryStore
let text_thread_store = cx.update(|cx| {
    cx.new(|cx| assistant_text_thread::TextThreadStore::fake(project.clone(), cx))
})?;

let history_store = cx.update(|cx| {
    cx.new(|cx| agent::HistoryStore::new(text_thread_store.clone(), cx))
})?;

// 3. Create the AcpThreadView
let thread_view = workspace_window.update(&mut cx, |workspace, window, cx| {
    let weak_workspace = workspace.weak_handle();
    cx.new(|cx| {
        agent_ui::acp::AcpThreadView::new(
            stub_agent,
            None,  // resume_thread
            None,  // summarize_thread
            weak_workspace,
            project.clone(),
            history_store.clone(),
            None,  // prompt_store
            false, // track_load_event
            window,
            cx,
        )
    })
})?;

// Wait for thread view to initialize
cx.run_until_parked();

// 4. Add the thread view to the workspace
workspace_window.update(&mut cx, |workspace, window, cx| {
    workspace.add_item_to_active_pane(
        Box::new(cx.new(|_| ThreadViewItem(thread_view.clone()))),
        None,
        true,
        window,
        cx,
    );
})?;

// Wait for pane to update
cx.background_executor()
    .timer(std::time::Duration::from_millis(500))
    .await;

// 5. Get the thread and send a message to trigger the image response
let thread = thread_view.read_with(&cx, |view, _| view.thread().cloned())?
    .ok_or_else(|| anyhow::anyhow!("Thread not available"))?;

thread.update(&mut cx, |thread, cx| {
    thread.send_raw("Show me the image", cx)
})?.await?;

// Wait for response to be processed and rendered
cx.run_until_parked();
cx.background_executor()
    .timer(std::time::Duration::from_secs(1))
    .await;

// 6. Refresh the window
cx.refresh()?;
cx.background_executor()
    .timer(std::time::Duration::from_millis(500))
    .await;

// 7. Capture and compare screenshot
let test_result = run_visual_test(
    "agent_thread_with_image",
    workspace_window.into(),
    &mut cx,
    update_baseline,
).await;

match test_result {
    Ok(TestResult::Passed) => {
        println!("✓ agent_thread_with_image: PASSED");
        passed += 1;
    }
    Ok(TestResult::BaselineUpdated(path)) => {
        println!("✓ agent_thread_with_image: Baseline updated at {}", path.display());
        updated += 1;
    }
    Err(e) => {
        eprintln!("✗ agent_thread_with_image: FAILED - {}", e);
        failed += 1;
    }
}
```

### Phase 4: Baseline Image

#### 4.1 Generate initial baseline

Run once with `UPDATE_BASELINE=1`:
```bash
UPDATE_BASELINE=1 cargo run -p zed --bin visual_test_runner --features visual-tests
```

This will create: `crates/zed/test_fixtures/visual_tests/agent_thread_with_image.png`

### File Changes Summary

| File | Change Type | Description |
|------|-------------|-------------|
| `crates/zed/Cargo.toml` | Modify | Add `acp_thread/test-support`, `assistant_text_thread/test-support` to `visual-tests` feature; add dependencies |
| `crates/zed/src/visual_test_runner.rs` | Modify | Add `StubAgentServer` helper, `ThreadViewItem` wrapper, `create_test_png_base64`, and new test case |
| `crates/zed/test_fixtures/visual_tests/agent_thread_with_image.png` | Create | Baseline screenshot |

### Required Imports in visual_test_runner.rs

```rust
// Add these imports
use std::rc::Rc;
use acp_thread::StubAgentConnection;
use agent_client_protocol as acp;
use assistant_text_thread::TextThreadStore;
use agent::HistoryStore;
use agent_ui::acp::AcpThreadView;
```

### Public API Requirements

The following items must be publicly accessible:
- `acp_thread::StubAgentConnection` - Currently `pub` under `#[cfg(feature = "test-support")]` ✓
- `assistant_text_thread::TextThreadStore::fake` - Currently `pub` under `#[cfg(any(test, feature = "test-support"))]` ✓
- `agent::HistoryStore` - Already public ✓
- `agent_ui::acp::AcpThreadView` - Need to verify this is exported in `agent_ui`'s public API

If `AcpThreadView` is not publicly exported, add to `crates/agent_ui/src/lib.rs`:
```rust
pub mod acp;
```

Or specifically:
```rust
pub use acp::AcpThreadView;
```

### Execution Checklist

1. [ ] Update `crates/zed/Cargo.toml` with new features and dependencies
2. [ ] Verify `AcpThreadView` is exported from `agent_ui`
3. [ ] Add helper module `visual_test_helpers` in `visual_test_runner.rs`
4. [ ] Add `ThreadViewItem` wrapper struct
5. [ ] Add `create_test_png_base64` function
6. [ ] Add `prompt_store::init(cx)` to initialization
7. [ ] Add Test 3 case with image thread
8. [ ] Run with `UPDATE_BASELINE=1` to generate baseline
9. [ ] Commit baseline image
10. [ ] Run without `UPDATE_BASELINE` to verify test passes

### Timeline Estimate

- Phase 1 (Cargo.toml + exports): 30 minutes
- Phase 2 (Initialization): 15 minutes
- Phase 3 (Test implementation): 1-2 hours
- Phase 4 (Baseline + debugging): 30 minutes

**Total: ~2-3 hours**

### Success Criteria

The visual test passes when:
1. A Zed workspace is rendered
2. An `AcpThreadView` is visible in the active pane
3. The thread shows:
   - A user message ("Show me the image")
   - A tool call output with a visible red 64×64 image
4. The screenshot matches the baseline within 99% threshold

### Alternative: Full AgentPanel Integration (Not Recommended)

If you need to test the complete AgentPanel chrome (history panel, model selector, etc.), you would need:

1. Add `agent_ui::init(fs, client, prompt_builder, languages, is_eval, cx)` 
2. Modify `AgentPanel` to accept an optional `AgentServer` factory
3. Use `AgentPanel::load()` with the custom factory

This approach is significantly more complex and requires modifying production code. Only pursue if the simpler `AcpThreadView` approach doesn't meet requirements.

---

## Phase 5: Production Image Support (Remaining Work)

The visual test above uses a stubbed response that directly provides image content. However, in production, the `read_file` tool does NOT correctly render images in the agent panel. There are two issues to fix:

### Issue 1: `read_file` Tool Description is Misleading

**Current state:** The `read_file` tool's doc comment says it's for reading file content, but doesn't mention image support. The model (Claude) incorrectly states it only works with "text-based files like code, configuration files, and documentation."

**Location:** `crates/agent/src/tools/read_file_tool.rs` lines 17-22

```rust
/// Reads the content of the given file in the project.
///
/// - Never attempt to read a path that hasn't been previously mentioned.
/// - For large files, this tool returns a file outline with symbol names and line numbers instead of the full content.
///   This outline IS a successful response - use the line numbers to read specific sections with start_line/end_line.
///   Do NOT retry reading the same file without line numbers if you receive an outline.
```

**Fix needed:** Update the doc comment to explain:
1. The tool supports both text and image files
2. Supported image formats (from `Img::extensions()` minus SVG):
   - avif, jpg, jpeg, png, gif, webp, tif, tiff, tga, dds, bmp, ico, hdr, exr, pbm, pam, ppm, pgm, ff, farbfeld, qoi
3. Image files are returned as base64-encoded image data that the model can analyze
4. The extension-to-mime-type mapping used:
   - `.png` → `image/png`
   - `.jpg`, `.jpeg` → `image/jpeg`
   - `.webp` → `image/webp`
   - `.gif` → `image/gif`
   - `.bmp` → `image/bmp`
   - `.tiff`, `.tif` → `image/tiff`
   - `.ico` → `image/ico`

### Issue 2: Image Output Not Rendered in UI

**Current state:** When `read_file` reads an image file, the tool output shows JSON with base64 data instead of rendering the actual image. This is because the tool only sends `acp::ToolCallContent::Content` for text results, not for images.

**Location:** `crates/agent/src/tools/read_file_tool.rs` lines 282-291

```rust
if let Ok(LanguageModelToolResultContent::Text(text)) = &result {
    let markdown = MarkdownCodeBlock {
        tag: &input.path,
        text,
    }
    .to_string();
    event_stream.update_fields(ToolCallUpdateFields::new().content(vec![
        acp::ToolCallContent::Content(acp::Content::new(markdown)),
    ]));
}
```

**The problem:** This code only handles `LanguageModelToolResultContent::Text`. When the result is `LanguageModelToolResultContent::Image`, no content is sent to the event stream, so the UI falls back to showing the `raw_output` JSON.

**Fix needed:** Add a branch to handle images:

```rust
match &result {
    Ok(LanguageModelToolResultContent::Text(text)) => {
        let markdown = MarkdownCodeBlock {
            tag: &input.path,
            text,
        }
        .to_string();
        event_stream.update_fields(ToolCallUpdateFields::new().content(vec![
            acp::ToolCallContent::Content(acp::Content::new(markdown)),
        ]));
    }
    Ok(LanguageModelToolResultContent::Image(image)) => {
        // Send the image as content so the UI can render it
        event_stream.update_fields(ToolCallUpdateFields::new().content(vec![
            acp::ToolCallContent::Content(acp::Content::new(
                acp::ContentBlock::Image(acp::ImageContent::new(
                    image.source.clone(),
                    "image/png", // The image is always converted to PNG format
                )),
            )),
        ]));
    }
    Err(_) => {}
}
```

**Note on mime type:** The `LanguageModelImage` struct doesn't store the original mime type. Images are processed through `LanguageModelImage::from_image()` which re-encodes them. Check what format this outputs - it may always be PNG, or it may preserve the original format. The mime type passed to `ImageContent::new()` must match the actual encoding.

### UI Rendering (Already Implemented)

The UI already has code to render images in tool call output:

**Location:** `crates/agent_ui/src/acp/thread_view.rs` lines 3064-3065, 3124-3148

```rust
} else if let Some(image) = content.image() {
    self.render_image_output(image.clone(), card_layout, cx)
```

And the `render_image_output` function properly renders the image using:

```rust
img(image)
    .max_w_96()
    .max_h_96()
    .object_fit(ObjectFit::ScaleDown)
```

This code path is exercised by the visual test (which uses stubbed data), but is never reached in production because the `read_file` tool doesn't send image content blocks.

### Implementation Checklist for Production Image Support

1. [ ] Update `read_file` tool description to mention image support and list supported formats
2. [ ] Modify `read_file_tool.rs` to send `acp::ToolCallContent::Content` with image data when result is `LanguageModelToolResultContent::Image`
3. [ ] Verify the mime type matches the actual image encoding (check `LanguageModelImage::from_image()`)
4. [ ] Test with actual image files in the agent panel
5. [ ] Consider adding a unit test that verifies image content is sent correctly

### Files to Modify

| File | Change |
|------|--------|
| `crates/agent/src/tools/read_file_tool.rs` | Update doc comment; add image handling in `run()` |

### Estimated Time

- Doc comment update: 15 minutes
- Image content sending fix: 30 minutes  
- Testing and verification: 30 minutes

**Total: ~1-1.5 hours**
