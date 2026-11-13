use anyhow::Result;
use credentials_provider::CredentialsProvider;
use futures::{FutureExt, StreamExt, future::BoxFuture, stream::BoxStream};
use google_ai::completion::{GoogleEventMapper, into_google};
use google_ai::{GenerateContentResponse, GoogleModelMode, ThinkingLevel};
use gpui::{App, AppContext as _, AsyncApp, Entity, Task};
use http_client::{CustomHeaders, HttpClient};
use language_model::{
    AuthenticateError, IconOrSvg, LanguageModel, LanguageModelClient, LanguageModelCompletionError,
    LanguageModelCompletionStream, LanguageModelEffortLevel, LanguageModelId, LanguageModelName,
    LanguageModelProvider, LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelProviderState, LanguageModelRequest, LanguageModelToolChoiceSupport,
    ModelRateLimiters, ProviderSettingsView, SubPageProviderSettings, unavailable_error,
};
use settings::Settings;
use std::sync::Arc;
use ui::IconName;

use crate::provider::api_compatible::{
    ApiCompatibleProviderConfigurationView, ApiCompatibleProviderSettings,
    ApiCompatibleProviderState,
};

pub use settings::GoogleCompatibleAvailableModel as AvailableModel;
pub use settings::GoogleCompatibleModelCapabilities as ModelCapabilities;

const API_KEY_PLACEHOLDER: &str = "AIzaSyXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";

#[derive(Default, Clone, Debug, PartialEq)]
pub struct GoogleCompatibleSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
    pub custom_headers: CustomHeaders,
}

impl ApiCompatibleProviderSettings for GoogleCompatibleSettings {
    fn api_url(&self) -> &str {
        &self.api_url
    }
}

pub type State = ApiCompatibleProviderState<GoogleCompatibleSettings>;

pub struct GoogleCompatibleLanguageModelProvider {
    id: LanguageModelProviderId,
    name: LanguageModelProviderName,
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
    request_limiters: ModelRateLimiters,
}

fn available_model_to_google_model(available: &AvailableModel) -> google_ai::Model {
    let (mode, supported_thinking_levels, default_thinking_level) =
        match available.mode.unwrap_or_default() {
            settings::ModelMode::Default => (GoogleModelMode::Default, Vec::new(), None),
            settings::ModelMode::Thinking { budget_tokens } => (
                GoogleModelMode::Thinking { budget_tokens },
                Vec::new(),
                None,
            ),
            settings::ModelMode::Adaptive => {
                let (levels, default_level) = google_ai::supported_thinking_levels(&available.name);
                (
                    GoogleModelMode::Thinking {
                        budget_tokens: None,
                    },
                    levels,
                    Some(default_level),
                )
            }
        };

    google_ai::Model {
        id: available.name.clone(),
        display_name: available
            .display_name
            .clone()
            .unwrap_or_else(|| available.name.clone()),
        max_input_tokens: available.max_tokens,
        max_output_tokens: available.max_output_tokens,
        mode,
        supported_thinking_levels,
        default_thinking_level,
    }
}

fn with_provider_name(
    error: LanguageModelCompletionError,
    provider: LanguageModelProviderName,
) -> LanguageModelCompletionError {
    match error {
        LanguageModelCompletionError::NoApiKey { .. } => {
            LanguageModelCompletionError::NoApiKey { provider }
        }
        LanguageModelCompletionError::ProviderRejection {
            status,
            code,
            message,
            retry_after,
            category,
            ..
        } => LanguageModelCompletionError::ProviderRejection {
            provider,
            status,
            code,
            message,
            retry_after,
            category,
        },
        LanguageModelCompletionError::ApiReadResponseError { error, .. } => {
            LanguageModelCompletionError::ApiReadResponseError { provider, error }
        }
        LanguageModelCompletionError::SerializeRequest { error, .. } => {
            LanguageModelCompletionError::SerializeRequest { provider, error }
        }
        LanguageModelCompletionError::BuildRequestBody { error, .. } => {
            LanguageModelCompletionError::BuildRequestBody { provider, error }
        }
        LanguageModelCompletionError::HttpSend { host, error, .. } => {
            LanguageModelCompletionError::HttpSend {
                provider,
                host,
                error,
            }
        }
        LanguageModelCompletionError::DeserializeResponse { error, .. } => {
            LanguageModelCompletionError::DeserializeResponse { provider, error }
        }
        LanguageModelCompletionError::StreamEndedUnexpectedly { .. } => {
            LanguageModelCompletionError::StreamEndedUnexpectedly { provider }
        }
        LanguageModelCompletionError::ModelUnavailable { model, .. } => {
            LanguageModelCompletionError::ModelUnavailable { provider, model }
        }
        other @ (LanguageModelCompletionError::DataRetentionConsentRequired { .. }
        | LanguageModelCompletionError::Other(_)) => other,
    }
}

impl GoogleCompatibleLanguageModelProvider {
    pub fn new(
        id: Arc<str>,
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut App,
    ) -> Self {
        let state = State::new(
            id.clone(),
            credentials_provider,
            |id, cx| {
                crate::AllLanguageModelSettings::get_global(cx)
                    .google_compatible
                    .get(id)
            },
            cx,
        );

        Self {
            id: id.clone().into(),
            name: id.into(),
            http_client,
            state,
            request_limiters: ModelRateLimiters::default(),
        }
    }

    fn available_models<'a>(&self, cx: &'a App) -> &'a [AvailableModel] {
        &self.state.read(cx).settings.available_models
    }

    fn create_language_model(&self, available: &AvailableModel) -> LanguageModel {
        let capabilities = &available.capabilities;
        let model = available_model_to_google_model(available);
        LanguageModel {
            supports_tools: capabilities.tools,
            supports_images: capabilities.images,
            supports_thinking: model.supports_thinking(),
            supports_disabling_thinking: model.supported_thinking_levels.is_empty()
                && model.supports_disabling_thinking(),
            supported_effort_levels: model
                .supported_thinking_levels
                .iter()
                .map(|level| LanguageModelEffortLevel {
                    name: level.name().into(),
                    value: level.value().into(),
                    is_default: Some(*level) == model.default_thinking_level,
                    supports_disabling_thinking: None,
                })
                .collect(),
            tool_choice_support: LanguageModelToolChoiceSupport {
                auto: capabilities.tools,
                any: capabilities.tools,
                none: true,
            },
            max_output_tokens: model.max_output_tokens,
            ..LanguageModel::new(
                LanguageModelId::from(model.id.clone()),
                LanguageModelName::from(model.display_name.clone()),
                self.id.clone(),
                self.name.clone(),
                format!("google/{}", model.id),
                model.max_input_tokens,
            )
        }
    }

    fn config(
        &self,
        model: &LanguageModel,
        cx: &App,
    ) -> Result<google_ai::Model, LanguageModelCompletionError> {
        self.available_models(cx)
            .iter()
            .find(|available| available.name == model.id.0.as_ref())
            .map(available_model_to_google_model)
            .ok_or_else(|| unavailable_error(model))
    }

    fn stream_google_request(
        &self,
        request: google_ai::GenerateContentRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            BoxStream<'static, Result<GenerateContentResponse, LanguageModelCompletionError>>,
            LanguageModelCompletionError,
        >,
    > {
        let http_client = self.http_client.clone();
        let provider_name = self.name.clone();

        let (api_key, api_url, extra_headers) = self.state.read_with(cx, |state, _cx| {
            let api_url = state.settings.api_url.clone();
            (
                state.api_key_state.key(&api_url),
                api_url,
                state.settings.custom_headers.clone(),
            )
        });

        async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey {
                    provider: provider_name,
                });
            };
            let trimmed_api_url = api_url.trim().trim_end_matches('/');
            let stream = google_ai::stream_generate_content(
                http_client.as_ref(),
                trimmed_api_url,
                &api_key,
                request,
                &extra_headers,
            )
            .await
            .map_err(|error| with_provider_name(error, provider_name))?;
            Ok(stream)
        }
        .boxed()
    }
}

impl LanguageModelProviderState for GoogleCompatibleLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for GoogleCompatibleLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelProviderName {
        self.name.clone()
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiGoogleCompat)
    }

    fn default_model(&self, cx: &App) -> Option<LanguageModel> {
        self.available_models(cx)
            .first()
            .map(|model| self.create_language_model(model))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<LanguageModel> {
        None
    }

    fn provided_models(&self, cx: &App) -> Vec<LanguageModel> {
        self.available_models(cx)
            .iter()
            .map(|model| self.create_language_model(model))
            .collect()
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated()
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        self.state.update(cx, |state, cx| state.authenticate(cx))
    }

    fn settings_view(&self, _cx: &mut App) -> Option<ProviderSettingsView> {
        let state = self.state.clone();
        Some(ProviderSettingsView::SubPage(SubPageProviderSettings::new(
            move |window, cx| {
                cx.new(|cx| {
                    ApiCompatibleProviderConfigurationView::new(
                        state.clone(),
                        "Google",
                        API_KEY_PLACEHOLDER,
                        window,
                        cx,
                    )
                })
                .into()
            },
        )))
    }

    fn set_api_key(&self, api_key: Option<String>, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.set_api_key(api_key, cx))
    }
}

impl LanguageModelClient for GoogleCompatibleLanguageModelProvider {
    fn stream_completion(
        &self,
        model: &LanguageModel,
        mut request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<LanguageModelCompletionStream, LanguageModelCompletionError>>
    {
        let config = match cx.update(|cx| self.config(model, cx)) {
            Ok(config) => config,
            Err(error) => return async move { Err(error) }.boxed(),
        };
        let request_limiter = self.request_limiters.for_model(&model.id);
        if request.max_output_tokens.is_some() {
            request.max_output_tokens =
                request.effective_max_output_tokens(config.max_output_tokens);
        }
        let effort_supported = request
            .thinking_effort
            .as_deref()
            .and_then(ThinkingLevel::from_effort)
            .is_some_and(|level| config.supported_thinking_levels.contains(&level));
        if !effort_supported {
            request.thinking_effort = None;
        }
        // Custom and prefixed model IDs aren't in google_ai's hardcoded table, so
        // avoid falling back to thinking_level when budget-based thinking is off.
        let mode = if !request.thinking_allowed
            && config.supported_thinking_levels.is_empty()
            && !config.supports_disabling_thinking()
        {
            GoogleModelMode::Default
        } else {
            config.mode
        };
        let request = match into_google(request, config.id, mode) {
            Ok(request) => request,
            Err(error) => return async move { Err(error.into()) }.boxed(),
        };
        let provider_name = self.name.clone();
        let request = self.stream_google_request(request, cx);
        let future = request_limiter.stream(async move {
            let response = request.await?;
            Ok(GoogleEventMapper::new()
                .map_stream(response)
                .map(move |event| {
                    event.map_err(|error| with_provider_name(error, provider_name.clone()))
                }))
        });
        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use credentials_provider::test_support::FakeCredentialsProvider;
    use futures::AsyncReadExt as _;
    use gpui::{BorrowAppContext as _, TestAppContext};
    use http_client::{AsyncBody, FakeHttpClient, Response, StatusCode};
    use language_model::{
        LanguageModelCompletionEvent, LanguageModelRequestMessage, MessageContent, Role,
    };
    use serde_json::Value;
    use settings::SettingsStore;
    use std::sync::Mutex;

    fn parse_available_model(json: &str) -> AvailableModel {
        serde_json::from_str(json).expect("test fixture should parse")
    }

    #[gpui::test]
    fn test_available_model_modes_and_default_capabilities(cx: &mut TestAppContext) {
        let http_client = FakeHttpClient::with_200_response();
        let credentials_provider = Arc::new(FakeCredentialsProvider::default());
        let provider = cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            GoogleCompatibleLanguageModelProvider::new(
                Arc::from("test-google"),
                http_client,
                credentials_provider,
                cx,
            )
        });

        let default_case = parse_available_model(
            r#"{
                "name": "gemini-2.5-pro",
                "display_name": "Gemini 2.5 Pro",
                "max_tokens": 1000000,
                "mode": { "type": "default" }
            }"#,
        );
        assert!(default_case.capabilities.tools);
        assert!(default_case.capabilities.images);
        let default_model = available_model_to_google_model(&default_case);
        assert_eq!(default_model.mode, GoogleModelMode::Default);
        assert!(default_model.supported_thinking_levels.is_empty());
        let default_language_model = provider.create_language_model(&default_case);
        assert!(!default_language_model.supports_thinking());
        assert!(default_language_model.supported_effort_levels().is_empty());

        let thinking_case = parse_available_model(
            r#"{
                "name": "gemini-2.5-flash",
                "max_tokens": 1000000,
                "max_output_tokens": 65536,
                "mode": { "type": "thinking", "budget_tokens": 4096 },
                "capabilities": { "tools": false }
            }"#,
        );
        assert!(!thinking_case.capabilities.tools);
        assert!(thinking_case.capabilities.images);
        let thinking_model = available_model_to_google_model(&thinking_case);
        assert_eq!(
            thinking_model.mode,
            GoogleModelMode::Thinking {
                budget_tokens: Some(4096)
            }
        );
        assert_eq!(thinking_model.max_output_tokens, Some(65536));
        assert!(thinking_model.supported_thinking_levels.is_empty());
        let thinking_language_model = provider.create_language_model(&thinking_case);
        assert!(thinking_language_model.supports_thinking());
        assert!(thinking_language_model.supports_disabling_thinking());
        assert!(thinking_language_model.supported_effort_levels().is_empty());

        let flash_adaptive_case = parse_available_model(
            r#"{
                "name": "gemini-3-flash-preview",
                "max_tokens": 1000000,
                "mode": { "type": "adaptive" }
            }"#,
        );
        let flash_adaptive_model = available_model_to_google_model(&flash_adaptive_case);
        assert_eq!(
            flash_adaptive_model.mode,
            GoogleModelMode::Thinking {
                budget_tokens: None
            }
        );
        assert_eq!(
            flash_adaptive_model.supported_thinking_levels,
            vec![
                ThinkingLevel::Minimal,
                ThinkingLevel::Low,
                ThinkingLevel::Medium,
                ThinkingLevel::High,
            ]
        );
        assert_eq!(
            flash_adaptive_model.default_thinking_level,
            Some(ThinkingLevel::High)
        );
        let flash_adaptive_language_model = provider.create_language_model(&flash_adaptive_case);
        assert!(flash_adaptive_language_model.supports_thinking());
        assert!(!flash_adaptive_language_model.supports_disabling_thinking());
        assert_eq!(
            flash_adaptive_language_model
                .default_effort_level()
                .map(|level| level.value.to_string()),
            Some("high".to_string())
        );

        let pro_adaptive_case = parse_available_model(
            r#"{
                "name": "gemini-3.1-pro-preview",
                "max_tokens": 1000000,
                "mode": { "type": "adaptive" }
            }"#,
        );
        let pro_adaptive_model = available_model_to_google_model(&pro_adaptive_case);
        assert_eq!(
            pro_adaptive_model.supported_thinking_levels,
            vec![
                ThinkingLevel::Low,
                ThinkingLevel::Medium,
                ThinkingLevel::High,
            ]
        );
        assert_eq!(
            pro_adaptive_model.default_thinking_level,
            Some(ThinkingLevel::High)
        );

        let custom_adaptive_case = parse_available_model(
            r#"{
                "name": "custom-gemini",
                "max_tokens": 1000000,
                "mode": { "type": "adaptive" }
            }"#,
        );
        let custom_adaptive_model = available_model_to_google_model(&custom_adaptive_case);
        assert_eq!(
            custom_adaptive_model.supported_thinking_levels,
            vec![
                ThinkingLevel::Low,
                ThinkingLevel::Medium,
                ThinkingLevel::High,
            ]
        );
        assert_eq!(
            custom_adaptive_model.default_thinking_level,
            Some(ThinkingLevel::Medium)
        );
    }

    #[gpui::test]
    async fn test_stream_completion_uses_custom_endpoint_and_provider_attribution(
        cx: &mut TestAppContext,
    ) {
        let captured_requests: Arc<Mutex<Vec<(String, Option<String>, Value)>>> =
            Arc::new(Mutex::new(Vec::new()));
        let return_error = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let http_client = FakeHttpClient::create({
            let captured_requests = captured_requests.clone();
            let return_error = return_error.clone();
            move |request| {
                let uri = request.uri().to_string();
                let header = request
                    .headers()
                    .get("x-custom-test")
                    .and_then(|value| value.to_str().ok())
                    .map(str::to_owned);
                let mut body = request.into_body();
                let captured_requests = captured_requests.clone();
                let should_error = return_error.load(std::sync::atomic::Ordering::SeqCst);
                async move {
                    let mut body_text = String::new();
                    body.read_to_string(&mut body_text).await?;
                    let body_json: Value = serde_json::from_str(&body_text)?;
                    captured_requests
                        .lock()
                        .unwrap()
                        .push((uri, header, body_json));
                    if should_error {
                        Ok(Response::builder()
                            .status(StatusCode::UNAUTHORIZED)
                            .body(AsyncBody::from(
                                r#"{"error":{"code":401,"message":"bad key","status":"UNAUTHENTICATED"}}"#,
                            ))?)
                    } else {
                        Ok(Response::builder().status(StatusCode::OK).body(AsyncBody::from(
                            "data: {\"candidates\":[{\"content\":{\"role\":\"model\",\"parts\":[{\"text\":\"Hello!\"}]},\"finishReason\":\"STOP\"}]}\n\n",
                        ))?)
                    }
                }
            }
        });

        let credentials_provider = Arc::new(FakeCredentialsProvider::default());
        let provider = Arc::new(cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store
                    .set_user_settings(
                        r#"{
                            "language_models": {
                                "google_compatible": {
                                    "my-google-provider": {
                                        "api_url": "https://gateway.example.com/",
                                        "custom_headers": { "x-custom-test": "abc" },
                                        "available_models": [
                                            {
                                                "name": "gemini-3.1-pro-preview",
                                                "display_name": "Gemini 3.1 Pro",
                                                "max_tokens": 1000000,
                                                "max_output_tokens": 32000,
                                                "mode": { "type": "adaptive" }
                                            },
                                            {
                                                "name": "google/gemini-2.5-flash",
                                                "max_tokens": 1000000,
                                                "mode": { "type": "thinking", "budget_tokens": 4096 }
                                            }
                                        ]
                                    }
                                }
                            }
                        }"#,
                        cx,
                    )
                    .expect("test settings should parse");
            });
            GoogleCompatibleLanguageModelProvider::new(
                Arc::from("my-google-provider"),
                http_client,
                credentials_provider,
                cx,
            )
        }));

        cx.update(|cx| provider.set_api_key(Some("secret-key".to_string()), cx))
            .await
            .expect("api key should save");

        let model = cx
            .update(|cx| provider.default_model(cx))
            .expect("model should exist");
        assert!(model.supports_thinking());
        assert_eq!(
            model
                .supported_effort_levels()
                .into_iter()
                .map(|level| level.value.to_string())
                .collect::<Vec<_>>(),
            vec!["low", "medium", "high"]
        );

        let request = LanguageModelRequest {
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text("Hi".to_string())],
                cache: false,
                reasoning_details: None,
            }],
            thinking_allowed: true,
            thinking_effort: Some("medium".to_string()),
            ..Default::default()
        };

        let mut stream = provider
            .stream_completion(&model, request.clone(), &cx.to_async())
            .await
            .expect("stream should succeed");
        let mut events = Vec::new();
        while let Some(event) = stream.next().await {
            events.push(event);
        }
        assert!(matches!(
            events.first(),
            Some(Ok(LanguageModelCompletionEvent::Text(text))) if text == "Hello!"
        ));

        {
            let recorded = captured_requests.lock().unwrap();
            assert_eq!(recorded.len(), 1);
            assert_eq!(
                recorded[0].0,
                "https://gateway.example.com/v1beta/models/gemini-3.1-pro-preview:streamGenerateContent?alt=sse&key=secret-key"
            );
            assert_eq!(recorded[0].1.as_deref(), Some("abc"));
            assert_eq!(
                recorded[0].2["generationConfig"]["thinkingConfig"],
                serde_json::json!({
                    "includeThoughts": true,
                    "thinkingLevel": "MEDIUM"
                })
            );
            assert!(
                recorded[0].2["generationConfig"]
                    .get("maxOutputTokens")
                    .is_none()
            );
        }

        let budget_model = cx
            .update(|cx| provider.provided_models(cx))
            .into_iter()
            .find(|model| model.id().0.as_ref() == "google/gemini-2.5-flash")
            .expect("budget model should exist");
        let budget_off_request = LanguageModelRequest {
            messages: request.messages.clone(),
            thinking_allowed: false,
            ..Default::default()
        };
        let mut budget_stream = provider
            .stream_completion(&budget_model, budget_off_request, &cx.to_async())
            .await
            .expect("budget stream should succeed");
        while budget_stream.next().await.is_some() {}
        {
            let recorded = captured_requests.lock().unwrap();
            assert_eq!(recorded.len(), 2);
            assert!(
                recorded[1].2["generationConfig"]
                    .get("thinkingConfig")
                    .is_none()
            );
        }

        return_error.store(true, std::sync::atomic::Ordering::SeqCst);
        let error = provider
            .stream_completion(&model, request, &cx.to_async())
            .await
            .err()
            .expect("request should fail");
        assert!(matches!(
            error,
            LanguageModelCompletionError::ProviderRejection {
                provider,
                status: Some(StatusCode::UNAUTHORIZED),
                ..
            } if provider.0.as_ref() == "my-google-provider"
        ));
    }
}
