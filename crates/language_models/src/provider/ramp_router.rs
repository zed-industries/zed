use anyhow::Result;
use collections::BTreeMap;
use credentials_provider::CredentialsProvider;
use futures::{AsyncReadExt, FutureExt, StreamExt, future::BoxFuture};
use gpui::{App, AppContext, AsyncApp, Context, Entity, SharedString, Task};
use http_client::{
    AsyncBody, CustomHeaders, HttpClient, Method, Request as HttpRequest, RequestBuilderExt,
};
use language_model::{
    ApiKeyConfiguration, ApiKeyState, AuthenticateError, EnvVar, IconOrSvg, LanguageModel,
    LanguageModelCompletionError, LanguageModelCompletionEvent, LanguageModelEffortLevel,
    LanguageModelId, LanguageModelName, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest,
    LanguageModelToolChoice, LanguageModelToolSchemaFormat, ProviderSettingsView, RateLimiter,
    env_var,
};
use open_ai::responses::{
    Request as ResponseRequest, StreamEvent as ResponsesStreamEvent, stream_response,
};
use serde::Deserialize;
use settings::{
    OpenAiCompatibleAvailableModel as AvailableModel,
    OpenAiCompatibleModelCapabilities as ModelCapabilities, Settings, SettingsStore,
};
use std::sync::{Arc, LazyLock};
use ui::IconName;

use crate::provider::open_ai::{OpenAiResponseEventMapper, into_open_ai_response};

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("ramp_router");
const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("Ramp Router");
const API_URL: &str = "https://api.router.com/v1";
const API_KEY_ENV_VAR_NAME: &str = "RAMP_ROUTER_API_KEY";
static API_KEY_ENV_VAR: LazyLock<EnvVar> = env_var!(API_KEY_ENV_VAR_NAME);

#[derive(Default, Clone, Debug, PartialEq)]
pub struct RampRouterSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
    pub custom_headers: CustomHeaders,
}

pub struct RampRouterLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

pub struct State {
    api_key_state: ApiKeyState,
    credentials_provider: Arc<dyn CredentialsProvider>,
    http_client: Arc<dyn HttpClient>,
    available_models: Vec<AvailableModel>,
    fetch_models_task: Option<Task<Result<(), LanguageModelCompletionError>>>,
}

impl State {
    fn is_authenticated(&self) -> bool {
        self.api_key_state.has_key()
    }

    fn set_api_key(&mut self, api_key: Option<String>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = RampRouterLanguageModelProvider::api_url(cx);
        let task = self.api_key_state.store(
            api_url,
            api_key,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        );

        cx.spawn(async move |this, cx| {
            task.await?;
            this.update(cx, |this, cx| this.restart_fetch_models_task(cx))?;
            Ok(())
        })
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = RampRouterLanguageModelProvider::api_url(cx);
        let task = self.api_key_state.load_if_needed(
            api_url,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        );

        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| this.restart_fetch_models_task(cx))
                .ok();
            result
        })
    }

    fn fetch_models(
        &mut self,
        cx: &mut Context<Self>,
    ) -> Task<Result<(), LanguageModelCompletionError>> {
        let http_client = self.http_client.clone();
        let api_url = RampRouterLanguageModelProvider::api_url(cx);
        let extra_headers = RampRouterLanguageModelProvider::settings(cx)
            .custom_headers
            .clone();
        let Some(api_key) = self.api_key_state.key(&api_url) else {
            return Task::ready(Err(LanguageModelCompletionError::NoApiKey {
                provider: PROVIDER_NAME,
            }));
        };

        cx.spawn(async move |this, cx| {
            let models =
                list_models(http_client.as_ref(), &api_url, &api_key, &extra_headers).await?;
            this.update(cx, |this, cx| {
                this.available_models = models;
                cx.notify();
            })
            .map_err(LanguageModelCompletionError::Other)?;
            Ok(())
        })
    }

    fn restart_fetch_models_task(&mut self, cx: &mut Context<Self>) {
        if self.is_authenticated() {
            self.fetch_models_task = Some(self.fetch_models(cx));
        } else {
            self.fetch_models_task = None;
            self.available_models.clear();
            cx.notify();
        }
    }
}

impl RampRouterLanguageModelProvider {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut App,
    ) -> Self {
        let state = cx.new(|cx| {
            cx.observe_global::<SettingsStore>({
                let mut last_settings = Self::settings(cx).clone();
                move |this: &mut State, cx| {
                    let current_settings = Self::settings(cx);
                    if current_settings != &last_settings {
                        last_settings = current_settings.clone();
                        this.authenticate(cx).detach();
                        cx.notify();
                    }
                }
            })
            .detach();
            State {
                api_key_state: ApiKeyState::new(Self::api_url(cx), (*API_KEY_ENV_VAR).clone()),
                credentials_provider,
                http_client: http_client.clone(),
                available_models: Vec::new(),
                fetch_models_task: None,
            }
        });

        Self { http_client, state }
    }

    fn settings(cx: &App) -> &RampRouterSettings {
        &crate::AllLanguageModelSettings::get_global(cx).ramp_router
    }

    fn api_url(cx: &App) -> SharedString {
        let api_url = &Self::settings(cx).api_url;
        if api_url.is_empty() {
            API_URL.into()
        } else {
            SharedString::new(api_url.as_str())
        }
    }

    fn normalized_model(mut model: AvailableModel) -> AvailableModel {
        model.capabilities.chat_completions = false;
        model
    }

    fn available_models(&self, cx: &App) -> BTreeMap<String, AvailableModel> {
        let mut models = BTreeMap::default();

        for model in self.state.read(cx).available_models.iter().cloned() {
            models.insert(model.name.clone(), Self::normalized_model(model));
        }
        for model in Self::settings(cx).available_models.iter().cloned() {
            models.insert(model.name.clone(), Self::normalized_model(model));
        }

        models
    }

    fn create_language_model(&self, model: AvailableModel) -> Arc<dyn LanguageModel> {
        Arc::new(RampRouterLanguageModel {
            id: LanguageModelId::from(model.name.clone()),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }
}

impl LanguageModelProviderState for RampRouterLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for RampRouterLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiRampRouter)
    }

    fn default_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        self.available_models(cx)
            .into_values()
            .next()
            .map(|model| self.create_language_model(model))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        None
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        self.available_models(cx)
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
            "https://app.router.com".into(),
        )))
    }

    fn set_api_key(&self, api_key: Option<String>, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.set_api_key(api_key, cx))
    }
}

pub struct RampRouterLanguageModel {
    id: LanguageModelId,
    model: AvailableModel,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl RampRouterLanguageModel {
    fn stream_response(
        &self,
        request: ResponseRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<futures::stream::BoxStream<'static, Result<ResponsesStreamEvent>>>>
    {
        let http_client = self.http_client.clone();
        let (api_key, api_url, extra_headers) = self.state.read_with(cx, |state, cx| {
            let api_url = RampRouterLanguageModelProvider::api_url(cx);
            let extra_headers = RampRouterLanguageModelProvider::settings(cx)
                .custom_headers
                .clone();
            (state.api_key_state.key(&api_url), api_url, extra_headers)
        });

        let future = self.request_limiter.stream(async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey {
                    provider: PROVIDER_NAME,
                });
            };
            let response = stream_response(
                http_client.as_ref(),
                PROVIDER_NAME.0.as_str(),
                &api_url,
                &api_key,
                request,
                &extra_headers,
            )
            .await?;
            Ok(response)
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

fn default_reasoning_effort(model: &AvailableModel) -> Option<open_ai::ReasoningEffort> {
    model
        .reasoning_effort
        .filter(|effort| *effort != open_ai::ReasoningEffort::None)
}

fn supported_reasoning_effort_levels(model: &AvailableModel) -> Vec<LanguageModelEffortLevel> {
    let Some(default_effort) = default_reasoning_effort(model) else {
        return Vec::new();
    };

    open_ai::ReasoningEffort::OPENAI_COMPATIBLE_SELECTABLE
        .into_iter()
        .map(|effort| LanguageModelEffortLevel {
            name: effort.label().into(),
            value: effort.value().into(),
            is_default: effort == default_effort,
        })
        .collect()
}

fn supports_none_reasoning_effort(model: &AvailableModel) -> bool {
    model.reasoning_effort.is_some()
}

fn normalize_request_reasoning(request: &mut LanguageModelRequest, model: &AvailableModel) {
    if model.reasoning_effort == Some(open_ai::ReasoningEffort::None) {
        request.thinking_allowed = false;
        request.thinking_effort = None;
    }
}

impl LanguageModel for RampRouterLanguageModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(
            self.model
                .display_name
                .clone()
                .unwrap_or_else(|| self.model.name.clone()),
        )
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn supports_tools(&self) -> bool {
        self.model.capabilities.tools
    }

    fn tool_input_format(&self) -> LanguageModelToolSchemaFormat {
        LanguageModelToolSchemaFormat::JsonSchemaSubset
    }

    fn supports_images(&self) -> bool {
        self.model.capabilities.images
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto | LanguageModelToolChoice::Any => {
                self.model.capabilities.tools
            }
            LanguageModelToolChoice::None => true,
        }
    }

    fn supports_streaming_tools(&self) -> bool {
        true
    }

    fn supports_thinking(&self) -> bool {
        default_reasoning_effort(&self.model).is_some()
    }

    fn supported_effort_levels(&self) -> Vec<LanguageModelEffortLevel> {
        supported_reasoning_effort_levels(&self.model)
    }

    fn supports_split_token_display(&self) -> bool {
        true
    }

    fn telemetry_id(&self) -> String {
        format!("ramp_router/{}", self.model.name)
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_tokens
    }

    fn max_output_tokens(&self) -> Option<u64> {
        self.model.max_output_tokens
    }

    fn stream_completion(
        &self,
        mut request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            futures::stream::BoxStream<
                'static,
                Result<LanguageModelCompletionEvent, LanguageModelCompletionError>,
            >,
            LanguageModelCompletionError,
        >,
    > {
        request.speed = None;
        normalize_request_reasoning(&mut request, &self.model);
        let request = match into_open_ai_response(
            request,
            &self.model.name,
            self.model.capabilities.parallel_tool_calls,
            self.model.capabilities.prompt_cache_key,
            self.max_output_tokens(),
            default_reasoning_effort(&self.model),
            supports_none_reasoning_effort(&self.model),
            &PROVIDER_ID,
        ) {
            Ok(request) => request,
            Err(error) => return async move { Err(error.into()) }.boxed(),
        };
        let responses = self.stream_response(request, cx);
        async move {
            let mapper = OpenAiResponseEventMapper::new(PROVIDER_ID);
            Ok(mapper.map_stream(responses.await?).boxed())
        }
        .boxed()
    }
}

#[derive(Deserialize)]
struct ModelsResponse {
    data: Vec<ApiModel>,
}

#[derive(Deserialize)]
struct ApiModel {
    id: String,
}

async fn list_models(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    extra_headers: &CustomHeaders,
) -> Result<Vec<AvailableModel>, LanguageModelCompletionError> {
    let request = HttpRequest::builder()
        .method(Method::GET)
        .uri(format!("{api_url}/models"))
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", api_key.trim()))
        .extra_headers(extra_headers)
        .body(AsyncBody::default())
        .map_err(|error| LanguageModelCompletionError::BuildRequestBody {
            provider: PROVIDER_NAME,
            error,
        })?;
    let host = request.uri().host().unwrap_or(api_url).to_owned();
    let mut response =
        client
            .send(request)
            .await
            .map_err(|error| LanguageModelCompletionError::HttpSend {
                provider: PROVIDER_NAME,
                host,
                error,
            })?;

    let mut body = String::new();
    response
        .body_mut()
        .read_to_string(&mut body)
        .await
        .map_err(|error| LanguageModelCompletionError::ApiReadResponseError {
            provider: PROVIDER_NAME,
            error,
        })?;

    if !response.status().is_success() {
        return Err(LanguageModelCompletionError::from_http_status(
            PROVIDER_NAME,
            response.status(),
            body,
            None,
        ));
    }

    let response: ModelsResponse = serde_json::from_str(&body).map_err(|error| {
        LanguageModelCompletionError::DeserializeResponse {
            provider: PROVIDER_NAME,
            error,
        }
    })?;

    Ok(response
        .data
        .into_iter()
        .map(|model| AvailableModel {
            display_name: Some(model.id.clone()),
            name: model.id,
            max_tokens: 128_000,
            max_output_tokens: None,
            max_completion_tokens: None,
            reasoning_effort: None,
            capabilities: ModelCapabilities {
                tools: false,
                images: false,
                parallel_tool_calls: false,
                prompt_cache_key: false,
                chat_completions: false,
                interleaved_reasoning: false,
                max_tokens_parameter: false,
            },
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;
    use http_client::Response;
    use serde_json::{Value, json};
    use std::sync::Mutex;

    #[test]
    fn list_models_uses_router_endpoint_and_conservative_defaults() {
        let captured_request = Arc::new(Mutex::new(None));
        let captured_request_for_handler = captured_request.clone();
        let client = http_client::FakeHttpClient::create(move |request| {
            let captured_request = captured_request_for_handler.clone();
            async move {
                let method = request.method().clone();
                let uri = request.uri().to_string();
                let authorization = request
                    .headers()
                    .get("Authorization")
                    .and_then(|value| value.to_str().ok())
                    .map(str::to_string);
                captured_request
                    .lock()
                    .expect("captured request lock")
                    .replace((method, uri, authorization));
                Ok(Response::builder().status(200).body(AsyncBody::from(
                    json!({
                        "object": "list",
                        "data": [{
                            "id": "router-model",
                            "object": "model",
                            "owned_by": "router"
                        }]
                    })
                    .to_string(),
                ))?)
            }
        });

        let models = block_on(list_models(
            client.as_ref(),
            API_URL,
            "  secret  ",
            &CustomHeaders::default(),
        ))
        .expect("model list request");

        let (method, uri, authorization) = captured_request
            .lock()
            .expect("captured request lock")
            .take()
            .expect("captured request");
        assert_eq!(method, Method::GET);
        assert_eq!(uri, "https://api.router.com/v1/models");
        assert_eq!(authorization.as_deref(), Some("Bearer secret"));
        assert_eq!(models.len(), 1);
        assert_eq!(models[0].name, "router-model");
        assert_eq!(models[0].max_tokens, 128_000);
        assert!(!models[0].capabilities.tools);
        assert!(!models[0].capabilities.images);
        assert!(!models[0].capabilities.chat_completions);
    }

    #[test]
    fn configured_models_always_use_responses_api() {
        let model = AvailableModel {
            name: "router-model".to_string(),
            display_name: None,
            max_tokens: 64_000,
            max_output_tokens: None,
            max_completion_tokens: None,
            reasoning_effort: None,
            capabilities: ModelCapabilities {
                chat_completions: true,
                ..Default::default()
            },
        };

        let model = RampRouterLanguageModelProvider::normalized_model(model);
        assert!(!model.capabilities.chat_completions);
    }

    #[test]
    fn responses_transport_appends_endpoint_once() {
        let captured_request = Arc::new(Mutex::new(None));
        let captured_request_for_handler = captured_request.clone();
        let client = http_client::FakeHttpClient::create(move |request| {
            let captured_request = captured_request_for_handler.clone();
            async move {
                let method = request.method().clone();
                let uri = request.uri().to_string();
                let authorization = request
                    .headers()
                    .get("Authorization")
                    .and_then(|value| value.to_str().ok())
                    .map(str::to_string);
                let content_type = request
                    .headers()
                    .get("Content-Type")
                    .and_then(|value| value.to_str().ok())
                    .map(str::to_string);
                let mut body = request.into_body();
                let mut body_text = String::new();
                body.read_to_string(&mut body_text).await?;
                captured_request
                    .lock()
                    .expect("captured request lock")
                    .replace((method, uri, authorization, content_type, body_text));
                Ok(Response::builder()
                    .status(200)
                    .body(AsyncBody::from("data: [DONE]\n\n"))?)
            }
        });
        let request = ResponseRequest {
            model: "router-model".to_string(),
            instructions: None,
            input: Default::default(),
            include: Vec::new(),
            stream: true,
            temperature: None,
            top_p: None,
            max_output_tokens: None,
            parallel_tool_calls: None,
            tool_choice: None,
            tools: Vec::new(),
            prompt_cache_key: None,
            reasoning: None,
            store: Some(false),
            service_tier: None,
            context_management: None,
        };

        block_on(async {
            stream_response(
                client.as_ref(),
                PROVIDER_NAME.0.as_str(),
                API_URL,
                "  secret  ",
                request,
                &CustomHeaders::default(),
            )
            .await
            .expect("responses request")
            .collect::<Vec<_>>()
            .await;
        });

        let (method, uri, authorization, content_type, body) = captured_request
            .lock()
            .expect("captured request lock")
            .take()
            .expect("captured request");
        assert_eq!(method, Method::POST);
        assert_eq!(uri, "https://api.router.com/v1/responses");
        assert_eq!(authorization.as_deref(), Some("Bearer secret"));
        assert_eq!(content_type.as_deref(), Some("application/json"));
        let body: Value = serde_json::from_str(&body).expect("JSON request body");
        assert_eq!(body["model"], "router-model");
        assert_eq!(body["stream"], true);
    }
}
