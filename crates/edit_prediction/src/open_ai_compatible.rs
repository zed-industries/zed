use crate::{
    EditPredictionId, EditPredictionModelInput, cursor_excerpt,
    ollama::{
        clean_fim_completion, clean_zeta_completion, format_fim_prompt, get_fim_stop_tokens,
        get_zeta_stop_tokens, is_zeta_model,
    },
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
use serde::{Deserialize, Serialize};
use std::{path::Path, sync::Arc, time::Instant};
use zeta_prompt::{ZetaPromptInput, zeta1::format_zeta1_prompt};

#[derive(Serialize)]
struct CompletionRequest {
    model: String,
    prompt: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    stop: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    stream: bool,
}

#[derive(Deserialize)]
struct CompletionResponse {
    id: String,
    choices: Vec<CompletionChoice>,
}

#[derive(Deserialize)]
struct CompletionChoice {
    text: String,
}

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

fn completions_url(api_url: &str) -> String {
    let trimmed = api_url.trim_end_matches('/');
    if trimmed.ends_with("/v1") {
        format!("{}/completions", trimmed)
    } else {
        format!("{}/v1/completions", trimmed)
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

            let request_body = CompletionRequest {
                model: model.clone(),
                prompt,
                max_tokens: Some(max_output_tokens as u64),
                stop: stop_tokens,
                temperature: Some(0.2),
                stream: false,
            };

            let buf = serde_json::to_vec(&request_body)?;
            let body: http_client::AsyncBody = buf.into();

            let url = completions_url(&api_url);

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

            let completion_response: CompletionResponse = serde_json::from_slice(&body_bytes)
                .context("Failed to parse completions response")?;

            let id = completion_response.id;
            let response_str = completion_response
                .choices
                .into_iter()
                .next()
                .map(|c| c.text)
                .unwrap_or_default();

            log::trace!("open_ai_compatible response: {}", response_str);

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
    use super::{CompletionRequest, completions_url};
    use crate::ollama::{
        format_fim_prompt, get_fim_stop_tokens, get_zeta_stop_tokens, is_zeta_model,
    };

    #[test]
    fn completions_url_appends_v1_when_missing() {
        assert_eq!(
            completions_url("http://localhost:8000"),
            "http://localhost:8000/v1/completions"
        );
        assert_eq!(
            completions_url("http://localhost:8000/"),
            "http://localhost:8000/v1/completions"
        );
    }

    #[test]
    fn completions_url_does_not_duplicate_v1() {
        assert_eq!(
            completions_url("http://localhost:8000/v1"),
            "http://localhost:8000/v1/completions"
        );
        assert_eq!(
            completions_url("http://localhost:8000/v1/"),
            "http://localhost:8000/v1/completions"
        );
    }

    #[test]
    fn completions_url_with_custom_path_prefix() {
        assert_eq!(
            completions_url("http://localhost:8000/api/v1"),
            "http://localhost:8000/api/v1/completions"
        );
        assert_eq!(
            completions_url("http://my-server.example.com/v1"),
            "http://my-server.example.com/v1/completions"
        );
    }

    #[test]
    fn is_zeta_model_detection() {
        assert!(is_zeta_model("zeta"));
        assert!(is_zeta_model("Zeta"));
        assert!(is_zeta_model("my-org/zeta-v1"));
        assert!(is_zeta_model("ZETA-large"));
        assert!(!is_zeta_model("qwen2.5-coder:3b-base"));
        assert!(!is_zeta_model("codellama"));
        assert!(!is_zeta_model("starcoder2"));
    }

    #[test]
    fn zeta_request_body_structure() {
        let stop_tokens = get_zeta_stop_tokens();
        let request = CompletionRequest {
            model: "zeta".into(),
            prompt: "test prompt".into(),
            stream: false,
            max_tokens: Some(1024),
            stop: stop_tokens.clone(),
            temperature: Some(0.2),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["model"], "zeta");
        assert_eq!(json["stream"], false);
        assert_eq!(json["max_tokens"], 1024);
        let temperature = json["temperature"].as_f64().unwrap();
        assert!(
            (temperature - 0.2).abs() < 0.001,
            "temperature should be ~0.2, got {temperature}"
        );
        assert_eq!(json["prompt"], "test prompt");

        let stop_arr = json["stop"].as_array().unwrap();
        assert!(stop_arr.len() >= 2);
        for token in &stop_tokens {
            assert!(
                stop_arr.iter().any(|v| v.as_str() == Some(token)),
                "stop tokens should contain {token}"
            );
        }
    }

    #[test]
    fn fim_request_body_structure() {
        let prefix = "fn main() {\n    let x = ";
        let suffix = ";\n}\n";
        let prompt = format_fim_prompt("qwen2.5-coder", prefix, suffix);
        let stop_tokens = get_fim_stop_tokens();

        let request = CompletionRequest {
            model: "qwen2.5-coder".into(),
            prompt: prompt.clone(),
            stream: false,
            max_tokens: Some(256),
            stop: stop_tokens,
            temperature: Some(0.2),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["model"], "qwen2.5-coder");
        assert_eq!(json["max_tokens"], 256);

        let content = json["prompt"].as_str().unwrap();
        assert!(
            content.contains(prefix),
            "FIM prompt should contain the prefix"
        );
        assert!(
            content.contains(suffix),
            "FIM prompt should contain the suffix"
        );
    }

    #[test]
    fn authorization_header_only_when_key_present() {
        let mut builder = gpui::http_client::Request::builder()
            .method(gpui::http_client::Method::POST)
            .uri("http://localhost:8000/v1/completions")
            .header("Content-Type", "application/json");

        let api_key: Option<String> = None;
        if let Some(key) = &api_key {
            builder = builder.header("Authorization", format!("Bearer {}", key));
        }
        let request = builder.body(gpui::http_client::AsyncBody::empty()).unwrap();
        assert!(request.headers().get("Authorization").is_none());

        let mut builder = gpui::http_client::Request::builder()
            .method(gpui::http_client::Method::POST)
            .uri("http://localhost:8000/v1/completions")
            .header("Content-Type", "application/json");

        let api_key = Some("sk-test-key".to_string());
        if let Some(key) = &api_key {
            builder = builder.header("Authorization", format!("Bearer {}", key));
        }
        let request = builder.body(gpui::http_client::AsyncBody::empty()).unwrap();
        assert_eq!(
            request.headers().get("Authorization").unwrap(),
            "Bearer sk-test-key"
        );
    }
}
