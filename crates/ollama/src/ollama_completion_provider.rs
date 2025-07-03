use crate::{GenerateOptions, GenerateRequest, generate};
use anyhow::{Context as AnyhowContext, Result};
use gpui::{App, Context, Entity, EntityId, Task};
use http_client::HttpClient;
use inline_completion::{Direction, EditPredictionProvider, InlineCompletion};
use language::{Anchor, Buffer, ToOffset};
use project::Project;
use std::{path::Path, sync::Arc, time::Duration};

pub const OLLAMA_DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(75);

pub struct OllamaCompletionProvider {
    http_client: Arc<dyn HttpClient>,
    api_url: String,
    model: String,
    buffer_id: Option<EntityId>,
    file_extension: Option<String>,
    current_completion: Option<String>,
    pending_refresh: Option<Task<Result<()>>>,
}

impl OllamaCompletionProvider {
    pub fn new(http_client: Arc<dyn HttpClient>, api_url: String, model: String) -> Self {
        Self {
            http_client,
            api_url,
            model,
            buffer_id: None,
            file_extension: None,
            current_completion: None,
            pending_refresh: None,
        }
    }

    /// Updates the model used by this provider
    pub fn update_model(&mut self, new_model: String) {
        self.model = new_model;
    }

    fn build_fim_prompt(&self, prefix: &str, suffix: &str) -> String {
        // Use model-specific FIM patterns
        let model_lower = self.model.to_lowercase();

        if model_lower.contains("qwen") && model_lower.contains("coder") {
            // QwenCoder models use pipes
            format!("<|fim_prefix|>{prefix}<|fim_suffix|>{suffix}<|fim_middle|>")
        } else if model_lower.contains("codellama") {
            format!("<PRE> {prefix} <SUF>{suffix} <MID>")
        } else if model_lower.contains("deepseek") {
            format!("<｜fim▁begin｜>{prefix}<｜fim▁hole｜>{suffix}<｜fim▁end｜>")
        } else if model_lower.contains("codestral") {
            // Codestral uses suffix-first order
            format!("[SUFFIX]{suffix}[PREFIX]{prefix}")
        } else if model_lower.contains("codegemma") {
            format!("<|fim_prefix|>{prefix}<|fim_suffix|>{suffix}<|fim_middle|>")
        } else if model_lower.contains("wizardcoder") {
            // WizardCoder models inherit patterns from their base model
            if model_lower.contains("deepseek") {
                format!("<｜fim▁begin｜>{prefix}<｜fim▁hole｜>{suffix}<｜fim▁end｜>")
            } else {
                // Most WizardCoder models use stable code pattern
                format!("<fim_prefix>{prefix}<fim_suffix>{suffix}<fim_middle>")
            }
        } else if model_lower.contains("starcoder")
            || model_lower.contains("santacoder")
            || model_lower.contains("stable")
            || model_lower.contains("qwen")
            || model_lower.contains("replit")
        {
            // Stable code pattern (no pipes) - used by StarCoder, SantaCoder, StableCode,
            // non-coder Qwen models, and Replit models
            format!("<fim_prefix>{prefix}<fim_suffix>{suffix}<fim_middle>")
        } else {
            // Default to stable code pattern for unknown models
            format!("<fim_prefix>{prefix}<fim_suffix>{suffix}<fim_middle>")
        }
    }

    fn get_stop_tokens(&self) -> Vec<String> {
        let model_lower = self.model.to_lowercase();

        let mut stop_tokens = vec!["\n\n".to_string(), "```".to_string()];

        if model_lower.contains("qwen") && model_lower.contains("coder") {
            stop_tokens.extend(vec![
                "<|endoftext|>".to_string(),
                "<|fim_prefix|>".to_string(),
                "<|fim_middle|>".to_string(),
                "<|fim_suffix|>".to_string(),
                "<|fim_pad|>".to_string(),
                "<|repo_name|>".to_string(),
                "<|file_sep|>".to_string(),
                "<|im_start|>".to_string(),
                "<|im_end|>".to_string(),
            ]);
        } else if model_lower.contains("codellama") {
            stop_tokens.extend(vec![
                "<PRE>".to_string(),
                "<SUF>".to_string(),
                "<MID>".to_string(),
                "</PRE>".to_string(),
            ]);
        } else if model_lower.contains("deepseek") {
            stop_tokens.extend(vec![
                "<｜fim▁begin｜>".to_string(),
                "<｜fim▁hole｜>".to_string(),
                "<｜fim▁end｜>".to_string(),
                "//".to_string(),
                "<｜end▁of▁sentence｜>".to_string(),
            ]);
        } else if model_lower.contains("codestral") {
            stop_tokens.extend(vec!["[PREFIX]".to_string(), "[SUFFIX]".to_string()]);
        } else if model_lower.contains("codegemma") {
            stop_tokens.extend(vec![
                "<|fim_prefix|>".to_string(),
                "<|fim_suffix|>".to_string(),
                "<|fim_middle|>".to_string(),
                "<|file_separator|>".to_string(),
                "<|endoftext|>".to_string(),
            ]);
        } else if model_lower.contains("wizardcoder") {
            // WizardCoder models inherit patterns from their base model
            if model_lower.contains("deepseek") {
                stop_tokens.extend(vec![
                    "<｜fim▁begin｜>".to_string(),
                    "<｜fim▁hole｜>".to_string(),
                    "<｜fim▁end｜>".to_string(),
                ]);
            } else {
                stop_tokens.extend(vec![
                    "<fim_prefix>".to_string(),
                    "<fim_suffix>".to_string(),
                    "<fim_middle>".to_string(),
                    "<|endoftext|>".to_string(),
                ]);
            }
        } else if model_lower.contains("starcoder")
            || model_lower.contains("santacoder")
            || model_lower.contains("stable")
            || model_lower.contains("qwen")
            || model_lower.contains("replit")
        {
            // Stable code pattern stop tokens
            stop_tokens.extend(vec![
                "<fim_prefix>".to_string(),
                "<fim_suffix>".to_string(),
                "<fim_middle>".to_string(),
                "<|endoftext|>".to_string(),
            ]);
        } else {
            // Generic stop tokens for unknown models - cover both patterns
            stop_tokens.extend(vec![
                "<|fim_prefix|>".to_string(),
                "<|fim_suffix|>".to_string(),
                "<|fim_middle|>".to_string(),
                "<fim_prefix>".to_string(),
                "<fim_suffix>".to_string(),
                "<fim_middle>".to_string(),
                "<|endoftext|>".to_string(),
            ]);
        }

        stop_tokens
    }

    fn clean_completion(&self, completion: &str) -> String {
        let mut cleaned = completion.to_string();

        // Remove common FIM tokens that might appear in responses
        let fim_tokens = [
            "<|fim_prefix|>",
            "<|fim_suffix|>",
            "<|fim_middle|>",
            "<|fim_pad|>",
            "<|repo_name|>",
            "<|file_sep|>",
            "<|im_start|>",
            "<|im_end|>",
            "<fim_prefix>",
            "<fim_suffix>",
            "<fim_middle>",
            "<PRE>",
            "<SUF>",
            "<MID>",
            "</PRE>",
            "<｜fim▁begin｜>",
            "<｜fim▁hole｜>",
            "<｜fim▁end｜>",
            "<｜end▁of▁sentence｜>",
            "[PREFIX]",
            "[SUFFIX]",
            "<|file_separator|>",
            "<|endoftext|>",
        ];

        for token in &fim_tokens {
            cleaned = cleaned.replace(token, "");
        }

        // Remove leading/trailing whitespace and common prefixes
        cleaned = cleaned.trim().to_string();

        // Remove common unwanted prefixes that models sometimes generate
        let unwanted_prefixes = [
            "// COMPLETION HERE",
            "// Complete the following code:",
            "// completion:",
            "// TODO:",
        ];

        for prefix in &unwanted_prefixes {
            if cleaned.starts_with(prefix) {
                cleaned = cleaned[prefix.len()..].trim_start().to_string();
            }
        }

        cleaned
    }

    fn extract_context(&self, buffer: &Buffer, cursor_position: Anchor) -> (String, String) {
        let cursor_offset = cursor_position.to_offset(buffer);
        let text = buffer.text();

        // Get reasonable context around cursor
        let context_size = 2000; // 2KB before and after cursor

        let start = cursor_offset.saturating_sub(context_size);
        let end = (cursor_offset + context_size).min(text.len());

        let prefix = text[start..cursor_offset].to_string();
        let suffix = text[cursor_offset..end].to_string();

        (prefix, suffix)
    }
}

impl EditPredictionProvider for OllamaCompletionProvider {
    fn name() -> &'static str {
        "ollama"
    }

    fn display_name() -> &'static str {
        "Ollama"
    }

    fn show_completions_in_menu() -> bool {
        false
    }

    fn is_enabled(&self, _buffer: &Entity<Buffer>, _cursor_position: Anchor, _cx: &App) -> bool {
        // TODO: Could ping Ollama API to check if it's running
        true
    }

    fn is_refreshing(&self) -> bool {
        self.pending_refresh.is_some()
    }

    fn refresh(
        &mut self,
        _project: Option<Entity<Project>>,
        buffer: Entity<Buffer>,
        cursor_position: Anchor,
        debounce: bool,
        cx: &mut Context<Self>,
    ) {
        let http_client = self.http_client.clone();
        let api_url = self.api_url.clone();

        self.pending_refresh = Some(cx.spawn(async move |this, cx| {
            if debounce {
                cx.background_executor()
                    .timer(OLLAMA_DEBOUNCE_TIMEOUT)
                    .await;
            }

            let (prefix, suffix) = this.update(cx, |this, cx| {
                let buffer_snapshot = buffer.read(cx);
                this.buffer_id = Some(buffer.entity_id());
                this.file_extension = buffer_snapshot.file().and_then(|file| {
                    Some(
                        Path::new(file.file_name(cx))
                            .extension()?
                            .to_str()?
                            .to_string(),
                    )
                });
                this.extract_context(buffer_snapshot, cursor_position)
            })?;

            let prompt = this.update(cx, |this, _| this.build_fim_prompt(&prefix, &suffix))?;

            let (model, stop_tokens) =
                this.update(cx, |this, _| (this.model.clone(), this.get_stop_tokens()))?;

            let request = GenerateRequest {
                model,
                prompt,
                stream: false,
                options: Some(GenerateOptions {
                    num_predict: Some(150), // Reasonable completion length
                    temperature: Some(0.1), // Low temperature for more deterministic results
                    top_p: Some(0.95),
                    stop: Some(stop_tokens),
                }),
                keep_alive: None,
                context: None,
            };

            let response = generate(http_client.as_ref(), &api_url, request)
                .await
                .context("Failed to get completion from Ollama")?;

            this.update(cx, |this, cx| {
                this.pending_refresh = None;
                let cleaned_completion = this.clean_completion(&response.response);
                if !cleaned_completion.is_empty() {
                    this.current_completion = Some(cleaned_completion);
                } else {
                    this.current_completion = None;
                }
                cx.notify();
            })?;

            Ok(())
        }));
    }

    fn cycle(
        &mut self,
        _buffer: Entity<Buffer>,
        _cursor_position: Anchor,
        _direction: Direction,
        _cx: &mut Context<Self>,
    ) {
        // Ollama doesn't provide multiple completions in a single request
        // Could be implemented by making multiple requests with different temperatures
        // or by using different models
    }

    fn accept(&mut self, _cx: &mut Context<Self>) {
        self.current_completion = None;
        // TODO: Could send accept telemetry to Ollama if supported
    }

    fn discard(&mut self, _cx: &mut Context<Self>) {
        self.current_completion = None;
        // TODO: Could send discard telemetry to Ollama if supported
    }

    fn suggest(
        &mut self,
        buffer: &Entity<Buffer>,
        cursor_position: Anchor,
        cx: &mut Context<Self>,
    ) -> Option<InlineCompletion> {
        let buffer_id = buffer.entity_id();
        if Some(buffer_id) != self.buffer_id {
            return None;
        }

        let completion_text = self.current_completion.as_ref()?.clone();

        if completion_text.trim().is_empty() {
            return None;
        }

        let buffer_snapshot = buffer.read(cx);
        let position = cursor_position.bias_right(buffer_snapshot);

        Some(InlineCompletion {
            id: None,
            edits: vec![(position..position, completion_text)],
            edit_preview: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{AppContext, TestAppContext};
    use http_client::FakeHttpClient;
    use std::sync::Arc;

    #[gpui::test]
    async fn test_fim_prompt_qwen_coder_pattern(_cx: &mut TestAppContext) {
        let provider = OllamaCompletionProvider::new(
            Arc::new(FakeHttpClient::with_404_response()),
            "http://localhost:11434".to_string(),
            "qwen2.5-coder:32b".to_string(),
        );

        let prefix = "def hello():";
        let suffix = "    pass";
        let prompt = provider.build_fim_prompt(prefix, suffix);

        assert!(prompt.contains("<|fim_prefix|>"));
        assert!(prompt.contains("<|fim_suffix|>"));
        assert!(prompt.contains("<|fim_middle|>"));
        assert!(prompt.contains(prefix));
        assert!(prompt.contains(suffix));
    }

    #[gpui::test]
    async fn test_fim_prompt_qwen_non_coder_pattern(_cx: &mut TestAppContext) {
        let provider = OllamaCompletionProvider::new(
            Arc::new(FakeHttpClient::with_404_response()),
            "http://localhost:11434".to_string(),
            "qwen2.5:32b".to_string(),
        );

        let prefix = "def hello():";
        let suffix = "    pass";
        let prompt = provider.build_fim_prompt(prefix, suffix);

        assert!(prompt.contains("<fim_prefix>"));
        assert!(prompt.contains("<fim_suffix>"));
        assert!(prompt.contains("<fim_middle>"));
        assert!(!prompt.contains("<|fim_prefix|>")); // Should NOT contain pipes
        assert!(prompt.contains(prefix));
        assert!(prompt.contains(suffix));
    }

    #[gpui::test]
    async fn test_fim_prompt_codellama_pattern(_cx: &mut TestAppContext) {
        let provider = OllamaCompletionProvider::new(
            Arc::new(FakeHttpClient::with_404_response()),
            "http://localhost:11434".to_string(),
            "codellama:7b".to_string(),
        );

        let prefix = "function hello() {";
        let suffix = "}";
        let prompt = provider.build_fim_prompt(prefix, suffix);

        assert!(prompt.contains("<PRE>"));
        assert!(prompt.contains("<SUF>"));
        assert!(prompt.contains("<MID>"));
        assert!(prompt.contains(prefix));
        assert!(prompt.contains(suffix));
    }

    #[gpui::test]
    async fn test_fim_prompt_deepseek_pattern(_cx: &mut TestAppContext) {
        let provider = OllamaCompletionProvider::new(
            Arc::new(FakeHttpClient::with_404_response()),
            "http://localhost:11434".to_string(),
            "deepseek-coder:6.7b".to_string(),
        );

        let prefix = "def hello():";
        let suffix = "    pass";
        let prompt = provider.build_fim_prompt(prefix, suffix);

        assert!(prompt.contains("<｜fim▁begin｜>"));
        assert!(prompt.contains("<｜fim▁hole｜>"));
        assert!(prompt.contains("<｜fim▁end｜>"));
    }

    #[gpui::test]
    async fn test_fim_prompt_starcoder_pattern(_cx: &mut TestAppContext) {
        let provider = OllamaCompletionProvider::new(
            Arc::new(FakeHttpClient::with_404_response()),
            "http://localhost:11434".to_string(),
            "starcoder:7b".to_string(),
        );

        let prefix = "def hello():";
        let suffix = "    pass";
        let prompt = provider.build_fim_prompt(prefix, suffix);

        assert!(prompt.contains("<fim_prefix>"));
        assert!(prompt.contains("<fim_suffix>"));
        assert!(prompt.contains("<fim_middle>"));
    }

    #[gpui::test]
    async fn test_fim_prompt_replit_pattern(_cx: &mut TestAppContext) {
        let provider = OllamaCompletionProvider::new(
            Arc::new(FakeHttpClient::with_404_response()),
            "http://localhost:11434".to_string(),
            "replit-code:3b".to_string(),
        );

        let prefix = "def hello():";
        let suffix = "    pass";
        let prompt = provider.build_fim_prompt(prefix, suffix);

        // Replit should use stable code pattern (no pipes)
        assert!(prompt.contains("<fim_prefix>"));
        assert!(prompt.contains("<fim_suffix>"));
        assert!(prompt.contains("<fim_middle>"));
        assert!(!prompt.contains("<|fim_prefix|>")); // Should NOT contain pipes
    }

    #[gpui::test]
    async fn test_fim_prompt_codestral_pattern(_cx: &mut TestAppContext) {
        let provider = OllamaCompletionProvider::new(
            Arc::new(FakeHttpClient::with_404_response()),
            "http://localhost:11434".to_string(),
            "codestral:22b".to_string(),
        );

        let prefix = "def hello():";
        let suffix = "    pass";
        let prompt = provider.build_fim_prompt(prefix, suffix);

        // Codestral uses suffix-first order
        assert!(prompt.contains("[SUFFIX]"));
        assert!(prompt.contains("[PREFIX]"));
        assert!(prompt.starts_with("[SUFFIX]"));
        assert!(prompt.contains(prefix));
        assert!(prompt.contains(suffix));
    }

    #[gpui::test]
    async fn test_fim_prompt_codegemma_pattern(_cx: &mut TestAppContext) {
        let provider = OllamaCompletionProvider::new(
            Arc::new(FakeHttpClient::with_404_response()),
            "http://localhost:11434".to_string(),
            "codegemma:7b".to_string(),
        );

        let prefix = "def hello():";
        let suffix = "    pass";
        let prompt = provider.build_fim_prompt(prefix, suffix);

        assert!(prompt.contains("<|fim_prefix|>"));
        assert!(prompt.contains("<|fim_suffix|>"));
        assert!(prompt.contains("<|fim_middle|>"));
    }

    #[gpui::test]
    async fn test_fim_prompt_wizardcoder_pattern(_cx: &mut TestAppContext) {
        let provider = OllamaCompletionProvider::new(
            Arc::new(FakeHttpClient::with_404_response()),
            "http://localhost:11434".to_string(),
            "wizardcoder:13b".to_string(),
        );

        let prefix = "def hello():";
        let suffix = "    pass";
        let prompt = provider.build_fim_prompt(prefix, suffix);

        // WizardCoder should use stable code pattern (no pipes) unless it's deepseek-based
        assert!(prompt.contains("<fim_prefix>"));
        assert!(prompt.contains("<fim_suffix>"));
        assert!(prompt.contains("<fim_middle>"));
        assert!(!prompt.contains("<|fim_prefix|>")); // Should NOT contain pipes
    }

    #[gpui::test]
    async fn test_fim_prompt_santacoder_pattern(_cx: &mut TestAppContext) {
        let provider = OllamaCompletionProvider::new(
            Arc::new(FakeHttpClient::with_404_response()),
            "http://localhost:11434".to_string(),
            "santacoder:1b".to_string(),
        );

        let prefix = "def hello():";
        let suffix = "    pass";
        let prompt = provider.build_fim_prompt(prefix, suffix);

        assert!(prompt.contains("<fim_prefix>"));
        assert!(prompt.contains("<fim_suffix>"));
        assert!(prompt.contains("<fim_middle>"));
    }

    #[gpui::test]
    async fn test_clean_completion_removes_fim_tokens(_cx: &mut TestAppContext) {
        let provider = OllamaCompletionProvider::new(
            Arc::new(FakeHttpClient::with_404_response()),
            "http://localhost:11434".to_string(),
            "qwen2.5-coder:32b".to_string(),
        );

        let completion_with_tokens = "console.log('hello');<|fim_middle|>";
        let cleaned = provider.clean_completion(completion_with_tokens);
        assert_eq!(cleaned, "console.log('hello');");

        let completion_with_multiple_tokens = "<|fim_prefix|>console.log('hello');<|fim_suffix|>";
        let cleaned = provider.clean_completion(completion_with_multiple_tokens);
        assert_eq!(cleaned, "console.log('hello');");

        let completion_with_starcoder_tokens = "console.log('hello');<fim_middle>";
        let cleaned = provider.clean_completion(completion_with_starcoder_tokens);
        assert_eq!(cleaned, "console.log('hello');");

        let completion_with_codestral_tokens = "console.log('hello');[SUFFIX]";
        let cleaned = provider.clean_completion(completion_with_codestral_tokens);
        assert_eq!(cleaned, "console.log('hello');");
    }

    #[gpui::test]
    async fn test_get_stop_tokens_qwen_coder(_cx: &mut TestAppContext) {
        let provider = OllamaCompletionProvider::new(
            Arc::new(FakeHttpClient::with_404_response()),
            "http://localhost:11434".to_string(),
            "qwen2.5-coder:32b".to_string(),
        );

        let stop_tokens = provider.get_stop_tokens();
        assert!(stop_tokens.contains(&"<|fim_prefix|>".to_string()));
        assert!(stop_tokens.contains(&"<|fim_suffix|>".to_string()));
        assert!(stop_tokens.contains(&"<|fim_middle|>".to_string()));
        assert!(stop_tokens.contains(&"<|endoftext|>".to_string()));
        assert!(stop_tokens.contains(&"<|fim_pad|>".to_string()));
        assert!(stop_tokens.contains(&"<|repo_name|>".to_string()));
    }

    #[gpui::test]
    async fn test_get_stop_tokens_codellama(_cx: &mut TestAppContext) {
        let provider = OllamaCompletionProvider::new(
            Arc::new(FakeHttpClient::with_404_response()),
            "http://localhost:11434".to_string(),
            "codellama:7b".to_string(),
        );

        let stop_tokens = provider.get_stop_tokens();
        assert!(stop_tokens.contains(&"<PRE>".to_string()));
        assert!(stop_tokens.contains(&"<SUF>".to_string()));
        assert!(stop_tokens.contains(&"<MID>".to_string()));
        assert!(stop_tokens.contains(&"</PRE>".to_string()));
    }

    #[gpui::test]
    async fn test_get_stop_tokens_codestral(_cx: &mut TestAppContext) {
        let provider = OllamaCompletionProvider::new(
            Arc::new(FakeHttpClient::with_404_response()),
            "http://localhost:11434".to_string(),
            "codestral:7b".to_string(),
        );

        let stop_tokens = provider.get_stop_tokens();
        assert!(stop_tokens.contains(&"[PREFIX]".to_string()));
        assert!(stop_tokens.contains(&"[SUFFIX]".to_string()));
    }

    #[gpui::test]
    async fn test_extract_context(cx: &mut TestAppContext) {
        let provider = OllamaCompletionProvider::new(
            Arc::new(FakeHttpClient::with_404_response()),
            "http://localhost:11434".to_string(),
            "codellama:7b".to_string(),
        );

        // Create a simple buffer using test context
        let buffer_text = "function example() {\n    let x = 1;\n    let y = 2;\n    // cursor here\n    return x + y;\n}";
        let buffer = cx.new(|cx| language::Buffer::local(buffer_text, cx));

        // Position cursor at the end of the "// cursor here" line
        let (prefix, suffix, _cursor_position) = cx.read(|cx| {
            let buffer_snapshot = buffer.read(cx);
            let cursor_position = buffer_snapshot.anchor_after(text::Point::new(3, 15)); // End of "// cursor here"
            let (prefix, suffix) = provider.extract_context(&buffer_snapshot, cursor_position);
            (prefix, suffix, cursor_position)
        });

        assert!(prefix.contains("function example()"));
        assert!(prefix.contains("// cursor h"));
        assert!(suffix.contains("ere"));
        assert!(suffix.contains("return x + y"));
        assert!(suffix.contains("}"));
    }

    #[gpui::test]
    async fn test_suggest_with_completion(cx: &mut TestAppContext) {
        let provider = cx.new(|_| {
            OllamaCompletionProvider::new(
                Arc::new(FakeHttpClient::with_404_response()),
                "http://localhost:11434".to_string(),
                "codellama:7b".to_string(),
            )
        });

        let buffer_text = "// test";
        let buffer = cx.new(|cx| language::Buffer::local(buffer_text, cx));

        // Set up a mock completion
        provider.update(cx, |provider, _| {
            provider.current_completion = Some("console.log('hello');".to_string());
            provider.buffer_id = Some(buffer.entity_id());
        });

        let cursor_position = cx.read(|cx| buffer.read(cx).anchor_after(text::Point::new(0, 7)));

        let completion = provider.update(cx, |provider, cx| {
            provider.suggest(&buffer, cursor_position, cx)
        });

        assert!(completion.is_some());
        let completion = completion.unwrap();
        assert_eq!(completion.edits.len(), 1);
        assert_eq!(completion.edits[0].1, "console.log('hello');");
    }

    #[gpui::test]
    async fn test_suggest_empty_completion(cx: &mut TestAppContext) {
        let provider = cx.new(|_| {
            OllamaCompletionProvider::new(
                Arc::new(FakeHttpClient::with_404_response()),
                "http://localhost:11434".to_string(),
                "codellama:7b".to_string(),
            )
        });

        let buffer_text = "// test";
        let buffer = cx.new(|cx| language::Buffer::local(buffer_text, cx));

        // Set up an empty completion
        provider.update(cx, |provider, _| {
            provider.current_completion = Some("   ".to_string()); // Only whitespace
            provider.buffer_id = Some(buffer.entity_id());
        });

        let cursor_position = cx.read(|cx| buffer.read(cx).anchor_after(text::Point::new(0, 7)));

        let completion = provider.update(cx, |provider, cx| {
            provider.suggest(&buffer, cursor_position, cx)
        });

        assert!(completion.is_none());
    }

    #[gpui::test]
    async fn test_update_model(_cx: &mut TestAppContext) {
        let mut provider = OllamaCompletionProvider::new(
            Arc::new(FakeHttpClient::with_404_response()),
            "http://localhost:11434".to_string(),
            "codellama:7b".to_string(),
        );

        // Verify initial model
        assert_eq!(provider.model, "codellama:7b");

        // Test updating model to Qwen Coder
        provider.update_model("qwen2.5-coder:32b".to_string());
        assert_eq!(provider.model, "qwen2.5-coder:32b");

        // Test FIM prompt changes with different model
        let prefix = "def hello():";
        let suffix = "    pass";
        let prompt = provider.build_fim_prompt(prefix, suffix);

        // Should now use qwen coder pattern (with pipes)
        assert!(prompt.contains("<|fim_prefix|>"));
        assert!(prompt.contains("<|fim_suffix|>"));
        assert!(prompt.contains("<|fim_middle|>"));

        // Update to regular Qwen model (non-coder)
        provider.update_model("qwen2.5:32b".to_string());
        assert_eq!(provider.model, "qwen2.5:32b");

        let prompt = provider.build_fim_prompt(prefix, suffix);

        // Should now use stable code pattern (no pipes)
        assert!(prompt.contains("<fim_prefix>"));
        assert!(prompt.contains("<fim_suffix>"));
        assert!(prompt.contains("<fim_middle>"));
        assert!(!prompt.contains("<|fim_prefix|>")); // Should NOT contain pipes

        // Update to starcoder model
        provider.update_model("starcoder:7b".to_string());
        assert_eq!(provider.model, "starcoder:7b");

        let prompt = provider.build_fim_prompt(prefix, suffix);

        // Should also use stable code pattern (no pipes)
        assert!(prompt.contains("<fim_prefix>"));
        assert!(prompt.contains("<fim_suffix>"));
        assert!(prompt.contains("<fim_middle>"));
        assert!(!prompt.contains("<|fim_prefix|>")); // Should NOT contain pipes

        // Update to codestral model
        provider.update_model("codestral:22b".to_string());
        assert_eq!(provider.model, "codestral:22b");

        let prompt = provider.build_fim_prompt(prefix, suffix);

        // Should use codestral pattern (suffix-first)
        assert!(prompt.contains("[SUFFIX]"));
        assert!(prompt.contains("[PREFIX]"));
        assert!(prompt.starts_with("[SUFFIX]"));
    }
}
