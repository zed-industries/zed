use anyhow::{Context as _, Result};
use cloud_llm_client::predict_edits_v3::Event;
use credentials_provider::CredentialsProvider;
use edit_prediction_context::RelatedFile;
use futures::{AsyncReadExt as _, FutureExt, future::Shared};
use gpui::{
    App, AppContext as _, Entity, Task,
    http_client::{self, AsyncBody, Method},
};
use language::{Buffer, BufferSnapshot, OffsetRangeExt as _, Point, ToPoint as _};
use project::{Project, ProjectPath};
use std::{
    collections::VecDeque, fmt::Write as _, mem, ops::Range, path::Path, sync::Arc, time::Instant,
};

use crate::{
    EditPredictionId, EditPredictionInputs, open_ai_response::text_from_response,
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

    pub fn request_prediction(
        &self,
        _project: &Entity<Project>,
        active_buffer: &Entity<Buffer>,
        snapshot: BufferSnapshot,
        position: language::Anchor,
        events: Vec<Arc<Event>>,
        _recent_paths: &VecDeque<ProjectPath>,
        related_files: Vec<RelatedFile>,
        _diagnostic_search_range: Range<Point>,
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

        let result = cx.background_spawn(async move {
            let (editable_range, context_range) =
                crate::cursor_excerpt::editable_and_context_ranges_for_cursor_position(
                    cursor_point,
                    &snapshot,
                    MAX_CONTEXT_TOKENS,
                    MAX_REWRITE_TOKENS,
                );

            let offset_range = editable_range.to_offset(&snapshot);
            let prompt = build_prompt(
                &events,
                &related_files,
                &snapshot,
                full_path.as_ref(),
                cursor_point,
                editable_range,
                context_range.clone(),
            );

            let inputs = EditPredictionInputs {
                events: events,
                included_files: vec![cloud_llm_client::predict_edits_v3::RelatedFile {
                    path: full_path.clone(),
                    max_row: cloud_llm_client::predict_edits_v3::Line(snapshot.max_point().row),
                    excerpts: vec![cloud_llm_client::predict_edits_v3::Excerpt {
                        start_line: cloud_llm_client::predict_edits_v3::Line(
                            context_range.start.row,
                        ),
                        text: snapshot
                            .text_for_range(context_range.clone())
                            .collect::<String>()
                            .into(),
                    }],
                }],
                cursor_point: cloud_llm_client::predict_edits_v3::Point {
                    column: cursor_point.column,
                    line: cloud_llm_client::predict_edits_v3::Line(cursor_point.row),
                },
                cursor_path: full_path.clone(),
            };

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

            let response_str = response_str.strip_prefix("```\n").unwrap_or(&response_str);
            let response_str = response_str.strip_suffix("\n```").unwrap_or(&response_str);

            let mut edits = Vec::new();
            const NO_PREDICTION_OUTPUT: &str = "None";

            if response_str != NO_PREDICTION_OUTPUT {
                let old_text = snapshot
                    .text_for_range(offset_range.clone())
                    .collect::<String>();
                edits.extend(
                    language::text_diff(&old_text, &response_str)
                        .into_iter()
                        .map(|(range, text)| {
                            (
                                snapshot.anchor_after(offset_range.start + range.start)
                                    ..snapshot.anchor_before(offset_range.start + range.end),
                                text,
                            )
                        }),
                );
            }

            anyhow::Ok((id, edits, snapshot, response_received_at, inputs))
        });

        let buffer = active_buffer.clone();

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

fn build_prompt(
    events: &[Arc<Event>],
    related_files: &[RelatedFile],
    cursor_buffer: &BufferSnapshot,
    cursor_buffer_path: &Path,
    cursor_point: Point,
    editable_range: Range<Point>,
    context_range: Range<Point>,
) -> String {
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
            for related_file in related_files {
                for related_excerpt in &related_file.excerpts {
                    push_delimited(
                        prompt,
                        RECENTLY_VIEWED_SNIPPET_START..RECENTLY_VIEWED_SNIPPET_END,
                        |prompt| {
                            prompt.push_str(CODE_SNIPPET_FILE_PATH_PREFIX);
                            prompt.push_str(related_file.path.path.as_unix_str());
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
            prompt.push_str(cursor_buffer_path.as_os_str().to_string_lossy().as_ref());
            prompt.push('\n');

            let prefix_range = context_range.start..editable_range.start;
            let suffix_range = editable_range.end..context_range.end;

            prompt.extend(cursor_buffer.text_for_range(prefix_range));
            push_delimited(prompt, CODE_TO_EDIT_START..CODE_TO_EDIT_END, |prompt| {
                let range_before_cursor = editable_range.start..cursor_point;
                let range_after_cursor = cursor_point..editable_range.end;
                prompt.extend(cursor_buffer.text_for_range(range_before_cursor));
                prompt.push_str(CURSOR_TAG);
                prompt.extend(cursor_buffer.text_for_range(range_after_cursor));
            });
            prompt.extend(cursor_buffer.text_for_range(suffix_range));
        },
    );

    push_delimited(
        &mut prompt,
        EDIT_DIFF_HISTORY_START..EDIT_DIFF_HISTORY_END,
        |prompt| {
            for event in events {
                writeln!(prompt, "{event}").unwrap();
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
