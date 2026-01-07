use std::pin::Pin;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::{Context as _, Result, anyhow};
use aws_config::stalled_stream_protection::StalledStreamProtectionConfig;
use aws_config::{BehaviorVersion, Region};
use aws_credential_types::{Credentials, Token};
use aws_http_client::AwsHttpClient;
use bedrock::bedrock_client::Client as BedrockClient;
use bedrock::bedrock_client::config::timeout::TimeoutConfig;
use bedrock::bedrock_client::types::{
    CachePointBlock, CachePointType, ContentBlockDelta, ContentBlockStart, ConverseStreamOutput,
    ReasoningContentBlockDelta, StopReason,
};
use bedrock::{
    BedrockAnyToolChoice, BedrockAutoToolChoice, BedrockBlob, BedrockError, BedrockInnerContent,
    BedrockMessage, BedrockModelMode, BedrockStreamingResponse, BedrockThinkingBlock,
    BedrockThinkingTextBlock, BedrockTool, BedrockToolChoice, BedrockToolConfig,
    BedrockToolInputSchema, BedrockToolResultBlock, BedrockToolResultContentBlock,
    BedrockToolResultStatus, BedrockToolSpec, BedrockToolUseBlock, Model, value_to_aws_document,
};
use collections::{BTreeMap, HashMap};
use credentials_provider::CredentialsProvider;
use futures::{FutureExt, Stream, StreamExt, future::BoxFuture, stream::BoxStream};
use gpui::{
    AnyView, App, AsyncApp, Context, Entity, FocusHandle, FontWeight, Subscription, Task, Window,
    actions,
};
use gpui_tokio::Tokio;
use http_client::HttpClient;
use language_model::{
    AuthenticateError, EnvVar, IconOrSvg, LanguageModel, LanguageModelCacheConfiguration,
    LanguageModelCompletionError, LanguageModelCompletionEvent, LanguageModelId, LanguageModelName,
    LanguageModelProvider, LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelProviderState, LanguageModelRequest, LanguageModelToolChoice,
    LanguageModelToolResultContent, LanguageModelToolUse, MessageContent, RateLimiter, Role,
    TokenUsage, env_var,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use settings::{BedrockAvailableModel as AvailableModel, Settings, SettingsStore};
use smol::lock::OnceCell;
use std::sync::LazyLock;
use strum::{EnumIter, IntoEnumIterator, IntoStaticStr};
use ui::{ButtonLink, ConfiguredApiCard, List, ListBulletItem, prelude::*};
use ui_input::InputField;
use util::ResultExt;

use crate::AllLanguageModelSettings;

actions!(bedrock, [Tab, TabPrev]);

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("amazon-bedrock");
const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("Amazon Bedrock");

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
    pub region: Option<String>,
    pub endpoint: Option<String>,
    pub profile_name: Option<String>,
    pub role_arn: Option<String>,
    pub authentication_method: Option<BedrockAuthMethod>,
    pub allow_global: Option<bool>,
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

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ModelMode {
    #[default]
    Default,
    Thinking {
        /// The maximum number of tokens to use for reasoning. Must be lower than the model's `max_output_tokens`.
        budget_tokens: Option<u64>,
    },
}

impl From<ModelMode> for BedrockModelMode {
    fn from(value: ModelMode) -> Self {
        match value {
            ModelMode::Default => BedrockModelMode::Default,
            ModelMode::Thinking { budget_tokens } => BedrockModelMode::Thinking { budget_tokens },
        }
    }
}

impl From<BedrockModelMode> for ModelMode {
    fn from(value: BedrockModelMode) -> Self {
        match value {
            BedrockModelMode::Default => ModelMode::Default,
            BedrockModelMode::Thinking { budget_tokens } => ModelMode::Thinking { budget_tokens },
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

pub struct State {
    /// The resolved authentication method. Settings take priority over UX credentials.
    auth: Option<BedrockAuth>,
    /// Raw settings from settings.json
    settings: Option<AmazonBedrockSettings>,
    /// Whether credentials came from environment variables (only relevant for static credentials)
    credentials_from_env: bool,
    _subscription: Subscription,
}

impl State {
    fn reset_auth(&self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = <dyn CredentialsProvider>::global(cx);
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
        let credentials_provider = <dyn CredentialsProvider>::global(cx);
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
        let credentials_provider = <dyn CredentialsProvider>::global(cx);
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
                .context("invalid {PROVIDER_NAME} credentials")?;

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
}

pub struct BedrockLanguageModelProvider {
    http_client: AwsHttpClient,
    handle: tokio::runtime::Handle,
    state: Entity<State>,
}

impl BedrockLanguageModelProvider {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
        let state = cx.new(|cx| State {
            auth: None,
            settings: Some(AllLanguageModelSettings::get_global(cx).bedrock.clone()),
            credentials_from_env: false,
            _subscription: cx.observe_global::<SettingsStore>(|_, cx| {
                cx.notify();
            }),
        });

        Self {
            http_client: AwsHttpClient::new(http_client),
            handle: Tokio::handle(cx),
            state,
        }
    }

    fn create_language_model(&self, model: bedrock::Model) -> Arc<dyn LanguageModel> {
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
        Some(self.create_language_model(bedrock::Model::default()))
    }

    fn default_fast_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        let region = self.state.read(cx).get_region();
        Some(self.create_language_model(bedrock::Model::default_fast(region.as_str())))
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let mut models = BTreeMap::default();

        for model in bedrock::Model::iter() {
            if !matches!(model, bedrock::Model::Custom { .. }) {
                models.insert(model.id().to_string(), model);
            }
        }

        // Override with available models from settings
        for model in AllLanguageModelSettings::get_global(cx)
            .bedrock
            .available_models
            .iter()
        {
            models.insert(
                model.name.clone(),
                bedrock::Model::Custom {
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

    fn configuration_view(
        &self,
        _target_agent: language_model::ConfigurationViewTargetAgent,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyView {
        cx.new(|cx| ConfigurationView::new(self.state.clone(), window, cx))
            .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state.update(cx, |state, cx| state.reset_auth(cx))
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
    model: Model,
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
        Result<BoxStream<'static, Result<BedrockStreamingResponse, BedrockError>>>,
    > {
        let Ok(runtime_client) = self
            .get_or_init_client(cx)
            .cloned()
            .context("Bedrock client not initialized")
        else {
            return futures::future::ready(Err(anyhow!("App state dropped"))).boxed();
        };

        let task = Tokio::spawn(cx, bedrock::stream_completion(runtime_client, request));
        async move { task.await.map_err(|err| anyhow!(err))? }.boxed()
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
        false
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

    fn telemetry_id(&self) -> String {
        format!("bedrock/{}", self.model.id())
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_token_count()
    }

    fn max_output_tokens(&self) -> Option<u64> {
        Some(self.model.max_output_tokens())
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &App,
    ) -> BoxFuture<'static, Result<u64>> {
        get_bedrock_tokens(request, cx)
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
        let (region, allow_global) = cx.read_entity(&self.state, |state, _cx| {
            (state.get_region(), state.get_allow_global())
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
            self.model.mode(),
            self.model.supports_caching(),
        ) {
            Ok(request) => request,
            Err(err) => return futures::future::ready(Err(err.into())).boxed(),
        };

        let request = self.stream_completion(request, cx);
        let future = self.request_limiter.stream(async move {
            let response = request.await.map_err(|err| anyhow!(err))?;
            let events = map_to_language_model_completion_events(response);

            if deny_tool_calls {
                Ok(deny_tool_use_events(events).boxed())
            } else {
                Ok(events.boxed())
            }
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }

    fn cache_configuration(&self) -> Option<LanguageModelCacheConfiguration> {
        self.model
            .cache_configuration()
            .map(|config| LanguageModelCacheConfiguration {
                max_cache_anchors: config.max_cache_anchors,
                should_speculate: false,
                min_total_token: config.min_total_token,
            })
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
    mode: BedrockModelMode,
    supports_caching: bool,
) -> Result<bedrock::Request> {
    let mut new_messages: Vec<BedrockMessage> = Vec::new();
    let mut system_message = String::new();

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
                        MessageContent::Thinking { text, signature } => {
                            if model.contains(Model::DeepSeekR1.request_id()) {
                                // DeepSeekR1 doesn't support thinking blocks
                                // And the AWS API demands that you strip them
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
                            if model.contains(Model::DeepSeekR1.request_id()) {
                                // DeepSeekR1 doesn't support thinking blocks
                                // And the AWS API demands that you strip them
                                return None;
                            }
                            let redacted =
                                BedrockThinkingBlock::RedactedContent(BedrockBlob::new(blob));

                            Some(BedrockInnerContent::ReasoningContent(redacted))
                        }
                        MessageContent::ToolUse(tool_use) => {
                            let input = if tool_use.input.is_null() {
                                // Bedrock API requires valid JsonValue, not null, for tool use input
                                value_to_aws_document(&serde_json::json!({}))
                            } else {
                                value_to_aws_document(&tool_use.input)
                            };
                            BedrockToolUseBlock::builder()
                                .name(tool_use.name.to_string())
                                .tool_use_id(tool_use.id.to_string())
                                .input(input)
                                .build()
                                .context("failed to build Bedrock tool use block")
                                .log_err()
                                .map(BedrockInnerContent::ToolUse)
                        },
                        MessageContent::ToolResult(tool_result) => {
                            BedrockToolResultBlock::builder()
                                .tool_use_id(tool_result.tool_use_id.to_string())
                                .content(match tool_result.content {
                                    LanguageModelToolResultContent::Text(text) => {
                                        BedrockToolResultContentBlock::Text(text.to_string())
                                    }
                                    LanguageModelToolResultContent::Image(_) => {
                                        BedrockToolResultContentBlock::Text(
                                            // TODO: Bedrock image support
                                            "[Tool responded with an image, but Zed doesn't support these in Bedrock models yet]".to_string()
                                        )
                                    }
                                })
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
                        _ => None,
                    })
                    .collect();
                if message.cache && supports_caching {
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

    let mut tool_spec: Vec<BedrockTool> = request
        .tools
        .iter()
        .filter_map(|tool| {
            Some(BedrockTool::ToolSpec(
                BedrockToolSpec::builder()
                    .name(tool.name.clone())
                    .description(tool.description.clone())
                    .input_schema(BedrockToolInputSchema::Json(value_to_aws_document(
                        &tool.input_schema,
                    )))
                    .build()
                    .log_err()?,
            ))
        })
        .collect();

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
    let tool_config: BedrockToolConfig = BedrockToolConfig::builder()
        .set_tools(Some(tool_spec))
        .tool_choice(tool_choice)
        .build()?;

    Ok(bedrock::Request {
        model,
        messages: new_messages,
        max_tokens: max_output_tokens,
        system: Some(system_message),
        tools: Some(tool_config),
        thinking: if request.thinking_allowed
            && let BedrockModelMode::Thinking { budget_tokens } = mode
        {
            Some(bedrock::Thinking::Enabled { budget_tokens })
        } else {
            None
        },
        metadata: None,
        stop_sequences: Vec::new(),
        temperature: request.temperature.or(Some(default_temperature)),
        top_k: None,
        top_p: None,
    })
}

// TODO: just call the ConverseOutput.usage() method:
// https://docs.rs/aws-sdk-bedrockruntime/latest/aws_sdk_bedrockruntime/operation/converse/struct.ConverseOutput.html#method.output
pub fn get_bedrock_tokens(
    request: LanguageModelRequest,
    cx: &App,
) -> BoxFuture<'static, Result<u64>> {
    cx.background_executor()
        .spawn(async move {
            let messages = request.messages;
            let mut tokens_from_images = 0;
            let mut string_messages = Vec::with_capacity(messages.len());

            for message in messages {
                use language_model::MessageContent;

                let mut string_contents = String::new();

                for content in message.content {
                    match content {
                        MessageContent::Text(text) | MessageContent::Thinking { text, .. } => {
                            string_contents.push_str(&text);
                        }
                        MessageContent::RedactedThinking(_) => {}
                        MessageContent::Image(image) => {
                            tokens_from_images += image.estimate_tokens();
                        }
                        MessageContent::ToolUse(_tool_use) => {
                            // TODO: Estimate token usage from tool uses.
                        }
                        MessageContent::ToolResult(tool_result) => match tool_result.content {
                            LanguageModelToolResultContent::Text(text) => {
                                string_contents.push_str(&text);
                            }
                            LanguageModelToolResultContent::Image(image) => {
                                tokens_from_images += image.estimate_tokens();
                            }
                        },
                    }
                }

                if !string_contents.is_empty() {
                    string_messages.push(tiktoken_rs::ChatCompletionRequestMessage {
                        role: match message.role {
                            Role::User => "user".into(),
                            Role::Assistant => "assistant".into(),
                            Role::System => "system".into(),
                        },
                        content: Some(string_contents),
                        name: None,
                        function_call: None,
                    });
                }
            }

            // Tiktoken doesn't yet support these models, so we manually use the
            // same tokenizer as GPT-4.
            tiktoken_rs::num_tokens_from_messages("gpt-4", &string_messages)
                .map(|tokens| (tokens + tokens_from_images) as u64)
        })
        .boxed()
}

pub fn map_to_language_model_completion_events(
    events: Pin<Box<dyn Send + Stream<Item = Result<BedrockStreamingResponse, BedrockError>>>>,
) -> impl Stream<Item = Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
    struct RawToolUse {
        id: String,
        name: String,
        input_json: String,
    }

    struct State {
        events: Pin<Box<dyn Send + Stream<Item = Result<BedrockStreamingResponse, BedrockError>>>>,
        tool_uses_by_index: HashMap<i32, RawToolUse>,
    }

    let initial_state = State {
        events,
        tool_uses_by_index: HashMap::default(),
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
                                }
                                None
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
                        ConverseStreamOutput::ContentBlockStop(cb_stop) => state
                            .tool_uses_by_index
                            .remove(&cb_stop.content_block_index)
                            .map(|tool_use| {
                                let input = if tool_use.input_json.is_empty() {
                                    Value::Null
                                } else {
                                    serde_json::Value::from_str(&tool_use.input_json)
                                        .unwrap_or(Value::Null)
                                };

                                Ok(LanguageModelCompletionEvent::ToolUse(
                                    LanguageModelToolUse {
                                        id: tool_use.id.into(),
                                        name: tool_use.name.into(),
                                        is_input_complete: true,
                                        raw_input: tool_use.input_json,
                                        input,
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
                            let stop_reason = match message_stop.stop_reason {
                                StopReason::ToolUse => language_model::StopReason::ToolUse,
                                _ => language_model::StopReason::EndTurn,
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

    fn should_render_editor(&self, cx: &Context<Self>) -> bool {
        self.state.read(cx).is_authenticated()
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

        if self.should_render_editor(cx) {
            return ConfiguredApiCard::new(configured_label)
                .disabled(env_var_set || is_settings_derived)
                .on_click(cx.listener(|this, _, window, cx| this.reset_credentials(window, cx)))
                .when_some(tooltip_label, |this, label| this.tooltip_label(label))
                .into_any_element();
        }

        v_flex()
            .size_full()
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::on_tab))
            .on_action(cx.listener(Self::on_tab_prev))
            .on_action(cx.listener(ConfigurationView::save_credentials))
            .child(Label::new("To use Zed's agent with Bedrock, you can set a custom authentication strategy through the settings.json, or use static credentials."))
            .child(Label::new("But, to access models on AWS, you need to:").mt_1())
            .child(
                List::new()
                    .child(
                        ListBulletItem::new("")
                            .child(Label::new("Grant permissions to the strategy you'll use according to the:"))
                            .child(ButtonLink::new("Prerequisites", "https://docs.aws.amazon.com/bedrock/latest/userguide/inference-prereq.html"))
                    )
                    .child(
                        ListBulletItem::new("")
                            .child(Label::new("Select the models you would like access to:"))
                            .child(ButtonLink::new("Bedrock Model Catalog", "https://us-east-1.console.aws.amazon.com/bedrock/home?region=us-east-1#/modelaccess"))
                    )
            )
            .child(self.render_static_credentials_ui())
            .child(
                Label::new(
                    format!("You can also set the {}, {} AND {} environment variables (or {} for Bedrock API Key authentication) and restart Zed.", ZED_BEDROCK_ACCESS_KEY_ID_VAR.name, ZED_BEDROCK_SECRET_ACCESS_KEY_VAR.name, ZED_BEDROCK_REGION_VAR.name, ZED_BEDROCK_BEARER_TOKEN_VAR.name),
                )
                    .size(LabelSize::Small)
                    .color(Color::Muted)
                    .my_1(),
            )
            .child(
                Label::new(
                    format!("Optionally, if your environment uses AWS CLI profiles, you can set {}; if it requires a custom endpoint, you can set {}; and if it requires a Session Token, you can set {}.", ZED_AWS_PROFILE_VAR.name, ZED_AWS_ENDPOINT_VAR.name, ZED_BEDROCK_SESSION_TOKEN_VAR.name),
                )
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .into_any()
    }
}

impl ConfigurationView {
    fn render_static_credentials_ui(&self) -> impl IntoElement {
        v_flex()
            .my_2()
            .tab_group()
            .gap_1p5()
            .child(
                Label::new("Static Keys")
                    .size(LabelSize::Default)
                    .weight(FontWeight::BOLD),
            )
            .child(
                Label::new(
                    "This method uses your AWS access key ID and secret access key, or a Bedrock API Key.",
                )
            )
            .child(
                List::new()
                    .child(
                        ListBulletItem::new("")
                            .child(Label::new("For access keys: Create an IAM user in the AWS console with programmatic access"))
                            .child(ButtonLink::new("IAM Console", "https://us-east-1.console.aws.amazon.com/iam/home?region=us-east-1#/users"))
                    )
                    .child(
                        ListBulletItem::new("")
                            .child(Label::new("For Bedrock API Keys: Generate an API key from the"))
                            .child(ButtonLink::new("Bedrock Console", "https://docs.aws.amazon.com/bedrock/latest/userguide/api-keys-use.html"))
                    )
                    .child(
                        ListBulletItem::new("")
                            .child(Label::new("Attach the necessary Bedrock permissions to this"))
                            .child(ButtonLink::new("user", "https://docs.aws.amazon.com/bedrock/latest/userguide/inference-prereq.html"))
                    )
                    .child(
                        ListBulletItem::new("Enter either access keys OR a Bedrock API Key below (not both)")
                    ),
            )
            .child(self.access_key_id_editor.clone())
            .child(self.secret_access_key_editor.clone())
            .child(self.session_token_editor.clone())
            .child(
                Label::new("OR")
                    .size(LabelSize::Default)
                    .weight(FontWeight::BOLD)
                    .my_1(),
            )
            .child(self.bearer_token_editor.clone())
            .child(
                Label::new(
                    format!("Region is configured via {} environment variable or settings.json (defaults to us-east-1).", ZED_BEDROCK_REGION_VAR.name),
                )
                    .size(LabelSize::Small)
                    .color(Color::Muted)
                    .mt_2(),
            )
    }
}
