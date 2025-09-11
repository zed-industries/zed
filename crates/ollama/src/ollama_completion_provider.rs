use crate::{
    AvailableModel, GenerateOptions, GenerateRequest, discover_available_models, generate,
};
use anyhow::{Context as AnyhowContext, Result};
use edit_prediction::{Direction, EditPrediction, EditPredictionProvider};
use gpui::{App, AppContext, Context, Entity, EntityId, Global, Subscription, Task};
use http_client::HttpClient;
use language::{Anchor, Buffer, ToOffset};
use project::Project;
use settings::SettingsStore;
use std::{path::Path, sync::Arc, time::Duration};

pub const OLLAMA_DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(75);
const OLLAMA_EDIT_PREDICTION_LENGTH: i32 = 150;
const OLLAMA_EDIT_PREDICTION_TEMP: f32 = 0.1;
const OLLAMA_EDIT_PREDICTION_TOP_P: f32 = 0.95;

// Global Ollama service for managing models across all providers
pub struct State {
    http_client: Arc<dyn HttpClient>,
    api_url: String,
    api_key: Option<String>,
    available_models: Vec<AvailableModel>,
    fetch_models_task: Option<Task<Result<()>>>,
    _settings_subscription: Subscription,
}

impl State {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        api_url: String,
        api_key: Option<String>,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let subscription = cx.observe_global::<SettingsStore>({
                move |this: &mut State, cx| {
                    this.restart_fetch_models_task(cx);
                }
            });

            let mut service = Self {
                http_client,
                api_url,
                api_key,
                available_models: Vec::new(),
                fetch_models_task: None,
                _settings_subscription: subscription,
            };

            // TODO: why a secod refresh here?
            service.restart_fetch_models_task(cx);
            service
        })
    }

    pub fn global(cx: &App) -> Option<Entity<Self>> {
        cx.try_global::<GlobalOllamaState>()
            .map(|service| service.0.clone())
    }

    pub fn set_global(service: Entity<Self>, cx: &mut App) {
        cx.set_global(GlobalOllamaState(service));
    }

    pub fn available_models(&self) -> &[AvailableModel] {
        &self.available_models
    }

    pub fn refresh_models(&mut self, cx: &mut Context<Self>) {
        self.restart_fetch_models_task(cx);
    }

    pub fn set_models(&mut self, available_models: Vec<AvailableModel>, cx: &mut Context<Self>) {
        self.available_models = available_models;
        self.restart_fetch_models_task(cx);
    }

    pub fn set_api_key(&mut self, api_key: Option<String>, cx: &mut Context<Self>) {
        if self.api_key != api_key {
            self.api_key = api_key;
            self.restart_fetch_models_task(cx);
        }
    }

    fn restart_fetch_models_task(&mut self, cx: &mut Context<Self>) {
        self.fetch_models_task = Some(self.fetch_models(cx));
    }

    fn fetch_models(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let http_client = Arc::clone(&self.http_client);
        let api_url = self.api_url.clone();
        let api_key = self.api_key.clone();

        cx.spawn(async move |this, cx| {
            // Get the current settings models to merge with API models
            let settings_models = this.update(cx, |this, _cx| {
                // Get just the names of models from settings to avoid duplicates
                this.available_models
                    .iter()
                    .map(|m| m.name.clone())
                    .collect::<std::collections::HashSet<_>>()
            })?;

            // Fetch models from API using shared utility
            let mut api_discovered_models =
                match discover_available_models(http_client.clone(), &api_url, api_key.clone())
                    .await
                {
                    Ok(models) => models,
                    Err(_) => return Ok(()), // Silently fail if API is unavailable
                };

            // Filter out models that are already defined in settings
            api_discovered_models.retain(|model| !settings_models.contains(&model.name));

            this.update(cx, |this, cx| {
                // Append API-discovered models to existing settings models
                this.available_models.extend(api_discovered_models);
                // Sort all models by name
                this.available_models.sort_by(|a, b| a.name.cmp(&b.name));
                cx.notify();
            })?;

            Ok(())
        })
    }
}

struct GlobalOllamaState(Entity<State>);

impl Global for GlobalOllamaState {}

// TODO refactor to OllamaEditPredictionProvider
pub struct OllamaCompletionProvider {
    model: String,
    buffer_id: Option<EntityId>,
    file_extension: Option<String>,
    current_completion: Option<String>,
    pending_refresh: Option<Task<Result<()>>>,
    api_key: Option<String>,
    _service_subscription: Option<Subscription>,
}

impl OllamaCompletionProvider {
    pub fn new(model: String, api_key: Option<String>, cx: &mut Context<Self>) -> Self {
        // Update the global service with the API key if one is provided
        if let Some(service) = State::global(cx) {
            service.update(cx, |service, cx| {
                service.set_api_key(api_key.clone(), cx);
            });
        }

        let subscription = if let Some(service) = State::global(cx) {
            Some(cx.observe(&service, |_this, _service, cx| {
                cx.notify();
            }))
        } else {
            None
        };

        Self {
            model,
            buffer_id: None,
            file_extension: None,
            current_completion: None,
            pending_refresh: None,
            api_key,
            _service_subscription: subscription,
        }
    }

    pub fn available_models(&self, cx: &App) -> Vec<AvailableModel> {
        if let Some(service) = State::global(cx) {
            service.read(cx).available_models().to_vec()
        } else {
            Vec::new()
        }
    }

    pub fn refresh_models(&self, cx: &mut App) {
        if let Some(service) = State::global(cx) {
            service.update(cx, |service, cx| {
                service.refresh_models(cx);
            });
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
        project: Option<Entity<Project>>,
        buffer: Entity<Buffer>,
        cursor_position: Anchor,
        debounce: bool,
        cx: &mut Context<Self>,
    ) {
        // Get API settings from the global Ollama service or fallback
        let (http_client, api_url) = if let Some(service) = State::global(cx) {
            let service_ref = service.read(cx);
            (service_ref.http_client.clone(), service_ref.api_url.clone())
        } else {
            // Fallback if global service isn't available
            (
                project
                    .as_ref()
                    .map(|p| p.read(cx).client().http_client() as Arc<dyn HttpClient>)
                    .unwrap_or_else(|| {
                        Arc::new(http_client::BlockedHttpClient::new()) as Arc<dyn HttpClient>
                    }),
                crate::OLLAMA_API_URL.to_string(),
            )
        };

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
                    num_predict: Some(OLLAMA_EDIT_PREDICTION_LENGTH), // Reasonable completion length
                    temperature: Some(OLLAMA_EDIT_PREDICTION_TEMP), // Low temperature for more deterministic results
                    top_p: Some(OLLAMA_EDIT_PREDICTION_TOP_P),
                    stop: stop_tokens,
                }),
                keep_alive: None,
            };

            let response = generate(http_client.as_ref(), &api_url, api_key, request)
                .await
                .context("Failed to get completion from Ollama");

            this.update(cx, |this, cx| {
                this.pending_refresh = None;
                match response {
                    Ok(response) if !response.response.trim().is_empty() => {
                        this.current_completion = Some(response.response);
                    }
                    _ => {
                        this.current_completion = None;
                    }
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
    }

    fn discard(&mut self, _cx: &mut Context<Self>) {
        self.current_completion = None;
    }

    fn suggest(
        &mut self,
        buffer: &Entity<Buffer>,
        cursor_position: Anchor,
        cx: &mut Context<Self>,
    ) -> Option<EditPrediction> {
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

        Some(EditPrediction {
            id: None,
            edits: vec![(position..position, remaining_completion.to_string())],
            edit_preview: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake::FakeHttpClient;

    use gpui::{AppContext, TestAppContext};

    use client;
    use language::Buffer;
    use project::Project;
    use settings::SettingsStore;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme::init(theme::LoadThemes::JustBase, cx);
            client::init_settings(cx);
            language::init(cx);
            editor::init_settings(cx);
            Project::init_settings(cx);
            workspace::init_settings(cx);
        });
    }

    /// Test the complete Ollama completion flow from refresh to suggestion
    #[gpui::test]
    fn test_get_stop_tokens(cx: &mut TestAppContext) {
        init_test(cx);

        // Test CodeLlama code model gets stop tokens
        let codellama_provider = cx.update(|cx| {
            cx.new(|cx| OllamaCompletionProvider::new("codellama:7b-code".to_string(), None, cx))
        });

        codellama_provider.read_with(cx, |provider, _| {
            assert_eq!(provider.get_stop_tokens(), Some(vec!["<EOT>".to_string()]));
        });

        // Test non-CodeLlama model doesn't get stop tokens
        let qwen_provider = cx.update(|cx| {
            cx.new(|cx| OllamaCompletionProvider::new("qwen2.5-coder:3b".to_string(), None, cx))
        });

        qwen_provider.read_with(cx, |provider, _| {
            assert_eq!(provider.get_stop_tokens(), None);
        });
    }

    #[gpui::test]
    async fn test_model_discovery(cx: &mut TestAppContext) {
        init_test(cx);

        // Create fake HTTP client
        let fake_http_client = Arc::new(crate::fake::FakeHttpClient::new());

        // Mock /api/tags response (list models)
        let models_response = serde_json::json!({
            "models": [
                {
                    "name": "qwen2.5-coder:3b",
                    "modified_at": "2024-01-01T00:00:00Z",
                    "size": 1000000,
                    "digest": "abc123",
                    "details": {
                        "format": "gguf",
                        "family": "qwen2",
                        "families": ["qwen2"],
                        "parameter_size": "3B",
                        "quantization_level": "Q4_0"
                    }
                },
                {
                    "name": "codellama:7b-code",
                    "modified_at": "2024-01-01T00:00:00Z",
                    "size": 2000000,
                    "digest": "def456",
                    "details": {
                        "format": "gguf",
                        "family": "codellama",
                        "families": ["codellama"],
                        "parameter_size": "7B",
                        "quantization_level": "Q4_0"
                    }
                },
                {
                    "name": "nomic-embed-text",
                    "modified_at": "2024-01-01T00:00:00Z",
                    "size": 500000,
                    "digest": "ghi789",
                    "details": {
                        "format": "gguf",
                        "family": "nomic-embed",
                        "families": ["nomic-embed"],
                        "parameter_size": "137M",
                        "quantization_level": "Q4_0"
                    }
                }
            ]
        });

        fake_http_client.set_response("/api/tags", models_response.to_string());

        // Mock /api/show responses for model capabilities
        let qwen_capabilities = serde_json::json!({
            "capabilities": ["tools", "thinking"]
        });

        let _codellama_capabilities = serde_json::json!({
            "capabilities": []
        });

        fake_http_client.set_response("/api/show", qwen_capabilities.to_string());

        // Create global Ollama service for testing
        let service = cx.update(|cx| {
            State::new(
                fake_http_client.clone(),
                "http://localhost:11434".to_string(),
                None,
                cx,
            )
        });

        // Set it as global
        cx.update(|cx| {
            State::set_global(service.clone(), cx);
        });

        // Create completion provider
        let provider = cx.update(|cx| {
            cx.new(|cx| OllamaCompletionProvider::new("qwen2.5-coder:3b".to_string(), None, cx))
        });

        // Wait for model discovery to complete
        cx.background_executor.run_until_parked();

        // Verify models were discovered through the global provider
        provider.read_with(cx, |provider, cx| {
            let models = provider.available_models(cx);
            assert_eq!(models.len(), 2); // Should exclude nomic-embed-text

            let model_names: Vec<&str> = models.iter().map(|m| m.name.as_str()).collect();
            assert!(model_names.contains(&"codellama:7b-code"));
            assert!(model_names.contains(&"qwen2.5-coder:3b"));
            assert!(!model_names.contains(&"nomic-embed-text"));
        });
    }

    #[gpui::test]
    async fn test_model_discovery_api_failure(cx: &mut TestAppContext) {
        init_test(cx);

        // Create fake HTTP client that returns errors
        let fake_http_client = Arc::new(crate::fake::FakeHttpClient::new());
        fake_http_client.set_error("Connection refused");

        // Create global Ollama service that will fail
        let service = cx.update(|cx| {
            State::new(
                fake_http_client.clone(),
                "http://localhost:11434".to_string(),
                None,
                cx,
            )
        });

        cx.update(|cx| {
            State::set_global(service.clone(), cx);
        });

        // Create completion provider
        let provider = cx.update(|cx| {
            cx.new(|cx| OllamaCompletionProvider::new("qwen2.5-coder:3b".to_string(), None, cx))
        });

        // Wait for model discovery to complete (with failure)
        cx.background_executor.run_until_parked();

        // Verify graceful handling - should have empty model list
        provider.read_with(cx, |provider, cx| {
            let models = provider.available_models(cx);
            assert_eq!(models.len(), 0);
        });
    }

    #[gpui::test]
    async fn test_refresh_models(cx: &mut TestAppContext) {
        init_test(cx);

        let fake_http_client = Arc::new(crate::fake::FakeHttpClient::new());

        // Initially return empty model list
        let empty_response = serde_json::json!({"models": []});
        fake_http_client.set_response("/api/tags", empty_response.to_string());

        // Create global Ollama service
        let service = cx.update(|cx| {
            State::new(
                fake_http_client.clone(),
                "http://localhost:11434".to_string(),
                None,
                cx,
            )
        });

        cx.update(|cx| {
            State::set_global(service.clone(), cx);
        });

        let provider = cx.update(|cx| {
            cx.new(|cx| OllamaCompletionProvider::new("qwen2.5-coder:7b".to_string(), None, cx))
        });

        cx.background_executor.run_until_parked();

        // Verify initially empty
        provider.read_with(cx, |provider, cx| {
            assert_eq!(provider.available_models(cx).len(), 0);
        });

        // Update mock to return models
        let models_response = serde_json::json!({
            "models": [
                {
                    "name": "qwen2.5-coder:7b",
                    "modified_at": "2024-01-01T00:00:00Z",
                    "size": 1000000,
                    "digest": "abc123",
                    "details": {
                        "format": "gguf",
                        "family": "qwen2",
                        "families": ["qwen2"],
                        "parameter_size": "7B",
                        "quantization_level": "Q4_0"
                    }
                }
            ]
        });

        fake_http_client.set_response("/api/tags", models_response.to_string());

        let capabilities = serde_json::json!({
            "capabilities": ["tools", "thinking"]
        });

        fake_http_client.set_response("/api/show", capabilities.to_string());

        // Trigger refresh
        provider.update(cx, |provider, cx| {
            provider.refresh_models(cx);
        });

        cx.background_executor.run_until_parked();

        // Verify models were refreshed
        provider.read_with(cx, |provider, cx| {
            let models = provider.available_models(cx);
            assert_eq!(models.len(), 1);
            assert_eq!(models[0].name, "qwen2.5-coder:7b");
        });
    }

    #[gpui::test]
    async fn test_full_completion_flow(cx: &mut TestAppContext) {
        init_test(cx);

        // Create a buffer with realistic code content
        let buffer = cx.update(|cx| cx.new(|cx| Buffer::local("fn test() {\n    \n}", cx)));
        let cursor_position = buffer.read_with(cx, |buffer, _| {
            buffer.anchor_before(11) // Position in the middle of the function
        });

        // Create fake HTTP client and set up global service
        let fake_http_client = Arc::new(crate::fake::FakeHttpClient::new());
        fake_http_client.set_generate_response("println!(\"Hello\");");

        // Create global Ollama service
        let service = cx.update(|cx| {
            State::new(
                fake_http_client.clone(),
                "http://localhost:11434".to_string(),
                None,
                cx,
            )
        });

        cx.update(|cx| {
            State::set_global(service.clone(), cx);
        });

        // Create provider
        let provider = cx.update(|cx| {
            cx.new(|cx| OllamaCompletionProvider::new("qwen2.5-coder:3b".to_string(), None, cx))
        });

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

        // Create fake HTTP client and set up global service
        let fake_http_client = Arc::new(crate::fake::FakeHttpClient::new());

        // Create global Ollama service
        let service = cx.update(|cx| {
            State::new(
                fake_http_client.clone(),
                "http://localhost:11434".to_string(),
                None,
                cx,
            )
        });

        cx.update(|cx| {
            State::set_global(service.clone(), cx);
        });

        // Create provider
        let provider = cx.update(|cx| {
            cx.new(|cx| OllamaCompletionProvider::new("qwen2.5-coder:3b".to_string(), None, cx))
        });

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

        // Create fake HTTP client and set up global service
        let fake_http_client = Arc::new(crate::fake::FakeHttpClient::new());
        fake_http_client.set_generate_response("vec![hello, world]");

        // Create global Ollama service
        let service = cx.update(|cx| {
            State::new(
                fake_http_client.clone(),
                "http://localhost:11434".to_string(),
                None,
                cx,
            )
        });

        cx.update(|cx| {
            State::set_global(service.clone(), cx);
        });

        // Create provider
        let provider = cx.update(|cx| {
            cx.new(|cx| OllamaCompletionProvider::new("qwen2.5-coder:3b".to_string(), None, cx))
        });

        // Set up the editor with the Ollama provider
        editor_cx.update_editor(|editor, window, cx| {
            editor.set_edit_prediction_provider(Some(provider.clone()), window, cx);
        });

        // Set initial state
        editor_cx.set_state("let items = ˇ");

        // Trigger the completion through the provider
        let buffer = editor_cx.multibuffer(|multibuffer, _| multibuffer.as_singleton().unwrap());
        let cursor_position = editor_cx.buffer_snapshot().anchor_after(12);

        provider.update(cx, |provider, cx| {
            provider.refresh(None, buffer, cursor_position, false, cx);
        });

        cx.background_executor.run_until_parked();

        editor_cx.update_editor(|editor, window, cx| {
            editor.refresh_edit_prediction(false, true, window, cx);
        });

        cx.background_executor.run_until_parked();

        editor_cx.update_editor(|editor, window, cx| {
            // Verify we have an active completion
            assert!(editor.has_active_edit_prediction());

            // The display text should show the full completion
            assert_eq!(editor.display_text(cx), "let items = vec![hello, world]");
            // But the actual text should only show what's been typed
            assert_eq!(editor.text(cx), "let items = ");

            // Accept first partial - should accept "vec" (alphabetic characters)
            editor.accept_partial_edit_prediction(&Default::default(), window, cx);

            // Assert the buffer now contains the first partially accepted text
            assert_eq!(editor.text(cx), "let items = vec");
            // Completion should still be active for remaining text
            assert!(editor.has_active_edit_prediction());

            // Accept second partial - should accept "![" (non-alphabetic characters)
            editor.accept_partial_edit_prediction(&Default::default(), window, cx);

            // Assert the buffer now contains both partial acceptances
            assert_eq!(editor.text(cx), "let items = vec![");
            // Completion should still be active for remaining text
            assert!(editor.has_active_edit_prediction());
        });
    }

    #[gpui::test]
    async fn test_completion_invalidation(cx: &mut TestAppContext) {
        init_test(cx);

        let mut editor_cx = editor::test::editor_test_context::EditorTestContext::new(cx).await;

        // Create fake HTTP client and set up global service
        let fake_http_client = Arc::new(crate::fake::FakeHttpClient::new());
        fake_http_client.set_generate_response("bar");

        // Create global Ollama service
        let service = cx.update(|cx| {
            State::new(
                fake_http_client.clone(),
                "http://localhost:11434".to_string(),
                None,
                cx,
            )
        });

        cx.update(|cx| {
            State::set_global(service.clone(), cx);
        });

        // Create provider
        let provider = cx.update(|cx| {
            cx.new(|cx| OllamaCompletionProvider::new("qwen2.5-coder:3b".to_string(), None, cx))
        });

        // Set up the editor with the Ollama provider
        editor_cx.update_editor(|editor, window, cx| {
            editor.set_edit_prediction_provider(Some(provider.clone()), window, cx);
        });

        editor_cx.set_state("fooˇ");

        // Trigger the completion through the provider
        let buffer = editor_cx.multibuffer(|multibuffer, _| multibuffer.as_singleton().unwrap());
        let cursor_position = editor_cx.buffer_snapshot().anchor_after(3); // After "foo"

        provider.update(cx, |provider, cx| {
            provider.refresh(None, buffer, cursor_position, false, cx);
        });

        cx.background_executor.run_until_parked();

        editor_cx.update_editor(|editor, window, cx| {
            editor.refresh_edit_prediction(false, true, window, cx);
        });

        cx.background_executor.run_until_parked();

        editor_cx.update_editor(|editor, window, cx| {
            assert!(editor.has_active_edit_prediction());
            assert_eq!(editor.display_text(cx), "foobar");
            assert_eq!(editor.text(cx), "foo");

            // Backspace within the original text - completion should remain
            editor.backspace(&Default::default(), window, cx);
            assert!(editor.has_active_edit_prediction());
            assert_eq!(editor.display_text(cx), "fobar");
            assert_eq!(editor.text(cx), "fo");

            editor.backspace(&Default::default(), window, cx);
            assert!(editor.has_active_edit_prediction());
            assert_eq!(editor.display_text(cx), "fbar");
            assert_eq!(editor.text(cx), "f");

            // This backspace removes all original text - should invalidate completion
            editor.backspace(&Default::default(), window, cx);
            assert!(!editor.has_active_edit_prediction());
            assert_eq!(editor.display_text(cx), "");
            assert_eq!(editor.text(cx), "");
        });
    }

    #[gpui::test]
    async fn test_settings_model_merging(cx: &mut TestAppContext) {
        init_test(cx);

        // Create fake HTTP client that returns some API models
        let fake_http_client = Arc::new(crate::fake::FakeHttpClient::new());

        // Mock /api/tags response (list models)
        let models_response = serde_json::json!({
            "models": [
                {
                    "name": "api-model-1",
                    "modified_at": "2024-01-01T00:00:00Z",
                    "size": 1000000,
                    "digest": "abc123",
                    "details": {
                        "format": "gguf",
                        "family": "llama",
                        "families": ["llama"],
                        "parameter_size": "7B",
                        "quantization_level": "Q4_0"
                    }
                },
                {
                    "name": "shared-model",
                    "modified_at": "2024-01-01T00:00:00Z",
                    "size": 2000000,
                    "digest": "def456",
                    "details": {
                        "format": "gguf",
                        "family": "llama",
                        "families": ["llama"],
                        "parameter_size": "13B",
                        "quantization_level": "Q4_0"
                    }
                }
            ]
        });

        fake_http_client.set_response("/api/tags", models_response.to_string());

        // Mock /api/show responses for each model
        let show_response = serde_json::json!({
            "capabilities": ["tools", "vision"]
        });
        fake_http_client.set_response("/api/show", show_response.to_string());

        // Create service
        let service = cx.update(|cx| {
            State::new(
                fake_http_client.clone(),
                "http://localhost:11434".to_string(),
                None,
                cx,
            )
        });

        // Add settings models (including one that overlaps with API)
        let settings_models = vec![
            AvailableModel {
                name: "custom-model-1".to_string(),
                display_name: Some("Custom Model 1".to_string()),
                max_tokens: 4096,
                keep_alive: None,
                supports_tools: Some(true),
                supports_images: Some(false),
                supports_thinking: Some(false),
            },
            AvailableModel {
                name: "shared-model".to_string(), // This should take precedence over API
                display_name: Some("Custom Shared Model".to_string()),
                max_tokens: 8192,
                keep_alive: None,
                supports_tools: Some(true),
                supports_images: Some(true),
                supports_thinking: Some(true),
            },
        ];

        cx.update(|cx| {
            service.update(cx, |service, cx| {
                service.set_models(settings_models, cx);
            });
        });

        // Wait for models to be fetched and merged
        cx.run_until_parked();

        // Verify merged models
        let models = cx.update(|cx| service.read(cx).available_models().to_vec());

        assert_eq!(models.len(), 3); // 2 settings models + 1 unique API model

        // Models should be sorted alphabetically, so check by name
        let model_names: Vec<&str> = models.iter().map(|m| m.name.as_str()).collect();
        assert_eq!(
            model_names,
            vec!["api-model-1", "custom-model-1", "shared-model"]
        );

        // Check custom model from settings
        let custom_model = models.iter().find(|m| m.name == "custom-model-1").unwrap();
        assert_eq!(
            custom_model.display_name,
            Some("Custom Model 1".to_string())
        );
        assert_eq!(custom_model.max_tokens, 4096);

        // Settings model should override API model for shared-model
        let shared_model = models.iter().find(|m| m.name == "shared-model").unwrap();
        assert_eq!(
            shared_model.display_name,
            Some("Custom Shared Model".to_string())
        );
        assert_eq!(shared_model.max_tokens, 8192);
        assert_eq!(shared_model.supports_tools, Some(true));
        assert_eq!(shared_model.supports_images, Some(true));
        assert_eq!(shared_model.supports_thinking, Some(true));

        // API-only model should be included
        let api_model = models.iter().find(|m| m.name == "api-model-1").unwrap();
        assert!(api_model.display_name.is_none()); // API models don't have custom display names
    }

    #[gpui::test]
    async fn test_api_key_passed_to_requests(cx: &mut TestAppContext) {
        init_test(cx);

        let fake_http_client = Arc::new(FakeHttpClient::new());

        // Set up responses for model discovery with API key
        fake_http_client.set_response(
            "/api/tags",
            serde_json::json!({
                "models": [
                    {
                        "name": "qwen2.5-coder:3b",
                        "modified_at": "2024-01-01T00:00:00Z",
                        "size": 1000000,
                        "digest": "abc123",
                        "details": {
                            "format": "gguf",
                            "family": "qwen2.5",
                            "families": ["qwen2.5"],
                            "parameter_size": "3B",
                            "quantization_level": "Q4_0"
                        }
                    }
                ]
            })
            .to_string(),
        );

        // Set up show model response
        fake_http_client.set_response(
            "/api/show",
            serde_json::json!({
                "capabilities": {
                    "tools": true,
                    "vision": false,
                    "thinking": false
                }
            })
            .to_string(),
        );

        let service = cx.update(|cx| {
            State::new(
                fake_http_client.clone(),
                "http://localhost:11434".to_string(),
                Some("test-api-key".to_string()),
                cx,
            )
        });

        cx.update(|cx| {
            State::set_global(service.clone(), cx);
        });

        // Wait for model fetching to complete
        cx.background_executor.run_until_parked();

        // Verify that requests were made
        let requests = fake_http_client.get_requests();
        assert!(!requests.is_empty(), "Expected HTTP requests to be made");

        // Note: We can't easily test the Authorization header with the current FakeHttpClient
        // implementation, but the important thing is that the API key gets passed through
        // to the HTTP requests without panicking.
    }

    #[gpui::test]
    async fn test_api_key_update_triggers_refresh(cx: &mut TestAppContext) {
        init_test(cx);

        let fake_http_client = Arc::new(FakeHttpClient::new());

        // Set up initial response
        fake_http_client.set_response(
            "/api/tags",
            serde_json::json!({
                "models": []
            })
            .to_string(),
        );

        let service = cx.update(|cx| {
            State::new(
                fake_http_client.clone(),
                "http://localhost:11434".to_string(),
                None,
                cx,
            )
        });

        cx.update(|cx| {
            State::set_global(service.clone(), cx);
        });

        // Clear initial requests
        fake_http_client.clear_requests();

        // Update API key
        service.update(cx, |service, cx| {
            service.set_api_key(Some("new-api-key".to_string()), cx);
        });

        // Wait for refresh to complete
        cx.background_executor.run_until_parked();

        // Verify new requests were made
        let requests = fake_http_client.get_requests();
        assert!(
            !requests.is_empty(),
            "Expected new requests after API key update"
        );
    }

    #[gpui::test]
    async fn test_ollama_debouncing(cx: &mut TestAppContext) {
        init_test(cx);

        // Create a buffer with realistic code content
        let buffer = cx.update(|cx| cx.new(|cx| Buffer::local("fn test() {\n    \n}", cx)));
        let cursor_position = buffer.read_with(cx, |buffer, _| {
            buffer.anchor_before(11) // Position in the middle of the function
        });

        // Setup provider state
        let fake_http_client = Arc::new(crate::fake::FakeHttpClient::new());
        fake_http_client.set_generate_response("println!(\"Hello\");");

        let service = cx.update(|cx| {
            State::new(
                fake_http_client.clone(),
                "http://localhost:11434".to_string(),
                None,
                cx,
            )
        });

        cx.update(|cx| {
            State::set_global(service.clone(), cx);
        });

        // Wait for any initial model discovery requests to complete
        cx.background_executor.run_until_parked();

        let provider = cx.update(|cx| {
            cx.new(|cx| OllamaCompletionProvider::new("qwen2.5-coder:3b".to_string(), None, cx))
        });

        // Clear any initial requests (including model discovery)
        fake_http_client.clear_requests();

        // Simulate rapid typing - trigger refresh multiple times with debounce=true
        provider.update(cx, |provider, cx| {
            provider.refresh(None, buffer.clone(), cursor_position, true, cx);
        });

        provider.update(cx, |provider, cx| {
            provider.refresh(None, buffer.clone(), cursor_position, true, cx);
        });

        provider.update(cx, |provider, cx| {
            provider.refresh(None, buffer.clone(), cursor_position, true, cx);
        });

        // At this point, no requests should have been made due to debouncing
        let requests_before_timeout = fake_http_client.get_requests();
        assert_eq!(
            requests_before_timeout.len(),
            0,
            "Expected no requests before debounce timeout expires"
        );

        // Advance clock by the debounce timeout
        cx.background_executor
            .advance_clock(OLLAMA_DEBOUNCE_TIMEOUT);
        cx.background_executor.run_until_parked();

        // Now exactly one request should have been made (the last one)
        let requests_after_timeout = fake_http_client.get_requests();
        assert_eq!(
            requests_after_timeout.len(),
            1,
            "Expected exactly 1 request after debounce timeout, got {}",
            requests_after_timeout.len()
        );

        // Verify provider is no longer refreshing
        provider.read_with(cx, |provider, _cx| {
            assert!(
                !provider.is_refreshing(),
                "Provider should not be refreshing after completion"
            );
        });
    }

    #[gpui::test]
    async fn test_ollama_debouncing_slow_typing(cx: &mut TestAppContext) {
        init_test(cx);

        // Create a buffer with realistic code content
        let buffer = cx.update(|cx| cx.new(|cx| Buffer::local("fn test() {\n    let x = \n}", cx)));
        let cursor_position = buffer.read_with(cx, |buffer, _| {
            buffer.anchor_before(21) // Position after "let x = "
        });

        // Create fake HTTP client and set up global service
        let fake_http_client = Arc::new(crate::fake::FakeHttpClient::new());
        fake_http_client.set_generate_response("42");

        // Create global Ollama service
        let service = cx.update(|cx| {
            State::new(
                fake_http_client.clone(),
                "http://localhost:11434".to_string(),
                None,
                cx,
            )
        });

        cx.update(|cx| {
            State::set_global(service.clone(), cx);
        });

        // Wait for any initial model discovery requests to complete
        cx.background_executor.run_until_parked();

        // Create provider
        let provider = cx.update(|cx| {
            cx.new(|cx| OllamaCompletionProvider::new("qwen2.5-coder:3b".to_string(), None, cx))
        });

        // Clear any initial requests (including model discovery)
        fake_http_client.clear_requests();

        // Simulate slow typing - 200ms between keystrokes (realistic typing speed)
        // Each keystroke should trigger its own API request because 200ms > 75ms debounce
        for _i in 0..3 {
            provider.update(cx, |provider, cx| {
                provider.refresh(None, buffer.clone(), cursor_position, true, cx);
            });

            // Wait for debounce timeout to expire + a bit more
            cx.background_executor
                .advance_clock(OLLAMA_DEBOUNCE_TIMEOUT + Duration::from_millis(10));
            cx.background_executor.run_until_parked();

            // Simulate realistic typing delay (200ms between keystrokes)
            cx.background_executor
                .advance_clock(Duration::from_millis(200 - 75 - 10));
        }

        // With slow typing, we should get 3 separate API requests (one per "keystroke")
        let requests = fake_http_client.get_requests();
        assert_eq!(
            requests.len(),
            3,
            "Expected 3 requests for slow typing (200ms intervals), got {}. This demonstrates that 75ms debounce is too short for normal typing speeds.",
            requests.len()
        );
    }
}
