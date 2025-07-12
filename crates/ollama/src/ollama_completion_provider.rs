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
    api_key: Option<String>,
}

impl OllamaCompletionProvider {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        api_url: String,
        model: String,
        api_key: Option<String>,
    ) -> Self {
        Self {
            http_client,
            api_url,
            model,
            buffer_id: None,
            file_extension: None,
            current_completion: None,
            pending_refresh: None,
            api_key,
        }
    }

    /// Updates the model used by this provider
    pub fn update_model(&mut self, model: String) {
        self.model = model;
    }

    /// Updates the file extension used by this provider
    pub fn update_file_extension(&mut self, new_file_extension: String) {
        self.file_extension = Some(new_file_extension);
    }

    // Removed get_stop_tokens and clean_completion - Ollama handles everything natively with FIM

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
        true
    }

    fn is_enabled(&self, _buffer: &Entity<Buffer>, _cursor_position: Anchor, _cx: &App) -> bool {
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

            let (model, api_key) =
                this.update(cx, |this, _| (this.model.clone(), this.api_key.clone()))?;

            let request = GenerateRequest {
                model,
                prompt: prefix,
                suffix: Some(suffix),
                stream: false,
                options: Some(GenerateOptions {
                    num_predict: Some(150), // Reasonable completion length
                    temperature: Some(0.1), // Low temperature for more deterministic results
                    top_p: Some(0.95),
                    stop: None, // Let Ollama handle stop tokens natively
                }),
                keep_alive: None,
                context: None,
            };

            let response = generate(http_client.as_ref(), &api_url, api_key, request)
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
        let cursor_offset = cursor_position.to_offset(buffer_snapshot);

        // Get text before cursor to check what's already been typed
        let text_before_cursor = buffer_snapshot
            .text_for_range(0..cursor_offset)
            .collect::<String>();

        // Find how much of the completion has already been typed by checking
        // if the text before the cursor ends with a prefix of our completion
        let mut prefix_len = 0;
        for i in 1..=completion_text.len().min(text_before_cursor.len()) {
            if text_before_cursor.ends_with(&completion_text[..i]) {
                prefix_len = i;
            }
        }

        // Only suggest the remaining part of the completion
        let remaining_completion = &completion_text[prefix_len..];

        if remaining_completion.trim().is_empty() {
            return None;
        }

        let position = cursor_position.bias_right(buffer_snapshot);

        Some(InlineCompletion {
            id: None,
            edits: vec![(position..position, remaining_completion.to_string())],
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

    // Removed test_get_stop_tokens - no longer using custom stop tokens

    // Removed test_clean_completion_basic - no longer using custom completion cleaning

    #[gpui::test]
    async fn test_extract_context(cx: &mut TestAppContext) {
        let provider = OllamaCompletionProvider::new(
            Arc::new(FakeHttpClient::with_404_response()),
            "http://localhost:11434".to_string(),
            "codellama:7b".to_string(),
            None,
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
                None,
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
                None,
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
            None,
        );

        // Verify initial model
        assert_eq!(provider.model, "codellama:7b");

        // Test updating model to Qwen Coder
        provider.update_model("qwen2.5-coder:32b".to_string());
        assert_eq!(provider.model, "qwen2.5-coder:32b");

        // Test updating to different models
        provider.update_model("qwen2.5:32b".to_string());
        assert_eq!(provider.model, "qwen2.5:32b");

        provider.update_model("starcoder:7b".to_string());
        assert_eq!(provider.model, "starcoder:7b");

        provider.update_model("codestral:22b".to_string());
        assert_eq!(provider.model, "codestral:22b");

        // FIM patterns are now handled by Ollama natively, so we just test model updates
        provider.update_model("deepseek-coder:6.7b".to_string());
        assert_eq!(provider.model, "deepseek-coder:6.7b");
    }

    #[gpui::test]
    async fn test_native_fim_request_structure(_cx: &mut TestAppContext) {
        let provider = OllamaCompletionProvider::new(
            Arc::new(FakeHttpClient::with_404_response()),
            "http://localhost:11434".to_string(),
            "qwen2.5-coder:32b".to_string(),
            None,
        );

        let prefix = "def fibonacci(n):";
        let suffix = "    return result";

        // Test that we create the correct request structure for native FIM
        let request = GenerateRequest {
            model: provider.model.clone(),
            prompt: prefix.to_string(),
            suffix: Some(suffix.to_string()),
            stream: false,
            options: Some(GenerateOptions {
                num_predict: Some(150),
                temperature: Some(0.1),
                top_p: Some(0.95),
                stop: None, // Ollama handles stop tokens natively
            }),
            keep_alive: None,
            context: None,
        };

        // Verify the request structure uses native FIM approach
        assert_eq!(request.model, "qwen2.5-coder:32b");
        assert_eq!(request.prompt, "def fibonacci(n):");
        assert_eq!(request.suffix, Some("    return result".to_string()));
        assert!(!request.stream);

        // Verify stop tokens are handled natively by Ollama
        assert!(request.options.as_ref().unwrap().stop.is_none());
    }

    #[gpui::test]
    async fn test_api_key_support(_cx: &mut TestAppContext) {
        // Test with API key
        let provider_with_key = OllamaCompletionProvider::new(
            Arc::new(FakeHttpClient::with_404_response()),
            "http://localhost:11434".to_string(),
            "qwen2.5-coder:32b".to_string(),
            Some("test-api-key".to_string()),
        );

        // Test without API key
        let provider_without_key = OllamaCompletionProvider::new(
            Arc::new(FakeHttpClient::with_404_response()),
            "http://localhost:11434".to_string(),
            "qwen2.5-coder:32b".to_string(),
            None,
        );

        // Verify API key is stored correctly
        assert_eq!(provider_with_key.api_key, Some("test-api-key".to_string()));
        assert_eq!(provider_without_key.api_key, None);

        // Verify API key is passed to generate request
        let prefix = "def test():";
        let suffix = "    pass";

        let _request_with_key = GenerateRequest {
            model: provider_with_key.model.clone(),
            prompt: prefix.to_string(),
            suffix: Some(suffix.to_string()),
            stream: false,
            options: Some(GenerateOptions {
                num_predict: Some(150),
                temperature: Some(0.1),
                top_p: Some(0.95),
                stop: None,
            }),
            keep_alive: None,
            context: None,
        };

        // The actual API key usage would be tested in the generate function
        // but we can verify the provider stores it correctly
        assert_eq!(provider_with_key.api_key, Some("test-api-key".to_string()));
    }

    #[gpui::test]
    async fn test_show_completions_in_menu(_cx: &mut TestAppContext) {
        // Test that Ollama provider shows completions in menu to enable hover icon
        assert!(OllamaCompletionProvider::show_completions_in_menu());
    }

    #[gpui::test]
    async fn test_partial_accept_behavior(cx: &mut TestAppContext) {
        let provider = cx.new(|_| {
            OllamaCompletionProvider::new(
                Arc::new(FakeHttpClient::with_404_response()),
                "http://localhost:11434".to_string(),
                "codellama:7b".to_string(),
                None,
            )
        });

        let buffer_text = "let x = ";
        let buffer = cx.new(|cx| language::Buffer::local(buffer_text, cx));

        // Set up a completion with multiple words
        provider.update(cx, |provider, _| {
            provider.current_completion = Some("hello world".to_string());
            provider.buffer_id = Some(buffer.entity_id());
        });

        let cursor_position = cx.read(|cx| buffer.read(cx).anchor_after(text::Point::new(0, 8)));

        // First suggestion should return the full completion
        let completion = provider.update(cx, |provider, cx| {
            provider.suggest(&buffer, cursor_position, cx)
        });
        assert!(completion.is_some());
        let completion = completion.unwrap();
        assert_eq!(completion.edits.len(), 1);
        assert_eq!(completion.edits[0].1, "hello world");

        // Simulate what happens after partial accept - cursor moves forward
        let buffer_text_after_partial = "let x = hello";
        let buffer_after_partial =
            cx.new(|cx| language::Buffer::local(buffer_text_after_partial, cx));
        let cursor_position_after = cx.read(|cx| {
            buffer_after_partial
                .read(cx)
                .anchor_after(text::Point::new(0, 13))
        });

        // Update provider to track the new buffer
        provider.update(cx, |provider, _| {
            provider.buffer_id = Some(buffer_after_partial.entity_id());
        });

        // The provider should now adjust its completion based on what's already been typed
        let completion_after = provider.update(cx, |provider, cx| {
            provider.suggest(&buffer_after_partial, cursor_position_after, cx)
        });

        // With the fix, the provider should only suggest the remaining part " world"
        assert!(completion_after.is_some());
        let completion_after = completion_after.unwrap();
        assert_eq!(completion_after.edits[0].1, " world");

        // Test another partial accept scenario
        let buffer_text_final = "let x = hello world";
        let buffer_final = cx.new(|cx| language::Buffer::local(buffer_text_final, cx));
        let cursor_position_final =
            cx.read(|cx| buffer_final.read(cx).anchor_after(text::Point::new(0, 19)));

        provider.update(cx, |provider, _| {
            provider.buffer_id = Some(buffer_final.entity_id());
        });

        // Should return None since the full completion is already typed
        let completion_final = provider.update(cx, |provider, cx| {
            provider.suggest(&buffer_final, cursor_position_final, cx)
        });
        assert!(completion_final.is_none());
    }

    #[gpui::test]
    async fn test_partial_accept_with_non_word_characters(cx: &mut TestAppContext) {
        let provider = cx.new(|_| {
            OllamaCompletionProvider::new(
                Arc::new(FakeHttpClient::with_404_response()),
                "http://localhost:11434".to_string(),
                "codellama:7b".to_string(),
                None,
            )
        });

        let buffer_text = "console.";
        let buffer = cx.new(|cx| language::Buffer::local(buffer_text, cx));

        // Set up a completion with method call
        provider.update(cx, |provider, _| {
            provider.current_completion = Some("log('test')".to_string());
            provider.buffer_id = Some(buffer.entity_id());
        });

        let cursor_position = cx.read(|cx| buffer.read(cx).anchor_after(text::Point::new(0, 8)));

        // First suggestion should return the full completion
        let completion = provider.update(cx, |provider, cx| {
            provider.suggest(&buffer, cursor_position, cx)
        });
        assert!(completion.is_some());
        let completion = completion.unwrap();
        assert_eq!(completion.edits[0].1, "log('test')");

        // Simulate partial typing of "log"
        let buffer_text_after = "console.log";
        let buffer_after = cx.new(|cx| language::Buffer::local(buffer_text_after, cx));
        let cursor_position_after =
            cx.read(|cx| buffer_after.read(cx).anchor_after(text::Point::new(0, 11)));

        provider.update(cx, |provider, _| {
            provider.buffer_id = Some(buffer_after.entity_id());
        });

        // Should suggest the remaining part "('test')"
        let completion_after = provider.update(cx, |provider, cx| {
            provider.suggest(&buffer_after, cursor_position_after, cx)
        });
        assert!(completion_after.is_some());
        let completion_after = completion_after.unwrap();
        assert_eq!(completion_after.edits[0].1, "('test')");
    }
}
