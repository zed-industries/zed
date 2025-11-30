use anyhow::{Result, anyhow};
use azure_foundry::{
    AzureFoundryConfig, AzureFoundryError, AzureFoundryModelMode, ContentDelta, Event,
    ResponseContent, ToolResultContent, ToolResultPart,
};
use collections::BTreeMap;
use fs::Fs;
use futures::{FutureExt, Stream, StreamExt, future, future::BoxFuture, stream::BoxStream};
use gpui::{AnyView, App, AsyncApp, Context, Entity, Task};
use http_client::HttpClient;
use language_model::{
    AuthenticateError, ConfigurationViewTargetAgent, LanguageModel,
    LanguageModelCacheConfiguration, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelId, LanguageModelName, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest,
    LanguageModelToolChoice, LanguageModelToolResultContent, LanguageModelToolUse, MessageContent,
    RateLimiter, Role, StopReason,
};
use settings::{ModelMode, Settings, SettingsStore, update_settings_file};
use std::sync::{Arc, LazyLock};
use strum::IntoEnumIterator;
use ui::{ElevationIndex, List, prelude::*};
use ui_input::InputField;
use util::ResultExt;
use zed_env_vars::{EnvVar, env_var};

use crate::api_key::ApiKeyState;
use crate::ui::{ConfiguredApiCard, InstructionListItem};

pub use settings::AzureFoundryAvailableModel as AvailableModel;

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("azure-foundry");
const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("Azure Foundry");

fn model_mode_to_azure_foundry(value: ModelMode) -> AzureFoundryModelMode {
    match value {
        ModelMode::Default => AzureFoundryModelMode::Default,
        ModelMode::Thinking { budget_tokens } => AzureFoundryModelMode::Thinking { budget_tokens },
    }
}

fn azure_foundry_error_to_completion_error(
    error: AzureFoundryError,
) -> LanguageModelCompletionError {
    match error {
        AzureFoundryError::ApiError(error) => {
            if let Some(tokens_used) = error.match_window_exceeded() {
                LanguageModelCompletionError::PromptTooLarge {
                    tokens: Some(tokens_used),
                }
            } else if error.is_rate_limit_error() {
                LanguageModelCompletionError::RateLimitExceeded {
                    provider: PROVIDER_NAME,
                    retry_after: None,
                }
            } else {
                LanguageModelCompletionError::Other(anyhow!(
                    "{}: {}",
                    error.error_type,
                    error.message
                ))
            }
        }
        AzureFoundryError::RateLimit { retry_after } => {
            LanguageModelCompletionError::RateLimitExceeded {
                provider: PROVIDER_NAME,
                retry_after: Some(retry_after),
            }
        }
        AzureFoundryError::ServerOverloaded { retry_after } => {
            LanguageModelCompletionError::ServerOverloaded {
                provider: PROVIDER_NAME,
                retry_after,
            }
        }
        AzureFoundryError::HttpResponseError {
            status_code,
            message,
        } => {
            if status_code.as_u16() == 401 {
                LanguageModelCompletionError::AuthenticationError {
                    provider: PROVIDER_NAME,
                    message,
                }
            } else {
                LanguageModelCompletionError::HttpResponseError {
                    provider: PROVIDER_NAME,
                    status_code,
                    message,
                }
            }
        }
        AzureFoundryError::SerializeRequest(error) => {
            LanguageModelCompletionError::SerializeRequest {
                provider: PROVIDER_NAME,
                error,
            }
        }
        AzureFoundryError::BuildRequestBody(error) => {
            LanguageModelCompletionError::BuildRequestBody {
                provider: PROVIDER_NAME,
                error,
            }
        }
        AzureFoundryError::HttpSend(error) => LanguageModelCompletionError::HttpSend {
            provider: PROVIDER_NAME,
            error,
        },
        AzureFoundryError::DeserializeResponse(error) => {
            LanguageModelCompletionError::DeserializeResponse {
                provider: PROVIDER_NAME,
                error,
            }
        }
        AzureFoundryError::ReadResponse(error) => {
            LanguageModelCompletionError::ApiReadResponseError {
                provider: PROVIDER_NAME,
                error,
            }
        }
    }
}

const DEFAULT_API_URL: &str = "";

#[derive(Default, Clone, Debug, PartialEq)]
pub struct AzureFoundrySettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
}

pub struct AzureFoundryLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

const API_KEY_ENV_VAR_NAME: &str = "AZURE_FOUNDRY_API_KEY";
static API_KEY_ENV_VAR: LazyLock<EnvVar> = env_var!(API_KEY_ENV_VAR_NAME);

pub struct State {
    api_key_state: ApiKeyState,
    has_endpoint_url: bool,
}

impl State {
    fn is_authenticated(&self) -> bool {
        self.api_key_state.has_key() && self.has_endpoint_url
    }

    fn set_api_key(&mut self, api_key: Option<String>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let api_url = AzureFoundryLanguageModelProvider::api_url(cx);
        self.api_key_state
            .store(api_url, api_key, |this| &mut this.api_key_state, cx)
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let api_url = AzureFoundryLanguageModelProvider::api_url(cx);
        self.api_key_state.load_if_needed(
            api_url,
            &API_KEY_ENV_VAR,
            |this| &mut this.api_key_state,
            cx,
        )
    }
}

impl AzureFoundryLanguageModelProvider {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
        let state = cx.new(|cx| {
            cx.observe_global::<SettingsStore>(|this: &mut State, cx| {
                let api_url = Self::api_url(cx);
                this.has_endpoint_url = !api_url.is_empty();
                this.api_key_state.handle_url_change(
                    api_url,
                    &API_KEY_ENV_VAR,
                    |this| &mut this.api_key_state,
                    cx,
                );
                cx.notify();
            })
            .detach();
            let api_url = Self::api_url(cx);
            State {
                api_key_state: ApiKeyState::new(api_url.clone()),
                has_endpoint_url: !api_url.is_empty(),
            }
        });

        Self { http_client, state }
    }

    fn create_language_model(&self, model: azure_foundry::Model) -> Arc<dyn LanguageModel> {
        Arc::new(AzureFoundryModel {
            id: LanguageModelId::from(model.id().to_string()),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }

    fn settings(cx: &App) -> &AzureFoundrySettings {
        &crate::AllLanguageModelSettings::get_global(cx).azure_foundry
    }

    fn api_url(cx: &App) -> SharedString {
        let api_url = &Self::settings(cx).api_url;
        if api_url.is_empty() {
            DEFAULT_API_URL.into()
        } else {
            SharedString::new(api_url.as_str())
        }
    }
}

impl LanguageModelProviderState for AzureFoundryLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for AzureFoundryLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconName {
        IconName::AiAzureFoundry
    }

    fn default_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(azure_foundry::Model::default()))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(azure_foundry::Model::ClaudeSonnet4_5))
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let mut models = BTreeMap::default();

        for model in azure_foundry::Model::iter() {
            if !matches!(model, azure_foundry::Model::Custom { .. }) {
                models.insert(model.id().to_string(), model);
            }
        }

        // Override with available models from settings
        for model in &AzureFoundryLanguageModelProvider::settings(cx).available_models {
            models.insert(
                model.name.clone(),
                azure_foundry::Model::Custom {
                    name: model.name.clone(),
                    display_name: model.display_name.clone(),
                    max_tokens: model.max_tokens,
                    tool_override: model.tool_override.clone(),
                    cache_configuration: model.cache_configuration.as_ref().map(|config| {
                        azure_foundry::AzureFoundryModelCacheConfiguration {
                            max_cache_anchors: config.max_cache_anchors,
                            should_speculate: config.should_speculate,
                            min_total_token: config.min_total_token,
                        }
                    }),
                    max_output_tokens: model.max_output_tokens,
                    default_temperature: model.default_temperature,
                    mode: model_mode_to_azure_foundry(model.mode.unwrap_or_default()),
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
        target_agent: ConfigurationViewTargetAgent,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyView {
        cx.new(|cx| ConfigurationView::new(self.state.clone(), target_agent, window, cx))
            .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.set_api_key(None, cx))
    }
}

pub struct AzureFoundryModel {
    id: LanguageModelId,
    model: azure_foundry::Model,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl AzureFoundryModel {
    fn stream_completion(
        &self,
        request: azure_foundry::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            BoxStream<'static, Result<azure_foundry::Event, AzureFoundryError>>,
            LanguageModelCompletionError,
        >,
    > {
        let http_client = self.http_client.clone();

        let Ok((api_key, api_url)) = self.state.read_with(cx, |state, cx| {
            let api_url = AzureFoundryLanguageModelProvider::api_url(cx);
            (state.api_key_state.key(&api_url), api_url)
        }) else {
            return future::ready(Err(anyhow!("App state dropped").into())).boxed();
        };

        let model_id = self.model.request_id().to_string();

        async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey {
                    provider: PROVIDER_NAME,
                });
            };

            // Azure AI Foundry uses Anthropic-compatible endpoint for Claude models
            let endpoint_url = format!("{}/anthropic/v1/messages", api_url.trim_end_matches('/'));
            let config = AzureFoundryConfig {
                endpoint_url,
                api_key: api_key.to_string(),
                model: model_id,
            };

            let request = azure_foundry::stream_completion(http_client.as_ref(), &config, request);
            request
                .await
                .map_err(azure_foundry_error_to_completion_error)
        }
        .boxed()
    }
}

impl LanguageModel for AzureFoundryModel {
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
        true
    }

    fn supports_images(&self) -> bool {
        true
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto
            | LanguageModelToolChoice::Any
            | LanguageModelToolChoice::None => true,
        }
    }

    fn telemetry_id(&self) -> String {
        format!("azure-foundry/{}", self.model.id())
    }

    fn api_key(&self, cx: &App) -> Option<String> {
        self.state.read_with(cx, |state, cx| {
            let api_url = AzureFoundryLanguageModelProvider::api_url(cx);
            state.api_key_state.key(&api_url).map(|key| key.to_string())
        })
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
        // Use the same token counting logic as Anthropic since it's Claude models
        count_azure_foundry_tokens(request, cx)
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
        let request = into_azure_foundry(
            request,
            self.model.request_id().into(),
            self.model.default_temperature(),
            self.model.max_output_tokens(),
            self.model.mode(),
        );
        let request = self.stream_completion(request, cx);
        let future = self.request_limiter.stream(async move {
            let response = request.await?;
            Ok(AzureFoundryEventMapper::new().map_stream(response))
        });
        async move { Ok(future.await?.boxed()) }.boxed()
    }

    fn cache_configuration(&self) -> Option<LanguageModelCacheConfiguration> {
        self.model
            .cache_configuration()
            .map(|config| LanguageModelCacheConfiguration {
                max_cache_anchors: config.max_cache_anchors,
                should_speculate: config.should_speculate,
                min_total_token: config.min_total_token,
            })
    }
}

pub fn count_azure_foundry_tokens(
    request: LanguageModelRequest,
    cx: &App,
) -> BoxFuture<'static, Result<u64>> {
    cx.background_spawn(async move {
        let messages = request.messages;
        let mut tokens_from_images = 0;
        let mut string_messages = Vec::with_capacity(messages.len());

        for message in messages {
            let mut string_contents = String::new();

            for content in message.content {
                match content {
                    MessageContent::Text(text) => {
                        string_contents.push_str(&text);
                    }
                    MessageContent::Thinking { .. } => {}
                    MessageContent::RedactedThinking(_) => {}
                    MessageContent::Image(image) => {
                        tokens_from_images += image.estimate_tokens();
                    }
                    MessageContent::ToolUse(_tool_use) => {}
                    MessageContent::ToolResult(tool_result) => match &tool_result.content {
                        LanguageModelToolResultContent::Text(text) => {
                            string_contents.push_str(text);
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

        tiktoken_rs::num_tokens_from_messages("gpt-4", &string_messages)
            .map(|tokens| (tokens + tokens_from_images) as u64)
    })
    .boxed()
}

pub fn into_azure_foundry(
    request: LanguageModelRequest,
    model: String,
    default_temperature: f32,
    max_output_tokens: u64,
    mode: AzureFoundryModelMode,
) -> azure_foundry::Request {
    let mut new_messages: Vec<azure_foundry::Message> = Vec::new();
    let mut system_message = String::new();

    for message in request.messages {
        if message.contents_empty() {
            continue;
        }

        match message.role {
            Role::User | Role::Assistant => {
                let mut content: Vec<azure_foundry::RequestContent> = message
                    .content
                    .into_iter()
                    .filter_map(|content| match content {
                        MessageContent::Text(text) => {
                            let text = if text.chars().last().is_some_and(|c| c.is_whitespace()) {
                                text.trim_end().to_string()
                            } else {
                                text
                            };
                            if !text.is_empty() {
                                Some(azure_foundry::RequestContent::Text {
                                    text,
                                    cache_control: None,
                                })
                            } else {
                                None
                            }
                        }
                        MessageContent::Thinking {
                            text: thinking,
                            signature,
                        } => {
                            if !thinking.is_empty() {
                                Some(azure_foundry::RequestContent::Thinking {
                                    thinking,
                                    signature: signature.unwrap_or_default(),
                                    cache_control: None,
                                })
                            } else {
                                None
                            }
                        }
                        MessageContent::RedactedThinking(data) => {
                            if !data.is_empty() {
                                Some(azure_foundry::RequestContent::RedactedThinking { data })
                            } else {
                                None
                            }
                        }
                        MessageContent::Image(image) => {
                            Some(azure_foundry::RequestContent::Image {
                                source: azure_foundry::ImageSource {
                                    source_type: "base64".to_string(),
                                    media_type: "image/png".to_string(),
                                    data: image.source.to_string(),
                                },
                                cache_control: None,
                            })
                        }
                        MessageContent::ToolUse(tool_use) => {
                            Some(azure_foundry::RequestContent::ToolUse {
                                id: tool_use.id.to_string(),
                                name: tool_use.name.to_string(),
                                input: tool_use.input,
                                cache_control: None,
                            })
                        }
                        MessageContent::ToolResult(tool_result) => {
                            Some(azure_foundry::RequestContent::ToolResult {
                                tool_use_id: tool_result.tool_use_id.to_string(),
                                is_error: tool_result.is_error,
                                content: match tool_result.content {
                                    LanguageModelToolResultContent::Text(text) => {
                                        ToolResultContent::Plain(text.to_string())
                                    }
                                    LanguageModelToolResultContent::Image(image) => {
                                        ToolResultContent::Multipart(vec![ToolResultPart::Image {
                                            source: azure_foundry::ImageSource {
                                                source_type: "base64".to_string(),
                                                media_type: "image/png".to_string(),
                                                data: image.source.to_string(),
                                            },
                                        }])
                                    }
                                },
                                cache_control: None,
                            })
                        }
                    })
                    .collect();

                if content.is_empty() {
                    continue;
                }

                if message.cache {
                    if let Some(last) = content.last_mut() {
                        set_cache_control(last);
                    }
                }

                let role = match message.role {
                    Role::User => azure_foundry::Role::User,
                    Role::Assistant => azure_foundry::Role::Assistant,
                    Role::System => unreachable!(),
                };

                if let Some(last_message) = new_messages.last_mut() {
                    if last_message.role == role {
                        last_message.content.extend(content);
                        continue;
                    }
                }

                new_messages.push(azure_foundry::Message { role, content });
            }
            Role::System => {
                for content in message.content {
                    if let MessageContent::Text(text) = content {
                        if !system_message.is_empty() {
                            system_message.push_str("\n\n");
                        }
                        system_message.push_str(&text);
                    }
                }
            }
        }
    }

    let tools = request
        .tools
        .into_iter()
        .map(|tool| azure_foundry::Tool {
            name: tool.name,
            description: tool.description,
            input_schema: tool.input_schema,
        })
        .collect::<Vec<_>>();

    let tool_choice = request.tool_choice.map(|choice| match choice {
        LanguageModelToolChoice::Auto => azure_foundry::ToolChoice::Auto,
        LanguageModelToolChoice::Any => azure_foundry::ToolChoice::Any,
        LanguageModelToolChoice::None => azure_foundry::ToolChoice::None,
    });

    let thinking = match mode {
        AzureFoundryModelMode::Default => None,
        AzureFoundryModelMode::Thinking { budget_tokens } => {
            if request.thinking_allowed {
                Some(azure_foundry::Thinking::Enabled { budget_tokens })
            } else {
                None
            }
        }
    };

    azure_foundry::Request {
        model,
        max_tokens: max_output_tokens,
        messages: new_messages,
        system: if system_message.is_empty() {
            None
        } else {
            Some(azure_foundry::StringOrContents::String(system_message))
        },
        tools,
        tool_choice,
        thinking,
        metadata: None,
        stop_sequences: request.stop,
        temperature: request.temperature.or(Some(default_temperature)),
        top_k: None,
        top_p: None,
    }
}

fn set_cache_control(content: &mut azure_foundry::RequestContent) {
    match content {
        azure_foundry::RequestContent::Text { cache_control, .. }
        | azure_foundry::RequestContent::Thinking { cache_control, .. }
        | azure_foundry::RequestContent::Image { cache_control, .. }
        | azure_foundry::RequestContent::ToolUse { cache_control, .. }
        | azure_foundry::RequestContent::ToolResult { cache_control, .. } => {
            *cache_control = Some(azure_foundry::CacheControl {
                cache_type: azure_foundry::CacheControlType::Ephemeral,
            });
        }
        azure_foundry::RequestContent::RedactedThinking { .. } => {}
    }
}

struct AzureFoundryEventMapper {
    current_tool_use: Option<ToolUseState>,
    current_thinking_signature: Option<String>,
}

struct ToolUseState {
    id: String,
    name: String,
    input_json: String,
}

impl AzureFoundryEventMapper {
    fn new() -> Self {
        Self {
            current_tool_use: None,
            current_thinking_signature: None,
        }
    }

    fn map_stream(
        mut self,
        stream: BoxStream<'static, Result<Event, AzureFoundryError>>,
    ) -> impl Stream<Item = Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>
    {
        stream.flat_map(move |event| {
            futures::stream::iter(match event {
                Ok(event) => self.map_event(event),
                Err(error) => vec![Err(azure_foundry_error_to_completion_error(error))],
            })
        })
    }

    fn map_event(
        &mut self,
        event: Event,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        match event {
            Event::MessageStart { message } => {
                vec![Ok(LanguageModelCompletionEvent::StartMessage {
                    message_id: message.id,
                })]
            }
            Event::ContentBlockStart { content_block, .. } => match content_block {
                ResponseContent::Text { .. } => vec![],
                ResponseContent::Thinking { .. } => vec![],
                ResponseContent::RedactedThinking { data } => {
                    vec![Ok(LanguageModelCompletionEvent::RedactedThinking { data })]
                }
                ResponseContent::ToolUse { id, name, .. } => {
                    self.current_tool_use = Some(ToolUseState {
                        id,
                        name,
                        input_json: String::new(),
                    });
                    vec![]
                }
            },
            Event::ContentBlockDelta { delta, .. } => match delta {
                ContentDelta::TextDelta { text } => {
                    vec![Ok(LanguageModelCompletionEvent::Text(text))]
                }
                ContentDelta::ThinkingDelta { thinking } => {
                    vec![Ok(LanguageModelCompletionEvent::Thinking {
                        text: thinking,
                        signature: None,
                    })]
                }
                ContentDelta::SignatureDelta { signature } => {
                    self.current_thinking_signature = Some(signature);
                    vec![]
                }
                ContentDelta::InputJsonDelta { partial_json } => {
                    if let Some(ref mut tool_use) = self.current_tool_use {
                        tool_use.input_json.push_str(&partial_json);
                    }
                    vec![]
                }
            },
            Event::ContentBlockStop { .. } => {
                let mut events = vec![];

                if let Some(signature) = self.current_thinking_signature.take() {
                    events.push(Ok(LanguageModelCompletionEvent::Thinking {
                        text: String::new(),
                        signature: Some(signature),
                    }));
                }

                if let Some(tool_use) = self.current_tool_use.take() {
                    let input_json = tool_use.input_json.trim();
                    let input_value = if input_json.is_empty() {
                        Ok(serde_json::Value::Object(serde_json::Map::default()))
                    } else {
                        serde_json::from_str(input_json)
                    };
                    let event_result = match input_value {
                        Ok(input) => Ok(LanguageModelCompletionEvent::ToolUse(
                            LanguageModelToolUse {
                                id: tool_use.id.into(),
                                name: tool_use.name.into(),
                                is_input_complete: true,
                                input,
                                raw_input: tool_use.input_json.clone(),
                                thought_signature: None,
                            },
                        )),
                        Err(json_parse_err) => {
                            Ok(LanguageModelCompletionEvent::ToolUseJsonParseError {
                                id: tool_use.id.into(),
                                tool_name: tool_use.name.into(),
                                raw_input: input_json.into(),
                                json_parse_error: json_parse_err.to_string(),
                            })
                        }
                    };
                    events.push(event_result);
                }

                events
            }
            Event::MessageDelta { delta, .. } => {
                if let Some(stop_reason) = delta.stop_reason {
                    let stop_reason = match stop_reason.as_str() {
                        "end_turn" => StopReason::EndTurn,
                        "tool_use" => StopReason::ToolUse,
                        "max_tokens" => StopReason::MaxTokens,
                        _ => StopReason::EndTurn,
                    };
                    vec![Ok(LanguageModelCompletionEvent::Stop(stop_reason))]
                } else {
                    vec![]
                }
            }
            Event::MessageStop | Event::Ping => vec![],
            Event::Error { error } => {
                vec![Err(azure_foundry_error_to_completion_error(
                    AzureFoundryError::ApiError(error),
                ))]
            }
        }
    }
}

struct ConfigurationView {
    state: Entity<State>,
    target_agent: ConfigurationViewTargetAgent,
    api_key_editor: Entity<InputField>,
    endpoint_url_editor: Entity<InputField>,
    load_credentials_task: Option<Task<()>>,
}

impl ConfigurationView {
    const PLACEHOLDER_TEXT: &'static str = "sk-...";
    const PLACEHOLDER_ENDPOINT_URL: &'static str = "https://your-resource.services.ai.azure.com";

    fn new(
        state: Entity<State>,
        target_agent: ConfigurationViewTargetAgent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.observe(&state, |_, _, cx| {
            cx.notify();
        })
        .detach();

        let endpoint_url_editor = cx.new(|cx| {
            let input =
                InputField::new(window, cx, Self::PLACEHOLDER_ENDPOINT_URL).label("Endpoint URL");
            let current_url = AzureFoundryLanguageModelProvider::api_url(cx);
            if !current_url.is_empty() {
                input.set_text(current_url, window, cx);
            }
            input
        });

        let load_credentials_task = Some(cx.spawn({
            let state = state.clone();
            async move |this, cx| {
                if let Some(task) = state
                    .update(cx, |state, cx| state.authenticate(cx))
                    .log_err()
                {
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
            api_key_editor: cx
                .new(|cx| InputField::new(window, cx, Self::PLACEHOLDER_TEXT).label("API Key")),
            endpoint_url_editor,
            state,
            load_credentials_task,
            target_agent,
        }
    }

    fn save_api_key(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let api_key = self.api_key_editor.read(cx).text(cx);
        if api_key.is_empty() {
            return;
        }

        // Also save the endpoint URL if provided
        self.save_endpoint_url(cx);

        self.api_key_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state
                .update(cx, |state, cx| {
                    state.set_api_key(Some(api_key.to_string()), cx)
                })?
                .await
        })
        .detach_and_log_err(cx);
    }

    fn save_endpoint_url(&mut self, cx: &mut Context<Self>) {
        let endpoint_url = self
            .endpoint_url_editor
            .read(cx)
            .text(cx)
            .trim()
            .to_string();
        let current_url = AzureFoundryLanguageModelProvider::api_url(cx);
        if !endpoint_url.is_empty() && endpoint_url != current_url {
            let fs = <dyn Fs>::global(cx);
            update_settings_file(fs, cx, move |settings, _| {
                settings
                    .language_models
                    .get_or_insert_default()
                    .azure_foundry
                    .get_or_insert_default()
                    .api_url = Some(endpoint_url);
            });
        }
    }

    fn reset_endpoint_url(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.endpoint_url_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));

        // Immediately update state to reflect no endpoint URL
        self.state.update(cx, |state, _cx| {
            state.has_endpoint_url = false;
        });

        let fs = <dyn Fs>::global(cx);
        update_settings_file(fs, cx, |settings, _cx| {
            if let Some(settings) = settings
                .language_models
                .as_mut()
                .and_then(|models| models.azure_foundry.as_mut())
            {
                settings.api_url = None;
            }
        });
        cx.notify();
    }

    fn reset_api_key(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.api_key_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(None, cx))?
                .await
        })
        .detach_and_log_err(cx);
    }

    fn should_render_editor(&self, cx: &mut Context<Self>) -> bool {
        !self.state.read(cx).is_authenticated()
    }

    fn render_endpoint_url_editor(&self, cx: &Context<Self>) -> impl IntoElement {
        let api_url = AzureFoundryLanguageModelProvider::api_url(cx);
        let custom_url_set = !api_url.is_empty();

        if custom_url_set {
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
                        .child(v_flex().gap_1().child(Label::new(api_url))),
                )
                .child(
                    Button::new("reset-endpoint-url", "Reset")
                        .label_size(LabelSize::Small)
                        .icon(IconName::Undo)
                        .icon_size(IconSize::Small)
                        .icon_position(IconPosition::Start)
                        .layer(ElevationIndex::ModalSurface)
                        .on_click(
                            cx.listener(|this, _, window, cx| this.reset_endpoint_url(window, cx)),
                        ),
                )
                .into_any_element()
        } else {
            v_flex()
                .child(self.endpoint_url_editor.clone())
                .into_any_element()
        }
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let env_var_set = self.state.read(cx).api_key_state.is_from_env_var();
        let api_url = AzureFoundryLanguageModelProvider::api_url(cx);
        let configured_card_label = if env_var_set {
            format!("API key set in {API_KEY_ENV_VAR_NAME} environment variable")
        } else if api_url.is_empty() {
            "API key configured".to_string()
        } else {
            format!("API key configured for {}", api_url)
        };

        if self.load_credentials_task.is_some() {
            div()
                .child(Label::new("Loading credentials..."))
                .into_any_element()
        } else if self.should_render_editor(cx) {
            v_flex()
                .size_full()
                .gap_2()
                .on_action(cx.listener(Self::save_api_key))
                .child(Label::new(format!(
                    "To use {}, you need to configure your endpoint and API key:",
                    match &self.target_agent {
                        ConfigurationViewTargetAgent::ZedAgent =>
                            "Zed's agent with Azure AI Foundry".into(),
                        ConfigurationViewTargetAgent::Other(agent) => agent.clone(),
                    }
                )))
                .child(
                    List::new()
                        .child(InstructionListItem::new(
                            "Go to your project in",
                            Some("Azure AI Foundry Portal"),
                            Some("https://ai.azure.com/"),
                        ))
                        .child(InstructionListItem::text_only(
                            "Copy your endpoint URL and API key from the project settings",
                        ))
                        .child(InstructionListItem::text_only(
                            "Paste them below and hit enter to start using the agent",
                        )),
                )
                .child(self.render_endpoint_url_editor(cx))
                .child(self.api_key_editor.clone())
                .child(
                    Label::new(format!(
                        "You can also assign the {API_KEY_ENV_VAR_NAME} environment variable and restart Zed.",
                    ))
                    .size(LabelSize::Small)
                    .color(Color::Muted),
                )
                .into_any_element()
        } else {
            ConfiguredApiCard::new(configured_card_label)
                .disabled(env_var_set)
                .on_click(cx.listener(|this, _, window, cx| this.reset_api_key(window, cx)))
                .when(env_var_set, |this| {
                    this.tooltip_label(format!(
                        "To reset your API key, unset the {API_KEY_ENV_VAR_NAME} environment variable."
                    ))
                })
                .into_any_element()
        }
    }
}
