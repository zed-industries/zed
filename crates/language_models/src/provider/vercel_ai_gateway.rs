use anyhow::Result;
use collections::BTreeMap;
use credentials_provider::CredentialsProvider;
use futures::{AsyncReadExt, FutureExt, StreamExt, future::BoxFuture};
use gpui::{App, AppContext, AsyncApp, Context, Entity, SharedString, Task};
use http_client::{
    AsyncBody, CustomHeaders, HttpClient, Method, Request as HttpRequest, RequestBuilderExt, http,
};
use language_model::{
    ApiKeyConfiguration, ApiKeyState, AuthenticateError, EnvVar, IconOrSvg, LanguageModel,
    LanguageModelCompletionError, LanguageModelCompletionEvent, LanguageModelId, LanguageModelName,
    LanguageModelProvider, LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelProviderState, LanguageModelRequest, LanguageModelToolChoice,
    LanguageModelToolSchemaFormat, ProviderSettingsView, RateLimiter, env_var,
};
use open_ai::ResponseStreamEvent;
use serde::Deserialize;
pub use settings::OpenAiCompatibleModelCapabilities as ModelCapabilities;
pub use settings::VercelAiGatewayAvailableModel as AvailableModel;
use settings::{Settings, SettingsStore};
use std::sync::{Arc, LazyLock};
use ui::IconName;

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("vercel_ai_gateway");
const PROVIDER_NAME: LanguageModelProviderName =
    LanguageModelProviderName::new("Vercel AI Gateway");

const API_URL: &str = "https://ai-gateway.vercel.sh/v1";
const API_KEY_ENV_VAR_NAME: &str = "VERCEL_AI_GATEWAY_API_KEY";
static API_KEY_ENV_VAR: LazyLock<EnvVar> = env_var!(API_KEY_ENV_VAR_NAME);

#[derive(Default, Clone, Debug, PartialEq)]
pub struct VercelAiGatewaySettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
    pub custom_headers: CustomHeaders,
}

pub struct VercelAiGatewayLanguageModelProvider {
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
        let api_url = VercelAiGatewayLanguageModelProvider::api_url(cx);
        self.api_key_state.store(
            api_url,
            api_key,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        )
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = VercelAiGatewayLanguageModelProvider::api_url(cx);
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
        let api_url = VercelAiGatewayLanguageModelProvider::api_url(cx);
        let api_key = self.api_key_state.key(&api_url);
        let extra_headers = VercelAiGatewayLanguageModelProvider::settings(cx)
            .custom_headers
            .clone();
        cx.spawn(async move |this, cx| {
            let models = list_models(
                http_client.as_ref(),
                &api_url,
                api_key.as_deref(),
                &extra_headers,
            )
            .await?;
            this.update(cx, |this, cx| {
                this.available_models = models;
                cx.notify();
            })
            .map_err(|e| LanguageModelCompletionError::Other(e))?;
            Ok(())
        })
    }

    fn restart_fetch_models_task(&mut self, cx: &mut Context<Self>) {
        if self.is_authenticated() {
            let task = self.fetch_models(cx);
            self.fetch_models_task.replace(task);
        } else {
            self.available_models = Vec::new();
        }
    }
}

impl VercelAiGatewayLanguageModelProvider {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut App,
    ) -> Self {
        let state = cx.new(|cx| {
            cx.observe_global::<SettingsStore>({
                let mut last_settings = VercelAiGatewayLanguageModelProvider::settings(cx).clone();
                move |this: &mut State, cx| {
                    let current_settings = VercelAiGatewayLanguageModelProvider::settings(cx);
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

    fn settings(cx: &App) -> &VercelAiGatewaySettings {
        &crate::AllLanguageModelSettings::get_global(cx).vercel_ai_gateway
    }

    fn api_url(cx: &App) -> SharedString {
        let api_url = &Self::settings(cx).api_url;
        if api_url.is_empty() {
            API_URL.into()
        } else {
            SharedString::new(api_url.as_str())
        }
    }

    fn default_available_model() -> AvailableModel {
        AvailableModel {
            name: "openai/gpt-5.3-codex".to_string(),
            display_name: Some("GPT 5.3 Codex".to_string()),
            max_tokens: 400_000,
            max_output_tokens: Some(128_000),
            max_completion_tokens: None,
            capabilities: ModelCapabilities::default(),
        }
    }

    fn create_language_model(&self, model: AvailableModel) -> Arc<dyn LanguageModel> {
        Arc::new(VercelAiGatewayLanguageModel {
            id: LanguageModelId::from(model.name.clone()),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }
}

impl LanguageModelProviderState for VercelAiGatewayLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for VercelAiGatewayLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiVercel)
    }

    fn default_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(Self::default_available_model()))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        None
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let mut models = BTreeMap::default();

        let default_model = Self::default_available_model();
        models.insert(default_model.name.clone(), default_model);

        for model in self.state.read(cx).available_models.clone() {
            models.insert(model.name.clone(), model);
        }

        for model in &Self::settings(cx).available_models {
            models.insert(model.name.clone(), model.clone());
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
            "https://vercel.com/d?to=%2F%5Bteam%5D%2F%7E%2Fai%2Fapi-keys&title=Go+to+AI+Gateway"
                .into(),
        )))
    }

    fn set_api_key(&self, api_key: Option<String>, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.set_api_key(api_key, cx))
    }
}

pub struct VercelAiGatewayLanguageModel {
    id: LanguageModelId,
    model: AvailableModel,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl VercelAiGatewayLanguageModel {
    fn stream_open_ai(
        &self,
        request: open_ai::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            futures::stream::BoxStream<'static, Result<ResponseStreamEvent>>,
            LanguageModelCompletionError,
        >,
    > {
        let http_client = self.http_client.clone();
        let (api_key, api_url, extra_headers) = self.state.read_with(cx, |state, cx| {
            let api_url = VercelAiGatewayLanguageModelProvider::api_url(cx);
            let extra_headers = VercelAiGatewayLanguageModelProvider::settings(cx)
                .custom_headers
                .clone();
            (state.api_key_state.key(&api_url), api_url, extra_headers)
        });

        let future = self.request_limiter.stream(async move {
            let provider = PROVIDER_NAME;
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey { provider });
            };
            let request = open_ai::stream_completion(
                http_client.as_ref(),
                provider.0.as_str(),
                &api_url,
                &api_key,
                request,
                &extra_headers,
            );
            let response = request.await.map_err(map_open_ai_error)?;
            Ok(response)
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

fn map_open_ai_error(error: open_ai::RequestError) -> LanguageModelCompletionError {
    match error {
        open_ai::RequestError::HttpResponseError {
            status_code,
            body,
            headers,
            ..
        } => {
            let retry_after = headers
                .get(http::header::RETRY_AFTER)
                .and_then(|value| value.to_str().ok()?.parse::<u64>().ok())
                .map(std::time::Duration::from_secs);

            LanguageModelCompletionError::from_http_status(
                PROVIDER_NAME,
                status_code,
                extract_error_message(&body),
                retry_after,
            )
        }
        open_ai::RequestError::Other(error) => LanguageModelCompletionError::Other(error),
    }
}

fn extract_error_message(body: &str) -> String {
    let json = match serde_json::from_str::<serde_json::Value>(body) {
        Ok(json) => json,
        Err(_) => return body.to_string(),
    };

    let message = json
        .get("error")
        .and_then(|value| {
            value
                .get("message")
                .and_then(serde_json::Value::as_str)
                .or_else(|| value.as_str())
        })
        .or_else(|| json.get("message").and_then(serde_json::Value::as_str))
        .map(ToString::to_string)
        .unwrap_or_else(|| body.to_string());

    clean_error_message(&message)
}

fn clean_error_message(message: &str) -> String {
    let lower = message.to_lowercase();

    if lower.contains("vercel_oidc_token") && lower.contains("oidc token") {
        return "Authentication failed for Vercel AI Gateway. Use a Vercel AI Gateway key (vck_...).\nCreate or manage keys in Vercel AI Gateway console.\nIf this persists, regenerate the key and update it in Vercel AI Gateway provider settings in Zed.".to_string();
    }

    if lower.contains("invalid api key") || lower.contains("invalid_api_key") {
        return "Authentication failed for Vercel AI Gateway. Check that your Vercel AI Gateway key starts with vck_ and is active.".to_string();
    }

    message.to_string()
}

fn has_tag(tags: &[String], expected: &str) -> bool {
    tags.iter()
        .any(|tag| tag.trim().eq_ignore_ascii_case(expected))
}

impl LanguageModel for VercelAiGatewayLanguageModel {
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
            LanguageModelToolChoice::Auto => self.model.capabilities.tools,
            LanguageModelToolChoice::Any => self.model.capabilities.tools,
            LanguageModelToolChoice::None => true,
        }
    }

    fn supports_streaming_tools(&self) -> bool {
        true
    }

    fn supports_split_token_display(&self) -> bool {
        true
    }

    fn telemetry_id(&self) -> String {
        format!("vercel_ai_gateway/{}", self.model.name)
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_tokens
    }

    fn max_output_tokens(&self) -> Option<u64> {
        self.model.max_output_tokens
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
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
        let request = match crate::provider::open_ai::into_open_ai(
            request,
            &self.model.name,
            self.model.capabilities.parallel_tool_calls,
            self.model.capabilities.prompt_cache_key,
            self.max_output_tokens(),
            crate::provider::open_ai::ChatCompletionMaxTokensParameter::MaxCompletionTokens,
            None,
            false,
        ) {
            Ok(request) => request,
            Err(error) => return async move { Err(error.into()) }.boxed(),
        };
        let completions = self.stream_open_ai(request, cx);
        async move {
            let mapper = crate::provider::open_ai::OpenAiEventMapper::new();
            Ok(mapper.map_stream(completions.await?).boxed())
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
    name: Option<String>,
    context_window: Option<u64>,
    max_tokens: Option<u64>,
    #[serde(default)]
    r#type: Option<String>,
    #[serde(default)]
    supported_parameters: Vec<String>,
    #[serde(default)]
    tags: Vec<String>,
    architecture: Option<ApiModelArchitecture>,
}

#[derive(Deserialize)]
struct ApiModelArchitecture {
    #[serde(default)]
    input_modalities: Vec<String>,
}

async fn list_models(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: Option<&str>,
    extra_headers: &CustomHeaders,
) -> Result<Vec<AvailableModel>, LanguageModelCompletionError> {
    let uri = format!("{api_url}/models?include_mappings=true");
    let mut request_builder = HttpRequest::builder()
        .method(Method::GET)
        .uri(uri)
        .header("Accept", "application/json");
    if let Some(api_key) = api_key {
        request_builder = request_builder.header("Authorization", format!("Bearer {}", api_key));
    }
    let request = request_builder
        .extra_headers(extra_headers)
        .body(AsyncBody::default())
        .map_err(|error| LanguageModelCompletionError::BuildRequestBody {
            provider: PROVIDER_NAME,
            error,
        })?;
    let mut response =
        client
            .send(request)
            .await
            .map_err(|error| LanguageModelCompletionError::HttpSend {
                provider: PROVIDER_NAME,
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
            extract_error_message(&body),
            None,
        ));
    }

    let response: ModelsResponse = serde_json::from_str(&body).map_err(|error| {
        LanguageModelCompletionError::DeserializeResponse {
            provider: PROVIDER_NAME,
            error,
        }
    })?;

    let mut models = Vec::new();
    for model in response.data {
        if let Some(model_type) = model.r#type.as_deref()
            && model_type != "language"
        {
            continue;
        }
        let supports_tools = model
            .supported_parameters
            .iter()
            .any(|parameter| parameter == "tools")
            || has_tag(&model.tags, "tool-use")
            || has_tag(&model.tags, "tools");
        let supports_images = model.architecture.is_some_and(|architecture| {
            architecture
                .input_modalities
                .iter()
                .any(|modality| modality == "image")
        }) || has_tag(&model.tags, "vision")
            || has_tag(&model.tags, "image-input");
        let parallel_tool_calls = model
            .supported_parameters
            .iter()
            .any(|parameter| parameter == "parallel_tool_calls");
        let prompt_cache_key = model
            .supported_parameters
            .iter()
            .any(|parameter| parameter == "prompt_cache_key" || parameter == "cache_control");
        models.push(AvailableModel {
            name: model.id.clone(),
            display_name: model.name.or(Some(model.id)),
            max_tokens: model.context_window.or(model.max_tokens).unwrap_or(128_000),
            max_output_tokens: model.max_tokens,
            max_completion_tokens: None,
            capabilities: ModelCapabilities {
                tools: supports_tools,
                images: supports_images,
                parallel_tool_calls,
                prompt_cache_key,
                chat_completions: true,
                interleaved_reasoning: false,
                max_tokens_parameter: false,
            },
        });
    }

    Ok(models)
}
