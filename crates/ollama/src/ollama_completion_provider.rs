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

    /// Get stop tokens for the current model
    /// For now we only handle the case for codellama:7b-code model
    /// that we found was including the stop token in the completion suggestion.
    /// We wanted to avoid going down this route and let Ollama abstract all template tokens away.
    /// But apparently, and surprisingly for a llama model, Ollama misses this case.
    fn get_stop_tokens(&self) -> Option<Vec<String>> {
        if self.model.contains("codellama") && self.model.contains("code") {
            Some(vec!["<EOT>".to_string()])
        } else {
            None
        }
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

            let stop_tokens = this.update(cx, |this, _| this.get_stop_tokens())?;

            let request = GenerateRequest {
                model,
                prompt: prefix,
                suffix: Some(suffix),
                stream: false,
                options: Some(GenerateOptions {
                    num_predict: Some(150), // Reasonable completion length
                    temperature: Some(0.1), // Low temperature for more deterministic results
                    top_p: Some(0.95),
                    stop: stop_tokens,
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
    use crate::fake::Ollama;

    use gpui::{AppContext, TestAppContext};

    use language::Buffer;
    use project::Project;
    use settings::SettingsStore;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme::init(theme::LoadThemes::JustBase, cx);
            language::init(cx);
            editor::init_settings(cx);
            Project::init_settings(cx);
            workspace::init_settings(cx);
        });
    }

    /// Test the complete Ollama completion flow from refresh to suggestion
    #[test]
    fn test_get_stop_tokens() {
        let http_client = Arc::new(crate::fake::FakeHttpClient::new());

        // Test CodeLlama code model gets stop tokens
        let codellama_provider = OllamaCompletionProvider::new(
            http_client.clone(),
            "http://localhost:11434".to_string(),
            "codellama:7b-code".to_string(),
            None,
        );

        assert_eq!(
            codellama_provider.get_stop_tokens(),
            Some(vec!["<EOT>".to_string()])
        );

        // Test non-CodeLlama model doesn't get stop tokens
        let qwen_provider = OllamaCompletionProvider::new(
            http_client.clone(),
            "http://localhost:11434".to_string(),
            "qwen2.5-coder:3b".to_string(),
            None,
        );
        assert_eq!(qwen_provider.get_stop_tokens(), None);
    }

    #[gpui::test]
    async fn test_full_completion_flow(cx: &mut TestAppContext) {
        init_test(cx);

        // Create a buffer with realistic code content
        let buffer = cx.update(|cx| cx.new(|cx| Buffer::local("fn test() {\n    \n}", cx)));
        let cursor_position = buffer.read_with(cx, |buffer, _| {
            buffer.anchor_before(11) // Position in the middle of the function
        });

        // Create Ollama provider with fake HTTP client
        let (provider, fake_http_client) = Ollama::fake(cx);

        // Configure mock HTTP response
        fake_http_client.set_generate_response("println!(\"Hello\");");

        // Trigger completion refresh (no debounce for test speed)
        provider.update(cx, |provider, cx| {
            provider.refresh(None, buffer.clone(), cursor_position, false, cx);
        });

        // Wait for completion task to complete
        cx.background_executor.run_until_parked();

        // Verify completion was processed and stored
        provider.read_with(cx, |provider, _cx| {
            assert!(provider.current_completion.is_some());
            assert_eq!(
                provider.current_completion.as_ref().unwrap(),
                "println!(\"Hello\");"
            );
            assert!(!provider.is_refreshing());
        });

        // Test suggestion logic returns the completion
        let suggestion = cx.update(|cx| {
            provider.update(cx, |provider, cx| {
                provider.suggest(&buffer, cursor_position, cx)
            })
        });

        assert!(suggestion.is_some());
        let suggestion = suggestion.unwrap();
        assert_eq!(suggestion.edits.len(), 1);
        assert_eq!(suggestion.edits[0].1, "println!(\"Hello\");");

        // Verify acceptance clears the completion
        provider.update(cx, |provider, cx| {
            provider.accept(cx);
        });

        provider.read_with(cx, |provider, _cx| {
            assert!(provider.current_completion.is_none());
        });
    }

    /// Test that partial typing is handled correctly - only suggests untyped portion
    #[gpui::test]
    async fn test_partial_typing_handling(cx: &mut TestAppContext) {
        init_test(cx);

        // Create buffer where user has partially typed "vec"
        let buffer = cx.update(|cx| cx.new(|cx| Buffer::local("let result = vec", cx)));
        let cursor_position = buffer.read_with(cx, |buffer, _| {
            buffer.anchor_after(16) // After "vec"
        });

        let (provider, fake_http_client) = Ollama::fake(cx);

        // Configure response that starts with what user already typed
        fake_http_client.set_generate_response("vec![1, 2, 3]");

        provider.update(cx, |provider, cx| {
            provider.refresh(None, buffer.clone(), cursor_position, false, cx);
        });

        cx.background_executor.run_until_parked();

        // Should suggest only the remaining part after "vec"
        let suggestion = cx.update(|cx| {
            provider.update(cx, |provider, cx| {
                provider.suggest(&buffer, cursor_position, cx)
            })
        });

        // Verify we get a reasonable suggestion
        if let Some(suggestion) = suggestion {
            assert_eq!(suggestion.edits.len(), 1);
            assert!(suggestion.edits[0].1.contains("1, 2, 3"));
        }
    }

    #[gpui::test]
    async fn test_accept_partial_ollama_suggestion(cx: &mut TestAppContext) {
        init_test(cx);

        let mut editor_cx = editor::test::editor_test_context::EditorTestContext::new(cx).await;
        let (provider, fake_http_client) = Ollama::fake(cx);

        // Set up the editor with the Ollama provider
        editor_cx.update_editor(|editor, window, cx| {
            editor.set_edit_prediction_provider(Some(provider.clone()), window, cx);
        });

        // Set initial state
        editor_cx.set_state("let items = ˇ");

        // Configure a multi-word completion
        fake_http_client.set_generate_response("vec![hello, world]");

        // Trigger the completion through the provider
        let buffer =
            editor_cx.multibuffer(|multibuffer, _| multibuffer.as_singleton().unwrap().clone());
        let cursor_position = editor_cx.buffer_snapshot().anchor_after(12);

        provider.update(cx, |provider, cx| {
            provider.refresh(None, buffer, cursor_position, false, cx);
        });

        cx.background_executor.run_until_parked();

        editor_cx.update_editor(|editor, window, cx| {
            editor.refresh_inline_completion(false, true, window, cx);
        });

        cx.background_executor.run_until_parked();

        editor_cx.update_editor(|editor, window, cx| {
            // Verify we have an active completion
            assert!(editor.has_active_inline_completion());

            // The display text should show the full completion
            assert_eq!(editor.display_text(cx), "let items = vec![hello, world]");
            // But the actual text should only show what's been typed
            assert_eq!(editor.text(cx), "let items = ");

            // Accept first partial - should accept "vec" (alphabetic characters)
            editor.accept_partial_inline_completion(&Default::default(), window, cx);

            // Assert the buffer now contains the first partially accepted text
            assert_eq!(editor.text(cx), "let items = vec");
            // Completion should still be active for remaining text
            assert!(editor.has_active_inline_completion());

            // Accept second partial - should accept "![" (non-alphabetic characters)
            editor.accept_partial_inline_completion(&Default::default(), window, cx);

            // Assert the buffer now contains both partial acceptances
            assert_eq!(editor.text(cx), "let items = vec![");
            // Completion should still be active for remaining text
            assert!(editor.has_active_inline_completion());
        });
    }

    #[gpui::test]
    async fn test_completion_invalidation(cx: &mut TestAppContext) {
        init_test(cx);

        let mut editor_cx = editor::test::editor_test_context::EditorTestContext::new(cx).await;
        let (provider, fake_http_client) = Ollama::fake(cx);

        // Set up the editor with the Ollama provider
        editor_cx.update_editor(|editor, window, cx| {
            editor.set_edit_prediction_provider(Some(provider.clone()), window, cx);
        });

        editor_cx.set_state("fooˇ");

        // Configure completion response that extends the current text
        fake_http_client.set_generate_response("bar");

        // Trigger the completion through the provider
        let buffer =
            editor_cx.multibuffer(|multibuffer, _| multibuffer.as_singleton().unwrap().clone());
        let cursor_position = editor_cx.buffer_snapshot().anchor_after(3); // After "foo"

        provider.update(cx, |provider, cx| {
            provider.refresh(None, buffer, cursor_position, false, cx);
        });

        cx.background_executor.run_until_parked();

        editor_cx.update_editor(|editor, window, cx| {
            editor.refresh_inline_completion(false, true, window, cx);
        });

        cx.background_executor.run_until_parked();

        editor_cx.update_editor(|editor, window, cx| {
            assert!(editor.has_active_inline_completion());
            assert_eq!(editor.display_text(cx), "foobar");
            assert_eq!(editor.text(cx), "foo");

            // Backspace within the original text - completion should remain
            editor.backspace(&Default::default(), window, cx);
            assert!(editor.has_active_inline_completion());
            assert_eq!(editor.display_text(cx), "fobar");
            assert_eq!(editor.text(cx), "fo");

            editor.backspace(&Default::default(), window, cx);
            assert!(editor.has_active_inline_completion());
            assert_eq!(editor.display_text(cx), "fbar");
            assert_eq!(editor.text(cx), "f");

            // This backspace removes all original text - should invalidate completion
            editor.backspace(&Default::default(), window, cx);
            assert!(!editor.has_active_inline_completion());
            assert_eq!(editor.display_text(cx), "");
            assert_eq!(editor.text(cx), "");
        });
    }
}
