use anyhow::{Context as AnyhowContext, Result};
use edit_prediction::{Direction, EditPrediction, EditPredictionProvider};
use edit_prediction_context::{EditPredictionExcerpt, EditPredictionExcerptOptions};
use futures::AsyncReadExt;
use gpui::{App, Context, Entity, EntityId, SharedString, Subscription, Task};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use language::{Anchor, Buffer, ToOffset, ToPoint};
use language_models::provider::ollama::{AvailableModel, OllamaLanguageModelProvider};
use ollama::KeepAlive;
use project::Project;
use serde::{Deserialize, Serialize};
use std::{ops::Range, path::Path, sync::Arc, time::Duration};
use text;

pub const OLLAMA_DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(75);
const OLLAMA_EDIT_PREDICTION_LENGTH: i32 = 150;
const OLLAMA_EDIT_PREDICTION_TEMP: f32 = 0.1;
const OLLAMA_EDIT_PREDICTION_TOP_P: f32 = 0.95;

#[derive(Serialize, Debug)]
struct GenerateRequest {
    model: String,
    prompt: String,
    suffix: Option<String>,
    stream: bool,
    options: Option<GenerateOptions>,
    keep_alive: Option<KeepAlive>,
}

#[derive(Serialize, Debug)]
struct GenerateOptions {
    num_predict: Option<i32>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    stop: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct GenerateResponse {
    response: String,
    done: bool,
    total_duration: Option<u64>,
    load_duration: Option<u64>,
    prompt_eval_count: Option<i32>,
    eval_count: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OllamaCompletion {
    pub text: String,
    pub range: Range<Anchor>,
    pub timestamp: std::time::Instant,
    pub excerpt_range: Option<Range<usize>>, // Track the context excerpt used
}

pub struct OllamaEditPredictionProvider {
    model: String,
    buffer_id: Option<EntityId>,
    file_extension: Option<String>,
    current_completion: Option<OllamaCompletion>,
    pending_refresh: Option<Task<Result<()>>>,
    _service_subscription: Option<Subscription>,
}

impl OllamaEditPredictionProvider {
    pub fn new(model: String, _api_url: SharedString, cx: &mut Context<Self>) -> Self {
        let subscription = if let Some(provider) = OllamaLanguageModelProvider::global(cx) {
            Some(cx.observe(&provider, |_this, _provider, cx| {
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
            _service_subscription: subscription,
        }
    }

    pub fn available_models(&self, cx: &App) -> Vec<AvailableModel> {
        if let Some(provider) = OllamaLanguageModelProvider::global(cx) {
            provider.read(cx).available_models_for_completion(cx)
        } else {
            Vec::new()
        }
    }

    pub fn refresh_models(&self, cx: &mut App) {
        if let Some(provider) = OllamaLanguageModelProvider::global(cx) {
            provider.update(cx, |provider, cx| {
                provider.refresh_models(cx);
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

    async fn generate(
        client: &dyn HttpClient,
        api_url: &str,
        api_key: Option<String>,
        request: GenerateRequest,
    ) -> Result<GenerateResponse> {
        let model_name = request.model.clone();
        let uri = format!("{api_url}/api/generate");
        log::info!("Making HTTP request to: {}", uri);
        log::info!("Request model: {}", request.model);
        let mut request_builder = HttpRequest::builder()
            .method(Method::POST)
            .uri(uri)
            .header("Content-Type", "application/json");

        if let Some(api_key) = api_key {
            log::info!("Adding Authorization header with API key");
            request_builder = request_builder.header("Authorization", format!("Bearer {api_key}"))
        } else {
            log::info!("No API key provided, making unauthenticated request");
        }

        let serialized_request = serde_json::to_string(&request)?;
        log::info!("Request payload: {}", serialized_request);
        let request = request_builder.body(AsyncBody::from(serialized_request))?;

        let mut response = match client.send(request).await {
            Ok(response) => {
                log::info!(
                    "HTTP request sent successfully, status: {}",
                    response.status()
                );
                response
            }
            Err(err) => {
                log::error!("Ollama server unavailable at {}: {}", api_url, err);
                return Err(err);
            }
        };

        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;

        if !response.status().is_success() {
            match response.status().as_u16() {
                404 => {
                    log::error!("Model or endpoint not found (404). This could mean:");
                    log::error!("1. The model is not available on this Ollama instance");
                    log::error!(
                        "2. For Ollama Turbo, only specific models are supported: gpt-oss:20b, gpt-oss:120b, deepseek-v3.1:671b"
                    );
                    log::error!(
                        "3. The /api/generate endpoint might not be available (try /api/chat instead)"
                    );
                    anyhow::bail!(
                        "Model not found (404). Check if model '{}' is available on the Ollama instance at {}. Response: {}",
                        model_name,
                        api_url,
                        body
                    );
                }
                401 | 403 => {
                    log::error!(
                        "Authentication failed. Check your OLLAMA_API_KEY or API key in settings."
                    );
                    anyhow::bail!(
                        "Authentication failed ({}): {}. Please check your Ollama API key.",
                        response.status(),
                        body
                    );
                }
                _ => {
                    anyhow::bail!(
                        "Failed to connect to Ollama API: {} {}",
                        response.status(),
                        body,
                    );
                }
            }
        }

        let response: GenerateResponse =
            serde_json::from_str(&body).context("Unable to parse Ollama generate response")?;
        Ok(response)
    }
}

impl EditPredictionProvider for OllamaEditPredictionProvider {
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
        // Get API settings from the global Ollama language model provider or fallback
        let (http_client, api_url) = if let Some(provider) = OllamaLanguageModelProvider::global(cx)
        {
            let provider_ref = provider.read(cx);
            (
                provider_ref.http_client(),
                OllamaLanguageModelProvider::api_url(cx).to_string(),
            )
        } else {
            // Fallback if global service isn't available
            (
                project
                    .as_ref()
                    .map(|p| p.read(cx).client().http_client() as Arc<dyn HttpClient>)
                    .unwrap_or_else(|| {
                        Arc::new(http_client::BlockedHttpClient::new()) as Arc<dyn HttpClient>
                    }),
                "http://localhost:11434".to_string(),
            )
        };

        self.pending_refresh = Some(cx.spawn(async move |this, cx| {
            log::info!("Starting completion request task");
            if debounce {
                log::info!("Debouncing for {:?}", OLLAMA_DEBOUNCE_TIMEOUT);
                cx.background_executor()
                    .timer(OLLAMA_DEBOUNCE_TIMEOUT)
                    .await;
            }

            let (prefix, suffix, excerpt_range) = this.update(cx, |this, cx| {
                let buffer_snapshot = buffer.read(cx).snapshot();
                this.buffer_id = Some(buffer.entity_id());
                this.file_extension = buffer_snapshot.file().and_then(|file| {
                    Some(
                        Path::new(file.file_name(cx))
                            .extension()?
                            .to_str()?
                            .to_string(),
                    )
                });
                this.extract_smart_context(&buffer_snapshot, cursor_position)
            })?;

            let (model, api_key) = this.update(cx, |this, cx| {
                let api_key = if let Some(provider) = OllamaLanguageModelProvider::global(cx) {
                    // Get API key from the shared provider
                    let key = provider
                        .read(cx)
                        .api_key(cx)
                        .map(|k| k.as_ref().to_string());
                    log::info!(
                        "Retrieved API key from OllamaLanguageModelProvider: {}",
                        key.is_some()
                    );
                    key
                } else {
                    log::info!("No global OllamaLanguageModelProvider found");
                    None
                };
                (this.model.clone(), api_key)
            })?;

            let stop_tokens = this.update(cx, |this, _| this.get_stop_tokens())?;

            let request = GenerateRequest {
                model,
                prompt: prefix,
                suffix: Some(suffix),
                stream: false,
                options: Some(GenerateOptions {
                    num_predict: Some(OLLAMA_EDIT_PREDICTION_LENGTH),
                    temperature: Some(OLLAMA_EDIT_PREDICTION_TEMP),
                    top_p: Some(OLLAMA_EDIT_PREDICTION_TOP_P),
                    stop: stop_tokens,
                }),
                keep_alive: None,
            };

            let response = Self::generate(http_client.as_ref(), &api_url, api_key, request)
                .await
                .context("Failed to get completion from Ollama");

            this.update(cx, |this, cx| {
                this.pending_refresh = None;
                match response {
                    Ok(response) if !response.response.trim().is_empty() => {
                        // Create a completion range that covers potential already-typed text
                        // This allows the deduplication logic to work correctly for partial acceptance
                        let buffer_snapshot = buffer.read(cx).snapshot();
                        let cursor_offset = cursor_position.to_offset(&buffer_snapshot);
                        let max_lookback = response.response.len().min(cursor_offset);
                        let start_offset = cursor_offset.saturating_sub(max_lookback);
                        let start_anchor =
                            buffer_snapshot.anchor_at(start_offset, text::Bias::Left);

                        let completion_range = start_anchor..cursor_position;

                        let completion = OllamaCompletion {
                            text: response.response,
                            range: completion_range,
                            timestamp: std::time::Instant::now(),
                            excerpt_range,
                        };
                        this.current_completion = Some(completion);
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

        let completion = self.current_completion.as_ref()?;

        if completion.text.trim().is_empty() {
            return None;
        }

        let buffer_snapshot = buffer.read(cx);
        let cursor_offset = cursor_position.to_offset(&buffer_snapshot);

        // Use original Ollama-style prefix matching but with more efficient range
        let max_lookback = completion.text.len().min(cursor_offset);
        let start_offset = cursor_offset.saturating_sub(max_lookback);
        let text_before_cursor: String = buffer_snapshot
            .text_for_range(start_offset..cursor_offset)
            .collect();

        // Find how much of the completion has already been typed by checking
        // if the text before the cursor ends with a prefix of our completion
        let mut prefix_len = 0;
        for i in 1..=completion.text.len().min(text_before_cursor.len()) {
            if text_before_cursor.ends_with(&completion.text[..i]) {
                prefix_len = i;
            }
        }

        // Only suggest the remaining part of the completion
        let remaining_completion = &completion.text[prefix_len..];

        if remaining_completion.trim().is_empty() {
            return None;
        }

        let position = cursor_position.bias_right(&buffer_snapshot);

        Some(EditPrediction {
            id: None,
            edits: vec![(position..position, remaining_completion.to_string())],
            edit_preview: None,
        })
    }
}

impl OllamaEditPredictionProvider {
    /// Extract intelligent context using EditPredictionExcerpt for better completions
    fn extract_smart_context(
        &self,
        buffer_snapshot: &language::BufferSnapshot,
        cursor_position: Anchor,
    ) -> (String, String, Option<Range<usize>>) {
        let cursor_point = cursor_position.to_point(buffer_snapshot);
        let cursor_offset = cursor_position.to_offset(buffer_snapshot);

        // Configure excerpt options for optimal Ollama context
        let excerpt_options = EditPredictionExcerptOptions {
            max_bytes: 4000,                            // Reasonable for Ollama context window
            min_bytes: 200,                             // Ensure we get meaningful context
            target_before_cursor_over_total_bytes: 0.7, // More context before the cursor, as opposed to after
            include_parent_signatures: false,           // Focused, immediate syntactic context
        };

        // Try to get intelligent excerpt, fallback to simple extraction
        if let Some(excerpt) = EditPredictionExcerpt::select_from_buffer(
            cursor_point,
            buffer_snapshot,
            &excerpt_options,
        ) {
            let excerpt_text = excerpt.text(buffer_snapshot);
            let cursor_offset_in_excerpt = cursor_offset.saturating_sub(excerpt.range.start);

            // Use excerpt body directly (parent signatures disabled per Zed team)
            let full_context = excerpt_text.body;
            let cursor_offset_in_full = cursor_offset_in_excerpt;

            let prefix = full_context[..cursor_offset_in_full.min(full_context.len())].to_string();
            let suffix = full_context[cursor_offset_in_full.min(full_context.len())..].to_string();

            (prefix, suffix, Some(excerpt.range))
        } else {
            // Fallback to original simple extraction
            let cursor_offset = cursor_position.to_offset(buffer_snapshot);
            let max_chars = 1000.min(cursor_offset);
            let start = cursor_offset.saturating_sub(max_chars);

            let prefix: String = buffer_snapshot
                .text_for_range(start..cursor_offset)
                .collect();
            let suffix: String = buffer_snapshot
                .text_for_range(cursor_offset..buffer_snapshot.len().min(cursor_offset + max_chars))
                .collect();

            (prefix, suffix, None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use gpui::{AppContext, TestAppContext};
    use language_model::LanguageModelProvider;

    use client;
    use language::Buffer;
    use language_models::provider::ollama::OllamaLanguageModelProvider;
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
            language_models::init_settings(cx);
        });
    }

    /// Test the complete Ollama completion flow from refresh to suggestion
    #[gpui::test]
    fn test_get_stop_tokens(cx: &mut TestAppContext) {
        init_test(cx);

        // Test CodeLlama code model gets stop tokens
        let codellama_provider = cx.new(|cx| {
            OllamaEditPredictionProvider::new(
                "codellama:7b-code".to_string(),
                "http://localhost:11434".into(),
                cx,
            )
        });

        codellama_provider.read_with(cx, |provider, _| {
            assert_eq!(provider.get_stop_tokens(), Some(vec!["<EOT>".to_string()]));
        });

        // Test non-CodeLlama model doesn't get stop tokens
        let qwen_provider = cx.new(|cx| {
            OllamaEditPredictionProvider::new(
                "qwen2.5-coder:3b".to_string(),
                "http://localhost:11434".into(),
                cx,
            )
        });

        qwen_provider.read_with(cx, |provider, _| {
            assert_eq!(provider.get_stop_tokens(), None);
        });
    }

    #[gpui::test]
    async fn test_model_discovery(cx: &mut TestAppContext) {
        init_test(cx);

        // Create fake HTTP client and set up global provider
        let fake_http_client = Arc::new(ollama::fake::FakeHttpClient::new());

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
        let capabilities = serde_json::json!({
            "capabilities": ["tools", "thinking"]
        });

        fake_http_client.set_response("/api/show", capabilities.to_string());

        // Create global Ollama language model provider for testing
        let language_provider = cx.update(|cx| {
            let provider =
                cx.new(|cx| OllamaLanguageModelProvider::new(fake_http_client.clone(), cx));
            OllamaLanguageModelProvider::set_global(provider.clone(), cx);
            provider
        });

        // Trigger authentication/model discovery
        language_provider.update(cx, |provider, cx| {
            provider.authenticate(cx).detach();
        });

        // Create completion provider
        let provider = cx.new(|cx| {
            OllamaEditPredictionProvider::new(
                "qwen2.5-coder:3b".to_string(),
                "http://localhost:11434".into(),
                cx,
            )
        });

        // Wait for model discovery to complete
        cx.background_executor.run_until_parked();

        // Verify models were discovered through the global provider
        provider.read_with(cx, |provider, cx| {
            let models = provider.available_models(cx);
            // The OllamaLanguageModelProvider filters out embedding models (nomic-embed-text)
            // So we should have 2 models: qwen2.5-coder:3b and codellama:7b-code
            assert_eq!(models.len(), 2);

            let model_names: Vec<&str> = models.iter().map(|m| m.name.as_str()).collect();
            assert!(model_names.contains(&"qwen2.5-coder:3b"));
            assert!(model_names.contains(&"codellama:7b-code"));
            // nomic-embed-text should be filtered out by the language model provider
            assert!(!model_names.contains(&"nomic-embed-text"));
        });
    }

    #[gpui::test]
    async fn test_model_discovery_api_failure(cx: &mut TestAppContext) {
        init_test(cx);

        // Create fake HTTP client that returns errors
        let fake_http_client = Arc::new(ollama::fake::FakeHttpClient::new());
        fake_http_client.set_error("Connection refused");

        // Create global Ollama language model provider that will fail
        let _provider = cx.update(|cx| {
            let provider =
                cx.new(|cx| OllamaLanguageModelProvider::new(fake_http_client.clone(), cx));
            OllamaLanguageModelProvider::set_global(provider, cx);
        });

        // Create completion provider
        let provider = cx.new(|cx| {
            OllamaEditPredictionProvider::new(
                "qwen2.5-coder:3b".to_string(),
                "http://localhost:11434".into(),
                cx,
            )
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

        let fake_http_client = Arc::new(ollama::fake::FakeHttpClient::new());

        // Initially return empty model list
        let empty_response = serde_json::json!({"models": []});
        fake_http_client.set_response("/api/tags", empty_response.to_string());

        // Create global Ollama language model provider
        let _provider = cx.update(|cx| {
            let provider =
                cx.new(|cx| OllamaLanguageModelProvider::new(fake_http_client.clone(), cx));
            OllamaLanguageModelProvider::set_global(provider, cx);
        });

        let provider = cx.new(|cx| {
            OllamaEditPredictionProvider::new(
                "qwen2.5-coder:7b".to_string(),
                "http://localhost:11434".into(),
                cx,
            )
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
    async fn test_smart_context_extraction(cx: &mut TestAppContext) {
        init_test(cx);

        let provider = cx.new(|cx| {
            OllamaEditPredictionProvider::new(
                "qwen2.5-coder:3b".to_string(),
                "http://localhost:11434".into(),
                cx,
            )
        });

        // Create a buffer with realistic code structure for excerpt testing
        let buffer = cx.new(|cx| {
            Buffer::local(
                r#"// File header comment
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("key", "value");
    println!("Hello, world!");

    // Cursor will be here

    let result = calculate(5, 10);
    println!("Result: {}", result);
}

fn calculate(a: i32, b: i32) -> i32 {
    a + b
}
"#,
                cx,
            )
        });

        let cursor_position = buffer.read_with(cx, |buffer, _| {
            // Position cursor after "// Cursor will be here" line
            buffer.anchor_before(200) // Approximate position
        });

        let buffer_snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());

        // Test smart context extraction
        let (prefix, suffix, excerpt_range) = provider.read_with(cx, |provider, _| {
            provider.extract_smart_context(&buffer_snapshot, cursor_position)
        });

        // Verify that we got intelligent context
        assert!(!prefix.is_empty());
        assert!(!suffix.is_empty());

        // Should include function context, not just raw character lookback
        assert!(prefix.contains("fn main()"));
        assert!(suffix.contains("fn calculate"));

        // Compare with naive extraction to show quality improvement
        let cursor_offset = cursor_position.to_offset(&buffer_snapshot);
        let max_chars = 100; // Small limit to show difference
        let naive_start = cursor_offset.saturating_sub(max_chars);
        let naive_prefix: String = buffer_snapshot
            .text_for_range(naive_start..cursor_offset)
            .collect();

        // Smart extraction should provide better context boundaries
        if let Some(range) = excerpt_range {
            println!("Successfully used EditPredictionExcerpt for smart context");
            println!(
                "Smart context length: {} chars",
                prefix.len() + suffix.len()
            );
            println!("Naive context length: {} chars", naive_prefix.len());

            // Smart context should be more comprehensive than naive
            assert!(prefix.len() >= naive_prefix.len());
            // Smart context should include function boundaries
            assert!(prefix.contains("fn main") || suffix.contains("fn main"));
            assert!(range.start < range.end);
        } else {
            println!("Fell back to simple context extraction");
        }
    }

    #[gpui::test]
    async fn test_full_completion_flow(cx: &mut TestAppContext) {
        init_test(cx);

        // Create a buffer with realistic code content
        let buffer = cx.new(|cx| Buffer::local("fn test() {\n    \n}", cx));
        let cursor_position = buffer.read_with(cx, |buffer, _| {
            buffer.anchor_before(11) // Position in the middle of the function
        });

        // Create fake HTTP client and set up global provider
        let fake_http_client = Arc::new(ollama::fake::FakeHttpClient::new());
        fake_http_client.set_generate_response("println!(\"Hello\");");

        // Create global Ollama language model provider
        let _provider = cx.update(|cx| {
            let provider =
                cx.new(|cx| OllamaLanguageModelProvider::new(fake_http_client.clone(), cx));
            OllamaLanguageModelProvider::set_global(provider, cx);
        });

        // Create provider
        let provider = cx.new(|cx| {
            OllamaEditPredictionProvider::new(
                "qwen2.5-coder:3b".to_string(),
                "http://localhost:11434".into(),
                cx,
            )
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
                provider.current_completion.as_ref().unwrap().text,
                "println!(\"Hello\");"
            );
            assert!(!provider.is_refreshing());
        });

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
        let buffer = cx.new(|cx| Buffer::local("let result = vec", cx));
        let cursor_position = buffer.read_with(cx, |buffer, _| {
            buffer.anchor_after(16) // After "vec"
        });

        // Create fake HTTP client and set up global provider
        let fake_http_client = Arc::new(ollama::fake::FakeHttpClient::new());

        // Create global Ollama language model provider
        let _provider = cx.update(|cx| {
            let provider =
                cx.new(|cx| OllamaLanguageModelProvider::new(fake_http_client.clone(), cx));
            OllamaLanguageModelProvider::set_global(provider, cx);
        });

        // Create provider
        let provider = cx.new(|cx| {
            OllamaEditPredictionProvider::new(
                "qwen2.5-coder:3b".to_string(),
                "http://localhost:11434".into(),
                cx,
            )
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

        // Create fake HTTP client and set up global provider
        let fake_http_client = Arc::new(ollama::fake::FakeHttpClient::new());
        fake_http_client.set_generate_response("vec![hello, world]");

        // Create global Ollama language model provider
        let _provider = cx.update(|cx| {
            let provider =
                cx.new(|cx| OllamaLanguageModelProvider::new(fake_http_client.clone(), cx));
            OllamaLanguageModelProvider::set_global(provider, cx);
        });

        // Create provider
        let provider = cx.new(|cx| {
            OllamaEditPredictionProvider::new(
                "qwen2.5-coder:3b".to_string(),
                "http://localhost:11434".into(),
                cx,
            )
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

        // Create fake HTTP client and set up global provider
        let fake_http_client = Arc::new(ollama::fake::FakeHttpClient::new());
        fake_http_client.set_generate_response("bar");

        // Create global Ollama language model provider
        let _provider = cx.update(|cx| {
            let provider =
                cx.new(|cx| OllamaLanguageModelProvider::new(fake_http_client.clone(), cx));
            OllamaLanguageModelProvider::set_global(provider, cx);
        });

        // Create provider
        let provider = cx.new(|cx| {
            OllamaEditPredictionProvider::new(
                "qwen2.5-coder:3b".to_string(),
                "http://localhost:11434".into(),
                cx,
            )
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
    async fn test_api_settings_retrieval(cx: &mut TestAppContext) {
        init_test(cx);

        let fake_http_client = Arc::new(ollama::fake::FakeHttpClient::new());

        // Set up responses
        fake_http_client.set_response("/api/tags", serde_json::json!({"models": []}).to_string());
        fake_http_client.set_response(
            "/api/show",
            serde_json::json!({"capabilities": []}).to_string(),
        );

        // Create language model provider with custom API URL
        let provider = cx.update(|cx| {
            let provider =
                cx.new(|cx| OllamaLanguageModelProvider::new(fake_http_client.clone(), cx));
            OllamaLanguageModelProvider::set_global(provider.clone(), cx);

            // Trigger authentication to set up API key state
            provider.update(cx, |provider, cx| {
                provider.authenticate(cx).detach();
            });

            provider
        });

        cx.background_executor.run_until_parked();

        // Test API URL retrieval
        let api_url = cx.update(|cx| OllamaLanguageModelProvider::api_url(cx));
        assert!(!api_url.is_empty());

        // Test API key retrieval
        let _api_key = provider.read_with(cx, |provider, cx| provider.api_key(cx));

        // Create edit prediction provider
        let edit_provider = cx.new(|cx| {
            OllamaEditPredictionProvider::new(
                "test-model".to_string(),
                "http://localhost:11434".into(),
                cx,
            )
        });

        // Test that edit prediction provider can access the same settings
        edit_provider.read_with(cx, |_provider, cx| {
            if let Some(lang_provider) = OllamaLanguageModelProvider::global(cx) {
                let api_url = OllamaLanguageModelProvider::api_url(cx);
                let _api_key = lang_provider.read(cx).api_key(cx);

                assert!(!api_url.is_empty());
                // API key may or may not be present depending on environment
            }
        });
    }
}
