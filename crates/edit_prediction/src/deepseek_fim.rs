use crate::{
    EditPredictionId, EditPredictionModelInput, cursor_excerpt,
    prediction::EditPredictionResult,
};
use anyhow::{Context as _, Result, anyhow};
use futures::AsyncReadExt as _;
use gpui::{App, AppContext as _, Entity, Global, SharedString, Task, http_client};
use language::{
    Anchor, Buffer, BufferSnapshot, ToOffset, ToPoint as _,
    language_settings::all_language_settings,
};
use language_model::{ApiKeyState, EnvVar, env_var};
use serde::Serialize;
use std::{path::Path, sync::Arc, time::Instant};
use zeta_prompt::{ZetaPromptInput, compute_editable_and_context_ranges};

const DEEPSEEK_FIM_API_URL: &str = "https://api.deepseek.com/beta";
const FIM_CONTEXT_TOKENS: usize = 512;

pub static DEEPSEEK_API_KEY_ENV_VAR: std::sync::LazyLock<EnvVar> = env_var!("DEEPSEEK_API_KEY");

struct GlobalDeepseekFimApiKey(Entity<ApiKeyState>);

impl Global for GlobalDeepseekFimApiKey {}

pub fn deepseek_fim_api_url(cx: &App) -> SharedString {
    all_language_settings(None, cx)
        .edit_predictions
        .deepseek_fim
        .as_ref()
        .map(|settings| settings.api_url.clone())
        .unwrap_or_else(|| DEEPSEEK_FIM_API_URL.into())
        .into()
}

pub fn deepseek_fim_api_key_state(cx: &mut App) -> Entity<ApiKeyState> {
    if let Some(global) = cx.try_global::<GlobalDeepseekFimApiKey>() {
        return global.0.clone();
    }

    let entity = cx.new(|cx| {
        ApiKeyState::new(deepseek_fim_api_url(cx), DEEPSEEK_API_KEY_ENV_VAR.clone())
    });
    cx.set_global(GlobalDeepseekFimApiKey(entity.clone()));
    entity
}

fn load_deepseek_fim_api_token(
    cx: &mut App,
) -> Task<Result<(), language_model::AuthenticateError>> {
    let credentials_provider = zed_credentials_provider::global(cx);
    let api_url = deepseek_fim_api_url(cx);
    deepseek_fim_api_key_state(cx).update(cx, |key_state, cx| {
        key_state.load_if_needed(api_url, |s| s, credentials_provider, cx)
    })
}

pub fn load_deepseek_fim_api_key(cx: &mut App) -> Option<Arc<str>> {
    _ = load_deepseek_fim_api_token(cx);
    let url = deepseek_fim_api_url(cx);
    deepseek_fim_api_key_state(cx).read(cx).key(&url)
}

#[derive(Debug, Serialize)]
struct DeepseekFimRequest {
    model: String,
    prompt: String,
    suffix: String,
    max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    stop: Vec<String>,
}

struct DeepseekFimRequestOutput {
    request_id: String,
    edits: Vec<(std::ops::Range<Anchor>, Arc<str>)>,
    snapshot: BufferSnapshot,
    inputs: ZetaPromptInput,
    buffer: Entity<Buffer>,
}

pub fn request_prediction(
    EditPredictionModelInput {
        buffer,
        snapshot,
        position,
        events,
        ..
    }: EditPredictionModelInput,
    cx: &mut App,
) -> Task<Result<Option<EditPredictionResult>>> {
    let settings = &all_language_settings(None, cx).edit_predictions;

    let Some(settings) = settings.deepseek_fim.clone() else {
        return Task::ready(Err(anyhow!("DeepseekFim edit prediction settings not configured")));
    };

    let full_path: Arc<Path> = snapshot
        .file()
        .map(|file| file.full_path(cx))
        .unwrap_or_else(|| "untitled".into())
        .into();

    let http_client = cx.http_client();
    let cursor_point = position.to_point(&snapshot);
    let request_start = cx.background_executor().now();
    let api_key = load_deepseek_fim_api_key(cx);

    let result = cx.background_spawn(async move {
        let cursor_offset = cursor_point.to_offset(&snapshot);
        let (excerpt_point_range, excerpt_offset_range, cursor_offset_in_excerpt) =
            cursor_excerpt::compute_cursor_excerpt(&snapshot, cursor_offset);
        let cursor_excerpt: Arc<str> = snapshot
            .text_for_range(excerpt_point_range.clone())
            .collect::<String>()
            .into();
        let syntax_ranges =
            cursor_excerpt::compute_syntax_ranges(&snapshot, cursor_offset, &excerpt_offset_range);
        let (editable_range, _) = compute_editable_and_context_ranges(
            &cursor_excerpt,
            cursor_offset_in_excerpt,
            &syntax_ranges,
            FIM_CONTEXT_TOKENS,
            0,
        );

        let inputs = ZetaPromptInput {
            events,
            related_files: Some(Vec::new()),
            active_buffer_diagnostics: Vec::new(),
            cursor_offset_in_excerpt: cursor_offset - excerpt_offset_range.start,
            cursor_path: full_path.clone(),
            excerpt_start_row: Some(excerpt_point_range.start.row),
            cursor_excerpt,
            excerpt_ranges: Default::default(),
            syntax_ranges: None,
            in_open_source_repo: false,
            can_collect_data: false,
            repo_url: None,
        };

        let editable_text = &inputs.cursor_excerpt[editable_range.clone()];
        let cursor_in_editable = cursor_offset_in_excerpt.saturating_sub(editable_range.start);
        let prefix = editable_text[..cursor_in_editable].to_string();
        let suffix = editable_text[cursor_in_editable..].to_string();
        let stop_tokens = get_fim_stop_tokens();

        let max_tokens = settings.max_output_tokens;

        let request = DeepseekFimRequest {
            model: settings.model.clone(),
            prompt: prefix,
            suffix,
            max_tokens,
            temperature: Some(0.2),
            stop: stop_tokens,
        };

        let request_body = serde_json::to_string(&request)?;
        let mut http_request_builder = http_client::Request::builder()
            .method(http_client::Method::POST)
            .uri(format!("{}/completions", settings.api_url))
            .header("Content-Type", "application/json");

        if let Some(api_key) = api_key {
            http_request_builder =
                http_request_builder.header("Authorization", format!("Bearer {}", api_key));
        }

        let http_request =
            http_request_builder.body(http_client::AsyncBody::from(request_body))?;

        let mut response = http_client.send(http_request).await?;
        let status = response.status();

        if !status.is_success() {
            let mut body = String::new();
            response.body_mut().read_to_string(&mut body).await?;
            anyhow::bail!("DeepseekFim API error: {} - {}", status, body);
        }

        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;

        let parsed: cloud_llm_client::predict_edits_v3::RawCompletionResponse =
            serde_json::from_str(&body).context("Failed to parse DeepseekFim completion response")?;
        let text = parsed
            .choices
            .into_iter()
            .next()
            .map(|choice| choice.text)
            .unwrap_or_default();

        let response_received_at = Instant::now();

        log::debug!(
            "deepseek_fim: completion received ({:.2}s)",
            (response_received_at - request_start).as_secs_f64()
        );

        let completion: Arc<str> = text.into();
        let edits = if completion.is_empty() {
            vec![]
        } else {
            let cursor_offset = cursor_point.to_offset(&snapshot);
            let anchor = snapshot.anchor_after(cursor_offset);
            vec![(anchor..anchor, completion)]
        };

        anyhow::Ok(DeepseekFimRequestOutput {
            request_id: parsed.id,
            edits,
            snapshot,
            inputs,
            buffer,
        })
    });

    cx.spawn(async move |cx: &mut gpui::AsyncApp| {
        let output = result.await.context("deepseek_fim fim edit prediction failed")?;
        anyhow::Ok(Some(
            EditPredictionResult::new(
                EditPredictionId(output.request_id.into()),
                &output.buffer,
                &output.snapshot,
                output.edits.into(),
                None,
                output.inputs,
                None,
                cx.background_executor().now() - request_start,
                cx,
            )
            .await,
        ))
    })
}

fn get_fim_stop_tokens() -> Vec<String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deepseek_fim_request_serialization() {
        let request = DeepseekFimRequest {
            model: "deepseek-v4-pro".to_string(),
            prompt: "def fib(a):\n    ".to_string(),
            suffix: "\n    return fib(a-1) + fib(a-2)".to_string(),
            max_tokens: 128,
            temperature: Some(0.2),
            stop: vec!["<|endoftext|>".to_string()],
        };

        let json = serde_json::to_string(&request).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["model"], "deepseek-v4-pro");
        assert_eq!(parsed["prompt"], "def fib(a):\n    ");
        assert_eq!(parsed["suffix"], "\n    return fib(a-1) + fib(a-2)");
        assert_eq!(parsed["max_tokens"], 128);
        assert_eq!(parsed["temperature"], 0.2);
        assert_eq!(parsed["stop"], serde_json::json!(["<|endoftext|>"]));
    }

    #[test]
    fn test_deepseek_fim_response_deserialization() {
        // Sample response from the actual DeepSeek FIM beta endpoint
        let response_json = r#"{
            "id": "00000000-0000-0000-0000-000000000000",
            "choices": [
                {
                    "text": "if a < 2:\n        return a",
                    "index": 0,
                    "logprobs": null,
                    "finish_reason": "stop"
                }
            ],
            "created": 0,
            "model": "deepseek-v4-pro",
            "system_fingerprint": "fp_redacted",
            "object": "text_completion",
            "usage": {
                "prompt_tokens": 23,
                "completion_tokens": 9,
                "total_tokens": 32,
                "prompt_tokens_details": {
                    "cached_tokens": 0
                },
                "completion_tokens_details": {
                    "reasoning_tokens": 0
                },
                "prompt_cache_hit_tokens": 0,
                "prompt_cache_miss_tokens": 23
            }
        }"#;

        let response: cloud_llm_client::predict_edits_v3::RawCompletionResponse =
            serde_json::from_str(response_json).unwrap();

        assert_eq!(response.id, "00000000-0000-0000-0000-000000000000");
        assert_eq!(response.model, "deepseek-v4-pro");
        assert_eq!(response.choices.len(), 1);
        assert_eq!(response.choices[0].text, "if a < 2:\n        return a");
        assert_eq!(response.choices[0].finish_reason.as_deref(), Some("stop"));
        assert_eq!(response.usage.prompt_tokens, 23);
        assert_eq!(response.usage.completion_tokens, 9);
        assert_eq!(response.usage.total_tokens, 32);
    }
}
