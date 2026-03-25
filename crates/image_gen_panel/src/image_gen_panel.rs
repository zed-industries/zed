use anyhow::Context as _;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use editor::Editor;
use google_ai::{
    Content, GenerateContentRequest, GenerationConfig, GenerativeContentBlob, ImageConfig,
    InlineDataPart, ModelName, Part, Role, TextPart, ThinkingConfig, ThinkingLevel,
};
use gpui::{
    AnyElement, Animation, AnimationExt, App, ClipboardItem, Context, Entity, EventEmitter,
    ExternalPaths, FocusHandle, Focusable, RenderImage, StatefulInteractiveElement, Subscription,
    Task, img, pulsating_between, px,
};
use http_client::HttpClient;
use language_models::provider::google::GoogleLanguageModelProvider;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use ui::{
    Button, ButtonCommon, ButtonStyle, Clickable, ContextMenu, FluentBuilder, Icon, IconName,
    InteractiveElement, IntoElement, Label, LabelCommon, LabelSize, ParentElement, Render,
    SharedString, Styled, Tooltip, div, divider, h_flex, prelude::*, right_click_menu, v_flex,
};
use workspace::item::{Item, ItemEvent, TabContentParams};

// ── Models ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageModel {
    NanoBanana,
    NanoBanana2,
    NanoBanana2Pro,
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
            Self::NanoBanana => "NB",
            Self::NanoBanana2 => "NB2",
            Self::NanoBanana2Pro => "NB2 Pro",
        }
    }

    fn all() -> &'static [Self] {
        &[Self::NanoBanana, Self::NanoBanana2, Self::NanoBanana2Pro]
    }

    fn supported_aspect_ratios(&self) -> &'static [AspectRatio] {
        use AspectRatio::*;
        match self {
            Self::NanoBanana | Self::NanoBanana2Pro => &[
                Ratio1x1, Ratio4x3, Ratio3x4, Ratio16x9, Ratio9x16, Ratio3x2, Ratio2x3,
                Ratio5x4, Ratio4x5, Ratio21x9,
            ],
            Self::NanoBanana2 => &[
                Ratio1x1, Ratio4x3, Ratio3x4, Ratio16x9, Ratio9x16, Ratio3x2, Ratio2x3,
                Ratio5x4, Ratio4x5, Ratio21x9, Ratio4x1, Ratio1x4, Ratio8x1, Ratio1x8,
            ],
        }
    }

    fn supported_resolutions(&self) -> &'static [Resolution] {
        match self {
            Self::NanoBanana => &[],
            Self::NanoBanana2 | Self::NanoBanana2Pro => {
                &[Resolution::Res1K, Resolution::Res2K, Resolution::Res4K]
            }
        }
    }

    fn supports_resolution_picker(&self) -> bool {
        !self.supported_resolutions().is_empty()
    }

    fn supports_thinking(&self) -> bool {
        matches!(self, Self::NanoBanana2)
    }

    fn supported_thinking_levels(&self) -> &'static [ThinkingLevel] {
        match self {
            Self::NanoBanana2 => &[ThinkingLevel::Minimal, ThinkingLevel::High],
            _ => &[],
        }
    }

    fn max_reference_images(&self) -> usize {
        match self {
            Self::NanoBanana => 5,
            Self::NanoBanana2 | Self::NanoBanana2Pro => 14,
        }
    }
}

// ── Aspect Ratio ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AspectRatio {
    Ratio1x1,
    Ratio4x3,
    Ratio3x4,
    Ratio16x9,
    Ratio9x16,
    Ratio3x2,
    Ratio2x3,
    Ratio5x4,
    Ratio4x5,
    Ratio21x9,
    Ratio4x1,
    Ratio1x4,
    Ratio8x1,
    Ratio1x8,
}

impl AspectRatio {
    fn api_value(&self) -> &'static str {
        match self {
            Self::Ratio1x1 => "1:1",
            Self::Ratio4x3 => "4:3",
            Self::Ratio3x4 => "3:4",
            Self::Ratio16x9 => "16:9",
            Self::Ratio9x16 => "9:16",
            Self::Ratio3x2 => "3:2",
            Self::Ratio2x3 => "2:3",
            Self::Ratio5x4 => "5:4",
            Self::Ratio4x5 => "4:5",
            Self::Ratio21x9 => "21:9",
            Self::Ratio4x1 => "4:1",
            Self::Ratio1x4 => "1:4",
            Self::Ratio8x1 => "8:1",
            Self::Ratio1x8 => "1:8",
        }
    }

    fn display_name(&self) -> &'static str {
        self.api_value()
    }
}

// ── Resolution ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resolution {
    Res1K,
    Res2K,
    Res4K,
}

impl Resolution {
    fn api_value(&self) -> &'static str {
        match self {
            Self::Res1K => "1K",
            Self::Res2K => "2K",
            Self::Res4K => "4K",
        }
    }

    fn display_name(&self) -> &'static str {
        self.api_value()
    }
}

// ── History Manifest (disk persistence) ─────────────────────────────────────

#[derive(Serialize, Deserialize, Clone, Debug)]
struct HistoryManifestEntry {
    id: String,
    filename: String,
    prompt: String,
    model: String,
    aspect_ratio: String,
    resolution: Option<String>,
    timestamp: u64,
    width: u32,
    height: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SortOrder {
    NewestFirst,
    OldestFirst,
}

// ── Reference Image ─────────────────────────────────────────────────────────

struct ReferenceImage {
    render_image: Arc<RenderImage>,
    raw_bytes: Vec<u8>,
    mime_type: String,
    width: u32,
    height: u32,
}

// ── Storage helpers ─────────────────────────────────────────────────────────

fn storage_dir() -> PathBuf {
    paths::data_dir().join("image_gen")
}

fn images_dir() -> PathBuf {
    storage_dir().join("images")
}

fn manifest_path() -> PathBuf {
    storage_dir().join("history.json")
}

fn load_manifest() -> Vec<HistoryManifestEntry> {
    let path = manifest_path();
    match std::fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

fn save_manifest(entries: &[HistoryManifestEntry]) -> anyhow::Result<()> {
    let path = manifest_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(entries)?;
    std::fs::write(&path, json)?;
    Ok(())
}

fn save_image_to_disk(bytes: &[u8], id: &str) -> anyhow::Result<()> {
    let dir = images_dir();
    std::fs::create_dir_all(&dir)?;
    std::fs::write(dir.join(format!("{id}.png")), bytes)?;
    Ok(())
}

fn generate_id() -> String {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();
    let nanos = duration.subsec_nanos();
    format!("{secs}_{nanos:08x}")
}

// ── Image loading ───────────────────────────────────────────────────────────

fn load_image_from_bytes(bytes: &[u8]) -> anyhow::Result<(Arc<RenderImage>, u32, u32)> {
    let format = image::guess_format(bytes).context("Failed to detect image format")?;
    let mut data = image::load_from_memory_with_format(bytes, format)
        .context("Failed to load image")?
        .into_rgba8();
    for pixel in data.chunks_exact_mut(4) {
        pixel.swap(0, 2);
    }
    let width = data.width();
    let height = data.height();
    let render_image = Arc::new(RenderImage::new(vec![image::Frame::new(data)]));
    Ok((render_image, width, height))
}

fn mime_type_for_extension(ext: &str) -> Option<&'static str> {
    match ext {
        "png" => Some("image/png"),
        "jpg" | "jpeg" => Some("image/jpeg"),
        "webp" => Some("image/webp"),
        _ => None,
    }
}

// ── Panel ───────────────────────────────────────────────────────────────────

pub enum ImageGenEvent {
    TabUpdate,
}

pub struct ImageGenPanel {
    focus_handle: FocusHandle,
    prompt_editor: Entity<Editor>,
    search_editor: Entity<Editor>,
    selected_model: ImageModel,
    selected_aspect_ratio: AspectRatio,
    selected_resolution: Resolution,
    selected_thinking_level: Option<ThinkingLevel>,
    is_generating: bool,
    error_message: Option<String>,
    _generation_task: Option<Task<()>>,
    http_client: Arc<dyn HttpClient>,
    history_manifest: Vec<HistoryManifestEntry>,
    thumbnail_cache: HashMap<String, Arc<RenderImage>>,
    full_image_cache: HashMap<String, (Arc<RenderImage>, Vec<u8>)>,
    selected_history_id: Option<String>,
    sort_order: SortOrder,
    reference_images: Vec<ReferenceImage>,
    _subscriptions: Vec<Subscription>,
}

impl ImageGenPanel {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let prompt_editor = cx.new(|cx| Editor::auto_height(2, 8, window, cx));
        let search_editor = cx.new(|cx| {
            let mut editor = Editor::auto_height(1, 1, window, cx);
            editor.set_placeholder_text("Search prompts...", window, cx);
            editor
        });

        let mut subscriptions = Vec::new();
        subscriptions.push(cx.observe(&search_editor, |_this, _editor, cx| {
            cx.notify();
        }));

        let history_manifest = load_manifest();

        let mut thumbnail_cache = HashMap::new();
        for entry in &history_manifest {
            let image_path = images_dir().join(&entry.filename);
            if let Ok(bytes) = std::fs::read(&image_path) {
                if let Ok((render_image, _, _)) = load_image_from_bytes(&bytes) {
                    thumbnail_cache.insert(entry.id.clone(), render_image);
                }
            }
        }

        Self {
            focus_handle,
            prompt_editor,
            search_editor,
            selected_model: ImageModel::NanoBanana,
            selected_aspect_ratio: AspectRatio::Ratio1x1,
            selected_resolution: Resolution::Res1K,
            selected_thinking_level: None,
            is_generating: false,
            error_message: None,
            _generation_task: None,
            http_client,
            history_manifest,
            thumbnail_cache,
            full_image_cache: HashMap::new(),
            selected_history_id: None,
            sort_order: SortOrder::NewestFirst,
            reference_images: Vec::new(),
            _subscriptions: subscriptions,
        }
    }

    fn on_model_changed(&mut self) {
        let model = self.selected_model;
        let supported_ratios = model.supported_aspect_ratios();
        if !supported_ratios.contains(&self.selected_aspect_ratio) {
            self.selected_aspect_ratio = supported_ratios
                .first()
                .copied()
                .unwrap_or(AspectRatio::Ratio1x1);
        }
        let supported_resolutions = model.supported_resolutions();
        if !supported_resolutions.contains(&self.selected_resolution) {
            self.selected_resolution = supported_resolutions
                .first()
                .copied()
                .unwrap_or(Resolution::Res1K);
        }
        if model.supports_thinking() {
            if self.selected_thinking_level.is_none() {
                self.selected_thinking_level = Some(ThinkingLevel::High);
            }
        } else {
            self.selected_thinking_level = None;
        }
        let max_refs = model.max_reference_images();
        if self.reference_images.len() > max_refs {
            self.reference_images.truncate(max_refs);
        }
    }

    fn generate_image(&mut self, window: &mut gpui::Window, cx: &mut Context<Self>) {
        let prompt = self.prompt_editor.read(cx).text(cx);
        if prompt.trim().is_empty() {
            self.error_message = Some("Please enter a prompt".to_string());
            cx.notify();
            return;
        }

        self.is_generating = true;
        self.error_message = None;
        cx.notify();

        let model = self.selected_model;
        let model_id = model.model_id().to_string();
        let aspect_ratio = self.selected_aspect_ratio.api_value().to_string();
        let image_size = if model.supports_resolution_picker() {
            Some(self.selected_resolution.api_value().to_string())
        } else {
            None
        };
        let thinking_level = self.selected_thinking_level;
        let http_client = self.http_client.clone();
        let api_key_task = GoogleLanguageModelProvider::api_key_for_gemini_cli(cx);

        let ref_images: Vec<(String, Vec<u8>)> = self
            .reference_images
            .iter()
            .map(|r| (r.mime_type.clone(), r.raw_bytes.clone()))
            .collect();

        let prompt_clone = prompt;
        let model_id_clone = model_id.clone();
        let aspect_ratio_clone = aspect_ratio.clone();
        let image_size_clone = image_size.clone();

        let task = cx.spawn_in(window, async move |this, cx| {
            let result = async {
                let api_key = api_key_task.await.context(
                    "Failed to get Google AI API key. Configure in Settings > LLM Providers > Google AI.",
                )?;

                let mut parts: Vec<Part> = ref_images
                    .iter()
                    .map(|(mime, bytes)| {
                        Part::InlineDataPart(InlineDataPart {
                            inline_data: GenerativeContentBlob {
                                mime_type: mime.clone(),
                                data: STANDARD.encode(bytes),
                            },
                        })
                    })
                    .collect();
                parts.push(Part::TextPart(TextPart {
                    text: prompt_clone.clone(),
                }));

                let thinking_config = thinking_level.map(|level| ThinkingConfig {
                    thinking_budget: None,
                    thinking_level: Some(level),
                });

                let request = GenerateContentRequest {
                    model: ModelName {
                        model_id: model_id.clone(),
                    },
                    contents: vec![Content {
                        parts,
                        role: Role::User,
                    }],
                    system_instruction: None,
                    generation_config: Some(GenerationConfig {
                        candidate_count: None,
                        stop_sequences: None,
                        max_output_tokens: None,
                        temperature: None,
                        top_p: None,
                        top_k: None,
                        thinking_config,
                        response_modalities: Some(vec!["IMAGE".to_string()]),
                        image_config: Some(ImageConfig {
                            aspect_ratio: Some(aspect_ratio),
                            image_size,
                        }),
                    }),
                    safety_settings: None,
                    tools: None,
                    tool_config: None,
                };

                let response = google_ai::generate_content(
                    http_client.as_ref(),
                    google_ai::API_URL,
                    &api_key,
                    request,
                )
                .await
                .context("Image generation request failed")?;

                let candidates = response.candidates.context("No candidates in response")?;
                let candidate = candidates.first().context("Empty candidates list")?;

                let image_part = candidate
                    .content
                    .parts
                    .iter()
                    .find_map(|part| {
                        if let Part::InlineDataPart(InlineDataPart { inline_data }) = part {
                            Some(inline_data)
                        } else {
                            None
                        }
                    })
                    .context("No image data in response")?;

                let bytes = STANDARD
                    .decode(&image_part.data)
                    .context("Failed to decode base64 image data")?;

                let (render_image, width, height) = load_image_from_bytes(&bytes)?;

                anyhow::Ok((render_image, bytes, width, height))
            }
            .await;

            this.update_in(cx, |this, _window, cx| {
                this.is_generating = false;
                match result {
                    Ok((render_image, raw_bytes, width, height)) => {
                        let id = generate_id();
                        let filename = format!("{id}.png");
                        let timestamp = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs();

                        if let Err(err) = save_image_to_disk(&raw_bytes, &id) {
                            this.error_message =
                                Some(format!("Failed to save image to disk: {err:#}"));
                        }

                        let manifest_entry = HistoryManifestEntry {
                            id: id.clone(),
                            filename,
                            prompt: prompt_clone,
                            model: model_id_clone,
                            aspect_ratio: aspect_ratio_clone,
                            resolution: image_size_clone,
                            timestamp,
                            width,
                            height,
                        };

                        this.history_manifest.push(manifest_entry);
                        if let Err(err) = save_manifest(&this.history_manifest) {
                            this.error_message =
                                Some(format!("Failed to save history: {err:#}"));
                        }

                        this.thumbnail_cache
                            .insert(id.clone(), render_image.clone());
                        this.full_image_cache
                            .insert(id.clone(), (render_image, raw_bytes));
                        this.selected_history_id = Some(id);
                        if this.error_message.is_none() {
                            this.error_message = None;
                        }
                    }
                    Err(err) => {
                        this.error_message = Some(format!("{err:#}"));
                    }
                }
                cx.notify();
            })
            .ok();
        });

        self._generation_task = Some(task);
    }

    fn add_reference_from_path(&mut self, path: &std::path::Path, cx: &mut Context<Self>) {
        let max = self.selected_model.max_reference_images();
        if self.reference_images.len() >= max {
            self.error_message = Some(format!(
                "Max {} reference images for {}",
                max,
                self.selected_model.display_name()
            ));
            cx.notify();
            return;
        }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        let Some(mime_type) = mime_type_for_extension(&ext) else {
            self.error_message =
                Some("Unsupported image format. Use PNG, JPG, or WebP.".to_string());
            cx.notify();
            return;
        };

        let bytes = match std::fs::read(path) {
            Ok(b) => b,
            Err(err) => {
                self.error_message = Some(format!("Failed to read file: {err}"));
                cx.notify();
                return;
            }
        };

        match load_image_from_bytes(&bytes) {
            Ok((render_image, width, height)) => {
                self.reference_images.push(ReferenceImage {
                    render_image,
                    raw_bytes: bytes,
                    mime_type: mime_type.to_string(),
                    width,
                    height,
                });
                self.error_message = None;
            }
            Err(err) => {
                self.error_message = Some(format!("Failed to load image: {err}"));
            }
        }
        cx.notify();
    }

    fn clear_selected_image(&mut self, cx: &mut Context<Self>) {
        self.selected_history_id = None;
        cx.notify();
    }

    fn selected_manifest_entry(&self) -> Option<&HistoryManifestEntry> {
        self.selected_history_id.as_ref().and_then(|id| {
            self.history_manifest.iter().find(|e| e.id == *id)
        })
    }

    fn select_history_image(&mut self, id: &str, cx: &mut Context<Self>) {
        self.selected_history_id = Some(id.to_string());

        if !self.full_image_cache.contains_key(id) {
            if let Some(entry) = self.history_manifest.iter().find(|e| e.id == id) {
                let image_path = images_dir().join(&entry.filename);
                if let Ok(bytes) = std::fs::read(&image_path) {
                    if let Ok((render_image, _, _)) = load_image_from_bytes(&bytes) {
                        self.full_image_cache
                            .insert(id.to_string(), (render_image, bytes));
                    }
                }
            }
        }

        cx.notify();
    }

    fn use_selected_as_reference(&mut self, cx: &mut Context<Self>) {
        let max = self.selected_model.max_reference_images();
        if self.reference_images.len() >= max {
            self.error_message = Some(format!(
                "Max {} reference images for {}",
                max,
                self.selected_model.display_name()
            ));
            cx.notify();
            return;
        }

        let Some(id) = self.selected_history_id.clone() else {
            return;
        };

        let Some((render_image, raw_bytes)) = self.full_image_cache.get(&id) else {
            return;
        };

        let Some(manifest_entry) = self.history_manifest.iter().find(|e| e.id == id) else {
            return;
        };

        self.reference_images.push(ReferenceImage {
            render_image: render_image.clone(),
            raw_bytes: raw_bytes.clone(),
            mime_type: "image/png".to_string(),
            width: manifest_entry.width,
            height: manifest_entry.height,
        });
        self.error_message = None;
        cx.notify();
    }

    fn filtered_sorted_history(&self, cx: &App) -> Vec<&HistoryManifestEntry> {
        let search_text = self
            .search_editor
            .read(cx)
            .text(cx)
            .to_lowercase();

        let mut entries: Vec<&HistoryManifestEntry> = self
            .history_manifest
            .iter()
            .filter(|e| {
                search_text.is_empty() || e.prompt.to_lowercase().contains(&search_text)
            })
            .collect();

        match self.sort_order {
            SortOrder::NewestFirst => entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp)),
            SortOrder::OldestFirst => entries.sort_by(|a, b| a.timestamp.cmp(&b.timestamp)),
        }

        entries
    }

    // ── Render helpers ──────────────────────────────────────────────────

    fn render_model_selector(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_1()
            .flex_wrap()
            .children(ImageModel::all().iter().map(|model| {
                let is_selected = *model == self.selected_model;
                let model = *model;
                Button::new(
                    SharedString::from(format!("model-{}", model.display_name())),
                    model.display_name(),
                )
                .style(if is_selected {
                    ButtonStyle::Filled
                } else {
                    ButtonStyle::Subtle
                })
                .on_click(cx.listener(move |this, _, _window, cx| {
                    this.selected_model = model;
                    this.on_model_changed();
                    cx.notify();
                }))
                .tooltip(Tooltip::text(model.model_id()))
            }))
    }

    fn render_aspect_ratio_selector(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let ratios = self.selected_model.supported_aspect_ratios();
        h_flex()
            .gap_1()
            .flex_wrap()
            .children(ratios.iter().map(|ratio| {
                let is_selected = *ratio == self.selected_aspect_ratio;
                let ratio = *ratio;
                Button::new(
                    SharedString::from(format!("ratio-{}", ratio.display_name())),
                    ratio.display_name(),
                )
                .style(if is_selected {
                    ButtonStyle::Filled
                } else {
                    ButtonStyle::Subtle
                })
                .on_click(cx.listener(move |this, _, _window, cx| {
                    this.selected_aspect_ratio = ratio;
                    cx.notify();
                }))
            }))
    }

    fn render_resolution_selector(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let resolutions = self.selected_model.supported_resolutions();
        h_flex().gap_1().children(resolutions.iter().map(|res| {
            let is_selected = *res == self.selected_resolution;
            let res = *res;
            Button::new(
                SharedString::from(format!("res-{}", res.display_name())),
                res.display_name(),
            )
            .style(if is_selected {
                ButtonStyle::Filled
            } else {
                ButtonStyle::Subtle
            })
            .on_click(cx.listener(move |this, _, _window, cx| {
                this.selected_resolution = res;
                cx.notify();
            }))
        }))
    }

    fn render_thinking_selector(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let levels = self.selected_model.supported_thinking_levels();
        h_flex().gap_1().children(levels.iter().map(|level| {
            let is_selected = self.selected_thinking_level == Some(*level);
            let level = *level;
            let name = match level {
                ThinkingLevel::Minimal => "Minimal",
                ThinkingLevel::Low => "Low",
                ThinkingLevel::Medium => "Medium",
                ThinkingLevel::High => "High",
            };
            Button::new(SharedString::from(format!("think-{name}")), name)
                .style(if is_selected {
                    ButtonStyle::Filled
                } else {
                    ButtonStyle::Subtle
                })
                .on_click(cx.listener(move |this, _, _window, cx| {
                    this.selected_thinking_level = Some(level);
                    cx.notify();
                }))
        }))
    }

    fn render_reference_images(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let max = self.selected_model.max_reference_images();
        let count = self.reference_images.len();

        v_flex()
            .gap(px(6.0))
            .child(
                h_flex()
                    .items_center()
                    .justify_between()
                    .child(
                        Label::new(format!("References ({}/{})", count, max))
                            .size(LabelSize::Small)
                            .color(ui::Color::Muted),
                    )
                    .when(count > 0, |el| {
                        el.child(
                            Button::new("clear-refs", "Clear")
                                .style(ButtonStyle::Subtle)
                                .on_click(cx.listener(|this, _, _window, cx| {
                                    this.reference_images.clear();
                                    cx.notify();
                                })),
                        )
                    }),
            )
            .child(
                Label::new("Guide style and content of generated images")
                    .size(LabelSize::XSmall)
                    .color(ui::Color::Muted),
            )
            .when(count > 0, |el| {
                el.child(
                    h_flex()
                        .gap(px(4.0))
                        .flex_wrap()
                        .children(
                            self.reference_images
                                .iter()
                                .enumerate()
                                .map(|(idx, refimg)| {
                                    let ri = refimg.render_image.clone();
                                    let aspect =
                                        refimg.width as f32 / refimg.height.max(1) as f32;
                                    div()
                                        .id(SharedString::from(format!("ref-{idx}")))
                                        .relative()
                                        .rounded(px(4.0))
                                        .overflow_hidden()
                                        .border_1()
                                        .border_color(gpui::hsla(0.0, 0.0, 0.4, 0.4))
                                        .child(img(ri).w(px(64.0)).h(px(64.0 / aspect)))
                                        .child(
                                            div()
                                                .absolute()
                                                .top(px(2.0))
                                                .right(px(2.0))
                                                .child(
                                                    div()
                                                        .id(SharedString::from(format!(
                                                            "rm-ref-{idx}"
                                                        )))
                                                        .cursor_pointer()
                                                        .flex()
                                                        .items_center()
                                                        .justify_center()
                                                        .w(px(16.0))
                                                        .h(px(16.0))
                                                        .rounded_full()
                                                        .bg(gpui::hsla(0.0, 0.0, 0.0, 0.7))
                                                        .hover(|s| {
                                                            s.bg(gpui::hsla(0.0, 0.0, 0.0, 0.9))
                                                        })
                                                        .child(
                                                            Label::new("×")
                                                                .size(LabelSize::XSmall)
                                                                .color(ui::Color::Default),
                                                        )
                                                        .on_click(cx.listener(
                                                            move |this, _, _window, cx| {
                                                                if idx
                                                                    < this.reference_images.len()
                                                                {
                                                                    this.reference_images
                                                                        .remove(idx);
                                                                }
                                                                cx.notify();
                                                            },
                                                        )),
                                                ),
                                        )
                                }),
                        ),
                )
            })
            .child(
                div()
                    .id("ref-drop-zone")
                    .rounded(px(6.0))
                    .border_1()
                    .border_color(gpui::hsla(0.0, 0.0, 0.4, 0.2))
                    .py(px(10.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .cursor_pointer()
                    .child(
                        Label::new("Drop images here to use as references")
                            .size(LabelSize::XSmall)
                            .color(ui::Color::Muted),
                    )
                    .drag_over::<ExternalPaths>(|el, _, _, _| {
                        el.border_color(gpui::hsla(0.58, 0.7, 0.5, 0.6))
                            .bg(gpui::hsla(0.58, 0.5, 0.5, 0.08))
                    })
                    .on_drop(cx.listener(|this, paths: &ExternalPaths, _window, cx| {
                        for path in paths.paths() {
                            this.add_reference_from_path(path, cx);
                        }
                    })),
            )
    }

    fn render_loading_placeholder(&self) -> impl IntoElement {
        h_flex()
            .w_full()
            .justify_center()
            .child(
                div()
                    .w(px(500.0))
                    .h(px(500.0))
                    .rounded_md()
                    .flex()
                    .items_center()
                    .justify_center()
                    .bg(gpui::hsla(0.0, 0.0, 0.3, 1.0))
                    .child(
                        Label::new("Generating...")
                            .size(LabelSize::Small)
                            .color(ui::Color::Muted),
                    )
                    .with_animation(
                        "loading-pulse",
                        Animation::new(Duration::from_secs(2)).repeat(),
                        |el, delta| el.opacity(pulsating_between(0.4, 0.8)(delta)),
                    ),
            )
    }

    fn render_main_image(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let element = div().id("main-image-area").flex_1().overflow_y_scroll();

        if self.is_generating {
            return element.child(self.render_loading_placeholder());
        }

        let Some(manifest_entry) = self.selected_manifest_entry() else {
            return element;
        };

        let Some((render_image, raw_bytes)) = self.full_image_cache.get(&manifest_entry.id)
        else {
            return element;
        };

        let render_image = render_image.clone();
        let raw_bytes = raw_bytes.clone();
        let aspect = manifest_entry.width as f32 / manifest_entry.height.max(1) as f32;
        let prompt = manifest_entry.prompt.clone();
        let image_width = 500.0_f32;
        let image_height = image_width / aspect;
        let bytes_for_save = raw_bytes.clone();
        let bytes_for_copy_menu = raw_bytes.clone();
        let bytes_for_copy_btn = raw_bytes;

        element.child(
            v_flex()
                .gap_2()
                .items_center()
                .child(
                    div()
                        .relative()
                        .child(
                            right_click_menu("image-ctx")
                                .trigger(move |_is_open, _window, _cx| {
                                    let ri = render_image.clone();
                                    img(ri)
                                        .w(px(image_width))
                                        .h(px(image_height))
                                        .rounded_md()
                                })
                                .menu(move |window, cx| {
                                    let bytes_save = bytes_for_save.clone();
                                    let bytes_copy = bytes_for_copy_menu.clone();
                                    ContextMenu::build(window, cx, move |menu, _, _cx| {
                                        let bytes_save = bytes_save.clone();
                                        menu.entry(
                                            "Save Image As...",
                                            None,
                                            move |_window, cx: &mut App| {
                                                let bytes = bytes_save.clone();
                                                let home = std::env::var("HOME")
                                                    .unwrap_or_else(|_| ".".to_string());
                                                let receiver = cx.prompt_for_new_path(
                                                    std::path::Path::new(&home),
                                                    Some("generated_image.png"),
                                                );
                                                cx.background_spawn(async move {
                                                    if let Ok(Ok(Some(path))) = receiver.await {
                                                        std::fs::write(&path, &bytes).ok();
                                                    }
                                                })
                                                .detach();
                                            },
                                        )
                                        .entry(
                                            "Copy to Clipboard",
                                            None,
                                            move |_window, cx: &mut App| {
                                                let clipboard_image = gpui::Image::from_bytes(
                                                    gpui::ImageFormat::Png,
                                                    bytes_copy.clone(),
                                                );
                                                cx.write_to_clipboard(
                                                    ClipboardItem::new_image(&clipboard_image),
                                                );
                                            },
                                        )
                                    })
                                }),
                        )
                        .child(
                            div()
                                .absolute()
                                .top(px(4.0))
                                .right(px(4.0))
                                .child(
                                    div()
                                        .id("close-image")
                                        .cursor_pointer()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .w(px(20.0))
                                        .h(px(20.0))
                                        .rounded_full()
                                        .bg(gpui::hsla(0.0, 0.0, 0.0, 0.7))
                                        .hover(|s| s.bg(gpui::hsla(0.0, 0.0, 0.0, 0.9)))
                                        .child(
                                            Label::new("×")
                                                .size(LabelSize::Small)
                                                .color(ui::Color::Default),
                                        )
                                        .on_click(cx.listener(|this, _, _window, cx| {
                                            this.clear_selected_image(cx);
                                        })),
                                ),
                        ),
                )
                .child(
                    div()
                        .px_2()
                        .max_w(px(500.0))
                        .child(
                            Label::new(SharedString::from(prompt))
                                .size(LabelSize::Small)
                                .color(ui::Color::Muted),
                        ),
                )
                .child(
                    h_flex()
                        .gap_2()
                        .child(
                            Button::new("use-as-ref", "Use as Reference")
                                .style(ButtonStyle::Subtle)
                                .icon(IconName::Image)
                                .on_click(cx.listener(|this, _, _window, cx| {
                                    this.use_selected_as_reference(cx);
                                })),
                        )
                        .child(
                            Button::new("copy-clipboard-btn", "Copy to Clipboard")
                                .style(ButtonStyle::Subtle)
                                .icon(IconName::Copy)
                                .on_click(cx.listener(move |_this, _, _window, cx| {
                                    let clipboard_image = gpui::Image::from_bytes(
                                        gpui::ImageFormat::Png,
                                        bytes_for_copy_btn.clone(),
                                    );
                                    cx.write_to_clipboard(ClipboardItem::new_image(
                                        &clipboard_image,
                                    ));
                                })),
                        ),
                ),
        )
    }

    fn render_history(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let entries = self.filtered_sorted_history(cx);
        let total_count = self.history_manifest.len();
        let filtered_count = entries.len();

        let sort_label = match self.sort_order {
            SortOrder::NewestFirst => "Newest",
            SortOrder::OldestFirst => "Oldest",
        };

        v_flex()
            .gap_1()
            .child(
                h_flex()
                    .items_center()
                    .justify_between()
                    .child(
                        Label::new(SharedString::from(format!(
                            "History ({filtered_count}/{total_count})"
                        )))
                        .size(LabelSize::Small),
                    )
                    .child(
                        Button::new("sort-toggle", sort_label)
                            .style(ButtonStyle::Subtle)
                            .icon(IconName::ArrowDown)
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.sort_order = match this.sort_order {
                                    SortOrder::NewestFirst => SortOrder::OldestFirst,
                                    SortOrder::OldestFirst => SortOrder::NewestFirst,
                                };
                                cx.notify();
                            })),
                    ),
            )
            .child(
                div()
                    .border_1()
                    .border_color(cx.theme().colors().border)
                    .rounded_md()
                    .bg(cx.theme().colors().surface_background)
                    .px_2()
                    .py_1()
                    .child(self.search_editor.clone()),
            )
            .when(entries.is_empty() && total_count == 0, |el| {
                el.child(
                    div()
                        .py_4()
                        .flex()
                        .justify_center()
                        .child(
                            Label::new("No images yet")
                                .size(LabelSize::Small)
                                .color(ui::Color::Muted),
                        ),
                )
            })
            .when(entries.is_empty() && total_count > 0, |el| {
                el.child(
                    div()
                        .py_4()
                        .flex()
                        .justify_center()
                        .child(
                            Label::new("No matching images")
                                .size(LabelSize::Small)
                                .color(ui::Color::Muted),
                        ),
                )
            })
            .when(!entries.is_empty(), |el| {
                let thumbnail_elements: Vec<_> = entries
                    .iter()
                    .filter_map(|entry| {
                        let ri = self.thumbnail_cache.get(&entry.id)?.clone();
                        let aspect = entry.width as f32 / entry.height.max(1) as f32;
                        let is_selected = self.selected_history_id.as_deref() == Some(&entry.id);
                        let id = entry.id.clone();
                        let prompt = entry.prompt.clone();
                        Some(
                            div()
                                .id(SharedString::from(format!("hist-{}", entry.id)))
                                .cursor_pointer()
                                .rounded_md()
                                .overflow_hidden()
                                .when(is_selected, |el| {
                                    el.border_2()
                                        .border_color(gpui::hsla(0.58, 0.7, 0.5, 1.0))
                                })
                                .when(!is_selected, |el| {
                                    el.border_1()
                                        .border_color(gpui::hsla(0.0, 0.0, 0.3, 0.5))
                                })
                                .child(img(ri).w(px(56.0)).h(px(56.0 / aspect)))
                                .on_click(cx.listener(move |this, _, _window, cx| {
                                    this.select_history_image(&id, cx);
                                }))
                                .tooltip(Tooltip::text(SharedString::from(prompt))),
                        )
                    })
                    .collect();
                el.child(
                    h_flex()
                        .id("history-scroll")
                        .gap_1()
                        .flex_wrap()
                        .overflow_y_scroll()
                        .max_h(px(250.0))
                        .children(thumbnail_elements),
                )
            })
    }
}

// ── Trait Impls ──────────────────────────────────────────────────────────────

impl EventEmitter<ImageGenEvent> for ImageGenPanel {}

impl Focusable for ImageGenPanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ImageGenPanel {
    fn render(&mut self, _window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        let model = self.selected_model;
        let border = cx.theme().colors().border;
        let surface = cx.theme().colors().surface_background;

        v_flex()
            .id("image-gen-root")
            .key_context("ImageGenPanel")
            .track_focus(&self.focus_handle(cx))
            .size_full()
            .overflow_y_scroll()
            .bg(cx.theme().colors().editor_background)
            // Settings section
            .child(
                v_flex()
                    .p_3()
                    .gap_3()
                    // Model selector
                    .child(
                        v_flex()
                            .gap(px(6.0))
                            .child(
                                Label::new("Model")
                                    .size(LabelSize::Small)
                                    .color(ui::Color::Muted),
                            )
                            .child(self.render_model_selector(cx)),
                    )
                    // Prompt
                    .child(
                        v_flex()
                            .gap(px(6.0))
                            .child(
                                Label::new("Prompt")
                                    .size(LabelSize::Small)
                                    .color(ui::Color::Muted),
                            )
                            .child(
                                div()
                                    .border_1()
                                    .border_color(border)
                                    .rounded_md()
                                    .bg(surface)
                                    .px_2()
                                    .py_1()
                                    .child(self.prompt_editor.clone()),
                            ),
                    )
                    // Aspect ratio
                    .child(
                        v_flex()
                            .gap(px(6.0))
                            .child(
                                Label::new("Aspect Ratio")
                                    .size(LabelSize::Small)
                                    .color(ui::Color::Muted),
                            )
                            .child(self.render_aspect_ratio_selector(cx)),
                    )
                    // Resolution (conditional)
                    .when(model.supports_resolution_picker(), |el: gpui::Div| {
                        el.child(
                            v_flex()
                                .gap(px(6.0))
                                .child(
                                    Label::new("Resolution")
                                        .size(LabelSize::Small)
                                        .color(ui::Color::Muted),
                                )
                                .child(self.render_resolution_selector(cx)),
                        )
                    })
                    // Thinking level (conditional)
                    .when(model.supports_thinking(), |el: gpui::Div| {
                        el.child(
                            v_flex()
                                .gap(px(6.0))
                                .child(
                                    Label::new("Thinking")
                                        .size(LabelSize::Small)
                                        .color(ui::Color::Muted),
                                )
                                .child(self.render_thinking_selector(cx)),
                        )
                    })
                    // Reference images
                    .child(self.render_reference_images(cx))
                    // Divider
                    .child(divider())
                    // Generate button — centered, compact
                    .child(
                        h_flex()
                            .justify_center()
                            .py_1()
                            .child(
                                Button::new(
                                    "generate",
                                    if self.is_generating {
                                        "Generating..."
                                    } else {
                                        "Generate"
                                    },
                                )
                                .style(ButtonStyle::Filled)
                                .disabled(self.is_generating)
                                .when(!self.is_generating, |btn| {
                                    btn.on_click(cx.listener(|this, _, window, cx| {
                                        this.generate_image(window, cx);
                                    }))
                                }),
                            ),
                    ),
            )
            // Error
            .when_some(
                self.error_message.clone(),
                |el: gpui::Stateful<gpui::Div>, msg| {
                    el.child(
                        div()
                            .mx_3()
                            .rounded_md()
                            .p_2()
                            .bg(cx.theme().status().error_background)
                            .child(
                                Label::new(msg)
                                    .size(LabelSize::Small)
                                    .color(ui::Color::Error),
                            ),
                    )
                },
            )
            // Results section
            .child(
                v_flex()
                    .p_3()
                    .gap_3()
                    .flex_1()
                    // Main image display
                    .child(self.render_main_image(cx))
                    // History
                    .child(self.render_history(cx)),
            )
    }
}

impl Item for ImageGenPanel {
    type Event = ImageGenEvent;

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(ItemEvent)) {
        match event {
            ImageGenEvent::TabUpdate => f(ItemEvent::UpdateTab),
        }
    }

    fn tab_content(
        &self,
        params: TabContentParams,
        _window: &gpui::Window,
        _cx: &App,
    ) -> AnyElement {
        Label::new("Image Generator")
            .single_line()
            .color(params.text_color())
            .into_any_element()
    }

    fn tab_icon(&self, _window: &gpui::Window, _cx: &App) -> Option<Icon> {
        Some(Icon::from(IconName::Image))
    }

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Image Generator".into()
    }

    fn tab_tooltip_text(&self, _cx: &App) -> Option<SharedString> {
        Some("Image Generator".into())
    }
}
