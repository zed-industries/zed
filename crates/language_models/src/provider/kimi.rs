use anyhow::{Context as _, Result, anyhow};
use credentials_provider::CredentialsProvider;
use fs::Fs;
use futures::{
    AsyncBufReadExt, AsyncReadExt, FutureExt, StreamExt, future::BoxFuture, future::Shared,
    io::BufReader,
};
use gpui::{
    AnyView, App, AsyncApp, ClipboardItem, Context, Entity, Global, SharedString, Task, TaskExt,
    Window,
};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use language_model::{
    AuthenticateError, IconOrSvg, LanguageModel, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelEffortLevel, LanguageModelId, LanguageModelName,
    LanguageModelProvider, LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelProviderState, LanguageModelRequest, LanguageModelToolChoice, RateLimiter,
};
use open_ai::{ResponseStreamEvent, ResponseStreamResult, StreamOptions, ToolChoice};
use rand::RngCore as _;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use settings::{Settings, SettingsStore, update_settings_file};
use std::{
    path::PathBuf,
    process::Command,
    sync::{Arc, LazyLock},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use ui::{ConfiguredApiCard, ElevationIndex, prelude::*};
use ui_input::InputField;
use url::form_urlencoded;
use util::{ResultExt as _, paths::home_dir};

use crate::provider::open_ai::{OpenAiEventMapper, into_open_ai};

pub use settings::KimiAvailableModel as AvailableModel;

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("kimi");
const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("Kimi");

const KIMI_API_URL: &str = "https://api.kimi.com/coding/v1";
const KIMI_FOR_CODING_MODEL_ID: &str = "kimi-for-coding";
const DEFAULT_MAX_TOKENS: u64 = 262_144;

const KIMI_CLI_VERSION: &str = "1.46.0";
const KIMI_USER_AGENT: &str = "KimiCLI/1.46.0";
const KIMI_DEVICE_AUTHORIZATION_URL: &str = "https://auth.kimi.com/api/oauth/device_authorization";
const KIMI_TOKEN_URL: &str = "https://auth.kimi.com/api/oauth/token";
const KIMI_CLIENT_ID: &str = "17e5f671-d194-4dfb-9706-5516cb48c098";
const KIMI_DEVICE_GRANT: &str = "urn:ietf:params:oauth:grant-type:device_code";
const KIMI_REFRESH_GRANT: &str = "refresh_token";
const CREDENTIALS_KEY: &str = "https://api.kimi.com/coding/v1/oauth";
const TOKEN_REFRESH_BUFFER_MS: u64 = 60 * 1000;
const REFRESH_MAX_RETRIES: usize = 3;
const DEFAULT_MAX_OUTPUT_TOKENS: u64 = 32_000;

static RETRYABLE_REFRESH_STATUSES: LazyLock<Vec<u16>> =
    LazyLock::new(|| vec![429, 500, 502, 503, 504]);

static KIMI_HEADERS: LazyLock<Vec<(String, String)>> = LazyLock::new(|| {
    vec![
        ("User-Agent".to_string(), KIMI_USER_AGENT.to_string()),
        ("X-Msh-Platform".to_string(), "kimi_cli".to_string()),
        ("X-Msh-Version".to_string(), KIMI_CLI_VERSION.to_string()),
        (
            "X-Msh-Device-Name".to_string(),
            ascii_header_value(device_name(), "unknown"),
        ),
        (
            "X-Msh-Device-Model".to_string(),
            ascii_header_value(device_model(), "unknown"),
        ),
        (
            "X-Msh-Os-Version".to_string(),
            ascii_header_value(os_version(), "unknown"),
        ),
        ("X-Msh-Device-Id".to_string(), get_device_id()),
    ]
});

#[derive(Default, Clone, Debug, PartialEq)]
pub struct KimiSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
    pub max_output_tokens: u64,
}

#[derive(Clone)]
pub struct GlobalKimiAuth(pub Entity<State>);

impl GlobalKimiAuth {
    fn get_or_init(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut App,
    ) -> Self {
        if let Some(auth) = cx.try_global::<Self>().cloned() {
            return auth;
        }

        cx.background_spawn(async {
            LazyLock::force(&KIMI_HEADERS);
        })
        .detach();

        let state = cx.new(|cx| {
            cx.observe_global::<SettingsStore>(|state: &mut State, cx| {
                state.models = merge_models(state.discovered_models.clone(), cx);
                cx.notify();
            })
            .detach();

            State {
                credentials: None,
                discovered_models: Vec::new(),
                models: merge_models(Vec::new(), cx),
                device_authorization: None,
                sign_in_task: None,
                sign_in_generation: None,
                refresh_task: None,
                load_task: None,
                credentials_provider,
                auth_generation: 0,
                last_auth_error: None,
            }
        });

        let auth = Self(state.clone());
        cx.set_global(auth.clone());
        load_credentials(state, http_client, cx);
        auth
    }
}

impl Global for GlobalKimiAuth {}

pub struct State {
    credentials: Option<KimiCredentials>,
    discovered_models: Vec<AvailableModel>,
    models: Vec<AvailableModel>,
    device_authorization: Option<DeviceAuthorization>,
    sign_in_task: Option<Shared<Task<Result<(), Arc<anyhow::Error>>>>>,
    sign_in_generation: Option<u64>,
    refresh_task: Option<Shared<Task<Result<KimiCredentials, Arc<anyhow::Error>>>>>,
    load_task: Option<Shared<Task<Result<(), Arc<anyhow::Error>>>>>,
    credentials_provider: Arc<dyn CredentialsProvider>,
    auth_generation: u64,
    last_auth_error: Option<SharedString>,
}

impl State {
    fn is_authenticated(&self) -> bool {
        self.credentials.is_some()
    }

    fn is_signing_in(&self) -> bool {
        self.sign_in_task.is_some()
    }
}

pub struct KimiLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

impl KimiLanguageModelProvider {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut App,
    ) -> Self {
        let auth = GlobalKimiAuth::get_or_init(http_client.clone(), credentials_provider, cx);
        Self {
            http_client,
            state: auth.0,
        }
    }

    fn create_language_model(&self, model: AvailableModel) -> Arc<dyn LanguageModel> {
        Arc::new(KimiLanguageModel {
            id: LanguageModelId::from(model.name.clone()),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }

    fn settings(cx: &App) -> &KimiSettings {
        &crate::AllLanguageModelSettings::get_global(cx).kimi
    }

    fn api_url(cx: &App) -> SharedString {
        let api_url = &Self::settings(cx).api_url;
        if api_url.is_empty() {
            KIMI_API_URL.into()
        } else {
            SharedString::new(api_url.as_str())
        }
    }
}

impl LanguageModelProviderState for KimiLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for KimiLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiOpenAiCompat)
    }

    fn default_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        self.state
            .read(cx)
            .models
            .first()
            .cloned()
            .map(|model| self.create_language_model(model))
    }

    fn default_fast_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        self.default_model(cx)
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        self.state
            .read(cx)
            .models
            .iter()
            .cloned()
            .map(|model| self.create_language_model(model))
            .collect()
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated()
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        if self.is_authenticated(cx) {
            return Task::ready(Ok(()));
        }

        let state = self.state.read(cx);
        let load_task = state.load_task.clone();
        let sign_in_task = state.sign_in_task.clone();
        if load_task.is_some() || sign_in_task.is_some() {
            let weak_state = self.state.downgrade();
            cx.spawn(async move |cx| {
                if let Some(load_task) = load_task {
                    load_task
                        .await
                        .map_err(|error| anyhow!("Failed to load Kimi credentials: {error}"))?;
                }

                let sign_in_task = if let Some(sign_in_task) = sign_in_task {
                    Some(sign_in_task)
                } else {
                    weak_state
                        .read_with(&*cx, |state, _| state.sign_in_task.clone())
                        .ok()
                        .flatten()
                };
                if let Some(sign_in_task) = sign_in_task {
                    sign_in_task
                        .await
                        .map_err(|error| anyhow!("Kimi sign-in failed: {error}"))?;
                }

                let is_authenticated = weak_state
                    .read_with(&*cx, |state, _| state.is_authenticated())
                    .unwrap_or(false);
                if is_authenticated {
                    Ok(())
                } else {
                    Err(anyhow!("Sign in to Kimi Code to use Kimi in Zed.").into())
                }
            })
        } else {
            Task::ready(Err(
                anyhow!("Sign in to Kimi Code to use Kimi in Zed.").into()
            ))
        }
    }

    fn configuration_view(
        &self,
        _target_agent: language_model::ConfigurationViewTargetAgent,
        _window: &mut Window,
        cx: &mut App,
    ) -> AnyView {
        cx.new(|cx| {
            ConfigurationView::new(self.state.clone(), self.http_client.clone(), _window, cx)
        })
        .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        do_sign_out(&self.state.downgrade(), cx)
    }
}

pub struct KimiLanguageModel {
    id: LanguageModelId,
    model: AvailableModel,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl KimiLanguageModel {
    fn stream_kimi_completion(
        &self,
        request: open_ai::Request,
        thinking: KimiThinking,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            futures::stream::BoxStream<'static, Result<ResponseStreamEvent>>,
            LanguageModelCompletionError,
        >,
    > {
        let body = match kimi_request_body(request, thinking) {
            Ok(body) => body,
            Err(error) => {
                return async move { Err(LanguageModelCompletionError::from(error)) }.boxed();
            }
        };
        let http_client = self.http_client.clone();
        let state = self.state.downgrade();
        let request_limiter = self.request_limiter.clone();
        let api_url = self
            .state
            .read_with(cx, |_, cx| KimiLanguageModelProvider::api_url(cx));

        let future = cx.spawn(async move |cx| {
            let credentials = get_fresh_credentials(&state, &http_client, cx, false).await?;
            let headers = kimi_headers(cx).await;
            let mut response = request_limiter
                .stream({
                    let http_client = http_client.clone();
                    let api_url = api_url.clone();
                    let body = body.clone();
                    let access_token = credentials.access_token.clone();
                    let headers = headers.clone();
                    async move {
                        stream_kimi_completion(
                            http_client.as_ref(),
                            &api_url,
                            &access_token,
                            body,
                            &headers,
                        )
                        .await
                        .map_err(LanguageModelCompletionError::from)
                    }
                })
                .await
                .map(|stream| stream.boxed());

            if matches!(
                &response,
                Err(LanguageModelCompletionError::AuthenticationError { .. })
            ) {
                let credentials = get_fresh_credentials(&state, &http_client, cx, true).await?;
                response = request_limiter
                    .stream({
                        let http_client = http_client.clone();
                        let access_token = credentials.access_token.clone();
                        let headers = headers.clone();
                        async move {
                            stream_kimi_completion(
                                http_client.as_ref(),
                                &api_url,
                                &access_token,
                                body,
                                &headers,
                            )
                            .await
                            .map_err(LanguageModelCompletionError::from)
                        }
                    })
                    .await
                    .map(|stream| stream.boxed());
            }

            response
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

impl LanguageModel for KimiLanguageModel {
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
        true
    }

    fn supports_images(&self) -> bool {
        self.model.supports_images.unwrap_or(false)
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto => true,
            LanguageModelToolChoice::Any => true,
            LanguageModelToolChoice::None => true,
        }
    }

    fn supports_streaming_tools(&self) -> bool {
        true
    }

    fn supports_split_token_display(&self) -> bool {
        true
    }

    fn supports_thinking(&self) -> bool {
        true
    }

    fn supported_effort_levels(&self) -> Vec<LanguageModelEffortLevel> {
        Vec::new()
    }

    fn telemetry_id(&self) -> String {
        format!("kimi/{}", self.model.name)
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
        let thread_id = request.thread_id.clone();
        let thinking_enabled = request.thinking_allowed;
        let mut request = into_open_ai(
            request,
            &self.model.name,
            false,
            false,
            self.max_output_tokens(),
            None,
            true,
        );
        request.stream_options = Some(StreamOptions {
            include_usage: true,
        });
        request.tool_choice = request.tool_choice.or(Some(ToolChoice::Auto));
        let thinking = apply_kimi_fields(&mut request, thread_id, thinking_enabled);
        let completions = self.stream_kimi_completion(request, thinking, cx);
        async move {
            let mapper = OpenAiEventMapper::new();
            Ok(mapper.map_stream(completions.await?).boxed())
        }
        .boxed()
    }
}

#[derive(Clone, Copy)]
enum KimiThinking {
    Enabled,
    Disabled,
}

fn apply_kimi_fields(
    request: &mut open_ai::Request,
    thread_id: Option<String>,
    thinking_allowed: bool,
) -> KimiThinking {
    request.prompt_cache_key = thread_id;
    request.reasoning_effort = None;

    if thinking_allowed {
        KimiThinking::Enabled
    } else {
        KimiThinking::Disabled
    }
}

async fn stream_kimi_completion(
    client: &dyn HttpClient,
    api_url: &str,
    access_token: &str,
    body: String,
    headers: &[(String, String)],
) -> Result<futures::stream::BoxStream<'static, Result<ResponseStreamEvent>>, open_ai::RequestError>
{
    let uri = format!("{}/chat/completions", api_url.trim_end_matches('/'));
    let mut builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Accept", "text/event-stream")
        .header("Authorization", format!("Bearer {}", access_token.trim()));
    for (name, value) in headers {
        builder = builder.header(name, value);
    }
    let request = builder
        .body(AsyncBody::from(body))
        .map_err(|error| open_ai::RequestError::Other(error.into()))?;

    let mut response = client.send(request).await?;
    if response.status().is_success() {
        let reader = BufReader::new(response.into_body());
        Ok(reader
            .lines()
            .filter_map(|line| async move {
                match line {
                    Ok(line) => {
                        let line = line
                            .strip_prefix("data: ")
                            .or_else(|| line.strip_prefix("data:"))?;
                        if line == "[DONE]" {
                            None
                        } else {
                            match serde_json::from_str(line) {
                                Ok(ResponseStreamResult::Ok(response)) => Some(Ok(response)),
                                Ok(ResponseStreamResult::Err { error }) => {
                                    Some(Err(anyhow!("{error:?}")))
                                }
                                Err(error) => Some(Err(anyhow!(error))),
                            }
                        }
                    }
                    Err(error) => Some(Err(anyhow!(error))),
                }
            })
            .boxed())
    } else {
        let mut body = String::new();
        response
            .body_mut()
            .read_to_string(&mut body)
            .await
            .map_err(|error| open_ai::RequestError::Other(error.into()))?;

        Err(open_ai::RequestError::HttpResponseError {
            provider: PROVIDER_NAME.0.to_string(),
            status_code: response.status(),
            body,
            headers: response.headers().clone(),
        })
    }
}

fn kimi_request_body(
    request: open_ai::Request,
    thinking: KimiThinking,
) -> Result<String, open_ai::RequestError> {
    let mut value = serde_json::to_value(request)
        .map_err(|error| open_ai::RequestError::Other(error.into()))?;
    let Some(object) = value.as_object_mut() else {
        return Err(open_ai::RequestError::Other(anyhow!(
            "invalid Kimi request body"
        )));
    };

    match thinking {
        KimiThinking::Enabled => {
            object.insert(
                "thinking".to_string(),
                serde_json::json!({ "type": "enabled" }),
            );
        }
        KimiThinking::Disabled => {
            object.insert(
                "thinking".to_string(),
                serde_json::json!({ "type": "disabled" }),
            );
        }
    }

    serde_json::to_string(&value).map_err(|error| open_ai::RequestError::Other(error.into()))
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct KimiCredentials {
    access_token: String,
    refresh_token: String,
    expires_at_ms: u64,
    scope: Option<String>,
    token_type: Option<String>,
}

impl KimiCredentials {
    fn from_token_response(response: TokenResponse) -> Self {
        Self {
            access_token: response.access_token,
            refresh_token: response.refresh_token,
            expires_at_ms: now_ms() + response.expires_in.saturating_mul(1000),
            scope: response.scope,
            token_type: response.token_type,
        }
    }

    fn is_expired(&self) -> bool {
        now_ms() + TOKEN_REFRESH_BUFFER_MS >= self.expires_at_ms
    }
}

#[derive(Clone, Debug, Deserialize)]
struct DeviceAuthorization {
    device_code: String,
    user_code: String,
    verification_uri: String,
    #[serde(default)]
    verification_uri_complete: Option<String>,
    #[serde(default)]
    expires_in: Option<u64>,
    #[serde(default)]
    interval: Option<u64>,
}

impl DeviceAuthorization {
    fn verification_url(&self) -> &str {
        self.verification_uri_complete
            .as_deref()
            .unwrap_or(&self.verification_uri)
    }
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: String,
    expires_in: u64,
    #[serde(default)]
    scope: Option<String>,
    #[serde(default)]
    token_type: Option<String>,
}

#[derive(Deserialize)]
struct ModelsResponse {
    data: Vec<ModelInfo>,
}

#[derive(Deserialize)]
struct ModelInfo {
    id: String,
    #[serde(default)]
    display_name: Option<String>,
    #[serde(default)]
    supports_image_in: Option<bool>,
}

#[derive(Debug)]
enum RefreshError {
    Fatal(anyhow::Error),
    Transient(anyhow::Error),
}

impl std::fmt::Display for RefreshError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RefreshError::Fatal(error) => write!(f, "{error}"),
            RefreshError::Transient(error) => write!(f, "{error}"),
        }
    }
}

fn load_credentials(state: Entity<State>, http_client: Arc<dyn HttpClient>, cx: &mut App) {
    let weak_state = state.downgrade();
    let max_output_tokens = KimiLanguageModelProvider::settings(cx).max_output_tokens;
    let load_task = cx
        .spawn(async move |cx| {
            let credentials_provider =
                weak_state.read_with(&*cx, |state, _| state.credentials_provider.clone())?;
            let api_url = weak_state.read_with(&*cx, |_, cx| KimiLanguageModelProvider::api_url(cx))?;
            let credentials = credentials_provider
                .read_credentials(CREDENTIALS_KEY, &*cx)
                .await;

            let mut loaded_credentials = None;
            if let Ok(Some((_, bytes))) = credentials {
                match serde_json::from_slice::<KimiCredentials>(&bytes) {
                    Ok(credentials) => loaded_credentials = Some(credentials),
                    Err(error) => log::warn!("Failed to deserialize Kimi credentials: {error}"),
                }
            }

            let discovered_models = if let Some(credentials) = loaded_credentials.as_ref() {
                if credentials.is_expired() {
                    Vec::new()
                } else {
                    let headers = kimi_headers(cx).await;
                    match list_models(
                        http_client.as_ref(),
                        &api_url,
                        &credentials.access_token,
                        max_output_tokens,
                        &headers,
                    )
                    .await
                    {
                        Ok(models) => models,
                        Err(error) => {
                            log::warn!("Failed to discover Kimi models while loading credentials: {error:?}");
                            Vec::new()
                        }
                    }
                }
            } else {
                Vec::new()
            };

            weak_state.update(cx, |state, cx| {
                state.credentials = loaded_credentials;
                state.discovered_models = discovered_models;
                state.models = merge_models(state.discovered_models.clone(), cx);
                state.load_task = None;
                cx.notify();
            })?;
            Ok::<(), Arc<anyhow::Error>>(())
        })
        .shared();

    state.update(cx, |state, _| {
        state.load_task = Some(load_task);
    });
}

async fn get_fresh_credentials(
    state: &gpui::WeakEntity<State>,
    http_client: &Arc<dyn HttpClient>,
    cx: &mut AsyncApp,
    force: bool,
) -> Result<KimiCredentials, LanguageModelCompletionError> {
    let (credentials, existing_task) = state
        .read_with(&*cx, |state, _| {
            (state.credentials.clone(), state.refresh_task.clone())
        })
        .map_err(LanguageModelCompletionError::Other)?;

    let credentials = credentials.ok_or(LanguageModelCompletionError::NoApiKey {
        provider: PROVIDER_NAME,
    })?;

    if !force && !credentials.is_expired() {
        return Ok(credentials);
    }

    if let Some(shared_task) = existing_task {
        return shared_task
            .await
            .map_err(|error| LanguageModelCompletionError::Other(anyhow!("{error}")));
    }

    let http_client = http_client.clone();
    let state_clone = state.clone();
    let refresh_token_value = credentials.refresh_token.clone();
    let generation = state
        .read_with(&*cx, |state, _| state.auth_generation)
        .map_err(LanguageModelCompletionError::Other)?;
    let max_output_tokens = state
        .read_with(&*cx, |_, cx| {
            KimiLanguageModelProvider::settings(cx).max_output_tokens
        })
        .map_err(LanguageModelCompletionError::Other)?;

    let shared_task = cx
        .spawn(async move |cx| {
            let result = refresh_token(http_client.as_ref(), &refresh_token_value, cx).await;
            match result {
                Ok(refreshed) => {
                    let persist_result: Result<KimiCredentials, Arc<anyhow::Error>> = async {
                        let current_generation = state_clone
                            .read_with(&*cx, |state, _| state.auth_generation)
                            .map_err(|error| Arc::new(error))?;
                        if current_generation != generation {
                            return Err(Arc::new(anyhow!(
                                "Sign-out occurred during Kimi token refresh"
                            )));
                        }

                        let credentials_provider = state_clone
                            .read_with(&*cx, |state, _| state.credentials_provider.clone())
                            .map_err(|error| Arc::new(error))?;
                        let json = serde_json::to_vec(&refreshed)
                            .map_err(|error| Arc::new(error.into()))?;
                        credentials_provider
                            .write_credentials(CREDENTIALS_KEY, "Bearer", &json, &*cx)
                            .await
                            .map_err(|error| Arc::new(error))?;

                        let api_url = state_clone
                            .read_with(&*cx, |_, cx| KimiLanguageModelProvider::api_url(cx))
                            .map_err(|error| Arc::new(error))?;
                        let headers = kimi_headers(cx).await;
                        let discovered_models = match list_models(
                            http_client.as_ref(),
                            &api_url,
                            &refreshed.access_token,
                            max_output_tokens,
                            &headers,
                        )
                        .await
                        {
                            Ok(models) => models,
                            Err(error) => {
                                log::warn!(
                                    "Failed to discover Kimi models after refresh: {error:?}"
                                );
                                Vec::new()
                            }
                        };

                        state_clone
                            .update(cx, |state, cx| {
                                state.credentials = Some(refreshed.clone());
                                if !discovered_models.is_empty() {
                                    state.discovered_models = discovered_models;
                                }
                                state.models = merge_models(state.discovered_models.clone(), cx);
                                state.refresh_task = None;
                                state.last_auth_error = None;
                                cx.notify();
                            })
                            .map_err(|error| Arc::new(error))?;

                        Ok(refreshed)
                    }
                    .await;

                    if persist_result.is_err() {
                        state_clone
                            .update(cx, |state, _| {
                                state.refresh_task = None;
                            })
                            .log_err();
                    }
                    persist_result
                }
                Err(RefreshError::Fatal(error)) => {
                    log::error!("Kimi token refresh failed fatally: {error:?}");
                    state_clone
                        .update(cx, |state, cx| {
                            state.refresh_task = None;
                            state.credentials = None;
                            state.last_auth_error =
                                Some("Your Kimi session has expired. Please sign in again.".into());
                            state.auth_generation += 1;
                            cx.notify();
                        })
                        .log_err();
                    if let Ok(credentials_provider) =
                        state_clone.read_with(&*cx, |state, _| state.credentials_provider.clone())
                    {
                        credentials_provider
                            .delete_credentials(CREDENTIALS_KEY, &*cx)
                            .await
                            .log_err();
                    }
                    Err(Arc::new(error))
                }
                Err(RefreshError::Transient(error)) => {
                    log::warn!("Kimi token refresh failed transiently: {error:?}");
                    state_clone
                        .update(cx, |state, _| {
                            state.refresh_task = None;
                        })
                        .log_err();
                    Err(Arc::new(error))
                }
            }
        })
        .shared();

    state
        .update(cx, |state, _| {
            state.refresh_task = Some(shared_task.clone());
        })
        .map_err(LanguageModelCompletionError::Other)?;

    shared_task
        .await
        .map_err(|error| LanguageModelCompletionError::Other(anyhow!("{error}")))
}

fn do_sign_in(state: &Entity<State>, http_client: &Arc<dyn HttpClient>, cx: &mut App) {
    let auth_generation = {
        let state = state.read(cx);
        if state.is_signing_in() {
            return;
        }
        state.auth_generation
    };

    let weak_state = state.downgrade();
    let http_client = http_client.clone();
    let max_output_tokens = KimiLanguageModelProvider::settings(cx).max_output_tokens;

    let task = cx
        .spawn(async move |cx| {
            let result = async {
                let headers = kimi_headers(cx).await;
                let authorization =
                    request_device_authorization(http_client.as_ref(), &headers).await?;
                let verification_url = authorization.verification_url().to_string();
                let should_continue = weak_state.update(cx, |state, cx| {
                    if state.auth_generation != auth_generation {
                        false
                    } else {
                        state.device_authorization = Some(authorization.clone());
                        cx.notify();
                        true
                    }
                })?;
                if !should_continue {
                    clear_sign_in_task_if_current(&weak_state, auth_generation, cx)?;
                    return anyhow::Ok(());
                }
                cx.update(|cx| cx.open_url(&verification_url));

                let token_response =
                    poll_device_token(http_client.as_ref(), &authorization, &headers, cx).await?;
                let credentials = KimiCredentials::from_token_response(token_response);
                if weak_state.read_with(&*cx, |state, _| state.auth_generation)? != auth_generation
                {
                    clear_sign_in_task_if_current(&weak_state, auth_generation, cx)?;
                    return anyhow::Ok(());
                }

                let api_url =
                    weak_state.read_with(&*cx, |_, cx| KimiLanguageModelProvider::api_url(cx))?;
                let discovered_models = match list_models(
                    http_client.as_ref(),
                    &api_url,
                    &credentials.access_token,
                    max_output_tokens,
                    &headers,
                )
                .await
                {
                    Ok(models) => models,
                    Err(error) => {
                        log::warn!("Failed to discover Kimi models after sign-in: {error:?}");
                        Vec::new()
                    }
                };

                let credentials_provider =
                    weak_state.read_with(&*cx, |state, _| state.credentials_provider.clone())?;
                let json = serde_json::to_vec(&credentials)?;

                if weak_state.read_with(&*cx, |state, _| state.auth_generation)? != auth_generation
                {
                    clear_sign_in_task_if_current(&weak_state, auth_generation, cx)?;
                    return anyhow::Ok(());
                }

                credentials_provider
                    .write_credentials(CREDENTIALS_KEY, "Bearer", &json, &*cx)
                    .await?;

                if weak_state.read_with(&*cx, |state, _| state.auth_generation)? != auth_generation
                {
                    reconcile_stale_credentials_write(&weak_state, credentials_provider, cx)
                        .await
                        .log_err();
                    clear_sign_in_task_if_current(&weak_state, auth_generation, cx)?;
                    return anyhow::Ok(());
                }

                weak_state.update(cx, |state, cx| {
                    if state.auth_generation == auth_generation {
                        state.credentials = Some(credentials);
                        state.discovered_models = discovered_models;
                        state.models = merge_models(state.discovered_models.clone(), cx);
                        state.device_authorization = None;
                        state.sign_in_task = None;
                        state.sign_in_generation = None;
                        state.last_auth_error = None;
                        cx.notify();
                    } else if state.sign_in_generation == Some(auth_generation) {
                        state.sign_in_task = None;
                        state.sign_in_generation = None;
                        state.device_authorization = None;
                        cx.notify();
                    }
                })?;

                anyhow::Ok(())
            }
            .await;

            if let Err(error) = result {
                let error = Arc::new(error);
                log::error!("Kimi sign-in failed: {error:?}");
                let should_delete_credentials = weak_state
                    .update(cx, |state, cx| {
                        if state.auth_generation == auth_generation {
                            let should_delete_credentials = state.credentials.is_none();
                            state.sign_in_task = None;
                            state.sign_in_generation = None;
                            state.device_authorization = None;
                            state.last_auth_error =
                                Some("Kimi sign-in failed. Please try again.".into());
                            cx.notify();
                            should_delete_credentials
                        } else if state.sign_in_generation == Some(auth_generation) {
                            state.sign_in_task = None;
                            state.sign_in_generation = None;
                            state.device_authorization = None;
                            cx.notify();
                            false
                        } else {
                            false
                        }
                    })
                    .log_err()
                    .unwrap_or(false);
                if should_delete_credentials {
                    delete_credentials_if_signed_out_current_generation(
                        &weak_state,
                        auth_generation,
                        cx,
                    )
                    .await
                    .log_err();
                }
                return Err(error);
            }

            Ok(())
        })
        .shared();

    state.update(cx, |state, cx| {
        state.sign_in_task = Some(task);
        state.sign_in_generation = Some(auth_generation);
        state.device_authorization = None;
        state.last_auth_error = None;
        cx.notify();
    });
}

fn clear_sign_in_task_if_current(
    state: &gpui::WeakEntity<State>,
    auth_generation: u64,
    cx: &mut AsyncApp,
) -> Result<()> {
    state.update(cx, |state, cx| {
        if state.sign_in_generation == Some(auth_generation) {
            state.sign_in_task = None;
            state.sign_in_generation = None;
            state.device_authorization = None;
            cx.notify();
        }
    })
}

async fn reconcile_stale_credentials_write(
    state: &gpui::WeakEntity<State>,
    credentials_provider: Arc<dyn CredentialsProvider>,
    cx: &mut AsyncApp,
) -> Result<()> {
    let (current_credentials, has_active_sign_in) = state.read_with(&*cx, |state, _| {
        (state.credentials.clone(), state.sign_in_task.is_some())
    })?;

    if let Some(current_credentials) = current_credentials {
        let json = serde_json::to_vec(&current_credentials)?;
        credentials_provider
            .write_credentials(CREDENTIALS_KEY, "Bearer", &json, &*cx)
            .await?;
    } else if !has_active_sign_in {
        credentials_provider
            .delete_credentials(CREDENTIALS_KEY, &*cx)
            .await?;
    }

    Ok(())
}

async fn delete_credentials_if_signed_out_current_generation(
    state: &gpui::WeakEntity<State>,
    auth_generation: u64,
    cx: &mut AsyncApp,
) -> Result<()> {
    let (should_delete, credentials_provider) = state.read_with(&*cx, |state, _| {
        (
            state.auth_generation == auth_generation && state.credentials.is_none(),
            state.credentials_provider.clone(),
        )
    })?;

    if should_delete {
        credentials_provider
            .delete_credentials(CREDENTIALS_KEY, &*cx)
            .await?;
    }

    Ok(())
}

fn do_sign_out(state: &gpui::WeakEntity<State>, cx: &mut App) -> Task<Result<()>> {
    let weak_state = state.clone();
    let sign_out_generation = weak_state
        .update(cx, |state, cx| {
            state.auth_generation += 1;
            state.credentials = None;
            state.device_authorization = None;
            state.sign_in_task = None;
            state.sign_in_generation = None;
            state.refresh_task = None;
            state.last_auth_error = None;
            state.discovered_models = Vec::new();
            state.models = merge_models(Vec::new(), cx);
            cx.notify();
            state.auth_generation
        })
        .log_err()
        .unwrap_or_default();

    cx.spawn(async move |cx| {
        let credentials_provider =
            weak_state.read_with(&*cx, |state, _| state.credentials_provider.clone())?;
        let should_delete = weak_state.read_with(&*cx, |state, _| {
            state.auth_generation == sign_out_generation
                && state.credentials.is_none()
                && state.sign_in_task.is_none()
        })?;
        if should_delete {
            credentials_provider
                .delete_credentials(CREDENTIALS_KEY, &*cx)
                .await
                .context("Failed to delete Kimi credentials from keychain")?;
        }
        anyhow::Ok(())
    })
}

async fn request_device_authorization(
    client: &dyn HttpClient,
    headers: &[(String, String)],
) -> Result<DeviceAuthorization> {
    post_form(
        client,
        KIMI_DEVICE_AUTHORIZATION_URL,
        &[("client_id", KIMI_CLIENT_ID)],
        headers,
    )
    .await
}

async fn poll_device_token(
    client: &dyn HttpClient,
    authorization: &DeviceAuthorization,
    headers: &[(String, String)],
    cx: &mut AsyncApp,
) -> Result<TokenResponse> {
    let mut interval = authorization.interval.unwrap_or(5).max(1);
    let deadline = now_ms() + authorization.expires_in.unwrap_or(600).saturating_mul(1000);

    loop {
        let now = now_ms();
        if now >= deadline {
            break;
        }

        let Some(sleep_ms) = poll_sleep_duration_ms(now, deadline, interval) else {
            break;
        };
        cx.background_executor()
            .timer(Duration::from_millis(sleep_ms))
            .await;

        if now_ms() >= deadline {
            break;
        }

        let response = post_form::<TokenResponse>(
            client,
            KIMI_TOKEN_URL,
            &[
                ("client_id", KIMI_CLIENT_ID),
                ("device_code", &authorization.device_code),
                ("grant_type", KIMI_DEVICE_GRANT),
            ],
            headers,
        )
        .await;

        match response {
            Ok(response) => return Ok(response),
            Err(error) if kimi_error_code(&error).as_deref() == Some("authorization_pending") => {}
            Err(error) if kimi_error_code(&error).as_deref() == Some("slow_down") => {
                interval += 5;
            }
            Err(error) if kimi_error_code(&error).as_deref() == Some("expired_token") => {
                return Err(anyhow!("Kimi device code expired"));
            }
            Err(error) => return Err(error),
        }
    }

    Err(anyhow!(
        "Kimi device code expired before authorization completed"
    ))
}

fn poll_sleep_duration_ms(now: u64, deadline: u64, interval_secs: u64) -> Option<u64> {
    if now >= deadline {
        None
    } else {
        Some(
            interval_secs
                .saturating_mul(1000)
                .min(deadline.saturating_sub(now)),
        )
    }
}

async fn refresh_token(
    client: &dyn HttpClient,
    refresh_token: &str,
    cx: &mut AsyncApp,
) -> Result<KimiCredentials, RefreshError> {
    let headers = kimi_headers(cx).await;
    let mut last_error = None;
    for attempt in 0..REFRESH_MAX_RETRIES {
        let response = post_form::<TokenResponse>(
            client,
            KIMI_TOKEN_URL,
            &[
                ("client_id", KIMI_CLIENT_ID),
                ("refresh_token", refresh_token),
                ("grant_type", KIMI_REFRESH_GRANT),
            ],
            &headers,
        )
        .await;

        match response {
            Ok(response) => return Ok(KimiCredentials::from_token_response(response)),
            Err(error) => {
                let status = kimi_error_status(&error);
                if matches!(status, Some(401 | 403)) {
                    return Err(RefreshError::Fatal(error));
                }

                let retryable = status
                    .map(|status| RETRYABLE_REFRESH_STATUSES.contains(&status))
                    .unwrap_or(true);
                if !retryable || attempt == REFRESH_MAX_RETRIES - 1 {
                    return Err(RefreshError::Transient(error));
                }

                last_error = Some(error);
                cx.background_executor()
                    .timer(Duration::from_secs(2_u64.saturating_pow(attempt as u32)))
                    .await;
            }
        }
    }

    Err(RefreshError::Transient(
        last_error.unwrap_or_else(|| anyhow!("Kimi token refresh failed")),
    ))
}

async fn list_models(
    client: &dyn HttpClient,
    api_url: &str,
    access_token: &str,
    max_output_tokens: u64,
    headers: &[(String, String)],
) -> Result<Vec<AvailableModel>> {
    let uri = format!("{}/models", api_url.trim_end_matches('/'));
    let mut builder = HttpRequest::builder()
        .method(Method::GET)
        .uri(uri)
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", access_token.trim()));
    for (name, value) in headers {
        builder = builder.header(name, value);
    }
    let request = builder.body(AsyncBody::default())?;
    let mut response = client.send(request).await?;
    let status = response.status();
    let mut body = String::new();
    response.body_mut().read_to_string(&mut body).await?;
    if !status.is_success() {
        return Err(anyhow!(
            "Kimi model discovery failed (HTTP {status}): {body}"
        ));
    }

    let response: ModelsResponse = serde_json::from_str(&body)
        .with_context(|| format!("Failed to parse Kimi models response: {body}"))?;
    let models = response
        .data
        .into_iter()
        .filter(|model| !model.id.is_empty())
        .map(|model| AvailableModel {
            name: model.id,
            display_name: model.display_name,
            max_tokens: DEFAULT_MAX_TOKENS,
            max_output_tokens: Some(max_output_tokens),
            supports_images: model.supports_image_in,
        })
        .collect();

    Ok(models)
}

async fn post_form<T: DeserializeOwned>(
    client: &dyn HttpClient,
    url: &str,
    params: &[(&str, &str)],
    headers: &[(String, String)],
) -> Result<T> {
    let mut body = form_urlencoded::Serializer::new(String::new());
    for (name, value) in params {
        body.append_pair(name, value);
    }

    let mut builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Accept", "application/json");
    for (name, value) in headers {
        builder = builder.header(name, value);
    }
    let request = builder.body(AsyncBody::from(body.finish()))?;
    let mut response = client.send(request).await?;
    let status = response.status();
    let mut response_body = String::new();
    response
        .body_mut()
        .read_to_string(&mut response_body)
        .await?;

    if !status.is_success() {
        let error = serde_json::from_str::<KimiErrorResponse>(&response_body).ok();
        return Err(KimiHttpError {
            status: status.as_u16(),
            code: error.as_ref().and_then(|error| error.error.clone()),
            message: error
                .and_then(|error| error.error_description)
                .unwrap_or(response_body),
        }
        .into());
    }

    serde_json::from_str(&response_body)
        .with_context(|| format!("Failed to parse Kimi response from {url}: {response_body}"))
}

#[derive(Deserialize)]
struct KimiErrorResponse {
    #[serde(default)]
    error: Option<String>,
    #[serde(default)]
    error_description: Option<String>,
}

#[derive(Debug)]
struct KimiHttpError {
    status: u16,
    code: Option<String>,
    message: String,
}

impl std::fmt::Display for KimiHttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.code.as_deref() {
            Some(code) => write!(f, "Kimi HTTP {} ({code}): {}", self.status, self.message),
            None => write!(f, "Kimi HTTP {}: {}", self.status, self.message),
        }
    }
}

impl std::error::Error for KimiHttpError {}

fn kimi_error_status(error: &anyhow::Error) -> Option<u16> {
    error.chain().find_map(|error| {
        error
            .downcast_ref::<KimiHttpError>()
            .map(|error| error.status)
    })
}

fn kimi_error_code(error: &anyhow::Error) -> Option<String> {
    error.chain().find_map(|error| {
        error
            .downcast_ref::<KimiHttpError>()
            .and_then(|error| error.code.clone())
    })
}

fn merge_models(mut discovered_models: Vec<AvailableModel>, cx: &App) -> Vec<AvailableModel> {
    let max_output_tokens = KimiLanguageModelProvider::settings(cx).max_output_tokens;
    for model in &mut discovered_models {
        model.max_output_tokens = Some(max_output_tokens);
    }

    let fallback = AvailableModel {
        name: KIMI_FOR_CODING_MODEL_ID.to_string(),
        display_name: Some("Kimi For Coding".to_string()),
        max_tokens: DEFAULT_MAX_TOKENS,
        max_output_tokens: Some(max_output_tokens),
        supports_images: Some(false),
    };

    if discovered_models.is_empty() {
        discovered_models.push(fallback);
    }

    for model in KimiLanguageModelProvider::settings(cx)
        .available_models
        .iter()
        .cloned()
    {
        if let Some(existing_model) = discovered_models
            .iter_mut()
            .find(|existing| existing.name == model.name)
        {
            *existing_model = model;
        } else {
            discovered_models.push(model);
        }
    }

    discovered_models
}

#[cfg(not(test))]
async fn kimi_headers(cx: &mut AsyncApp) -> Vec<(String, String)> {
    cx.background_spawn(async { KIMI_HEADERS.iter().cloned().collect() })
        .await
}

#[cfg(test)]
async fn kimi_headers(_cx: &mut AsyncApp) -> Vec<(String, String)> {
    test_kimi_headers()
}

#[cfg(test)]
fn test_kimi_headers() -> Vec<(String, String)> {
    vec![
        ("User-Agent".to_string(), KIMI_USER_AGENT.to_string()),
        ("X-Msh-Platform".to_string(), "kimi_cli".to_string()),
        ("X-Msh-Version".to_string(), KIMI_CLI_VERSION.to_string()),
        ("X-Msh-Device-Name".to_string(), "test-device".to_string()),
        ("X-Msh-Device-Model".to_string(), "test-model".to_string()),
        ("X-Msh-Os-Version".to_string(), "test-os".to_string()),
        ("X-Msh-Device-Id".to_string(), "test-device-id".to_string()),
    ]
}

fn ascii_header_value(value: String, fallback: &str) -> String {
    let sanitized = value
        .chars()
        .filter(|character| character.is_ascii() && !character.is_ascii_control())
        .collect::<String>()
        .trim()
        .to_string();
    if sanitized.is_empty() {
        fallback.to_string()
    } else {
        sanitized
    }
}

fn get_device_id() -> String {
    let device_id_path = device_id_path();
    if let Ok(existing) = std::fs::read_to_string(&device_id_path) {
        let existing = existing.trim();
        if !existing.is_empty() {
            set_private_device_id_permissions(&device_id_path);
            return existing.to_string();
        }
    }

    if let Some(parent) = device_id_path.parent()
        && let Err(error) = std::fs::create_dir_all(parent)
    {
        log::warn!("Failed to create Kimi device id directory: {error}");
    }

    let mut bytes = [0_u8; 16];
    rand::rng().fill_bytes(&mut bytes);
    let device_id = bytes
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    if let Err(error) = std::fs::write(&device_id_path, &device_id) {
        log::warn!("Failed to persist Kimi device id: {error}");
    }
    set_private_device_id_permissions(&device_id_path);
    device_id
}

#[cfg(unix)]
fn set_private_device_id_permissions(device_id_path: &PathBuf) {
    use std::os::unix::fs::PermissionsExt as _;

    if let Err(error) =
        std::fs::set_permissions(device_id_path, std::fs::Permissions::from_mode(0o600))
    {
        log::warn!("Failed to set Kimi device id permissions: {error}");
    }
}

#[cfg(not(unix))]
fn set_private_device_id_permissions(_device_id_path: &PathBuf) {}

fn device_id_path() -> PathBuf {
    home_dir().join(".kimi").join("device_id")
}

fn device_name() -> String {
    std::env::var("HOSTNAME")
        .ok()
        .filter(|hostname| !hostname.is_empty())
        .unwrap_or_else(|| command_output("hostname").unwrap_or_else(|| "unknown".to_string()))
}

fn device_model() -> String {
    let system = platform_system();
    let release = command_output("uname -r");
    let machine = command_output("uname -m").unwrap_or_else(|| std::env::consts::ARCH.to_string());

    if system == "Darwin" {
        let version = command_output("sw_vers -productVersion").or(release);
        return match version {
            Some(version) if !machine.is_empty() => format!("macOS {version} {machine}"),
            Some(version) => format!("macOS {version}"),
            None => format!("macOS {machine}").trim().to_string(),
        };
    }

    if system == "Windows_NT" {
        let windows_label = command_output("cmd /C ver")
            .as_deref()
            .and_then(windows_version_label)
            .unwrap_or_else(|| "Windows".to_string());
        return format!("{windows_label} {machine}").trim().to_string();
    }

    match release {
        Some(release) if !machine.is_empty() => format!("{system} {release} {machine}"),
        Some(release) => format!("{system} {release}"),
        None => format!("{system} {machine}").trim().to_string(),
    }
}

fn windows_version_label(version: &str) -> Option<String> {
    let version = version
        .split(|character: char| !character.is_ascii_digit() && character != '.')
        .find(|part| part.split('.').count() >= 3)?;
    let mut parts = version.split('.');
    let major = parts.next()?;
    let _minor = parts.next()?;
    let build = parts.next()?.parse::<u64>().ok()?;

    if major == "10" {
        if build >= 22_000 {
            Some("Windows 11".to_string())
        } else {
            Some("Windows 10".to_string())
        }
    } else {
        Some(format!("Windows {version}"))
    }
}

fn os_version() -> String {
    command_output("uname -v").unwrap_or_else(|| platform_system())
}

fn platform_system() -> String {
    if cfg!(target_os = "macos") {
        "Darwin".to_string()
    } else if cfg!(target_os = "windows") {
        "Windows_NT".to_string()
    } else if cfg!(target_os = "linux") {
        "Linux".to_string()
    } else {
        std::env::consts::OS.to_string()
    }
}

fn command_output(command: &str) -> Option<String> {
    let mut parts = command.split_whitespace();
    let program = parts.next()?;
    let output = Command::new(program).args(parts).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8(output.stdout).ok()?;
    let stdout = stdout.trim().to_string();
    (!stdout.is_empty()).then_some(stdout)
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .unwrap_or_else(|error| {
            log::error!("System clock is before UNIX epoch: {error}");
            0
        })
}

struct ConfigurationView {
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    max_output_tokens_editor: Entity<InputField>,
}

impl ConfigurationView {
    fn new(
        state: Entity<State>,
        http_client: Arc<dyn HttpClient>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let max_output_tokens_editor = cx.new(|cx| {
            let input = InputField::new(window, cx, "32000").label("Max Output Tokens");
            let current = KimiLanguageModelProvider::settings(cx).max_output_tokens;
            if current != DEFAULT_MAX_OUTPUT_TOKENS {
                input.set_text(&current.to_string(), window, cx);
            }
            input
        });

        cx.observe(&state, |_, _, cx| {
            cx.notify();
        })
        .detach();

        Self {
            state,
            http_client,
            max_output_tokens_editor,
        }
    }

    fn save_max_output_tokens(&mut self, cx: &mut Context<Self>) {
        let value_str = self
            .max_output_tokens_editor
            .read(cx)
            .text(cx)
            .trim()
            .to_string();
        let current = KimiLanguageModelProvider::settings(cx).max_output_tokens;

        if let Ok(value) = value_str.parse::<u64>() {
            if value != current {
                let fs = <dyn Fs>::global(cx);
                update_settings_file(fs, cx, move |settings, _| {
                    settings
                        .language_models
                        .get_or_insert_default()
                        .kimi
                        .get_or_insert_default()
                        .max_output_tokens = Some(value);
                });
            }
        } else if value_str.is_empty() && current != DEFAULT_MAX_OUTPUT_TOKENS {
            let fs = <dyn Fs>::global(cx);
            update_settings_file(fs, cx, move |settings, _| {
                settings
                    .language_models
                    .get_or_insert_default()
                    .kimi
                    .get_or_insert_default()
                    .max_output_tokens = None;
            });
        }
    }

    fn reset_max_output_tokens(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.max_output_tokens_editor
            .update(cx, |input, cx| input.set_text("", window, cx));
        let fs = <dyn Fs>::global(cx);
        update_settings_file(fs, cx, |settings, _cx| {
            if let Some(settings) = settings
                .language_models
                .as_mut()
                .and_then(|models| models.kimi.as_mut())
            {
                settings.max_output_tokens = None;
            }
        });
        cx.notify();
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.state.read(cx);

        if state.load_task.is_some() {
            return div()
                .child(Label::new("Loading Kimi credentials..."))
                .into_any_element();
        }

        if state.is_authenticated() {
            let model_count = state.discovered_models.len();
            let label = if model_count == 0 {
                "Signed in to Kimi Code".to_string()
            } else {
                format!("Signed in to Kimi Code ({model_count} models)")
            };
            let state = self.state.downgrade();
            let settings = KimiLanguageModelProvider::settings(cx);
            let custom_max_output_set = settings.max_output_tokens != DEFAULT_MAX_OUTPUT_TOKENS;

            return v_flex()
                .gap_2()
                .child(
                    ConfiguredApiCard::new(label)
                        .button_label("Sign Out")
                        .on_click(cx.listener(move |_this, _, _window, cx| {
                            do_sign_out(&state, cx).detach_and_log_err(cx);
                        })),
                )
                .child(if custom_max_output_set {
                    h_flex()
                        .p_3()
                        .justify_between()
                        .rounded_md()
                        .border_1()
                        .border_color(cx.theme().colors().border)
                        .bg(cx.theme().colors().elevated_surface_background)
                        .child(
                            h_flex()
                                .gap_2()
                                .child(Icon::new(IconName::Check).color(Color::Success))
                                .child(v_flex().gap_1().child(Label::new(format!(
                                    "Max Output Tokens: {}",
                                    settings.max_output_tokens
                                )))),
                        )
                        .child(
                            Button::new("reset-max-output-tokens", "Reset")
                                .label_size(LabelSize::Small)
                                .start_icon(Icon::new(IconName::Undo).size(IconSize::Small))
                                .layer(ElevationIndex::ModalSurface)
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.reset_max_output_tokens(window, cx)
                                })),
                        )
                        .into_any_element()
                } else {
                    v_flex()
                        .on_action(cx.listener(|this, _: &menu::Confirm, _window, cx| {
                            this.save_max_output_tokens(cx)
                        }))
                        .child(self.max_output_tokens_editor.clone())
                        .child(
                            Label::new("Default: 32000")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                        .into_any_element()
                })
                .into_any_element();
        }

        let is_signing_in = state.is_signing_in();
        let device_authorization = state.device_authorization.clone();
        let last_auth_error = state.last_auth_error.clone();
        let state = self.state.clone();
        let http_client = self.http_client.clone();
        let button_label = if is_signing_in {
            "Signing in..."
        } else {
            "Sign in to Kimi Code"
        };

        v_flex()
            .gap_2()
            .child(Label::new(
                "Sign in with Kimi Code to use Kimi's official coding endpoint in Zed.",
            ))
            .when_some(device_authorization, |this, authorization| {
                let user_code = authorization.user_code.clone();
                let verification_url = authorization.verification_url().to_string();
                this.child(Label::new("Enter this code in your browser:"))
                    .child(
                        Button::new("copy-kimi-code", user_code.clone())
                            .full_width()
                            .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                            .on_click(move |_, window, cx| {
                                cx.write_to_clipboard(ClipboardItem::new_string(user_code.clone()));
                                window.refresh();
                            }),
                    )
                    .child(
                        Button::new("open-kimi-authorization", "Open Kimi Authorization")
                            .full_width()
                            .style(ButtonStyle::Outlined)
                            .on_click(move |_, _, cx| cx.open_url(&verification_url)),
                    )
                    .child(
                        Label::new("Waiting for authorization to complete...")
                            .color(Color::Muted)
                            .size(LabelSize::Small),
                    )
            })
            .child(
                Button::new("sign-in-kimi", button_label)
                    .full_width()
                    .style(ButtonStyle::Outlined)
                    .loading(is_signing_in)
                    .disabled(is_signing_in)
                    .on_click(move |_, _window, cx| {
                        do_sign_in(&state, &http_client, cx);
                    }),
            )
            .when_some(last_auth_error, |this, error| {
                this.child(
                    h_flex()
                        .gap_1()
                        .justify_center()
                        .child(
                            Icon::new(IconName::XCircle)
                                .color(Color::Error)
                                .size(IconSize::Small),
                        )
                        .child(Label::new(error).color(Color::Muted)),
                )
            })
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use http_client::FakeHttpClient;
    use parking_lot::Mutex;
    use std::{
        future::Future,
        pin::Pin,
        sync::atomic::{AtomicUsize, Ordering},
    };

    struct FakeCredentialsProvider {
        storage: Mutex<Option<(String, Vec<u8>)>>,
        delete_count: AtomicUsize,
        write_count: AtomicUsize,
    }

    impl FakeCredentialsProvider {
        fn new() -> Self {
            Self {
                storage: Mutex::new(None),
                delete_count: AtomicUsize::new(0),
                write_count: AtomicUsize::new(0),
            }
        }

        fn stored_credentials(&self) -> Option<(String, Vec<u8>)> {
            self.storage.lock().clone()
        }

        fn delete_count(&self) -> usize {
            self.delete_count.load(Ordering::SeqCst)
        }

        fn write_count(&self) -> usize {
            self.write_count.load(Ordering::SeqCst)
        }
    }

    impl CredentialsProvider for FakeCredentialsProvider {
        fn read_credentials<'a>(
            &'a self,
            _url: &'a str,
            _cx: &'a AsyncApp,
        ) -> Pin<Box<dyn Future<Output = Result<Option<(String, Vec<u8>)>>> + 'a>> {
            Box::pin(async { Ok(self.storage.lock().clone()) })
        }

        fn write_credentials<'a>(
            &'a self,
            _url: &'a str,
            username: &'a str,
            password: &'a [u8],
            _cx: &'a AsyncApp,
        ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>> {
            self.storage
                .lock()
                .replace((username.to_string(), password.to_vec()));
            self.write_count.fetch_add(1, Ordering::SeqCst);
            Box::pin(async { Ok(()) })
        }

        fn delete_credentials<'a>(
            &'a self,
            _url: &'a str,
            _cx: &'a AsyncApp,
        ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>> {
            *self.storage.lock() = None;
            self.delete_count.fetch_add(1, Ordering::SeqCst);
            Box::pin(async { Ok(()) })
        }
    }

    fn init_settings(cx: &mut App) {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
    }

    fn make_expired_credentials() -> KimiCredentials {
        KimiCredentials {
            access_token: "old_access".to_string(),
            refresh_token: "old_refresh".to_string(),
            expires_at_ms: 0,
            scope: None,
            token_type: None,
        }
    }

    fn make_fresh_credentials() -> KimiCredentials {
        KimiCredentials {
            access_token: "fresh_access".to_string(),
            refresh_token: "fresh_refresh".to_string(),
            expires_at_ms: now_ms() + TOKEN_REFRESH_BUFFER_MS + 60_000,
            scope: None,
            token_type: None,
        }
    }

    fn fake_token_response(access_token: &str, refresh_token: &str) -> String {
        serde_json::json!({
            "access_token": access_token,
            "refresh_token": refresh_token,
            "expires_in": 3600,
            "scope": "openid profile",
            "token_type": "Bearer"
        })
        .to_string()
    }

    fn fake_models_response() -> String {
        serde_json::json!({
            "data": [
                {
                    "id": "kimi-for-coding",
                    "display_name": "Kimi For Coding",
                    "supports_image_in": false
                },
                {
                    "id": "",
                    "display_name": "Ignored Empty Model"
                }
            ]
        })
        .to_string()
    }

    fn test_state(
        cx: &mut TestAppContext,
        credentials: Option<KimiCredentials>,
        credentials_provider: Arc<dyn CredentialsProvider>,
    ) -> Entity<State> {
        cx.new(|_cx| State {
            credentials,
            discovered_models: Vec::new(),
            models: Vec::new(),
            device_authorization: None,
            sign_in_task: None,
            sign_in_generation: None,
            refresh_task: None,
            load_task: None,
            credentials_provider,
            auth_generation: 0,
            last_auth_error: None,
        })
    }

    fn test_open_ai_request() -> open_ai::Request {
        open_ai::Request {
            model: "kimi-for-coding".to_string(),
            messages: Vec::new(),
            stream: true,
            stream_options: None,
            max_completion_tokens: None,
            stop: Vec::new(),
            temperature: None,
            tool_choice: None,
            parallel_tool_calls: None,
            tools: Vec::new(),
            prompt_cache_key: None,
            reasoning_effort: Some(open_ai::ReasoningEffort::High),
            service_tier: None,
        }
    }

    fn test_available_model(
        name: &str,
        display_name: &str,
        max_tokens: u64,
        max_output_tokens: u64,
        supports_images: bool,
    ) -> AvailableModel {
        AvailableModel {
            name: name.to_string(),
            display_name: Some(display_name.to_string()),
            max_tokens,
            max_output_tokens: Some(max_output_tokens),
            supports_images: Some(supports_images),
        }
    }

    #[test]
    fn ascii_header_value_sanitizes_and_falls_back() {
        assert_eq!(
            ascii_header_value(" Zed\nKimi💥 ".to_string(), "unknown"),
            "ZedKimi"
        );
        assert_eq!(
            ascii_header_value("\n💥\t".to_string(), "unknown"),
            "unknown"
        );
    }

    #[test]
    fn device_authorization_prefers_complete_verification_uri() {
        let authorization = DeviceAuthorization {
            device_code: "device".to_string(),
            user_code: "user".to_string(),
            verification_uri: "https://auth.kimi.com/device".to_string(),
            verification_uri_complete: Some(
                "https://auth.kimi.com/device?user_code=user".to_string(),
            ),
            expires_in: Some(600),
            interval: Some(5),
        };

        assert_eq!(
            authorization.verification_url(),
            "https://auth.kimi.com/device?user_code=user"
        );

        let authorization = DeviceAuthorization {
            verification_uri_complete: None,
            ..authorization
        };
        assert_eq!(
            authorization.verification_url(),
            "https://auth.kimi.com/device"
        );
    }

    #[test]
    fn credentials_from_token_response_maps_fields_and_expiry() {
        let before = now_ms();
        let credentials = KimiCredentials::from_token_response(TokenResponse {
            access_token: "access".to_string(),
            refresh_token: "refresh".to_string(),
            expires_in: 2,
            scope: Some("openid".to_string()),
            token_type: Some("Bearer".to_string()),
        });
        let after = now_ms();

        assert_eq!(credentials.access_token, "access");
        assert_eq!(credentials.refresh_token, "refresh");
        assert_eq!(credentials.scope.as_deref(), Some("openid"));
        assert_eq!(credentials.token_type.as_deref(), Some("Bearer"));
        assert!(credentials.expires_at_ms >= before + 2_000);
        assert!(credentials.expires_at_ms <= after + 2_000);
    }

    #[test]
    fn credentials_expire_with_refresh_buffer() {
        let expired = KimiCredentials {
            expires_at_ms: now_ms() + TOKEN_REFRESH_BUFFER_MS,
            ..make_fresh_credentials()
        };
        assert!(expired.is_expired());

        let fresh = KimiCredentials {
            expires_at_ms: now_ms() + TOKEN_REFRESH_BUFFER_MS + 1_000,
            ..make_fresh_credentials()
        };
        assert!(!fresh.is_expired());
    }

    #[test]
    fn kimi_request_body_injects_thinking_mode() {
        let enabled = kimi_request_body(test_open_ai_request(), KimiThinking::Enabled)
            .expect("request body should serialize");
        let enabled: serde_json::Value =
            serde_json::from_str(&enabled).expect("request body should be json");
        assert_eq!(
            enabled["thinking"],
            serde_json::json!({ "type": "enabled" })
        );

        let disabled = kimi_request_body(test_open_ai_request(), KimiThinking::Disabled)
            .expect("request body should serialize");
        let disabled: serde_json::Value =
            serde_json::from_str(&disabled).expect("request body should be json");
        assert_eq!(
            disabled["thinking"],
            serde_json::json!({ "type": "disabled" })
        );
    }

    #[test]
    fn apply_kimi_fields_sets_prompt_cache_key_and_clears_reasoning_effort() {
        let mut request = test_open_ai_request();

        let thinking = apply_kimi_fields(&mut request, Some("thread-1".to_string()), false);

        assert!(matches!(thinking, KimiThinking::Disabled));
        assert_eq!(request.prompt_cache_key.as_deref(), Some("thread-1"));
        assert_eq!(request.reasoning_effort, None);
    }

    #[test]
    fn post_form_sends_form_body_and_kimi_headers() {
        let captured = Arc::new(Mutex::new(None));
        let captured_request = captured.clone();
        let http_client = FakeHttpClient::create(move |mut request| {
            let captured_request = captured_request.clone();
            async move {
                let mut body = String::new();
                request.body_mut().read_to_string(&mut body).await?;
                *captured_request.lock() = Some((
                    request.method().clone(),
                    request.uri().to_string(),
                    request.headers().clone(),
                    body,
                ));

                Ok(http_client::Response::builder().status(200).body(
                    http_client::AsyncBody::from(fake_token_response("access", "refresh")),
                )?)
            }
        });

        let headers = test_kimi_headers();
        let response = futures::executor::block_on(post_form::<TokenResponse>(
            http_client.as_ref(),
            "https://auth.kimi.com/api/oauth/token",
            &[("client_id", "client"), ("grant_type", KIMI_REFRESH_GRANT)],
            &headers,
        ))
        .expect("post_form should parse response");

        assert_eq!(response.access_token, "access");
        let (method, uri, headers, body) = captured.lock().clone().expect("captured request");
        assert_eq!(method, Method::POST);
        assert_eq!(uri, "https://auth.kimi.com/api/oauth/token");
        assert_eq!(
            headers
                .get("Content-Type")
                .and_then(|value| value.to_str().ok()),
            Some("application/x-www-form-urlencoded")
        );
        assert_eq!(
            headers
                .get("User-Agent")
                .and_then(|value| value.to_str().ok()),
            Some(KIMI_USER_AGENT)
        );
        assert_eq!(
            headers
                .get("X-Msh-Device-Id")
                .and_then(|value| value.to_str().ok()),
            Some("test-device-id")
        );
        assert_eq!(body, "client_id=client&grant_type=refresh_token");
    }

    #[test]
    fn post_form_exposes_kimi_error_status_and_code() {
        let http_client = FakeHttpClient::create(|_| async {
            Ok(http_client::Response::builder()
                .status(429)
                .body(http_client::AsyncBody::from(
                    r#"{"error":"slow_down","error_description":"poll slower"}"#,
                ))?)
        });

        let headers = test_kimi_headers();
        let error = match futures::executor::block_on(post_form::<TokenResponse>(
            http_client.as_ref(),
            "https://auth.kimi.com/api/oauth/token",
            &[("client_id", "client")],
            &headers,
        )) {
            Ok(_) => panic!("non-success response should error"),
            Err(error) => error,
        };

        assert_eq!(kimi_error_status(&error), Some(429));
        assert_eq!(kimi_error_code(&error).as_deref(), Some("slow_down"));
        assert!(error.to_string().contains("poll slower"));
    }

    #[test]
    fn list_models_maps_response_and_skips_empty_ids() {
        let http_client = FakeHttpClient::create(|request| async move {
            assert_eq!(request.method(), Method::GET);
            assert_eq!(request.uri().to_string(), "https://api.kimi.test/models");
            assert_eq!(
                request
                    .headers()
                    .get("Authorization")
                    .and_then(|value| value.to_str().ok()),
                Some("Bearer token")
            );

            Ok(http_client::Response::builder()
                .status(200)
                .body(http_client::AsyncBody::from(fake_models_response()))?)
        });

        let headers = test_kimi_headers();
        let models = futures::executor::block_on(list_models(
            http_client.as_ref(),
            "https://api.kimi.test/",
            " token ",
            32_000,
            &headers,
        ))
        .expect("models should parse");

        assert_eq!(models.len(), 1);
        assert_eq!(models[0].name, "kimi-for-coding");
        assert_eq!(models[0].display_name.as_deref(), Some("Kimi For Coding"));
        assert_eq!(models[0].max_tokens, DEFAULT_MAX_TOKENS);
        assert_eq!(models[0].max_output_tokens, Some(32_000));
        assert_eq!(models[0].supports_images, Some(false));
    }

    #[gpui::test]
    async fn merge_models_updates_discovered_models_to_current_default(cx: &mut TestAppContext) {
        cx.update(|cx| {
            init_settings(cx);
            SettingsStore::update(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .language_models
                        .get_or_insert_default()
                        .kimi
                        .get_or_insert_default()
                        .max_output_tokens = Some(64_000);
                });
            });
        });

        let models = cx.read(|cx| {
            merge_models(
                vec![test_available_model(
                    KIMI_FOR_CODING_MODEL_ID,
                    "Discovered Kimi",
                    789,
                    32_000,
                    false,
                )],
                cx,
            )
        });

        assert_eq!(models.len(), 1);
        assert_eq!(models[0].max_output_tokens, Some(64_000));
    }

    #[gpui::test]
    async fn merge_models_settings_override_fallback_and_discovered_models(
        cx: &mut TestAppContext,
    ) {
        cx.update(|cx| {
            init_settings(cx);
            SettingsStore::update(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    let kimi_settings = settings
                        .language_models
                        .get_or_insert_default()
                        .kimi
                        .get_or_insert_default();
                    kimi_settings.available_models = Some(vec![test_available_model(
                        KIMI_FOR_CODING_MODEL_ID,
                        "Custom Kimi",
                        123,
                        456,
                        true,
                    )]);
                });
            });
        });

        let fallback_models = cx.read(|cx| merge_models(Vec::new(), cx));
        assert_eq!(fallback_models.len(), 1);
        assert_eq!(
            fallback_models[0].display_name.as_deref(),
            Some("Custom Kimi")
        );
        assert_eq!(fallback_models[0].max_tokens, 123);
        assert_eq!(fallback_models[0].max_output_tokens, Some(456));
        assert_eq!(fallback_models[0].supports_images, Some(true));

        let discovered_models = cx.read(|cx| {
            merge_models(
                vec![test_available_model(
                    KIMI_FOR_CODING_MODEL_ID,
                    "Discovered Kimi",
                    789,
                    1_000,
                    false,
                )],
                cx,
            )
        });
        assert_eq!(discovered_models.len(), 1);
        assert_eq!(
            discovered_models[0].display_name.as_deref(),
            Some("Custom Kimi")
        );
        assert_eq!(discovered_models[0].max_tokens, 123);
        assert_eq!(discovered_models[0].max_output_tokens, Some(456));
        assert_eq!(discovered_models[0].supports_images, Some(true));
    }

    #[test]
    fn poll_sleep_duration_is_capped_to_remaining_deadline() {
        assert_eq!(poll_sleep_duration_ms(1_000, 3_500, 10), Some(2_500));
        assert_eq!(poll_sleep_duration_ms(1_000, 20_000, 10), Some(10_000));
        assert_eq!(poll_sleep_duration_ms(3_500, 3_500, 10), None);
    }

    #[gpui::test]
    async fn stale_sign_in_reconciles_persisted_credentials_to_current_state(
        cx: &mut TestAppContext,
    ) {
        let credentials_provider = Arc::new(FakeCredentialsProvider::new());
        let current_credentials = make_fresh_credentials();
        let state = test_state(
            cx,
            Some(current_credentials.clone()),
            credentials_provider.clone(),
        );
        credentials_provider.storage.lock().replace((
            "Bearer".to_string(),
            serde_json::to_vec(&make_expired_credentials()).expect("credentials should serialize"),
        ));
        let weak_state = cx.read(|_cx| state.downgrade());

        cx.spawn({
            let credentials_provider = credentials_provider.clone();
            async move |mut cx| {
                reconcile_stale_credentials_write(&weak_state, credentials_provider, &mut cx).await
            }
        })
        .await
        .expect("stale credential reconciliation should succeed");

        let (_, stored_credentials) = credentials_provider
            .stored_credentials()
            .expect("credentials should be stored");
        let stored_credentials = serde_json::from_slice::<KimiCredentials>(&stored_credentials)
            .expect("stored credentials should deserialize");
        assert_eq!(
            stored_credentials.access_token,
            current_credentials.access_token
        );
        assert_eq!(credentials_provider.delete_count(), 0);
        assert_eq!(credentials_provider.write_count(), 1);
    }

    #[gpui::test]
    async fn stale_sign_in_reconciliation_deletes_when_signed_out(cx: &mut TestAppContext) {
        let credentials_provider = Arc::new(FakeCredentialsProvider::new());
        credentials_provider.storage.lock().replace((
            "Bearer".to_string(),
            serde_json::to_vec(&make_expired_credentials()).expect("credentials should serialize"),
        ));
        let state = test_state(cx, None, credentials_provider.clone());
        let weak_state = cx.read(|_cx| state.downgrade());

        cx.spawn({
            let credentials_provider = credentials_provider.clone();
            async move |mut cx| {
                reconcile_stale_credentials_write(&weak_state, credentials_provider, &mut cx).await
            }
        })
        .await
        .expect("stale credential reconciliation should succeed");

        assert!(credentials_provider.stored_credentials().is_none());
        assert_eq!(credentials_provider.delete_count(), 1);
        assert_eq!(credentials_provider.write_count(), 0);
    }

    #[gpui::test]
    async fn sign_out_delete_skips_when_new_sign_in_started(cx: &mut TestAppContext) {
        cx.update(init_settings);
        let credentials_provider = Arc::new(FakeCredentialsProvider::new());
        credentials_provider.storage.lock().replace((
            "Bearer".to_string(),
            serde_json::to_vec(&make_fresh_credentials()).expect("credentials should serialize"),
        ));
        let state = test_state(
            cx,
            Some(make_fresh_credentials()),
            credentials_provider.clone(),
        );
        let weak_state = cx.read(|_cx| state.downgrade());

        let sign_out_task = cx.update(|cx| do_sign_out(&weak_state, cx));
        cx.update(|cx| {
            state.update(cx, |state, _cx| {
                state.sign_in_task = Some(Task::ready(Ok(())).shared());
                state.sign_in_generation = Some(state.auth_generation);
            });
        });

        sign_out_task
            .await
            .expect("sign-out task should not fail when skipping delete");
        assert!(credentials_provider.stored_credentials().is_some());
        assert_eq!(credentials_provider.delete_count(), 0);
    }

    #[gpui::test]
    async fn sign_in_failure_after_sign_out_deletes_stale_credentials(cx: &mut TestAppContext) {
        cx.update(init_settings);
        let credentials_provider = Arc::new(FakeCredentialsProvider::new());
        credentials_provider.storage.lock().replace((
            "Bearer".to_string(),
            serde_json::to_vec(&make_fresh_credentials()).expect("credentials should serialize"),
        ));
        let state = test_state(
            cx,
            Some(make_fresh_credentials()),
            credentials_provider.clone(),
        );
        let weak_state = cx.read(|_cx| state.downgrade());

        let sign_out_task = cx.update(|cx| do_sign_out(&weak_state, cx));
        let auth_generation = cx.update(|cx| {
            state.update(cx, |state, _cx| {
                state.sign_in_task = Some(Task::ready(Ok(())).shared());
                state.sign_in_generation = Some(state.auth_generation);
                state.auth_generation
            })
        });
        sign_out_task
            .await
            .expect("sign-out task should not fail when skipping delete");
        assert!(credentials_provider.stored_credentials().is_some());

        cx.update(|cx| {
            state.update(cx, |state, _cx| {
                state.sign_in_task = None;
                state.sign_in_generation = None;
            });
        });
        cx.spawn(async move |mut cx| {
            delete_credentials_if_signed_out_current_generation(
                &weak_state,
                auth_generation,
                &mut cx,
            )
            .await
        })
        .await
        .expect("failed sign-in cleanup should succeed");

        assert!(credentials_provider.stored_credentials().is_none());
        assert_eq!(credentials_provider.delete_count(), 1);
    }

    #[gpui::test]
    async fn fresh_credentials_skip_refresh(cx: &mut TestAppContext) {
        cx.update(init_settings);
        let refresh_count = Arc::new(AtomicUsize::new(0));
        let refresh_count_clone = refresh_count.clone();
        let http_client = FakeHttpClient::create(move |_| {
            let refresh_count = refresh_count_clone.clone();
            async move {
                refresh_count.fetch_add(1, Ordering::SeqCst);
                Ok(http_client::Response::builder()
                    .status(200)
                    .body(http_client::AsyncBody::default())?)
            }
        });
        let state = test_state(
            cx,
            Some(make_fresh_credentials()),
            Arc::new(FakeCredentialsProvider::new()),
        );
        let weak_state = cx.read(|_cx| state.downgrade());
        let http: Arc<dyn HttpClient> = http_client;

        let result = cx
            .spawn(async move |mut cx| {
                get_fresh_credentials(&weak_state, &http, &mut cx, false).await
            })
            .await
            .expect("fresh credentials should be returned");

        assert_eq!(result.access_token, "fresh_access");
        assert_eq!(refresh_count.load(Ordering::SeqCst), 0);
    }

    #[gpui::test]
    async fn missing_credentials_return_no_api_key(cx: &mut TestAppContext) {
        cx.update(init_settings);
        let http_client = FakeHttpClient::with_200_response();
        let state = test_state(cx, None, Arc::new(FakeCredentialsProvider::new()));
        let weak_state = cx.read(|_cx| state.downgrade());
        let http: Arc<dyn HttpClient> = http_client;

        let result = cx
            .spawn(async move |mut cx| {
                get_fresh_credentials(&weak_state, &http, &mut cx, false).await
            })
            .await;

        assert!(matches!(
            result,
            Err(LanguageModelCompletionError::NoApiKey { .. })
        ));
    }

    #[gpui::test]
    async fn concurrent_refresh_deduplicates_requests(cx: &mut TestAppContext) {
        cx.update(init_settings);
        let refresh_count = Arc::new(AtomicUsize::new(0));
        let model_count = Arc::new(AtomicUsize::new(0));
        let refresh_count_clone = refresh_count.clone();
        let model_count_clone = model_count.clone();
        let http_client = FakeHttpClient::create(move |request| {
            let refresh_count = refresh_count_clone.clone();
            let model_count = model_count_clone.clone();
            async move {
                if request.uri().path().ends_with("/oauth/token") {
                    refresh_count.fetch_add(1, Ordering::SeqCst);
                    Ok(http_client::Response::builder().status(200).body(
                        http_client::AsyncBody::from(fake_token_response(
                            "new_access",
                            "new_refresh",
                        )),
                    )?)
                } else if request.uri().path().ends_with("/models") {
                    model_count.fetch_add(1, Ordering::SeqCst);
                    Ok(http_client::Response::builder()
                        .status(200)
                        .body(http_client::AsyncBody::from(fake_models_response()))?)
                } else {
                    Ok(http_client::Response::builder()
                        .status(404)
                        .body(http_client::AsyncBody::default())?)
                }
            }
        });
        let credentials_provider = Arc::new(FakeCredentialsProvider::new());
        let state = test_state(cx, Some(make_expired_credentials()), credentials_provider);
        let weak_state = cx.read(|_cx| state.downgrade());
        let http: Arc<dyn HttpClient> = http_client;

        let first = cx.spawn({
            let weak_state = weak_state.clone();
            let http = http.clone();
            async move |mut cx| get_fresh_credentials(&weak_state, &http, &mut cx, false).await
        });
        let second = cx.spawn({
            let weak_state = weak_state.clone();
            let http = http.clone();
            async move |mut cx| get_fresh_credentials(&weak_state, &http, &mut cx, false).await
        });

        cx.run_until_parked();
        let first = first.await.expect("first refresh should succeed");
        let second = second.await.expect("second refresh should succeed");

        assert_eq!(first.access_token, "new_access");
        assert_eq!(second.access_token, "new_access");
        assert_eq!(refresh_count.load(Ordering::SeqCst), 1);
        assert_eq!(model_count.load(Ordering::SeqCst), 1);
    }

    #[gpui::test]
    async fn fatal_refresh_clears_auth_state(cx: &mut TestAppContext) {
        cx.update(init_settings);
        let http_client = FakeHttpClient::create(|_| async {
            Ok(http_client::Response::builder()
                .status(401)
                .body(http_client::AsyncBody::from(
                    r#"{"error":"invalid_grant","error_description":"expired"}"#,
                ))?)
        });
        let credentials_provider = Arc::new(FakeCredentialsProvider::new());
        credentials_provider
            .storage
            .lock()
            .replace(("Bearer".to_string(), b"credentials".to_vec()));
        let state = test_state(
            cx,
            Some(make_expired_credentials()),
            credentials_provider.clone(),
        );
        let weak_state = cx.read(|_cx| state.downgrade());
        let http: Arc<dyn HttpClient> = http_client;

        let result = cx
            .spawn(async move |mut cx| {
                get_fresh_credentials(&weak_state, &http, &mut cx, false).await
            })
            .await;

        assert!(result.is_err());
        assert!(credentials_provider.stored_credentials().is_none());
        assert_eq!(credentials_provider.delete_count(), 1);
        cx.read(|cx| {
            let state = state.read(cx);
            assert!(state.credentials.is_none());
            assert!(state.refresh_task.is_none());
            assert!(state.last_auth_error.is_some());
            assert_eq!(state.auth_generation, 1);
        });
    }

    #[gpui::test]
    async fn non_retryable_transient_refresh_keeps_credentials(cx: &mut TestAppContext) {
        cx.update(init_settings);
        let http_client = FakeHttpClient::create(|_| async {
            Ok(http_client::Response::builder()
                .status(400)
                .body(http_client::AsyncBody::from("bad request"))?)
        });
        let state = test_state(
            cx,
            Some(make_expired_credentials()),
            Arc::new(FakeCredentialsProvider::new()),
        );
        let weak_state = cx.read(|_cx| state.downgrade());
        let http: Arc<dyn HttpClient> = http_client;

        let result = cx
            .spawn(async move |mut cx| {
                get_fresh_credentials(&weak_state, &http, &mut cx, false).await
            })
            .await;

        assert!(result.is_err());
        cx.read(|cx| {
            let state = state.read(cx);
            assert!(state.credentials.is_some());
            assert!(state.refresh_task.is_none());
            assert!(state.last_auth_error.is_none());
            assert_eq!(state.auth_generation, 0);
        });
    }

    #[gpui::test]
    async fn sign_out_during_refresh_discards_refreshed_credentials(cx: &mut TestAppContext) {
        cx.update(init_settings);
        let (gate_tx, gate_rx) = futures::channel::oneshot::channel::<()>();
        let gate_rx = Arc::new(Mutex::new(Some(gate_rx)));
        let gate_rx_clone = gate_rx.clone();
        let http_client = FakeHttpClient::create(move |_request| {
            let gate_rx = gate_rx_clone.clone();
            async move {
                let receiver = gate_rx.lock().take();
                if let Some(receiver) = receiver {
                    receiver.await.ok();
                }
                Ok(http_client::Response::builder().status(200).body(
                    http_client::AsyncBody::from(fake_token_response("new_access", "new_refresh")),
                )?)
            }
        });
        let credentials_provider = Arc::new(FakeCredentialsProvider::new());
        let state = test_state(
            cx,
            Some(make_expired_credentials()),
            credentials_provider.clone(),
        );
        let weak_state = cx.read(|_cx| state.downgrade());
        let http: Arc<dyn HttpClient> = http_client;
        let refresh_task = cx.spawn({
            let weak_state = weak_state.clone();
            async move |mut cx| get_fresh_credentials(&weak_state, &http, &mut cx, false).await
        });

        cx.run_until_parked();
        cx.update(|cx| {
            do_sign_out(&weak_state, cx).detach();
        });
        cx.run_until_parked();
        gate_tx.send(()).ok();
        cx.run_until_parked();

        let result = refresh_task.await;
        assert!(result.is_err());
        assert!(credentials_provider.stored_credentials().is_none());
        cx.read(|cx| {
            let state = state.read(cx);
            assert!(state.credentials.is_none());
            assert_eq!(state.auth_generation, 1);
        });
    }

    #[gpui::test]
    async fn clear_sign_in_task_only_clears_matching_generation(cx: &mut TestAppContext) {
        let state = test_state(cx, None, Arc::new(FakeCredentialsProvider::new()));
        cx.update(|cx| {
            state.update(cx, |state, _cx| {
                state.sign_in_task = Some(Task::ready(Ok(())).shared());
                state.sign_in_generation = Some(2);
                state.device_authorization = Some(DeviceAuthorization {
                    device_code: "device".to_string(),
                    user_code: "user".to_string(),
                    verification_uri: "https://auth.kimi.com/device".to_string(),
                    verification_uri_complete: None,
                    expires_in: Some(600),
                    interval: Some(5),
                });
            });
        });
        let weak_state = cx.read(|_cx| state.downgrade());

        cx.spawn({
            let weak_state = weak_state.clone();
            async move |mut cx| clear_sign_in_task_if_current(&weak_state, 1, &mut cx)
        })
        .await
        .expect("non-matching cleanup should succeed");
        cx.read(|cx| {
            let state = state.read(cx);
            assert!(state.sign_in_task.is_some());
            assert_eq!(state.sign_in_generation, Some(2));
            assert!(state.device_authorization.is_some());
        });

        cx.spawn(async move |mut cx| clear_sign_in_task_if_current(&weak_state, 2, &mut cx))
            .await
            .expect("matching cleanup should succeed");
        cx.read(|cx| {
            let state = state.read(cx);
            assert!(state.sign_in_task.is_none());
            assert_eq!(state.sign_in_generation, None);
            assert!(state.device_authorization.is_none());
        });
    }
}
