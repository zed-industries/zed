use crate::{Codestral, CodestralRequest, CodestralResponse};
use anyhow::{Context as _, Result};
use edit_prediction::{Direction, EditPrediction, EditPredictionProvider};
use edit_prediction_context::{EditPredictionExcerpt, EditPredictionExcerptOptions};
use futures::AsyncReadExt;
use gpui::{App, Context, Entity, EntityId, Task};
use http_client::HttpClient;
use language::unified_diff;
use language::{
    language_settings::all_language_settings, Anchor, Buffer, BufferSnapshot, EditPreview, ToPoint,
};
use project::Project;
use std::{
    collections::{HashMap, VecDeque},
    ops::Range,
    sync::Arc,
    time::{Duration, Instant},
};
use text::ToOffset;

pub const DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(150);
const CODESTRAL_API_URL: &str = "https://codestral.mistral.ai/v1/fim/completions";

const EXCERPT_OPTIONS: EditPredictionExcerptOptions = EditPredictionExcerptOptions {
    max_bytes: 1050,
    min_bytes: 525,
    target_before_cursor_over_total_bytes: 0.66,
};

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
                        log::debug!(
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
        log::debug!(
            "Codestral: built recent edits header with {} entries ({} chars)",
            self.recent_edits.len(),
            header.len()
        );
        Some(header)
    }

    /// Uses Codestral's Fill-in-the-Middle API for code completion.
    async fn fetch_completion(
        http_client: Arc<dyn HttpClient>,
        api_key: String,
        prompt: String,
        suffix: String,
        model: String,
        max_tokens: Option<u32>,
    ) -> Result<String> {
        let start_time = Instant::now();

        log::debug!(
            "Codestral: Requesting completion (model: {}, max_tokens: {:?})",
            model,
            max_tokens
        );

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

            log::debug!(
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

        self.record_buffer_change(&buffer_handle, &snapshot);

        // Check if current completion is still valid
        if let Some(current_completion) = self.current_completion.as_ref() {
            if current_completion.interpolate(&snapshot).is_some() {
                return;
            }
        }

        let http_client = self.http_client.clone();

        // Get settings
        let settings = all_language_settings(None, cx);
        let model = settings
            .edit_predictions
            .codestral
            .model
            .clone()
            .unwrap_or_else(|| "codestral-latest".to_string());
        let max_tokens = settings.edit_predictions.codestral.max_tokens;

        self.pending_request = Some(cx.spawn(async move |this, cx| {
            if debounce {
                log::debug!("Codestral: Debouncing for {:?}", DEBOUNCE_TIMEOUT);
                smol::Timer::after(DEBOUNCE_TIMEOUT).await;
            }

            let cursor_offset = cursor_position.to_offset(&snapshot);
            let cursor_point = cursor_offset.to_point(&snapshot);
            let excerpt = EditPredictionExcerpt::select_from_buffer(
                cursor_point,
                &snapshot,
                &EXCERPT_OPTIONS,
                None,
            )
            .context("Line containing cursor doesn't fit in excerpt max bytes")?;

            let excerpt_text = excerpt.text(&snapshot);
            let cursor_within_excerpt = cursor_offset
                .saturating_sub(excerpt.range.start)
                .min(excerpt_text.body.len());
            let prompt = excerpt_text.body[..cursor_within_excerpt].to_string();
            let suffix = excerpt_text.body[cursor_within_excerpt..].to_string();

            let completion_text = match Self::fetch_completion(
                http_client,
                api_key,
                prompt,
                suffix,
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
                log::debug!("Codestral: Completion was empty after trimming; ignoring");
                this.update(cx, |this, cx| {
                    this.pending_request = None;
                    cx.notify();
                })?;
                return Ok(());
            }

            let edits: Arc<[(Range<Anchor>, String)]> =
                vec![(cursor_position..cursor_position, completion_text)].into();
            let edit_preview = buffer_handle
                .read_with(cx, |buffer, cx| buffer.preview_edits(edits.clone(), cx))?
                .await;

            this.update(cx, |this, cx| {
                this.current_completion = Some(CurrentCompletion {
                    snapshot,
                    edits,
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
