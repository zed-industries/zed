//! Codestral completion provider using Fill-in-the-Middle API.
//!
//! This module implements a code completion provider for Codestral that uses its
//! Fill-in-the-Middle (FIM) API to provide intelligent completions at the cursor position.
//!
//! # Key Features
//!
//! - **Smart Context Selection**: Uses tree-sitter to find logical code blocks around cursor
//! - **Interpolation**: Adjusts predictions as the user types
//! - **Simple Cursor Insertion**: Provides completions exactly where the user is typing
//!
//! # Architecture
//!
//! The provider consists of several cooperating components:
//!
//! 1. **Context Selection** (`excerpt_for_cursor_position`): Provides context around cursor
//! 2. **API Integration** (`fetch_completion`): Calls Codestral's FIM API
//! 3. **Interpolation** (`interpolate`): Keeps predictions valid as user types

use crate::input_excerpt::{excerpt_for_cursor_position, prompt_for_outline, CURSOR_MARKER};
use crate::{Codestral, CodestralRequest, CodestralResponse};
use anyhow::Result;
use edit_prediction::{Direction, EditPrediction, EditPredictionProvider};
use futures::AsyncReadExt;
use gpui::{App, Context, Entity, EntityId, Task};
use http_client::HttpClient;
use language::unified_diff;
use language::{
    language_settings::all_language_settings, Anchor, Buffer, BufferSnapshot, EditPreview,
    ToOffset, ToPoint,
};
use project::Project;
use std::{
    collections::{HashMap, VecDeque},
    ops::Range,
    sync::Arc,
    time::{Duration, Instant},
};

pub const DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(150);
const CODESTRAL_API_URL: &str = "https://codestral.mistral.ai/v1/fim/completions";

// Token limits similar to Zeta
/// Maximum tokens for additional context around the editable region
const MAX_CONTEXT_TOKENS: usize = 150;
/// Maximum tokens for the editable region itself
const MAX_REWRITE_TOKENS: usize = 350;

/// Represents a completion that has been received and processed from Codestral.
/// This struct maintains the state needed to interpolate the completion as the user types.
#[derive(Clone)]
struct CurrentCompletion {
    /// The buffer snapshot at the time the completion was generated.
    /// Used to detect changes and interpolate edits.
    snapshot: BufferSnapshot,
    /// The edits that should be applied to transform the original text into the predicted text.
    /// Each edit is a range in the buffer and the text to replace it with.
    edits: Arc<[(Range<Anchor>, String)]>,
    /// Preview of how the buffer will look after applying the edits.
    edit_preview: EditPreview,
}

impl CurrentCompletion {
    /// Attempts to adjust the edits based on changes made to the buffer since the completion was generated.
    /// Returns None if the user's edits conflict with the predicted edits.
    fn interpolate(&self, new_snapshot: &BufferSnapshot) -> Option<Vec<(Range<Anchor>, String)>> {
        edit_prediction::interpolate_edits(&self.snapshot, new_snapshot, &self.edits)
    }
}

#[derive(Clone)]
struct RecentEdit {
    path: String,
    diff: String,
}

pub struct CodestralCompletionProvider {
    codestral: Entity<Codestral>,
    http_client: Arc<dyn HttpClient>,
    pending_request: Option<Task<Result<()>>>,
    current_completion: Option<CurrentCompletion>,
    snapshot_cache: HashMap<EntityId, BufferSnapshot>,
    recent_edits: VecDeque<RecentEdit>,
}

impl CodestralCompletionProvider {
    pub fn new(codestral: Entity<Codestral>, http_client: Arc<dyn HttpClient>) -> Self {
        Self {
            codestral,
            http_client,
            pending_request: None,
            current_completion: None,
            snapshot_cache: HashMap::new(),
            recent_edits: VecDeque::new(),
        }
    }

    const MAX_RECENT_EDITS: usize = 5;
    const MAX_DIFF_LINES: usize = 80;
    const MAX_HEADER_CHARS: usize = 2_000;

    fn record_buffer_change(&mut self, buffer: &Entity<Buffer>, snapshot: &BufferSnapshot) {
        let buffer_id = buffer.entity_id();

        let prev_snapshot = self.snapshot_cache.get(&buffer_id);
        if let Some(prev_snapshot) = prev_snapshot {
            if prev_snapshot.version == snapshot.version {
                return;
            }

            let old_text = prev_snapshot.text();
            let new_text = snapshot.text();

            if old_text != new_text {
                let diff = unified_diff(&old_text, &new_text);
                if !diff.is_empty() {
                    let trimmed: String = diff
                        .lines()
                        .take(Self::MAX_DIFF_LINES)
                        .collect::<Vec<_>>()
                        .join("\n");
                    if !trimmed.is_empty() {
                        let path = snapshot
                            .file()
                            .map(|f| f.path().to_string_lossy().into_owned())
                            .unwrap_or_else(|| "untitled".to_string());

                        self.recent_edits.push_front(RecentEdit {
                            path: path.clone(),
                            diff: trimmed,
                        });
                        while self.recent_edits.len() > Self::MAX_RECENT_EDITS {
                            self.recent_edits.pop_back();
                        }
                        log::info!(
                            "Codestral: queued recent edit for {} ({} total)",
                            path,
                            self.recent_edits.len()
                        );
                    }
                }
            }
        }

        self.snapshot_cache.insert(buffer_id, snapshot.clone());
    }

    fn build_context_header(&self) -> Option<String> {
        if self.recent_edits.is_empty() {
            return None;
        }

        let mut header = String::from("/* RECENT EDITS:\n");
        for edit in &self.recent_edits {
            if header.len() >= Self::MAX_HEADER_CHARS {
                break;
            }

            header.push_str("File: ");
            header.push_str(&edit.path);
            header.push('\n');
            header.push_str("```diff\n");

            let remaining = Self::MAX_HEADER_CHARS.saturating_sub(header.len());
            if remaining == 0 {
                break;
            }

            if edit.diff.len() > remaining {
                header.push_str(&edit.diff[..remaining]);
            } else {
                header.push_str(&edit.diff);
            }
            header.push('\n');
            header.push_str("```\n");

            if header.len() >= Self::MAX_HEADER_CHARS {
                break;
            }
        }

        header.push_str("*/");
        log::info!(
            "Codestral: built recent edits header with {} entries ({} chars)",
            self.recent_edits.len(),
            header.len()
        );
        Some(header)
    }

    /// Uses Codestral's Fill-in-the-Middle API for code completion.
    ///
    /// This function extracts the cursor position from the input and creates a simple
    /// prompt/suffix pair for Codestral's FIM API.
    ///
    /// # Arguments
    /// * `input_excerpt` - Code context with cursor marker
    /// * `_outline` - File structure outline (currently unused by Codestral)
    ///
    /// # Returns
    /// The completion text to insert at the cursor position
    async fn fetch_completion(
        http_client: Arc<dyn HttpClient>,
        api_key: String,
        input_excerpt: String,
        _outline: String,
        model: String,
        max_tokens: Option<u32>,
    ) -> Result<String> {
        let start_time = Instant::now();

        log::debug!(
            "Codestral: Requesting completion (model: {}, max_tokens: {:?})",
            model,
            max_tokens
        );

        // For the simplified implementation, we just need to find the cursor position
        // and split the text there for FIM
        let (prompt, suffix) = if let Some(cursor_pos) = input_excerpt.find(CURSOR_MARKER) {
            // Split at cursor marker
            let before_cursor = &input_excerpt[..cursor_pos];
            let after_cursor = &input_excerpt[cursor_pos + CURSOR_MARKER.len()..];

            (before_cursor.to_string(), after_cursor.to_string())
        } else {
            // No cursor marker found, assume cursor is at the end
            (input_excerpt, "".to_string())
        };

        let request = CodestralRequest {
            model,
            prompt,
            suffix: if suffix.is_empty() {
                None
            } else {
                Some(suffix)
            },
            max_tokens: max_tokens.or(Some(350)),
            temperature: Some(0.2),
            top_p: Some(1.0),
            stream: Some(false),
            stop: None,
            random_seed: None,
            min_tokens: None,
        };

        let request_body = serde_json::to_string(&request)?;

        log::debug!("Codestral: Sending FIM request");

        let http_request = http_client::Request::builder()
            .method(http_client::Method::POST)
            .uri(CODESTRAL_API_URL)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", api_key))
            .body(http_client::AsyncBody::from(request_body))?;

        let mut response = http_client.send(http_request).await?;
        let status = response.status();

        log::debug!("Codestral: Response status: {}", status);

        if !status.is_success() {
            let mut body = String::new();
            response.body_mut().read_to_string(&mut body).await?;
            log::error!("Codestral API error: {} - {}", status, body);
            return Err(anyhow::anyhow!(
                "Codestral API error: {} - {}",
                status,
                body
            ));
        }

        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;

        let codestral_response: CodestralResponse = serde_json::from_str(&body)?;

        let elapsed = start_time.elapsed();

        if let Some(choice) = codestral_response.choices.first() {
            let completion = &choice.message.content;

            log::info!(
                "Codestral: Completion received ({} tokens, {:.2}s)",
                codestral_response.usage.completion_tokens,
                elapsed.as_secs_f64()
            );

            // Return just the completion text for insertion at cursor
            Ok(completion.clone())
        } else {
            log::error!("Codestral: No completion returned in response");
            Err(anyhow::anyhow!("No completion returned from Codestral"))
        }
    }

    /// Creates an edit for inserting the completion text at the cursor position.
    ///
    /// In the simplified implementation, we just need to create a single edit
    /// that inserts the completion text at the cursor.
    ///
    /// # Arguments
    /// * `completion_text` - The text to insert at the cursor
    /// * `cursor_offset` - The offset in the buffer where the cursor is
    /// * `snapshot` - The buffer snapshot for creating anchors
    ///
    /// # Returns
    /// A vector containing a single edit for the insertion
    fn parse_edits(
        completion_text: String,
        cursor_offset: usize,
        snapshot: &BufferSnapshot,
    ) -> Result<Vec<(Range<Anchor>, String)>> {
        log::debug!(
            "Codestral: Creating insertion edit at offset {} with {} chars",
            cursor_offset,
            completion_text.len()
        );

        // Create an anchor at the cursor position
        let cursor_anchor = snapshot.anchor_after(cursor_offset);

        // Return a single edit that inserts the completion at the cursor
        Ok(vec![(cursor_anchor..cursor_anchor, completion_text)])
    }
}

impl EditPredictionProvider for CodestralCompletionProvider {
    fn name() -> &'static str {
        "codestral"
    }

    fn display_name() -> &'static str {
        "Codestral"
    }

    fn show_completions_in_menu() -> bool {
        true
    }

    fn is_enabled(&self, _buffer: &Entity<Buffer>, _cursor_position: Anchor, cx: &App) -> bool {
        let enabled = self.codestral.read(cx).is_enabled();
        log::debug!("Codestral: Provider enabled: {}", enabled);
        enabled
    }

    fn is_refreshing(&self) -> bool {
        self.pending_request.is_some()
    }

    /// Called when the editor requests a new completion prediction.
    ///
    /// This is the main entry point that orchestrates the simplified prediction flow:
    /// 1. Checks if current completion is still valid
    /// 2. Prepares context around cursor position
    /// 3. Sends request to Codestral's FIM API
    /// 4. Creates a simple insertion edit at cursor
    ///
    /// The method spawns an async task to avoid blocking the UI while waiting for the API.
    fn refresh(
        &mut self,
        _project: Option<Entity<Project>>,
        buffer_handle: Entity<Buffer>,
        cursor_position: Anchor,
        debounce: bool,
        cx: &mut Context<Self>,
    ) {
        log::debug!("Codestral: Refresh called (debounce: {})", debounce);

        let Some(api_key) = self.codestral.read(cx).api_key().map(|k| k.to_string()) else {
            log::warn!("Codestral: No API key configured, skipping refresh");
            return;
        };

        let snapshot = buffer_handle.read(cx).snapshot();
        let cursor_point = cursor_position.to_point(&snapshot);

        self.record_buffer_change(&buffer_handle, &snapshot);

        // Check if current completion is still valid
        if let Some(current_completion) = self.current_completion.as_ref() {
            if current_completion.interpolate(&snapshot).is_some() {
                return;
            }
        }

        // Get file path and extension
        let path_str = snapshot
            .file()
            .map(|f| f.full_path(cx).to_string_lossy().into_owned())
            .unwrap_or_else(|| "untitled".to_string());

        let http_client = self.http_client.clone();
        let context_header = self.build_context_header();

        // Get settings
        let settings = all_language_settings(None, cx);
        let model = settings
            .edit_predictions
            .codestral
            .model
            .clone()
            .unwrap_or_else(|| "codestral-latest".to_string());
        let max_tokens = settings.edit_predictions.codestral.max_tokens;

        // Get cursor offset for the simplified implementation
        let cursor_offset = cursor_position.to_offset(&snapshot);

        self.pending_request = Some(cx.spawn(async move |this, cx| {
            if debounce {
                log::debug!("Codestral: Debouncing for {:?}", DEBOUNCE_TIMEOUT);
                smol::Timer::after(DEBOUNCE_TIMEOUT).await;
            }

            // Generate input excerpt with cursor position
            let mut input_excerpt = excerpt_for_cursor_position(
                cursor_point,
                &path_str,
                &snapshot,
                MAX_REWRITE_TOKENS,
                MAX_CONTEXT_TOKENS,
            );
            if let Some(header) = context_header {
                input_excerpt.prompt = format!("{header}\n{}", input_excerpt.prompt);
            }

            let outline = prompt_for_outline(&snapshot);

            let completion_text = match Self::fetch_completion(
                http_client,
                api_key,
                input_excerpt.prompt,
                outline,
                model,
                max_tokens,
            )
            .await
            {
                Ok(completion) => completion,
                Err(e) => {
                    log::error!("Codestral: Failed to fetch completion: {}", e);
                    this.update(cx, |this, cx| {
                        this.pending_request = None;
                        cx.notify();
                    })?;
                    return Err(e);
                }
            };

            if completion_text.trim().is_empty() {
                log::info!("Codestral: Completion was empty after trimming; ignoring");
                this.update(cx, |this, cx| {
                    this.pending_request = None;
                    cx.notify();
                })?;
                return Ok(());
            }

            // Create a simple insertion edit at the cursor position
            let edits = match Self::parse_edits(completion_text, cursor_offset, &snapshot) {
                Ok(edits) => edits,
                Err(e) => {
                    log::error!("Codestral: Failed to parse edits: {}", e);
                    this.update(cx, |this, cx| {
                        this.pending_request = None;
                        cx.notify();
                    })?;
                    return Err(e);
                }
            };

            // Create edit preview
            let edits_arc: Arc<[(Range<Anchor>, String)]> = edits.into();
            let edit_preview = buffer_handle
                .read_with(cx, |buffer, cx| buffer.preview_edits(edits_arc.clone(), cx))?;
            let edit_preview = edit_preview.await;

            this.update(cx, |this, cx| {
                this.current_completion = Some(CurrentCompletion {
                    snapshot,
                    edits: edits_arc,
                    edit_preview,
                });
                this.pending_request = None;
                cx.notify();
            })?;

            Ok(())
        }));
    }

    fn cycle(
        &mut self,
        _buffer: Entity<Buffer>,
        _cursor_position: Anchor,
        _direction: Direction,
        _cx: &mut Context<Self>,
    ) {
        // Codestral doesn't support multiple completions, so cycling does nothing
    }

    fn accept(&mut self, _cx: &mut Context<Self>) {
        log::debug!("Codestral: Completion accepted");
        self.pending_request = None;
        self.current_completion = None;
    }

    fn discard(&mut self, _cx: &mut Context<Self>) {
        log::debug!("Codestral: Completion discarded");
        self.pending_request = None;
        self.current_completion = None;
    }

    /// Returns the current completion suggestion, adjusted for any typing the user has done.
    ///
    /// In the simplified implementation:
    /// 1. Checks if we have a valid completion
    /// 2. Interpolates the single edit based on user changes
    /// 3. Returns the completion
    fn suggest(
        &mut self,
        buffer: &Entity<Buffer>,
        _cursor_position: Anchor,
        cx: &mut Context<Self>,
    ) -> Option<EditPrediction> {
        let current_completion = self.current_completion.as_ref()?;
        let buffer = buffer.read(cx);

        // Try to interpolate the completion based on user changes
        let edits = current_completion.interpolate(&buffer.snapshot())?;

        if edits.is_empty() {
            return None;
        }

        Some(EditPrediction {
            id: None,
            edits,
            edit_preview: Some(current_completion.edit_preview.clone()),
        })
    }
}
