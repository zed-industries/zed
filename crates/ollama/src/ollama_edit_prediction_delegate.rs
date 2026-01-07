use anyhow::{Context as _, Result};
use edit_prediction_context::{EditPredictionExcerpt, EditPredictionExcerptOptions};
use edit_prediction_types::{EditPrediction, EditPredictionDelegate};
use futures::AsyncReadExt;
use gpui::{App, Context, Entity, Task};
use http_client::HttpClient;
use language::{
    Anchor, Buffer, BufferSnapshot, EditPreview, ToPoint, language_settings::all_language_settings,
};
use language_model::{LanguageModelProviderId, LanguageModelRegistry};
use serde::{Deserialize, Serialize};
use std::{
    ops::Range,
    sync::Arc,
    time::{Duration, Instant},
};
use text::ToOffset;

use crate::{OLLAMA_API_URL, get_models};

pub const DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(150);

const EXCERPT_OPTIONS: EditPredictionExcerptOptions = EditPredictionExcerptOptions {
    max_bytes: 1050,
    min_bytes: 525,
    target_before_cursor_over_total_bytes: 0.66,
};

pub const RECOMMENDED_EDIT_PREDICTION_MODELS: [&str; 4] = [
    "qwen2.5-coder:3b-base",
    "qwen2.5-coder:3b",
    "qwen2.5-coder:7b-base",
    "qwen2.5-coder:7b",
];

#[derive(Clone)]
struct CurrentCompletion {
    snapshot: BufferSnapshot,
    edits: Arc<[(Range<Anchor>, Arc<str>)]>,
    edit_preview: EditPreview,
}

impl CurrentCompletion {
    fn interpolate(&self, new_snapshot: &BufferSnapshot) -> Option<Vec<(Range<Anchor>, Arc<str>)>> {
        edit_prediction_types::interpolate_edits(&self.snapshot, new_snapshot, &self.edits)
    }
}

pub struct OllamaEditPredictionDelegate {
    http_client: Arc<dyn HttpClient>,
    pending_request: Option<Task<Result<()>>>,
    current_completion: Option<CurrentCompletion>,
}

impl OllamaEditPredictionDelegate {
    pub fn new(http_client: Arc<dyn HttpClient>) -> Self {
        Self {
            http_client,
            pending_request: None,
            current_completion: None,
        }
    }

    pub fn is_available(cx: &App) -> bool {
        let ollama_provider_id = LanguageModelProviderId::new("ollama");
        LanguageModelRegistry::read_global(cx)
            .provider(&ollama_provider_id)
            .is_some_and(|provider| provider.is_authenticated(cx))
    }

    async fn fetch_completion(
        http_client: Arc<dyn HttpClient>,
        prompt: String,
        suffix: String,
        model: String,
        api_url: String,
    ) -> Result<String> {
        let start_time = Instant::now();

        log::debug!("Ollama: Requesting completion (model: {})", model);

        let fim_prompt = format_fim_prompt(&model, &prompt, &suffix);

        let request = OllamaGenerateRequest {
            model,
            prompt: fim_prompt,
            raw: true,
            stream: false,
            options: Some(OllamaGenerateOptions {
                num_predict: Some(64),
                temperature: Some(0.2),
                stop: Some(get_stop_tokens()),
            }),
        };

        let request_body = serde_json::to_string(&request)?;

        log::debug!("Ollama: Sending FIM request");

        let http_request = http_client::Request::builder()
            .method(http_client::Method::POST)
            .uri(format!("{}/api/generate", api_url))
            .header("Content-Type", "application/json")
            .body(http_client::AsyncBody::from(request_body))?;

        let mut response = http_client.send(http_request).await?;
        let status = response.status();

        log::debug!("Ollama: Response status: {}", status);

        if !status.is_success() {
            let mut body = String::new();
            response.body_mut().read_to_string(&mut body).await?;
            return Err(anyhow::anyhow!("Ollama API error: {} - {}", status, body));
        }

        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;

        let ollama_response: OllamaGenerateResponse =
            serde_json::from_str(&body).context("Failed to parse Ollama response")?;

        let elapsed = start_time.elapsed();

        log::debug!(
            "Ollama: Completion received ({:.2}s)",
            elapsed.as_secs_f64()
        );

        let completion = clean_completion(&ollama_response.response);
        Ok(completion)
    }
}

impl EditPredictionDelegate for OllamaEditPredictionDelegate {
    fn name() -> &'static str {
        "ollama"
    }

    fn display_name() -> &'static str {
        "Ollama"
    }

    fn show_predictions_in_menu() -> bool {
        true
    }

    fn is_enabled(&self, _buffer: &Entity<Buffer>, _cursor_position: Anchor, cx: &App) -> bool {
        Self::is_available(cx)
    }

    fn is_refreshing(&self, _cx: &App) -> bool {
        self.pending_request.is_some()
    }

    fn refresh(
        &mut self,
        buffer: Entity<Buffer>,
        cursor_position: Anchor,
        debounce: bool,
        cx: &mut Context<Self>,
    ) {
        log::debug!("Ollama: Refresh called (debounce: {})", debounce);

        let snapshot = buffer.read(cx).snapshot();

        if let Some(current_completion) = self.current_completion.as_ref() {
            if current_completion.interpolate(&snapshot).is_some() {
                return;
            }
        }

        let http_client = self.http_client.clone();

        let settings = all_language_settings(None, cx);
        let configured_model = settings.edit_predictions.ollama.model.clone();
        let api_url = settings
            .edit_predictions
            .ollama
            .api_url
            .clone()
            .unwrap_or_else(|| OLLAMA_API_URL.to_string());

        self.pending_request = Some(cx.spawn(async move |this, cx| {
            if debounce {
                log::debug!("Ollama: Debouncing for {:?}", DEBOUNCE_TIMEOUT);
                cx.background_executor().timer(DEBOUNCE_TIMEOUT).await;
            }

            let model = if let Some(model) = configured_model
                .as_deref()
                .map(str::trim)
                .filter(|model| !model.is_empty())
            {
                model.to_string()
            } else {
                let local_models = get_models(http_client.as_ref(), &api_url, None).await?;
                let available_model_names = local_models.iter().map(|model| model.name.as_str());

                match pick_recommended_edit_prediction_model(available_model_names) {
                    Some(recommended) => recommended.to_string(),
                    None => {
                        log::debug!(
                            "Ollama: No model configured and no recommended local model found; skipping edit prediction"
                        );
                        this.update(cx, |this, cx| {
                            this.pending_request = None;
                            cx.notify();
                        })?;
                        return Ok(());
                    }
                }
            };

            let cursor_offset = cursor_position.to_offset(&snapshot);
            let cursor_point = cursor_offset.to_point(&snapshot);
            let excerpt = EditPredictionExcerpt::select_from_buffer(
                cursor_point,
                &snapshot,
                &EXCERPT_OPTIONS,
            )
            .context("Line containing cursor doesn't fit in excerpt max bytes")?;

            let excerpt_text = excerpt.text(&snapshot);
            let cursor_within_excerpt = cursor_offset
                .saturating_sub(excerpt.range.start)
                .min(excerpt_text.body.len());
            let prompt = excerpt_text.body[..cursor_within_excerpt].to_string();
            let suffix = excerpt_text.body[cursor_within_excerpt..].to_string();

            let completion_text =
                match Self::fetch_completion(http_client, prompt, suffix, model, api_url).await {
                    Ok(completion) => completion,
                    Err(e) => {
                        log::error!("Ollama: Failed to fetch completion: {}", e);
                        this.update(cx, |this, cx| {
                            this.pending_request = None;
                            cx.notify();
                        })?;
                        return Err(e);
                    }
                };

            if completion_text.trim().is_empty() {
                log::debug!("Ollama: Completion was empty after trimming; ignoring");
                this.update(cx, |this, cx| {
                    this.pending_request = None;
                    cx.notify();
                })?;
                return Ok(());
            }

            let edits: Arc<[(Range<Anchor>, Arc<str>)]> = buffer.read_with(cx, |buffer, _cx| {
                // Clamp the requested offset to the current buffer snapshot length.
                //
                // `anchor_after` ultimately asserts that the offset is within the rope bounds
                // (in debug builds), and our `cursor_position` may be stale vs. the snapshot
                // we used to compute `cursor_offset`.
                let snapshot = buffer.snapshot();
                let clamped_cursor_offset = cursor_offset.min(snapshot.len());

                // Use anchor_after (Right bias) so the cursor stays before the completion text,
                // not at the end of it. This matches how Copilot handles edit predictions.
                let position = buffer.anchor_after(clamped_cursor_offset);
                vec![(position..position, completion_text.into())].into()
            })?;
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

    fn accept(&mut self, _cx: &mut Context<Self>) {
        log::debug!("Ollama: Completion accepted");
        self.pending_request = None;
        self.current_completion = None;
    }

    fn discard(&mut self, _cx: &mut Context<Self>) {
        log::debug!("Ollama: Completion discarded");
        self.pending_request = None;
        self.current_completion = None;
    }

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

fn format_fim_prompt(model: &str, prefix: &str, suffix: &str) -> String {
    let model_base = model.split(':').next().unwrap_or(model);

    match model_base {
        "codellama" | "code-llama" => {
            format!("<PRE> {prefix} <SUF>{suffix} <MID>")
        }
        "starcoder" | "starcoder2" | "starcoderbase" => {
            format!("<fim_prefix>{prefix}<fim_suffix>{suffix}<fim_middle>")
        }
        "deepseek-coder" | "deepseek-coder-v2" => {
            // DeepSeek uses special Unicode characters for FIM tokens
            format!("<｜fim▁begin｜>{prefix}<｜fim▁hole｜>{suffix}<｜fim▁end｜>")
        }
        "qwen2.5-coder" | "qwen-coder" | "qwen" => {
            format!("<|fim_prefix|>{prefix}<|fim_suffix|>{suffix}<|fim_middle|>")
        }
        "codegemma" => {
            format!("<|fim_prefix|>{prefix}<|fim_suffix|>{suffix}<|fim_middle|>")
        }
        "codestral" | "mistral" => {
            format!("[SUFFIX]{suffix}[PREFIX]{prefix}")
        }
        "glm" | "glm-4" | "glm-4.5" => {
            format!("<|code_prefix|>{prefix}<|code_suffix|>{suffix}<|code_middle|>")
        }
        _ => {
            format!("<fim_prefix>{prefix}<fim_suffix>{suffix}<fim_middle>")
        }
    }
}

fn get_stop_tokens() -> Vec<String> {
    vec![
        "<|endoftext|>".to_string(),
        "<|file_separator|>".to_string(),
        "<|fim_pad|>".to_string(),
        "<|fim_prefix|>".to_string(),
        "<|fim_middle|>".to_string(),
        "<|fim_suffix|>".to_string(),
        "<fim_prefix>".to_string(),
        "<fim_middle>".to_string(),
        "<fim_suffix>".to_string(),
        "<PRE>".to_string(),
        "<SUF>".to_string(),
        "<MID>".to_string(),
        "[PREFIX]".to_string(),
        "[SUFFIX]".to_string(),
    ]
}

fn clean_completion(response: &str) -> String {
    let mut result = response.to_string();

    let end_tokens = [
        "<|endoftext|>",
        "<|file_separator|>",
        "<|fim_pad|>",
        "<|fim_prefix|>",
        "<|fim_middle|>",
        "<|fim_suffix|>",
        "<fim_prefix>",
        "<fim_middle>",
        "<fim_suffix>",
        "<PRE>",
        "<SUF>",
        "<MID>",
        "[PREFIX]",
        "[SUFFIX]",
    ];

    for token in &end_tokens {
        if let Some(pos) = result.find(token) {
            result.truncate(pos);
        }
    }

    result
}

#[derive(Debug, Serialize)]
struct OllamaGenerateRequest {
    model: String,
    prompt: String,
    raw: bool,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<OllamaGenerateOptions>,
}

#[derive(Debug, Serialize)]
struct OllamaGenerateOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    num_predict: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct OllamaGenerateResponse {
    response: String,
}
pub fn pick_recommended_edit_prediction_model<'a>(
    available_models: impl IntoIterator<Item = &'a str>,
) -> Option<&'static str> {
    let available: std::collections::HashSet<&str> = available_models.into_iter().collect();

    RECOMMENDED_EDIT_PREDICTION_MODELS
        .into_iter()
        .find(|recommended| available.contains(recommended))
}
