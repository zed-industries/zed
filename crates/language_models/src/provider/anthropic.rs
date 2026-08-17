pub mod telemetry;

use anthropic::{ANTHROPIC_API_URL, AnthropicError, AnthropicModelMode};
use anyhow::Result;
use collections::BTreeMap;
use credentials_provider::CredentialsProvider;
use futures::{FutureExt, StreamExt, future::BoxFuture, stream::BoxStream};
use gpui::{App, AppContext, AsyncApp, Context, Entity, SharedString, Task};
use http_client::{CustomHeaders, HttpClient};
use language_model::{
    ANTHROPIC_PROVIDER_ID, ANTHROPIC_PROVIDER_NAME, ApiKeyConfiguration, ApiKeyState,
    AuthenticateError, CompactionResult, EnvVar, FastModeConfirmation, IconOrSvg, LanguageModel,
    LanguageModelCompletionError, LanguageModelCompletionEvent, LanguageModelId, LanguageModelName,
    LanguageModelProvider, LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelProviderState, LanguageModelRequest, LanguageModelToolChoice,
    ProviderSettingsView, RateLimiter, env_var,
};
use settings::{Settings, SettingsStore};
use std::sync::{Arc, LazyLock};
use ui::IconName;

use anthropic::completion::collect_compaction_result;
pub use anthropic::completion::{AnthropicEventMapper, AnthropicPromptCacheMode, into_anthropic};
pub use settings::AnthropicAvailableModel as AvailableModel;

const PROVIDER_ID: LanguageModelProviderId = ANTHROPIC_PROVIDER_ID;
const PROVIDER_NAME: LanguageModelProviderName = ANTHROPIC_PROVIDER_NAME;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct AnthropicSettings {
    pub api_url: String,
    /// Extend Zed's list of Anthropic models.
    pub available_models: Vec<AvailableModel>,
    /// User-configured headers added to every Anthropic request.
    pub custom_headers: CustomHeaders,
}

pub struct AnthropicLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

const API_KEY_ENV_VAR_NAME: &str = "ANTHROPIC_API_KEY";
static API_KEY_ENV_VAR: LazyLock<EnvVar> = env_var!(API_KEY_ENV_VAR_NAME);

pub(crate) const RESERVED_HEADER_NAMES: &[&str] =
    &["X-Api-Key", "Anthropic-Version", "Anthropic-Beta"];

pub struct State {
    api_key_state: ApiKeyState,
    credentials_provider: Arc<dyn CredentialsProvider>,
    http_client: Arc<dyn HttpClient>,
    fetched_models: Vec<anthropic::Model>,
    fetch_models_task: Option<Task<Result<()>>>,
}

impl State {
    fn is_authenticated(&self) -> bool {
        self.api_key_state.has_key()
    }

    fn set_api_key(&mut self, api_key: Option<String>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = AnthropicLanguageModelProvider::api_url(cx);
        let should_fetch_models = api_key.is_some();
        let task = self.api_key_state.store(
            api_url,
            api_key,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        );
        self.fetched_models.clear();
        cx.spawn(async move |this, cx| {
            let result = task.await;
            if result.is_ok() && should_fetch_models {
                this.update(cx, |this, cx| this.restart_fetch_models_task(cx))
                    .ok();
            }
            result
        })
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = AnthropicLanguageModelProvider::api_url(cx);
        let task = self.api_key_state.load_if_needed(
            api_url,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        );

        cx.spawn(async move |this, cx| {
            let result = task.await;
            if result.is_ok() {
                this.update(cx, |this, cx| this.restart_fetch_models_task(cx))
                    .ok();
            }
            result
        })
    }

    fn fetch_models(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let http_client = self.http_client.clone();
        let api_url = AnthropicLanguageModelProvider::api_url(cx);
        let Some(api_key) = self.api_key_state.key(&api_url) else {
            return Task::ready(Err(anyhow::anyhow!(
                "cannot fetch Anthropic models without an API key"
            )));
        };
        let extra_headers = AnthropicLanguageModelProvider::settings(cx)
            .custom_headers
            .clone();

        cx.spawn(async move |this, cx| {
            let models = anthropic::list_models(
                http_client.as_ref(),
                &api_url,
                api_key.as_ref(),
                &extra_headers,
            )
            .await?;

            this.update(cx, |this, cx| {
                this.fetched_models = models;
                cx.notify();
            })
        })
    }

    fn restart_fetch_models_task(&mut self, cx: &mut Context<Self>) {
        let task = self.fetch_models(cx);
        self.fetch_models_task.replace(task);
    }
}

impl AnthropicLanguageModelProvider {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut App,
    ) -> Self {
        let state = cx.new(|cx| {
            cx.observe_global::<SettingsStore>({
                let mut last_api_url = Self::api_url(cx);
                move |this: &mut State, cx| {
                    let credentials_provider = this.credentials_provider.clone();
                    let api_url = Self::api_url(cx);
                    let url_changed = api_url != last_api_url;
                    last_api_url = api_url.clone();
                    this.api_key_state.handle_url_change(
                        api_url,
                        |this| &mut this.api_key_state,
                        credentials_provider,
                        cx,
                    );
                    if url_changed {
                        this.fetched_models.clear();
                        this.authenticate(cx).detach();
                    }
                    cx.notify();
                }
            })
            .detach();
            State {
                api_key_state: ApiKeyState::new(Self::api_url(cx), (*API_KEY_ENV_VAR).clone()),
                credentials_provider,
                http_client: http_client.clone(),
                fetched_models: Vec::new(),
                fetch_models_task: None,
            }
        });

        Self { http_client, state }
    }

    fn create_language_model(&self, model: anthropic::Model) -> Arc<dyn LanguageModel> {
        Arc::new(AnthropicModel {
            id: LanguageModelId::from(model.id.to_string()),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }

    fn settings(cx: &App) -> &AnthropicSettings {
        &crate::AllLanguageModelSettings::get_global(cx).anthropic
    }

    fn api_url(cx: &App) -> SharedString {
        let api_url = &Self::settings(cx).api_url;
        if api_url.is_empty() {
            ANTHROPIC_API_URL.into()
        } else {
            SharedString::new(api_url.as_str())
        }
    }
}

impl LanguageModelProviderState for AnthropicLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for AnthropicLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiAnthropic)
    }

    fn default_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        let fetched = self.state.read(cx).fetched_models.clone();
        // Pick the highest-version Sonnet we know about; otherwise the first
        // Claude model returned. Returning `None` until the fetch completes
        // matches the Ollama provider's behavior.
        pick_preferred_model(&fetched, &["claude-sonnet-", "claude-opus-", "claude-"])
            .map(|model| self.create_language_model(model))
    }

    fn default_fast_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        let fetched = self.state.read(cx).fetched_models.clone();
        pick_preferred_model(&fetched, &["claude-haiku-", "claude-"])
            .map(|model| self.create_language_model(model))
    }

    fn recommended_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let fetched = self.state.read(cx).fetched_models.clone();
        pick_preferred_model(&fetched, &["claude-sonnet-"])
            .map(|model| vec![self.create_language_model(model)])
            .unwrap_or_default()
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let mut models: BTreeMap<String, anthropic::Model> = BTreeMap::default();

        // Models reported by Anthropic's `/v1/models` endpoint are the
        // primary source. The list will be empty until authentication has
        // succeeded and the first fetch completes.
        for model in &self.state.read(cx).fetched_models {
            models.insert(model.id.to_string(), model.clone());
        }

        // User-defined `available_models` from settings can either add
        // entirely new entries or override fields on a fetched model with
        // the same id (e.g. enable Fast mode or set a tool override).
        for available in &AnthropicLanguageModelProvider::settings(cx).available_models {
            let model = available_model_to_anthropic_model(available);
            models.insert(model.id.to_string(), model);
        }

        models
            .into_values()
            .map(|model| self.create_language_model(model))
            .collect()
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated()
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        self.state.update(cx, |state, cx| state.authenticate(cx))
    }

    fn settings_view(&self, cx: &mut App) -> Option<ProviderSettingsView> {
        let state = self.state.read(cx);
        Some(ProviderSettingsView::ApiKey(ApiKeyConfiguration::new(
            state.api_key_state.has_key(),
            state.api_key_state.is_from_env_var(),
            state.api_key_state.env_var_name().clone(),
            "https://console.anthropic.com/settings/keys".into(),
        )))
    }

    fn set_api_key(&self, api_key: Option<String>, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.set_api_key(api_key, cx))
    }

    fn fast_mode_confirmation(&self, _cx: &App) -> Option<FastModeConfirmation> {
        Some(FastModeConfirmation {
            title: "Enable Fast Mode for Anthropic?".into(),
            message: "Fast mode lets requests use your Anthropic Priority Tier capacity, which \
                Anthropic prioritizes over standard requests during peak load. Requires a \
                Priority Tier commitment with Anthropic; without one, requests behave the same \
                as the standard tier."
                .into(),
        })
    }
}

/// Pick the model from `models` whose id starts with the earliest matching
/// prefix in `preferred_prefixes`. Within a single prefix bucket the model
/// with the lexicographically greatest id wins, which roughly corresponds to
/// the highest version since Anthropic ids embed dated suffixes.
fn pick_preferred_model(
    models: &[anthropic::Model],
    preferred_prefixes: &[&str],
) -> Option<anthropic::Model> {
    for prefix in preferred_prefixes {
        let candidate = models
            .iter()
            .filter(|m| m.id.starts_with(prefix))
            .max_by(|a, b| a.id.cmp(&b.id));
        if let Some(model) = candidate {
            return Some(model.clone());
        }
    }
    None
}

/// Convert a settings-defined `available_models` entry into an `anthropic::Model`.
fn available_model_to_anthropic_model(available: &AvailableModel) -> anthropic::Model {
    let mode = match available.mode.unwrap_or_default() {
        settings::ModelMode::Default => AnthropicModelMode::Default,
        settings::ModelMode::Thinking { budget_tokens } => {
            AnthropicModelMode::Thinking { budget_tokens }
        }
        settings::ModelMode::Adaptive => AnthropicModelMode::AdaptiveThinking,
    };
    let supports_thinking = matches!(
        mode,
        AnthropicModelMode::Thinking { .. } | AnthropicModelMode::AdaptiveThinking
    );
    let supports_adaptive_thinking = matches!(mode, AnthropicModelMode::AdaptiveThinking);
    let supports_speed = available
        .supports_fast_mode
        .unwrap_or_else(|| anthropic::supports_fast_mode(&available.name));
    let mut extra_beta_headers = available.extra_beta_headers.clone();
    if supports_speed
        && !extra_beta_headers
            .iter()
            .any(|header| header.trim() == anthropic::FAST_MODE_BETA_HEADER)
    {
        extra_beta_headers.push(anthropic::FAST_MODE_BETA_HEADER.to_string());
    }

    anthropic::Model {
        display_name: available
            .display_name
            .clone()
            .unwrap_or_else(|| available.name.clone()),
        id: available.name.clone(),
        max_input_tokens: available.max_tokens,
        max_output_tokens: available.max_output_tokens.unwrap_or(4_096),
        default_temperature: available.default_temperature.unwrap_or(1.0),
        mode,
        supports_thinking,
        supports_adaptive_thinking,
        supports_images: true,
        supports_speed,
        supports_compaction: false,
        supported_effort_levels: if supports_adaptive_thinking {
            vec![
                anthropic::Effort::Low,
                anthropic::Effort::Medium,
                anthropic::Effort::High,
                anthropic::Effort::XHigh,
                anthropic::Effort::Max,
            ]
        } else {
            vec![]
        },
        tool_override: available.tool_override.clone(),
        extra_beta_headers,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::AsyncReadExt as _;
    use http_client::{AsyncBody, FakeHttpClient};
    use language_model::{LanguageModelRequestMessage, MessageContent};
    use serde_json::json;
    use std::sync::Mutex;

    fn parse_available_model(json: &str) -> AvailableModel {
        serde_json::from_str(json).expect("test fixture should parse")
    }

    #[test]
    fn adaptive_mode_maps_to_adaptive_thinking_with_all_effort_levels() {
        let available = parse_available_model(
            r#"{
                "name": "claude-opus-4-7",
                "max_tokens": 1000000,
                "max_output_tokens": 128000,
                "mode": { "type": "adaptive" }
            }"#,
        );
        let model = available_model_to_anthropic_model(&available);

        assert_eq!(model.mode, AnthropicModelMode::AdaptiveThinking);
        assert!(model.supports_thinking);
        assert!(model.supports_adaptive_thinking);
        assert_eq!(
            model.supported_effort_levels,
            vec![
                anthropic::Effort::Low,
                anthropic::Effort::Medium,
                anthropic::Effort::High,
                anthropic::Effort::XHigh,
                anthropic::Effort::Max,
            ]
        );
    }

    #[test]
    fn thinking_mode_does_not_enable_adaptive() {
        let available = parse_available_model(
            r#"{
                "name": "claude-sonnet-4-5",
                "max_tokens": 200000,
                "mode": { "type": "thinking", "budget_tokens": 4096 }
            }"#,
        );
        let model = available_model_to_anthropic_model(&available);

        assert!(matches!(model.mode, AnthropicModelMode::Thinking { .. }));
        assert!(model.supports_thinking);
        assert!(!model.supports_adaptive_thinking);
        assert!(model.supported_effort_levels.is_empty());
    }

    #[test]
    fn default_mode_disables_thinking() {
        let available = parse_available_model(
            r#"{
                "name": "claude-3-5-haiku",
                "max_tokens": 200000
            }"#,
        );
        let model = available_model_to_anthropic_model(&available);

        assert_eq!(model.mode, AnthropicModelMode::Default);
        assert!(!model.supports_thinking);
        assert!(!model.supports_adaptive_thinking);
        assert!(model.supported_effort_levels.is_empty());
    }

    #[gpui::test]
    fn direct_anthropic_supports_explicit_compaction_after_minimum_input(
        cx: &mut gpui::TestAppContext,
    ) {
        let provider = direct_anthropic_test_provider(FakeHttpClient::with_404_response(), cx);
        let model = direct_anthropic_test_model(&provider);

        assert!(model.supports_explicit_compaction());
        assert_eq!(
            model.minimum_explicit_compaction_input_tokens(),
            Some(anthropic::MIN_COMPACTION_TRIGGER_TOKENS)
        );
    }

    #[gpui::test]
    async fn direct_anthropic_explicit_compaction_uses_paused_completion(
        cx: &mut gpui::TestAppContext,
    ) {
        let captured_request = Arc::new(Mutex::new(None));
        let captured_request_for_handler = captured_request.clone();
        let http_client = FakeHttpClient::create(move |request| {
            let captured_request = captured_request_for_handler.clone();
            async move {
                if request.uri().path() == "/v1/models" {
                    return Ok(http_client::Response::builder()
                        .status(200)
                        .body(AsyncBody::from(r#"{"data":[]}"#))?);
                }

                let beta_header = request
                    .headers()
                    .get("Anthropic-Beta")
                    .and_then(|value| value.to_str().ok())
                    .map(str::to_string);
                let mut body = request.into_body();
                let mut body_text = String::new();
                body.read_to_string(&mut body_text).await?;
                *captured_request.lock().unwrap() = Some((beta_header, body_text));

                let response_lines = [
                    json!({
                        "type": "message_start",
                        "message": {
                            "id": "msg_compact",
                            "type": "message",
                            "role": "assistant",
                            "content": [],
                            "model": "claude-opus-4-6",
                            "stop_reason": null,
                            "stop_sequence": null,
                            "usage": {
                                "input_tokens": 0,
                                "output_tokens": 0
                            }
                        }
                    }),
                    json!({
                        "type": "content_block_start",
                        "index": 0,
                        "content_block": {
                            "type": "compaction",
                            "content": null,
                            "encrypted_content": null
                        }
                    }),
                    json!({
                        "type": "content_block_delta",
                        "index": 0,
                        "delta": {
                            "type": "compaction_delta",
                            "content": "Summary of the conversation.",
                            "encrypted_content": "opaque-state"
                        }
                    }),
                    json!({
                        "type": "content_block_stop",
                        "index": 0
                    }),
                    json!({
                        "type": "message_delta",
                        "delta": {
                            "stop_reason": "compaction",
                            "stop_sequence": null
                        },
                        "usage": {
                            "input_tokens": 0,
                            "output_tokens": 0,
                            "iterations": [{
                                "type": "compaction",
                                "input_tokens": 60_000,
                                "output_tokens": 1_000
                            }]
                        }
                    }),
                    json!({"type": "message_stop"}),
                ]
                .into_iter()
                .map(|line| format!("data: {line}"))
                .collect::<Vec<_>>()
                .join("\n");

                Ok(http_client::Response::builder()
                    .status(200)
                    .body(AsyncBody::from(format!("{response_lines}\n")))?)
            }
        });
        let provider = direct_anthropic_test_provider(http_client, cx);
        let store_key = cx.update(|cx| provider.set_api_key(Some("test-key".to_string()), cx));
        store_key.await.unwrap();
        let model = direct_anthropic_test_model(&provider);
        let request = LanguageModelRequest {
            messages: vec![LanguageModelRequestMessage {
                role: language_model::Role::User,
                content: vec![MessageContent::Text("Retain this context.".to_string())],
                cache: false,
                reasoning_details: None,
            }],
            ..Default::default()
        };

        let result = model.compact(request, &cx.to_async()).await.unwrap();

        assert_eq!(
            result.usage,
            language_model::TokenUsage {
                input_tokens: 60_000,
                output_tokens: 1_000,
                ..Default::default()
            }
        );
        let language_model::CompactedContext::Summary {
            content,
            provider_state,
        } = result.context
        else {
            panic!("expected summary compaction");
        };
        assert_eq!(content.as_ref(), "Summary of the conversation.");
        assert_eq!(
            anthropic::completion::provider_compaction_encrypted_content(
                &provider_state.expect("expected opaque provider state"),
                &ANTHROPIC_PROVIDER_ID,
            )
            .unwrap()
            .as_deref(),
            Some("opaque-state")
        );

        let (beta_header, body) = captured_request.lock().unwrap().take().unwrap();
        assert!(
            beta_header
                .as_deref()
                .is_some_and(|header| header.contains(anthropic::COMPACTION_BETA_HEADER))
        );
        let body = serde_json::from_str::<serde_json::Value>(&body).unwrap();
        assert_eq!(
            body["context_management"],
            json!({
                "edits": [{
                    "type": "compact_20260112",
                    "trigger": {
                        "type": "input_tokens",
                        "value": anthropic::MIN_COMPACTION_TRIGGER_TOKENS
                    },
                    "pause_after_compaction": true
                }]
            })
        );
        assert!(body["tools"].is_null());
    }

    fn direct_anthropic_test_provider(
        http_client: Arc<dyn HttpClient>,
        cx: &mut gpui::TestAppContext,
    ) -> AnthropicLanguageModelProvider {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            AnthropicLanguageModelProvider::new(http_client, Arc::new(TestCredentialsProvider), cx)
        })
    }

    fn direct_anthropic_test_model(
        provider: &AnthropicLanguageModelProvider,
    ) -> Arc<dyn LanguageModel> {
        provider.create_language_model(anthropic::Model::from_listed(anthropic::ListModelEntry {
            id: "claude-opus-4-6".to_string(),
            display_name: "Claude Opus 4.6".to_string(),
            max_input_tokens: 1_000_000,
            max_tokens: 128_000,
            capabilities: None,
        }))
    }

    struct TestCredentialsProvider;

    impl CredentialsProvider for TestCredentialsProvider {
        fn read_credentials<'a>(
            &'a self,
            _url: &'a str,
            _cx: &'a AsyncApp,
        ) -> std::pin::Pin<
            Box<dyn std::future::Future<Output = Result<Option<(String, Vec<u8>)>>> + 'a>,
        > {
            Box::pin(async { Ok(None) })
        }

        fn write_credentials<'a>(
            &'a self,
            _url: &'a str,
            _username: &'a str,
            _password: &'a [u8],
            _cx: &'a AsyncApp,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + 'a>> {
            Box::pin(async { Ok(()) })
        }

        fn delete_credentials<'a>(
            &'a self,
            _url: &'a str,
            _cx: &'a AsyncApp,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + 'a>> {
            Box::pin(async { Ok(()) })
        }
    }
}

pub struct AnthropicModel {
    id: LanguageModelId,
    model: anthropic::Model,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl AnthropicModel {
    fn stream_completion(
        &self,
        request: anthropic::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            BoxStream<'static, Result<anthropic::Event, AnthropicError>>,
            LanguageModelCompletionError,
        >,
    > {
        let http_client = self.http_client.clone();

        let (api_key, api_url, extra_headers) = self.state.read_with(cx, |state, cx| {
            let api_url = AnthropicLanguageModelProvider::api_url(cx);
            let extra_headers = AnthropicLanguageModelProvider::settings(cx)
                .custom_headers
                .clone();
            (state.api_key_state.key(&api_url), api_url, extra_headers)
        });

        let beta_headers = self.model.beta_headers();

        async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey {
                    provider: PROVIDER_NAME,
                });
            };
            let request = anthropic::stream_completion(
                http_client.as_ref(),
                &api_url,
                &api_key,
                request,
                beta_headers,
                &extra_headers,
            );
            request.await.map_err(Into::into)
        }
        .boxed()
    }
}

impl LanguageModel for AnthropicModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(self.model.display_name.clone())
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn supports_tools(&self) -> bool {
        true
    }

    fn supports_images(&self) -> bool {
        self.model.supports_images
    }

    fn supports_streaming_tools(&self) -> bool {
        true
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto
            | LanguageModelToolChoice::Any
            | LanguageModelToolChoice::None => true,
        }
    }

    fn supports_thinking(&self) -> bool {
        self.model.supports_thinking
    }

    fn supports_fast_mode(&self) -> bool {
        self.model.supports_speed
    }

    fn refusal_fallback_model_id(&self) -> Option<&'static str> {
        if self.model.id.starts_with(anthropic::FABLE_MODEL_ID_PREFIX) {
            Some(anthropic::FABLE_FALLBACK_MODEL_ID)
        } else {
            None
        }
    }

    fn supports_server_side_compaction(&self) -> bool {
        self.model.supports_compaction
    }

    fn supports_explicit_compaction(&self) -> bool {
        self.model.supports_compaction
    }

    fn minimum_explicit_compaction_input_tokens(&self) -> Option<u64> {
        self.supports_explicit_compaction()
            .then_some(anthropic::MIN_COMPACTION_TRIGGER_TOKENS)
    }

    fn compact(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<CompactionResult, LanguageModelCompletionError>> {
        if !self.supports_explicit_compaction() {
            return async {
                Err(LanguageModelCompletionError::Other(anyhow::anyhow!(
                    "this Anthropic model does not support explicit compaction"
                )))
            }
            .boxed();
        }

        let mut request = match into_anthropic(
            request,
            self.model.request_id(false).to_string(),
            self.model.default_temperature,
            self.model.max_output_tokens,
            self.model.mode.clone(),
            AnthropicPromptCacheMode::Automatic,
            &PROVIDER_ID,
        ) {
            Ok(request) => request.into_compact_request(),
            Err(error) => return async move { Err(error.into()) }.boxed(),
        };
        if !self.model.supports_speed {
            request.speed = None;
        }
        let request = self.stream_completion(request, cx);
        let future = self.request_limiter.run(async move {
            let response = request.await?;
            let stream = AnthropicEventMapper::new(PROVIDER_NAME, PROVIDER_ID).map_stream(response);
            let (context, usage) = collect_compaction_result(stream.boxed(), PROVIDER_NAME).await?;
            Ok(CompactionResult { context, usage })
        });
        future.boxed()
    }

    fn supported_effort_levels(&self) -> Vec<language_model::LanguageModelEffortLevel> {
        self.model
            .supported_effort_levels
            .iter()
            .map(|e| {
                let is_default = matches!(e, anthropic::Effort::High);
                let (name, value) = match e {
                    anthropic::Effort::Low => ("Low".into(), "low".into()),
                    anthropic::Effort::Medium => ("Medium".into(), "medium".into()),
                    anthropic::Effort::High => ("High".into(), "high".into()),
                    anthropic::Effort::XHigh => ("XHigh".into(), "xhigh".into()),
                    anthropic::Effort::Max => ("Max".into(), "max".into()),
                };
                language_model::LanguageModelEffortLevel {
                    name,
                    value,
                    is_default,
                }
            })
            .collect::<Vec<_>>()
    }

    fn telemetry_id(&self) -> String {
        format!("anthropic/{}", self.model.id)
    }

    fn api_key(&self, cx: &App) -> Option<String> {
        self.state.read_with(cx, |state, cx| {
            let api_url = AnthropicLanguageModelProvider::api_url(cx);
            state.api_key_state.key(&api_url).map(|key| key.to_string())
        })
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_input_tokens
    }

    fn max_output_tokens(&self) -> Option<u64> {
        Some(self.model.max_output_tokens)
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            BoxStream<'static, Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>,
            LanguageModelCompletionError,
        >,
    > {
        let has_tools = !request.tools.is_empty();
        let request_id = self.model.request_id(has_tools).to_string();
        let mut request = match into_anthropic(
            request,
            request_id,
            self.model.default_temperature,
            self.model.max_output_tokens,
            self.model.mode.clone(),
            AnthropicPromptCacheMode::Automatic,
            &PROVIDER_ID,
        ) {
            Ok(request) => request,
            Err(error) => return async move { Err(error.into()) }.boxed(),
        };
        if !self.model.supports_speed {
            request.speed = None;
        }
        let request = self.stream_completion(request, cx);
        let future = self.request_limiter.stream(async move {
            let response = request.await?;
            Ok(AnthropicEventMapper::new(PROVIDER_NAME, PROVIDER_ID).map_stream(response))
        });
        async move { Ok(future.await?.boxed()) }.boxed()
    }
}
