use anyhow::{Result, anyhow};
use collections::HashMap;
use futures::stream::{BoxStream, Stream};
use futures::{FutureExt, StreamExt, future, future::BoxFuture};
use gpui::{AnyView, App, AsyncApp, Context, Entity, SharedString, Task, Window};
use http_client::HttpClient;
use language_model::{
    ApiKeyState, AuthenticateError, EnvVar, IconOrSvg, LanguageModel, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelId, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelRequest, LanguageModelToolChoice, LanguageModelToolResultContent,
    LanguageModelToolUse, MessageContent, RateLimiter, Role, StopReason, TokenUsage, env_var,
};
use minimax::{self, MINIMAX_API_URL_CHINA};
use settings::{Settings, SettingsStore};
use std::pin::Pin;
use std::str::FromStr;
use std::sync::{Arc, LazyLock};
use ui::{ButtonLink, ConfiguredApiCard, List, ListBulletItem, prelude::*};
use ui_input::InputField;
use util::ResultExt;

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("minimax");
const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("MiniMax");

const API_KEY_ENV_VAR_NAME: &str = "MINIMAX_API_KEY";
static API_KEY_ENV_VAR: LazyLock<EnvVar> = env_var!(API_KEY_ENV_VAR_NAME);

#[derive(Default)]
struct RawToolCall {
    id: String,
    name: String,
    arguments: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MiniMaxSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
}

pub struct MiniMaxLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

pub struct State {
    api_key_state: ApiKeyState,
}

impl State {
    fn is_authenticated(&self) -> bool {
        self.api_key_state.has_key()
    }

    fn set_api_key(&mut self, api_key: Option<String>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let api_url = MiniMaxLanguageModelProvider::api_url(cx);
        self.api_key_state
            .store(api_url, api_key, |this| &mut this.api_key_state, cx)
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let api_url = MiniMaxLanguageModelProvider::api_url(cx);
        self.api_key_state
            .load_if_needed(api_url, |this| &mut this.api_key_state, cx)
    }
}

impl MiniMaxLanguageModelProvider {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
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
            }
        });

        Self { http_client, state }
    }

    fn create_language_model(&self, model: minimax::Model) -> Arc<dyn LanguageModel> {
        Arc::new(MiniMaxLanguageModel {
            id: LanguageModelId::from(model.id().to_string()),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }

    fn settings(cx: &App) -> &MiniMaxSettings {
        &crate::AllLanguageModelSettings::get_global(cx).minimax
    }

    fn api_url(cx: &App) -> SharedString {
        let api_url = &Self::settings(cx).api_url;
        if api_url.is_empty() {
            MINIMAX_API_URL_CHINA.into()
        } else {
            SharedString::new(api_url.as_str())
        }
    }
}

impl LanguageModelProviderState for MiniMaxLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for MiniMaxLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiOpenAiCompat)
    }

    fn default_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(minimax::Model::default()))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(minimax::Model::default_fast()))
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        use std::collections::BTreeMap;
        let mut models = BTreeMap::default();

        models.insert("MiniMax-M2.1", minimax::Model::M2_1);
        models.insert("MiniMax-M2", minimax::Model::M2);
        // Now Minimax-M2.1-lightning is not supported due to its capabilities.
        // models.insert("MiniMax-M2.1-lightning", minimax::Model::M2_1Lightning);

        for available_model in &Self::settings(cx).available_models {
            models.insert(
                &available_model.name,
                minimax::Model::Custom {
                    name: available_model.name.clone(),
                    display_name: available_model.display_name.clone(),
                    max_tokens: available_model.max_tokens,
                    max_output_tokens: available_model.max_output_tokens,
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
            .update(cx, |state, cx| state.set_api_key(None, cx))
    }
}

pub struct MiniMaxLanguageModel {
    id: LanguageModelId,
    model: minimax::Model,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl MiniMaxLanguageModel {
    fn stream_completion(
        &self,
        request: minimax::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<minimax::StreamResponse>>>> {
        let http_client = self.http_client.clone();

        let Ok((api_key, api_url)) = self.state.read_with(cx, |state, cx| {
            let api_url = MiniMaxLanguageModelProvider::api_url(cx);
            (state.api_key_state.key(&api_url), api_url)
        }) else {
            return future::ready(Err(anyhow!("App state dropped"))).boxed();
        };

        let future = self.request_limiter.stream(async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey {
                    provider: PROVIDER_NAME,
                });
            };
            let request =
                minimax::stream_completion(http_client.as_ref(), &api_url, &api_key, request);
            let response = request.await?;
            Ok(response)
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

impl LanguageModel for MiniMaxLanguageModel {
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

    fn supports_tool_choice(&self, _choice: LanguageModelToolChoice) -> bool {
        true
    }

    fn supports_images(&self) -> bool {
        false
    }

    fn telemetry_id(&self) -> String {
        format!("minimax/{}", self.model.id())
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
        // reasoning_split defaults to true, which separates thinking content into reasoning_details
        // When false, thinking content comes as <thinking>...</thinking> tags in the content field
        let request = into_minimax(request, &self.model, self.max_output_tokens(), true);
        let stream = self.stream_completion(request, cx);

        async move {
            let mapper = MiniMaxEventMapper::new();
            Ok(mapper.map_stream(stream.await?).boxed())
        }
        .boxed()
    }
}

pub fn into_minimax(
    request: LanguageModelRequest,
    model: &minimax::Model,
    max_output_tokens: Option<u64>,
    reasoning_split: bool,
) -> minimax::Request {
    let mut messages = Vec::new();

    for message in request.messages {
        for content in message.content {
            match content {
                MessageContent::Text(text) => messages.push(match message.role {
                    Role::User => minimax::RequestMessage::User { content: text },
                    Role::Assistant => minimax::RequestMessage::Assistant {
                        content: Some(text),
                        tool_calls: Vec::new(),
                        reasoning_details: None,
                    },
                    Role::System => minimax::RequestMessage::System { content: text },
                }),
                MessageContent::Thinking { text, .. } => {
                    if let Some(last_msg) = messages.last_mut() {
                        if let minimax::RequestMessage::Assistant {
                            reasoning_details, ..
                        } = last_msg
                        {
                            let detail = minimax::ReasoningDetail {
                                text,
                                signature: None,
                            };
                            if let Some(details) = reasoning_details {
                                details.push(detail);
                            } else {
                                *reasoning_details = Some(vec![detail]);
                            }
                        }
                    }
                }
                MessageContent::RedactedThinking(_) => {}
                MessageContent::Image(_) => {}
                MessageContent::ToolUse(tool_use) => {
                    let tool_call = minimax::ToolCall {
                        id: tool_use.id.to_string(),
                        content: minimax::ToolCallContent::Function {
                            function: minimax::FunctionContent {
                                name: tool_use.name.to_string(),
                                arguments: serde_json::to_string(&tool_use.input)
                                    .unwrap_or_default(),
                            },
                        },
                    };

                    if let Some(minimax::RequestMessage::Assistant { tool_calls, .. }) =
                        messages.last_mut()
                    {
                        tool_calls.push(tool_call);
                    } else {
                        messages.push(minimax::RequestMessage::Assistant {
                            content: None,
                            tool_calls: vec![tool_call],
                            reasoning_details: None,
                        });
                    }
                }
                MessageContent::ToolResult(tool_result) => {
                    match &tool_result.content {
                        LanguageModelToolResultContent::Text(text) => {
                            messages.push(minimax::RequestMessage::Tool {
                                content: text.to_string(),
                                tool_call_id: tool_result.tool_use_id.to_string(),
                            });
                        }
                        LanguageModelToolResultContent::Image(_) => {}
                    };
                }
            }
        }
    }

    minimax::Request {
        model: model.id().to_string(),
        messages,
        stream: true,
        max_tokens: max_output_tokens,
        temperature: request.temperature,
        stop: Vec::new(),
        response_format: None,
        tools: request
            .tools
            .into_iter()
            .map(|tool| minimax::ToolDefinition::Function {
                function: minimax::FunctionDefinition {
                    name: tool.name,
                    description: Some(tool.description),
                    parameters: Some(tool.input_schema),
                },
            })
            .collect(),
        tool_choice: None,
        reasoning_split: reasoning_split,
    }
}

/// Extract thinking content from native format content.
/// When reasoning_split=false, thinking content is embedded as <thinking>...</thinking> tags.
/// Returns (Some(extracted_thinking), remaining_text) or (None, full_text) if no thinking found.
fn extract_thinking(content: &str) -> (Option<String>, String) {
    // MiniMax uses <thinking>...</thinking> tags in native format
    let start_tag = "<think>";
    let end_tag = "</think>";

    if let Some(start) = content.find(start_tag) {
        if let Some(end) = content.find(end_tag) {
            let thinking = content[start + start_tag.len()..end].to_string();
            let after_end = content[end + end_tag.len()..].to_string();
            let before_start = &content[..start];
            let remaining_text = format!("{}{}", before_start, after_end);
            return (Some(thinking), remaining_text);
        }
    }
    (None, content.to_string())
}

pub struct MiniMaxEventMapper {
    tool_calls_by_index: HashMap<usize, RawToolCall>,
}

impl MiniMaxEventMapper {
    pub fn new() -> Self {
        Self {
            tool_calls_by_index: HashMap::default(),
        }
    }

    pub fn map_stream(
        mut self,
        events: Pin<Box<dyn Send + Stream<Item = Result<minimax::StreamResponse>>>>,
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
        event: minimax::StreamResponse,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        let Some(choice) = event.choices.first() else {
            return vec![Err(LanguageModelCompletionError::from(anyhow!(
                "Response contained no choices"
            )))];
        };

        let mut events = Vec::new();

        if let Some(content) = choice.delta.content.clone() {
            // Check for native thinking format (reasoning_split=false)
            // Thinking content is embedded as <thinking>...</thinking> or similar tags
            let (thinking_content, remaining_text) = extract_thinking(&content);
            if let Some(thinking) = thinking_content {
                events.push(Ok(LanguageModelCompletionEvent::Thinking {
                    text: thinking,
                    signature: None,
                }));
            }
            if !remaining_text.is_empty() {
                events.push(Ok(LanguageModelCompletionEvent::Text(remaining_text)));
            }
        }

        if let Some(reasoning_details) = choice.delta.reasoning_details.as_ref() {
            for detail in reasoning_details {
                events.push(Ok(LanguageModelCompletionEvent::Thinking {
                    text: detail.text.clone(),
                    signature: detail.signature.clone(),
                }));
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
                input_tokens: usage.prompt_tokens.unwrap_or(0),
                output_tokens: usage.completion_tokens.unwrap_or(0),
                cache_creation_input_tokens: 0,
                cache_read_input_tokens: 0,
            })));
        }

        match choice.finish_reason.as_deref() {
            Some("stop") | Some("completion") => {
                events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::EndTurn)));
            }
            Some("tool_calls") => {
                events.extend(self.tool_calls_by_index.drain().map(|(_, tool_call)| {
                    match serde_json::Value::from_str(&tool_call.arguments) {
                        Ok(input) => Ok(LanguageModelCompletionEvent::ToolUse(
                            LanguageModelToolUse {
                                id: tool_call.id.clone().into(),
                                name: tool_call.name.as_str().into(),
                                is_input_complete: true,
                                input,
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
            Some(reason) => {
                log::error!("Unexpected MiniMax finish_reason: {reason:?}",);
                events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::EndTurn)));
            }
            None => {}
        }

        events
    }
}

struct ConfigurationView {
    api_key_editor: Entity<InputField>,
    state: Entity<State>,
    load_credentials_task: Option<Task<()>>,
}

impl ConfigurationView {
    fn new(state: Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let api_key_editor =
            cx.new(|cx| InputField::new(window, cx, "mk-00000000000000000000000000000000"));

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

    fn save_api_key(&mut self, _: &menu::Confirm, _window: &mut Window, cx: &mut Context<Self>) {
        let api_key = self.api_key_editor.read(cx).text(cx).trim().to_string();
        if api_key.is_empty() {
            return;
        }

        let state = self.state.clone();
        cx.spawn(async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(Some(api_key), cx))?
                .await
        })
        .detach_and_log_err(cx);
    }

    fn reset_api_key(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.api_key_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn(async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(None, cx))?
                .await
        })
        .detach_and_log_err(cx);
    }

    fn should_render_editor(&self, cx: &mut Context<Self>) -> bool {
        !self.state.read(cx).is_authenticated()
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let env_var_set = self.state.read(cx).api_key_state.is_from_env_var();
        let configured_card_label = if env_var_set {
            format!("API key set in {API_KEY_ENV_VAR_NAME} environment variable")
        } else {
            let api_url = MiniMaxLanguageModelProvider::api_url(cx);
            if api_url == MINIMAX_API_URL_CHINA {
                "API key configured".to_string()
            } else {
                format!("API key configured for {}", api_url)
            }
        };

        if self.load_credentials_task.is_some() {
            div()
                .child(Label::new("Loading credentials..."))
                .into_any_element()
        } else if self.should_render_editor(cx) {
            v_flex()
                .size_full()
                .on_action(cx.listener(Self::save_api_key))
                .child(Label::new("To use MiniMax in Zed, you need an API key:"))
                .child(
                    List::new()
                        .child(
                            ListBulletItem::new("")
                                .child(Label::new("Get your API key from the"))
                                .child(ButtonLink::new(
                                    "MiniMax console",
                                    "https://platform.minimaxi.com/api-keys",
                                )),
                        )
                        .child(ListBulletItem::new(
                            "Paste your API key below and hit enter to start using the assistant",
                        )),
                )
                .child(self.api_key_editor.clone())
                .child(
                    Label::new(format!(
                        "Or set the {API_KEY_ENV_VAR_NAME} environment variable."
                    ))
                    .size(LabelSize::Small)
                    .color(Color::Muted),
                )
                .into_any_element()
        } else {
            ConfiguredApiCard::new(configured_card_label)
                .disabled(env_var_set)
                .on_click(cx.listener(|this, _, window, cx| this.reset_api_key(window, cx)))
                .into_any_element()
        }
    }
}

pub use settings::MiniMaxAvailableModel as AvailableModel;
