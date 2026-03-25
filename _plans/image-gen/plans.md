# Image Generator Panel - Implementation Plan

## Status: IN_PROGRESS
## Last Updated: 2026-03-22

## Goal
Add a left dock panel to Zed that generates images using Google Gemini image generation models (Nano Banana family), with model selection, prompt input, aspect ratio picker, and inline image display.

## Context
The user already has a Google AI API key configured in Zed's settings. We need to leverage the existing `google_ai` crate's types and HTTP patterns, extend them for image generation (non-streaming `generateContent` with `responseModalities` and `imageConfig`), and build a new panel crate that integrates into the workspace dock system.

## Technical Approach

### Three-step implementation:
1. **Extend `google_ai` crate** — Add image generation fields to `GenerationConfig` and a non-streaming `generate_content()` function
2. **Create `image_gen_panel` crate** — New panel with UI and generation logic
3. **Wire up in `zed` crate** — Register the panel in `initialize_panels()`

### Models supported (all use `generateContent` endpoint):
- `gemini-2.5-flash-image` — Nano Banana (fastest, cheapest)
- `gemini-3.1-flash-image-preview` — Nano Banana 2 (fast, 4K, search grounding)
- `gemini-3-pro-image-preview` — Nano Banana 2 Pro (highest quality, studio-grade)

### API endpoint:
```
POST https://generativelanguage.googleapis.com/v1beta/models/{model_id}:generateContent?key={api_key}
```

### Request body shape:
```json
{
  "contents": [{"parts": [{"text": "prompt"}], "role": "user"}],
  "generationConfig": {
    "responseModalities": ["IMAGE"],
    "imageConfig": {
      "aspectRatio": "1:1"
    }
  }
}
```

### Response shape (image part):
```json
{
  "candidates": [{
    "content": {
      "parts": [{
        "inlineData": {
          "mimeType": "image/png",
          "data": "<base64-encoded-png>"
        }
      }]
    }
  }]
}
```

## Files to Create/Modify

| Action | File | Purpose |
|--------|------|---------|
| EDIT | `crates/google_ai/src/google_ai.rs` | Add `response_modalities`, `image_config` to `GenerationConfig`; add `ImageConfig` struct; add non-streaming `generate_content()` function |
| CREATE | `crates/image_gen_panel/Cargo.toml` | New crate manifest |
| CREATE | `crates/image_gen_panel/src/image_gen_panel.rs` | Panel implementation (struct, Panel trait, Render, generation logic) |
| EDIT | `crates/zed/src/zed.rs` | Import and register `ImageGenPanel` in `initialize_panels()` (~line 649) |
| EDIT | `crates/zed/Cargo.toml` | Add `image_gen_panel` dependency |
| EDIT | `Cargo.toml` (root workspace) | Add `image_gen_panel` to workspace members |

## Code Snippets

### Step 1: google_ai crate additions

**New struct (add near other config structs around line 303):**
```rust
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
}
```

**Add to `GenerationConfig` (around line 305):**
```rust
#[serde(skip_serializing_if = "Option::is_none")]
pub response_modalities: Option<Vec<String>>,
#[serde(skip_serializing_if = "Option::is_none")]
pub image_config: Option<ImageConfig>,
```

**Non-streaming generate_content function (add after stream_generate_content):**
```rust
pub async fn generate_content(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    mut request: GenerateContentRequest,
) -> Result<GenerateContentResponse> {
    let api_key = api_key.trim();
    validate_generate_content_request(&request)?;
    let model_id = mem::take(&mut request.model.model_id);
    let uri = format!("{api_url}/v1beta/models/{model_id}:generateContent?key={api_key}");
    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json");
    let request = request_builder.body(AsyncBody::from(serde_json::to_string(&request)?))?;
    let mut response = client.send(request).await?;
    let mut text = String::new();
    response.body_mut().read_to_string(&mut text).await?;
    if response.status().is_success() {
        Ok(serde_json::from_str::<GenerateContentResponse>(&text)?)
    } else {
        Err(anyhow!(
            "error during generateContent, status code: {:?}, body: {}",
            response.status(),
            text
        ))
    }
}
```

### Step 2: image_gen_panel crate

**Cargo.toml:**
```toml
[package]
name = "image_gen_panel"
version = "0.1.0"
edition.workspace = true
publish.workspace = true
license = "GPL-3.0-or-later"

[lints]
workspace = true

[lib]
path = "src/image_gen_panel.rs"

[dependencies]
anyhow.workspace = true
base64.workspace = true
gpui.workspace = true
google_ai.workspace = true
http_client.workspace = true
image.workspace = true
language_model.workspace = true
language_models.workspace = true
credentials_provider.workspace = true
serde.workspace = true
serde_json.workspace = true
settings.workspace = true
ui.workspace = true
util.workspace = true
workspace.workspace = true
zed_env_vars.workspace = true
editor.workspace = true
```

**Panel struct:**
```rust
pub struct ImageGenPanel {
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    width: Option<Pixels>,
    prompt_editor: Entity<Editor>,
    selected_model: ImageModel,
    selected_aspect_ratio: AspectRatio,
    generated_image: Option<Arc<RenderImage>>,
    is_generating: bool,
    error_message: Option<String>,
    generation_task: Option<gpui::Task<()>>,
    http_client: Arc<dyn HttpClient>,
}
```

**Enums:**
```rust
#[derive(Clone, Copy, PartialEq)]
enum ImageModel {
    NanoBanana,      // gemini-2.5-flash-image
    NanoBanana2,     // gemini-3.1-flash-image-preview
    NanoBanana2Pro,  // gemini-3-pro-image-preview
}

impl ImageModel {
    fn model_id(&self) -> &'static str {
        match self {
            Self::NanoBanana => "gemini-2.5-flash-image",
            Self::NanoBanana2 => "gemini-3.1-flash-image-preview",
            Self::NanoBanana2Pro => "gemini-3-pro-image-preview",
        }
    }
    fn display_name(&self) -> &'static str {
        match self {
            Self::NanoBanana => "Nano Banana",
            Self::NanoBanana2 => "Nano Banana 2",
            Self::NanoBanana2Pro => "NB2 Pro",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum AspectRatio {
    Square,    // "1:1"
    Landscape, // "4:3"
    Portrait,  // "3:4"
    Wide,      // "16:9"
    Tall,      // "9:16"
}

impl AspectRatio {
    fn api_value(&self) -> &'static str {
        match self {
            Self::Square => "1:1",
            Self::Landscape => "4:3",
            Self::Portrait => "3:4",
            Self::Wide => "16:9",
            Self::Tall => "9:16",
        }
    }
    fn label(&self) -> &'static str {
        match self {
            Self::Square => "1:1",
            Self::Landscape => "4:3",
            Self::Portrait => "3:4",
            Self::Wide => "16:9",
            Self::Tall => "9:16",
        }
    }
}
```

**Panel trait implementation:**
```rust
const IMAGE_GEN_PANEL_KEY: &str = "ImageGenPanel";

impl Panel for ImageGenPanel {
    fn persistent_name() -> &'static str { "ImageGenPanel" }
    fn panel_key() -> &'static str { IMAGE_GEN_PANEL_KEY }
    fn position(&self, _window: &Window, _cx: &App) -> DockPosition { DockPosition::Left }
    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }
    fn set_position(&mut self, _position: DockPosition, _window: &mut Window, _cx: &mut Context<Self>) {}
    fn size(&self, _window: &Window, _cx: &App) -> Pixels {
        self.width.unwrap_or(px(360.0))
    }
    fn set_size(&mut self, size: Option<Pixels>, _window: &mut Window, _cx: &mut Context<Self>) {
        self.width = size;
    }
    fn icon(&self, _window: &Window, _cx: &App) -> Option<IconName> { Some(IconName::Image) }
    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> { Some("Image Generator") }
    fn toggle_action(&self) -> Box<dyn Action> { Box::new(ToggleFocus) }
    fn activation_priority(&self) -> u32 { 10 }
}
```

**Load function pattern (from DebugPanel):**
```rust
pub async fn load(
    workspace: WeakEntity<Workspace>,
    mut cx: AsyncWindowContext,
) -> Result<Entity<Self>> {
    workspace.update_in(&mut cx, |workspace, window, cx| {
        Self::new(workspace, window, cx)
    })?
}
```

**Image decoding (from repl/src/outputs/image.rs pattern):**
```rust
fn decode_image(base64_data: &str) -> Result<Arc<RenderImage>> {
    use base64::{Engine as _, alphabet, engine::{DecodePaddingMode, GeneralPurpose, GeneralPurposeConfig}};
    let config = GeneralPurposeConfig::new()
        .with_encode_padding(false)
        .with_decode_padding_mode(DecodePaddingMode::Indifferent);
    let engine = GeneralPurpose::new(&alphabet::STANDARD, config);
    let bytes = engine.decode(base64_data.replace(&[' ', '\n', '\t', '\r'][..], ""))?;
    let format = image::guess_format(&bytes)?;
    let mut data = image::load_from_memory_with_format(&bytes, format)?.into_rgba8();
    // RGBA → BGRA (required by GPUI)
    for pixel in data.chunks_exact_mut(4) {
        pixel.swap(0, 2);
    }
    Ok(Arc::new(RenderImage::new(vec![image::Frame::new(data)])))
}
```

**API key retrieval:**
```rust
// Uses GoogleLanguageModelProvider::api_key_for_gemini_cli(cx)
// from crates/language_models/src/provider/google.rs:120
let api_key_task = GoogleLanguageModelProvider::api_key_for_gemini_cli(cx);
```

### Step 3: Registration in zed.rs

**Import (add near other panel imports ~line 24):**
```rust
use image_gen_panel::ImageGenPanel;
```

**In initialize_panels() (~line 649), add after debug_panel:**
```rust
let image_gen_panel = ImageGenPanel::load(workspace_handle.clone(), cx.clone());
```

**In futures::join! (~line 676), add:**
```rust
add_panel_when_ready(image_gen_panel, workspace_handle.clone(), cx.clone()),
```

## Integration Points

- **API key**: `GoogleLanguageModelProvider::api_key_for_gemini_cli(cx)` at `crates/language_models/src/provider/google.rs:120`
- **API URL**: `google_ai::API_URL` constant = `"https://generativelanguage.googleapis.com"` at `crates/google_ai/src/google_ai.rs:9`
- **HTTP client**: Obtained from workspace client or app context (`Arc<dyn HttpClient>`)
- **Panel system**: `workspace::dock::{Panel, PanelEvent, DockPosition}` at `crates/workspace/src/dock.rs:96`
- **Image rendering**: `gpui::{Image, RenderImage, img}` + `image::Frame`
- **Existing types reused**: `google_ai::{GenerateContentRequest, GenerateContentResponse, Content, Part, InlineDataPart, GenerativeContentBlob, Role, ModelName, GenerationConfig}`

## Decisions Made

1. **Non-streaming API**: Image generation returns complete response, not SSE stream. Added `generate_content()` alongside existing `stream_generate_content()`.
2. **Icon**: `IconName::Image` (not Sparkle, which is already used elsewhere).
3. **Three models**: User confirmed they want all three Nano Banana variants as selectable options.
4. **Panel position**: Left dock by default, also valid in Right dock.
5. **Prompt input**: Use `Editor` entity for multi-line text input (consistent with Zed patterns).
6. **Activation priority 10**: Low priority so it appears at the end of dock buttons.
