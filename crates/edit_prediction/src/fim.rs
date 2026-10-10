use crate::{
    EditPredictionId, EditPredictionInputs, EditPredictionModelInput, cursor_excerpt, deepseek_fim,
    open_ai_compatible::{self, load_open_ai_compatible_api_key_if_needed},
    prediction::EditPredictionResult,
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
    if provider == settings::EditPredictionProvider::OpenAiCompatibleApi {
        if let Some(response) =
            deepseek_fim::try_request(settings, prefix, suffix, api_key.clone(), http_client)
                .await?
        {
            return Ok(response);
        }
    }

    open_ai_compatible::send_custom_server_request(
        provider,
        settings,
        format_fim_prompt(prompt_format, prefix, suffix),
        settings.max_output_tokens,
        fim_stop_tokens(prompt_format)
            .iter()
            .map(|token| token.to_string())
            .collect(),
        api_key,
        http_client,
    )
    .await
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
        "deepseek-flash" => {
            // This selects the FIM route; the official API uses a native suffix instead of Coder markers.
            EditPredictionPromptFormat::DeepseekCoder
        }
        "qwen2.5-coder" | "qwen-coder" | "qwen" => EditPredictionPromptFormat::Qwen,
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

// OpenAI-compatible servers such as vLLM reject requests with more than 4 stop
// sequences, so each format may only list up to 4 tokens.
fn fim_stop_tokens(prompt_format: EditPredictionPromptFormat) -> &'static [&'static str] {
    match prompt_format {
        EditPredictionPromptFormat::CodeLlama => &["<PRE>", "<SUF>", "<MID>"],
        EditPredictionPromptFormat::DeepseekCoder => {
            &["<｜fim▁begin｜>", "<｜fim▁hole｜>", "<｜fim▁end｜>"]
        }
        EditPredictionPromptFormat::Qwen => &[
            "<|endoftext|>",
            "<|fim_prefix|>",
            "<|fim_middle|>",
            "<|fim_suffix|>",
        ],
        EditPredictionPromptFormat::CodeGemma => &[
            "<|file_separator|>",
            "<|fim_prefix|>",
            "<|fim_middle|>",
            "<|fim_suffix|>",
        ],
        EditPredictionPromptFormat::Codestral => &["[PREFIX]", "[SUFFIX]"],
        EditPredictionPromptFormat::Glm => &[
            "<|endoftext|>",
            "<|code_prefix|>",
            "<|code_suffix|>",
            "<|code_middle|>",
        ],
        _ => &[
            "<|endoftext|>",
            "<fim_prefix>",
            "<fim_middle>",
            "<fim_suffix>",
        ],
    }
}

fn clean_fim_completion(response: &str) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;
    use futures::AsyncReadExt as _;
    use gpui::http_client::FakeHttpClient;
    use serde_json::{Value, json};

    #[test]
    fn deepseek_flash_uses_native_fim_with_separate_suffix() -> Result<()> {
        futures::executor::block_on(async {
            for api_url in [
                "https://api.deepseek.com/v1/completions",
                "https://api.deepseek.com/v1/chat/completions",
                "https://api.deepseek.com/beta/completions",
            ] {
                let http_client: Arc<dyn http_client::HttpClient> = FakeHttpClient::create(
                    |mut request| async move {
                        assert_eq!(
                            request.uri().to_string(),
                            "https://api.deepseek.com/beta/completions"
                        );
                        let mut body = String::new();
                        request.body_mut().read_to_string(&mut body).await?;
                        let body: Value = serde_json::from_str(&body)?;
                        assert_eq!(
                            body,
                            json!({
                                "model": "deepseek-flash",
                                "prompt": "def square(number):\n    return number ",
                                "suffix": " number\n",
                                "max_tokens": 32
                            })
                        );
                        Ok(http_client::Response::builder().status(200).body(json!({
                            "id": "native-fim", "object": "text_completion", "created": 0,
                            "model": "deepseek-flash",
                            "choices": [{"text": "*", "finish_reason": "stop"}],
                            "usage": {"prompt_tokens": 10, "completion_tokens": 1, "total_tokens": 11}
                        }).to_string().into())?)
                    },
                );
                let settings = OpenAiCompatibleEditPredictionSettings {
                    model: "deepseek-flash".into(),
                    api_url: api_url.into(),
                    max_output_tokens: 32,
                    ..Default::default()
                };
                assert_eq!(
                    send_fim_request(
                        settings::EditPredictionProvider::OpenAiCompatibleApi,
                        EditPredictionPromptFormat::DeepseekCoder,
                        &settings,
                        "def square(number):\n    return number ",
                        " number\n",
                        None,
                        &http_client
                    )
                    .await?,
                    ("*".into(), "native-fim".into()),
                    "{api_url}"
                );
            }
            Ok(())
        })
    }

    #[test]
    fn infer_prompt_format_matches_known_model_families() {
        assert_eq!(
            infer_prompt_format("qwen2.5-coder:3b"),
            Some(EditPredictionPromptFormat::Qwen)
        );
        assert_eq!(
            infer_prompt_format("codellama:7b"),
            Some(EditPredictionPromptFormat::CodeLlama)
        );
        assert_eq!(
            infer_prompt_format("deepseek-coder-v2:16b"),
            Some(EditPredictionPromptFormat::DeepseekCoder)
        );
        assert_eq!(
            infer_prompt_format("deepseek-flash"),
            Some(EditPredictionPromptFormat::DeepseekCoder)
        );
        assert_eq!(
            infer_prompt_format("starcoder2:3b"),
            Some(EditPredictionPromptFormat::StarCoder)
        );
        assert_eq!(
            infer_prompt_format("codestral:latest"),
            Some(EditPredictionPromptFormat::Codestral)
        );
        assert_eq!(
            infer_prompt_format("glm-4:9b"),
            Some(EditPredictionPromptFormat::Glm)
        );
    }

    #[test]
    fn infer_prompt_format_matches_zeta_and_sweep() {
        assert_eq!(
            infer_prompt_format("zeta2"),
            Some(EditPredictionPromptFormat::Zeta(ZetaVersion::Zeta2))
        );
        assert_eq!(
            infer_prompt_format("zeta2.1"),
            Some(EditPredictionPromptFormat::Zeta(ZetaVersion::Zeta2_1))
        );
        assert_eq!(
            infer_prompt_format("my-sweep-next-edit-v1"),
            Some(EditPredictionPromptFormat::Sweep)
        );
    }

    const FIM_PROMPT_FORMATS: [EditPredictionPromptFormat; 7] = [
        EditPredictionPromptFormat::CodeLlama,
        EditPredictionPromptFormat::StarCoder,
        EditPredictionPromptFormat::DeepseekCoder,
        EditPredictionPromptFormat::Qwen,
        EditPredictionPromptFormat::CodeGemma,
        EditPredictionPromptFormat::Codestral,
        EditPredictionPromptFormat::Glm,
    ];

    #[test]
    fn fim_stop_tokens_respect_openai_stop_limit() {
        for format in FIM_PROMPT_FORMATS {
            let stop_tokens = fim_stop_tokens(format);
            assert!(!stop_tokens.is_empty(), "{format:?} has no stop tokens");
            assert!(
                stop_tokens.len() <= 4,
                "{format:?} has {} stop tokens, but OpenAI allows at most 4",
                stop_tokens.len()
            );
        }
    }

    #[test]
    fn fim_stop_tokens_match_prompt_markers() {
        let end_of_text_tokens = ["<|endoftext|>", "<|file_separator|>"];
        for format in FIM_PROMPT_FORMATS {
            let prompt = format_fim_prompt(format, "a", "b");
            for token in fim_stop_tokens(format) {
                if end_of_text_tokens.contains(token) {
                    continue;
                }
                assert!(
                    prompt.contains(token),
                    "stop token {token:?} for {format:?} does not appear in prompt {prompt:?}"
                );
            }
        }
    }

    #[test]
    fn infer_prompt_format_returns_none_for_unsupported_models() {
        assert_eq!(infer_prompt_format("llama3:70b"), None);
        assert_eq!(infer_prompt_format("phi3:mini"), None);
        assert_eq!(infer_prompt_format("nomic-embed-text"), None);
    }
}
