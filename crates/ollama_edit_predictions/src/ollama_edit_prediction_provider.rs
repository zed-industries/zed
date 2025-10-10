use anyhow::{Context as AnyhowContext, Result};
use edit_prediction::{Direction, EditPrediction, EditPredictionProvider};
use edit_prediction_context::{EditPredictionExcerpt, EditPredictionExcerptOptions};
use futures::AsyncReadExt;
use gpui::{App, Context, Entity, EntityId, Subscription, Task};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use language::{Anchor, Buffer, ToOffset, ToPoint};
use language_models::provider::ollama::OllamaLanguageModelProvider;
use ollama::KeepAlive;
use ollama::Model;

use serde::{Deserialize, Serialize};
use settings::Settings;
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
    pub excerpt_range: Option<Range<usize>>,
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
    pub fn new(model: String, cx: &mut Context<Self>) -> Self {
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

    pub fn available_models(&self, cx: &App) -> Vec<Model> {
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

    pub fn update_model(&mut self, model: String) {
        self.model = model;
    }

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
        let mut request_builder = HttpRequest::builder()
            .method(Method::POST)
            .uri(uri)
            .header("Content-Type", "application/json");

        if let Some(api_key) = api_key {
            request_builder = request_builder.header("Authorization", format!("Bearer {api_key}"))
        }

        let serialized_request = serde_json::to_string(&request)?;
        let request = request_builder.body(AsyncBody::from(serialized_request))?;

        let mut response = match client.send(request).await {
            Ok(response) => response,
            Err(err) => {
                return Err(err);
            }
        };

        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;

        if !response.status().is_success() {
            match response.status().as_u16() {
                404 => {
                    anyhow::bail!(
                        "Model not found (404). Check if model '{}' is available on the Ollama instance at {}. Response: {}",
                        model_name,
                        api_url,
                        body
                    );
                }
                401 | 403 => {
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

    fn is_enabled(&self, _buffer: &Entity<Buffer>, _cursor_position: Anchor, cx: &App) -> bool {
        if let Some(provider) = OllamaLanguageModelProvider::global(cx) {
            let provider_ref = provider.read(cx);
            let available_models = provider_ref.available_models_for_completion(cx);
            !available_models.is_empty()
        } else {
            // Provider might still be initializing, check if we have a configured model in settings
            let settings = &language_models::AllLanguageModelSettings::get_global(cx).ollama;
            !settings.available_models.is_empty()
        }
    }

    fn is_refreshing(&self) -> bool {
        self.pending_refresh.is_some()
    }

    fn refresh(
        &mut self,
        buffer: Entity<Buffer>,
        cursor_position: Anchor,
        debounce: bool,
        cx: &mut Context<Self>,
    ) {
        let (http_client, api_url) = if let Some(provider) = OllamaLanguageModelProvider::global(cx)
        {
            let provider_ref = provider.read(cx);
            let url = OllamaLanguageModelProvider::api_url(cx).to_string();
            (provider_ref.http_client(), url)
        } else {
            (
                Arc::new(http_client::BlockedHttpClient::new()) as Arc<dyn HttpClient>,
                "http://localhost:11434".to_string(),
            )
        };

        self.pending_refresh = Some(cx.spawn(async move |this, cx| {
            if debounce {
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
                    let key = provider
                        .read(cx)
                        .api_key(cx)
                        .map(|k| k.as_ref().to_string());
                    key
                } else {
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

        let max_lookback = completion.text.len().min(cursor_offset);
        let start_offset = cursor_offset.saturating_sub(max_lookback);
        let text_before_cursor: String = buffer_snapshot
            .text_for_range(start_offset..cursor_offset)
            .collect();

        let mut prefix_len = 0;
        for i in 1..=completion.text.len().min(text_before_cursor.len()) {
            if text_before_cursor.ends_with(&completion.text[..i]) {
                prefix_len = i;
            }
        }

        let remaining_completion = &completion.text[prefix_len..];

        if remaining_completion.trim().is_empty() {
            return None;
        }

        let position = cursor_position.bias_right(&buffer_snapshot);

        Some(EditPrediction::Local {
            id: None,
            edits: vec![(position..position, remaining_completion.to_string())],
            edit_preview: None,
        })
    }
}

impl OllamaEditPredictionProvider {
    fn extract_smart_context(
        &self,
        buffer_snapshot: &language::BufferSnapshot,
        cursor_position: Anchor,
    ) -> (String, String, Option<Range<usize>>) {
        let cursor_point = cursor_position.to_point(buffer_snapshot);
        let cursor_offset = cursor_position.to_offset(buffer_snapshot);

        let excerpt_options = EditPredictionExcerptOptions {
            max_bytes: 4000,                            // Reasonable for Ollama context window
            min_bytes: 200,                             // Ensure we get meaningful context
            target_before_cursor_over_total_bytes: 0.7, // More context before the cursor, as opposed to after
        };

        if let Some(excerpt) = EditPredictionExcerpt::select_from_buffer(
            cursor_point,
            buffer_snapshot,
            &excerpt_options,
            None,
        ) {
            let excerpt_text = excerpt.text(buffer_snapshot);
            let cursor_offset_in_excerpt = cursor_offset.saturating_sub(excerpt.range.start);

            let full_context = excerpt_text.body;
            let cursor_offset_in_full = cursor_offset_in_excerpt;

            let prefix = full_context[..cursor_offset_in_full.min(full_context.len())].to_string();
            let suffix = full_context[cursor_offset_in_full.min(full_context.len())..].to_string();

            (prefix, suffix, Some(excerpt.range))
        } else {
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
    use http_client::{AsyncBody, FakeHttpClient, Response};
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

    #[gpui::test]
    fn test_get_stop_tokens(cx: &mut TestAppContext) {
        init_test(cx);

        let codellama_provider =
            cx.new(|cx| OllamaEditPredictionProvider::new("codellama:7b-code".to_string(), cx));

        codellama_provider.read_with(cx, |provider, _| {
            assert_eq!(provider.get_stop_tokens(), Some(vec!["<EOT>".to_string()]));
        });

        let qwen_provider =
            cx.new(|cx| OllamaEditPredictionProvider::new("qwen2.5-coder:3b".to_string(), cx));

        qwen_provider.read_with(cx, |provider, _| {
            assert_eq!(provider.get_stop_tokens(), None);
        });
    }

    #[gpui::test]
    async fn test_model_discovery(cx: &mut TestAppContext) {
        init_test(cx);

        let empty_response = serde_json::json!({"models": []});
        let fake_http_client = FakeHttpClient::create({
            let empty_response = empty_response.to_string();
            move |req| {
                let response_body = empty_response.clone();
                async move {
                    match req.uri().path() {
                        "/api/tags" => Ok(Response::builder()
                            .status(200)
                            .header("content-type", "application/json")
                            .body(AsyncBody::from(response_body))
                            .unwrap()),
                        _ => Err(anyhow::anyhow!(
                            "No mock response set for {}",
                            req.uri().path()
                        )),
                    }
                }
            }
        });

        let _language_provider = cx.update(|cx| {
            let provider =
                cx.new(|cx| OllamaLanguageModelProvider::new(fake_http_client.clone(), cx));
            OllamaLanguageModelProvider::set_global(provider.clone(), cx);
            provider
        });

        let provider =
            cx.new(|cx| OllamaEditPredictionProvider::new("qwen2.5-coder:3b".to_string(), cx));

        cx.background_executor.run_until_parked();

        provider.read_with(cx, |provider, cx| {
            assert_eq!(provider.available_models(cx).len(), 0);
        });

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

        let capabilities = serde_json::json!({
            "capabilities": ["tools", "thinking"]
        });

        let fake_http_client = FakeHttpClient::create({
            let models_response = models_response.to_string();
            let capabilities_response = capabilities.to_string();
            move |req| {
                let models_response = models_response.clone();
                let capabilities_response = capabilities_response.clone();
                async move {
                    match req.uri().path() {
                        "/api/tags" => Ok(Response::builder()
                            .status(200)
                            .header("content-type", "application/json")
                            .body(AsyncBody::from(models_response))
                            .unwrap()),
                        "/api/show" => Ok(Response::builder()
                            .status(200)
                            .header("content-type", "application/json")
                            .body(AsyncBody::from(capabilities_response))
                            .unwrap()),
                        _ => Err(anyhow::anyhow!(
                            "No mock response set for {}",
                            req.uri().path()
                        )),
                    }
                }
            }
        });

        let language_provider = cx.update(|cx| {
            let provider =
                cx.new(|cx| OllamaLanguageModelProvider::new(fake_http_client.clone(), cx));
            OllamaLanguageModelProvider::set_global(provider.clone(), cx);
            provider
        });

        language_provider.update(cx, |provider, cx| {
            provider.authenticate(cx).detach();
        });

        provider.update(cx, |provider, cx| {
            provider.refresh_models(cx);
        });

        cx.background_executor.run_until_parked();

        provider.read_with(cx, |provider, cx| {
            let models = provider.available_models(cx);
            assert_eq!(models.len(), 1);

            let model_names: Vec<&str> = models.iter().map(|m| m.name.as_str()).collect();
            assert!(model_names.contains(&"qwen2.5-coder:3b"));
            assert!(!model_names.contains(&"nomic-embed-text"));
        });
    }

    #[gpui::test]
    async fn test_model_discovery_api_failure(cx: &mut TestAppContext) {
        init_test(cx);

        let fake_http_client =
            FakeHttpClient::create(
                |_req| async move { Err(anyhow::anyhow!("Connection refused")) },
            );

        let _provider = cx.update(|cx| {
            let provider =
                cx.new(|cx| OllamaLanguageModelProvider::new(fake_http_client.clone(), cx));
            OllamaLanguageModelProvider::set_global(provider, cx);
        });

        let provider =
            cx.new(|cx| OllamaEditPredictionProvider::new("qwen2.5-coder:3b".to_string(), cx));

        cx.background_executor.run_until_parked();

        provider.read_with(cx, |provider, cx| {
            let models = provider.available_models(cx);
            assert_eq!(models.len(), 0);
        });
    }

    #[gpui::test]
    async fn test_full_completion_flow(cx: &mut TestAppContext) {
        init_test(cx);

        let buffer = cx.new(|cx| Buffer::local("fn test() {\n    \n}", cx));
        let cursor_position = buffer.read_with(cx, |buffer, _| {
            buffer.anchor_before(11) // Position in the middle of the function
        });

        let fake_http_client = FakeHttpClient::create(move |req| async move {
            match req.uri().path() {
                "/api/generate" => {
                    let response = serde_json::json!({
                        "response": "println!(\"Hello\");",
                        "done": true,
                        "context": [],
                        "total_duration": 1000000_u64,
                        "load_duration": 1000000_u64,
                        "prompt_eval_count": 10,
                        "prompt_eval_duration": 1000000_u64,
                        "eval_count": 20,
                        "eval_duration": 1000000_u64
                    });
                    Ok(Response::builder()
                        .status(200)
                        .header("content-type", "application/json")
                        .body(AsyncBody::from(response.to_string()))
                        .unwrap())
                }
                _ => Err(anyhow::anyhow!(
                    "No mock response set for {}",
                    req.uri().path()
                )),
            }
        });

        let _provider = cx.update(|cx| {
            let provider =
                cx.new(|cx| OllamaLanguageModelProvider::new(fake_http_client.clone(), cx));
            OllamaLanguageModelProvider::set_global(provider, cx);
        });

        let provider =
            cx.new(|cx| OllamaEditPredictionProvider::new("qwen2.5-coder:3b".to_string(), cx));

        provider.update(cx, |provider, cx| {
            provider.refresh(buffer.clone(), cursor_position, false, cx);
        });

        cx.background_executor.run_until_parked();

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
        if let EditPrediction::Local { edits, .. } = &suggestion {
            assert_eq!(edits.len(), 1);
            assert_eq!(edits[0].1, "println!(\"Hello\");");
        } else {
            panic!("Expected Local EditPrediction");
        }

        provider.update(cx, |provider, cx| {
            provider.accept(cx);
        });

        provider.read_with(cx, |provider, _cx| {
            assert!(provider.current_completion.is_none());
        });

        // Partial typing scenario
        let buffer = cx.new(|cx| Buffer::local("let result = vec", cx));
        let cursor_position = buffer.read_with(cx, |buffer, _| {
            buffer.anchor_after(16) // After "vec"
        });

        let fake_http_client = FakeHttpClient::create(move |req| async move {
            match req.uri().path() {
                "/api/generate" => {
                    let response = serde_json::json!({
                        "response": "vec![1, 2, 3]",
                        "done": true,
                        "context": [],
                        "total_duration": 1000000_u64,
                        "load_duration": 1000000_u64,
                        "prompt_eval_count": 10,
                        "prompt_eval_duration": 1000000_u64,
                        "eval_count": 20,
                        "eval_duration": 1000000_u64
                    });
                    Ok(Response::builder()
                        .status(200)
                        .header("content-type", "application/json")
                        .body(AsyncBody::from(response.to_string()))
                        .unwrap())
                }
                _ => Err(anyhow::anyhow!(
                    "No mock response set for {}",
                    req.uri().path()
                )),
            }
        });

        let _provider = cx.update(|cx| {
            let provider =
                cx.new(|cx| OllamaLanguageModelProvider::new(fake_http_client.clone(), cx));
            OllamaLanguageModelProvider::set_global(provider, cx);
        });

        provider.update(cx, |provider, cx| {
            provider.refresh(buffer.clone(), cursor_position, false, cx);
        });

        cx.background_executor.run_until_parked();

        let suggestion = cx.update(|cx| {
            provider.update(cx, |provider, cx| {
                provider.suggest(&buffer, cursor_position, cx)
            })
        });

        if let Some(suggestion) = suggestion {
            if let EditPrediction::Local { edits, .. } = &suggestion {
                assert_eq!(edits.len(), 1);
                assert!(edits[0].1.contains("1, 2, 3"));
            } else {
                panic!("Expected Local EditPrediction");
            }
        }
    }

    #[gpui::test]
    async fn test_api_settings_retrieval(cx: &mut TestAppContext) {
        init_test(cx);

        let fake_http_client = FakeHttpClient::create(move |req| async move {
            match req.uri().path() {
                "/api/tags" => Ok(Response::builder()
                    .status(200)
                    .header("content-type", "application/json")
                    .body(AsyncBody::from(
                        serde_json::json!({"models": []}).to_string(),
                    ))
                    .unwrap()),
                "/api/show" => Ok(Response::builder()
                    .status(200)
                    .header("content-type", "application/json")
                    .body(AsyncBody::from(
                        serde_json::json!({"capabilities": []}).to_string(),
                    ))
                    .unwrap()),
                _ => Err(anyhow::anyhow!(
                    "No mock response set for {}",
                    req.uri().path()
                )),
            }
        });

        let provider = cx.update(|cx| {
            let provider =
                cx.new(|cx| OllamaLanguageModelProvider::new(fake_http_client.clone(), cx));
            OllamaLanguageModelProvider::set_global(provider.clone(), cx);

            provider.update(cx, |provider, cx| {
                provider.authenticate(cx).detach();
            });

            provider
        });

        cx.background_executor.run_until_parked();

        let api_url = cx.update(|cx| OllamaLanguageModelProvider::api_url(cx));
        assert!(!api_url.is_empty());

        let _api_key = provider.read_with(cx, |provider, cx| provider.api_key(cx));

        let edit_provider =
            cx.new(|cx| OllamaEditPredictionProvider::new("test-model".to_string(), cx));

        edit_provider.read_with(cx, |_provider, cx| {
            if let Some(lang_provider) = OllamaLanguageModelProvider::global(cx) {
                let api_url = OllamaLanguageModelProvider::api_url(cx);
                let _api_key = lang_provider.read(cx).api_key(cx);

                assert!(!api_url.is_empty());
            }
        });
    }

    #[gpui::test]
    async fn test_partial_accept_edit_prediction(cx: &mut TestAppContext) {
        init_test(cx);

        let mut editor_cx = editor::test::editor_test_context::EditorTestContext::new(cx).await;

        let fake_http_client = FakeHttpClient::create(move |req| async move {
            match req.uri().path() {
                "/api/tags" => {
                    let response = serde_json::json!({
                        "models": [
                            {
                                "name": "qwen2.5-coder:3b",
                                "model": "qwen2.5-coder:3b",
                                "modified_at": "2023-12-07T09:32:18.757212583Z",
                                "size": 1978143818,
                                "digest": "abc123",
                                "details": {
                                    "parent_model": "",
                                    "format": "gguf",
                                    "family": "qwen2",
                                    "families": ["qwen2"],
                                    "parameter_size": "3B",
                                    "quantization_level": "Q4_0"
                                }
                            }
                        ]
                    });
                    Ok(Response::builder()
                        .status(200)
                        .header("content-type", "application/json")
                        .body(AsyncBody::from(response.to_string()))
                        .unwrap())
                }
                "/api/show" => {
                    let response = serde_json::json!({
                        "modelfile": "# Modelfile generated by \"ollama show\"\n# To build a new Modelfile based on this, replace FROM with:\n# FROM qwen2.5-coder:3b\n\nFROM /Users/user/.ollama/models/blobs/sha256-abc123\nTEMPLATE \"{{ if .System }}<|im_start|>system\n{{ .System }}<|im_end|>\n{{ end }}{{ if .Prompt }}<|im_start|>user\n{{ .Prompt }}<|im_end|>\n{{ end }}<|im_start|>assistant\n{{ .Response }}<|im_end|>\"\nPARAMETER stop \"<|im_start|>\"\nPARAMETER stop \"<|im_end|>\"\n",
                        "parameters": "stop                           [\"<|im_start|>\", \"<|im_end|>\"]\nnum_ctx                        32768",
                        "template": "{{ if .System }}<|im_start|>system\n{{ .System }}<|im_end|>\n{{ end }}{{ if .Prompt }}<|im_start|>user\n{{ .Prompt }}<|im_end|>\n{{ end }}<|im_start|>assistant\n{{ .Response }}<|im_end|>",
                        "details": {
                            "parent_model": "",
                            "format": "gguf",
                            "family": "qwen2",
                            "families": ["qwen2"],
                            "parameter_size": "3B",
                            "quantization_level": "Q4_0"
                        },
                        "model_info": {
                            "general.architecture": "qwen2",
                            "general.file_type": 2,
                            "general.parameter_count": "2720000000",
                            "general.quantization_version": 2,
                            "qwen2.attention.head_count": 24,
                            "qwen2.attention.head_count_kv": 4,
                            "qwen2.attention.layer_norm_rms_epsilon": 1e-6,
                            "qwen2.block_count": 28,
                            "qwen2.context_length": 32768,
                            "qwen2.embedding_length": 2048,
                            "qwen2.feed_forward_length": 11008,
                            "qwen2.rope.freq_base": 1000000,
                            "tokenizer.ggml.bos_token_id": 151643,
                            "tokenizer.ggml.eos_token_id": 151645,
                            "tokenizer.ggml.model": "gpt2",
                            "tokenizer.ggml.token_type_count": 0,
                            "tokenizer.ggml.tokens": []
                        }
                    });
                    Ok(Response::builder()
                        .status(200)
                        .header("content-type", "application/json")
                        .body(AsyncBody::from(response.to_string()))
                        .unwrap())
                }
                "/api/generate" => {
                    let response = serde_json::json!({
                        "response": "hello world",
                        "done": true,
                        "context": [],
                        "total_duration": 1000000_u64,
                        "load_duration": 1000000_u64,
                        "prompt_eval_count": 10,
                        "prompt_eval_duration": 1000000_u64,
                        "eval_count": 20,
                        "eval_duration": 1000000_u64
                    });
                    Ok(Response::builder()
                        .status(200)
                        .header("content-type", "application/json")
                        .body(AsyncBody::from(response.to_string()))
                        .unwrap())
                }
                _ => Err(anyhow::anyhow!(
                    "No mock response set for {}",
                    req.uri().path()
                )),
            }
        });

        let _provider = cx.update(|cx| {
            let provider =
                cx.new(|cx| OllamaLanguageModelProvider::new(fake_http_client.clone(), cx));
            OllamaLanguageModelProvider::set_global(provider, cx);
        });

        // Wait for the provider to fetch models
        cx.background_executor.run_until_parked();

        let provider =
            cx.new(|cx| OllamaEditPredictionProvider::new("qwen2.5-coder:3b".to_string(), cx));

        editor_cx.update_editor(|editor, window, cx| {
            editor.set_edit_prediction_provider(Some(provider.clone()), window, cx);
        });

        editor_cx.set_state("let x = ˇ;");

        editor_cx.update_editor(|editor, window, cx| {
            editor.refresh_edit_prediction(false, true, window, cx);
        });

        cx.background_executor.run_until_parked();

        editor_cx.update_editor(|editor, window, cx| {
            assert!(editor.has_active_edit_prediction());
            assert_eq!(editor.display_text(cx), "let x = hello world;");
            assert_eq!(editor.text(cx), "let x = ;");

            // Accept partial completion - should accept first word
            editor.accept_partial_edit_prediction(&Default::default(), window, cx);

            assert_eq!(editor.text(cx), "let x = hello;");
        });
    }
}
