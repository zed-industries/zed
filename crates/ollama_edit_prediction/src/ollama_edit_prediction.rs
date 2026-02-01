use anyhow::Result;
use edit_prediction::cursor_excerpt;
use edit_prediction_types::{EditPrediction, EditPredictionDelegate};
use gpui::{App, Context, Entity, Task};
use http_client::HttpClient;
use language::{
    language_settings::all_language_settings, Anchor, Buffer, BufferSnapshot, EditPreview, ToPoint,
};
use ollama::{GenerateOptions, GenerateRequest, OLLAMA_API_URL};
use settings::KeepAlive;
use std::{
    ops::Range,
    sync::Arc,
    time::{Duration, Instant},
};
use text::{OffsetRangeExt as _, ToOffset};

pub const DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(150);
pub const DEFAULT_MODEL: &str = "qwen2.5-coder:7b";

#[derive(Clone)]
struct CurrentCompletion {
    snapshot: BufferSnapshot,
    edits: Arc<[(Range<Anchor>, Arc<str>)]>,
    edit_preview: EditPreview,
}

impl CurrentCompletion {
    fn interpolate(&self, new_snapshot: &BufferSnapshot) -> Option<Vec<(Range<Anchor>, Arc<str>)>> {
        edit_prediction_types::interpolate_edits(&self.snapshot, new_snapshot, &self.edits)
    }
}

pub struct OllamaEditPredictionDelegate {
    http_client: Arc<dyn HttpClient>,
    pending_request: Option<Task<Result<()>>>,
    current_completion: Option<CurrentCompletion>,
}

impl OllamaEditPredictionDelegate {
    pub fn new(http_client: Arc<dyn HttpClient>) -> Self {
        Self {
            http_client,
            pending_request: None,
            current_completion: None,
        }
    }

    async fn fetch_completion(
        http_client: Arc<dyn HttpClient>,
        api_url: &str,
        api_key: Option<&str>,
        prompt: String,
        suffix: String,
        model: String,
        max_tokens: Option<i32>,
    ) -> Result<String> {
        let start_time = Instant::now();

        log::info!(
            "Ollama: Requesting completion (model: {}, max_tokens: {:?})",
            model,
            max_tokens
        );

        let (fim_prompt, use_raw) = Self::format_fim_prompt(&model, &prompt, &suffix);

        let request = GenerateRequest {
            model,
            prompt: fim_prompt,
            suffix: None,
            stream: false,
            raw: if use_raw { Some(true) } else { None },
            keep_alive: KeepAlive::indefinite(),
            options: Some(GenerateOptions {
                num_predict: max_tokens.or(Some(150)),
                temperature: Some(0.2),
                top_p: Some(1.0),
                stop: None,
            }),
        };

        log::info!("Ollama: Sending FIM request to {}", api_url);

        let response = ollama::generate_completion(http_client.as_ref(), api_url, api_key, request)
            .await?;

        let elapsed = start_time.elapsed();

        log::info!(
            "Ollama: Completion received (eval_count: {:?}, {:.2}s)",
            response.eval_count,
            elapsed.as_secs_f64()
        );

        let completion = Self::clean_fim_response(&response.response);

        Ok(completion)
    }

    fn format_fim_prompt(model: &str, prefix: &str, suffix: &str) -> (String, bool) {
        let model_lower = model.to_lowercase();

        if model_lower.contains("qwen") {
            let prompt = format!(
                "<|fim_prefix|>{}<|fim_suffix|>{}<|fim_middle|>",
                prefix, suffix
            );
            (prompt, true)
        } else if model_lower.contains("codellama") || model_lower.contains("code-llama") {
            let prompt = format!("<PRE> {} <SUF>{} <MID>", prefix, suffix);
            (prompt, true)
        } else if model_lower.contains("starcoder") || model_lower.contains("star-coder") {
            let prompt = format!(
                "<fim_prefix>{}<fim_suffix>{}<fim_middle>",
                prefix, suffix
            );
            (prompt, true)
        } else if model_lower.contains("deepseek") {
            let prompt = format!(
                "<|fim_begin|>{}<|fim_hole|>{}<|fim_end|>",
                prefix, suffix
            );
            (prompt, true)
        } else if model_lower.contains("codestral") {
            let prompt = format!("[SUFFIX]{suffix}[PREFIX]{prefix}");
            (prompt, true)
        } else {
            log::warn!(
                "Ollama: Unknown model '{}' - using basic completion without FIM tokens",
                model
            );
            (prefix.to_string(), false)
        }
    }

    fn clean_fim_response(response: &str) -> String {
        let mut result = response.to_string();

        let stop_patterns = [
            "<|endoftext|>",
            "<|fim_pad|>",
            "<|fim_prefix|>",
            "<|fim_suffix|>",
            "<|fim_middle|>",
            "<|end|>",
            "<|im_end|>",
            "<|eot_id|>",
            "<EOT>",
        ];

        for pattern in &stop_patterns {
            if let Some(idx) = result.find(pattern) {
                result.truncate(idx);
            }
        }

        result
    }
}

impl EditPredictionDelegate for OllamaEditPredictionDelegate {
    fn name() -> &'static str {
        "ollama"
    }

    fn display_name() -> &'static str {
        "Ollama"
    }

    fn show_predictions_in_menu() -> bool {
        true
    }

    fn is_enabled(&self, _buffer: &Entity<Buffer>, _cursor_position: Anchor, _cx: &App) -> bool {
        true
    }

    fn is_refreshing(&self, _cx: &App) -> bool {
        self.pending_request.is_some()
    }

    fn refresh(
        &mut self,
        buffer: Entity<Buffer>,
        cursor_position: language::Anchor,
        debounce: bool,
        cx: &mut Context<Self>,
    ) {
        log::info!("Ollama: Refresh called (debounce: {})", debounce);

        let snapshot = buffer.read(cx).snapshot();

        if let Some(current_completion) = self.current_completion.as_ref() {
            if current_completion.interpolate(&snapshot).is_some() {
                return;
            }
        }

        if self.pending_request.is_some() {
            log::debug!("Ollama: Request already pending, not starting a new one");
            return;
        }

        let http_client = self.http_client.clone();

        let settings = all_language_settings(None, cx);
        let model = settings
            .edit_predictions
            .ollama
            .model
            .clone()
            .unwrap_or_else(|| DEFAULT_MODEL.to_string());
        let max_tokens = settings.edit_predictions.ollama.max_tokens;
        let api_url = settings
            .edit_predictions
            .ollama
            .api_url
            .clone()
            .unwrap_or_else(|| OLLAMA_API_URL.to_string());
        let api_key = settings.edit_predictions.ollama.api_key.clone();

        self.pending_request = Some(cx.spawn(async move |this, cx| {
            if debounce {
                log::debug!("Ollama: Debouncing for {:?}", DEBOUNCE_TIMEOUT);
                cx.background_executor().timer(DEBOUNCE_TIMEOUT).await;
            }

            let cursor_offset = cursor_position.to_offset(&snapshot);
            let cursor_point = cursor_offset.to_point(&snapshot);

            const MAX_CONTEXT_TOKENS: usize = 150;
            const MAX_REWRITE_TOKENS: usize = 350;

            let (_, context_range) =
                cursor_excerpt::editable_and_context_ranges_for_cursor_position(
                    cursor_point,
                    &snapshot,
                    MAX_REWRITE_TOKENS,
                    MAX_CONTEXT_TOKENS,
                );

            let context_range = context_range.to_offset(&snapshot);
            let excerpt_text = snapshot
                .text_for_range(context_range.clone())
                .collect::<String>();
            let cursor_within_excerpt = cursor_offset
                .saturating_sub(context_range.start)
                .min(excerpt_text.len());
            let prompt = excerpt_text[..cursor_within_excerpt].to_string();
            let suffix = excerpt_text[cursor_within_excerpt..].to_string();

            let completion_text = match Self::fetch_completion(
                http_client,
                &api_url,
                api_key.as_deref(),
                prompt,
                suffix,
                model,
                max_tokens,
            )
            .await
            {
                Ok(completion) => completion,
                Err(e) => {
                    log::error!("Ollama: Failed to fetch completion: {}", e);
                    this.update(cx, |this, cx| {
                        this.pending_request = None;
                        cx.notify();
                    })?;
                    return Err(e);
                }
            };

            if completion_text.trim().is_empty() {
                log::info!("Ollama: Completion was empty after trimming; ignoring");
                this.update(cx, |this, cx| {
                    this.pending_request = None;
                    cx.notify();
                })?;
                return Ok(());
            }

            let edits: Arc<[(Range<Anchor>, Arc<str>)]> =
                vec![(cursor_position..cursor_position, completion_text.into())].into();
            let edit_preview = buffer
                .read_with(cx, |buffer, cx| buffer.preview_edits(edits.clone(), cx))
                .await;

            this.update(cx, |this, cx| {
                log::info!("Ollama: Storing completion and notifying editor");
                this.current_completion = Some(CurrentCompletion {
                    snapshot,
                    edits,
                    edit_preview,
                });
                this.pending_request = None;
                cx.notify();
            })?;

            Ok(())
        }));
    }

    fn accept(&mut self, _cx: &mut Context<Self>) {
        log::debug!("Ollama: Completion accepted");
        self.pending_request = None;
        self.current_completion = None;
    }

    fn discard(&mut self, _cx: &mut Context<Self>) {
        log::debug!("Ollama: Completion discarded");
        self.pending_request = None;
        self.current_completion = None;
    }

    fn suggest(
        &mut self,
        buffer: &Entity<Buffer>,
        cursor_position: Anchor,
        cx: &mut Context<Self>,
    ) -> Option<EditPrediction> {
        let current_completion = match self.current_completion.as_ref() {
            Some(c) => c,
            None => {
                log::debug!("Ollama suggest: No current completion");
                return None;
            }
        };

        let buffer = buffer.read(cx);
        let new_snapshot = buffer.snapshot();

        if let Some(edits) = current_completion.interpolate(&new_snapshot) {
            if !edits.is_empty() {
                log::info!("Ollama suggest: Returning {} interpolated edit(s)", edits.len());
                return Some(EditPrediction::Local {
                    id: None,
                    edits,
                    cursor_position: None,
                    edit_preview: Some(current_completion.edit_preview.clone()),
                });
            }
        }

        let original_edits = &current_completion.edits;
        if original_edits.len() == 1 {
            let (original_range, completion_text) = &original_edits[0];
            let original_start = original_range.start.to_offset(&new_snapshot);
            let original_end = original_range.end.to_offset(&new_snapshot);
            let cursor_offset = cursor_position.to_offset(&new_snapshot);

            if original_start == original_end && cursor_offset >= original_start {
                let distance = cursor_offset - original_start;
                if distance <= 50 {
                    log::info!(
                        "Ollama suggest: Showing completion at cursor (moved {} chars from original)",
                        distance
                    );
                    let edits = vec![(cursor_position..cursor_position, completion_text.clone())];
                    return Some(EditPrediction::Local {
                        id: None,
                        edits,
                        cursor_position: None,
                        edit_preview: None,
                    });
                }
            }
        }

        log::info!("Ollama suggest: Completion no longer valid for current cursor position");
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_fim_prompt_qwen() {
        let (prompt, raw) =
            OllamaEditPredictionDelegate::format_fim_prompt("qwen2.5-coder:7b", "def hello(", ")");
        assert!(prompt.contains("<|fim_prefix|>"));
        assert!(prompt.contains("<|fim_suffix|>"));
        assert!(prompt.contains("<|fim_middle|>"));
        assert_eq!(
            prompt,
            "<|fim_prefix|>def hello(<|fim_suffix|>)<|fim_middle|>"
        );
        assert!(raw);
    }

    #[test]
    fn test_format_fim_prompt_codellama() {
        let (prompt, raw) =
            OllamaEditPredictionDelegate::format_fim_prompt("codellama:7b", "fn main() {", "}");
        assert!(prompt.contains("<PRE>"));
        assert!(prompt.contains("<SUF>"));
        assert!(prompt.contains("<MID>"));
        assert_eq!(prompt, "<PRE> fn main() { <SUF>} <MID>");
        assert!(raw);
    }

    #[test]
    fn test_format_fim_prompt_starcoder() {
        let (prompt, raw) =
            OllamaEditPredictionDelegate::format_fim_prompt("starcoder2:3b", "let x = ", ";");
        assert!(prompt.contains("<fim_prefix>"));
        assert!(prompt.contains("<fim_suffix>"));
        assert!(prompt.contains("<fim_middle>"));
        assert_eq!(prompt, "<fim_prefix>let x = <fim_suffix>;<fim_middle>");
        assert!(raw);
    }

    #[test]
    fn test_format_fim_prompt_deepseek() {
        let (prompt, raw) = OllamaEditPredictionDelegate::format_fim_prompt(
            "deepseek-coder:6.7b",
            "async fn fetch(",
            ") {}",
        );
        assert!(prompt.contains("<|fim_begin|>"));
        assert!(prompt.contains("<|fim_hole|>"));
        assert!(prompt.contains("<|fim_end|>"));
        assert_eq!(
            prompt,
            "<|fim_begin|>async fn fetch(<|fim_hole|>) {}<|fim_end|>"
        );
        assert!(raw);
    }

    #[test]
    fn test_format_fim_prompt_codestral() {
        let (prompt, raw) = OllamaEditPredictionDelegate::format_fim_prompt(
            "codestral:22b",
            "class User:",
            "\n    pass",
        );
        assert!(prompt.contains("[SUFFIX]"));
        assert!(prompt.contains("[PREFIX]"));
        assert_eq!(prompt, "[SUFFIX]\n    pass[PREFIX]class User:");
        assert!(raw);
    }

    #[test]
    fn test_format_fim_prompt_unknown_model() {
        let (prompt, raw) = OllamaEditPredictionDelegate::format_fim_prompt(
            "llama3:8b",
            "print('hello')",
            "",
        );
        assert_eq!(prompt, "print('hello')");
        assert!(!raw);
    }

    #[test]
    fn test_format_fim_prompt_case_insensitive() {
        let (prompt, raw) =
            OllamaEditPredictionDelegate::format_fim_prompt("QWEN2.5-CODER:7B", "x", "y");
        assert!(prompt.contains("<|fim_prefix|>"));
        assert!(raw);

        let (prompt, raw) =
            OllamaEditPredictionDelegate::format_fim_prompt("CodeLlama:13b", "x", "y");
        assert!(prompt.contains("<PRE>"));
        assert!(raw);
    }

    #[test]
    fn test_clean_fim_response_with_endoftext() {
        let result = OllamaEditPredictionDelegate::clean_fim_response(
            "hello world<|endoftext|>extra stuff",
        );
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_clean_fim_response_with_fim_pad() {
        let result =
            OllamaEditPredictionDelegate::clean_fim_response("completed code<|fim_pad|>garbage");
        assert_eq!(result, "completed code");
    }

    #[test]
    fn test_clean_fim_response_with_im_end() {
        let result =
            OllamaEditPredictionDelegate::clean_fim_response("return value<|im_end|>more");
        assert_eq!(result, "return value");
    }

    #[test]
    fn test_clean_fim_response_with_eot() {
        let result = OllamaEditPredictionDelegate::clean_fim_response("some code<EOT>trailing");
        assert_eq!(result, "some code");
    }

    #[test]
    fn test_clean_fim_response_no_stop_token() {
        let result = OllamaEditPredictionDelegate::clean_fim_response("clean completion text");
        assert_eq!(result, "clean completion text");
    }

    #[test]
    fn test_clean_fim_response_multiple_stop_tokens() {
        let result = OllamaEditPredictionDelegate::clean_fim_response(
            "first part<|endoftext|>second<|fim_pad|>third",
        );
        assert_eq!(result, "first part");
    }

    #[test]
    fn test_clean_fim_response_empty() {
        let result = OllamaEditPredictionDelegate::clean_fim_response("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_clean_fim_response_only_stop_token() {
        let result = OllamaEditPredictionDelegate::clean_fim_response("<|endoftext|>");
        assert_eq!(result, "");
    }
}
