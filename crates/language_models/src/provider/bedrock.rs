use std::pin::Pin;
use std::str::FromStr;
use std::sync::Arc;

use crate::ui::InstructionListItem;
use anyhow::{Context as _, Result, anyhow};
use aws_config::stalled_stream_protection::StalledStreamProtectionConfig;
use aws_config::{BehaviorVersion, Region};
use aws_credential_types::Credentials;
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
use editor::{Editor, EditorElement, EditorStyle};
use futures::{FutureExt, Stream, StreamExt, future::BoxFuture, stream::BoxStream};
use gpui::{
    AnyView, App, AsyncApp, Context, Entity, FontStyle, FontWeight, Subscription, Task, TextStyle,
    WhiteSpace,
};
use gpui_tokio::Tokio;
use http_client::HttpClient;
use language_model::{
    AuthenticateError, LanguageModel, LanguageModelCacheConfiguration,
    LanguageModelCompletionError, LanguageModelCompletionEvent, LanguageModelId, LanguageModelName,
    LanguageModelProvider, LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelProviderState, LanguageModelRequest, LanguageModelToolChoice,
    LanguageModelToolResultContent, LanguageModelToolUse, MessageContent, RateLimiter, Role,
    TokenUsage,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use settings::{Settings, SettingsStore};
use smol::lock::OnceCell;
use strum::{EnumIter, IntoEnumIterator, IntoStaticStr};
use theme::ThemeSettings;
use ui::{Icon, IconName, List, Tooltip, prelude::*};
use util::ResultExt;

use crate::AllLanguageModelSettings;

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("amazon-bedrock");
const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("Amazon Bedrock");

#[derive(Default, Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct BedrockCredentials {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: Option<String>,
    pub region: String,
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct AmazonBedrockSettings {
    pub available_models: Vec<AvailableModel>,
    pub region: Option<String>,
    pub endpoint: Option<String>,
    pub profile_name: Option<String>,
    pub role_arn: Option<String>,
    pub authentication_method: Option<BedrockAuthMethod>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EnumIter, IntoStaticStr, JsonSchema)]
pub enum BedrockAuthMethod {
    #[serde(rename = "named_profile")]
    NamedProfile,
    #[serde(rename = "sso")]
    SingleSignOn,
    /// IMDSv2, PodIdentity, env vars, etc.
    #[serde(rename = "default")]
    Automatic,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AvailableModel {
    pub name: String,
    pub display_name: Option<String>,
    pub max_tokens: u64,
    pub cache_configuration: Option<LanguageModelCacheConfiguration>,
    pub max_output_tokens: Option<u64>,
    pub default_temperature: Option<f32>,
    pub mode: Option<ModelMode>,
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
const ZED_BEDROCK_ACCESS_KEY_ID_VAR: &str = "ZED_ACCESS_KEY_ID";
const ZED_BEDROCK_SECRET_ACCESS_KEY_VAR: &str = "ZED_SECRET_ACCESS_KEY";
const ZED_BEDROCK_SESSION_TOKEN_VAR: &str = "ZED_SESSION_TOKEN";
const ZED_AWS_PROFILE_VAR: &str = "ZED_AWS_PROFILE";
const ZED_BEDROCK_REGION_VAR: &str = "ZED_AWS_REGION";
const ZED_AWS_CREDENTIALS_VAR: &str = "ZED_AWS_CREDENTIALS";
const ZED_AWS_ENDPOINT_VAR: &str = "ZED_AWS_ENDPOINT";

pub struct State {
    credentials: Option<BedrockCredentials>,
    settings: Option<AmazonBedrockSettings>,
    credentials_from_env: bool,
    _subscription: Subscription,
}

impl State {
    fn reset_credentials(&self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        cx.spawn(async move |this, cx| {
            credentials_provider
                .delete_credentials(AMAZON_AWS_URL, cx)
                .await
                .log_err();
            this.update(cx, |this, cx| {
                this.credentials = None;
                this.credentials_from_env = false;
                this.settings = None;
                cx.notify();
            })
        })
    }

    fn set_credentials(
        &mut self,
        credentials: BedrockCredentials,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
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
                this.credentials = Some(credentials);
                cx.notify();
            })
        })
    }

    fn is_authenticated(&self) -> bool {
        let derived = self
            .settings
            .as_ref()
            .and_then(|s| s.authentication_method.as_ref());
        let creds = self.credentials.as_ref();

        derived.is_some() || creds.is_some()
    }

    fn authenticate(&self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        if self.is_authenticated() {
            return Task::ready(Ok(()));
        }

        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        cx.spawn(async move |this, cx| {
            let (credentials, from_env) =
                if let Ok(credentials) = std::env::var(ZED_AWS_CREDENTIALS_VAR) {
                    (credentials, true)
                } else {
                    let (_, credentials) = credentials_provider
                        .read_credentials(AMAZON_AWS_URL, cx)
                        .await?
                        .ok_or_else(|| AuthenticateError::CredentialsNotFound)?;
                    (
                        String::from_utf8(credentials)
                            .context("invalid {PROVIDER_NAME} credentials")?,
                        false,
                    )
                };

            let credentials: BedrockCredentials =
                serde_json::from_str(&credentials).context("failed to parse credentials")?;

            this.update(cx, |this, cx| {
                this.credentials = Some(credentials);
                this.credentials_from_env = from_env;
                cx.notify();
            })?;

            Ok(())
        })
    }

    fn get_region(&self) -> String {
        // Get region - from credentials or directly from settings
        let credentials_region = self.credentials.as_ref().map(|s| s.region.clone());
        let settings_region = self.settings.as_ref().and_then(|s| s.region.clone());

        // Use credentials region if available, otherwise use settings region, finally fall back to default
        credentials_region
            .or(settings_region)
            .unwrap_or(String::from("us-east-1"))
    }
}

pub struct BedrockLanguageModelProvider {
    http_client: AwsHttpClient,
    handle: tokio::runtime::Handle,
    state: gpui::Entity<State>,
}

impl BedrockLanguageModelProvider {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
        let state = cx.new(|cx| State {
            credentials: None,
            settings: Some(AllLanguageModelSettings::get_global(cx).bedrock.clone()),
            credentials_from_env: false,
            _subscription: cx.observe_global::<SettingsStore>(|_, cx| {
                cx.notify();
            }),
        });

        Self {
            http_client: AwsHttpClient::new(http_client.clone()),
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

    fn icon(&self) -> IconName {
        IconName::AiBedrock
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
                // TODO: Sonnet 3.7 vs. 3.7 Thinking bug is here.
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
        self.state
            .update(cx, |state, cx| state.reset_credentials(cx))
    }
}

impl LanguageModelProviderState for BedrockLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<gpui::Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

struct BedrockModel {
    id: LanguageModelId,
    model: Model,
    http_client: AwsHttpClient,
    handle: tokio::runtime::Handle,
    client: OnceCell<BedrockClient>,
    state: gpui::Entity<State>,
    request_limiter: RateLimiter,
}

impl BedrockModel {
    fn get_or_init_client(&self, cx: &AsyncApp) -> anyhow::Result<&BedrockClient> {
        self.client
            .get_or_try_init_blocking(|| {
                let (auth_method, credentials, endpoint, region, settings) =
                    cx.read_entity(&self.state, |state, _cx| {
                        let auth_method = state
                            .settings
                            .as_ref()
                            .and_then(|s| s.authentication_method.clone());

                        let endpoint = state.settings.as_ref().and_then(|s| s.endpoint.clone());

                        let region = state.get_region();

                        (
                            auth_method,
                            state.credentials.clone(),
                            endpoint,
                            region,
                            state.settings.clone(),
                        )
                    })?;

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

                match auth_method {
                    None => {
                        if let Some(creds) = credentials {
                            let aws_creds = Credentials::new(
                                creds.access_key_id,
                                creds.secret_access_key,
                                creds.session_token,
                                None,
                                "zed-bedrock-provider",
                            );
                            config_builder = config_builder.credentials_provider(aws_creds);
                        }
                    }
                    Some(BedrockAuthMethod::NamedProfile)
                    | Some(BedrockAuthMethod::SingleSignOn) => {
                        // Currently NamedProfile and SSO behave the same way but only the instructions change
                        // Until we support BearerAuth through SSO, this will not change.
                        let profile_name = settings
                            .and_then(|s| s.profile_name)
                            .unwrap_or_else(|| "default".to_string());

                        if !profile_name.is_empty() {
                            config_builder = config_builder.profile_name(profile_name);
                        }
                    }
                    Some(BedrockAuthMethod::Automatic) => {
                        // Use default credential provider chain
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

        match Tokio::spawn(cx, bedrock::stream_completion(runtime_client, request)) {
            Ok(res) => async { res.await.map_err(|err| anyhow!(err))? }.boxed(),
            Err(err) => futures::future::ready(Err(anyhow!(err))).boxed(),
        }
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
        let Ok(region) = cx.read_entity(&self.state, |state, _cx| state.get_region()) else {
            return async move { Err(anyhow::anyhow!("App State Dropped").into()) }.boxed();
        };

        let model_id = match self.model.cross_region_inference_id(&region) {
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
    access_key_id_editor: Entity<Editor>,
    secret_access_key_editor: Entity<Editor>,
    session_token_editor: Entity<Editor>,
    region_editor: Entity<Editor>,
    state: gpui::Entity<State>,
    load_credentials_task: Option<Task<()>>,
}

impl ConfigurationView {
    const PLACEHOLDER_ACCESS_KEY_ID_TEXT: &'static str = "XXXXXXXXXXXXXXXX";
    const PLACEHOLDER_SECRET_ACCESS_KEY_TEXT: &'static str =
        "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
    const PLACEHOLDER_SESSION_TOKEN_TEXT: &'static str = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
    const PLACEHOLDER_REGION: &'static str = "us-east-1";

    fn new(state: gpui::Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        cx.observe(&state, |_, _, cx| {
            cx.notify();
        })
        .detach();

        let load_credentials_task = Some(cx.spawn({
            let state = state.clone();
            async move |this, cx| {
                if let Some(task) = state
                    .update(cx, |state, cx| state.authenticate(cx))
                    .log_err()
                {
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
            access_key_id_editor: cx.new(|cx| {
                let mut editor = Editor::single_line(window, cx);
                editor.set_placeholder_text(Self::PLACEHOLDER_ACCESS_KEY_ID_TEXT, cx);
                editor
            }),
            secret_access_key_editor: cx.new(|cx| {
                let mut editor = Editor::single_line(window, cx);
                editor.set_placeholder_text(Self::PLACEHOLDER_SECRET_ACCESS_KEY_TEXT, cx);
                editor
            }),
            session_token_editor: cx.new(|cx| {
                let mut editor = Editor::single_line(window, cx);
                editor.set_placeholder_text(Self::PLACEHOLDER_SESSION_TOKEN_TEXT, cx);
                editor
            }),
            region_editor: cx.new(|cx| {
                let mut editor = Editor::single_line(window, cx);
                editor.set_placeholder_text(Self::PLACEHOLDER_REGION, cx);
                editor
            }),
            state,
            load_credentials_task,
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
        let region = self.region_editor.read(cx).text(cx).trim().to_string();
        let region = if region.is_empty() {
            "us-east-1".to_string()
        } else {
            region
        };

        let state = self.state.clone();
        cx.spawn(async move |_, cx| {
            state
                .update(cx, |state, cx| {
                    let credentials: BedrockCredentials = BedrockCredentials {
                        region: region.clone(),
                        access_key_id: access_key_id.clone(),
                        secret_access_key: secret_access_key.clone(),
                        session_token: session_token.clone(),
                    };

                    state.set_credentials(credentials, cx)
                })?
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
        self.region_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn(async move |_, cx| {
            state
                .update(cx, |state, cx| state.reset_credentials(cx))?
                .await
        })
        .detach_and_log_err(cx);
    }

    fn make_text_style(&self, cx: &Context<Self>) -> TextStyle {
        let settings = ThemeSettings::get_global(cx);
        TextStyle {
            color: cx.theme().colors().text,
            font_family: settings.ui_font.family.clone(),
            font_features: settings.ui_font.features.clone(),
            font_fallbacks: settings.ui_font.fallbacks.clone(),
            font_size: rems(0.875).into(),
            font_weight: settings.ui_font.weight,
            font_style: FontStyle::Normal,
            line_height: relative(1.3),
            background_color: None,
            underline: None,
            strikethrough: None,
            white_space: WhiteSpace::Normal,
            text_overflow: None,
            text_align: Default::default(),
            line_clamp: None,
        }
    }

    fn make_input_styles(&self, cx: &Context<Self>) -> Div {
        let bg_color = cx.theme().colors().editor_background;
        let border_color = cx.theme().colors().border;

        h_flex()
            .w_full()
            .px_2()
            .py_1()
            .bg(bg_color)
            .border_1()
            .border_color(border_color)
            .rounded_sm()
    }

    fn should_render_editor(&self, cx: &Context<Self>) -> bool {
        self.state.read(cx).is_authenticated()
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let env_var_set = self.state.read(cx).credentials_from_env;
        let bedrock_settings = self.state.read(cx).settings.as_ref();
        let bedrock_method = bedrock_settings
            .as_ref()
            .and_then(|s| s.authentication_method.clone());

        if self.load_credentials_task.is_some() {
            return div().child(Label::new("Loading credentials...")).into_any();
        }

        if self.should_render_editor(cx) {
            return h_flex()
                .mt_1()
                .p_1()
                .justify_between()
                .rounded_md()
                .border_1()
                .border_color(cx.theme().colors().border)
                .bg(cx.theme().colors().background)
                .child(
                    h_flex()
                        .gap_1()
                        .child(Icon::new(IconName::Check).color(Color::Success))
                        .child(Label::new(if env_var_set {
                            format!("Access Key ID is set in {ZED_BEDROCK_ACCESS_KEY_ID_VAR}, Secret Key is set in {ZED_BEDROCK_SECRET_ACCESS_KEY_VAR}, Region is set in {ZED_BEDROCK_REGION_VAR} environment variables.")
                        } else {
                            match bedrock_method {
                                Some(BedrockAuthMethod::Automatic) => "You are using automatic credentials".into(),
                                Some(BedrockAuthMethod::NamedProfile) => {
                                    "You are using named profile".into()
                                },
                                Some(BedrockAuthMethod::SingleSignOn) => "You are using a single sign on profile".into(),
                                None => "You are using static credentials".into(),
                            }
                        })),
                )
                .child(
                    Button::new("reset-key", "Reset Key")
                        .icon(Some(IconName::Trash))
                        .icon_size(IconSize::Small)
                        .icon_position(IconPosition::Start)
                        .disabled(env_var_set || bedrock_method.is_some())
                        .when(env_var_set, |this| {
                            this.tooltip(Tooltip::text(format!("To reset your credentials, unset the {ZED_BEDROCK_ACCESS_KEY_ID_VAR}, {ZED_BEDROCK_SECRET_ACCESS_KEY_VAR}, and {ZED_BEDROCK_REGION_VAR} environment variables.")))
                        })
                        .when(bedrock_method.is_some(), |this| {
                            this.tooltip(Tooltip::text("You cannot reset credentials as they're being derived, check Zed settings to understand how"))
                        })
                        .on_click(cx.listener(|this, _, window, cx| this.reset_credentials(window, cx))),
                )
                .into_any();
        }

        v_flex()
            .size_full()
            .on_action(cx.listener(ConfigurationView::save_credentials))
            .child(Label::new("To use Zed's agent with Bedrock, you can set a custom authentication strategy through the settings.json, or use static credentials."))
            .child(Label::new("But, to access models on AWS, you need to:").mt_1())
            .child(
                List::new()
                    .child(
                        InstructionListItem::new(
                            "Grant permissions to the strategy you'll use according to the:",
                            Some("Prerequisites"),
                            Some("https://docs.aws.amazon.com/bedrock/latest/userguide/inference-prereq.html"),
                        )
                    )
                    .child(
                        InstructionListItem::new(
                            "Select the models you would like access to:",
                            Some("Bedrock Model Catalog"),
                            Some("https://us-east-1.console.aws.amazon.com/bedrock/home?region=us-east-1#/modelaccess"),
                        )
                    )
            )
            .child(self.render_static_credentials_ui(cx))
            .child(self.render_common_fields(cx))
            .child(
                Label::new(
                    format!("You can also assign the {ZED_BEDROCK_ACCESS_KEY_ID_VAR}, {ZED_BEDROCK_SECRET_ACCESS_KEY_VAR} AND {ZED_BEDROCK_REGION_VAR} environment variables and restart Zed."),
                )
                    .size(LabelSize::Small)
                    .color(Color::Muted)
                    .my_1(),
            )
            .child(
                Label::new(
                    format!("Optionally, if your environment uses AWS CLI profiles, you can set {ZED_AWS_PROFILE_VAR}; if it requires a custom endpoint, you can set {ZED_AWS_ENDPOINT_VAR}; and if it requires a Session Token, you can set {ZED_BEDROCK_SESSION_TOKEN_VAR}."),
                )
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .into_any()
    }
}

impl ConfigurationView {
    fn render_access_key_id_editor(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let text_style = self.make_text_style(cx);

        EditorElement::new(
            &self.access_key_id_editor,
            EditorStyle {
                background: cx.theme().colors().editor_background,
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }

    fn render_secret_key_editor(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let text_style = self.make_text_style(cx);

        EditorElement::new(
            &self.secret_access_key_editor,
            EditorStyle {
                background: cx.theme().colors().editor_background,
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }

    fn render_session_token_editor(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let text_style = self.make_text_style(cx);

        EditorElement::new(
            &self.session_token_editor,
            EditorStyle {
                background: cx.theme().colors().editor_background,
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }

    fn render_region_editor(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let text_style = self.make_text_style(cx);

        EditorElement::new(
            &self.region_editor,
            EditorStyle {
                background: cx.theme().colors().editor_background,
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }

    fn render_static_credentials_ui(&self, cx: &mut Context<Self>) -> AnyElement {
        v_flex()
            .my_2()
            .gap_1p5()
            .child(
                Label::new("Static Keys")
                    .size(LabelSize::Default)
                    .weight(FontWeight::BOLD),
            )
            .child(
                Label::new(
                    "This method uses your AWS access key ID and secret access key directly.",
                )
            )
            .child(
                List::new()
                    .child(InstructionListItem::new(
                        "Create an IAM user in the AWS console with programmatic access",
                        Some("IAM Console"),
                        Some("https://us-east-1.console.aws.amazon.com/iam/home?region=us-east-1#/users"),
                    ))
                    .child(InstructionListItem::new(
                        "Attach the necessary Bedrock permissions to this ",
                        Some("user"),
                        Some("https://docs.aws.amazon.com/bedrock/latest/userguide/inference-prereq.html"),
                    ))
                    .child(InstructionListItem::text_only(
                        "Copy the access key ID and secret access key when provided",
                    ))
                    .child(InstructionListItem::text_only(
                        "Enter these credentials below",
                    )),
            )
            .child(
                v_flex()
                    .gap_0p5()
                    .child(Label::new("Access Key ID").size(LabelSize::Small))
                    .child(
                        self.make_input_styles(cx)
                            .child(self.render_access_key_id_editor(cx)),
                    ),
            )
            .child(
                v_flex()
                    .gap_0p5()
                    .child(Label::new("Secret Access Key").size(LabelSize::Small))
                    .child(self.make_input_styles(cx).child(self.render_secret_key_editor(cx))),
            )
            .child(
                v_flex()
                    .gap_0p5()
                    .child(Label::new("Session Token (Optional)").size(LabelSize::Small))
                    .child(
                        self.make_input_styles(cx)
                            .child(self.render_session_token_editor(cx)),
                    ),
            )
            .into_any_element()
    }

    fn render_common_fields(&self, cx: &mut Context<Self>) -> AnyElement {
        v_flex()
            .gap_0p5()
            .child(Label::new("Region").size(LabelSize::Small))
            .child(
                self.make_input_styles(cx)
                    .child(self.render_region_editor(cx)),
            )
            .into_any_element()
    }
}
