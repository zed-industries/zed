use anyhow::Result;
use collections::BTreeMap;
use credentials_provider::CredentialsProvider;
use futures::{AsyncReadExt, FutureExt, StreamExt, future::BoxFuture};
use gpui::{App, AppContext, AsyncApp, Context, Entity, SharedString, Task};
use http_client::{
    AsyncBody, CustomHeaders, HttpClient, Method, Request as HttpRequest, RequestBuilderExt, http,
};
use language_model::chat_completion::{ChatCompletionEventMapper, ResponseStreamEvent};
use language_model::{
    ApiKeyConfiguration, ApiKeyState, AuthenticateError, EnvVar, IconOrSvg, LanguageModel,
    LanguageModelClient, LanguageModelCompletionError, LanguageModelCompletionStream,
    LanguageModelId, LanguageModelName, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest,
    LanguageModelToolChoiceSupport, ModelRateLimiters, ProviderSettingsView, RateLimiter, env_var,
    unavailable_error,
};
use serde::Deserialize;
pub use settings::GmiCloudAvailableModel as AvailableModel;
pub use settings::OpenAiCompatibleModelCapabilities as ModelCapabilities;
use settings::{Settings, SettingsStore};
use std::sync::{Arc, LazyLock};
use ui::IconName;
use util::ResultExt;

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("gmi_cloud");
const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("GMI Cloud");

const API_URL: &str = "https://api.gmi-serving.com/v1";
const API_KEY_ENV_VAR_NAME: &str = "GMI_API_KEY";
static API_KEY_ENV_VAR: LazyLock<EnvVar> = env_var!(API_KEY_ENV_VAR_NAME);

const DEFAULT_MODEL: &str = "deepseek-ai/DeepSeek-V4.1-Flash";
const DEFAULT_CONTEXT_WINDOW: u64 = 128_000;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct GmiCloudSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
    pub custom_headers: CustomHeaders,
}

pub struct GmiCloudLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
    request_limiters: ModelRateLimiters,
}

pub struct State {
    api_key_state: ApiKeyState,
    credentials_provider: Arc<dyn CredentialsProvider>,
    http_client: Arc<dyn HttpClient>,
    available_models: Vec<AvailableModel>,
    fetch_models_task: Option<Task<()>>,
    last_fetch_models_error: Option<String>,
}

impl State {
    fn is_authenticated(&self) -> bool {
        self.api_key_state.has_key()
    }

    fn set_api_key(&mut self, api_key: Option<String>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = GmiCloudLanguageModelProvider::api_url(cx);
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
        let api_url = GmiCloudLanguageModelProvider::api_url(cx);
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
        let api_url = GmiCloudLanguageModelProvider::api_url(cx);
        let api_key = self.api_key_state.key(&api_url);
        let extra_headers = GmiCloudLanguageModelProvider::settings(cx)
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
            .map_err(LanguageModelCompletionError::Other)?;
            Ok(())
        })
    }

    fn restart_fetch_models_task(&mut self, cx: &mut Context<Self>) {
        if self.is_authenticated() {
            let fetch_task = self.fetch_models(cx);
            let task = cx.spawn(async move |this, cx| {
                let result = fetch_task.await;
                this.update(cx, |this, _cx| {
                    this.last_fetch_models_error =
                        result.as_ref().err().map(|error| format!("{error:#}"));
                })
                .ok();
                result.log_err();
            });
            self.fetch_models_task.replace(task);
        } else {
            self.available_models = Vec::new();
            self.last_fetch_models_error = None;
        }
    }
}

impl GmiCloudLanguageModelProvider {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut App,
    ) -> Self {
        let state = cx.new(|cx| {
            cx.observe_global::<SettingsStore>({
                let mut last_settings = GmiCloudLanguageModelProvider::settings(cx).clone();
                move |this: &mut State, cx| {
                    let current_settings = GmiCloudLanguageModelProvider::settings(cx);
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
                last_fetch_models_error: None,
            }
        });

        Self {
            http_client,
            state,
            request_limiters: ModelRateLimiters::default(),
        }
    }

    fn settings(cx: &App) -> &GmiCloudSettings {
        &crate::AllLanguageModelSettings::get_global(cx).gmi_cloud
    }

    fn api_url(cx: &App) -> SharedString {
        let api_url = &Self::settings(cx).api_url;
        if api_url.is_empty() {
            API_URL.into()
        } else {
            SharedString::new(api_url.as_str())
        }
    }

    /// Every model this provider offers, keyed by id: the fetched list, then
    /// settings entries, each overriding the last.
    fn gmi_cloud_models(&self, cx: &App) -> BTreeMap<String, AvailableModel> {
        let mut models = BTreeMap::default();

        for model in self.state.read(cx).available_models.clone() {
            models.insert(model.name.clone(), model);
        }

        for model in &Self::settings(cx).available_models {
            models.insert(model.name.clone(), model.clone());
        }

        models
    }

    /// The current configuration of `model`, if this provider still offers it.
    fn config(
        &self,
        model: &LanguageModel,
        cx: &App,
    ) -> Result<AvailableModel, LanguageModelCompletionError> {
        self.gmi_cloud_models(cx)
            .remove(model.id.0.as_ref())
            .ok_or_else(|| unavailable_error(model))
    }

    fn stream_open_ai(
        &self,
        request_limiter: &RateLimiter,
        request: open_ai::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            futures::stream::BoxStream<
                'static,
                Result<ResponseStreamEvent, LanguageModelCompletionError>,
            >,
            LanguageModelCompletionError,
        >,
    > {
        let http_client = self.http_client.clone();
        let (api_key, api_url, extra_headers) = self.state.read_with(cx, |state, cx| {
            let api_url = GmiCloudLanguageModelProvider::api_url(cx);
            let extra_headers = GmiCloudLanguageModelProvider::settings(cx)
                .custom_headers
                .clone();
            (state.api_key_state.key(&api_url), api_url, extra_headers)
        });

        let future = request_limiter.stream(async move {
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

impl LanguageModelProviderState for GmiCloudLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for GmiCloudLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiGmiCloud)
    }

    fn default_model(&self, cx: &App) -> Option<LanguageModel> {
        let models = self.gmi_cloud_models(cx);
        // The served model list changes over time, so fall back to any
        // offered model rather than pointing at one that was retired.
        models
            .get(DEFAULT_MODEL)
            .or_else(|| models.values().next())
            .map(language_model)
    }

    fn default_fast_model(&self, _cx: &App) -> Option<LanguageModel> {
        None
    }

    fn provided_models(&self, cx: &App) -> Vec<LanguageModel> {
        self.gmi_cloud_models(cx)
            .values()
            .map(language_model)
            .collect()
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated()
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        self.state.update(cx, |state, cx| state.authenticate(cx))
    }

    fn refresh_models(&self, cx: &mut App) {
        self.state
            .update(cx, |state, cx| state.restart_fetch_models_task(cx));
    }

    fn settings_view(&self, cx: &mut App) -> Option<ProviderSettingsView> {
        let state = self.state.read(cx);
        Some(ProviderSettingsView::ApiKey(ApiKeyConfiguration::new(
            state.api_key_state.has_key(),
            state.api_key_state.is_from_env_var(),
            state.api_key_state.env_var_name().clone(),
            "https://console.gmicloud.ai".into(),
        )))
    }

    fn set_api_key(&self, api_key: Option<String>, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.set_api_key(api_key, cx))
    }
}

impl LanguageModelClient for GmiCloudLanguageModelProvider {
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
        // `speed` can leak in from a parent thread's model; GMI Cloud has no
        // fast mode and would reject the resulting `service_tier`.
        request.speed = None;
        let max_tokens_parameter = if config.capabilities.max_tokens_parameter {
            crate::provider::open_ai::ChatCompletionMaxTokensParameter::MaxTokens
        } else {
            crate::provider::open_ai::ChatCompletionMaxTokensParameter::MaxCompletionTokens
        };
        let request = match crate::provider::open_ai::into_open_ai(
            request,
            &config.name,
            config.capabilities.parallel_tool_calls,
            config.capabilities.prompt_cache_key,
            config.max_output_tokens,
            max_tokens_parameter,
            None,
            config.capabilities.interleaved_reasoning,
        ) {
            Ok(request) => request,
            Err(error) => return async move { Err(error.into()) }.boxed(),
        };
        let completions = self.stream_open_ai(&request_limiter, request, cx);
        let executor = cx.background_executor().clone();
        async move {
            let mapper = ChatCompletionEventMapper::new();
            Ok(language_model::stream_in_background(
                mapper.map_stream(completions.await?).boxed(),
                executor,
            ))
        }
        .boxed()
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
        error => error.into(),
    }
}

/// GMI Cloud returns errors both as OpenAI-style `{"error": {"message": ..}}`
/// and as a flat `{"message": ..}` (e.g. for a missing API key).
fn extract_error_message(body: &str) -> String {
    let json = match serde_json::from_str::<serde_json::Value>(body) {
        Ok(json) => json,
        Err(_) => return body.to_string(),
    };

    json.get("error")
        .and_then(|value| {
            value
                .get("message")
                .and_then(serde_json::Value::as_str)
                .or_else(|| value.as_str())
        })
        .or_else(|| json.get("message").and_then(serde_json::Value::as_str))
        .map(ToString::to_string)
        .unwrap_or_else(|| body.to_string())
}

fn language_model(model: &AvailableModel) -> LanguageModel {
    LanguageModel {
        supports_tools: model.capabilities.tools,
        supports_images: model.capabilities.images,
        tool_choice_support: LanguageModelToolChoiceSupport {
            auto: model.capabilities.tools,
            any: model.capabilities.tools,
            none: true,
        },
        supports_streaming_tools: true,
        supports_split_token_display: true,
        max_output_tokens: model.max_output_tokens,
        ..LanguageModel::new(
            LanguageModelId::from(model.name.clone()),
            LanguageModelName::from(
                model
                    .display_name
                    .clone()
                    .unwrap_or_else(|| model.name.clone()),
            ),
            PROVIDER_ID,
            PROVIDER_NAME,
            format!("gmi_cloud/{}", model.name),
            model.max_tokens,
        )
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
    context_length: Option<u64>,
}

async fn list_models(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: Option<&str>,
    extra_headers: &CustomHeaders,
) -> Result<Vec<AvailableModel>, LanguageModelCompletionError> {
    let uri = format!("{api_url}/models");
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
            extract_error_message(&body),
            None,
        ));
    }

    parse_models_response(&body)
}

fn parse_models_response(body: &str) -> Result<Vec<AvailableModel>, LanguageModelCompletionError> {
    let response: ModelsResponse = serde_json::from_str(body).map_err(|error| {
        LanguageModelCompletionError::DeserializeResponse {
            provider: PROVIDER_NAME,
            error,
        }
    })?;

    Ok(response
        .data
        .into_iter()
        .map(|model| AvailableModel {
            name: model.id,
            display_name: model.name.filter(|name| !name.is_empty()),
            max_tokens: model.context_length.unwrap_or(DEFAULT_CONTEXT_WINDOW),
            max_output_tokens: None,
            capabilities: ModelCapabilities::default(),
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

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

    fn init_settings(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
        });
    }

    #[test]
    fn parses_models_response() {
        let models = parse_models_response(
            r#"{
                "object": "list",
                "data": [
                    {
                        "id": "deepseek-ai/DeepSeek-V4.1-Flash",
                        "internal_id": "9aa6bb1e-170b-41ec-929d-1532858f9390",
                        "name": "DeepSeek-V4.1-Flash",
                        "object": "model",
                        "created": 1792449200,
                        "context_length": 1048575,
                        "quantization": "fp8",
                        "owned_by": "GMI Cloud",
                        "pricing": {"prompt": "0.000000300", "completion": "0.000001200"}
                    },
                    {"id": "zai-org/GLM-5.3", "name": "", "context_length": 1048576},
                    {"id": "Qwen/Qwen3.8-Flash"}
                ]
            }"#,
        )
        .unwrap();

        assert_eq!(models.len(), 3);
        assert_eq!(models[0].name, "deepseek-ai/DeepSeek-V4.1-Flash");
        assert_eq!(
            models[0].display_name.as_deref(),
            Some("DeepSeek-V4.1-Flash")
        );
        assert_eq!(models[0].max_tokens, 1_048_575);
        assert_eq!(models[1].display_name, None);
        assert_eq!(models[2].max_tokens, DEFAULT_CONTEXT_WINDOW);
        assert!(models.iter().all(|model| model.capabilities.tools));
    }

    #[gpui::test]
    async fn streams_tool_calls_with_empty_continuation_fields(cx: &mut gpui::TestAppContext) {
        init_settings(cx);

        // Recorded from the live API: after the first chunk, GMI Cloud sends
        // `id`, `type`, and `name` as empty strings rather than omitting them.
        let stream_body = [
            r#"{"choices":[{"index":0,"delta":{"role":"assistant","content":""}}],"service_tier":"default"}"#,
            r#"{"choices":[{"index":0,"delta":{"content":"","tool_calls":[{"id":"call_00_kcoaip6mhewm1hdup405h2zl","index":0,"type":"function","function":{"name":"read_file","arguments":""}}]}}],"service_tier":"default"}"#,
            r#"{"choices":[{"index":0,"delta":{"content":"","tool_calls":[{"id":"","index":0,"type":"","function":{"name":"","arguments":"{\"path\": "}}]}}],"service_tier":"default"}"#,
            r#"{"choices":[{"index":0,"delta":{"content":"","tool_calls":[{"id":"","index":0,"type":"","function":{"name":"","arguments":"\"src/main.rs\"}"}}]}}],"service_tier":"default"}"#,
            r#"{"choices":[{"index":0,"delta":{"content":""},"finish_reason":"tool_calls"}],"usage":{"prompt_tokens":296,"completion_tokens":46,"total_tokens":342},"service_tier":"default"}"#,
        ]
        .iter()
        .map(|chunk| format!("data: {chunk}\n\n"))
        .chain(["data: [DONE]\n\n".to_string()])
        .collect::<String>();

        let http_client = http_client::FakeHttpClient::create(move |_request| {
            let stream_body = stream_body.clone();
            async move {
                Ok(http_client::Response::builder()
                    .status(200)
                    .body(AsyncBody::from(stream_body))?)
            }
        });
        let provider = cx.update(|cx| {
            GmiCloudLanguageModelProvider::new(http_client, Arc::new(TestCredentialsProvider), cx)
        });
        cx.update(|cx| provider.set_api_key(Some("test-key".to_string()), cx))
            .await
            .unwrap();
        let model = parse_models_response(
            r#"{"data":[{"id":"deepseek-ai/DeepSeek-V4.1-Flash","context_length":1048575}]}"#,
        )
        .unwrap();
        provider.state.update(cx, |state, _| {
            state.available_models = model;
        });
        let model = cx
            .read(|cx| provider.default_model(cx))
            .expect("the fetched model should be the default");

        let events = provider
            .stream_completion(&model, LanguageModelRequest::default(), &cx.to_async())
            .await
            .unwrap()
            .collect::<Vec<_>>()
            .await;

        let tool_use = events
            .into_iter()
            .rev()
            .find_map(|event| match event {
                Ok(language_model::LanguageModelCompletionEvent::ToolUse(tool_use))
                    if tool_use.is_input_complete =>
                {
                    Some(tool_use)
                }
                _ => None,
            })
            .expect("the stream should produce a complete tool call");
        assert_eq!(tool_use.id.to_string(), "call_00_kcoaip6mhewm1hdup405h2zl");
        assert_eq!(tool_use.name.as_ref(), "read_file");
        assert_eq!(tool_use.raw_input, r#"{"path": "src/main.rs"}"#);
    }

    #[test]
    fn extracts_nested_and_flat_error_messages() {
        assert_eq!(
            extract_error_message(r#"{"error":{"message":"invalid key"}}"#),
            "invalid key"
        );
        assert_eq!(
            extract_error_message(r#"{"group":"api_key","message":"API key is required."}"#),
            "API key is required."
        );
        assert_eq!(extract_error_message("Bad Gateway"), "Bad Gateway");
    }

    #[gpui::test]
    async fn default_model_falls_back_to_first_fetched_model(cx: &mut gpui::TestAppContext) {
        init_settings(cx);

        let http_client = http_client::FakeHttpClient::create(|_request| async move {
            Ok(http_client::Response::builder()
                .status(200)
                .body(AsyncBody::from(
                    r#"{"data":[{"id":"zai-org/GLM-4.6"},{"id":"moonshotai/Kimi-K2-Instruct"}]}"#,
                ))?)
        });
        let provider = cx.update(|cx| {
            GmiCloudLanguageModelProvider::new(http_client, Arc::new(TestCredentialsProvider), cx)
        });

        cx.update(|cx| provider.set_api_key(Some("test-key".to_string()), cx))
            .await
            .unwrap();
        cx.update(|cx| {
            provider
                .state
                .update(cx, |state, cx| state.restart_fetch_models_task(cx))
        });
        cx.run_until_parked();

        let default_model = cx.read(|cx| provider.default_model(cx)).unwrap();
        assert_eq!(default_model.id.0.as_ref(), "moonshotai/Kimi-K2-Instruct");
        assert_eq!(cx.read(|cx| provider.provided_models(cx)).len(), 2);
    }

    #[gpui::test]
    async fn restart_fetch_models_task_records_error_on_failure(cx: &mut gpui::TestAppContext) {
        init_settings(cx);

        let http_client = http_client::FakeHttpClient::create(|_request| async move {
            Ok(http_client::Response::builder()
                .status(401)
                .body(AsyncBody::from(r#"{"error":{"message":"invalid key"}}"#))?)
        });
        let provider = cx.update(|cx| {
            GmiCloudLanguageModelProvider::new(http_client, Arc::new(TestCredentialsProvider), cx)
        });

        cx.update(|cx| provider.set_api_key(Some("test-key".to_string()), cx))
            .await
            .unwrap();
        cx.update(|cx| {
            provider
                .state
                .update(cx, |state, cx| state.restart_fetch_models_task(cx))
        });
        cx.run_until_parked();

        let error = cx.read(|cx| provider.state.read(cx).last_fetch_models_error.clone());
        assert!(
            error.is_some(),
            "a failed model fetch should be recorded, not silently dropped"
        );
    }
}
