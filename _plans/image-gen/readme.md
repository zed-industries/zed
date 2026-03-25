# Image Generator Panel - Important Context

## Critical Information

- The Google AI API key is already configured by the user (visible in their screenshot showing "API key configured" under Google AI provider)
- All three models use the same `generateContent` endpoint — NOT the `predict` endpoint used by older Imagen models
- The `google_ai` crate currently only has `stream_generate_content()` (SSE-based). Image generation needs a non-streaming variant because the response contains the full base64 image inline
- GPUI requires BGRA pixel format, not RGBA. The `image` crate produces RGBA. Must swap channels (R and B) before creating `RenderImage`

## Caveats

- The `GenerationConfig` struct in `google_ai` currently has NO `response_modalities` or `image_config` fields — these must be added with `#[serde(skip_serializing_if = "Option::is_none")]` to avoid breaking existing text-only requests
- The `validate_generate_content_request()` function may need adjustment if image-only requests don't have user content parts in the same way
- Model IDs with `-preview` suffix may change as Google promotes models out of preview
- The `GoogleLanguageModelProvider::api_key_for_gemini_cli()` method first checks env vars (`GEMINI_API_KEY`, `GOOGLE_AI_API_KEY`) then falls back to system keychain
- When running dev Zed from a Claude Code session, must use `env -u CLAUDECODE` to avoid terminal issues (from user memory)

## Dependencies

- **google_ai crate**: `crates/google_ai/src/google_ai.rs` — types and HTTP functions
- **language_models crate**: `crates/language_models/src/provider/google.rs` — API key access
- **workspace crate**: `crates/workspace/src/dock.rs` — Panel trait and dock system
- **image crate**: For PNG decoding and RGBA→BGRA conversion
- **base64 crate**: For decoding the API response image data
- **gpui**: `RenderImage`, `img()` element, `Entity`, `Context`, etc.

## Testing Notes

1. Build: `cargo build -p image_gen_panel`
2. Run: `env -u CLAUDECODE ZED_RELEASE_CHANNEL=nightly cargo run`
3. Look for Image icon in left dock button bar
4. Click to open panel
5. Type a prompt (e.g., "a red cat sitting on a blue chair")
6. Select aspect ratio
7. Click Generate
8. Wait for image to appear below
9. Try different models and aspect ratios

## Known Limitations

- No image saving/exporting (future work)
- No image history (future work)
- No image editing/inpainting (future work)
- Single image per generation (models return 1 image via generateContent)
- No progress indicator beyond disabled button state
- Panel doesn't persist settings (model/aspect ratio selection) across restarts

## Related Files

- Plan file: `/Users/alesloas/.claude/plans/snuggly-swinging-rivest.md`
- Existing image rendering example: `crates/repl/src/outputs/image.rs`
- Google provider: `crates/language_models/src/provider/google.rs`
- Panel trait: `crates/workspace/src/dock.rs:96-128`
- Panel registration: `crates/zed/src/zed.rs:643-691`
- Icon enum: `crates/icons/src/icons.rs` (line 144 = `Image`)

## Recovery Instructions

If resuming after compaction:
1. Read this file first for caveats and gotchas
2. Check `task.md` for current progress
3. Read `plans.md` for full technical context with code snippets
4. Check `git status` for any uncommitted work
5. Key files to re-read: `crates/google_ai/src/google_ai.rs` (GenerationConfig struct ~line 303), `crates/zed/src/zed.rs` (initialize_panels ~line 643)
