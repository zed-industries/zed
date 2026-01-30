use crate::{
    DebugEvent, EditPredictionFinishedDebugEvent, EditPredictionId, EditPredictionModelInput,
    EditPredictionStartedDebugEvent, open_ai_response::text_from_response,
    prediction::EditPredictionResult, zeta1::compute_edits,
};
use anyhow::{Context as _, Result};
use futures::AsyncReadExt as _;
use gpui::{
    App, AppContext as _, Entity, Global, SharedString, Task,
    http_client::{self, AsyncBody, Method},
};
use language::{OffsetRangeExt as _, ToOffset, ToPoint as _};
use language_model::{ApiKeyState, EnvVar, env_var};
use std::{mem, ops::Range, path::Path, sync::Arc, time::Instant};
use zeta_prompt::ZetaPromptInput;

const MERCURY_API_URL: &str = "https://api.inceptionlabs.ai/v1/edit/completions";
const MAX_REWRITE_TOKENS: usize = 150;
const MAX_CONTEXT_TOKENS: usize = 350;

pub struct Mercury {
    pub api_token: Entity<ApiKeyState>,
}

impl Mercury {
    pub fn new(cx: &mut App) -> Self {
        Mercury {
            api_token: mercury_api_token(cx),
        }
    }

    pub(crate) fn request_prediction(
        &self,
        EditPredictionModelInput {
            buffer,
            snapshot,
            position,
            events,
            related_files,
            debug_tx,
            ..
        }: EditPredictionModelInput,
        cx: &mut App,
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
            let (editable_range, context_range) =
                crate::cursor_excerpt::editable_and_context_ranges_for_cursor_position(
                    cursor_point,
                    &snapshot,
                    MAX_CONTEXT_TOKENS,
                    MAX_REWRITE_TOKENS,
                );

            let related_files = crate::filter_redundant_excerpts(
                related_files,
                full_path.as_ref(),
                context_range.start.row..context_range.end.row,
            );

            let context_offset_range = context_range.to_offset(&snapshot);
            let context_start_row = context_range.start.row;

            let editable_offset_range = editable_range.to_offset(&snapshot);

            let inputs = zeta_prompt::ZetaPromptInput {
                events,
                related_files,
                cursor_offset_in_excerpt: cursor_point.to_offset(&snapshot)
                    - context_offset_range.start,
                cursor_path: full_path.clone(),
                cursor_excerpt: snapshot
                    .text_for_range(context_range)
                    .collect::<String>()
                    .into(),
                editable_range_in_excerpt: (editable_offset_range.start
                    - context_offset_range.start)
                    ..(editable_offset_range.end - context_offset_range.start),
                excerpt_start_row: Some(context_start_row),
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

        cx.spawn(async move |cx| {
            let (id, edits, old_snapshot, response_received_at, inputs) =
                result.await.context("Mercury edit prediction failed")?;
            anyhow::Ok(Some(
                EditPredictionResult::new(
                    EditPredictionId(id.into()),
                    &buffer,
                    &old_snapshot,
                    edits.into(),
                    buffer_snapshotted_at,
                    response_received_at,
                    inputs,
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
            for related_file in inputs.related_files.iter() {
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

            prompt.push_str(&inputs.cursor_excerpt[0..inputs.editable_range_in_excerpt.start]);
            push_delimited(prompt, CODE_TO_EDIT_START..CODE_TO_EDIT_END, |prompt| {
                prompt.push_str(
                    &inputs.cursor_excerpt
                        [inputs.editable_range_in_excerpt.start..inputs.cursor_offset_in_excerpt],
                );
                prompt.push_str(CURSOR_TAG);
                prompt.push_str(
                    &inputs.cursor_excerpt
                        [inputs.cursor_offset_in_excerpt..inputs.editable_range_in_excerpt.end],
                );
            });
            prompt.push_str(&inputs.cursor_excerpt[inputs.editable_range_in_excerpt.end..]);
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
