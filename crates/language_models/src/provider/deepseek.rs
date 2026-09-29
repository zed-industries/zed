use anyhow::{Result, anyhow};
use collections::{HashMap, IndexMap};
use credentials_provider::CredentialsProvider;
use deepseek::DEEPSEEK_API_URL;

use futures::Stream;
use futures::{FutureExt, StreamExt, future::BoxFuture, stream::BoxStream};
use gpui::{App, AppContext, AsyncApp, Context, Entity, SharedString, Task};
use http_client::{CustomHeaders, HttpClient};
use language_model::{
    ApiKeyConfiguration, ApiKeyState, AuthenticateError, EnvVar, IconOrSvg, LanguageModel,
    LanguageModelClient, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelCompletionStream, LanguageModelEffortLevel, LanguageModelId, LanguageModelName,
    LanguageModelProvider, LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelProviderState, LanguageModelRequest, LanguageModelToolChoice,
    LanguageModelToolChoiceSupport, LanguageModelToolResultContent, LanguageModelToolUse,
    MessageContent, ModelRateLimiters, ProviderSettingsView, RateLimiter, Role, StopReason,
    TokenUsage, env_var, unavailable_error,
};
pub use settings::DeepseekAvailableModel as AvailableModel;
use settings::{Settings, SettingsStore};
use std::pin::Pin;
use std::sync::{Arc, LazyLock};

use ui::IconName;

use language_model::util::{fix_streamed_json, parse_tool_arguments};

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("deepseek");
const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("DeepSeek");

const API_KEY_ENV_VAR_NAME: &str = "DEEPSEEK_API_KEY";
static API_KEY_ENV_VAR: LazyLock<EnvVar> = env_var!(API_KEY_ENV_VAR_NAME);

#[derive(Default)]
struct RawToolCall {
    id: String,
    name: String,
    arguments: String,
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct DeepSeekSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
    pub custom_headers: CustomHeaders,
}
pub struct DeepSeekLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
    request_limiters: ModelRateLimiters,
}

pub struct State {
    api_key_state: ApiKeyState,
    credentials_provider: Arc<dyn CredentialsProvider>,
}

impl State {
    fn is_authenticated(&self) -> bool {
        self.api_key_state.has_key()
    }

    fn set_api_key(&mut self, api_key: Option<String>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = DeepSeekLanguageModelProvider::api_url(cx);
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
        let api_url = DeepSeekLanguageModelProvider::api_url(cx);
        self.api_key_state.load_if_needed(
            api_url,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        )
    }
}

impl DeepSeekLanguageModelProvider {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut App,
    ) -> Self {
        let state = cx.new(|cx| {
            cx.observe_global::<SettingsStore>(|this: &mut State, cx| {
                let credentials_provider = this.credentials_provider.clone();
                let api_url = Self::api_url(cx);
                this.api_key_state.handle_url_change(
                    api_url,
                    |this| &mut this.api_key_state,
                    credentials_provider,
                    cx,
                );
                cx.notify();
            })
            .detach();
            State {
                api_key_state: ApiKeyState::new(Self::api_url(cx), (*API_KEY_ENV_VAR).clone()),
                credentials_provider,
            }
        });

        Self {
            http_client,
            state,
            request_limiters: ModelRateLimiters::default(),
        }
    }

    /// Every model this provider offers, keyed by id: the built-in models,
    /// with settings entries added or overriding built-in ones.
    fn deepseek_models(&self, cx: &App) -> IndexMap<String, deepseek::Model> {
        let mut models = IndexMap::default();

        for model in [deepseek::Model::V4_1Flash, deepseek::Model::V4Pro] {
            models.insert(model.id().to_string(), model);
        }

        for available_model in &Self::settings(cx).available_models {
            models.insert(
                available_model.name.clone(),
                deepseek::Model::Custom {
                    name: available_model.name.clone(),
                    display_name: available_model.display_name.clone(),
                    max_tokens: available_model.max_tokens,
                    max_output_tokens: available_model.max_output_tokens,
                },
            );
        }

        models
    }

    /// The current configuration of `model`, if this provider still offers it.
    fn config(
        &self,
        model: &LanguageModel,
        cx: &App,
    ) -> Result<deepseek::Model, LanguageModelCompletionError> {
        self.deepseek_models(cx)
            .swap_remove(model.id.0.as_ref())
            .ok_or_else(|| unavailable_error(model))
    }

    fn settings(cx: &App) -> &DeepSeekSettings {
        &crate::AllLanguageModelSettings::get_global(cx).deepseek
    }

    fn api_url(cx: &App) -> SharedString {
        let api_url = &Self::settings(cx).api_url;
        if api_url.is_empty() {
            DEEPSEEK_API_URL.into()
        } else {
            SharedString::new(api_url.as_str())
        }
    }
}

impl LanguageModelProviderState for DeepSeekLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for DeepSeekLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiDeepSeek)
    }

    fn default_model(&self, cx: &App) -> Option<LanguageModel> {
        self.deepseek_models(cx)
            .get(deepseek::Model::default().id())
            .map(language_model)
    }

    fn default_fast_model(&self, cx: &App) -> Option<LanguageModel> {
        self.deepseek_models(cx)
            .get(deepseek::Model::default_fast().id())
            .map(language_model)
    }

    fn provided_models(&self, cx: &App) -> Vec<LanguageModel> {
        self.deepseek_models(cx)
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

    fn settings_view(&self, cx: &mut App) -> Option<ProviderSettingsView> {
        let state = self.state.read(cx);
        Some(ProviderSettingsView::ApiKey(ApiKeyConfiguration::new(
            state.api_key_state.has_key(),
            state.api_key_state.is_from_env_var(),
            state.api_key_state.env_var_name().clone(),
            "https://platform.deepseek.com/api_keys".into(),
        )))
    }

    fn set_api_key(&self, api_key: Option<String>, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.set_api_key(api_key, cx))
    }
}

impl LanguageModelClient for DeepSeekLanguageModelProvider {
    fn stream_completion(
        &self,
        model: &LanguageModel,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<LanguageModelCompletionStream, LanguageModelCompletionError>>
    {
        let config = match cx.update(|cx| self.config(model, cx)) {
            Ok(config) => config,
            Err(error) => return async move { Err(error) }.boxed(),
        };
        let request_limiter = self.request_limiters.for_model(&model.id);
        let request = match into_deepseek(request, &config, config.max_output_tokens()) {
            Ok(request) => request,
            Err(error) => return async move { Err(error.into()) }.boxed(),
        };
        let stream = self.stream_deepseek_request(request, &request_limiter, cx);
        let executor = cx.background_executor().clone();

        async move {
            let mapper = DeepSeekEventMapper::new();
            Ok(language_model::stream_in_background(
                mapper.map_stream(stream.await?).boxed(),
                executor,
            ))
        }
        .boxed()
    }
}

impl DeepSeekLanguageModelProvider {
    fn stream_deepseek_request(
        &self,
        request: deepseek::Request,
        request_limiter: &RateLimiter,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<deepseek::StreamResponse>>>> {
        let http_client = self.http_client.clone();

        let (api_key, api_url, extra_headers) = self.state.read_with(cx, |state, cx| {
            let api_url = DeepSeekLanguageModelProvider::api_url(cx);
            let extra_headers = DeepSeekLanguageModelProvider::settings(cx)
                .custom_headers
                .clone();
            (state.api_key_state.key(&api_url), api_url, extra_headers)
        });

        let future = request_limiter.stream(async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey {
                    provider: PROVIDER_NAME,
                });
            };
            let request = deepseek::stream_completion(
                http_client.as_ref(),
                &api_url,
                &api_key,
                request,
                &extra_headers,
            );
            let response = request.await?;
            Ok(response)
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

fn language_model(model: &deepseek::Model) -> LanguageModel {
    let supports_thinking = matches!(model, deepseek::Model::V4_1Flash | deepseek::Model::V4Pro);
    LanguageModel {
        supports_tools: true,
        supports_streaming_tools: true,
        supports_thinking,
        supported_effort_levels: if supports_thinking {
            Arc::new([
                LanguageModelEffortLevel {
                    name: "Low".into(),
                    value: "low".into(),
                    is_default: false,
                },
                LanguageModelEffortLevel {
                    name: "High".into(),
                    value: "high".into(),
                    is_default: true,
                },
                LanguageModelEffortLevel {
                    name: "Max".into(),
                    value: "max".into(),
                    is_default: false,
                },
            ])
        } else {
            Arc::default()
        },
        tool_choice_support: LanguageModelToolChoiceSupport::ALL,
        supports_images: model.supports_images(),
        max_output_tokens: model.max_output_tokens(),
        ..LanguageModel::new(
            LanguageModelId::from(model.id().to_string()),
            LanguageModelName::from(model.display_name().to_string()),
            PROVIDER_ID,
            PROVIDER_NAME,
            format!("deepseek/{}", model.id()),
            model.max_token_count(),
        )
    }
}

pub fn into_deepseek(
    request: LanguageModelRequest,
    model: &deepseek::Model,
    max_output_tokens: Option<u64>,
) -> Result<deepseek::Request> {
    let max_output_tokens = request.effective_max_output_tokens(max_output_tokens);
    if request.contains_custom_tool_input() {
        anyhow::bail!("DeepSeek does not support custom tools");
    }

    let thinking = deepseek_thinking(model, request.thinking_allowed);
    let thinking_enabled = thinking
        .as_ref()
        .is_some_and(|thinking| thinking.kind == deepseek::ThinkingType::Enabled);

    let supports_images = model.supports_images();

    let mut messages = Vec::new();
    let mut current_reasoning: Option<String> = None;

    for message in request.messages {
        for content in message.content {
            match content {
                MessageContent::Text(text) => {
                    let should_add = if message.role == Role::User {
                        !text.trim().is_empty()
                    } else {
                        !text.is_empty()
                    };

                    if should_add {
                        match message.role {
                            Role::User => {
                                add_user_message_content_part(
                                    deepseek::MessagePart::Text { text },
                                    &mut messages,
                                );
                            }
                            Role::Assistant => {
                                messages.push(deepseek::RequestMessage::Assistant {
                                    content: Some(text),
                                    tool_calls: Vec::new(),
                                    reasoning_content: current_reasoning.take(),
                                });
                            }
                            Role::System => {
                                messages.push(deepseek::RequestMessage::System { content: text });
                            }
                        }
                    }
                }
                MessageContent::Thinking { text, .. } => {
                    // Accumulate reasoning content for next assistant message
                    current_reasoning.get_or_insert_default().push_str(&text);
                }
                MessageContent::RedactedThinking(_) => {}
                MessageContent::Image(image) if message.role == Role::User => {
                    add_user_message_content_part(
                        deepseek::MessagePart::Image {
                            image_url: deepseek::ImageUrl {
                                url: image.to_base64_url(),
                                detail: None,
                            },
                        },
                        &mut messages,
                    );
                }
                MessageContent::Image(_) => {}
                MessageContent::Compaction(_) => {}
                MessageContent::ToolUse(tool_use) => {
                    let input = tool_use
                        .input
                        .as_json()
                        .ok_or_else(|| anyhow!("DeepSeek does not support custom tool calls"))?;
                    let tool_call = deepseek::ToolCall {
                        id: tool_use.id.to_string(),
                        content: deepseek::ToolCallContent::Function {
                            function: deepseek::FunctionContent {
                                name: tool_use.name.to_string(),
                                arguments: serde_json::to_string(input).unwrap_or_default(),
                            },
                        },
                    };

                    if let Some(deepseek::RequestMessage::Assistant { tool_calls, .. }) =
                        messages.last_mut()
                    {
                        tool_calls.push(tool_call);
                    } else {
                        messages.push(deepseek::RequestMessage::Assistant {
                            content: None,
                            tool_calls: vec![tool_call],
                            reasoning_content: current_reasoning.take(),
                        });
                    }
                }
                MessageContent::ToolResult(tool_result) => {
                    let content: Vec<deepseek::MessagePart> = tool_result
                        .content
                        .iter()
                        .filter_map(|part| match part {
                            LanguageModelToolResultContent::Text(text) => {
                                Some(deepseek::MessagePart::Text {
                                    text: text.to_string(),
                                })
                            }
                            LanguageModelToolResultContent::Image(image) => {
                                if supports_images {
                                    Some(deepseek::MessagePart::Image {
                                        image_url: deepseek::ImageUrl {
                                            url: image.to_base64_url(),
                                            detail: None,
                                        },
                                    })
                                } else {
                                    None
                                }
                            }
                        })
                        .collect();

                    messages.push(deepseek::RequestMessage::Tool {
                        content: content.into(),
                        tool_call_id: tool_result.tool_use_id.to_string(),
                    });
                }
            }
        }
    }

    Ok(deepseek::Request {
        model: model.id().to_string(),
        messages,
        stream: true,
        max_tokens: max_output_tokens,
        temperature: if thinking_enabled {
            None
        } else {
            request.temperature
        },
        thinking,
        reasoning_effort: if thinking_enabled {
            into_deepseek_reasoning_effort(request.thinking_effort.as_deref())
        } else {
            None
        },
        response_format: None,
        tool_choice: request.tool_choice.map(|choice| match choice {
            LanguageModelToolChoice::Auto => deepseek::ToolChoice::Auto,
            LanguageModelToolChoice::Any => deepseek::ToolChoice::Required,
            LanguageModelToolChoice::None => deepseek::ToolChoice::None,
        }),
        tools: request
            .tools
            .into_iter()
            .map(|tool| {
                let input_schema = match tool.input {
                    language_model::LanguageModelRequestToolInput::Function {
                        input_schema,
                        ..
                    } => input_schema,
                    language_model::LanguageModelRequestToolInput::Custom { .. } => {
                        return Err(anyhow::anyhow!("DeepSeek does not support custom tools"));
                    }
                };
                Ok(deepseek::ToolDefinition::Function {
                    function: deepseek::FunctionDefinition {
                        name: tool.name,
                        description: Some(tool.description),
                        parameters: Some(input_schema),
                    },
                })
            })
            .collect::<Result<_>>()?,
    })
}

fn add_user_message_content_part(
    new_part: deepseek::MessagePart,
    messages: &mut Vec<deepseek::RequestMessage>,
) {
    match messages.last_mut() {
        Some(deepseek::RequestMessage::User { content }) => {
            content.push_part(new_part);
        }
        _ => {
            messages.push(deepseek::RequestMessage::User {
                content: deepseek::MessageContent::from(vec![new_part]),
            });
        }
    }
}

fn deepseek_thinking(
    model: &deepseek::Model,
    thinking_allowed: bool,
) -> Option<deepseek::Thinking> {
    let kind = match model {
        deepseek::Model::V4_1Flash | deepseek::Model::V4Pro => {
            if thinking_allowed {
                deepseek::ThinkingType::Enabled
            } else {
                deepseek::ThinkingType::Disabled
            }
        }
        deepseek::Model::Custom { .. } => return None,
    };

    Some(deepseek::Thinking { kind })
}

fn into_deepseek_reasoning_effort(effort: Option<&str>) -> Option<deepseek::ReasoningEffort> {
    match effort {
        Some("low") => Some(deepseek::ReasoningEffort::Low),
        Some("high") => Some(deepseek::ReasoningEffort::High),
        Some("max") => Some(deepseek::ReasoningEffort::Max),
        _ => None,
    }
}

pub struct DeepSeekEventMapper {
    tool_calls_by_index: HashMap<usize, RawToolCall>,
}

impl DeepSeekEventMapper {
    pub fn new() -> Self {
        Self {
            tool_calls_by_index: HashMap::default(),
        }
    }

    pub fn map_stream(
        mut self,
        events: Pin<Box<dyn Send + Stream<Item = Result<deepseek::StreamResponse>>>>,
    ) -> impl Stream<Item = Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>
    {
        events.flat_map(move |event| {
            futures::stream::iter(match event {
                Ok(event) => self.map_event(event),
                Err(error) => vec![Err(LanguageModelCompletionError::from(error))],
            })
        })
    }

    pub fn map_event(
        &mut self,
        event: deepseek::StreamResponse,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        let Some(choice) = event.choices.first() else {
            return vec![Err(LanguageModelCompletionError::from(anyhow!(
                "Response contained no choices"
            )))];
        };

        let mut events = Vec::new();
        if let Some(content) = choice.delta.content.clone()
            && !content.is_empty()
        {
            events.push(Ok(LanguageModelCompletionEvent::Text(content)));
        }

        if let Some(reasoning_content) = choice.delta.reasoning_content.clone() {
            events.push(Ok(LanguageModelCompletionEvent::Thinking {
                text: reasoning_content,
                signature: None,
            }));
        }

        if let Some(tool_calls) = choice.delta.tool_calls.as_ref() {
            for tool_call in tool_calls {
                let entry = self.tool_calls_by_index.entry(tool_call.index).or_default();

                if let Some(tool_id) = tool_call.id.clone() {
                    entry.id = tool_id;
                }

                if let Some(function) = tool_call.function.as_ref() {
                    if let Some(name) = function.name.clone() {
                        entry.name = name;
                    }

                    if let Some(arguments) = function.arguments.clone() {
                        entry.arguments.push_str(&arguments);
                    }
                }

                if !entry.id.is_empty() && !entry.name.is_empty() {
                    if let Ok(input) = serde_json::from_str::<serde_json::Value>(
                        &fix_streamed_json(&entry.arguments),
                    ) {
                        events.push(Ok(LanguageModelCompletionEvent::ToolUse(
                            LanguageModelToolUse {
                                id: entry.id.clone().into(),
                                name: entry.name.as_str().into(),
                                is_input_complete: false,
                                input: language_model::LanguageModelToolUseInput::Json(input),
                                raw_input: entry.arguments.clone(),
                                thought_signature: None,
                            },
                        )));
                    }
                }
            }
        }

        if let Some(usage) = event.usage {
            events.push(Ok(LanguageModelCompletionEvent::UsageUpdate(TokenUsage {
                input_tokens: usage.prompt_tokens,
                output_tokens: usage.completion_tokens,
                cache_creation_input_tokens: 0,
                cache_read_input_tokens: 0,
            })));
        }

        match choice.finish_reason.as_deref() {
            Some("stop") => {
                events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::EndTurn)));
            }
            Some("tool_calls") => {
                events.extend(self.tool_calls_by_index.drain().map(|(_, tool_call)| {
                    match parse_tool_arguments(&tool_call.arguments) {
                        Ok(input) => Ok(LanguageModelCompletionEvent::ToolUse(
                            LanguageModelToolUse {
                                id: tool_call.id.clone().into(),
                                name: tool_call.name.as_str().into(),
                                is_input_complete: true,
                                input: language_model::LanguageModelToolUseInput::Json(input),
                                raw_input: tool_call.arguments.clone(),
                                thought_signature: None,
                            },
                        )),
                        Err(error) => Ok(LanguageModelCompletionEvent::ToolUseJsonParseError {
                            id: tool_call.id.clone().into(),
                            tool_name: tool_call.name.as_str().into(),
                            raw_input: tool_call.arguments.into(),
                            json_parse_error: error.to_string(),
                        }),
                    }
                }));

                events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::ToolUse)));
            }
            Some(stop_reason) => {
                log::error!("Unexpected DeepSeek stop_reason: {stop_reason:?}",);
                events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::EndTurn)));
            }
            None => {}
        }

        events
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use language_model::{LanguageModelImage, LanguageModelRequestMessage};
    use serde_json::json;

    #[test]
    fn serializes_deepseek_image_parts() -> Result<()> {
        let image = LanguageModelImage {
            source: SharedString::from("aGVsbG8="),
        };
        let image_url = image.to_base64_url();
        let request = into_deepseek(
            LanguageModelRequest {
                messages: vec![LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![
                        MessageContent::Text("Describe this".to_string()),
                        MessageContent::Image(image),
                    ],
                    cache: false,
                    reasoning_details: None,
                }],
                ..Default::default()
            },
            &deepseek::Model::V4_1Flash,
            Some(1024),
        )?;

        assert_eq!(
            serde_json::to_value(&request.messages)?,
            json!([
                {
                    "role": "user",
                    "content": [
                        { "type": "text", "text": "Describe this" },
                        { "type": "image_url", "image_url": { "url": image_url } }
                    ]
                }
            ])
        );

        Ok(())
    }

    #[test]
    fn request_output_limits_reach_deepseek_payloads() -> Result<()> {
        for (limit, expected) in [(None, 4096), (Some(1024), 1024), (Some(8192), 4096)] {
            let request = into_deepseek(
                LanguageModelRequest {
                    max_output_tokens: limit,
                    ..Default::default()
                },
                &deepseek::Model::V4Pro,
                Some(4096),
            )?;
            assert_eq!(serde_json::to_value(request)?["max_tokens"], expected);
        }
        Ok(())
    }
}
