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

    fn build_fim_prompt(&self, prefix: &str, suffix: &str) -> String {
        // Use model-specific FIM patterns
        match self.model.as_str() {
            m if m.contains("codellama") => {
                format!("<PRE> {prefix} <SUF>{suffix} <MID>")
            }
            m if m.contains("deepseek") => {
                format!("<｜fim▁begin｜>{prefix}<｜fim▁hole｜>{suffix}<｜fim▁end｜>")
            }
            m if m.contains("starcoder") => {
                format!("<fim_prefix>{prefix}<fim_suffix>{suffix}<fim_middle>")
            }
            _ => {
                // Generic FIM pattern - fallback for models without specific support
                format!("// Complete the following code:\n{prefix}\n// COMPLETION HERE\n{suffix}")
            }
        }
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
        let model = self.model.clone();

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

            let request = GenerateRequest {
                model: model.clone(),
                prompt,
                stream: false,
                options: Some(GenerateOptions {
                    num_predict: Some(150), // Reasonable completion length
                    temperature: Some(0.1), // Low temperature for more deterministic results
                    top_p: Some(0.95),
                    stop: Some(vec![
                        "\n\n".to_string(),
                        "```".to_string(),
                        "</PRE>".to_string(),
                        "<SUF>".to_string(),
                    ]),
                }),
                keep_alive: None,
                context: None,
            };

            let response = generate(http_client.as_ref(), &api_url, request)
                .await
                .context("Failed to get completion from Ollama")?;

            this.update(cx, |this, cx| {
                this.pending_refresh = None;
                if !response.response.trim().is_empty() {
                    this.current_completion = Some(response.response);
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

        // Clean up the completion text
        let completion_text = completion_text.trim_start().trim_end();

        Some(InlineCompletion {
            id: None,
            edits: vec![(position..position, completion_text.to_string())],
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
    async fn test_fim_prompt_patterns(_cx: &mut TestAppContext) {
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
}
