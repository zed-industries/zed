use crate::{
    DebugEvent, EditPredictionFinishedDebugEvent, EditPredictionId, EditPredictionModelInput,
    EditPredictionStartedDebugEvent, EditPredictionStore, open_ai_response::text_from_response,
    prediction::EditPredictionResult, zeta::compute_edits,
};
use anyhow::{Context as _, Result};
use cloud_llm_client::EditPredictionRejectReason;
use futures::AsyncReadExt as _;
use gpui::{
    App, AppContext as _, Context, Entity, Global, SharedString, Task,
    http_client::{self, AsyncBody, HttpClient, Method, StatusCode},
};
use language::{ToOffset, ToPoint as _};
use language_model::{ApiKeyState, EnvVar, env_var};
use release_channel::AppVersion;
use serde::{Deserialize, Serialize};
use std::{mem, ops::Range, path::Path, sync::Arc, time::Instant};
use zeta_prompt::ZetaPromptInput;

const MERCURY_API_URL: &str = "https://api.inceptionlabs.ai/v1/edit/completions";

pub struct Mercury {
    pub api_token: Entity<ApiKeyState>,
    payment_required_error: bool,
}

impl Mercury {
    pub fn new(cx: &mut App) -> Self {
        Mercury {
            api_token: mercury_api_token(cx),
            payment_required_error: false,
        }
    }

    pub fn has_payment_required_error(&self) -> bool {
        self.payment_required_error
    }

    pub fn set_payment_required_error(&mut self, payment_required_error: bool) {
        self.payment_required_error = payment_required_error;
    }

    pub(crate) fn request_prediction(
        &mut self,
        EditPredictionModelInput {
            buffer,
            snapshot,
            position,
            events,
            related_files,
            debug_tx,
            ..
        }: EditPredictionModelInput,
        cx: &mut Context<EditPredictionStore>,
    ) -> Task<Result<Option<EditPredictionResult>>> {
        self.api_token.update(cx, |key_state, cx| {
            _ = key_state.load_if_needed(MERCURY_CREDENTIALS_URL, |s| s, cx);
        });
        let Some(api_token) = self.api_token.read(cx).key(&MERCURY_CREDENTIALS_URL) else {
            return Task::ready(Ok(None));
        };
        let full_path: Arc<Path> = snapshot
            .file()
            .map(|file| file.full_path(cx))
            .unwrap_or_else(|| "untitled".into())
            .into();

        let http_client = cx.http_client();
        let cursor_point = position.to_point(&snapshot);
        let buffer_snapshotted_at = Instant::now();
        let active_buffer = buffer.clone();

        let result = cx.background_spawn(async move {
            let cursor_offset = cursor_point.to_offset(&snapshot);
            let (excerpt_point_range, excerpt_offset_range, cursor_offset_in_excerpt) =
                crate::cursor_excerpt::compute_cursor_excerpt(&snapshot, cursor_offset);

            let related_files = zeta_prompt::filter_redundant_excerpts(
                related_files,
                full_path.as_ref(),
                excerpt_point_range.start.row..excerpt_point_range.end.row,
            );

            let cursor_excerpt: Arc<str> = snapshot
                .text_for_range(excerpt_point_range.clone())
                .collect::<String>()
                .into();
            let syntax_ranges = crate::cursor_excerpt::compute_syntax_ranges(
                &snapshot,
                cursor_offset,
                &excerpt_offset_range,
            );
            let excerpt_ranges = zeta_prompt::compute_legacy_excerpt_ranges(
                &cursor_excerpt,
                cursor_offset_in_excerpt,
                &syntax_ranges,
            );

            let editable_offset_range = (excerpt_offset_range.start
                + excerpt_ranges.editable_350.start)
                ..(excerpt_offset_range.start + excerpt_ranges.editable_350.end);

            let inputs = zeta_prompt::ZetaPromptInput {
                events,
                related_files: Some(related_files),
                cursor_offset_in_excerpt: cursor_point.to_offset(&snapshot)
                    - excerpt_offset_range.start,
                cursor_path: full_path.clone(),
                cursor_excerpt,
                experiment: None,
                excerpt_start_row: Some(excerpt_point_range.start.row),
                excerpt_ranges,
                syntax_ranges: Some(syntax_ranges),
                active_buffer_diagnostics: vec![],
                in_open_source_repo: false,
                can_collect_data: false,
                repo_url: None,
            };

            let prompt = build_prompt(&inputs);

            if let Some(debug_tx) = &debug_tx {
                debug_tx
                    .unbounded_send(DebugEvent::EditPredictionStarted(
                        EditPredictionStartedDebugEvent {
                            buffer: active_buffer.downgrade(),
                            prompt: Some(prompt.clone()),
                            position,
                        },
                    ))
                    .ok();
            }

            let request_body = open_ai::Request {
                model: "mercury-coder".into(),
                messages: vec![open_ai::RequestMessage::User {
                    content: open_ai::MessageContent::Plain(prompt),
                }],
                stream: false,
                max_completion_tokens: None,
                stop: vec![],
                temperature: None,
                tool_choice: None,
                parallel_tool_calls: None,
                tools: vec![],
                prompt_cache_key: None,
                reasoning_effort: None,
            };

            let buf = serde_json::to_vec(&request_body)?;
            let body: AsyncBody = buf.into();

            let request = http_client::Request::builder()
                .uri(MERCURY_API_URL)
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", api_token))
                .header("Connection", "keep-alive")
                .method(Method::POST)
                .body(body)
                .context("Failed to create request")?;

            let mut response = http_client
                .send(request)
                .await
                .context("Failed to send request")?;

            let mut body: Vec<u8> = Vec::new();
            response
                .body_mut()
                .read_to_end(&mut body)
                .await
                .context("Failed to read response body")?;

            let response_received_at = Instant::now();
            if !response.status().is_success() {
                if response.status() == StatusCode::PAYMENT_REQUIRED {
                    anyhow::bail!(MercuryPaymentRequiredError(
                        mercury_payment_required_message(&body),
                    ));
                }

                anyhow::bail!(
                    "Request failed with status: {:?}\nBody: {}",
                    response.status(),
                    String::from_utf8_lossy(&body),
                );
            };

            let mut response: open_ai::Response =
                serde_json::from_slice(&body).context("Failed to parse response")?;

            let id = mem::take(&mut response.id);
            let response_str = text_from_response(response).unwrap_or_default();

            if let Some(debug_tx) = &debug_tx {
                debug_tx
                    .unbounded_send(DebugEvent::EditPredictionFinished(
                        EditPredictionFinishedDebugEvent {
                            buffer: active_buffer.downgrade(),
                            model_output: Some(response_str.clone()),
                            position,
                        },
                    ))
                    .ok();
            }

            let response_str = response_str.strip_prefix("```\n").unwrap_or(&response_str);
            let response_str = response_str.strip_suffix("\n```").unwrap_or(&response_str);

            let mut edits = Vec::new();
            const NO_PREDICTION_OUTPUT: &str = "None";

            if response_str != NO_PREDICTION_OUTPUT {
                let old_text = snapshot
                    .text_for_range(editable_offset_range.clone())
                    .collect::<String>();
                edits = compute_edits(
                    old_text,
                    &response_str,
                    editable_offset_range.start,
                    &snapshot,
                );
            }

            anyhow::Ok((id, edits, snapshot, response_received_at, inputs))
        });

        cx.spawn(async move |ep_store, cx| {
            let result = result.await.context("Mercury edit prediction failed");

            let has_payment_required_error = result
                .as_ref()
                .err()
                .is_some_and(is_mercury_payment_required_error);

            ep_store.update(cx, |store, cx| {
                store
                    .mercury
                    .set_payment_required_error(has_payment_required_error);
                cx.notify();
            })?;

            let (id, edits, old_snapshot, response_received_at, inputs) = result?;
            anyhow::Ok(Some(
                EditPredictionResult::new(
                    EditPredictionId(id.into()),
                    &buffer,
                    &old_snapshot,
                    edits.into(),
                    None,
                    buffer_snapshotted_at,
                    response_received_at,
                    inputs,
                    None,
                    cx,
                )
                .await,
            ))
        })
    }
}

fn build_prompt(inputs: &ZetaPromptInput) -> String {
    const RECENTLY_VIEWED_SNIPPETS_START: &str = "<|recently_viewed_code_snippets|>\n";
    const RECENTLY_VIEWED_SNIPPETS_END: &str = "<|/recently_viewed_code_snippets|>\n";
    const RECENTLY_VIEWED_SNIPPET_START: &str = "<|recently_viewed_code_snippet|>\n";
    const RECENTLY_VIEWED_SNIPPET_END: &str = "<|/recently_viewed_code_snippet|>\n";
    const CURRENT_FILE_CONTENT_START: &str = "<|current_file_content|>\n";
    const CURRENT_FILE_CONTENT_END: &str = "<|/current_file_content|>\n";
    const CODE_TO_EDIT_START: &str = "<|code_to_edit|>\n";
    const CODE_TO_EDIT_END: &str = "<|/code_to_edit|>\n";
    const EDIT_DIFF_HISTORY_START: &str = "<|edit_diff_history|>\n";
    const EDIT_DIFF_HISTORY_END: &str = "<|/edit_diff_history|>\n";
    const CURSOR_TAG: &str = "<|cursor|>";
    const CODE_SNIPPET_FILE_PATH_PREFIX: &str = "code_snippet_file_path: ";
    const CURRENT_FILE_PATH_PREFIX: &str = "current_file_path: ";

    let mut prompt = String::new();

    push_delimited(
        &mut prompt,
        RECENTLY_VIEWED_SNIPPETS_START..RECENTLY_VIEWED_SNIPPETS_END,
        |prompt| {
            for related_file in inputs.related_files.as_deref().unwrap_or_default().iter() {
                for related_excerpt in &related_file.excerpts {
                    push_delimited(
                        prompt,
                        RECENTLY_VIEWED_SNIPPET_START..RECENTLY_VIEWED_SNIPPET_END,
                        |prompt| {
                            prompt.push_str(CODE_SNIPPET_FILE_PATH_PREFIX);
                            prompt.push_str(related_file.path.to_string_lossy().as_ref());
                            prompt.push('\n');
                            prompt.push_str(related_excerpt.text.as_ref());
                        },
                    );
                }
            }
        },
    );

    push_delimited(
        &mut prompt,
        CURRENT_FILE_CONTENT_START..CURRENT_FILE_CONTENT_END,
        |prompt| {
            prompt.push_str(CURRENT_FILE_PATH_PREFIX);
            prompt.push_str(inputs.cursor_path.as_os_str().to_string_lossy().as_ref());
            prompt.push('\n');

            let editable_range = &inputs.excerpt_ranges.editable_350;
            prompt.push_str(&inputs.cursor_excerpt[0..editable_range.start]);
            push_delimited(prompt, CODE_TO_EDIT_START..CODE_TO_EDIT_END, |prompt| {
                prompt.push_str(
                    &inputs.cursor_excerpt[editable_range.start..inputs.cursor_offset_in_excerpt],
                );
                prompt.push_str(CURSOR_TAG);
                prompt.push_str(
                    &inputs.cursor_excerpt[inputs.cursor_offset_in_excerpt..editable_range.end],
                );
            });
            prompt.push_str(&inputs.cursor_excerpt[editable_range.end..]);
        },
    );

    push_delimited(
        &mut prompt,
        EDIT_DIFF_HISTORY_START..EDIT_DIFF_HISTORY_END,
        |prompt| {
            for event in inputs.events.iter() {
                zeta_prompt::write_event(prompt, &event);
            }
        },
    );

    prompt
}

fn push_delimited(prompt: &mut String, delimiters: Range<&str>, cb: impl FnOnce(&mut String)) {
    prompt.push_str(delimiters.start);
    cb(prompt);
    prompt.push('\n');
    prompt.push_str(delimiters.end);
}

pub const MERCURY_CREDENTIALS_URL: SharedString =
    SharedString::new_static("https://api.inceptionlabs.ai/v1/edit/completions");
pub const MERCURY_CREDENTIALS_USERNAME: &str = "mercury-api-token";

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
struct MercuryPaymentRequiredError(SharedString);

#[derive(Deserialize)]
struct MercuryErrorResponse {
    error: MercuryErrorMessage,
}

#[derive(Deserialize)]
struct MercuryErrorMessage {
    message: String,
}

fn is_mercury_payment_required_error(error: &anyhow::Error) -> bool {
    error
        .downcast_ref::<MercuryPaymentRequiredError>()
        .is_some()
}

fn mercury_payment_required_message(body: &[u8]) -> SharedString {
    serde_json::from_slice::<MercuryErrorResponse>(body)
        .map(|response| response.error.message.into())
        .unwrap_or_else(|_| String::from_utf8_lossy(body).trim().to_string().into())
}

pub static MERCURY_TOKEN_ENV_VAR: std::sync::LazyLock<EnvVar> = env_var!("MERCURY_AI_TOKEN");

struct GlobalMercuryApiKey(Entity<ApiKeyState>);

impl Global for GlobalMercuryApiKey {}

pub fn mercury_api_token(cx: &mut App) -> Entity<ApiKeyState> {
    if let Some(global) = cx.try_global::<GlobalMercuryApiKey>() {
        return global.0.clone();
    }
    let entity =
        cx.new(|_| ApiKeyState::new(MERCURY_CREDENTIALS_URL, MERCURY_TOKEN_ENV_VAR.clone()));
    cx.set_global(GlobalMercuryApiKey(entity.clone()));
    entity
}

pub fn load_mercury_api_token(cx: &mut App) -> Task<Result<(), language_model::AuthenticateError>> {
    mercury_api_token(cx).update(cx, |key_state, cx| {
        key_state.load_if_needed(MERCURY_CREDENTIALS_URL, |s| s, cx)
    })
}

const FEEDBACK_API_URL: &str = "https://api-feedback.inceptionlabs.ai/feedback";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum MercuryUserAction {
    Accept,
    Reject,
    Ignore,
}

#[derive(Serialize)]
struct FeedbackRequest {
    request_id: SharedString,
    provider_name: &'static str,
    user_action: MercuryUserAction,
    provider_version: String,
}

pub(crate) fn edit_prediction_accepted(
    prediction_id: EditPredictionId,
    http_client: Arc<dyn HttpClient>,
    cx: &App,
) {
    send_feedback(prediction_id, MercuryUserAction::Accept, http_client, cx);
}

pub(crate) fn edit_prediction_rejected(
    prediction_id: EditPredictionId,
    was_shown: bool,
    reason: EditPredictionRejectReason,
    http_client: Arc<dyn HttpClient>,
    cx: &App,
) {
    if !was_shown {
        return;
    }
    let action = match reason {
        EditPredictionRejectReason::Rejected => MercuryUserAction::Reject,
        EditPredictionRejectReason::Discarded => MercuryUserAction::Ignore,
        _ => return,
    };
    send_feedback(prediction_id, action, http_client, cx);
}

fn send_feedback(
    prediction_id: EditPredictionId,
    action: MercuryUserAction,
    http_client: Arc<dyn HttpClient>,
    cx: &App,
) {
    let request_id = prediction_id.0;
    let app_version = AppVersion::global(cx);
    cx.background_spawn(async move {
        let body = FeedbackRequest {
            request_id,
            provider_name: "zed",
            user_action: action,
            provider_version: app_version.to_string(),
        };

        let request = http_client::Request::builder()
            .uri(FEEDBACK_API_URL)
            .method(Method::POST)
            .header("Content-Type", "application/json")
            .body(AsyncBody::from(serde_json::to_vec(&body)?))?;

        let response = http_client.send(request).await?;
        if !response.status().is_success() {
            anyhow::bail!("Feedback API returned status: {}", response.status());
        }

        log::debug!(
            "Mercury feedback sent: request_id={}, action={:?}",
            body.request_id,
            body.user_action
        );

        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}
