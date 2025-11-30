use crate::AllLanguageModelSettings;
use crate::ui::InstructionListItem;
use anthropic::AnthropicError;
use anthropic_vertex_ai::{
    ContentDelta, Event, ResponseContent, ToolResultContent, ToolResultPart, Usage,
};
use anyhow::{Result, anyhow};
use collections::{BTreeMap, HashMap};
use credentials_provider::CredentialsProvider;
use futures::Stream;
use futures::{FutureExt, StreamExt, future::BoxFuture, stream::BoxStream};
use gpui::{AnyView, App, AsyncApp, Context, Subscription, Task};
use http_client::HttpClient;
use language_model::{
    AuthenticateError, ConfigurationViewTargetAgent, LanguageModel,
    LanguageModelCacheConfiguration, LanguageModelCompletionError, LanguageModelId,
    LanguageModelName, LanguageModelProvider, LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelProviderState, LanguageModelRequest, LanguageModelToolChoice,
    LanguageModelToolResultContent, MessageContent, RateLimiter, Role,
};
use language_model::{LanguageModelCompletionEvent, LanguageModelToolUse, StopReason};
pub use settings::AnthropicVertexAvailableModel as AvailableModel;
use settings::{ModelMode, Settings, SettingsStore};
use std::pin::Pin;
use std::str::FromStr;
use std::sync::Arc;
use strum::IntoEnumIterator;
use ui::{Icon, IconName, List, Tooltip, prelude::*};
use util::ResultExt;

const PROVIDER_ID: &str = "anthropic-vertex-ai";
const PROVIDER_NAME: &str = "Anthropic Vertex AI";

#[derive(Default, Clone, Debug, PartialEq)]
pub struct AnthropicVertexSettings {
    pub api_url: String,
    pub project_id: String,  // ADDED
    pub location_id: String, // ADDED
    pub available_models: Vec<AvailableModel>,
}

pub struct AnthropicVertexLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: gpui::Entity<State>,
}

pub struct State {
    api_key: Option<String>,
    api_key_from_env: bool,
    _subscription: Subscription,
}

impl State {
    fn is_authenticated(&self) -> bool {
        self.api_key.is_some()
    }

    fn reset_api_key(&self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        // Ensure api_url, project_id, and location_id are available for credentials deletion
        let settings = AllLanguageModelSettings::get_global(cx)
            .google_vertex
            .clone();

        cx.spawn(async move |this, cx| {
            credentials_provider
                .delete_credentials(&settings.api_url, &cx) // Use api_url
                .await
                .log_err();
            this.update(cx, |this, cx| {
                this.api_key = None;
                this.api_key_from_env = false;
                cx.notify();
            })
        })
    }

    fn authenticate(&self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        log::info!("Authenticating Google Vertex AI...");

        if self.is_authenticated() {
            return Task::ready(Ok(()));
        }

        // The Tokio runtime provided by `gpui::spawn` is not sufficient for `tokio::process`
        // or `tokio::task::spawn_blocking`. We must fall back to the standard library's threading
        // to run the synchronous `gcloud` command, and use a channel to communicate the
        // result back to our async context.
        cx.spawn(async move |this, cx| {
            let (tx, rx) = futures::channel::oneshot::channel();

            std::thread::spawn(move || {
                let result = std::process::Command::new("gcloud")
                    .args(&["auth", "application-default", "print-access-token"])
                    .output()
                    .map_err(|e| {
                        AuthenticateError::Other(anyhow!("Failed to execute gcloud command: {}", e))
                    });

                // Send the result back to the async task, ignoring if the receiver was dropped.
                let _ = tx.send(result);
            });

            // Await the result from the channel.
            // First, explicitly handle the channel's `Canceled` error.
            // Then, use `?` to propagate the `AuthenticateError` from the command execution.
            let token_output = rx.await.map_err(|_cancelled| {
                AuthenticateError::Other(anyhow!("Authentication task was cancelled"))
            })??;

            // Retrieve the access token from the gcloud command output.
            // Ensure UTF-8 decoding and trim whitespace.
            let access_token = String::from_utf8(token_output.stdout)
                .map_err(|e| {
                    AuthenticateError::Other(anyhow!("Invalid UTF-8 in gcloud output: {}", e))
                })?
                .trim()
                .to_string();

            // Check the exit status of the gcloud command.
            if !token_output.status.success() {
                let stderr = String::from_utf8_lossy(&token_output.stderr).into_owned();
                return Err(AuthenticateError::Other(anyhow!(
                    "gcloud command failed: {}",
                    stderr
                )));
            }

            let api_key = access_token; // Use the retrieved token as the API key.
            let from_env = false; // This token is dynamically fetched, not from env or keychain.

            this.update(cx, |this, cx| {
                this.api_key = Some(api_key);
                this.api_key_from_env = from_env;
                cx.notify();
            })?;

            Ok(())
        })
    }
}

impl AnthropicVertexLanguageModelProvider {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
        let state = cx.new(|cx| State {
            api_key: None,
            api_key_from_env: false,
            _subscription: cx.observe_global::<SettingsStore>(|_, cx| {
                cx.notify();
            }),
        });

        Self { http_client, state }
    }

    fn create_language_model(&self, model: anthropic_vertex_ai::Model) -> Arc<dyn LanguageModel> {
        Arc::new(AnthropicVertexModel {
            id: LanguageModelId::from(model.id().to_string()),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }
}

impl LanguageModelProviderState for AnthropicVertexLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<gpui::Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for AnthropicVertexLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        LanguageModelProviderId(PROVIDER_ID.into())
    }

    fn name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn icon(&self) -> IconName {
        IconName::AiAnthropic
    }

    fn default_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(anthropic_vertex_ai::Model::default()))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(anthropic_vertex_ai::Model::default_fast()))
    }

    fn recommended_models(&self, _cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        [
            anthropic_vertex_ai::Model::ClaudeSonnet4,
            anthropic_vertex_ai::Model::ClaudeSonnet4Thinking,
        ]
        .into_iter()
        .map(|model| self.create_language_model(model))
        .collect()
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let mut models = BTreeMap::default();

        // Add base models from anthropic_vertex_ai::Model::iter()
        for model in anthropic_vertex_ai::Model::iter() {
            if !matches!(model, anthropic_vertex_ai::Model::Custom { .. }) {
                models.insert(model.id().to_string(), model);
            }
        }

        // Override with available models from settings
        for model in AllLanguageModelSettings::get_global(cx)
            .anthropic_vertex
            .available_models
            .iter()
        {
            models.insert(
                model.name.clone(),
                anthropic_vertex_ai::Model::Custom {
                    name: model.name.clone(),
                    display_name: model.display_name.clone(),
                    max_tokens: model.max_tokens,
                    tool_override: model.tool_override.clone(),
                    cache_configuration: model.cache_configuration.as_ref().map(|config| {
                        anthropic_vertex_ai::AnthropicVertexModelCacheConfiguration {
                            max_cache_anchors: config.max_cache_anchors,
                            should_speculate: config.should_speculate,
                            min_total_token: config.min_total_token,
                        }
                    }),
                    max_output_tokens: model.max_output_tokens,
                    default_temperature: model.default_temperature,
                    mode: model.mode.clone().unwrap(),
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
        _configuration_view_target_agent: ConfigurationViewTargetAgent,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyView {
        cx.new(|cx| ConfigurationView::new(self.state.clone(), window, cx))
            .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state.update(cx, |state, cx| state.reset_api_key(cx))
    }
}

pub struct AnthropicVertexModel {
    id: LanguageModelId,
    model: anthropic_vertex_ai::Model,
    state: gpui::Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

pub fn count_anthropic_tokens(
    request: LanguageModelRequest,
    cx: &App,
) -> BoxFuture<'static, Result<u64>> {
    cx.background_spawn(async move {
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
                    MessageContent::Thinking { .. } => {
                        // Thinking blocks are not included in the input token count.
                    }
                    MessageContent::RedactedThinking(_) => {
                        // Thinking blocks are not included in the input token count.
                    }
                    MessageContent::Image(image) => {
                        tokens_from_images += image.estimate_tokens();
                    }
                    MessageContent::ToolUse(_tool_use) => {
                        // TODO: Estimate token usage from tool uses.
                    }
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

        // Tiktoken doesn't yet support these models, so we manually use the
        // same tokenizer as GPT-4.
        tiktoken_rs::num_tokens_from_messages("gpt-4", &string_messages)
            .map(|tokens| (tokens + tokens_from_images) as u64)
    })
    .boxed()
}

impl AnthropicVertexModel {
    fn stream_completion(
        &self,
        request: anthropic_vertex_ai::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            BoxStream<'static, Result<anthropic_vertex_ai::Event, AnthropicError>>,
            LanguageModelCompletionError,
        >,
    > {
        let http_client = self.http_client.clone();

        let Ok((access_token_option, api_url, project_id, location_id)) =
            cx.read_entity(&self.state, |state, cx| {
                let settings = &AllLanguageModelSettings::get_global(cx).anthropic_vertex;
                (
                    state.api_key.clone(), // This is the access token for Vertex AI
                    settings.api_url.clone(),
                    settings.project_id.clone(),  // ADDED
                    settings.location_id.clone(), // ADDED
                )
            })
        else {
            return futures::future::ready(Err(anyhow!("App state dropped").into())).boxed();
        };

        async move {
            let access_token = access_token_option.unwrap();
            let request = anthropic_vertex_ai::stream_completion(
                http_client.as_ref(),
                &api_url,
                &project_id,  // ADDED
                &location_id, // ADDED
                &access_token,
                request,
            );
            request.await.map_err(Into::into)
        }
        .boxed()
    }
}

impl LanguageModel for AnthropicVertexModel {
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
        format!("anthropic/{}", self.model.id())
    }

    fn api_key(&self, cx: &App) -> Option<String> {
        self.state.read(cx).api_key.clone()
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
        count_anthropic_tokens(request, cx)
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
        let request = into_anthropic(
            request,
            self.model.request_id().into(),
            self.model.default_temperature(),
            self.model.max_output_tokens(),
            self.model.mode(),
        );
        let request = self.stream_completion(request, cx);
        let future = self.request_limiter.stream(async move {
            let response = request.await?;
            Ok(AnthropicVertexEventMapper::new().map_stream(response))
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

pub fn into_anthropic(
    request: LanguageModelRequest,
    model: String,
    default_temperature: f32,
    max_output_tokens: u64,
    mode: ModelMode,
) -> anthropic_vertex_ai::Request {
    let mut new_messages: Vec<anthropic_vertex_ai::Message> = Vec::new();
    let mut system_message = String::new();

    for message in request.messages {
        if message.contents_empty() {
            continue;
        }

        match message.role {
            Role::User | Role::Assistant => {
                let mut anthropic_message_content: Vec<anthropic_vertex_ai::RequestContent> =
                    message
                        .content
                        .into_iter()
                        .filter_map(|content| match content {
                            MessageContent::Text(text) => {
                                let text =
                                    if text.chars().last().map_or(false, |c| c.is_whitespace()) {
                                        text.trim_end().to_string()
                                    } else {
                                        text
                                    };
                                if !text.is_empty() {
                                    Some(anthropic_vertex_ai::RequestContent::Text {
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
                                    Some(anthropic_vertex_ai::RequestContent::Thinking {
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
                                    Some(anthropic_vertex_ai::RequestContent::RedactedThinking {
                                        data,
                                    })
                                } else {
                                    None
                                }
                            }
                            MessageContent::Image(image) => {
                                Some(anthropic_vertex_ai::RequestContent::Image {
                                    source: anthropic_vertex_ai::ImageSource {
                                        source_type: "base64".to_string(),
                                        media_type: "image/png".to_string(),
                                        data: image.source.to_string(),
                                    },
                                    cache_control: None,
                                })
                            }
                            MessageContent::ToolUse(tool_use) => {
                                Some(anthropic_vertex_ai::RequestContent::ToolUse {
                                    id: tool_use.id.to_string(),
                                    name: tool_use.name.to_string(),
                                    input: tool_use.input,
                                    cache_control: None,
                                })
                            }
                            MessageContent::ToolResult(tool_result) => {
                                Some(anthropic_vertex_ai::RequestContent::ToolResult {
                                    tool_use_id: tool_result.tool_use_id.to_string(),
                                    is_error: tool_result.is_error,
                                    content: match tool_result.content {
                                        LanguageModelToolResultContent::Text(text) => {
                                            ToolResultContent::Plain(text.to_string())
                                        }
                                        LanguageModelToolResultContent::Image(image) => {
                                            ToolResultContent::Multipart(vec![
                                                ToolResultPart::Image {
                                                    source: anthropic_vertex_ai::ImageSource {
                                                        source_type: "base64".to_string(),
                                                        media_type: "image/png".to_string(),
                                                        data: image.source.to_string(),
                                                    },
                                                },
                                            ])
                                        }
                                    },
                                    cache_control: None,
                                })
                            }
                        })
                        .collect();
                let anthropic_role = match message.role {
                    Role::User => anthropic_vertex_ai::Role::User,
                    Role::Assistant => anthropic_vertex_ai::Role::Assistant,
                    Role::System => unreachable!("System role should never occur here"),
                };
                if let Some(last_message) = new_messages.last_mut() {
                    if last_message.role == anthropic_role {
                        last_message.content.extend(anthropic_message_content);
                        continue;
                    }
                }

                // Mark the last segment of the message as cached
                if message.cache {
                    let cache_control_value = Some(anthropic_vertex_ai::CacheControl {
                        cache_type: anthropic_vertex_ai::CacheControlType::Ephemeral,
                    });
                    for message_content in anthropic_message_content.iter_mut().rev() {
                        match message_content {
                            anthropic_vertex_ai::RequestContent::RedactedThinking { .. } => {
                                // Caching is not possible, fallback to next message
                            }
                            anthropic_vertex_ai::RequestContent::Text { cache_control, .. }
                            | anthropic_vertex_ai::RequestContent::Thinking {
                                cache_control, ..
                            }
                            | anthropic_vertex_ai::RequestContent::Image {
                                cache_control, ..
                            }
                            | anthropic_vertex_ai::RequestContent::ToolUse {
                                cache_control, ..
                            }
                            | anthropic_vertex_ai::RequestContent::ToolResult {
                                cache_control,
                                ..
                            } => {
                                *cache_control = cache_control_value;
                                break;
                            }
                        }
                    }
                }

                new_messages.push(anthropic_vertex_ai::Message {
                    role: anthropic_role,
                    content: anthropic_message_content,
                });
            }
            Role::System => {
                if !system_message.is_empty() {
                    system_message.push_str("\n\n");
                }
                system_message.push_str(&message.string_contents());
            }
        }
    }

    anthropic_vertex_ai::Request {
        model: model,
        anthropic_version: "vertex-2023-10-16".to_string(),
        messages: new_messages,
        max_tokens: max_output_tokens,
        system: if system_message.is_empty() {
            None
        } else {
            Some(anthropic_vertex_ai::StringOrContents::String(
                system_message,
            ))
        },
        thinking: if request.thinking_allowed
            && let ModelMode::Thinking { budget_tokens } = mode
        {
            Some(anthropic_vertex_ai::Thinking::Enabled { budget_tokens })
        } else {
            None
        },
        tools: request
            .tools
            .into_iter()
            .map(|tool| anthropic_vertex_ai::Tool {
                name: tool.name,
                description: tool.description,
                input_schema: tool.input_schema,
            })
            .collect(),
        tool_choice: request.tool_choice.map(|choice| match choice {
            LanguageModelToolChoice::Auto => anthropic_vertex_ai::ToolChoice::Auto,
            LanguageModelToolChoice::Any => anthropic_vertex_ai::ToolChoice::Any,
            LanguageModelToolChoice::None => anthropic_vertex_ai::ToolChoice::None,
        }),
        metadata: None,
        stop_sequences: Vec::new(),
        temperature: request.temperature.or(Some(default_temperature)),
        top_k: None,
        top_p: None,
    }
}

pub struct AnthropicVertexEventMapper {
    tool_uses_by_index: HashMap<usize, RawToolUse>,
    usage: Usage,
    stop_reason: StopReason,
}

impl AnthropicVertexEventMapper {
    pub fn new() -> Self {
        Self {
            tool_uses_by_index: HashMap::default(),
            usage: Usage::default(),
            stop_reason: StopReason::EndTurn,
        }
    }

    pub fn map_stream(
        mut self,
        events: Pin<Box<dyn Send + Stream<Item = Result<Event, AnthropicError>>>>,
    ) -> impl Stream<Item = Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>
    {
        events.flat_map(move |event| {
            futures::stream::iter(match event {
                Ok(event) => self.map_event(event),
                Err(error) => vec![Err(error.into())],
            })
        })
    }

    pub fn map_event(
        &mut self,
        event: Event,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        match event {
            Event::ContentBlockStart {
                index,
                content_block,
            } => match content_block {
                ResponseContent::Text { text } => {
                    vec![Ok(LanguageModelCompletionEvent::Text(text))]
                }
                ResponseContent::Thinking { thinking } => {
                    vec![Ok(LanguageModelCompletionEvent::Thinking {
                        text: thinking,
                        signature: None,
                    })]
                }
                ResponseContent::RedactedThinking { data } => {
                    vec![Ok(LanguageModelCompletionEvent::RedactedThinking { data })]
                }
                ResponseContent::ToolUse { id, name, .. } => {
                    self.tool_uses_by_index.insert(
                        index,
                        RawToolUse {
                            id,
                            name,
                            input_json: String::new(),
                        },
                    );
                    Vec::new()
                }
            },
            Event::ContentBlockDelta { index, delta } => match delta {
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
                    vec![Ok(LanguageModelCompletionEvent::Thinking {
                        text: "".to_string(),
                        signature: Some(signature),
                    })]
                }
                ContentDelta::InputJsonDelta { partial_json } => {
                    if let Some(tool_use) = self.tool_uses_by_index.get_mut(&index) {
                        tool_use.input_json.push_str(&partial_json);

                        // Try to convert invalid (incomplete) JSON into
                        // valid JSON that serde can accept, e.g. by closing
                        // unclosed delimiters. This way, we can update the
                        // UI with whatever has been streamed back so far.
                        if let Ok(input) = serde_json::Value::from_str(
                            &partial_json_fixer::fix_json(&tool_use.input_json),
                        ) {
                            return vec![Ok(LanguageModelCompletionEvent::ToolUse(
                                LanguageModelToolUse {
                                    id: tool_use.id.clone().into(),
                                    name: tool_use.name.clone().into(),
                                    is_input_complete: false,
                                    raw_input: tool_use.input_json.clone(),
                                    input,
                                },
                            ))];
                        }
                    }
                    return vec![];
                }
            },
            Event::ContentBlockStop { index } => {
                if let Some(tool_use) = self.tool_uses_by_index.remove(&index) {
                    let input_json = tool_use.input_json.trim();
                    let input_value = if input_json.is_empty() {
                        Ok(serde_json::Value::Object(serde_json::Map::default()))
                    } else {
                        serde_json::Value::from_str(input_json)
                    };
                    let event_result = match input_value {
                        Ok(input) => Ok(LanguageModelCompletionEvent::ToolUse(
                            LanguageModelToolUse {
                                id: tool_use.id.into(),
                                name: tool_use.name.into(),
                                is_input_complete: true,
                                input,
                                raw_input: tool_use.input_json.clone(),
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

                    vec![event_result]
                } else {
                    Vec::new()
                }
            }
            Event::MessageStart { message } => {
                update_usage(&mut self.usage, &message.usage);
                vec![
                    Ok(LanguageModelCompletionEvent::UsageUpdate(convert_usage(
                        &self.usage,
                    ))),
                    Ok(LanguageModelCompletionEvent::StartMessage {
                        message_id: message.id,
                    }),
                ]
            }
            Event::MessageDelta { delta, usage } => {
                update_usage(&mut self.usage, &usage);
                if let Some(stop_reason) = delta.stop_reason.as_deref() {
                    self.stop_reason = match stop_reason {
                        "end_turn" => StopReason::EndTurn,
                        "max_tokens" => StopReason::MaxTokens,
                        "tool_use" => StopReason::ToolUse,
                        "refusal" => StopReason::Refusal,
                        _ => {
                            log::error!("Unexpected anthropic stop_reason: {stop_reason}");
                            StopReason::EndTurn
                        }
                    };
                }
                vec![Ok(LanguageModelCompletionEvent::UsageUpdate(
                    convert_usage(&self.usage),
                ))]
            }
            Event::MessageStop => {
                vec![Ok(LanguageModelCompletionEvent::Stop(self.stop_reason))]
            }
            Event::Error { error } => {
                vec![Err(error.into())]
            }
            _ => Vec::new(),
        }
    }
}

struct RawToolUse {
    id: String,
    name: String,
    input_json: String,
}

/// Updates usage data by preferring counts from `new`.
fn update_usage(usage: &mut Usage, new: &Usage) {
    if let Some(input_tokens) = new.input_tokens {
        usage.input_tokens = Some(input_tokens);
    }
    if let Some(output_tokens) = new.output_tokens {
        usage.output_tokens = Some(output_tokens);
    }
    if let Some(cache_creation_input_tokens) = new.cache_creation_input_tokens {
        usage.cache_creation_input_tokens = Some(cache_creation_input_tokens);
    }
    if let Some(cache_read_input_tokens) = new.cache_read_input_tokens {
        usage.cache_read_input_tokens = Some(cache_read_input_tokens);
    }
}

fn convert_usage(usage: &Usage) -> language_model::TokenUsage {
    language_model::TokenUsage {
        input_tokens: usage.input_tokens.unwrap_or(0),
        output_tokens: usage.output_tokens.unwrap_or(0),
        cache_creation_input_tokens: usage.cache_creation_input_tokens.unwrap_or(0),
        cache_read_input_tokens: usage.cache_read_input_tokens.unwrap_or(0),
    }
}

struct ConfigurationView {
    state: gpui::Entity<State>,
    load_credentials_task: Option<Task<()>>,
}

impl ConfigurationView {
    fn new(state: gpui::Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        cx.observe(&state, |_, _, cx| {
            cx.notify();
        })
        .detach();

        let load_credentials_task = Some(cx.spawn_in(window, {
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
            state,
            load_credentials_task,
        }
    }

    fn authenticate_gcloud(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        println!("Authenticating with gcloud...");

        let state = self.state.clone();
        self.load_credentials_task = Some(cx.spawn_in(window, {
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
        cx.notify();
    }

    fn reset_gcloud_auth(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state.update(cx, |state, cx| state.reset_api_key(cx))?.await
        })
        .detach_and_log_err(cx);

        cx.notify();
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_authenticated = self.state.read(cx).is_authenticated();

        if self.load_credentials_task.is_some() {
            div()
                .child(Label::new("Attempting to authenticate with gcloud..."))
                .into_any()
        } else if !is_authenticated {
            v_flex()
                .size_full()
                .child(Label::new("Please authenticate with Google Cloud to use this provider."))
                .child(
                    List::new()
                        .child(InstructionListItem::text_only(
                            "1. Ensure Google Cloud SDK is installed and configured.",
                        ))
                        .child(InstructionListItem::text_only(
                            "2. Run 'gcloud auth application-default login' in your terminal.",
                        ))
                        .child(InstructionListItem::text_only(
                            "3. Configure your desired Google Cloud Project ID and Location ID in Zed's settings.json file under 'language_models.google_vertex'.",
                        ))
                )
                .child(
                    h_flex()
                        .w_full()
                        .my_2()
                        .child(
                            Button::new("authenticate-gcloud", "Authenticate with gcloud")
                                .label_size(LabelSize::Small)
                                .icon_size(IconSize::Small)
                                .on_click(cx.listener(|this, _, window, cx| this.authenticate_gcloud(window, cx))),
                        ),
                )
                .child(
                    Label::new(
                        "This will attempt to acquire an access token using your
                        gcloud application-default credentials. You might need to run
                        'gcloud auth application-default login' manually first."
                    )
                    .size(LabelSize::Small).color(Color::Muted),
                )
                .into_any()
        } else {
            h_flex()
                .mt_1()
                .p_1()
                // .justify_between() // Removed, button is handled separately
                .rounded_md()
                .border_1()
                .border_color(cx.theme().colors().border)
                .bg(cx.theme().colors().background)
                .child(
                    h_flex()
                        .gap_1()
                        .child(Icon::new(IconName::Check).color(Color::Success))
                        .child(Label::new("Authenticated with gcloud.")),
                )
                  .child(
                    Button::new("reset-gcloud-auth", "Clear Token")
                        .label_size(LabelSize::Small)
                        .icon(Some(IconName::Trash))
                        .icon_size(IconSize::Small)
                        .icon_position(IconPosition::Start)
                        .tooltip(Tooltip::text("Clear the in-memory access token. You will need to re-authenticate to use the provider."))
                        .on_click(cx.listener(|this, _, window, cx| this.reset_gcloud_auth(window, cx))),
                )
                .into_any()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use language_model::{LanguageModelRequestMessage, MessageContent};
    use settings::ModelMode;

    #[test]
    fn test_cache_control_only_on_last_segment() {
        let request = LanguageModelRequest {
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec![
                    MessageContent::Text("Some prompt".to_string()),
                    MessageContent::Image(language_model::LanguageModelImage::empty()),
                    MessageContent::Image(language_model::LanguageModelImage::empty()),
                    MessageContent::Image(language_model::LanguageModelImage::empty()),
                    MessageContent::Image(language_model::LanguageModelImage::empty()),
                ],
                cache: true,
            }],
            thread_id: None,
            prompt_id: None,
            intent: None,
            mode: None,
            stop: vec![],
            temperature: None,
            tools: vec![],
            tool_choice: None,
            thinking_allowed: true,
        };

        let anthropic_request = into_anthropic(
            request,
            "claude-sonnet-4@20250514".to_string(),
            0.7,
            4096,
            ModelMode::Default,
        );

        assert_eq!(anthropic_request.messages.len(), 1);

        let message = &anthropic_request.messages[0];
        assert_eq!(message.content.len(), 5);

        assert!(matches!(
            message.content[0],
            anthropic_vertex_ai::RequestContent::Text {
                cache_control: None,
                ..
            }
        ));
        for i in 1..3 {
            assert!(matches!(
                message.content[i],
                anthropic_vertex_ai::RequestContent::Image {
                    cache_control: None,
                    ..
                }
            ));
        }

        assert!(matches!(
            message.content[4],
            anthropic_vertex_ai::RequestContent::Image {
                cache_control: Some(anthropic_vertex_ai::CacheControl {
                    cache_type: anthropic_vertex_ai::CacheControlType::Ephemeral,
                }),
                ..
            }
        ));
    }
}
