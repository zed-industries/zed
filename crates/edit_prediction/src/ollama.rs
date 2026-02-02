use crate::{
    EditPredictionId, EditPredictionModelInput, prediction::EditPredictionResult,
    zeta1::compute_edits,
};
use anyhow::{Context as _, Result};
use futures::AsyncReadExt as _;
use gpui::{App, AppContext as _, Task, http_client};
use language::{OffsetRangeExt as _, ToOffset, ToPoint as _};
use language_model::{LanguageModelProviderId, LanguageModelRegistry};
use serde::{Deserialize, Serialize};
use std::{path::Path, sync::Arc, time::Instant};
use zeta_prompt::ZetaPromptInput;

const MAX_REWRITE_TOKENS: usize = 150;
const MAX_CONTEXT_TOKENS: usize = 350;

pub struct Ollama {
    model_name: Option<String>,
    api_url: String,
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

pub fn is_available(cx: &App) -> bool {
    let ollama_provider_id = LanguageModelProviderId::new("ollama");
    LanguageModelRegistry::read_global(cx)
        .provider(&ollama_provider_id)
        .is_some_and(|provider| provider.is_authenticated(cx))
}

impl Ollama {
    pub fn new() -> Self {
        Ollama {
            model_name: None,
            api_url: "http://localhost:11434".to_string(),
        }
    }

    pub fn request_prediction(
        &self,
        EditPredictionModelInput {
            buffer,
            snapshot,
            position,
            events,
            related_files,
            ..
        }: EditPredictionModelInput,
        cx: &mut App,
    ) -> Task<Result<Option<EditPredictionResult>>> {
        let model = self
            .model_name
            .as_deref()
            .unwrap_or("qwen2.5-coder:3b-base")
            .to_string();

        log::debug!("Ollama: Requesting completion (model: {})", model);

        let full_path: Arc<Path> = snapshot
            .file()
            .map(|file| file.full_path(cx))
            .unwrap_or_else(|| "untitled".into())
            .into();

        let http_client = cx.http_client();
        let cursor_point = position.to_point(&snapshot);
        let buffer_snapshotted_at = Instant::now();
        let api_url = self.api_url.clone();

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

            let inputs = ZetaPromptInput {
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

            let prefix = inputs.cursor_excerpt[..inputs.cursor_offset_in_excerpt].to_string();
            let suffix = inputs.cursor_excerpt[inputs.cursor_offset_in_excerpt..].to_string();

            let fim_prompt = format_fim_prompt(&model, &prefix, &suffix);

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

            log::debug!("Ollama: Sending FIM request");

            let request_body = serde_json::to_string(&request)?;
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

            let response_received_at = Instant::now();

            log::debug!(
                "Ollama: Completion received ({:.2}s)",
                (response_received_at - buffer_snapshotted_at).as_secs_f64()
            );

            let completion = clean_completion(&ollama_response.response);

            let old_text = snapshot
                .text_for_range(editable_offset_range.clone())
                .collect::<String>();
            let edits = compute_edits(
                old_text,
                &completion,
                editable_offset_range.start,
                &snapshot,
            );

            anyhow::Ok((edits, snapshot, response_received_at, inputs))
        });

        cx.spawn(async move |cx: &mut gpui::AsyncApp| {
            let (edits, old_snapshot, response_received_at, inputs) =
                result.await.context("Ollama edit prediction failed")?;
            anyhow::Ok(Some(
                EditPredictionResult::new(
                    EditPredictionId(String::new().into()),
                    &buffer,
                    &old_snapshot,
                    edits.into(),
                    None,
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
