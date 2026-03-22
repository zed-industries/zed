# Session: Image Generator Panel - Research & Planning
Date: 2026-03-22

## Goal
Design and plan a new left dock panel in Zed that generates images using Google Gemini's image generation models (Nano Banana family). The panel needs: model selector (3 models), text prompt input, aspect ratio picker, generate button, and inline image display.

## Research Conducted

### 1. Zed Panel System
- **Panel trait**: `crates/workspace/src/dock.rs:96-128` — requires `Focusable`, `EventEmitter<PanelEvent>`, `Render`
- **Panel registration**: `crates/zed/src/zed.rs:643-691` — `initialize_panels()` function uses `add_panel_when_ready()` helper with `futures::join!`
- **Dock positions**: `DockPosition::Left | Bottom | Right` at `crates/workspace/src/dock.rs:285-290`
- **Reference panels**: DebugPanel (`crates/debugger_ui/src/debugger_panel.rs`), ProjectPanel, GitPanel, TerminalPanel

### 2. Google AI API Key Access
- **API key retrieval**: `GoogleLanguageModelProvider::api_key_for_gemini_cli(cx)` at `crates/language_models/src/provider/google.rs:120-134`
- **Env vars checked**: `GEMINI_API_KEY` (primary), `GOOGLE_AI_API_KEY` (fallback)
- **Keychain fallback**: Uses `CredentialsProvider` system for keychain storage
- **API URL**: `google_ai::API_URL = "https://generativelanguage.googleapis.com"` at `crates/google_ai/src/google_ai.rs:9`

### 3. Gemini Image Generation API
- **Endpoint**: `POST /v1beta/models/{model_id}:generateContent?key={api_key}`
- **Key request fields**: `generationConfig.responseModalities: ["IMAGE"]`, `generationConfig.imageConfig.aspectRatio: "1:1"`
- **Response**: Base64-encoded PNG in `candidates[0].content.parts[].inlineData.data`
- **Models confirmed**:
  - `gemini-2.5-flash-image` — Nano Banana (fastest)
  - `gemini-3.1-flash-image-preview` — Nano Banana 2 (fast, 4K)
  - `gemini-3-pro-image-preview` — Nano Banana 2 Pro (highest quality)

### 4. GPUI Image Rendering
- **Pattern source**: `crates/repl/src/outputs/image.rs:28-68`
- **Flow**: Base64 decode → `image::load_from_memory_with_format()` → RGBA→BGRA swap → `RenderImage::new(vec![Frame::new(data)])` → `img(render_image)`
- **Critical**: GPUI requires BGRA, `image` crate outputs RGBA — must swap R and B channels

### 5. Available Icons
- `IconName::Image` exists at `crates/icons/src/icons.rs:144`
- `IconName::Sparkle` exists but already used elsewhere

## Files Changed

### Created
- `_plans/image-gen/plans.md` — Full implementation plan with all code snippets, struct definitions, function signatures
- `_plans/image-gen/readme.md` — Caveats, dependencies, testing notes, recovery instructions
- `_plans/image-gen/task.md` — Granular task breakdown with checkboxes

### No source code modified yet
This session was entirely research and planning. No Rust source files were changed.

## Key Findings & Decisions

### Decision 1: Non-streaming API needed
**Issue**: The `google_ai` crate only has `stream_generate_content()` which uses SSE (`alt=sse`). Image generation returns the full base64 image in a single response.
**Solution**: Add a new `generate_content()` function that does a simple POST and reads the full JSON response body.

### Decision 2: Extend GenerationConfig, don't fork
**Issue**: `GenerationConfig` in `crates/google_ai/src/google_ai.rs:303-320` lacks `responseModalities` and `imageConfig` fields needed for image generation.
**Solution**: Add optional fields with `#[serde(skip_serializing_if = "Option::is_none")]` so existing text-only requests are unaffected.

### Decision 3: New crate, not embedded in existing
**Issue**: Where to put the panel code?
**Solution**: New `crates/image_gen_panel/` crate, following the same pattern as other panel crates (project_panel, debugger_ui, git_ui).

### Decision 4: Icon choice
**Issue**: User noted Sparkle icon already in use.
**Solution**: Use `IconName::Image` which exists in the icon enum.

### Decision 5: Use Editor entity for prompt input
**Issue**: Need multi-line text input in the panel.
**Solution**: Use Zed's `Editor` entity, consistent with how other panels handle text input.

## Current Status
- ✅ Research complete — panel system, API, image rendering all understood
- ✅ Plan approved by user
- ✅ Plan docs saved to `_plans/image-gen/`
- ⚠️ Implementation not started — next session should begin with Step 1 (extend google_ai crate)
- 📝 When running dev Zed, must use `env -u CLAUDECODE ZED_RELEASE_CHANNEL=nightly cargo run`
- 📝 The three Nano Banana models all use the same `generateContent` endpoint, just different model IDs
- 📝 Activation priority set to 10 (low) so the Image icon appears at the end of the dock button bar

## Resume Instructions
1. Read `_plans/image-gen/task.md` for current progress
2. Start with Step 1: Edit `crates/google_ai/src/google_ai.rs` to add `ImageConfig`, `response_modalities`, and `generate_content()`
3. Then Step 2: Create the `image_gen_panel` crate
4. Then Step 3: Wire up in `crates/zed/src/zed.rs`
5. Reference `_plans/image-gen/plans.md` for all code snippets
