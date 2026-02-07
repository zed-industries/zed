use anyhow::Result;
use edit_prediction::cursor_excerpt;
use edit_prediction_types::{
    EditPrediction, EditPredictionDelegate, EditPredictionDiscardReason, EditPredictionIconSet,
};
use futures::AsyncReadExt;
use gpui::{App, AppContext as _, Context, Entity, Global, SharedString, Task};
use http_client::HttpClient;
use icons::IconName;
use language::{
    Anchor, Buffer, BufferSnapshot, EditPreview, ToPoint, language_settings::all_language_settings,
};
use language_model::{ApiKeyState, AuthenticateError, EnvVar, env_var};
use serde::{Deserialize, Serialize};

use std::{
    ops::Range,
    sync::Arc,
    time::{Duration, Instant},
};
use text::{OffsetRangeExt as _, ToOffset};

pub const DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(150);

static OPEN_AI_COMPATIBLE_EP_API_KEY_ENV_VAR: std::sync::LazyLock<EnvVar> =
    env_var!("OPENAI_COMPATIBLE_EDIT_PREDICTION_API_KEY");

struct GlobalOpenAiCompatibleEpApiKey(Entity<ApiKeyState>);

impl Global for GlobalOpenAiCompatibleEpApiKey {}

pub fn open_ai_compatible_ep_api_key_state(cx: &mut App) -> Entity<ApiKeyState> {
    if let Some(global) = cx.try_global::<GlobalOpenAiCompatibleEpApiKey>() {
        return global.0.clone();
    }
    let api_url = open_ai_compatible_ep_api_url(cx);
    let entity = cx.new(|_cx| {
        ApiKeyState::new(api_url, OPEN_AI_COMPATIBLE_EP_API_KEY_ENV_VAR.clone())
    });
    cx.set_global(GlobalOpenAiCompatibleEpApiKey(entity.clone()));
    entity
}

pub fn open_ai_compatible_ep_api_key(cx: &App) -> Option<Arc<str>> {
    let url = open_ai_compatible_ep_api_url(cx);
    cx.try_global::<GlobalOpenAiCompatibleEpApiKey>()?
        .0
        .read(cx)
        .key(&url)
}

pub fn load_open_ai_compatible_ep_api_key(cx: &mut App) -> Task<Result<(), AuthenticateError>> {
    let api_url = open_ai_compatible_ep_api_url(cx);
    open_ai_compatible_ep_api_key_state(cx).update(cx, |key_state, cx| {
        key_state.load_if_needed(api_url, |s| s, cx)
    })
}

pub fn open_ai_compatible_ep_api_url(cx: &App) -> SharedString {
    all_language_settings(None, cx)
        .edit_predictions
        .open_ai_compatible
        .api_url
        .clone()
        .unwrap_or_default()
        .into()
}

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

pub struct OpenAiCompatibleEditPredictionDelegate {
    http_client: Arc<dyn HttpClient>,
    pending_request: Option<Task<Result<()>>>,
    current_completion: Option<CurrentCompletion>,
}

impl OpenAiCompatibleEditPredictionDelegate {
    pub fn new(http_client: Arc<dyn HttpClient>) -> Self {
        Self {
            http_client,
            pending_request: None,
            current_completion: None,
        }
    }

    pub fn ensure_api_key_loaded(cx: &mut App) {
        load_open_ai_compatible_ep_api_key(cx).detach();
    }

    async fn fetch_completion(
        http_client: Arc<dyn HttpClient>,
        api_key: &str,
        api_url: &str,
        request: ChatCompletionRequest,
    ) -> Result<String> {
        let start_time = Instant::now();

        let url = format!("{}/chat/completions", api_url.trim_end_matches('/'));
        log::debug!(
            "OpenAI Compatible EP: Requesting completion (url: {}, model: {}, max_tokens: {})",
            url,
            request.model,
            request.max_tokens
        );

        let request_body = serde_json::to_string(&request)?;

        let http_request = http_client::Request::builder()
            .method(http_client::Method::POST)
            .uri(&url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", api_key))
            .body(http_client::AsyncBody::from(request_body))?;

        let mut response = http_client.send(http_request).await?;
        let status = response.status();

        log::debug!("OpenAI Compatible EP: Response status: {}", status);

        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;

        if !status.is_success() {
            return Err(anyhow::anyhow!(
                "OpenAI Compatible EP API error: {} - {}",
                status,
                body
            ));
        }

        let completion_response: ChatCompletionResponse = serde_json::from_str(&body)?;

        let elapsed = start_time.elapsed();

        if let Some(choice) = completion_response.choices.first() {
            log::debug!(
                "OpenAI Compatible EP: Completion received ({:.2}s)",
                elapsed.as_secs_f64()
            );

            Ok(strip_markdown_code_fences(&choice.message.content))
        } else {
            log::error!("OpenAI Compatible EP: No completion returned in response");
            Err(anyhow::anyhow!(
                "No completion returned from OpenAI Compatible endpoint"
            ))
        }
    }
}

impl EditPredictionDelegate for OpenAiCompatibleEditPredictionDelegate {
    fn name() -> &'static str {
        "open-ai-compatible"
    }

    fn display_name() -> &'static str {
        "OpenAI Compatible"
    }

    fn show_predictions_in_menu() -> bool {
        true
    }

    fn icons(&self, _cx: &App) -> EditPredictionIconSet {
        EditPredictionIconSet::new(IconName::AiOpenAiCompat)
    }

    fn is_enabled(&self, _buffer: &Entity<Buffer>, _cursor_position: Anchor, cx: &App) -> bool {
        open_ai_compatible_ep_api_key(cx).is_some()
    }

    fn is_refreshing(&self, _cx: &App) -> bool {
        self.pending_request.is_some()
    }

    fn refresh(
        &mut self,
        buffer: Entity<Buffer>,
        cursor_position: language::Anchor,
        debounce: bool,
        cx: &mut Context<Self>,
    ) {
        log::debug!(
            "OpenAI Compatible EP: Refresh called (debounce: {})",
            debounce
        );

        let Some(api_key) = open_ai_compatible_ep_api_key(cx) else {
            log::warn!("OpenAI Compatible EP: No API key configured, skipping refresh");
            return;
        };

        let snapshot = buffer.read(cx).snapshot();

        if let Some(current_completion) = self.current_completion.as_ref() {
            if current_completion.interpolate(&snapshot).is_some() {
                return;
            }
        }

        let http_client = self.http_client.clone();

        let settings = all_language_settings(None, cx);
        let open_ai_settings = &settings.edit_predictions.open_ai_compatible;

        let Some(model) = open_ai_settings.model.clone() else {
            log::warn!("OpenAI Compatible EP: No model configured, skipping refresh");
            return;
        };

        let Some(ref api_url) = open_ai_settings.api_url else {
            log::warn!("OpenAI Compatible EP: No API URL configured, skipping refresh");
            return;
        };
        let api_url = api_url.clone();

        let completion_length = open_ai_settings.completion_length;
        let max_tokens = open_ai_settings.max_tokens.unwrap_or(match completion_length {
            settings_content::CompletionLength::Short => 64,
            settings_content::CompletionLength::Medium => 256,
            settings_content::CompletionLength::Long => 512,
        });
        let temperature = open_ai_settings.temperature.unwrap_or(0.2);

        self.pending_request = Some(cx.spawn(async move |this, cx| {
            if debounce {
                log::debug!(
                    "OpenAI Compatible EP: Debouncing for {:?}",
                    DEBOUNCE_TIMEOUT
                );
                cx.background_executor().timer(DEBOUNCE_TIMEOUT).await;
            }

            let cursor_offset = cursor_position.to_offset(&snapshot);
            let cursor_point = cursor_offset.to_point(&snapshot);

            const MAX_CONTEXT_TOKENS: usize = 150;
            const MAX_REWRITE_TOKENS: usize = 350;

            let (_, context_range) =
                cursor_excerpt::editable_and_context_ranges_for_cursor_position(
                    cursor_point,
                    &snapshot,
                    MAX_REWRITE_TOKENS,
                    MAX_CONTEXT_TOKENS,
                );

            let context_range = context_range.to_offset(&snapshot);
            let excerpt_text = snapshot
                .text_for_range(context_range.clone())
                .collect::<String>();
            let cursor_within_excerpt = cursor_offset
                .saturating_sub(context_range.start)
                .min(excerpt_text.len());
            let prompt = excerpt_text[..cursor_within_excerpt].to_string();
            let suffix = excerpt_text[cursor_within_excerpt..].to_string();

            let user_content = if suffix.is_empty() {
                format!(
                    "Continue the code from the cursor position marked <CURSOR>. \
                     Output ONLY the code to insert.\n\n{prompt}<CURSOR>"
                )
            } else {
                format!(
                    "Fill in the code at the cursor position marked <CURSOR>. \
                     Output ONLY the code to insert.\n\n{prompt}<CURSOR>{suffix}"
                )
            };

            let length_instruction = match completion_length {
                settings_content::CompletionLength::Short =>
                    " Keep completions concise: 1-3 lines. Only produce more \
                     when absolutely necessary to complete a started block.",
                settings_content::CompletionLength::Medium =>
                    " Produce moderate completions of up to ~10 lines.",
                settings_content::CompletionLength::Long =>
                    "",
            };

            let system_content = format!(
                "You are a code completion engine. Output ONLY raw code \
                 that should be inserted at the cursor position. Do not include \
                 explanations, markdown formatting, code fences, or any text \
                 other than the code itself. Do not repeat code that is already \
                 present before or after the cursor.{length_instruction}"
            );

            let request = ChatCompletionRequest {
                model,
                messages: vec![
                    ChatMessage {
                        role: "system",
                        content: system_content,
                    },
                    ChatMessage {
                        role: "user",
                        content: user_content,
                    },
                ],
                max_tokens,
                temperature,
                stream: false,
            };

            let completion_text = match Self::fetch_completion(
                http_client,
                &api_key,
                &api_url,
                request,
            )
            .await
            {
                Ok(completion) => completion,
                Err(error) => {
                    log::error!(
                        "OpenAI Compatible EP: Failed to fetch completion: {}",
                        error
                    );
                    this.update(cx, |this, cx| {
                        this.pending_request = None;
                        cx.notify();
                    })?;
                    return Err(error);
                }
            };

            if completion_text.trim().is_empty() {
                log::debug!("OpenAI Compatible EP: Completion was empty after trimming; ignoring");
                this.update(cx, |this, cx| {
                    this.pending_request = None;
                    cx.notify();
                })?;
                return Ok(());
            }

            let completion_text =
                if needs_leading_newline(&prompt, &suffix, &completion_text) {
                    format!("\n{completion_text}")
                } else {
                    completion_text
                };

            let edits: Arc<[(Range<Anchor>, Arc<str>)]> =
                vec![(cursor_position..cursor_position, completion_text.into())].into();
            let edit_preview = buffer
                .read_with(cx, |buffer, cx| buffer.preview_edits(edits.clone(), cx))
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
        log::debug!("OpenAI Compatible EP: Completion accepted");
        self.pending_request = None;
        self.current_completion = None;
    }

    fn discard(&mut self, _reason: EditPredictionDiscardReason, _cx: &mut Context<Self>) {
        log::debug!("OpenAI Compatible EP: Completion discarded");
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
            cursor_position: None,
            edit_preview: Some(current_completion.edit_preview.clone()),
        })
    }
}

#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
    max_tokens: u32,
    temperature: f32,
    stream: bool,
}

#[derive(Debug, Serialize)]
struct ChatMessage {
    role: &'static str,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<ChatCompletionChoice>,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionChoice {
    message: ChatCompletionMessage,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionMessage {
    content: String,
}

/// Strip markdown code fences that instruct models sometimes add despite instructions.
fn strip_markdown_code_fences(text: &str) -> String {
    let trimmed = text.trim();
    if let Some(after_opening) = trimmed.strip_prefix("```") {
        // Skip optional language tag on the first line
        let after_lang = after_opening
            .find('\n')
            .map(|i| &after_opening[i + 1..])
            .unwrap_or(after_opening);
        let code = after_lang
            .strip_suffix("```")
            .unwrap_or(after_lang)
            .trim_end();
        code.to_string()
    } else {
        text.to_string()
    }
}

/// Check if we need to insert a newline before the completion text.
/// This handles the case where the cursor is at the end of a complete line
/// (e.g. a comment or statement) and the model omits the leading newline.
fn needs_leading_newline(prompt: &str, suffix: &str, completion: &str) -> bool {
    if completion.starts_with('\n') {
        return false;
    }

    // If the prompt already ends with a newline, the cursor is at the start
    // of a new line — no need to add another.
    if prompt.ends_with('\n') || prompt.is_empty() {
        return false;
    }

    // Cursor must be at the end of a line (suffix starts with newline or is empty)
    if !suffix.is_empty() && !suffix.starts_with('\n') {
        return false;
    }

    let last_line = match prompt.lines().last() {
        Some(line) => line,
        None => return false,
    };
    let trimmed = last_line.trim_end();
    if trimmed.is_empty() {
        return false;
    }

    // If the line ends with characters that expect a continuation on the same line,
    // don't add a newline (e.g. `let x = `, `foo(`, `[1, `)
    if let Some(last_char) = trimmed.chars().last() {
        if matches!(last_char, '=' | '(' | '[' | ',') {
            return false;
        }
    }

    true
}
