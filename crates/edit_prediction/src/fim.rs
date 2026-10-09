use crate::{
    EditPredictionId, EditPredictionInputs, EditPredictionModelInput, cursor_excerpt,
    open_ai_compatible::{self, load_open_ai_compatible_api_key_if_needed},
    prediction::EditPredictionResult,
    qwen,
};
use anyhow::{Context as _, Result, anyhow};
use gpui::{App, AppContext as _, Entity, Task, http_client};
use language::{
    Anchor, Buffer, BufferSnapshot, EditPredictionPromptFormat, ToOffset, ToPoint as _,
    ZetaVersion,
    language_settings::{OpenAiCompatibleEditPredictionSettings, all_language_settings},
};
use std::{path::Path, sync::Arc, time::Instant};
use zeta_prompt::{Zeta2PromptInput, compute_editable_and_context_ranges};

const FIM_CONTEXT_TOKENS: usize = 512;

struct FimRequestOutput {
    request_id: String,
    edits: Vec<(std::ops::Range<Anchor>, Arc<str>)>,
    editable_range: std::ops::Range<Anchor>,
    snapshot: BufferSnapshot,
    inputs: Zeta2PromptInput,
    buffer: Entity<Buffer>,
}

pub fn request_prediction(
    EditPredictionModelInput {
        buffer,
        snapshot,
        position,
        events,
        trigger,
        ..
    }: EditPredictionModelInput,
    prompt_format: EditPredictionPromptFormat,
    cx: &mut App,
) -> Task<Result<Option<EditPredictionResult>>> {
    let settings = &all_language_settings(None, cx).edit_predictions;
    let provider = settings.provider;

    let full_path: Arc<Path> = snapshot
        .file()
        .map(|file| file.full_path(cx))
        .unwrap_or_else(|| "untitled".into())
        .into();

    let http_client = cx.http_client();
    let cursor_point = position.to_point(&snapshot);
    let request_start = cx.background_executor().now();

    let Some(settings) = (match provider {
        settings::EditPredictionProvider::Ollama => settings.ollama.clone(),
        settings::EditPredictionProvider::OpenAiCompatibleApi => {
            settings.open_ai_compatible_api.clone()
        }
        _ => None,
    }) else {
        return Task::ready(Err(anyhow!("Unsupported edit prediction provider for FIM")));
    };

    let api_key = load_open_ai_compatible_api_key_if_needed(provider, cx);

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

        let inputs = Zeta2PromptInput {
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
        let (response_text, request_id) = send_fim_request(
            provider,
            prompt_format,
            &settings,
            &prefix,
            &suffix,
            api_key,
            &http_client,
        )
        .await?;

        let response_received_at = Instant::now();

        log::debug!(
            "fim: completion received ({:.2}s)",
            (response_received_at - request_start).as_secs_f64()
        );

        let completion: Arc<str> = clean_fim_completion(&response_text).into();
        let edits = if completion.is_empty() {
            vec![]
        } else {
            let cursor_offset = cursor_point.to_offset(&snapshot);
            let anchor = snapshot.anchor_after(cursor_offset);
            vec![(anchor..anchor, completion)]
        };

        let editable_range = snapshot.anchor_range_inside(
            (excerpt_offset_range.start + editable_range.start)
                ..(excerpt_offset_range.start + editable_range.end),
        );

        anyhow::Ok(FimRequestOutput {
            request_id,
            edits,
            editable_range,
            snapshot,
            inputs,
            buffer,
        })
    });

    cx.spawn(async move |cx: &mut gpui::AsyncApp| {
        let output = result.await.context("fim edit prediction failed")?;
        anyhow::Ok(Some(
            EditPredictionResult::new(
                EditPredictionId(output.request_id.into()),
                &output.buffer,
                &output.snapshot,
                output.edits.into(),
                None,
                Some(output.editable_range),
                EditPredictionInputs::V2(output.inputs),
                None,
                trigger,
                cx.background_executor().now() - request_start,
                cx,
            )
            .await,
        ))
    })
}

async fn send_fim_request(
    provider: settings::EditPredictionProvider,
    prompt_format: EditPredictionPromptFormat,
    settings: &OpenAiCompatibleEditPredictionSettings,
    prefix: &str,
    suffix: &str,
    api_key: Option<Arc<str>>,
    http_client: &Arc<dyn http_client::HttpClient>,
) -> Result<(String, String)> {
    let response = match (provider, prompt_format) {
        (
            settings::EditPredictionProvider::OpenAiCompatibleApi,
            EditPredictionPromptFormat::Qwen,
        ) => qwen::try_request(settings, prefix, suffix, api_key.clone(), http_client).await?,
        _ => None,
    };

    match response {
        Some(response) => Ok(response),
        None => {
            open_ai_compatible::send_custom_server_request(
                provider,
                settings,
                format_fim_prompt(prompt_format, prefix, suffix),
                settings.max_output_tokens,
                get_fim_stop_tokens(),
                api_key,
                http_client,
            )
            .await
        }
    }
}

/// Infers the FIM prompt format from an Ollama/OpenAI-compatible model name.
/// Returns `None` if the model isn't a known FIM-capable model.
pub fn infer_prompt_format(model: &str) -> Option<EditPredictionPromptFormat> {
    let model_base = model.split(':').next().unwrap_or(model);

    Some(match model_base {
        "zeta2" => EditPredictionPromptFormat::Zeta(ZetaVersion::Zeta2),
        "zeta2.1" => EditPredictionPromptFormat::Zeta(ZetaVersion::Zeta2_1),
        model_base if model_base.to_ascii_lowercase().contains("sweep-next-edit") => {
            EditPredictionPromptFormat::Sweep
        }
        "codellama" | "code-llama" => EditPredictionPromptFormat::CodeLlama,
        "starcoder" | "starcoder2" | "starcoderbase" => EditPredictionPromptFormat::StarCoder,
        "deepseek-coder" | "deepseek-coder-v2" => EditPredictionPromptFormat::DeepseekCoder,
        "qwen2.5-coder" | "qwen-coder" | "qwen" | "qwen3-coder" | "qwen3.8-flash" => {
            EditPredictionPromptFormat::Qwen
        }
        model_base if model_base.starts_with("qwen3-coder-") => EditPredictionPromptFormat::Qwen,
        "codegemma" => EditPredictionPromptFormat::CodeGemma,
        "codestral" | "mistral" => EditPredictionPromptFormat::Codestral,
        "glm" | "glm-4" | "glm-4.5" => EditPredictionPromptFormat::Glm,
        _ => {
            return None;
        }
    })
}

fn format_fim_prompt(
    prompt_format: EditPredictionPromptFormat,
    prefix: &str,
    suffix: &str,
) -> String {
    match prompt_format {
        EditPredictionPromptFormat::CodeLlama => {
            format!("<PRE> {prefix} <SUF>{suffix} <MID>")
        }
        EditPredictionPromptFormat::StarCoder => {
            format!("<fim_prefix>{prefix}<fim_suffix>{suffix}<fim_middle>")
        }
        EditPredictionPromptFormat::DeepseekCoder => {
            format!("<｜fim▁begin｜>{prefix}<｜fim▁hole｜>{suffix}<｜fim▁end｜>")
        }
        EditPredictionPromptFormat::Qwen | EditPredictionPromptFormat::CodeGemma => {
            format!("<|fim_prefix|>{prefix}<|fim_suffix|>{suffix}<|fim_middle|>")
        }
        EditPredictionPromptFormat::Codestral => {
            format!("[SUFFIX]{suffix}[PREFIX]{prefix}")
        }
        EditPredictionPromptFormat::Glm => {
            format!("<|code_prefix|>{prefix}<|code_suffix|>{suffix}<|code_middle|>")
        }
        _ => {
            format!("<fim_prefix>{prefix}<fim_suffix>{suffix}<fim_middle>")
        }
    }
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

fn clean_fim_completion(response: &str) -> String {
    let mut result = response.to_string();

    let end_tokens = [
        "<|endoftext|>",
        "<|im_end|>",
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

#[cfg(test)]
mod tests {
    use super::*;
    use futures::AsyncReadExt as _;
    use gpui::http_client::FakeHttpClient;
    use serde_json::{Value, json};
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn request_dispatch_preserves_raw_fim_formats() -> Result<()> {
        futures::executor::block_on(async {
            let chat_url = "https://example.com/v1/chat/completions";
            for (format, api_url, expected_prompt) in [
                (
                    EditPredictionPromptFormat::CodeLlama,
                    chat_url,
                    "<PRE> before <SUF>after <MID>",
                ),
                (
                    EditPredictionPromptFormat::StarCoder,
                    chat_url,
                    "<fim_prefix>before<fim_suffix>after<fim_middle>",
                ),
                (
                    EditPredictionPromptFormat::DeepseekCoder,
                    chat_url,
                    "<｜fim▁begin｜>before<｜fim▁hole｜>after<｜fim▁end｜>",
                ),
                (
                    EditPredictionPromptFormat::CodeGemma,
                    chat_url,
                    "<|fim_prefix|>before<|fim_suffix|>after<|fim_middle|>",
                ),
                (
                    EditPredictionPromptFormat::Codestral,
                    chat_url,
                    "[SUFFIX]after[PREFIX]before",
                ),
                (
                    EditPredictionPromptFormat::Glm,
                    chat_url,
                    "<|code_prefix|>before<|code_suffix|>after<|code_middle|>",
                ),
                (
                    EditPredictionPromptFormat::Qwen,
                    "https://example.com/v1/completions",
                    "<|fim_prefix|>before<|fim_suffix|>after<|fim_middle|>",
                ),
            ] {
                let http_client: Arc<dyn http_client::HttpClient> =
                    FakeHttpClient::create(move |mut request| async move {
                        assert_eq!(request.uri().to_string(), api_url);
                        let mut body = String::new();
                        request.body_mut().read_to_string(&mut body).await?;
                        let body: Value = serde_json::from_str(&body)?;
                        assert_eq!(body["prompt"], expected_prompt, "{format:?}");
                        assert!(body.get("messages").is_none(), "{format:?}");
                        Ok(http_client::Response::builder().status(200).body(json!({
                        "id": "raw-request", "object": "text_completion", "created": 0,
                        "model": "test-completion-model",
                        "choices": [{"text": "completion", "finish_reason": "stop"}],
                        "usage": {"prompt_tokens": 0, "completion_tokens": 0, "total_tokens": 0}
                    }).to_string().into())?)
                    });
                let settings = OpenAiCompatibleEditPredictionSettings {
                    model: "test-completion-model".into(),
                    api_url: api_url.into(),
                    max_output_tokens: 64,
                    ..Default::default()
                };
                assert_eq!(
                    send_fim_request(
                        settings::EditPredictionProvider::OpenAiCompatibleApi,
                        format,
                        &settings,
                        "before",
                        "after",
                        None,
                        &http_client
                    )
                    .await?,
                    ("completion".into(), "raw-request".into()),
                    "{format:?}"
                );
            }
            Ok(())
        })
    }

    #[test]
    fn request_dispatch_keeps_the_selected_provider_transport() -> Result<()> {
        futures::executor::block_on(async {
            let http_client: Arc<dyn http_client::HttpClient> =
                FakeHttpClient::create(|mut request| async move {
                    assert_eq!(request.uri().path(), "/v1/chat/completions/api/generate");
                    let mut body = String::new();
                    request.body_mut().read_to_string(&mut body).await?;
                    let body: Value = serde_json::from_str(&body)?;
                    assert_eq!(body["raw"], true);
                    assert!(body.get("messages").is_none());
                    Ok(http_client::Response::builder().status(200).body(
                        json!({
                            "created_at": "provider-request", "response": "completion"
                        })
                        .to_string()
                        .into(),
                    )?)
                });
            let settings = OpenAiCompatibleEditPredictionSettings {
                api_url: "https://example.com/v1/chat/completions".into(),
                ..Default::default()
            };
            assert_eq!(
                send_fim_request(
                    settings::EditPredictionProvider::Ollama,
                    EditPredictionPromptFormat::Qwen,
                    &settings,
                    "before",
                    "after",
                    None,
                    &http_client
                )
                .await?,
                ("completion".into(), "provider-request".into())
            );
            Ok(())
        })
    }

    #[test]
    fn request_dispatch_does_not_retry_a_failed_adapter() -> Result<()> {
        futures::executor::block_on(async {
            let request_count = Arc::new(AtomicUsize::new(0));
            let http_client: Arc<dyn http_client::HttpClient> = FakeHttpClient::create({
                let request_count = request_count.clone();
                move |_request| {
                    request_count.fetch_add(1, Ordering::SeqCst);
                    async move {
                        Ok(http_client::Response::builder()
                            .status(400)
                            .body("adapter request rejected".into())?)
                    }
                }
            });
            let settings = OpenAiCompatibleEditPredictionSettings {
                api_url: "https://example.com/v1/chat/completions".into(),
                ..Default::default()
            };
            let error = send_fim_request(
                settings::EditPredictionProvider::OpenAiCompatibleApi,
                EditPredictionPromptFormat::Qwen,
                &settings,
                "before",
                "after",
                None,
                &http_client,
            )
            .await
            .err()
            .context("Expected the adapter error to propagate")?;
            assert!(error.to_string().contains("400 Bad Request"));
            assert!(error.to_string().contains("adapter request rejected"));
            assert_eq!(request_count.load(Ordering::SeqCst), 1);
            Ok(())
        })
    }

    #[test]
    fn request_dispatch_keeps_an_empty_successful_completion() -> Result<()> {
        futures::executor::block_on(async {
            let request_count = Arc::new(AtomicUsize::new(0));
            let http_client: Arc<dyn http_client::HttpClient> = FakeHttpClient::create({
                let request_count = request_count.clone();
                move |_request| {
                    request_count.fetch_add(1, Ordering::SeqCst);
                    async move {
                        Ok(http_client::Response::builder().status(200).body(json!({
                            "id": "empty-request", "object": "chat.completion", "created": 0,
                            "model": "test-completion-model",
                            "choices": [{"message": {"role": "assistant", "content": ""}, "finish_reason": "stop"}],
                            "usage": {"prompt_tokens": 0, "completion_tokens": 0, "total_tokens": 0}
                        }).to_string().into())?)
                    }
                }
            });
            let settings = OpenAiCompatibleEditPredictionSettings {
                api_url: "https://example.com/v1/chat/completions".into(),
                ..Default::default()
            };
            assert_eq!(
                send_fim_request(
                    settings::EditPredictionProvider::OpenAiCompatibleApi,
                    EditPredictionPromptFormat::Qwen,
                    &settings,
                    "before",
                    "after",
                    None,
                    &http_client
                )
                .await?,
                (String::new(), "empty-request".into())
            );
            assert_eq!(request_count.load(Ordering::SeqCst), 1);
            Ok(())
        })
    }

    #[test]
    fn infer_prompt_format_matches_known_model_families() {
        use EditPredictionPromptFormat::*;
        for (model, expected_format) in [
            ("codellama:7b", CodeLlama),
            ("starcoder2:3b", StarCoder),
            ("deepseek-coder:6.7b", DeepseekCoder),
            ("deepseek-coder-v2:16b", DeepseekCoder),
            ("qwen2.5-coder:3b", Qwen),
            ("qwen3-coder", Qwen),
            ("qwen3-coder:30b", Qwen),
            ("qwen3-coder-plus", Qwen),
            ("qwen3-coder-flash", Qwen),
            ("qwen3-coder-plus-2025-09-23", Qwen),
            ("qwen3.8-flash", Qwen),
            ("codegemma:7b", CodeGemma),
            ("codestral:latest", Codestral),
            ("glm-4:9b", Glm),
            ("glm-4.5:latest", Glm),
            ("zeta2", Zeta(ZetaVersion::Zeta2)),
            ("zeta2.1", Zeta(ZetaVersion::Zeta2_1)),
            ("my-sweep-next-edit-v1", Sweep),
        ] {
            assert_eq!(infer_prompt_format(model), Some(expected_format), "{model}");
        }
    }

    #[test]
    fn completion_cleanup_preserves_whitespace_and_strips_stop_tokens() {
        for token in ["<|endoftext|>", "<|im_end|>", "<PRE>", "<|fim_suffix|>"] {
            assert_eq!(
                clean_fim_completion(&format!("    return n\n{token}extra")),
                "    return n\n",
                "{token}"
            );
        }
    }

    #[test]
    fn infer_prompt_format_returns_none_for_unsupported_models() {
        for model in ["qwen3:8b", "llama3:70b", "phi3:mini", "nomic-embed-text"] {
            assert_eq!(infer_prompt_format(model), None, "{model}");
        }
    }
}
