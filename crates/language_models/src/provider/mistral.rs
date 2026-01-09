use anyhow::{Result, anyhow};
use collections::BTreeMap;

use futures::{FutureExt, Stream, StreamExt, future::BoxFuture, stream::BoxStream};
use gpui::{AnyView, App, AsyncApp, Context, Entity, Global, SharedString, Task, Window};
use http_client::HttpClient;
use language_model::{
    ApiKeyState, AuthenticateError, EnvVar, IconOrSvg, LanguageModel, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelId, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelRequest, LanguageModelToolChoice, LanguageModelToolResultContent,
    LanguageModelToolUse, MessageContent, RateLimiter, Role, StopReason, TokenUsage, env_var,
};
pub use mistral::{CODESTRAL_API_URL, MISTRAL_API_URL, StreamResponse};
pub use settings::MistralAvailableModel as AvailableModel;
use settings::{Settings, SettingsStore};
use std::collections::HashMap;
use std::pin::Pin;
use std::str::FromStr;
use std::sync::{Arc, LazyLock};
use strum::IntoEnumIterator;
use ui::{ButtonLink, ConfiguredApiCard, List, ListBulletItem, prelude::*};
use ui_input::InputField;
use util::ResultExt;

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("mistral");
const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("Mistral");

const API_KEY_ENV_VAR_NAME: &str = "MISTRAL_API_KEY";
static API_KEY_ENV_VAR: LazyLock<EnvVar> = env_var!(API_KEY_ENV_VAR_NAME);

const CODESTRAL_API_KEY_ENV_VAR_NAME: &str = "CODESTRAL_API_KEY";
static CODESTRAL_API_KEY_ENV_VAR: LazyLock<EnvVar> = env_var!(CODESTRAL_API_KEY_ENV_VAR_NAME);

#[derive(Default, Clone, Debug, PartialEq)]
pub struct MistralSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
}

pub struct MistralLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    pub state: Entity<State>,
}

pub struct State {
    api_key_state: ApiKeyState,
    codestral_api_key_state: Entity<ApiKeyState>,
}

struct CodestralApiKey(Entity<ApiKeyState>);
impl Global for CodestralApiKey {}

pub fn codestral_api_key(cx: &mut App) -> Entity<ApiKeyState> {
    if cx.has_global::<CodestralApiKey>() {
        cx.global::<CodestralApiKey>().0.clone()
    } else {
        let api_key_state = cx
            .new(|_| ApiKeyState::new(CODESTRAL_API_URL.into(), CODESTRAL_API_KEY_ENV_VAR.clone()));
        cx.set_global(CodestralApiKey(api_key_state.clone()));
        api_key_state
    }
}

impl State {
    fn is_authenticated(&self) -> bool {
        self.api_key_state.has_key()
    }

    fn set_api_key(&mut self, api_key: Option<String>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let api_url = MistralLanguageModelProvider::api_url(cx);
        self.api_key_state
            .store(api_url, api_key, |this| &mut this.api_key_state, cx)
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let api_url = MistralLanguageModelProvider::api_url(cx);
        self.api_key_state
            .load_if_needed(api_url, |this| &mut this.api_key_state, cx)
    }

    fn authenticate_codestral(
        &mut self,
        cx: &mut Context<Self>,
    ) -> Task<Result<(), AuthenticateError>> {
        self.codestral_api_key_state.update(cx, |state, cx| {
            state.load_if_needed(CODESTRAL_API_URL.into(), |state| state, cx)
        })
    }
}

struct GlobalMistralLanguageModelProvider(Arc<MistralLanguageModelProvider>);

impl Global for GlobalMistralLanguageModelProvider {}

impl MistralLanguageModelProvider {
    pub fn try_global(cx: &App) -> Option<&Arc<MistralLanguageModelProvider>> {
        cx.try_global::<GlobalMistralLanguageModelProvider>()
            .map(|this| &this.0)
    }

    pub fn global(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Arc<Self> {
        if let Some(this) = cx.try_global::<GlobalMistralLanguageModelProvider>() {
            return this.0.clone();
        }
        let state = cx.new(|cx| {
            cx.observe_global::<SettingsStore>(|this: &mut State, cx| {
                let api_url = Self::api_url(cx);
                this.api_key_state
                    .handle_url_change(api_url, |this| &mut this.api_key_state, cx);
                cx.notify();
            })
            .detach();
            State {
                api_key_state: ApiKeyState::new(Self::api_url(cx), (*API_KEY_ENV_VAR).clone()),
                codestral_api_key_state: codestral_api_key(cx),
            }
        });

        let this = Arc::new(Self { http_client, state });
        cx.set_global(GlobalMistralLanguageModelProvider(this));
        cx.global::<GlobalMistralLanguageModelProvider>().0.clone()
    }

    pub fn load_codestral_api_key(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        self.state
            .update(cx, |state, cx| state.authenticate_codestral(cx))
    }

    pub fn codestral_api_key(&self, url: &str, cx: &App) -> Option<Arc<str>> {
        self.state
            .read(cx)
            .codestral_api_key_state
            .read(cx)
            .key(url)
    }

    fn create_language_model(&self, model: mistral::Model) -> Arc<dyn LanguageModel> {
        Arc::new(MistralLanguageModel {
            id: LanguageModelId::from(model.id().to_string()),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }

    fn settings(cx: &App) -> &MistralSettings {
        &crate::AllLanguageModelSettings::get_global(cx).mistral
    }

    pub fn api_url(cx: &App) -> SharedString {
        let api_url = &Self::settings(cx).api_url;
        if api_url.is_empty() {
            mistral::MISTRAL_API_URL.into()
        } else {
            SharedString::new(api_url.as_str())
        }
    }
}

impl LanguageModelProviderState for MistralLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for MistralLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiMistral)
    }

    fn default_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(mistral::Model::default()))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(mistral::Model::default_fast()))
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let mut models = BTreeMap::default();

        // Add base models from mistral::Model::iter()
        for model in mistral::Model::iter() {
            if !matches!(model, mistral::Model::Custom { .. }) {
                models.insert(model.id().to_string(), model);
            }
        }

        // Override with available models from settings
        for model in &Self::settings(cx).available_models {
            models.insert(
                model.name.clone(),
                mistral::Model::Custom {
                    name: model.name.clone(),
                    display_name: model.display_name.clone(),
                    max_tokens: model.max_tokens,
                    max_output_tokens: model.max_output_tokens,
                    max_completion_tokens: model.max_completion_tokens,
                    supports_tools: model.supports_tools,
                    supports_images: model.supports_images,
                    supports_thinking: model.supports_thinking,
                },
            );
        }

        models
            .into_values()
            .map(|model| {
                Arc::new(MistralLanguageModel {
                    id: LanguageModelId::from(model.id().to_string()),
                    model,
                    state: self.state.clone(),
                    http_client: self.http_client.clone(),
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
            .update(cx, |state, cx| state.set_api_key(None, cx))
    }
}

pub struct MistralLanguageModel {
    id: LanguageModelId,
    model: mistral::Model,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl MistralLanguageModel {
    fn stream_completion(
        &self,
        request: mistral::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<futures::stream::BoxStream<'static, Result<mistral::StreamResponse>>>,
    > {
        let http_client = self.http_client.clone();

        let (api_key, api_url) = self.state.read_with(cx, |state, cx| {
            let api_url = MistralLanguageModelProvider::api_url(cx);
            (state.api_key_state.key(&api_url), api_url)
        });

        let future = self.request_limiter.stream(async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey {
                    provider: PROVIDER_NAME,
                });
            };
            let request =
                mistral::stream_completion(http_client.as_ref(), &api_url, &api_key, request);
            let response = request.await?;
            Ok(response)
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

impl LanguageModel for MistralLanguageModel {
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

    fn supports_tool_choice(&self, _choice: LanguageModelToolChoice) -> bool {
        self.model.supports_tools()
    }

    fn supports_images(&self) -> bool {
        self.model.supports_images()
    }

    fn telemetry_id(&self) -> String {
        format!("mistral/{}", self.model.id())
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_token_count()
    }

    fn max_output_tokens(&self) -> Option<u64> {
        self.model.max_output_tokens()
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &App,
    ) -> BoxFuture<'static, Result<u64>> {
        cx.background_spawn(async move {
            let messages = request
                .messages
                .into_iter()
                .map(|message| tiktoken_rs::ChatCompletionRequestMessage {
                    role: match message.role {
                        Role::User => "user".into(),
                        Role::Assistant => "assistant".into(),
                        Role::System => "system".into(),
                    },
                    content: Some(message.string_contents()),
                    name: None,
                    function_call: None,
                })
                .collect::<Vec<_>>();

            tiktoken_rs::num_tokens_from_messages("gpt-4", &messages).map(|tokens| tokens as u64)
        })
        .boxed()
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
        let request = into_mistral(request, self.model.clone(), self.max_output_tokens());
        let stream = self.stream_completion(request, cx);

        async move {
            let stream = stream.await?;
            let mapper = MistralEventMapper::new();
            Ok(mapper.map_stream(stream).boxed())
        }
        .boxed()
    }
}

pub fn into_mistral(
    request: LanguageModelRequest,
    model: mistral::Model,
    max_output_tokens: Option<u64>,
) -> mistral::Request {
    let stream = true;

    let mut messages = Vec::new();
    for message in &request.messages {
        match message.role {
            Role::User => {
                let mut message_content = mistral::MessageContent::empty();
                for content in &message.content {
                    match content {
                        MessageContent::Text(text) => {
                            message_content
                                .push_part(mistral::MessagePart::Text { text: text.clone() });
                        }
                        MessageContent::Image(image_content) => {
                            if model.supports_images() {
                                message_content.push_part(mistral::MessagePart::ImageUrl {
                                    image_url: image_content.to_base64_url(),
                                });
                            }
                        }
                        MessageContent::Thinking { text, .. } => {
                            if model.supports_thinking() {
                                message_content.push_part(mistral::MessagePart::Thinking {
                                    thinking: vec![mistral::ThinkingPart::Text {
                                        text: text.clone(),
                                    }],
                                });
                            }
                        }
                        MessageContent::RedactedThinking(_) => {}
                        MessageContent::ToolUse(_) => {
                            // Tool use is not supported in User messages for Mistral
                        }
                        MessageContent::ToolResult(tool_result) => {
                            let tool_content = match &tool_result.content {
                                LanguageModelToolResultContent::Text(text) => text.to_string(),
                                LanguageModelToolResultContent::Image(_) => {
                                    "[Tool responded with an image, but Zed doesn't support these in Mistral models yet]".to_string()
                                }
                            };
                            messages.push(mistral::RequestMessage::Tool {
                                content: tool_content,
                                tool_call_id: tool_result.tool_use_id.to_string(),
                            });
                        }
                    }
                }
                if !matches!(message_content, mistral::MessageContent::Plain { ref content } if content.is_empty())
                {
                    messages.push(mistral::RequestMessage::User {
                        content: message_content,
                    });
                }
            }
            Role::Assistant => {
                for content in &message.content {
                    match content {
                        MessageContent::Text(text) => {
                            messages.push(mistral::RequestMessage::Assistant {
                                content: Some(mistral::MessageContent::Plain {
                                    content: text.clone(),
                                }),
                                tool_calls: Vec::new(),
                            });
                        }
                        MessageContent::Thinking { text, .. } => {
                            if model.supports_thinking() {
                                messages.push(mistral::RequestMessage::Assistant {
                                    content: Some(mistral::MessageContent::Multipart {
                                        content: vec![mistral::MessagePart::Thinking {
                                            thinking: vec![mistral::ThinkingPart::Text {
                                                text: text.clone(),
                                            }],
                                        }],
                                    }),
                                    tool_calls: Vec::new(),
                                });
                            }
                        }
                        MessageContent::RedactedThinking(_) => {}
                        MessageContent::Image(_) => {}
                        MessageContent::ToolUse(tool_use) => {
                            let tool_call = mistral::ToolCall {
                                id: tool_use.id.to_string(),
                                content: mistral::ToolCallContent::Function {
                                    function: mistral::FunctionContent {
                                        name: tool_use.name.to_string(),
                                        arguments: serde_json::to_string(&tool_use.input)
                                            .unwrap_or_default(),
                                    },
                                },
                            };

                            if let Some(mistral::RequestMessage::Assistant { tool_calls, .. }) =
                                messages.last_mut()
                            {
                                tool_calls.push(tool_call);
                            } else {
                                messages.push(mistral::RequestMessage::Assistant {
                                    content: None,
                                    tool_calls: vec![tool_call],
                                });
                            }
                        }
                        MessageContent::ToolResult(_) => {
                            // Tool results are not supported in Assistant messages
                        }
                    }
                }
            }
            Role::System => {
                for content in &message.content {
                    match content {
                        MessageContent::Text(text) => {
                            messages.push(mistral::RequestMessage::System {
                                content: mistral::MessageContent::Plain {
                                    content: text.clone(),
                                },
                            });
                        }
                        MessageContent::Thinking { text, .. } => {
                            if model.supports_thinking() {
                                messages.push(mistral::RequestMessage::System {
                                    content: mistral::MessageContent::Multipart {
                                        content: vec![mistral::MessagePart::Thinking {
                                            thinking: vec![mistral::ThinkingPart::Text {
                                                text: text.clone(),
                                            }],
                                        }],
                                    },
                                });
                            }
                        }
                        MessageContent::RedactedThinking(_) => {}
                        MessageContent::Image(_)
                        | MessageContent::ToolUse(_)
                        | MessageContent::ToolResult(_) => {
                            // Images and tools are not supported in System messages
                        }
                    }
                }
            }
        }
    }

    mistral::Request {
        model: model.id().to_string(),
        messages,
        stream,
        max_tokens: max_output_tokens,
        temperature: request.temperature,
        response_format: None,
        tool_choice: match request.tool_choice {
            Some(LanguageModelToolChoice::Auto) if !request.tools.is_empty() => {
                Some(mistral::ToolChoice::Auto)
            }
            Some(LanguageModelToolChoice::Any) if !request.tools.is_empty() => {
                Some(mistral::ToolChoice::Any)
            }
            Some(LanguageModelToolChoice::None) => Some(mistral::ToolChoice::None),
            _ if !request.tools.is_empty() => Some(mistral::ToolChoice::Auto),
            _ => None,
        },
        parallel_tool_calls: if !request.tools.is_empty() {
            Some(false)
        } else {
            None
        },
        tools: request
            .tools
            .into_iter()
            .map(|tool| mistral::ToolDefinition::Function {
                function: mistral::FunctionDefinition {
                    name: tool.name,
                    description: Some(tool.description),
                    parameters: Some(tool.input_schema),
                },
            })
            .collect(),
    }
}

pub struct MistralEventMapper {
    tool_calls_by_index: HashMap<usize, RawToolCall>,
}

impl MistralEventMapper {
    pub fn new() -> Self {
        Self {
            tool_calls_by_index: HashMap::default(),
        }
    }

    pub fn map_stream(
        mut self,
        events: Pin<Box<dyn Send + Stream<Item = Result<StreamResponse>>>>,
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
        event: mistral::StreamResponse,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        let Some(choice) = event.choices.first() else {
            return vec![Err(LanguageModelCompletionError::from(anyhow!(
                "Response contained no choices"
            )))];
        };

        let mut events = Vec::new();
        if let Some(content) = choice.delta.content.as_ref() {
            match content {
                mistral::MessageContentDelta::Text(text) => {
                    events.push(Ok(LanguageModelCompletionEvent::Text(text.clone())));
                }
                mistral::MessageContentDelta::Parts(parts) => {
                    for part in parts {
                        match part {
                            mistral::MessagePart::Text { text } => {
                                events.push(Ok(LanguageModelCompletionEvent::Text(text.clone())));
                            }
                            mistral::MessagePart::Thinking { thinking } => {
                                for tp in thinking.iter().cloned() {
                                    match tp {
                                        mistral::ThinkingPart::Text { text } => {
                                            events.push(Ok(
                                                LanguageModelCompletionEvent::Thinking {
                                                    text,
                                                    signature: None,
                                                },
                                            ));
                                        }
                                    }
                                }
                            }
                            mistral::MessagePart::ImageUrl { .. } => {
                                // We currently don't emit a separate event for images in responses.
                            }
                        }
                    }
                }
            }
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

        if let Some(finish_reason) = choice.finish_reason.as_deref() {
            match finish_reason {
                "stop" => {
                    events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::EndTurn)));
                }
                "tool_calls" => {
                    events.extend(self.process_tool_calls());
                    events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::ToolUse)));
                }
                unexpected => {
                    log::error!("Unexpected Mistral stop_reason: {unexpected:?}");
                    events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::EndTurn)));
                }
            }
        }

        events
    }

    fn process_tool_calls(
        &mut self,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        let mut results = Vec::new();

        for (_, tool_call) in self.tool_calls_by_index.drain() {
            if tool_call.id.is_empty() || tool_call.name.is_empty() {
                results.push(Err(LanguageModelCompletionError::from(anyhow!(
                    "Received incomplete tool call: missing id or name"
                ))));
                continue;
            }

            match serde_json::Value::from_str(&tool_call.arguments) {
                Ok(input) => results.push(Ok(LanguageModelCompletionEvent::ToolUse(
                    LanguageModelToolUse {
                        id: tool_call.id.into(),
                        name: tool_call.name.into(),
                        is_input_complete: true,
                        input,
                        raw_input: tool_call.arguments,
                        thought_signature: None,
                    },
                ))),
                Err(error) => {
                    results.push(Ok(LanguageModelCompletionEvent::ToolUseJsonParseError {
                        id: tool_call.id.into(),
                        tool_name: tool_call.name.into(),
                        raw_input: tool_call.arguments.into(),
                        json_parse_error: error.to_string(),
                    }))
                }
            }
        }

        results
    }
}

#[derive(Default)]
struct RawToolCall {
    id: String,
    name: String,
    arguments: String,
}

struct ConfigurationView {
    api_key_editor: Entity<InputField>,
    state: Entity<State>,
    load_credentials_task: Option<Task<()>>,
}

impl ConfigurationView {
    fn new(state: Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let api_key_editor =
            cx.new(|cx| InputField::new(window, cx, "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"));

        cx.observe(&state, |_, _, cx| {
            cx.notify();
        })
        .detach();

        let load_credentials_task = Some(cx.spawn_in(window, {
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
            api_key_editor,
            state,
            load_credentials_task,
        }
    }

    fn save_api_key(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let api_key = self.api_key_editor.read(cx).text(cx).trim().to_string();
        if api_key.is_empty() {
            return;
        }

        // url changes can cause the editor to be displayed again
        self.api_key_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(Some(api_key), cx))
                .await
        })
        .detach_and_log_err(cx);
    }

    fn reset_api_key(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.api_key_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(None, cx))
                .await
        })
        .detach_and_log_err(cx);
    }

    fn should_render_api_key_editor(&self, cx: &mut Context<Self>) -> bool {
        !self.state.read(cx).is_authenticated()
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let env_var_set = self.state.read(cx).api_key_state.is_from_env_var();
        let configured_card_label = if env_var_set {
            format!("API key set in {API_KEY_ENV_VAR_NAME} environment variable")
        } else {
            let api_url = MistralLanguageModelProvider::api_url(cx);
            if api_url == MISTRAL_API_URL {
                "API key configured".to_string()
            } else {
                format!("API key configured for {}", api_url)
            }
        };

        if self.load_credentials_task.is_some() {
            div().child(Label::new("Loading credentials...")).into_any()
        } else if self.should_render_api_key_editor(cx) {
            v_flex()
                .size_full()
                .on_action(cx.listener(Self::save_api_key))
                .child(Label::new("To use Zed's agent with Mistral, you need to add an API key. Follow these steps:"))
                .child(
                    List::new()
                        .child(
                            ListBulletItem::new("")
                                .child(Label::new("Create one by visiting"))
                                .child(ButtonLink::new("Mistral's console", "https://console.mistral.ai/api-keys"))
                        )
                        .child(
                            ListBulletItem::new("Ensure your Mistral account has credits")
                        )
                        .child(
                            ListBulletItem::new("Paste your API key below and hit enter to start using the assistant")
                        ),
                )
                .child(self.api_key_editor.clone())
                .child(
                    Label::new(
                        format!("You can also set the {API_KEY_ENV_VAR_NAME} environment variable and restart Zed."),
                    )
                    .size(LabelSize::Small).color(Color::Muted),
                )
                .into_any()
        } else {
            v_flex()
                .size_full()
                .gap_1()
                .child(
                    ConfiguredApiCard::new(configured_card_label)
                        .disabled(env_var_set)
                        .on_click(cx.listener(|this, _, window, cx| this.reset_api_key(window, cx)))
                        .when(env_var_set, |this| {
                            this.tooltip_label(format!(
                                "To reset your API key, \
                                unset the {API_KEY_ENV_VAR_NAME} environment variable."
                            ))
                        }),
                )
                .into_any()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use language_model::{LanguageModelImage, LanguageModelRequestMessage, MessageContent};

    #[test]
    fn test_into_mistral_basic_conversion() {
        let request = LanguageModelRequest {
            messages: vec![
                LanguageModelRequestMessage {
                    role: Role::System,
                    content: vec![MessageContent::Text("System prompt".into())],
                    cache: false,
                    reasoning_details: None,
                },
                LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![MessageContent::Text("Hello".into())],
                    cache: false,
                    reasoning_details: None,
                },
            ],
            temperature: Some(0.5),
            tools: vec![],
            tool_choice: None,
            thread_id: None,
            prompt_id: None,
            intent: None,
            mode: None,
            stop: vec![],
            thinking_allowed: true,
        };

        let mistral_request = into_mistral(request, mistral::Model::MistralSmallLatest, None);

        assert_eq!(mistral_request.model, "mistral-small-latest");
        assert_eq!(mistral_request.temperature, Some(0.5));
        assert_eq!(mistral_request.messages.len(), 2);
        assert!(mistral_request.stream);
    }

    #[test]
    fn test_into_mistral_with_image() {
        let request = LanguageModelRequest {
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec![
                    MessageContent::Text("What's in this image?".into()),
                    MessageContent::Image(LanguageModelImage {
                        source: "base64data".into(),
                        size: None,
                    }),
                ],
                cache: false,
                reasoning_details: None,
            }],
            tools: vec![],
            tool_choice: None,
            temperature: None,
            thread_id: None,
            prompt_id: None,
            intent: None,
            mode: None,
            stop: vec![],
            thinking_allowed: true,
        };

        let mistral_request = into_mistral(request, mistral::Model::Pixtral12BLatest, None);

        assert_eq!(mistral_request.messages.len(), 1);
        assert!(matches!(
            &mistral_request.messages[0],
            mistral::RequestMessage::User {
                content: mistral::MessageContent::Multipart { .. }
            }
        ));

        if let mistral::RequestMessage::User {
            content: mistral::MessageContent::Multipart { content },
        } = &mistral_request.messages[0]
        {
            assert_eq!(content.len(), 2);
            assert!(matches!(
                &content[0],
                mistral::MessagePart::Text { text } if text == "What's in this image?"
            ));
            assert!(matches!(
                &content[1],
                mistral::MessagePart::ImageUrl { image_url } if image_url.starts_with("data:image/png;base64,")
            ));
        }
    }
}
