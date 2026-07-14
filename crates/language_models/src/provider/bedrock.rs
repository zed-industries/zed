use std::pin::Pin;
use std::sync::Arc;

use anyhow::{Context as _, Result, anyhow};
use async_lock::OnceCell;
use aws_config::stalled_stream_protection::StalledStreamProtectionConfig;
use aws_config::{BehaviorVersion, Region};
use aws_credential_types::provider::{ProvideCredentials, SharedCredentialsProvider};
use aws_credential_types::{Credentials, Token};
use aws_http_client::AwsHttpClient;
use aws_sigv4::http_request::{SignableBody, SignableRequest, SigningSettings, sign};
use aws_sigv4::sign::v4;
use bedrock::BedrockSystemContentBlock;
use bedrock::bedrock_client::Client as BedrockClient;
use bedrock::bedrock_client::config::timeout::TimeoutConfig;
use bedrock::bedrock_client::types::{
    CachePointBlock, CachePointType, ContentBlockDelta, ContentBlockStart, ConverseStreamOutput,
    ReasoningContentBlockDelta, StopReason,
};
use bedrock::{
    BedrockAnyToolChoice, BedrockAutoToolChoice, BedrockBlob, BedrockError, BedrockImageBlock,
    BedrockImageFormat, BedrockImageSource, BedrockInnerContent, BedrockMessage, BedrockModelMode,
    BedrockStreamingResponse, BedrockThinkingBlock, BedrockThinkingTextBlock, BedrockTool,
    BedrockToolChoice, BedrockToolConfig, BedrockToolInputSchema, BedrockToolResultBlock,
    BedrockToolResultContentBlock, BedrockToolResultStatus, BedrockToolSpec, BedrockToolUseBlock,
    ConverseModel, MantleModel, MantleProtocol, value_to_aws_document,
};
use collections::{BTreeMap, HashMap};
use credentials_provider::CredentialsProvider;
use futures::{
    AsyncBufReadExt, AsyncReadExt, FutureExt, Stream, StreamExt, future::BoxFuture, io::BufReader,
    stream::BoxStream,
};
use gpui::{
    App, AsyncApp, Context, Entity, FocusHandle, Subscription, Task, TaskExt, Window, actions,
};
use gpui_tokio::Tokio;
use http_client::{
    AsyncBody, CustomHeaders, HttpClient, Method, Request as HttpRequest, RequestBuilderExt,
    http::{HeaderValue, header::AUTHORIZATION},
};
use language_model::{
    AuthenticateError, EnvVar, IconOrSvg, InlineDescription, LanguageModel,
    LanguageModelCompletionError, LanguageModelCompletionEvent, LanguageModelEffortLevel,
    LanguageModelId, LanguageModelName, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest,
    LanguageModelToolChoice, LanguageModelToolResultContent, LanguageModelToolSchemaFormat,
    LanguageModelToolUse, MessageContent, ProviderSettingsView, RateLimiter, Role,
    SubPageProviderSettings, TokenUsage, env_var,
};
use open_ai::responses::Request as OpenAiResponseRequest;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use settings::{
    BedrockAvailableModel as AvailableModel, BedrockMantleAvailableModel as MantleAvailableModel,
    Settings, SettingsStore,
};
use std::sync::LazyLock;
use std::time::SystemTime;
use strum::{EnumIter, IntoEnumIterator, IntoStaticStr};
use ui::{ButtonLink, ConfiguredApiCard, Divider, List, ListBulletItem, prelude::*};
use ui_input::InputField;
use util::ResultExt;

use crate::AllLanguageModelSettings;
use crate::provider::open_ai::{
    ChatCompletionMaxTokensParameter, OpenAiEventMapper, OpenAiResponseEventMapper, into_open_ai,
    into_open_ai_response,
};
use language_model::util::{fix_streamed_json, parse_tool_arguments};
use open_ai::{ReasoningEffort, RequestError, ResponseStreamEvent};

actions!(bedrock, [Tab, TabPrev]);

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("amazon-bedrock");
const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("Amazon Bedrock");
pub(crate) const RESERVED_HEADER_NAMES: &[&str] = &[
    "host",
    "x-amz-date",
    "x-amz-security-token",
    "x-amz-content-sha256",
    "amz-sdk-invocation-id",
    "amz-sdk-request",
];

/// Credentials stored in the keychain for static authentication.
/// Region is handled separately since it's orthogonal to auth method.
#[derive(Default, Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct BedrockCredentials {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: Option<String>,
    pub bearer_token: Option<String>,
}

/// Resolved authentication configuration for Bedrock.
/// Settings take priority over UX-provided credentials.
#[derive(Clone, Debug, PartialEq)]
pub enum BedrockAuth {
    /// Use default AWS credential provider chain (IMDSv2, PodIdentity, env vars, etc.)
    Automatic,
    /// Use AWS named profile from ~/.aws/credentials or ~/.aws/config
    NamedProfile { profile_name: String },
    /// Use AWS SSO profile
    SingleSignOn { profile_name: String },
    /// Use IAM credentials (access key + secret + optional session token)
    IamCredentials {
        access_key_id: String,
        secret_access_key: String,
        session_token: Option<String>,
    },
    /// Use Bedrock API Key (bearer token authentication)
    ApiKey { api_key: String },
}

impl BedrockCredentials {
    /// Convert stored credentials to the appropriate auth variant.
    /// Prefers API key if present, otherwise uses IAM credentials.
    fn into_auth(self) -> Option<BedrockAuth> {
        if let Some(api_key) = self.bearer_token.filter(|t| !t.is_empty()) {
            Some(BedrockAuth::ApiKey { api_key })
        } else if !self.access_key_id.is_empty() && !self.secret_access_key.is_empty() {
            Some(BedrockAuth::IamCredentials {
                access_key_id: self.access_key_id,
                secret_access_key: self.secret_access_key,
                session_token: self.session_token.filter(|t| !t.is_empty()),
            })
        } else {
            None
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct AmazonBedrockSettings {
    pub available_models: Vec<AvailableModel>,
    pub mantle_available_models: Vec<MantleAvailableModel>,
    pub custom_headers: CustomHeaders,
    pub region: Option<String>,
    pub endpoint: Option<String>,
    pub profile_name: Option<String>,
    pub role_arn: Option<String>,
    pub authentication_method: Option<BedrockAuthMethod>,
    pub allow_global: Option<bool>,
    pub guardrail_identifier: Option<String>,
    pub guardrail_version: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EnumIter, IntoStaticStr, JsonSchema)]
pub enum BedrockAuthMethod {
    #[serde(rename = "named_profile")]
    NamedProfile,
    #[serde(rename = "sso")]
    SingleSignOn,
    #[serde(rename = "api_key")]
    ApiKey,
    /// IMDSv2, PodIdentity, env vars, etc.
    #[serde(rename = "default")]
    Automatic,
}

impl From<settings::BedrockAuthMethodContent> for BedrockAuthMethod {
    fn from(value: settings::BedrockAuthMethodContent) -> Self {
        match value {
            settings::BedrockAuthMethodContent::SingleSignOn => BedrockAuthMethod::SingleSignOn,
            settings::BedrockAuthMethodContent::Automatic => BedrockAuthMethod::Automatic,
            settings::BedrockAuthMethodContent::NamedProfile => BedrockAuthMethod::NamedProfile,
            settings::BedrockAuthMethodContent::ApiKey => BedrockAuthMethod::ApiKey,
        }
    }
}

fn mantle_protocol_from_settings(value: settings::BedrockMantleProtocolContent) -> MantleProtocol {
    match value {
        settings::BedrockMantleProtocolContent::ChatCompletions => MantleProtocol::ChatCompletions,
        settings::BedrockMantleProtocolContent::Responses => MantleProtocol::Responses,
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ModelMode {
    #[default]
    Default,
    Thinking {
        /// The maximum number of tokens to use for reasoning. Must be lower than the model's `max_output_tokens`.
        budget_tokens: Option<u64>,
    },
    AdaptiveThinking {
        effort: bedrock::BedrockAdaptiveThinkingEffort,
    },
}

impl From<ModelMode> for BedrockModelMode {
    fn from(value: ModelMode) -> Self {
        match value {
            ModelMode::Default => BedrockModelMode::Default,
            ModelMode::Thinking { budget_tokens } => BedrockModelMode::Thinking { budget_tokens },
            ModelMode::AdaptiveThinking { effort } => BedrockModelMode::AdaptiveThinking { effort },
        }
    }
}

impl From<BedrockModelMode> for ModelMode {
    fn from(value: BedrockModelMode) -> Self {
        match value {
            BedrockModelMode::Default => ModelMode::Default,
            BedrockModelMode::Thinking { budget_tokens } => ModelMode::Thinking { budget_tokens },
            BedrockModelMode::AdaptiveThinking { effort } => ModelMode::AdaptiveThinking { effort },
        }
    }
}

/// The URL of the base AWS service.
///
/// Right now we're just using this as the key to store the AWS credentials
/// under in the keychain.
const AMAZON_AWS_URL: &str = "https://amazonaws.com";

// These environment variables all use a `ZED_` prefix because we don't want to overwrite the user's AWS credentials.
static ZED_BEDROCK_ACCESS_KEY_ID_VAR: LazyLock<EnvVar> = env_var!("ZED_ACCESS_KEY_ID");
static ZED_BEDROCK_SECRET_ACCESS_KEY_VAR: LazyLock<EnvVar> = env_var!("ZED_SECRET_ACCESS_KEY");
static ZED_BEDROCK_SESSION_TOKEN_VAR: LazyLock<EnvVar> = env_var!("ZED_SESSION_TOKEN");
static ZED_AWS_PROFILE_VAR: LazyLock<EnvVar> = env_var!("ZED_AWS_PROFILE");
static ZED_BEDROCK_REGION_VAR: LazyLock<EnvVar> = env_var!("ZED_AWS_REGION");
static ZED_AWS_ENDPOINT_VAR: LazyLock<EnvVar> = env_var!("ZED_AWS_ENDPOINT");
static ZED_BEDROCK_BEARER_TOKEN_VAR: LazyLock<EnvVar> = env_var!("ZED_BEDROCK_BEARER_TOKEN");

/// AWS Regions where the `bedrock-mantle` endpoint is available.
/// See <https://docs.aws.amazon.com/bedrock/latest/userguide/bedrock-mantle.html#regions>.
const MANTLE_SUPPORTED_REGIONS: &[&str] = &[
    "us-east-2",
    "us-east-1",
    "us-west-2",
    "ap-southeast-3",
    "ap-south-1",
    "ap-southeast-2",
    "ap-northeast-1",
    "eu-central-1",
    "eu-west-1",
    "eu-west-2",
    "eu-south-1",
    "eu-north-1",
    "sa-east-1",
    "us-gov-west-1",
];

fn mantle_endpoint_url(region: &str) -> String {
    format!("https://bedrock-mantle.{region}.api.aws/openai/v1")
}

enum MantleAuth {
    ApiKey { api_key: String },
    SigV4 { credentials: Credentials },
}

impl MantleAuth {
    fn apply(&self, request: &mut HttpRequest<AsyncBody>, body: &[u8], region: &str) -> Result<()> {
        match self {
            MantleAuth::ApiKey { api_key } => {
                let value = HeaderValue::from_str(&format!("Bearer {}", api_key.trim()))
                    .context("building Mantle bearer token authorization header")?;
                request.headers_mut().insert(AUTHORIZATION, value);
            }
            MantleAuth::SigV4 { credentials } => {
                sign_mantle_request_sigv4(request, body, credentials, region)?;
            }
        }

        Ok(())
    }
}

fn sign_mantle_request_sigv4(
    request: &mut HttpRequest<AsyncBody>,
    body: &[u8],
    credentials: &Credentials,
    region: &str,
) -> Result<()> {
    sign_mantle_request_sigv4_at(request, body, credentials, region, SystemTime::now())
}

fn sign_mantle_request_sigv4_at(
    request: &mut HttpRequest<AsyncBody>,
    body: &[u8],
    credentials: &Credentials,
    region: &str,
    time: SystemTime,
) -> Result<()> {
    if !request
        .headers()
        .contains_key(http_client::http::header::HOST)
        && let Some(authority) = request.uri().authority()
    {
        let host = HeaderValue::from_str(authority.as_str())
            .context("invalid host header derived from Mantle request URI")?;
        request
            .headers_mut()
            .insert(http_client::http::header::HOST, host);
    }

    let identity = credentials.clone().into();
    let signing_params: aws_sigv4::http_request::SigningParams = v4::SigningParams::builder()
        .identity(&identity)
        .region(region)
        .name("bedrock-mantle")
        .time(time)
        .settings(SigningSettings::default())
        .build()
        .context("building Mantle SigV4 signing params")?
        .into();

    let method = request.method().as_str();
    let uri = request.uri().to_string();
    let headers = request
        .headers()
        .iter()
        .map(|(name, value)| {
            value
                .to_str()
                .map(|value| (name.as_str(), value))
                .with_context(|| format!("header {name} is not valid UTF-8 and cannot be signed"))
        })
        .collect::<Result<Vec<_>>>()?;

    let signable_request =
        SignableRequest::new(method, uri, headers.into_iter(), SignableBody::Bytes(body))
            .context("constructing Mantle SigV4 request")?;

    let (instructions, _signature) = sign(signable_request, &signing_params)
        .context("signing Mantle request with SigV4")?
        .into_parts();
    instructions.apply_to_request_http1x(request);

    Ok(())
}

pub struct State {
    /// The resolved authentication method. Settings take priority over UX credentials.
    auth: Option<BedrockAuth>,
    /// Raw settings from settings.json
    settings: Option<AmazonBedrockSettings>,
    /// Whether credentials came from environment variables (only relevant for static credentials)
    credentials_from_env: bool,
    credentials_provider: Arc<dyn CredentialsProvider>,
    _subscription: Subscription,
}

impl State {
    fn reset_auth(&self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = self.credentials_provider.clone();
        cx.spawn(async move |this, cx| {
            credentials_provider
                .delete_credentials(AMAZON_AWS_URL, cx)
                .await
                .log_err();
            this.update(cx, |this, cx| {
                this.auth = None;
                this.credentials_from_env = false;
                cx.notify();
            })
        })
    }

    fn set_static_credentials(
        &mut self,
        credentials: BedrockCredentials,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let auth = credentials.clone().into_auth();
        let credentials_provider = self.credentials_provider.clone();
        cx.spawn(async move |this, cx| {
            credentials_provider
                .write_credentials(
                    AMAZON_AWS_URL,
                    "Bearer",
                    &serde_json::to_vec(&credentials)?,
                    cx,
                )
                .await?;
            this.update(cx, |this, cx| {
                this.auth = auth;
                this.credentials_from_env = false;
                cx.notify();
            })
        })
    }

    fn is_authenticated(&self) -> bool {
        self.auth.is_some()
    }

    /// Resolve authentication. Settings take priority over UX-provided credentials.
    fn authenticate(&self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        if self.is_authenticated() {
            return Task::ready(Ok(()));
        }

        // Step 1: Check if settings specify an auth method (enterprise control)
        if let Some(settings) = &self.settings {
            if let Some(method) = &settings.authentication_method {
                let profile_name = settings
                    .profile_name
                    .clone()
                    .unwrap_or_else(|| "default".to_string());

                let auth = match method {
                    BedrockAuthMethod::Automatic => BedrockAuth::Automatic,
                    BedrockAuthMethod::NamedProfile => BedrockAuth::NamedProfile { profile_name },
                    BedrockAuthMethod::SingleSignOn => BedrockAuth::SingleSignOn { profile_name },
                    BedrockAuthMethod::ApiKey => {
                        // ApiKey method means "use static credentials from keychain/env"
                        // Fall through to load them below
                        return self.load_static_credentials(cx);
                    }
                };

                return cx.spawn(async move |this, cx| {
                    this.update(cx, |this, cx| {
                        this.auth = Some(auth);
                        this.credentials_from_env = false;
                        cx.notify();
                    })?;
                    Ok(())
                });
            }
        }

        // Step 2: No settings auth method - try to load static credentials
        self.load_static_credentials(cx)
    }

    /// Load static credentials from environment variables or keychain.
    fn load_static_credentials(
        &self,
        cx: &mut Context<Self>,
    ) -> Task<Result<(), AuthenticateError>> {
        let credentials_provider = self.credentials_provider.clone();
        cx.spawn(async move |this, cx| {
            // Try environment variables first
            let (auth, from_env) = if let Some(bearer_token) = &ZED_BEDROCK_BEARER_TOKEN_VAR.value {
                if !bearer_token.is_empty() {
                    (
                        Some(BedrockAuth::ApiKey {
                            api_key: bearer_token.to_string(),
                        }),
                        true,
                    )
                } else {
                    (None, false)
                }
            } else if let Some(access_key_id) = &ZED_BEDROCK_ACCESS_KEY_ID_VAR.value {
                if let Some(secret_access_key) = &ZED_BEDROCK_SECRET_ACCESS_KEY_VAR.value {
                    if !access_key_id.is_empty() && !secret_access_key.is_empty() {
                        let session_token = ZED_BEDROCK_SESSION_TOKEN_VAR
                            .value
                            .as_deref()
                            .filter(|s| !s.is_empty())
                            .map(|s| s.to_string());
                        (
                            Some(BedrockAuth::IamCredentials {
                                access_key_id: access_key_id.to_string(),
                                secret_access_key: secret_access_key.to_string(),
                                session_token,
                            }),
                            true,
                        )
                    } else {
                        (None, false)
                    }
                } else {
                    (None, false)
                }
            } else {
                (None, false)
            };

            // If we got auth from env vars, use it
            if let Some(auth) = auth {
                this.update(cx, |this, cx| {
                    this.auth = Some(auth);
                    this.credentials_from_env = from_env;
                    cx.notify();
                })?;
                return Ok(());
            }

            // Try keychain
            let (_, credentials_bytes) = credentials_provider
                .read_credentials(AMAZON_AWS_URL, cx)
                .await?
                .ok_or(AuthenticateError::CredentialsNotFound)?;

            let credentials_str = String::from_utf8(credentials_bytes)
                .with_context(|| format!("invalid {PROVIDER_NAME} credentials"))?;

            let credentials: BedrockCredentials =
                serde_json::from_str(&credentials_str).context("failed to parse credentials")?;

            let auth = credentials
                .into_auth()
                .ok_or(AuthenticateError::CredentialsNotFound)?;

            this.update(cx, |this, cx| {
                this.auth = Some(auth);
                this.credentials_from_env = false;
                cx.notify();
            })?;

            Ok(())
        })
    }

    /// Get the resolved region. Checks env var, then settings, then defaults to us-east-1.
    fn get_region(&self) -> String {
        // Priority: env var > settings > default
        if let Some(region) = ZED_BEDROCK_REGION_VAR.value.as_deref() {
            if !region.is_empty() {
                return region.to_string();
            }
        }

        self.settings
            .as_ref()
            .and_then(|s| s.region.clone())
            .unwrap_or_else(|| "us-east-1".to_string())
    }

    fn get_allow_global(&self) -> bool {
        self.settings
            .as_ref()
            .and_then(|s| s.allow_global)
            .unwrap_or(false)
    }

    fn get_guardrail_config(&self) -> (Option<String>, Option<String>) {
        self.settings.as_ref().map_or((None, None), |s| {
            (s.guardrail_identifier.clone(), s.guardrail_version.clone())
        })
    }
}

pub struct BedrockLanguageModelProvider {
    http_client: AwsHttpClient,
    plain_http_client: Arc<dyn HttpClient>,
    handle: tokio::runtime::Handle,
    state: Entity<State>,
}

impl BedrockLanguageModelProvider {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut App,
    ) -> Self {
        let state = cx.new(|cx| State {
            auth: None,
            settings: Some(AllLanguageModelSettings::get_global(cx).bedrock.clone()),
            credentials_from_env: false,
            credentials_provider,
            _subscription: cx.observe_global::<SettingsStore>(|_, cx| {
                cx.notify();
            }),
        });

        Self {
            http_client: AwsHttpClient::new(http_client.clone()),
            plain_http_client: http_client,
            handle: Tokio::handle(cx),
            state,
        }
    }

    fn create_language_model(&self, model: bedrock::ConverseModel) -> Arc<dyn LanguageModel> {
        Arc::new(BedrockModel {
            id: LanguageModelId::from(model.id().to_string()),
            model,
            http_client: self.http_client.clone(),
            handle: self.handle.clone(),
            state: self.state.clone(),
            client: OnceCell::new(),
            request_limiter: RateLimiter::new(4),
        })
    }

    fn create_mantle_language_model(&self, model: bedrock::MantleModel) -> Arc<dyn LanguageModel> {
        Arc::new(BedrockMantleModel {
            id: LanguageModelId::from(model.id().to_string()),
            model,
            http_client: self.plain_http_client.clone(),
            state: self.state.clone(),
            credentials_provider: Arc::new(OnceCell::new()),
            request_limiter: RateLimiter::new(4),
        })
    }
}

impl LanguageModelProvider for BedrockLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiBedrock)
    }

    fn default_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(bedrock::ConverseModel::default()))
    }

    fn default_fast_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        let region = self.state.read(cx).get_region();
        Some(self.create_language_model(bedrock::ConverseModel::default_fast(region.as_str())))
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let bedrock_settings = &AllLanguageModelSettings::get_global(cx).bedrock;
        let mut models = BTreeMap::default();

        for model in bedrock::ConverseModel::iter() {
            if !matches!(model, bedrock::ConverseModel::Custom { .. }) {
                models.insert(model.id().to_string(), model);
            }
        }

        // Override with available models from settings
        for model in bedrock_settings.available_models.iter() {
            models.insert(
                model.name.clone(),
                bedrock::ConverseModel::Custom {
                    name: model.name.clone(),
                    display_name: model.display_name.clone(),
                    max_tokens: model.max_tokens,
                    max_output_tokens: model.max_output_tokens,
                    default_temperature: model.default_temperature,
                    cache_configuration: model.cache_configuration.as_ref().map(|config| {
                        bedrock::BedrockModelCacheConfiguration {
                            max_cache_anchors: config.max_cache_anchors,
                            min_total_token: config.min_total_token,
                        }
                    }),
                },
            );
        }

        let mut models: Vec<Arc<dyn LanguageModel>> = models
            .into_values()
            .map(|model| self.create_language_model(model))
            .collect();

        let mut mantle_models = BTreeMap::default();

        for model in bedrock::MantleModel::iter() {
            if !matches!(model, bedrock::MantleModel::Custom { .. }) {
                mantle_models.insert(model.id().to_string(), model);
            }
        }

        // Override with available Mantle models from settings
        for model in bedrock_settings.mantle_available_models.iter() {
            mantle_models.insert(
                model.name.clone(),
                bedrock::MantleModel::Custom {
                    name: model.name.clone(),
                    display_name: model.display_name.clone(),
                    max_tokens: model.max_tokens,
                    max_output_tokens: model.max_output_tokens,
                    protocol: mantle_protocol_from_settings(model.protocol),
                    supports_tools: model.supports_tools.unwrap_or(false),
                    supports_images: model.supports_images.unwrap_or(false),
                    supports_thinking: model.supports_thinking.unwrap_or(false),
                },
            );
        }

        models.extend(
            mantle_models
                .into_values()
                .map(|model| self.create_mantle_language_model(model)),
        );

        models
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated()
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        self.state.update(cx, |state, cx| state.authenticate(cx))
    }

    fn settings_view(&self, _cx: &mut App) -> Option<ProviderSettingsView> {
        let state = self.state.clone();
        Some(ProviderSettingsView::SubPage(
            SubPageProviderSettings::new(move |window, cx| {
                cx.new(|cx| ConfigurationView::new(state.clone(), window, cx))
                    .into()
            })
            .description(InlineDescription::Text(
                "To use Zed's agent with Bedrock, set a custom authentication strategy in your settings or use static credentials. Mantle-only models (e.g. GPT-5.5, GPT-5.4, Grok 4.3) additionally require IAM permissions for the `bedrock-mantle` endpoint.".into(),
            )),
        ))
    }
}

impl LanguageModelProviderState for BedrockLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

struct BedrockModel {
    id: LanguageModelId,
    model: ConverseModel,
    http_client: AwsHttpClient,
    handle: tokio::runtime::Handle,
    client: OnceCell<BedrockClient>,
    state: Entity<State>,
    request_limiter: RateLimiter,
}

impl BedrockModel {
    fn get_or_init_client(&self, cx: &AsyncApp) -> anyhow::Result<&BedrockClient> {
        self.client
            .get_or_try_init_blocking(|| {
                let (auth, endpoint, region) = cx.read_entity(&self.state, |state, _cx| {
                    let endpoint = state.settings.as_ref().and_then(|s| s.endpoint.clone());
                    let region = state.get_region();
                    (state.auth.clone(), endpoint, region)
                });

                let mut config_builder = aws_config::defaults(BehaviorVersion::latest())
                    .stalled_stream_protection(StalledStreamProtectionConfig::disabled())
                    .http_client(self.http_client.clone())
                    .region(Region::new(region))
                    .timeout_config(TimeoutConfig::disabled());

                if let Some(endpoint_url) = endpoint
                    && !endpoint_url.is_empty()
                {
                    config_builder = config_builder.endpoint_url(endpoint_url);
                }

                match auth {
                    Some(BedrockAuth::Automatic) | None => {
                        // Use default AWS credential provider chain
                    }
                    Some(BedrockAuth::NamedProfile { profile_name })
                    | Some(BedrockAuth::SingleSignOn { profile_name }) => {
                        if !profile_name.is_empty() {
                            config_builder = config_builder.profile_name(profile_name);
                        }
                    }
                    Some(BedrockAuth::IamCredentials {
                        access_key_id,
                        secret_access_key,
                        session_token,
                    }) => {
                        let aws_creds = Credentials::new(
                            access_key_id,
                            secret_access_key,
                            session_token,
                            None,
                            "zed-bedrock-provider",
                        );
                        config_builder = config_builder.credentials_provider(aws_creds);
                    }
                    Some(BedrockAuth::ApiKey { api_key }) => {
                        config_builder = config_builder
                            .auth_scheme_preference(["httpBearerAuth".into()]) // https://github.com/smithy-lang/smithy-rs/pull/4241
                            .token_provider(Token::new(api_key, None));
                    }
                }

                let config = self.handle.block_on(config_builder.load());

                anyhow::Ok(BedrockClient::new(&config))
            })
            .context("initializing Bedrock client")?;

        self.client.get().context("Bedrock client not initialized")
    }

    fn stream_completion(
        &self,
        request: bedrock::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<BoxStream<'static, Result<BedrockStreamingResponse, anyhow::Error>>, BedrockError>,
    > {
        let Ok(runtime_client) = self
            .get_or_init_client(cx)
            .cloned()
            .context("Bedrock client not initialized")
        else {
            return futures::future::ready(Err(BedrockError::Other(anyhow!("App state dropped"))))
                .boxed();
        };
        let extra_headers = self.state.read_with(cx, |_, cx| {
            AllLanguageModelSettings::get_global(cx)
                .bedrock
                .custom_headers
                .clone()
        });

        let task = Tokio::spawn(
            cx,
            bedrock::stream_completion(runtime_client, request, extra_headers),
        );
        async move { task.await.map_err(|e| BedrockError::Other(e.into()))? }.boxed()
    }
}

impl LanguageModel for BedrockModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(self.model.display_name().to_string())
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn supports_tools(&self) -> bool {
        self.model.supports_tool_use()
    }

    fn supports_images(&self) -> bool {
        self.model.supports_images()
    }

    fn supports_thinking(&self) -> bool {
        self.model.supports_thinking()
    }

    fn refusal_fallback_model_id(&self) -> Option<&'static str> {
        if self
            .model
            .id()
            .starts_with(anthropic::FABLE_MODEL_ID_PREFIX)
        {
            Some(anthropic::FABLE_FALLBACK_MODEL_ID)
        } else {
            None
        }
    }

    fn supported_effort_levels(&self) -> Vec<language_model::LanguageModelEffortLevel> {
        if self.model.supports_adaptive_thinking() {
            vec![
                language_model::LanguageModelEffortLevel {
                    name: "Low".into(),
                    value: "low".into(),
                    is_default: false,
                },
                language_model::LanguageModelEffortLevel {
                    name: "Medium".into(),
                    value: "medium".into(),
                    is_default: false,
                },
                language_model::LanguageModelEffortLevel {
                    name: "High".into(),
                    value: "high".into(),
                    is_default: true,
                },
                language_model::LanguageModelEffortLevel {
                    name: "XHigh".into(),
                    value: "xhigh".into(),
                    is_default: false,
                },
                language_model::LanguageModelEffortLevel {
                    name: "Max".into(),
                    value: "max".into(),
                    is_default: false,
                },
            ]
            .into_iter()
            .filter(|effort_level| {
                effort_level.value != "xhigh" || self.model.supports_xhigh_adaptive_thinking()
            })
            .collect()
        } else {
            Vec::new()
        }
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto | LanguageModelToolChoice::Any => {
                self.model.supports_tool_use()
            }
            // Add support for None - we'll filter tool calls at response
            LanguageModelToolChoice::None => self.model.supports_tool_use(),
        }
    }

    fn supports_streaming_tools(&self) -> bool {
        true
    }

    fn telemetry_id(&self) -> String {
        format!("bedrock/{}", self.model.id())
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_token_count()
    }

    fn max_output_tokens(&self) -> Option<u64> {
        Some(self.model.max_output_tokens())
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
        if request.contains_custom_tool_input() {
            return async move {
                Err(anyhow::anyhow!("Bedrock does not support custom tools").into())
            }
            .boxed();
        }

        let (region, allow_global, guardrail_identifier, guardrail_version) =
            cx.read_entity(&self.state, |state, _cx| {
                let (gid, gv) = state.get_guardrail_config();
                (state.get_region(), state.get_allow_global(), gid, gv)
            });

        let model_id = match self.model.cross_region_inference_id(&region, allow_global) {
            Ok(s) => s,
            Err(e) => {
                return async move { Err(e.into()) }.boxed();
            }
        };

        let deny_tool_calls = request.tool_choice == Some(LanguageModelToolChoice::None);

        let request = match into_bedrock(
            request,
            model_id,
            self.model.default_temperature(),
            self.model.max_output_tokens(),
            self.model.thinking_mode(),
            self.model.supports_caching(),
            self.model.supports_tool_use(),
            guardrail_identifier,
            guardrail_version,
        ) {
            Ok(request) => request,
            Err(err) => return futures::future::ready(Err(err.into())).boxed(),
        };

        let request = self.stream_completion(request, cx);
        let display_name = self.model.display_name().to_string();
        let future = self.request_limiter.stream(async move {
            let response = request.await.map_err(|err| match err {
                BedrockError::Validation(ref msg) => {
                    if msg.contains("model identifier is invalid") {
                        LanguageModelCompletionError::Other(anyhow!(
                            "{display_name} is not available in {region}. \
                                 Try switching to a region where this model is supported."
                        ))
                    } else {
                        LanguageModelCompletionError::BadRequestFormat {
                            provider: PROVIDER_NAME,
                            message: msg.clone(),
                        }
                    }
                }
                BedrockError::RateLimited => LanguageModelCompletionError::RateLimitExceeded {
                    provider: PROVIDER_NAME,
                    retry_after: None,
                },
                BedrockError::ServiceUnavailable => {
                    LanguageModelCompletionError::ServerOverloaded {
                        provider: PROVIDER_NAME,
                        retry_after: None,
                    }
                }
                BedrockError::AccessDenied(msg) => LanguageModelCompletionError::PermissionError {
                    provider: PROVIDER_NAME,
                    message: msg,
                },
                BedrockError::InternalServer(msg) => {
                    LanguageModelCompletionError::ApiInternalServerError {
                        provider: PROVIDER_NAME,
                        message: msg,
                    }
                }
                other => LanguageModelCompletionError::Other(anyhow!(other)),
            })?;
            let events = map_to_language_model_completion_events(response);

            if deny_tool_calls {
                Ok(deny_tool_use_events(events).boxed())
            } else {
                Ok(events.boxed())
            }
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

const MANTLE_SELECTABLE_REASONING_EFFORTS: &[ReasoningEffort] = &[
    ReasoningEffort::Low,
    ReasoningEffort::Medium,
    ReasoningEffort::High,
    ReasoningEffort::XHigh,
];

fn mantle_default_reasoning_effort(model: &MantleModel) -> Option<ReasoningEffort> {
    model.supports_thinking().then_some(ReasoningEffort::Medium)
}

fn mantle_selected_reasoning_effort(
    request: &LanguageModelRequest,
    model: &MantleModel,
) -> Option<ReasoningEffort> {
    if !model.supports_thinking() {
        return None;
    }

    if request.thinking_allowed {
        request
            .thinking_effort
            .as_deref()
            .and_then(|effort| effort.parse::<ReasoningEffort>().ok())
            .filter(|effort| *effort != ReasoningEffort::None)
            .or_else(|| mantle_default_reasoning_effort(model))
    } else {
        Some(ReasoningEffort::None)
    }
}

fn mantle_supported_effort_levels(model: &MantleModel) -> Vec<LanguageModelEffortLevel> {
    let Some(default_effort) = mantle_default_reasoning_effort(model) else {
        return Vec::new();
    };

    MANTLE_SELECTABLE_REASONING_EFFORTS
        .iter()
        .copied()
        .map(|effort| LanguageModelEffortLevel {
            name: effort.label().into(),
            value: effort.value().into(),
            is_default: effort == default_effort,
        })
        .collect()
}

/// Special-cases Mantle authorization failures with a message that points at
/// the separate `bedrock-mantle` IAM policy namespace instead of regular
/// `bedrock-runtime` permissions.
fn map_mantle_error(model: &MantleModel, error: RequestError) -> LanguageModelCompletionError {
    if let RequestError::HttpResponseError { status_code, .. } = &error
        && *status_code == http_client::http::StatusCode::FORBIDDEN
    {
        return LanguageModelCompletionError::PermissionError {
            provider: PROVIDER_NAME,
            message: format!(
                "Bedrock Mantle denied this request for {}. Mantle-only models require IAM \
                 permissions for the `bedrock-mantle` endpoint (for example via the \
                 `AmazonBedrockMantleInferenceAccess` managed policy) in addition to whatever \
                 permissions your existing Bedrock credentials already have.",
                model.display_name()
            ),
        };
    }
    error.into()
}

/// Resolves an AWS credentials provider for profile/SSO/automatic auth.
/// Cached in `cell` since building it may read config files from disk;
/// credentials themselves are still re-resolved on every call. Async so this
/// never blocks the foreground thread (unlike `BedrockModel::get_or_init_client`).
async fn resolve_mantle_credentials_provider(
    cell: &OnceCell<SharedCredentialsProvider>,
    profile_name: Option<String>,
    region: String,
) -> Result<SharedCredentialsProvider> {
    let provider = cell
        .get_or_try_init(move || async move {
            let mut config_builder =
                aws_config::defaults(BehaviorVersion::latest()).region(Region::new(region));

            if let Some(profile_name) = profile_name.filter(|name| !name.is_empty()) {
                config_builder = config_builder.profile_name(profile_name);
            }

            let config = config_builder.load().await;
            config
                .credentials_provider()
                .context("no AWS credentials provider is configured")
        })
        .await
        .context("resolving AWS credentials for Bedrock Mantle")?;
    Ok(provider.clone())
}

/// Resolves provider settings into concrete Mantle request auth. A configured
/// Bedrock API key is sent as bearer auth; every AWS-credential-based method
/// signs the Mantle HTTP request directly with SigV4.
async fn resolve_mantle_auth(
    credentials_provider: Arc<OnceCell<SharedCredentialsProvider>>,
    auth: Option<BedrockAuth>,
    region: String,
) -> Result<MantleAuth> {
    match auth {
        Some(BedrockAuth::ApiKey { api_key }) => Ok(MantleAuth::ApiKey { api_key }),
        Some(BedrockAuth::IamCredentials {
            access_key_id,
            secret_access_key,
            session_token,
        }) => Ok(MantleAuth::SigV4 {
            credentials: Credentials::new(
                access_key_id,
                secret_access_key,
                session_token,
                None,
                "zed-bedrock-provider",
            ),
        }),
        Some(BedrockAuth::NamedProfile { profile_name })
        | Some(BedrockAuth::SingleSignOn { profile_name }) => {
            let provider = resolve_mantle_credentials_provider(
                &credentials_provider,
                Some(profile_name),
                region.clone(),
            )
            .await?;
            let credentials = provider
                .provide_credentials()
                .await
                .context("failed to resolve AWS credentials")?;
            Ok(MantleAuth::SigV4 { credentials })
        }
        Some(BedrockAuth::Automatic) | None => {
            let provider =
                resolve_mantle_credentials_provider(&credentials_provider, None, region.clone())
                    .await?;
            let credentials = provider
                .provide_credentials()
                .await
                .context("failed to resolve AWS credentials")?;
            Ok(MantleAuth::SigV4 { credentials })
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum MantleChatStreamResult {
    Ok(ResponseStreamEvent),
    Err { error: MantleChatStreamError },
}

#[derive(Deserialize)]
struct MantleChatStreamError {
    message: String,
}

fn parse_mantle_chat_stream_line(line: &str) -> Result<ResponseStreamEvent> {
    match serde_json::from_str(line) {
        Ok(MantleChatStreamResult::Ok(response)) => Ok(response),
        Ok(MantleChatStreamResult::Err { error }) => Err(anyhow!(error.message)),
        Err(error) => {
            log::error!(
                "Failed to parse Mantle chat completion stream event: `{}`\nResponse: `{}`",
                error,
                line,
            );
            Err(anyhow!(error))
        }
    }
}

fn parse_mantle_response_stream_line(line: &str) -> Result<open_ai::responses::StreamEvent> {
    serde_json::from_str(line).map_err(|error| {
        log::error!(
            "Failed to parse Mantle responses stream event: `{}`\nResponse: `{}`",
            error,
            line,
        );
        anyhow!(error)
    })
}

async fn stream_mantle_sse<Request, Event>(
    client: &dyn HttpClient,
    provider_name: &str,
    url: &str,
    region: &str,
    auth: &MantleAuth,
    request: Request,
    extra_headers: &CustomHeaders,
    parse_stream_line: fn(&str) -> Result<Event>,
) -> std::result::Result<BoxStream<'static, Result<Event>>, RequestError>
where
    Request: Serialize,
    Event: Send + 'static,
{
    let body = serde_json::to_vec(&request).map_err(|error| RequestError::Other(error.into()))?;
    let mut request = HttpRequest::builder()
        .method(Method::POST)
        .uri(url)
        .header("Content-Type", "application/json")
        .extra_headers(extra_headers)
        .body(AsyncBody::from(body.clone()))
        .map_err(|error| RequestError::Other(error.into()))?;

    auth.apply(&mut request, &body, region)
        .map_err(RequestError::Other)?;

    let mut response = client.send(request).await?;
    if response.status().is_success() {
        let reader = BufReader::new(response.into_body());
        Ok(reader
            .lines()
            .filter_map(move |line| async move {
                match line {
                    Ok(line) => {
                        let line = line
                            .strip_prefix("data: ")
                            .or_else(|| line.strip_prefix("data:"))?;
                        if line == "[DONE]" || line.is_empty() {
                            None
                        } else {
                            Some(parse_stream_line(line))
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
            .map_err(|error| RequestError::Other(error.into()))?;

        Err(RequestError::HttpResponseError {
            provider: provider_name.to_owned(),
            status_code: response.status(),
            body,
            headers: response.headers().clone(),
        })
    }
}

fn strip_unsupported_mantle_response_fields(request: &mut OpenAiResponseRequest) {
    request.context_management = None;
}

struct BedrockMantleModel {
    id: LanguageModelId,
    model: MantleModel,
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
    credentials_provider: Arc<OnceCell<SharedCredentialsProvider>>,
    request_limiter: RateLimiter,
}

impl BedrockMantleModel {
    fn stream_mantle_request<Request, Event>(
        &self,
        request: Request,
        cx: &AsyncApp,
        endpoint: &'static str,
        parse_stream_line: fn(&str) -> Result<Event>,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<Event>>, LanguageModelCompletionError>>
    where
        Request: Serialize + Send + 'static,
        Event: Send + 'static,
    {
        let http_client = self.http_client.clone();
        let model = self.model.clone();
        let credentials_provider = self.credentials_provider.clone();
        let (auth, region) = cx.read_entity(&self.state, |state, _cx| {
            (state.auth.clone(), state.get_region())
        });
        let url = format!("{}/{}", mantle_endpoint_url(&region), endpoint);
        let extra_headers = cx.read_entity(&self.state, |_, cx| {
            AllLanguageModelSettings::get_global(cx)
                .bedrock
                .custom_headers
                .clone()
        });
        let provider_name = PROVIDER_NAME.0.to_string();
        let auth_task = Tokio::spawn_result(
            cx,
            resolve_mantle_auth(credentials_provider, auth, region.clone()),
        );

        let future = self.request_limiter.stream(async move {
            let auth = auth_task
                .await
                .map_err(LanguageModelCompletionError::Other)?;
            stream_mantle_sse(
                http_client.as_ref(),
                &provider_name,
                &url,
                &region,
                &auth,
                request,
                &extra_headers,
                parse_stream_line,
            )
            .await
            .map_err(|err| map_mantle_error(&model, err))
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }

    fn stream_completion(
        &self,
        request: open_ai::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<BoxStream<'static, Result<ResponseStreamEvent>>, LanguageModelCompletionError>,
    > {
        self.stream_mantle_request(
            request,
            cx,
            "chat/completions",
            parse_mantle_chat_stream_line,
        )
    }

    fn stream_response(
        &self,
        request: OpenAiResponseRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            BoxStream<'static, Result<open_ai::responses::StreamEvent>>,
            LanguageModelCompletionError,
        >,
    > {
        let mut request = request;
        strip_unsupported_mantle_response_fields(&mut request);
        self.stream_mantle_request(request, cx, "responses", parse_mantle_response_stream_line)
    }
}

impl LanguageModel for BedrockMantleModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(self.model.display_name().to_string())
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn supports_tools(&self) -> bool {
        self.model.supports_tools()
    }

    fn tool_input_format(&self) -> LanguageModelToolSchemaFormat {
        LanguageModelToolSchemaFormat::JsonSchemaSubset
    }

    fn supports_images(&self) -> bool {
        self.model.supports_images()
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto | LanguageModelToolChoice::Any => {
                self.model.supports_tools()
            }
            LanguageModelToolChoice::None => true,
        }
    }

    fn supports_streaming_tools(&self) -> bool {
        true
    }

    fn supports_thinking(&self) -> bool {
        self.model.supports_thinking()
    }

    fn supported_effort_levels(&self) -> Vec<LanguageModelEffortLevel> {
        mantle_supported_effort_levels(&self.model)
    }

    fn supports_split_token_display(&self) -> bool {
        true
    }

    fn telemetry_id(&self) -> String {
        format!("bedrock-mantle/{}", self.model.id())
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_token_count()
    }

    fn max_output_tokens(&self) -> Option<u64> {
        Some(self.model.max_output_tokens())
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
        let region = cx.read_entity(&self.state, |state, _cx| state.get_region());

        if !MANTLE_SUPPORTED_REGIONS.contains(&region.as_str()) {
            let display_name = self.model.display_name().to_string();
            let supported = MANTLE_SUPPORTED_REGIONS.join(", ");
            return futures::future::ready(Err(LanguageModelCompletionError::Other(anyhow!(
                "{display_name} is not available in {region} because Bedrock Mantle isn't offered \
                 there. Try switching to one of the following regions: {supported}."
            ))))
            .boxed();
        }

        let model_id = self.model.request_id().to_string();
        let max_output_tokens = Some(self.model.max_output_tokens());

        match self.model.protocol() {
            MantleProtocol::Responses => {
                let request = into_open_ai_response(
                    request,
                    &model_id,
                    self.model.supports_tools(),
                    false,
                    max_output_tokens,
                    mantle_default_reasoning_effort(&self.model),
                    self.model.supports_thinking(),
                );
                let completions = self.stream_response(request, cx);
                async move {
                    let mapper = OpenAiResponseEventMapper::new();
                    Ok(mapper.map_stream(completions.await?).boxed())
                }
                .boxed()
            }
            MantleProtocol::ChatCompletions => {
                let reasoning_effort = mantle_selected_reasoning_effort(&request, &self.model);
                let request = match into_open_ai(
                    request,
                    &model_id,
                    self.model.supports_tools(),
                    false,
                    max_output_tokens,
                    ChatCompletionMaxTokensParameter::MaxCompletionTokens,
                    reasoning_effort,
                    false,
                ) {
                    Ok(request) => request,
                    Err(error) => return async move { Err(error.into()) }.boxed(),
                };
                let completions = self.stream_completion(request, cx);
                async move {
                    let mapper = OpenAiEventMapper::new();
                    Ok(mapper.map_stream(completions.await?).boxed())
                }
                .boxed()
            }
        }
    }
}

fn deny_tool_use_events(
    events: impl Stream<Item = Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>,
) -> impl Stream<Item = Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
    events.map(|event| {
        match event {
            Ok(LanguageModelCompletionEvent::ToolUse(tool_use)) => {
                // Convert tool use to an error message if model decided to call it
                Ok(LanguageModelCompletionEvent::Text(format!(
                    "\n\n[Error: Tool calls are disabled in this context. Attempted to call '{}']",
                    tool_use.name
                )))
            }
            other => other,
        }
    })
}

pub fn into_bedrock(
    request: LanguageModelRequest,
    model: String,
    default_temperature: f32,
    max_output_tokens: u64,
    thinking_mode: BedrockModelMode,
    supports_caching: bool,
    supports_tool_use: bool,
    guardrail_identifier: Option<String>,
    guardrail_version: Option<String>,
) -> Result<bedrock::Request> {
    if request.contains_custom_tool_input() {
        anyhow::bail!("Bedrock does not support custom tools");
    }

    let mut new_messages: Vec<BedrockMessage> = Vec::new();
    let mut system_message = String::new();

    // Track whether messages contain tool content - Bedrock requires toolConfig
    // when tool blocks are present, so we may need to add a dummy tool
    let mut messages_contain_tool_content = false;

    for message in request.messages {
        if message.contents_empty() {
            continue;
        }

        match message.role {
            Role::User | Role::Assistant => {
                let mut bedrock_message_content: Vec<BedrockInnerContent> = message
                    .content
                    .into_iter()
                    .filter_map(|content| match content {
                        MessageContent::Text(text) => {
                            if !text.is_empty() {
                                Some(BedrockInnerContent::Text(text))
                            } else {
                                None
                            }
                        }
                        MessageContent::Compaction(_) => None,
                        MessageContent::Thinking { text, signature } => {
                            if model.contains(ConverseModel::DeepSeekR1.request_id()) {
                                // DeepSeekR1 doesn't support thinking blocks
                                // And the AWS API demands that you strip them
                                return None;
                            }
                            if signature.is_none() {
                                // Thinking blocks without a signature are invalid
                                // (e.g. from cancellation mid-think) and must be
                                // stripped to avoid API errors.
                                return None;
                            }
                            let thinking = BedrockThinkingTextBlock::builder()
                                .text(text)
                                .set_signature(signature)
                                .build()
                                .context("failed to build reasoning block")
                                .log_err()?;

                            Some(BedrockInnerContent::ReasoningContent(
                                BedrockThinkingBlock::ReasoningText(thinking),
                            ))
                        }
                        MessageContent::RedactedThinking(blob) => {
                            if model.contains(ConverseModel::DeepSeekR1.request_id()) {
                                // DeepSeekR1 doesn't support thinking blocks
                                // And the AWS API demands that you strip them
                                return None;
                            }
                            let redacted =
                                BedrockThinkingBlock::RedactedContent(BedrockBlob::new(blob));

                            Some(BedrockInnerContent::ReasoningContent(redacted))
                        }
                        MessageContent::ToolUse(tool_use) => {
                            messages_contain_tool_content = true;
                            let input =
                                if let language_model::LanguageModelToolUseInput::Json(input) =
                                    &tool_use.input
                                {
                                    if input.is_null() {
                                        // Bedrock API requires valid JsonValue, not null, for tool use input
                                        value_to_aws_document(&serde_json::json!({}))
                                    } else {
                                        value_to_aws_document(input)
                                    }
                                } else {
                                    value_to_aws_document(&serde_json::json!({}))
                                };
                            BedrockToolUseBlock::builder()
                                .name(tool_use.name.to_string())
                                .tool_use_id(tool_use.id.to_string())
                                .input(input)
                                .build()
                                .context("failed to build Bedrock tool use block")
                                .log_err()
                                .map(BedrockInnerContent::ToolUse)
                        }
                        MessageContent::ToolResult(tool_result) => {
                            messages_contain_tool_content = true;
                            let mut builder = BedrockToolResultBlock::builder()
                                .tool_use_id(tool_result.tool_use_id.to_string());
                            for part in tool_result.content {
                                let block = match part {
                                    LanguageModelToolResultContent::Text(text) => {
                                        BedrockToolResultContentBlock::Text(text.to_string())
                                    }
                                    LanguageModelToolResultContent::Image(image) => {
                                        use base64::Engine;

                                        match base64::engine::general_purpose::STANDARD
                                            .decode(image.source.as_bytes())
                                        {
                                            Ok(image_bytes) => {
                                                match BedrockImageBlock::builder()
                                                    .format(BedrockImageFormat::Png)
                                                    .source(BedrockImageSource::Bytes(
                                                        BedrockBlob::new(image_bytes),
                                                    ))
                                                    .build()
                                                {
                                                    Ok(image_block) => {
                                                        BedrockToolResultContentBlock::Image(
                                                            image_block,
                                                        )
                                                    }
                                                    Err(err) => {
                                                        BedrockToolResultContentBlock::Text(
                                                            format!(
                                                                "[Failed to build image block: {}]",
                                                                err
                                                            ),
                                                        )
                                                    }
                                                }
                                            }
                                            Err(err) => {
                                                BedrockToolResultContentBlock::Text(format!(
                                                    "[Failed to decode tool result image: {}]",
                                                    err
                                                ))
                                            }
                                        }
                                    }
                                };
                                builder = builder.content(block);
                            }
                            builder
                                .status({
                                    if tool_result.is_error {
                                        BedrockToolResultStatus::Error
                                    } else {
                                        BedrockToolResultStatus::Success
                                    }
                                })
                                .build()
                                .context("failed to build Bedrock tool result block")
                                .log_err()
                                .map(BedrockInnerContent::ToolResult)
                        }
                        MessageContent::Image(image) => {
                            use base64::Engine;

                            let image_bytes = base64::engine::general_purpose::STANDARD
                                .decode(image.source.as_bytes())
                                .context("failed to decode base64 image data")
                                .log_err()?;

                            BedrockImageBlock::builder()
                                .format(BedrockImageFormat::Png)
                                .source(BedrockImageSource::Bytes(BedrockBlob::new(image_bytes)))
                                .build()
                                .context("failed to build Bedrock image block")
                                .log_err()
                                .map(BedrockInnerContent::Image)
                        }
                    })
                    .collect();
                if message.cache && supports_caching && !bedrock_message_content.is_empty() {
                    bedrock_message_content.push(BedrockInnerContent::CachePoint(
                        CachePointBlock::builder()
                            .r#type(CachePointType::Default)
                            .build()
                            .context("failed to build cache point block")?,
                    ));
                }
                let bedrock_role = match message.role {
                    Role::User => bedrock::BedrockRole::User,
                    Role::Assistant => bedrock::BedrockRole::Assistant,
                    Role::System => unreachable!("System role should never occur here"),
                };
                if bedrock_message_content.is_empty() {
                    continue;
                }

                if let Some(last_message) = new_messages.last_mut()
                    && last_message.role == bedrock_role
                {
                    last_message.content.extend(bedrock_message_content);
                    continue;
                }
                new_messages.push(
                    BedrockMessage::builder()
                        .role(bedrock_role)
                        .set_content(Some(bedrock_message_content))
                        .build()
                        .context("failed to build Bedrock message")?,
                );
            }
            Role::System => {
                if !system_message.is_empty() {
                    system_message.push_str("\n\n");
                }
                system_message.push_str(&message.string_contents());
            }
        }
    }

    let mut tool_spec: Vec<BedrockTool> = if supports_tool_use {
        request
            .tools
            .iter()
            .map(|tool| {
                let language_model::LanguageModelRequestToolInput::Function {
                    input_schema, ..
                } = &tool.input
                else {
                    anyhow::bail!("Bedrock does not support custom tools");
                };
                Ok(BedrockTool::ToolSpec(
                    BedrockToolSpec::builder()
                        .name(tool.name.clone())
                        .description(tool.description.clone())
                        .input_schema(BedrockToolInputSchema::Json(value_to_aws_document(
                            input_schema,
                        )))
                        .build()
                        .context("failed to build Bedrock tool spec")?,
                ))
            })
            .collect::<Result<_>>()?
    } else {
        Vec::new()
    };

    // Bedrock requires toolConfig when messages contain tool use/result blocks.
    // If no tools are defined but messages contain tool content (e.g., when
    // summarising a conversation that used tools), add a dummy tool to satisfy
    // the API requirement.
    if supports_tool_use && tool_spec.is_empty() && messages_contain_tool_content {
        tool_spec.push(BedrockTool::ToolSpec(
            BedrockToolSpec::builder()
                .name("_placeholder")
                .description("Placeholder tool to satisfy Bedrock API requirements when conversation history contains tool usage")
                .input_schema(BedrockToolInputSchema::Json(value_to_aws_document(
                    &serde_json::json!({"type": "object", "properties": {}}),
                )))
                .build()
                .context("failed to build placeholder tool spec")?,
        ));
    }

    if !tool_spec.is_empty() && supports_caching {
        tool_spec.push(BedrockTool::CachePoint(
            CachePointBlock::builder()
                .r#type(CachePointType::Default)
                .build()
                .context("failed to build cache point block")?,
        ));
    }

    let tool_choice = match request.tool_choice {
        Some(LanguageModelToolChoice::Auto) | None => {
            BedrockToolChoice::Auto(BedrockAutoToolChoice::builder().build())
        }
        Some(LanguageModelToolChoice::Any) => {
            BedrockToolChoice::Any(BedrockAnyToolChoice::builder().build())
        }
        Some(LanguageModelToolChoice::None) => {
            // For None, we still use Auto but will filter out tool calls in the response
            BedrockToolChoice::Auto(BedrockAutoToolChoice::builder().build())
        }
    };
    let tool_config = if tool_spec.is_empty() {
        None
    } else {
        Some(
            BedrockToolConfig::builder()
                .set_tools(Some(tool_spec))
                .tool_choice(tool_choice)
                .build()?,
        )
    };

    let mut system_blocks: Vec<BedrockSystemContentBlock> = Vec::new();
    if !system_message.is_empty() {
        system_blocks.push(BedrockSystemContentBlock::Text(system_message));
        if supports_caching {
            system_blocks.push(BedrockSystemContentBlock::CachePoint(
                CachePointBlock::builder()
                    .r#type(CachePointType::Default)
                    .build()
                    .context("failed to build system cache point block")?,
            ));
        }
    }

    Ok(bedrock::Request {
        model,
        messages: new_messages,
        max_tokens: max_output_tokens,
        system: system_blocks,
        tools: tool_config,
        thinking: if request.thinking_allowed {
            match thinking_mode {
                BedrockModelMode::Thinking { budget_tokens } => {
                    Some(bedrock::Thinking::Enabled { budget_tokens })
                }
                BedrockModelMode::AdaptiveThinking {
                    effort: default_effort,
                } => {
                    let effort = request
                        .thinking_effort
                        .as_deref()
                        .and_then(|e| match e {
                            "low" => Some(bedrock::BedrockAdaptiveThinkingEffort::Low),
                            "medium" => Some(bedrock::BedrockAdaptiveThinkingEffort::Medium),
                            "high" => Some(bedrock::BedrockAdaptiveThinkingEffort::High),
                            "xhigh" => Some(bedrock::BedrockAdaptiveThinkingEffort::XHigh),
                            "max" => Some(bedrock::BedrockAdaptiveThinkingEffort::Max),
                            _ => None,
                        })
                        .unwrap_or(default_effort);
                    Some(bedrock::Thinking::Adaptive { effort })
                }
                BedrockModelMode::Default => None,
            }
        } else {
            None
        },
        metadata: None,
        stop_sequences: Vec::new(),
        temperature: request.temperature.or(Some(default_temperature)),
        top_k: None,
        top_p: None,
        guardrail_identifier,
        guardrail_version,
    })
}

pub fn map_to_language_model_completion_events(
    events: Pin<Box<dyn Send + Stream<Item = Result<BedrockStreamingResponse, anyhow::Error>>>>,
) -> impl Stream<Item = Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
    struct RawToolUse {
        id: String,
        name: String,
        input_json: String,
    }

    struct State {
        events: Pin<Box<dyn Send + Stream<Item = Result<BedrockStreamingResponse, anyhow::Error>>>>,
        tool_uses_by_index: HashMap<i32, RawToolUse>,
        emitted_tool_use: bool,
    }

    let initial_state = State {
        events,
        tool_uses_by_index: HashMap::default(),
        emitted_tool_use: false,
    };

    futures::stream::unfold(initial_state, |mut state| async move {
        match state.events.next().await {
            Some(event_result) => match event_result {
                Ok(event) => {
                    let result = match event {
                        ConverseStreamOutput::ContentBlockDelta(cb_delta) => match cb_delta.delta {
                            Some(ContentBlockDelta::Text(text)) => {
                                Some(Ok(LanguageModelCompletionEvent::Text(text)))
                            }
                            Some(ContentBlockDelta::ToolUse(tool_output)) => {
                                if let Some(tool_use) = state
                                    .tool_uses_by_index
                                    .get_mut(&cb_delta.content_block_index)
                                {
                                    tool_use.input_json.push_str(tool_output.input());
                                    if let Ok(input) = serde_json::from_str::<serde_json::Value>(
                                        &fix_streamed_json(&tool_use.input_json),
                                    ) {
                                        Some(Ok(LanguageModelCompletionEvent::ToolUse(
                                            LanguageModelToolUse {
                                                id: tool_use.id.clone().into(),
                                                name: tool_use.name.clone().into(),
                                                is_input_complete: false,
                                                raw_input: tool_use.input_json.clone(),
                                                input:
                                                    language_model::LanguageModelToolUseInput::Json(
                                                        input,
                                                    ),
                                                thought_signature: None,
                                            },
                                        )))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some(ContentBlockDelta::ReasoningContent(thinking)) => match thinking {
                                ReasoningContentBlockDelta::Text(thoughts) => {
                                    Some(Ok(LanguageModelCompletionEvent::Thinking {
                                        text: thoughts,
                                        signature: None,
                                    }))
                                }
                                ReasoningContentBlockDelta::Signature(sig) => {
                                    Some(Ok(LanguageModelCompletionEvent::Thinking {
                                        text: "".into(),
                                        signature: Some(sig),
                                    }))
                                }
                                ReasoningContentBlockDelta::RedactedContent(redacted) => {
                                    let content = String::from_utf8(redacted.into_inner())
                                        .unwrap_or("REDACTED".to_string());
                                    Some(Ok(LanguageModelCompletionEvent::Thinking {
                                        text: content,
                                        signature: None,
                                    }))
                                }
                                _ => None,
                            },
                            _ => None,
                        },
                        ConverseStreamOutput::ContentBlockStart(cb_start) => {
                            if let Some(ContentBlockStart::ToolUse(tool_start)) = cb_start.start {
                                state.tool_uses_by_index.insert(
                                    cb_start.content_block_index,
                                    RawToolUse {
                                        id: tool_start.tool_use_id,
                                        name: tool_start.name,
                                        input_json: String::new(),
                                    },
                                );
                            }
                            None
                        }
                        ConverseStreamOutput::MessageStart(_) => None,
                        ConverseStreamOutput::ContentBlockStop(cb_stop) => state
                            .tool_uses_by_index
                            .remove(&cb_stop.content_block_index)
                            .map(|tool_use| {
                                state.emitted_tool_use = true;

                                let input = parse_tool_arguments(&tool_use.input_json)
                                    .unwrap_or_else(|_| Value::Object(Default::default()));

                                Ok(LanguageModelCompletionEvent::ToolUse(
                                    LanguageModelToolUse {
                                        id: tool_use.id.into(),
                                        name: tool_use.name.into(),
                                        is_input_complete: true,
                                        raw_input: tool_use.input_json,
                                        input: language_model::LanguageModelToolUseInput::Json(
                                            input,
                                        ),
                                        thought_signature: None,
                                    },
                                ))
                            }),
                        ConverseStreamOutput::Metadata(cb_meta) => cb_meta.usage.map(|metadata| {
                            Ok(LanguageModelCompletionEvent::UsageUpdate(TokenUsage {
                                input_tokens: metadata.input_tokens as u64,
                                output_tokens: metadata.output_tokens as u64,
                                cache_creation_input_tokens: metadata
                                    .cache_write_input_tokens
                                    .unwrap_or_default()
                                    as u64,
                                cache_read_input_tokens: metadata
                                    .cache_read_input_tokens
                                    .unwrap_or_default()
                                    as u64,
                            }))
                        }),
                        ConverseStreamOutput::MessageStop(message_stop) => {
                            let stop_reason = if state.emitted_tool_use {
                                // Some models (e.g. Kimi) send EndTurn even when
                                // they've made tool calls. Trust the content over
                                // the stop reason.
                                language_model::StopReason::ToolUse
                            } else {
                                match message_stop.stop_reason {
                                    StopReason::ToolUse => language_model::StopReason::ToolUse,
                                    _ => language_model::StopReason::EndTurn,
                                }
                            };
                            Some(Ok(LanguageModelCompletionEvent::Stop(stop_reason)))
                        }
                        _ => None,
                    };

                    Some((result, state))
                }
                Err(err) => Some((
                    Some(Err(LanguageModelCompletionError::Other(anyhow!(err)))),
                    state,
                )),
            },
            None => None,
        }
    })
    .filter_map(|result| async move { result })
}

struct ConfigurationView {
    access_key_id_editor: Entity<InputField>,
    secret_access_key_editor: Entity<InputField>,
    session_token_editor: Entity<InputField>,
    bearer_token_editor: Entity<InputField>,
    state: Entity<State>,
    load_credentials_task: Option<Task<()>>,
    focus_handle: FocusHandle,
}

impl ConfigurationView {
    const PLACEHOLDER_ACCESS_KEY_ID_TEXT: &'static str = "XXXXXXXXXXXXXXXX";
    const PLACEHOLDER_SECRET_ACCESS_KEY_TEXT: &'static str =
        "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
    const PLACEHOLDER_SESSION_TOKEN_TEXT: &'static str = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
    const PLACEHOLDER_BEARER_TOKEN_TEXT: &'static str = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";

    fn new(state: Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();

        cx.observe(&state, |_, _, cx| {
            cx.notify();
        })
        .detach();

        let access_key_id_editor = cx.new(|cx| {
            InputField::new(window, cx, Self::PLACEHOLDER_ACCESS_KEY_ID_TEXT)
                .label("Access Key ID")
                .tab_index(0)
                .tab_stop(true)
        });

        let secret_access_key_editor = cx.new(|cx| {
            InputField::new(window, cx, Self::PLACEHOLDER_SECRET_ACCESS_KEY_TEXT)
                .label("Secret Access Key")
                .tab_index(1)
                .tab_stop(true)
        });

        let session_token_editor = cx.new(|cx| {
            InputField::new(window, cx, Self::PLACEHOLDER_SESSION_TOKEN_TEXT)
                .label("Session Token (Optional)")
                .tab_index(2)
                .tab_stop(true)
        });

        let bearer_token_editor = cx.new(|cx| {
            InputField::new(window, cx, Self::PLACEHOLDER_BEARER_TOKEN_TEXT)
                .label("Bedrock API Key")
                .tab_index(3)
                .tab_stop(true)
        });

        let load_credentials_task = Some(cx.spawn({
            let state = state.clone();
            async move |this, cx| {
                if let Some(task) = Some(state.update(cx, |state, cx| state.authenticate(cx))) {
                    // We don't log an error, because "not signed in" is also an error.
                    let _ = task.await;
                }
                this.update(cx, |this, cx| {
                    this.load_credentials_task = None;
                    cx.notify();
                })
                .log_err();
            }
        }));

        Self {
            access_key_id_editor,
            secret_access_key_editor,
            session_token_editor,
            bearer_token_editor,
            state,
            load_credentials_task,
            focus_handle,
        }
    }

    fn save_credentials(
        &mut self,
        _: &menu::Confirm,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let access_key_id = self
            .access_key_id_editor
            .read(cx)
            .text(cx)
            .trim()
            .to_string();
        let secret_access_key = self
            .secret_access_key_editor
            .read(cx)
            .text(cx)
            .trim()
            .to_string();
        let session_token = self
            .session_token_editor
            .read(cx)
            .text(cx)
            .trim()
            .to_string();
        let session_token = if session_token.is_empty() {
            None
        } else {
            Some(session_token)
        };
        let bearer_token = self
            .bearer_token_editor
            .read(cx)
            .text(cx)
            .trim()
            .to_string();
        let bearer_token = if bearer_token.is_empty() {
            None
        } else {
            Some(bearer_token)
        };

        let state = self.state.clone();
        cx.spawn(async move |_, cx| {
            state
                .update(cx, |state, cx| {
                    let credentials = BedrockCredentials {
                        access_key_id,
                        secret_access_key,
                        session_token,
                        bearer_token,
                    };

                    state.set_static_credentials(credentials, cx)
                })
                .await
        })
        .detach_and_log_err(cx);
    }

    fn reset_credentials(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.access_key_id_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));
        self.secret_access_key_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));
        self.session_token_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));
        self.bearer_token_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn(async move |_, cx| state.update(cx, |state, cx| state.reset_auth(cx)).await)
            .detach_and_log_err(cx);
    }

    fn on_tab(&mut self, _: &menu::SelectNext, window: &mut Window, cx: &mut Context<Self>) {
        window.focus_next(cx);
    }

    fn on_tab_prev(
        &mut self,
        _: &menu::SelectPrevious,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus_prev(cx);
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.state.read(cx);
        let env_var_set = state.credentials_from_env;
        let auth = state.auth.clone();
        let settings_auth_method = state
            .settings
            .as_ref()
            .and_then(|s| s.authentication_method.clone());

        if self.load_credentials_task.is_some() {
            return div().child(Label::new("Loading credentials...")).into_any();
        }

        let configured_label = match &auth {
            Some(BedrockAuth::Automatic) => {
                "Using automatic credentials (AWS default chain)".into()
            }
            Some(BedrockAuth::NamedProfile { profile_name }) => {
                format!("Using AWS profile: {profile_name}")
            }
            Some(BedrockAuth::SingleSignOn { profile_name }) => {
                format!("Using AWS SSO profile: {profile_name}")
            }
            Some(BedrockAuth::IamCredentials { .. }) if env_var_set => {
                format!(
                    "Using IAM credentials from {} and {} environment variables",
                    ZED_BEDROCK_ACCESS_KEY_ID_VAR.name, ZED_BEDROCK_SECRET_ACCESS_KEY_VAR.name
                )
            }
            Some(BedrockAuth::IamCredentials { .. }) => "Using IAM credentials".into(),
            Some(BedrockAuth::ApiKey { .. }) if env_var_set => {
                format!(
                    "Using Bedrock API Key from {} environment variable",
                    ZED_BEDROCK_BEARER_TOKEN_VAR.name
                )
            }
            Some(BedrockAuth::ApiKey { .. }) => "Using Bedrock API Key".into(),
            None => "Not authenticated".into(),
        };

        // Determine if credentials can be reset
        // Settings-derived auth (non-ApiKey) cannot be reset from UI
        let is_settings_derived = matches!(
            settings_auth_method,
            Some(BedrockAuthMethod::Automatic)
                | Some(BedrockAuthMethod::NamedProfile)
                | Some(BedrockAuthMethod::SingleSignOn)
        );

        let tooltip_label = if env_var_set {
            Some(format!(
                "To reset your credentials, unset the {}, {}, and {} or {} environment variables.",
                ZED_BEDROCK_ACCESS_KEY_ID_VAR.name,
                ZED_BEDROCK_SECRET_ACCESS_KEY_VAR.name,
                ZED_BEDROCK_SESSION_TOKEN_VAR.name,
                ZED_BEDROCK_BEARER_TOKEN_VAR.name
            ))
        } else if is_settings_derived {
            Some(
                "Authentication method is configured in settings. Edit settings.json to change."
                    .to_string(),
            )
        } else {
            None
        };

        let credentials_control = if self.state.read(cx).is_authenticated() {
            ConfiguredApiCard::new("bedrock-reset", configured_label)
                .disabled(env_var_set || is_settings_derived)
                .on_click(cx.listener(|this, _, window, cx| this.reset_credentials(window, cx)))
                .when_some(tooltip_label, |this, label| this.tooltip_label(label))
                .into_any_element()
        } else {
            self.render_static_credentials_ui().into_any_element()
        };

        v_flex()
            .min_w_0()
            .w_full()
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::on_tab))
            .on_action(cx.listener(Self::on_tab_prev))
            .on_action(cx.listener(ConfigurationView::save_credentials))
            .gap_1()
            .child(Headline::new("Amazon Bedrock").size(HeadlineSize::Small))
            .child(
                Label::new(
                    "To use Zed's agent with Bedrock, you can set a custom authentication strategy through your settings file or use static credentials.",
                )
                .color(Color::Muted),
            )
            .child(
                Label::new("But first, to access models on AWS, you need to:")
                    .mt_1()
                    .color(Color::Muted),
            )
            .child(
                List::new()
                    .child(
                        ListBulletItem::new("")
                            .child(
                                Label::new(
                                    "Grant permissions to the strategy you'll use according to the:",
                                )
                                .color(Color::Muted),
                            )
                            .child(ButtonLink::new(
                                "Prerequisites",
                                "https://docs.aws.amazon.com/bedrock/latest/userguide/inference-prereq.html",
                            )),
                    )
                    .child(
                        ListBulletItem::new("")
                            .child(
                                Label::new("Select the models you would like access to:")
                                    .color(Color::Muted),
                            )
                            .child(ButtonLink::new(
                                "Bedrock Model Catalog",
                                "https://us-east-1.console.aws.amazon.com/bedrock/home?region=us-east-1#/model-catalog",
                            )),
                    ),
            )
            .child(credentials_control)
            .into_any()
    }
}

impl ConfigurationView {
    fn render_static_credentials_ui(&self) -> impl IntoElement {
        let list_item = List::new()
            .child(
                ListBulletItem::new("")
                    .child(
                        Label::new(
                            "For access keys: Create an IAM user in the AWS console with programmatic access",
                        )
                        .color(Color::Muted),
                    )
                    .child(ButtonLink::new(
                        "IAM Console",
                        "https://us-east-1.console.aws.amazon.com/iam/home?region=us-east-1#/users",
                    )),
            )
            .child(
                ListBulletItem::new("")
                    .child(
                        Label::new("For Bedrock API Keys: Generate an API key from the")
                            .color(Color::Muted),
                    )
                    .child(ButtonLink::new(
                        "Bedrock Console",
                        "https://docs.aws.amazon.com/bedrock/latest/userguide/api-keys-use.html",
                    )),
            )
            .child(
                ListBulletItem::new("")
                    .child(
                        Label::new("Attach the necessary Bedrock permissions to")
                            .color(Color::Muted),
                    )
                    .child(ButtonLink::new(
                        "this user",
                        "https://docs.aws.amazon.com/bedrock/latest/userguide/inference-prereq.html",
                    )),
            )
            .child(
                ListBulletItem::new(
                    "Enter either access keys OR a Bedrock API Key below (not both)",
                )
                .label_color(Color::Muted),
            );

        v_flex()
            .my_2()
            .tab_group()
            .gap_1p5()
            .child(Divider::horizontal())
            .child(Label::new("Static Credentials").mt_2())
            .child(
                Label::new(
                    "This method uses your AWS access key ID and secret access key, or a Bedrock API Key.",
                )
                .color(Color::Muted),
            )
            .child(list_item)
            .child(
                v_flex()
                    .gap_1()
                    .child(self.access_key_id_editor.clone())
                    .child(self.secret_access_key_editor.clone())
                    .child(self.session_token_editor.clone()),
            )
            .child(
                Label::new(format!(
                    "You can also set the {}, {} and {} environment variables (or {} for Bedrock API Key authentication) and restart Zed.",
                    ZED_BEDROCK_ACCESS_KEY_ID_VAR.name,
                    ZED_BEDROCK_SECRET_ACCESS_KEY_VAR.name,
                    ZED_BEDROCK_REGION_VAR.name,
                    ZED_BEDROCK_BEARER_TOKEN_VAR.name
                ))
                .size(LabelSize::Small)
                .color(Color::Muted),
            )
            .child(
                Label::new(format!(
                    "Optionally, if your environment uses AWS CLI profiles, you can set {}; if it requires a custom endpoint, you can set {}; and if it requires a Session Token, you can set {}.",
                    ZED_AWS_PROFILE_VAR.name,
                    ZED_AWS_ENDPOINT_VAR.name,
                    ZED_BEDROCK_SESSION_TOKEN_VAR.name
                ))
                .size(LabelSize::Small)
                .color(Color::Muted)
                .mt_1()
                .mb_2p5(),
            )
            .child(Divider::horizontal())
            .child(Label::new("Using the API key").mt_2().mb_1())
            .child(self.bearer_token_editor.clone())
            .child(
                Label::new(format!(
                    "Region is configured via {} environment variable or settings.json (defaults to us-east-1).",
                    ZED_BEDROCK_REGION_VAR.name
                ))
                .size(LabelSize::Small)
                .color(Color::Muted)
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use language_model::LanguageModelRequestMessage;

    fn into_bedrock_request(messages: Vec<LanguageModelRequestMessage>) -> bedrock::Request {
        into_bedrock(
            LanguageModelRequest {
                messages,
                ..Default::default()
            },
            "claude-sonnet-4-5".to_string(),
            1.0,
            4096,
            BedrockModelMode::Default,
            true,
            true,
            None,
            None,
        )
        .unwrap()
    }

    #[test]
    fn test_cache_marked_message_that_filters_to_empty_is_dropped() {
        let request = into_bedrock_request(vec![
            LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text("What's the weather?".into())],
                cache: false,
                reasoning_details: None,
            },
            LanguageModelRequestMessage {
                role: Role::Assistant,
                content: vec![MessageContent::Thinking {
                    text: "Let me think about this...".into(),
                    signature: None,
                }],
                cache: true,
                reasoning_details: None,
            },
            LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text("Summarize this conversation.".into())],
                cache: false,
                reasoning_details: None,
            },
        ]);

        for message in &request.messages {
            assert!(
                message
                    .content()
                    .iter()
                    .any(|block| !matches!(block, BedrockInnerContent::CachePoint(_))),
                "message must not consist solely of cache points: {:?}",
                message
            );
        }
        assert!(
            request
                .messages
                .iter()
                .all(|message| *message.role() == bedrock::BedrockRole::User),
            "the assistant message stripped to empty content should be dropped entirely"
        );
    }

    #[test]
    fn test_cache_marked_message_with_content_gets_cache_point() {
        let request = into_bedrock_request(vec![LanguageModelRequestMessage {
            role: Role::User,
            content: vec![MessageContent::Text("What's the weather?".into())],
            cache: true,
            reasoning_details: None,
        }]);

        assert_eq!(request.messages.len(), 1);
        assert!(
            matches!(
                request.messages[0].content().last(),
                Some(BedrockInnerContent::CachePoint(_))
            ),
            "a cache-marked message with content should end with a cache point"
        );
    }

    #[test]
    fn test_sign_mantle_request_sigv4_uses_mantle_service() {
        let credentials = Credentials::new(
            "AKIDEXAMPLE",
            "wJalrXUtnFEMI/K7MDENG+bPxRfiCYEXAMPLEKEY",
            None,
            None,
            "test",
        );
        let body = br#"{"model":"openai.gpt-5.5"}"#;
        let mut request = HttpRequest::builder()
            .method(Method::POST)
            .uri("https://bedrock-mantle.us-east-1.api.aws/openai/v1/responses")
            .header("Content-Type", "application/json")
            .body(AsyncBody::from(body.to_vec()))
            .unwrap();
        let time = std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_700_000_000);

        sign_mantle_request_sigv4_at(&mut request, body, &credentials, "us-east-1", time).unwrap();

        assert_eq!(
            request
                .headers()
                .get(http_client::http::header::HOST)
                .and_then(|value| value.to_str().ok()),
            Some("bedrock-mantle.us-east-1.api.aws")
        );
        assert_eq!(
            request
                .headers()
                .get("x-amz-date")
                .and_then(|value| value.to_str().ok()),
            Some("20231114T221320Z")
        );
        let authorization = request
            .headers()
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .unwrap();
        assert!(authorization.starts_with("AWS4-HMAC-SHA256 "));
        assert!(
            authorization
                .contains("Credential=AKIDEXAMPLE/20231114/us-east-1/bedrock-mantle/aws4_request")
        );
        assert!(authorization.contains("SignedHeaders=content-type;host"));
        assert!(authorization.contains("Signature="));
    }

    #[test]
    fn test_mantle_endpoint_url_uses_openai_path_prefix() {
        assert_eq!(
            mantle_endpoint_url("us-east-1"),
            "https://bedrock-mantle.us-east-1.api.aws/openai/v1"
        );
        assert_eq!(
            mantle_endpoint_url("us-west-2"),
            "https://bedrock-mantle.us-west-2.api.aws/openai/v1"
        );
    }

    #[test]
    fn test_mantle_protocol_from_settings() {
        assert_eq!(
            mantle_protocol_from_settings(settings::BedrockMantleProtocolContent::ChatCompletions),
            MantleProtocol::ChatCompletions
        );
        assert_eq!(
            mantle_protocol_from_settings(settings::BedrockMantleProtocolContent::Responses),
            MantleProtocol::Responses
        );
    }

    #[test]
    fn test_mantle_supported_regions_matches_docs() {
        assert!(MANTLE_SUPPORTED_REGIONS.contains(&"us-east-1"));
        assert!(MANTLE_SUPPORTED_REGIONS.contains(&"eu-west-1"));
        assert!(!MANTLE_SUPPORTED_REGIONS.contains(&"ap-southeast-1"));
    }

    #[test]
    fn test_builtin_mantle_models_support_thinking() {
        assert!(MantleModel::Gpt5_5.supports_thinking());
        assert!(MantleModel::Gpt5_4.supports_thinking());
        assert!(MantleModel::Grok4_3.supports_thinking());
        assert_eq!(
            mantle_default_reasoning_effort(&MantleModel::Gpt5_5),
            Some(ReasoningEffort::Medium)
        );
        assert_eq!(
            mantle_default_reasoning_effort(&MantleModel::Grok4_3),
            Some(ReasoningEffort::Medium)
        );
    }

    #[test]
    fn test_mantle_supported_effort_levels_hide_none() {
        let effort_levels = mantle_supported_effort_levels(&MantleModel::Gpt5_5);
        let values = effort_levels
            .iter()
            .map(|level| level.value.as_ref())
            .collect::<Vec<_>>();

        assert_eq!(values, ["low", "medium", "high", "xhigh"]);
        assert_eq!(
            effort_levels
                .iter()
                .find(|level| level.is_default)
                .map(|level| level.value.as_ref()),
            Some("medium")
        );
    }

    #[test]
    fn test_custom_mantle_model_can_disable_thinking() {
        let model = MantleModel::Custom {
            name: "custom-mantle-model".to_string(),
            display_name: None,
            max_tokens: 128_000,
            max_output_tokens: None,
            protocol: MantleProtocol::Responses,
            supports_tools: true,
            supports_images: false,
            supports_thinking: false,
        };

        assert!(!model.supports_thinking());
        assert_eq!(mantle_default_reasoning_effort(&model), None);
        assert!(mantle_supported_effort_levels(&model).is_empty());
        assert_eq!(
            mantle_selected_reasoning_effort(
                &LanguageModelRequest {
                    thinking_effort: Some("high".to_string()),
                    ..Default::default()
                },
                &model,
            ),
            None
        );
    }

    #[test]
    fn test_disabled_mantle_thinking_serializes_none() {
        let request = into_open_ai_response(
            LanguageModelRequest {
                thinking_allowed: false,
                ..Default::default()
            },
            MantleModel::Grok4_3.request_id(),
            true,
            false,
            Some(MantleModel::Grok4_3.max_output_tokens()),
            mantle_default_reasoning_effort(&MantleModel::Grok4_3),
            MantleModel::Grok4_3.supports_thinking(),
        );

        assert_eq!(
            serde_json::to_value(&request).unwrap()["reasoning"],
            serde_json::json!({ "effort": "none" })
        );
    }

    #[test]
    fn test_mantle_reasoning_passes_known_efforts_through() {
        for effort in ["low", "medium", "high", "xhigh", "minimal", "max"] {
            assert_eq!(
                mantle_selected_reasoning_effort(
                    &LanguageModelRequest {
                        thinking_allowed: true,
                        thinking_effort: Some(effort.to_string()),
                        ..Default::default()
                    },
                    &MantleModel::Gpt5_5,
                )
                .map(|effort| effort.value()),
                Some(effort)
            );
        }

        assert_eq!(
            mantle_selected_reasoning_effort(
                &LanguageModelRequest {
                    thinking_allowed: true,
                    thinking_effort: Some("none".to_string()),
                    ..Default::default()
                },
                &MantleModel::Gpt5_5,
            ),
            Some(ReasoningEffort::Medium)
        );
    }

    #[test]
    fn test_strip_unsupported_mantle_response_fields_removes_context_management() {
        let mut request = into_open_ai_response(
            LanguageModelRequest {
                compact_at_tokens: Some(10_000),
                ..Default::default()
            },
            "openai.gpt-5.5",
            true,
            false,
            Some(128_000),
            Some(ReasoningEffort::Medium),
            false,
        );

        assert!(request.context_management.is_some());
        strip_unsupported_mantle_response_fields(&mut request);
        assert!(request.context_management.is_none());

        let request = serde_json::to_value(&request).unwrap();
        assert!(request.get("context_management").is_none());
    }
}
