use crate::{
    EditPredictionId, EditPredictionModelInput, cursor_excerpt,
    ollama::{
        clean_fim_completion, clean_zeta_completion, format_fim_prompt, get_fim_stop_tokens,
        get_zeta_stop_tokens, is_zeta_model,
    },
    open_ai_response::text_from_response,
    prediction::EditPredictionResult,
    zeta1::{
        self, MAX_CONTEXT_TOKENS as ZETA_MAX_CONTEXT_TOKENS,
        MAX_EVENT_TOKENS as ZETA_MAX_EVENT_TOKENS,
    },
};
use anyhow::{Context as _, Result};
use futures::AsyncReadExt as _;
use gpui::{App, AppContext as _, Entity, Task, http_client};
use language::{
    Anchor, Buffer, BufferSnapshot, OffsetRangeExt as _, ToOffset, ToPoint as _,
    language_settings::all_language_settings,
};
use std::{path::Path, sync::Arc, time::Instant};
use zeta_prompt::{ZetaPromptInput, zeta1::format_zeta1_prompt};

const FIM_CONTEXT_TOKENS: usize = 512;

pub struct OpenAiCompatibleEditPrediction;

struct RequestOutput {
    id: String,
    edits: Vec<(std::ops::Range<Anchor>, Arc<str>)>,
    snapshot: BufferSnapshot,
    response_received_at: Instant,
    inputs: ZetaPromptInput,
    buffer: Entity<Buffer>,
    buffer_snapshotted_at: Instant,
}

fn chat_completions_url(api_url: &str) -> String {
    let trimmed = api_url.trim_end_matches('/');
    if trimmed.ends_with("/v1") {
        format!("{}/chat/completions", trimmed)
    } else {
        format!("{}/v1/chat/completions", trimmed)
    }
}

impl OpenAiCompatibleEditPrediction {
    pub fn new() -> Self {
        Self
    }

    pub fn request_prediction(
        &self,
        EditPredictionModelInput {
            buffer,
            snapshot,
            position,
            events,
            ..
        }: EditPredictionModelInput,
        cx: &mut App,
    ) -> Task<Result<Option<EditPredictionResult>>> {
        let settings = &all_language_settings(None, cx)
            .edit_predictions
            .openai_compatible;
        let Some(model) = settings.model.clone() else {
            return Task::ready(Ok(None));
        };
        let api_url = settings.api_url.clone();
        let api_key = settings
            .api_key
            .clone()
            .or_else(|| std::env::var("OPENAI_COMPATIBLE_EDIT_PREDICTION_API_KEY").ok());

        log::debug!(
            "OpenAI Compatible: Requesting completion (model: {}, url: {})",
            model,
            api_url
        );

        let full_path: Arc<Path> = snapshot
            .file()
            .map(|file| file.full_path(cx))
            .unwrap_or_else(|| "untitled".into())
            .into();

        let http_client = cx.http_client();
        let cursor_point = position.to_point(&snapshot);
        let buffer_snapshotted_at = Instant::now();

        let is_zeta = is_zeta_model(&model);

        let max_output_tokens = settings.max_output_tokens;
        let max_output_tokens = if is_zeta {
            max_output_tokens * 4
        } else {
            max_output_tokens
        };

        let result = cx.background_spawn(async move {
            let zeta_editable_region_tokens = max_output_tokens as usize;

            let (prompt, stop_tokens, editable_range_override, inputs) = if is_zeta {
                let path_str = full_path.to_string_lossy();
                let input_excerpt = zeta1::excerpt_for_cursor_position(
                    cursor_point,
                    &path_str,
                    &snapshot,
                    zeta_editable_region_tokens,
                    ZETA_MAX_CONTEXT_TOKENS,
                );
                let input_events = zeta1::prompt_for_events(&events, ZETA_MAX_EVENT_TOKENS);
                let prompt = format_zeta1_prompt(&input_events, &input_excerpt.prompt);
                let editable_offset_range = input_excerpt.editable_range.to_offset(&snapshot);
                let context_offset_range = input_excerpt.context_range.to_offset(&snapshot);
                let stop_tokens = get_zeta_stop_tokens();

                let inputs = ZetaPromptInput {
                    events,
                    related_files: Vec::new(),
                    cursor_offset_in_excerpt: cursor_point.to_offset(&snapshot)
                        - context_offset_range.start,
                    cursor_path: full_path.clone(),
                    cursor_excerpt: snapshot
                        .text_for_range(input_excerpt.context_range.clone())
                        .collect::<String>()
                        .into(),
                    editable_range_in_excerpt: (editable_offset_range.start
                        - context_offset_range.start)
                        ..(editable_offset_range.end - context_offset_range.start),
                    excerpt_start_row: Some(input_excerpt.context_range.start.row),
                    excerpt_ranges: None,
                    preferred_model: None,
                    in_open_source_repo: false,
                };

                (prompt, stop_tokens, Some(editable_offset_range), inputs)
            } else {
                let (excerpt_range, _) =
                    cursor_excerpt::editable_and_context_ranges_for_cursor_position(
                        cursor_point,
                        &snapshot,
                        FIM_CONTEXT_TOKENS,
                        0,
                    );
                let excerpt_offset_range = excerpt_range.to_offset(&snapshot);
                let cursor_offset = cursor_point.to_offset(&snapshot);

                let inputs = ZetaPromptInput {
                    events,
                    related_files: Vec::new(),
                    cursor_offset_in_excerpt: cursor_offset - excerpt_offset_range.start,
                    editable_range_in_excerpt: cursor_offset - excerpt_offset_range.start
                        ..cursor_offset - excerpt_offset_range.start,
                    cursor_path: full_path.clone(),
                    excerpt_start_row: Some(excerpt_range.start.row),
                    cursor_excerpt: snapshot
                        .text_for_range(excerpt_range)
                        .collect::<String>()
                        .into(),
                    excerpt_ranges: None,
                    preferred_model: None,
                    in_open_source_repo: false,
                };

                let Some((prefix, suffix)) = inputs
                    .cursor_excerpt
                    .split_at_checked(inputs.cursor_offset_in_excerpt)
                else {
                    return Err(anyhow::anyhow!(
                        "cursor offset {} was out of bounds for excerpt length {}",
                        inputs.cursor_offset_in_excerpt,
                        inputs.cursor_excerpt.len()
                    ));
                };
                let prompt = format_fim_prompt(&model, prefix, suffix);
                let stop_tokens = get_fim_stop_tokens();

                (prompt, stop_tokens, None, inputs)
            };

            let request_body = open_ai::Request {
                model: model.clone(),
                messages: vec![open_ai::RequestMessage::User {
                    content: open_ai::MessageContent::Plain(prompt),
                }],
                stream: false,
                max_completion_tokens: Some(max_output_tokens as u64),
                stop: stop_tokens,
                temperature: Some(0.2),
                tool_choice: None,
                parallel_tool_calls: None,
                tools: vec![],
                prompt_cache_key: None,
                reasoning_effort: None,
            };

            let buf = serde_json::to_vec(&request_body)?;
            let body: http_client::AsyncBody = buf.into();

            let url = chat_completions_url(&api_url);

            let mut request_builder = http_client::Request::builder()
                .method(http_client::Method::POST)
                .uri(&url)
                .header("Content-Type", "application/json");

            if let Some(key) = &api_key {
                request_builder =
                    request_builder.header("Authorization", format!("Bearer {}", key));
            }

            let http_request = request_builder.body(body)?;

            let mut response = http_client.send(http_request).await?;
            let status = response.status();

            log::debug!("OpenAI Compatible: Response status: {}", status);

            if !status.is_success() {
                let mut body = String::new();
                response.body_mut().read_to_string(&mut body).await?;
                return Err(anyhow::anyhow!(
                    "OpenAI Compatible API error: {} - {}",
                    status,
                    body
                ));
            }

            let mut body_bytes: Vec<u8> = Vec::new();
            response
                .body_mut()
                .read_to_end(&mut body_bytes)
                .await
                .context("Failed to read response body")?;

            let response_received_at = Instant::now();

            log::debug!(
                "OpenAI Compatible: Completion received ({:.2}s)",
                (response_received_at - buffer_snapshotted_at).as_secs_f64()
            );

            let mut open_ai_response: open_ai::Response =
                serde_json::from_slice(&body_bytes).context("Failed to parse OpenAI response")?;

            let id = std::mem::take(&mut open_ai_response.id);
            let response_str = text_from_response(open_ai_response).unwrap_or_default();

            log::trace!("openai_compatible response: {}", response_str);

            let edits = if is_zeta {
                let editable_range =
                    editable_range_override.context("zeta model should have editable range")?;

                let cleaned = clean_zeta_completion(&response_str);
                match zeta1::parse_edits(cleaned, editable_range, &snapshot) {
                    Ok(edits) => edits,
                    Err(err) => {
                        log::warn!("OpenAI Compatible zeta: Failed to parse response: {}", err);
                        vec![]
                    }
                }
            } else {
                let completion: Arc<str> = clean_fim_completion(&response_str).into();
                if completion.is_empty() {
                    vec![]
                } else {
                    let cursor_offset = cursor_point.to_offset(&snapshot);
                    let anchor = snapshot.anchor_after(cursor_offset);
                    vec![(anchor..anchor, completion)]
                }
            };

            anyhow::Ok(RequestOutput {
                id,
                edits,
                snapshot,
                response_received_at,
                inputs,
                buffer,
                buffer_snapshotted_at,
            })
        });

        cx.spawn(async move |cx: &mut gpui::AsyncApp| {
            let output = result
                .await
                .context("OpenAI Compatible edit prediction failed")?;
            anyhow::Ok(Some(
                EditPredictionResult::new(
                    EditPredictionId(output.id.into()),
                    &output.buffer,
                    &output.snapshot,
                    output.edits.into(),
                    None,
                    output.buffer_snapshotted_at,
                    output.response_received_at,
                    output.inputs,
                    cx,
                )
                .await,
            ))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::chat_completions_url;

    #[test]
    fn chat_completions_url_appends_v1_when_missing() {
        assert_eq!(
            chat_completions_url("http://localhost:8000"),
            "http://localhost:8000/v1/chat/completions"
        );
        assert_eq!(
            chat_completions_url("http://localhost:8000/"),
            "http://localhost:8000/v1/chat/completions"
        );
    }

    #[test]
    fn chat_completions_url_does_not_duplicate_v1() {
        assert_eq!(
            chat_completions_url("http://localhost:8000/v1"),
            "http://localhost:8000/v1/chat/completions"
        );
        assert_eq!(
            chat_completions_url("http://localhost:8000/v1/"),
            "http://localhost:8000/v1/chat/completions"
        );
    }
}
