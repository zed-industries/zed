use std::pin::Pin;
use std::str::FromStr;
use std::sync::Arc;

use crate::ui::InstructionListItem;
use anyhow::{anyhow, Context as _, Result};
use aws_config::default_provider::credentials::default_provider;
use aws_config::stalled_stream_protection::StalledStreamProtectionConfig;
use aws_config::{BehaviorVersion, Region};
use aws_credential_types::Credentials;
use aws_http_client::AwsHttpClient;
use bedrock::bedrock_client::config::timeout::TimeoutConfig;
use bedrock::bedrock_client::types::{
    ContentBlockDelta, ContentBlockStart, ContentBlockStartEvent, ConverseStreamOutput,
};
use bedrock::bedrock_client::{self, Config};
use bedrock::{
    value_to_aws_document, BedrockError, BedrockInnerContent, BedrockMessage, BedrockSpecificTool,
    BedrockStreamingResponse, BedrockTool, BedrockToolChoice, BedrockToolInputSchema, Model,
};
use collections::{BTreeMap, HashMap};
use credentials_provider::CredentialsProvider;
use editor::{Editor, EditorElement, EditorStyle};
use futures::{future::BoxFuture, stream::BoxStream, FutureExt, Stream, StreamExt};
use gpui::{
    AnyView, App, AsyncApp, Context, Entity, FontStyle, Subscription, Task, TextStyle, WhiteSpace,
};
use gpui_tokio::Tokio;
use http_client::HttpClient;
use language_model::{
    AuthenticateError, LanguageModel, LanguageModelCacheConfiguration,
    LanguageModelCompletionEvent, LanguageModelId, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelRequest, LanguageModelToolUse, MessageContent, RateLimiter, Role,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use settings::{EditableSettingControl, Settings, SettingsStore};
use strum::{EnumIter, IntoEnumIterator, IntoStaticStr};
use theme::ThemeSettings;
use tokio::runtime::Handle;
use ui::{prelude::*, ContextMenu, ContextMenuEntry, DropdownMenu, Icon, IconName, List, Tooltip};
use util::{maybe, ResultExt};

use crate::AllLanguageModelSettings;

const PROVIDER_ID: &str = "amazon-bedrock";
const PROVIDER_NAME: &str = "Amazon Bedrock";

#[derive(Default, Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct BedrockCredentials {
    pub region: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: Option<String>,
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct AmazonBedrockSettings {
    pub available_models: Vec<AvailableModel>,
    pub endpoint: Option<String>,
    pub profile_name: Option<String>,
    pub role_arn: Option<String>,
    pub authentication_method: Option<BedrockAuthMethod>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EnumIter, IntoStaticStr, JsonSchema)]
pub enum BedrockAuthMethod {
    #[serde(rename = "named_profile")]
    NamedProfile,
    #[serde(rename = "static_credentials")]
    StaticCredentials,
    #[serde(rename = "sso")]
    SingleSignOn,
    #[serde(rename = "default")]
    Automatic, // IMDSv2, PodIdentity, Env vars, etc.
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AvailableModel {
    pub name: String,
    pub display_name: Option<String>,
    pub max_tokens: usize,
    pub cache_configuration: Option<LanguageModelCacheConfiguration>,
    pub max_output_tokens: Option<u32>,
    pub default_temperature: Option<f32>,
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
const ZED_AWS_PROFILE: &str = "ZED_AWS_PROFILE";
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
        cx.spawn(|this, mut cx| async move {
            credentials_provider
                .delete_credentials(AMAZON_AWS_URL, &cx)
                .await
                .log_err();
            this.update(&mut cx, |this, cx| {
                this.credentials = None;
                this.credentials_from_env = false;
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
        cx.spawn(|this, mut cx| async move {
            credentials_provider
                .write_credentials(
                    AMAZON_AWS_URL,
                    "Bearer",
                    &serde_json::to_vec(&credentials)?,
                    &cx,
                )
                .await?;
            this.update(&mut cx, |this, cx| {
                this.credentials = Some(credentials);
                cx.notify();
            })
        })
    }

    fn set_authentication_method(
        &mut self,
        auth_method: BedrockAuthMethod,
        cx: &mut Context<Self>,
    ) {
        match self.settings.as_mut() {
            None => {
                self.settings = Some(AmazonBedrockSettings {
                    authentication_method: Some(auth_method),
                    ..Default::default()
                });
            }
            Some(auth) => {
                auth.authentication_method = Some(auth_method);
            }
        }
        cx.notify();
    }

    fn is_authenticated(&self) -> bool {
        self.credentials.is_some()
    }

    fn authenticate(&self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        if self.is_authenticated() {
            return Task::ready(Ok(()));
        }

        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        cx.spawn(|this, mut cx| async move {
            let (credentials, from_env) =
                if let Ok(credentials) = std::env::var(ZED_AWS_CREDENTIALS_VAR) {
                    (credentials, true)
                } else {
                    let (_, credentials) = credentials_provider
                        .read_credentials(AMAZON_AWS_URL, &cx)
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

            this.update(&mut cx, |this, cx| {
                this.credentials = Some(credentials);
                this.credentials_from_env = from_env;
                cx.notify();
            })?;

            Ok(())
        })
    }
}

pub struct BedrockLanguageModelProvider {
    http_client: AwsHttpClient,
    handler: tokio::runtime::Handle,
    state: gpui::Entity<State>,
}

impl BedrockLanguageModelProvider {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
        let state = cx.new(|cx| State {
            credentials: None,
            settings: None,
            credentials_from_env: false,
            _subscription: cx.observe_global::<SettingsStore>(|_, cx| {
                cx.notify();
            }),
        });

        let tokio_handle = Tokio::handle(cx);

        let coerced_client = AwsHttpClient::new(http_client.clone(), tokio_handle.clone());

        Self {
            http_client: coerced_client,
            handler: tokio_handle.clone(),
            state,
        }
    }
}

impl LanguageModelProvider for BedrockLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        LanguageModelProviderId(PROVIDER_ID.into())
    }

    fn name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn icon(&self) -> IconName {
        IconName::AiBedrock
    }

    fn default_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        let model = bedrock::Model::default();
        Some(Arc::new(BedrockModel {
            id: LanguageModelId::from(model.id().to_string()),
            model,
            http_client: self.http_client.clone(),
            handler: self.handler.clone(),
            state: self.state.clone(),
            request_limiter: RateLimiter::new(4),
        }))
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
                },
            );
        }

        models
            .into_values()
            .map(|model| {
                Arc::new(BedrockModel {
                    id: LanguageModelId::from(model.id().to_string()),
                    model,
                    http_client: self.http_client.clone(),
                    handler: self.handler.clone(),
                    state: self.state.clone(),
                    request_limiter: RateLimiter::new(4),
                }) as Arc<dyn LanguageModel>
            })
            .collect()
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated()
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        self.state.update(cx, |state, cx| state.authenticate(cx))
    }

    fn configuration_view(&self, window: &mut Window, cx: &mut App) -> AnyView {
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
    handler: tokio::runtime::Handle,
    state: gpui::Entity<State>,
    request_limiter: RateLimiter,
}

impl BedrockModel {
    fn stream_completion(
        &self,
        request: bedrock::Request,
        cx: &AsyncApp,
    ) -> Result<
        BoxFuture<'static, BoxStream<'static, Result<BedrockStreamingResponse, BedrockError>>>,
    > {
        // Read state to get authentication method and other settings
        let Ok(Ok((auth_method, credentials, endpoint, region, settings))) =
            cx.read_entity(&self.state, |state, _cx| {
                // Get the authentication method and credentials
                let auth_method = state
                    .settings
                    .as_ref()
                    .and_then(|s| s.authentication_method.clone())
                    .unwrap_or(BedrockAuthMethod::Automatic);

                // Get endpoint if configured
                let endpoint = state.settings.as_ref().and_then(|s| s.endpoint.clone());

                // Get region - from credentials or directly from settings
                let region = if let Some(creds) = &state.credentials {
                    creds.region.clone()
                } else {
                    std::env::var(ZED_BEDROCK_REGION_VAR)
                        .ok()
                        .unwrap_or_else(|| ConfigurationView::PLACEHOLDER_REGION.to_string())
                };

                Ok::<(BedrockAuthMethod, Option<BedrockCredentials>, Option<String>, String, Option<AmazonBedrockSettings>), BedrockError>((
                    auth_method,
                    state.credentials.clone(),
                    endpoint,
                    region,
                    state.settings.clone(),
                ))
            })
        else {
            return Err(anyhow!("App state dropped"));
        };

        let owned_handle = self.handler.clone();
        let http_client = self.http_client.clone();

        // Configure AWS based on the authentication method
        Ok(async move {
            let mut config_builder = aws_config::defaults(BehaviorVersion::latest())
                .http_client(http_client)
                .region(Region::new(region))
                .timeout_config(TimeoutConfig::disabled());

            // Apply endpoint configuration if specified
            if let Some(endpoint_url) = endpoint {
                if !endpoint_url.is_empty() {
                    config_builder = config_builder.endpoint_url(endpoint_url);
                }
            }

            // Configure authentication based on method
            match auth_method {
                BedrockAuthMethod::StaticCredentials => {
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
                BedrockAuthMethod::NamedProfile => {
                    // Use profile from settings or environment
                    let profile_name = settings
                        .and_then(|s| s.profile_name)
                        .or_else(|| std::env::var(ZED_AWS_PROFILE).ok())
                        .unwrap_or_else(|| "default".to_string());

                    if !profile_name.is_empty() {
                        config_builder = config_builder.profile_name(profile_name);
                    }
                }
                BedrockAuthMethod::SingleSignOn => {
                    // For SSO, we rely on the default credential chain which includes SSO
                    // The user should have configured SSO via AWS CLI (`aws sso login`)
                    // TODO: Decide if we want to integrate web based SSO right here, or we extract it
                }
                BedrockAuthMethod::Automatic => {
                    // Use default credential provider chain
                }
            }

            // Load the AWS configuration
            let config = owned_handle.block_on(config_builder.load());
            let runtime_client = bedrock_client::Client::new(&config);

            // Stream completion with the configured client
            let request = bedrock::stream_completion(runtime_client, request, owned_handle);
            request.await.unwrap_or_else(|e| {
                futures::stream::once(async move { Err(BedrockError::ClientError(e)) }).boxed()
            })
        }
            .boxed())
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
        LanguageModelProviderId(PROVIDER_ID.into())
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn telemetry_id(&self) -> String {
        format!("bedrock/{}", self.model.id())
    }

    fn max_token_count(&self) -> usize {
        self.model.max_token_count()
    }

    fn max_output_tokens(&self) -> Option<u32> {
        Some(self.model.max_output_tokens())
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &App,
    ) -> BoxFuture<'static, Result<usize>> {
        get_bedrock_tokens(request, cx)
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<LanguageModelCompletionEvent>>>> {
        let request = into_bedrock(
            request,
            self.model.id().into(),
            self.model.default_temperature(),
            self.model.max_output_tokens(),
        );

        let owned_handle = self.handler.clone();

        let request = self.stream_completion(request, cx);
        let future = self.request_limiter.stream(async move {
            let response = request.map_err(|err| anyhow!(err))?.await;
            Ok(map_to_language_model_completion_events(
                response,
                owned_handle,
            ))
        });
        async move { Ok(future.await?.boxed()) }.boxed()
    }

    fn use_any_tool(
        &self,
        request: LanguageModelRequest,
        name: String,
        description: String,
        schema: Value,
        _cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        let mut request = into_bedrock(
            request,
            self.model.id().into(),
            self.model.default_temperature(),
            self.model.max_output_tokens(),
        );

        request.tool_choice = BedrockSpecificTool::builder()
            .name(name.clone())
            .build()
            .log_err()
            .map(BedrockToolChoice::Tool);

        if let Some(tool) = BedrockTool::builder()
            .name(name.clone())
            .description(description.clone())
            .input_schema(BedrockToolInputSchema::Json(value_to_aws_document(&schema)))
            .build()
            .log_err()
        {
            request.tools.push(tool);
        }

        let handle = self.handler.clone();

        let request = self.stream_completion(request, _cx);
        self.request_limiter
            .run(async move {
                let response = request.map_err(|err| anyhow!(err))?.await;
                Ok(extract_tool_args_from_events(name, response, handle)
                    .await?
                    .boxed())
            })
            .boxed()
    }

    fn cache_configuration(&self) -> Option<LanguageModelCacheConfiguration> {
        None
    }
}

pub fn into_bedrock(
    request: LanguageModelRequest,
    model: String,
    default_temperature: f32,
    max_output_tokens: u32,
) -> bedrock::Request {
    let mut new_messages: Vec<BedrockMessage> = Vec::new();
    let mut system_message = String::new();

    for message in request.messages {
        if message.contents_empty() {
            continue;
        }

        match message.role {
            Role::User | Role::Assistant => {
                let bedrock_message_content: Vec<BedrockInnerContent> = message
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
                        _ => None,
                    })
                    .collect();
                let bedrock_role = match message.role {
                    Role::User => bedrock::BedrockRole::User,
                    Role::Assistant => bedrock::BedrockRole::Assistant,
                    Role::System => unreachable!("System role should never occur here"),
                };
                if let Some(last_message) = new_messages.last_mut() {
                    if last_message.role == bedrock_role {
                        last_message.content.extend(bedrock_message_content);
                        continue;
                    }
                }
                new_messages.push(
                    BedrockMessage::builder()
                        .role(bedrock_role)
                        .set_content(Some(bedrock_message_content))
                        .build()
                        .expect("failed to build Bedrock message"),
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

    bedrock::Request {
        model,
        messages: new_messages,
        max_tokens: max_output_tokens,
        system: Some(system_message),
        tools: vec![],
        tool_choice: None,
        metadata: None,
        stop_sequences: Vec::new(),
        temperature: request.temperature.or(Some(default_temperature)),
        top_k: None,
        top_p: None,
    }
}

// TODO: just call the ConverseOutput.usage() method:
// https://docs.rs/aws-sdk-bedrockruntime/latest/aws_sdk_bedrockruntime/operation/converse/struct.ConverseOutput.html#method.output
pub fn get_bedrock_tokens(
    request: LanguageModelRequest,
    cx: &App,
) -> BoxFuture<'static, Result<usize>> {
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
                        MessageContent::Text(text) => {
                            string_contents.push_str(&text);
                        }
                        MessageContent::Image(image) => {
                            tokens_from_images += image.estimate_tokens();
                        }
                        MessageContent::ToolUse(_tool_use) => {
                            // TODO: Estimate token usage from tool uses.
                        }
                        MessageContent::ToolResult(tool_result) => {
                            string_contents.push_str(&tool_result.content);
                        }
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
                .map(|tokens| tokens + tokens_from_images)
        })
        .boxed()
}

pub async fn extract_tool_args_from_events(
    name: String,
    mut events: Pin<Box<dyn Send + Stream<Item=Result<BedrockStreamingResponse, BedrockError>>>>,
    handle: Handle,
) -> Result<impl Send + Stream<Item=Result<String>>> {
    handle
        .spawn(async move {
            let mut tool_use_index = None;
            while let Some(event) = events.next().await {
                if let BedrockStreamingResponse::ContentBlockStart(ContentBlockStartEvent {
                                                                       content_block_index,
                                                                       start,
                                                                       ..
                                                                   }) = event?
                {
                    match start {
                        None => {
                            continue;
                        }
                        Some(start) => match start.as_tool_use() {
                            Ok(tool_use) => {
                                if name == tool_use.name {
                                    tool_use_index = Some(content_block_index);
                                    break;
                                }
                            }
                            Err(err) => {
                                return Err(anyhow!("Failed to parse tool use event: {:?}", err));
                            }
                        },
                    }
                }
            }

            let Some(tool_use_index) = tool_use_index else {
                return Err(anyhow!("Tool is not used"));
            };

            Ok(events.filter_map(move |event| {
                let result = match event {
                    Err(_err) => None,
                    Ok(output) => match output.clone() {
                        BedrockStreamingResponse::ContentBlockDelta(inner) => {
                            match inner.clone().delta {
                                Some(ContentBlockDelta::ToolUse(tool_use)) => {
                                    if inner.content_block_index == tool_use_index {
                                        Some(Ok(tool_use.input))
                                    } else {
                                        None
                                    }
                                }
                                _ => None,
                            }
                        }
                        _ => None,
                    },
                };

                async move { result }
            }))
        })
        .await?
}

pub fn map_to_language_model_completion_events(
    events: Pin<Box<dyn Send + Stream<Item=Result<BedrockStreamingResponse, BedrockError>>>>,
    handle: Handle,
) -> impl Stream<Item=Result<LanguageModelCompletionEvent>> {
    struct RawToolUse {
        id: String,
        name: String,
        input_json: String,
    }

    struct State {
        events: Pin<Box<dyn Send + Stream<Item=Result<BedrockStreamingResponse, BedrockError>>>>,
        tool_uses_by_index: HashMap<i32, RawToolUse>,
    }

    futures::stream::unfold(
        State {
            events,
            tool_uses_by_index: HashMap::default(),
        },
        move |mut state: State| {
            let inner_handle = handle.clone();
            async move {
                inner_handle
                    .spawn(async {
                        while let Some(event) = state.events.next().await {
                            match event {
                                Ok(event) => match event {
                                    ConverseStreamOutput::ContentBlockDelta(cb_delta) => {
                                        if let Some(ContentBlockDelta::Text(text_out)) =
                                            cb_delta.delta
                                        {
                                            return Some((
                                                Some(Ok(LanguageModelCompletionEvent::Text(
                                                    text_out,
                                                ))),
                                                state,
                                            ));
                                        } else if let Some(ContentBlockDelta::ToolUse(text_out)) =
                                            cb_delta.delta
                                        {
                                            if let Some(tool_use) = state
                                                .tool_uses_by_index
                                                .get_mut(&cb_delta.content_block_index)
                                            {
                                                tool_use.input_json.push_str(text_out.input());
                                                return Some((None, state));
                                            };

                                            return Some((None, state));
                                        } else if cb_delta.delta.is_none() {
                                            return Some((None, state));
                                        }
                                    }
                                    ConverseStreamOutput::ContentBlockStart(cb_start) => {
                                        if let Some(start) = cb_start.start {
                                            match start {
                                                ContentBlockStart::ToolUse(text_out) => {
                                                    let tool_use = RawToolUse {
                                                        id: text_out.tool_use_id,
                                                        name: text_out.name,
                                                        input_json: String::new(),
                                                    };

                                                    state.tool_uses_by_index.insert(
                                                        cb_start.content_block_index,
                                                        tool_use,
                                                    );
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                    ConverseStreamOutput::ContentBlockStop(cb_stop) => {
                                        if let Some(tool_use) = state
                                            .tool_uses_by_index
                                            .remove(&cb_stop.content_block_index)
                                        {
                                            return Some((
                                                Some(maybe!({
                                                    Ok(LanguageModelCompletionEvent::ToolUse(
                                                        LanguageModelToolUse {
                                                            id: tool_use.id.into(),
                                                            name: tool_use.name.into(),
                                                            input: if tool_use.input_json.is_empty()
                                                            {
                                                                Value::Null
                                                            } else {
                                                                serde_json::Value::from_str(
                                                                    &tool_use.input_json,
                                                                )
                                                                .map_err(|err| anyhow!(err))?
                                                            },
                                                        },
                                                    ))
                                                })),
                                                state,
                                            ));
                                        }
                                    }
                                    _ => {}
                                },
                                Err(err) => return Some((Some(Err(anyhow!(err))), state)),
                            }
                        }
                        None
                    })
                    .await
                    .log_err()
                    .flatten()
            }
        },
    )
        .filter_map(|event| async move { event })
}

struct ConfigurationView {
    access_key_id_editor: Entity<Editor>,
    secret_access_key_editor: Entity<Editor>,
    region_editor: Entity<Editor>,
    endpoint_editor: Entity<Editor>,
    profile_name_editor: Entity<Editor>,
    start_url_editor: Entity<Editor>,
    role_arn_editor: Entity<Editor>,
    state: gpui::Entity<State>,
    load_credentials_task: Option<Task<()>>,
}

impl ConfigurationView {
    const PLACEHOLDER_ACCESS_KEY_ID_TEXT: &'static str = "XXXXXXXXXXXXXXXX";
    const PLACEHOLDER_SECRET_ACCESS_KEY_TEXT: &'static str =
        "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
    const PLACEHOLDER_PROFILE_NAME_TEXT: &'static str = "default";
    const PLACEHOLDER_REGION: &'static str = "us-east-1";
    const PLACEHOLDER_START_URL: &'static str = "https://XXXXXXXXXXX.awsapps.com/start";
    const PLACEHOLDER_ROLE_ARN: &'static str = "arn:aws:iam::XXXXXXXXXXX:role/MyRoleName";

    fn new(state: gpui::Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        cx.observe(&state, |_, _, cx| {
            cx.notify();
        })
            .detach();

        let load_credentials_task = Some(cx.spawn({
            let state = state.clone();
            |this, mut cx| async move {
                if let Some(task) = state
                    .update(&mut cx, |state, cx| state.authenticate(cx))
                    .log_err()
                {
                    // We don't log an error, because "not signed in" is also an error.
                    let _ = task.await;
                }
                this.update(&mut cx, |this, cx| {
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
            region_editor: cx.new(|cx| {
                let mut editor = Editor::single_line(window, cx);
                editor.set_placeholder_text(Self::PLACEHOLDER_REGION, cx);
                editor
            }),
            endpoint_editor: cx.new(|cx| {
                let mut editor = Editor::single_line(window, cx);
                editor.set_placeholder_text("", cx);
                editor
            }),
            profile_name_editor: cx.new(|cx| {
                let mut editor = Editor::single_line(window, cx);
                editor.set_placeholder_text(Self::PLACEHOLDER_PROFILE_NAME_TEXT, cx);
                editor
            }),
            start_url_editor: cx.new(|cx| {
                let mut editor = Editor::single_line(window, cx);
                editor.set_placeholder_text(Self::PLACEHOLDER_START_URL, cx);
                editor
            }),
            role_arn_editor: cx.new(|cx| {
                let mut editor = Editor::single_line(window, cx);
                editor.set_placeholder_text(Self::PLACEHOLDER_ROLE_ARN, cx);
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
            .to_string()
            .trim()
            .to_string();
        let secret_access_key = self
            .secret_access_key_editor
            .read(cx)
            .text(cx)
            .to_string()
            .trim()
            .to_string();
        let region = self
            .region_editor
            .read(cx)
            .text(cx)
            .to_string()
            .trim()
            .to_string();

        let state = self.state.clone();
        cx.spawn(|_, mut cx| async move {
            state
                .update(&mut cx, |state, cx| {
                    let credentials: BedrockCredentials = BedrockCredentials {
                        access_key_id: access_key_id.clone(),
                        secret_access_key: secret_access_key.clone(),
                        region: region.clone(),
                        session_token: None,
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
        self.region_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn(|_, mut cx| async move {
            state
                .update(&mut cx, |state, cx| state.reset_credentials(cx))?
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
        let border_color = cx.theme().colors().border_variant;

        h_flex()
            .w_full()
            .px_2()
            .py_1()
            .bg(bg_color)
            .border_1()
            .border_color(border_color)
            .rounded_sm()
    }

    fn should_render_editor(&self, cx: &mut Context<Self>) -> bool {
        !self.state.read(cx).is_authenticated()
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let env_var_set = self.state.read(cx).credentials_from_env;

        // Handle loading state
        if self.load_credentials_task.is_some() {
            return div().child(Label::new("Loading credentials...")).into_any();
        }

        // Handle already authenticated state
        if !self.should_render_editor(cx) {
            return h_flex()
                .size_full()
                .justify_between()
                .child(
                    h_flex()
                        .gap_1()
                        .child(Icon::new(IconName::Check).color(Color::Success))
                        .child(Label::new(if env_var_set {
                            format!("Access Key ID is set in {ZED_BEDROCK_ACCESS_KEY_ID_VAR}, Secret Key is set in {ZED_BEDROCK_SECRET_ACCESS_KEY_VAR}, Region is set in {ZED_BEDROCK_REGION_VAR} environment variables.")
                        } else {
                            "Credentials configured.".to_string()
                        })),
                )
                .child(
                    Button::new("reset-key", "Reset key")
                        .icon(Some(IconName::Trash))
                        .icon_size(IconSize::Small)
                        .icon_position(IconPosition::Start)
                        .disabled(env_var_set)
                        .when(env_var_set, |this| {
                            this.tooltip(Tooltip::text(format!("To reset your credentials, unset the {ZED_BEDROCK_ACCESS_KEY_ID_VAR}, {ZED_BEDROCK_SECRET_ACCESS_KEY_VAR}, and {ZED_BEDROCK_REGION_VAR} environment variables.")))
                        })
                        .on_click(cx.listener(|this, _, window, cx| this.reset_credentials(window, cx))),
                )
                .into_any();
        }

        // Get the current authentication method
        let authentication_method = match self.state.read(cx).settings {
            Some(ref settings) => settings
                .authentication_method
                .clone()
                .unwrap_or(BedrockAuthMethod::Automatic),
            None => BedrockAuthMethod::StaticCredentials,
        };

        // Map of auth method render functions
        let auth_method_ui = match authentication_method {
            BedrockAuthMethod::NamedProfile => self.render_named_profile_ui(cx),
            BedrockAuthMethod::StaticCredentials => self.render_static_credentials_ui(cx),
            BedrockAuthMethod::SingleSignOn => self.render_sso_ui(cx),
            BedrockAuthMethod::Automatic => self.render_automatic_ui(),
        };

        // Main configuration view
        v_flex()
            .size_full()
            .on_action(cx.listener(ConfigurationView::save_credentials))
            .child(Label::new("To use Zed's assistant with Bedrock, you need to add the Access Key ID, Secret Access Key and AWS Region. Follow these steps:"))
            .child(Label::new("Select an authentication provider based on the list below:"))
            .child(self.render_auth_method_selector(window, cx, authentication_method))
            .child(
                List::new()
                    .child(
                        InstructionListItem::new(
                            "Start by",
                            Some("creating a user and security credentials"),
                            Some("https://us-east-1.console.aws.amazon.com/iam/home"),
                        )
                    )
                    .child(
                        InstructionListItem::new(
                            "Grant that user permissions according to this documentation:",
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
            .child(auth_method_ui)
            .child(self.render_common_fields(cx))
            .child(
                Label::new(
                    format!("You can also assign the {ZED_BEDROCK_ACCESS_KEY_ID_VAR}, {ZED_BEDROCK_SECRET_ACCESS_KEY_VAR}, {ZED_BEDROCK_REGION_VAR}, AND {ZED_AWS_PROFILE} environment variables and restart Zed."),
                )
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .into_any()
    }
}

impl ConfigurationView {
    fn render_aa_id_editor(&self, cx: &mut Context<Self>) -> impl IntoElement {
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

    fn render_sk_editor(&self, cx: &mut Context<Self>) -> impl IntoElement {
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

    fn render_profile_name_editor(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let text_style = self.make_text_style(cx);

        EditorElement::new(
            &self.profile_name_editor,
            EditorStyle {
                background: cx.theme().colors().editor_background,
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }

    fn render_endpoint_editor(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let text_style = self.make_text_style(cx);

        EditorElement::new(
            &self.endpoint_editor,
            EditorStyle {
                background: cx.theme().colors().editor_background,
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }

    fn render_start_url_editor(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let text_style = self.make_text_style(cx);

        EditorElement::new(
            &self.start_url_editor,
            EditorStyle {
                background: cx.theme().colors().editor_background,
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }

    // Helper method to render the authentication method selector
    fn render_auth_method_selector(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
        current_method: BedrockAuthMethod,
    ) -> impl IntoElement {
        let context_menu = ContextMenu::build(window, cx, |mut menu, _, _| {
            for method in BedrockAuthMethod::iter() {
                let bedrock_string: &'static str = method.clone().into();
                let state = self.state.clone();
                menu = menu.custom_entry(
                    move |_, _| Label::new(bedrock_string).into_any_element(),
                    move |_, cx| {
                        state.update(cx, |state, cx| {
                            let owned_method = method.clone();
                            state.set_authentication_method(owned_method, cx);
                            cx.notify();
                        })
                    },
                );
            }
            menu
        });

        DropdownMenu::new(
            "aws-auth-selector",
            <BedrockAuthMethod as Into<&'static str>>::into(current_method),
            context_menu,
        )
    }

    // Render UI for Named Profile auth method
    fn render_named_profile_ui(&self, cx: &mut Context<Self>) -> AnyElement {
        v_flex()
            .my_2()
            .gap_1p5()
            .child(
                v_flex()
                    .gap_0p5()
                    .child(Label::new("Profile").size(LabelSize::Small))
                    .child(
                        self.make_input_styles(cx)
                            .child(self.render_profile_name_editor(cx)),
                    ),
            )
            .into_any_element()
    }

    // Render UI for Static Credentials auth method
    fn render_static_credentials_ui(&self, cx: &mut Context<Self>) -> AnyElement {
        v_flex()
            .my_2()
            .gap_1p5()
            .child(
                v_flex()
                    .gap_0p5()
                    .child(Label::new("Access Key ID").size(LabelSize::Small))
                    .child(
                        self.make_input_styles(cx)
                            .child(self.render_aa_id_editor(cx)),
                    ),
            )
            .child(
                v_flex()
                    .gap_0p5()
                    .child(Label::new("Secret Access Key").size(LabelSize::Small))
                    .child(self.make_input_styles(cx).child(self.render_sk_editor(cx))),
            )
            .into_any_element()
    }

    // Render UI for SSO auth method
    fn render_sso_ui(&self, cx: &mut Context<Self>) -> AnyElement {
        v_flex()
            .my_2()
            .gap_1p5()
            .child(
                v_flex()
                    .gap_0p5()
                    .child(Label::new("Start URL").size(LabelSize::Small))
                    .child(
                        self.make_input_styles(cx)
                            .child(self.render_start_url_editor(cx)),
                    ),
            )
            .child(
                v_flex()
                    .gap_0p5()
                    .child(Label::new("Role ARN").size(LabelSize::Small))
                    .child(
                        self.make_input_styles(cx)
                            .child(self.render_role_arn_editor(cx)),
                    ),
            )
            .into_any_element()
    }

    // Add a renderer for the role ARN editor
    fn render_role_arn_editor(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let text_style = self.make_text_style(cx);

        EditorElement::new(
            &self.role_arn_editor,
            EditorStyle {
                background: cx.theme().colors().editor_background,
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }

    // Render UI for Automatic auth method
    fn render_automatic_ui(&self) -> AnyElement {
        InstructionListItem::text_only("AWS will automatically discover credentials from environment variables, IAM roles, or EC2 instance profiles").into_any_element()
    }

    // Render common fields for all auth methods
    fn render_common_fields(&self, cx: &mut Context<Self>) -> AnyElement {
        v_flex()
            .my_2()
            .gap_1p5()
            .child(
                v_flex()
                    .gap_0p5()
                    .child(Label::new("Region").size(LabelSize::Small))
                    .child(
                        self.make_input_styles(cx)
                            .child(self.render_region_editor(cx)),
                    ),
            )
            .child(
                v_flex()
                    .gap_0p5()
                    .child(Label::new("Endpoint (Optional)").size(LabelSize::Small))
                    .child(
                        self.make_input_styles(cx)
                            .child(self.render_endpoint_editor(cx)),
                    ),
            )
            .into_any_element()
    }
}
