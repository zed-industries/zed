use anyhow::{Context as _, Result};
use credentials_provider::CredentialsProvider;
use futures::{AsyncReadExt as _, FutureExt, future::Shared};
use gpui::{
    App, AppContext as _, Task,
    http_client::{self, AsyncBody, Method},
};
use language::{OffsetRangeExt as _, ToOffset, ToPoint as _};
use std::{mem, ops::Range, path::Path, sync::Arc, time::Instant};
use zeta_prompt::ZetaPromptInput;

use crate::{
    DebugEvent, EditPredictionFinishedDebugEvent, EditPredictionId, EditPredictionModelInput,
    EditPredictionStartedDebugEvent, open_ai_response::text_from_response,
    prediction::EditPredictionResult,
};

const MERCURY_API_URL: &str = "https://api.inceptionlabs.ai/v1/edit/completions";
const MAX_CONTEXT_TOKENS: usize = 150;
const MAX_REWRITE_TOKENS: usize = 350;

pub struct Mercury {
    pub api_token: Shared<Task<Option<String>>>,
}

impl Mercury {
    pub fn new(cx: &App) -> Self {
        Mercury {
            api_token: load_api_token(cx).shared(),
        }
    }

    pub fn set_api_token(&mut self, api_token: Option<String>, cx: &mut App) -> Task<Result<()>> {
        self.api_token = Task::ready(api_token.clone()).shared();
        store_api_token_in_keychain(api_token, cx)
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
        let Some(api_token) = self.api_token.clone().now_or_never().flatten() else {
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

            let context_offset_range = context_range.to_offset(&snapshot);

            let editable_offset_range = editable_range.to_offset(&snapshot);

            let inputs = zeta_prompt::ZetaPromptInput {
                events,
                related_files,
                cursor_offset_in_excerpt: cursor_point.to_offset(&snapshot)
                    - context_range.start.to_offset(&snapshot),
                cursor_path: full_path.clone(),
                cursor_excerpt: snapshot
                    .text_for_range(context_range)
                    .collect::<String>()
                    .into(),
                editable_range_in_excerpt: (editable_offset_range.start
                    - context_offset_range.start)
                    ..(editable_offset_range.end - context_offset_range.start),
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
                edits.extend(
                    language::text_diff(&old_text, &response_str)
                        .into_iter()
                        .map(|(range, text)| {
                            (
                                snapshot.anchor_after(editable_offset_range.start + range.start)
                                    ..snapshot
                                        .anchor_before(editable_offset_range.start + range.end),
                                text,
                            )
                        }),
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
                            prompt.push_str(&related_excerpt.text.to_string());
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
    prompt.push_str(delimiters.end);
}

pub const MERCURY_CREDENTIALS_URL: &str = "https://api.inceptionlabs.ai/v1/edit/completions";
pub const MERCURY_CREDENTIALS_USERNAME: &str = "mercury-api-token";

pub fn load_api_token(cx: &App) -> Task<Option<String>> {
    if let Some(api_token) = std::env::var("MERCURY_AI_TOKEN")
        .ok()
        .filter(|value| !value.is_empty())
    {
        return Task::ready(Some(api_token));
    }
    let credentials_provider = <dyn CredentialsProvider>::global(cx);
    cx.spawn(async move |cx| {
        let (_, credentials) = credentials_provider
            .read_credentials(MERCURY_CREDENTIALS_URL, &cx)
            .await
            .ok()??;
        String::from_utf8(credentials).ok()
    })
}

fn store_api_token_in_keychain(api_token: Option<String>, cx: &App) -> Task<Result<()>> {
    let credentials_provider = <dyn CredentialsProvider>::global(cx);

    cx.spawn(async move |cx| {
        if let Some(api_token) = api_token {
            credentials_provider
                .write_credentials(
                    MERCURY_CREDENTIALS_URL,
                    MERCURY_CREDENTIALS_USERNAME,
                    api_token.as_bytes(),
                    cx,
                )
                .await
                .context("Failed to save Mercury API token to system keychain")
        } else {
            credentials_provider
                .delete_credentials(MERCURY_CREDENTIALS_URL, cx)
                .await
                .context("Failed to delete Mercury API token from system keychain")
        }
    })
}
