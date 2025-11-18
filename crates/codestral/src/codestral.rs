use anyhow::{Context as _, Result};
use edit_prediction::{Direction, EditPrediction, EditPredictionProvider};
use edit_prediction_context::{EditPredictionExcerpt, EditPredictionExcerptOptions};
use futures::AsyncReadExt;
use gpui::{App, Context, Entity, Task};
use http_client::HttpClient;
use language::{
    language_settings::all_language_settings, Anchor, Buffer, BufferSnapshot, EditPreview, ToPoint,
};
use language_models::MistralLanguageModelProvider;
use mistral::CODESTRAL_API_URL;
use serde::{Deserialize, Serialize};
use std::{
    ops::Range,
    sync::Arc,
    time::{Duration, Instant},
};
use text::ToOffset;

pub const DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(150);

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
    edits: Arc<[(Range<Anchor>, Arc<str>)]>,
    /// Preview of how the buffer will look after applying the edits.
    edit_preview: EditPreview,
}

impl CurrentCompletion {
    /// Attempts to adjust the edits based on changes made to the buffer since the completion was generated.
    /// Returns None if the user's edits conflict with the predicted edits.
    fn interpolate(&self, new_snapshot: &BufferSnapshot) -> Option<Vec<(Range<Anchor>, Arc<str>)>> {
        edit_prediction::interpolate_edits(&self.snapshot, new_snapshot, &self.edits)
    }
}

pub struct CodestralCompletionProvider {
    http_client: Arc<dyn HttpClient>,
    pending_request: Option<Task<Result<()>>>,
    current_completion: Option<CurrentCompletion>,
}

impl CodestralCompletionProvider {
    pub fn new(http_client: Arc<dyn HttpClient>) -> Self {
        Self {
            http_client,
            pending_request: None,
            current_completion: None,
        }
    }

    pub fn has_api_key(cx: &App) -> bool {
        Self::api_key(cx).is_some()
    }

    /// This is so we can immediately show Codestral as a provider users can
    /// switch to in the edit prediction menu, if the API has been added
    pub fn ensure_api_key_loaded(http_client: Arc<dyn HttpClient>, cx: &mut App) {
        MistralLanguageModelProvider::global(http_client, cx)
            .load_codestral_api_key(cx)
            .detach();
    }

    fn api_key(cx: &App) -> Option<Arc<str>> {
        MistralLanguageModelProvider::try_global(cx)
            .and_then(|provider| provider.codestral_api_key(CODESTRAL_API_URL, cx))
    }

    /// Uses Codestral's Fill-in-the-Middle API for code completion.
    async fn fetch_completion(
        http_client: Arc<dyn HttpClient>,
        api_key: &str,
        prompt: String,
        suffix: String,
        model: String,
        max_tokens: Option<u32>,
        api_url: String,
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
            .uri(format!("{}/v1/fim/completions", api_url))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", api_key))
            .body(http_client::AsyncBody::from(request_body))?;

        let mut response = http_client.send(http_request).await?;
        let status = response.status();

        log::debug!("Codestral: Response status: {}", status);

        if !status.is_success() {
            let mut body = String::new();
            response.body_mut().read_to_string(&mut body).await?;
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
        Self::api_key(cx).is_some()
    }

    fn is_refreshing(&self) -> bool {
        self.pending_request.is_some()
    }

    fn refresh(
        &mut self,
        buffer: Entity<Buffer>,
        cursor_position: language::Anchor,
        debounce: bool,
        cx: &mut Context<Self>,
    ) {
        log::debug!("Codestral: Refresh called (debounce: {})", debounce);

        let Some(api_key) = Self::api_key(cx) else {
            log::warn!("Codestral: No API key configured, skipping refresh");
            return;
        };

        let snapshot = buffer.read(cx).snapshot();

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
        let api_url = settings
            .edit_predictions
            .codestral
            .api_url
            .clone()
            .unwrap_or_else(|| CODESTRAL_API_URL.to_string());

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
                &api_key,
                prompt,
                suffix,
                model,
                max_tokens,
                api_url,
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

            let edits: Arc<[(Range<Anchor>, Arc<str>)]> =
                vec![(cursor_position..cursor_position, completion_text.into())].into();
            let edit_preview = buffer
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

    /// Returns the completion suggestion, adjusted or invalidated based on user edits
    fn suggest(
        &mut self,
        buffer: &Entity<Buffer>,
        _cursor_position: Anchor,
        cx: &mut Context<Self>,
    ) -> Option<EditPrediction> {
        let current_completion = self.current_completion.as_ref()?;
        let buffer = buffer.read(cx);
        let edits = current_completion.interpolate(&buffer.snapshot())?;
        if edits.is_empty() {
            return None;
        }
        Some(EditPrediction::Local {
            id: None,
            edits,
            edit_preview: Some(current_completion.edit_preview.clone()),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodestralRequest {
    pub model: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_tokens: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct CodestralResponse {
    pub id: String,
    pub object: String,
    pub model: String,
    pub usage: Usage,
    pub created: u64,
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub index: u32,
    pub message: Message,
    pub finish_reason: String,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub content: String,
    pub role: String,
}
