use std::pin::Pin;
use std::str::FromStr as _;
use std::sync::Arc;

use anyhow::{Result, anyhow};
use collections::HashMap;
use copilot::copilot_chat::{
    ChatMessage, CopilotChat, Model as CopilotChatModel, Request as CopilotChatRequest,
    ResponseEvent, Tool, ToolCall,
};
use copilot::{Copilot, Status};
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use futures::{FutureExt, Stream, StreamExt};
use gpui::{
    Action, Animation, AnimationExt, AnyView, App, AsyncApp, Entity, Render, Subscription, Task,
    Transformation, percentage, svg,
};
use language_model::{
    AuthenticateError, LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelId, LanguageModelName, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest,
    LanguageModelRequestMessage, LanguageModelToolUse, MessageContent, RateLimiter, Role,
    StopReason,
};
use settings::SettingsStore;
use std::time::Duration;
use strum::IntoEnumIterator;
use ui::prelude::*;

use super::anthropic::count_anthropic_tokens;
use super::google::count_google_tokens;
use super::open_ai::count_open_ai_tokens;

const PROVIDER_ID: &str = "copilot_chat";
const PROVIDER_NAME: &str = "GitHub Copilot Chat";

#[derive(Default, Clone, Debug, PartialEq)]
pub struct CopilotChatSettings {}

pub struct CopilotChatLanguageModelProvider {
    state: Entity<State>,
}

pub struct State {
    _copilot_chat_subscription: Option<Subscription>,
    _settings_subscription: Subscription,
}

impl State {
    fn is_authenticated(&self, cx: &App) -> bool {
        CopilotChat::global(cx)
            .map(|m| m.read(cx).is_authenticated())
            .unwrap_or(false)
    }
}

impl CopilotChatLanguageModelProvider {
    pub fn new(cx: &mut App) -> Self {
        let state = cx.new(|cx| {
            let _copilot_chat_subscription = CopilotChat::global(cx)
                .map(|copilot_chat| cx.observe(&copilot_chat, |_, _, cx| cx.notify()));
            State {
                _copilot_chat_subscription,
                _settings_subscription: cx.observe_global::<SettingsStore>(|_, cx| {
                    cx.notify();
                }),
            }
        });

        Self { state }
    }

    fn create_language_model(&self, model: CopilotChatModel) -> Arc<dyn LanguageModel> {
        Arc::new(CopilotChatLanguageModel {
            model,
            request_limiter: RateLimiter::new(4),
        })
    }
}

impl LanguageModelProviderState for CopilotChatLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<gpui::Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for CopilotChatLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        LanguageModelProviderId(PROVIDER_ID.into())
    }

    fn name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn icon(&self) -> IconName {
        IconName::Copilot
    }

    fn default_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(CopilotChatModel::default()))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(CopilotChatModel::default_fast()))
    }

    fn provided_models(&self, _cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        CopilotChatModel::iter()
            .map(|model| self.create_language_model(model))
            .collect()
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated(cx)
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        if self.is_authenticated(cx) {
            return Task::ready(Ok(()));
        };

        let Some(copilot) = Copilot::global(cx) else {
            return Task::ready( Err(anyhow!(
                "Copilot must be enabled for Copilot Chat to work. Please enable Copilot and try again."
            ).into()));
        };

        let err = match copilot.read(cx).status() {
            Status::Authorized => return Task::ready(Ok(())),
            Status::Disabled => anyhow!(
                "Copilot must be enabled for Copilot Chat to work. Please enable Copilot and try again."
            ),
            Status::Error(err) => anyhow!(format!(
                "Received the following error while signing into Copilot: {err}"
            )),
            Status::Starting { task: _ } => anyhow!(
                "Copilot is still starting, please wait for Copilot to start then try again"
            ),
            Status::Unauthorized => anyhow!(
                "Unable to authorize with Copilot. Please make sure that you have an active Copilot and Copilot Chat subscription."
            ),
            Status::SignedOut { .. } => {
                anyhow!("You have signed out of Copilot. Please sign in to Copilot and try again.")
            }
            Status::SigningIn { prompt: _ } => anyhow!("Still signing into Copilot..."),
        };

        Task::ready(Err(err.into()))
    }

    fn configuration_view(&self, _: &mut Window, cx: &mut App) -> AnyView {
        let state = self.state.clone();
        cx.new(|cx| ConfigurationView::new(state, cx)).into()
    }

    fn reset_credentials(&self, _cx: &mut App) -> Task<Result<()>> {
        Task::ready(Err(anyhow!(
            "Signing out of GitHub Copilot Chat is currently not supported."
        )))
    }
}

pub struct CopilotChatLanguageModel {
    model: CopilotChatModel,
    request_limiter: RateLimiter,
}

impl LanguageModel for CopilotChatLanguageModel {
    fn id(&self) -> LanguageModelId {
        LanguageModelId::from(self.model.id().to_string())
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
        match self.model {
            CopilotChatModel::Gpt4o
            | CopilotChatModel::Gpt4_1
            | CopilotChatModel::O4Mini
            | CopilotChatModel::Claude3_5Sonnet
            | CopilotChatModel::Claude3_7Sonnet => true,
            _ => false,
        }
    }

    fn telemetry_id(&self) -> String {
        format!("copilot_chat/{}", self.model.id())
    }

    fn max_token_count(&self) -> usize {
        self.model.max_token_count()
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &App,
    ) -> BoxFuture<'static, Result<usize>> {
        match self.model {
            CopilotChatModel::Claude3_5Sonnet => count_anthropic_tokens(request, cx),
            CopilotChatModel::Claude3_7Sonnet => count_anthropic_tokens(request, cx),
            CopilotChatModel::Claude3_7SonnetThinking => count_anthropic_tokens(request, cx),
            CopilotChatModel::Gemini20Flash | CopilotChatModel::Gemini25Pro => {
                count_google_tokens(request, cx)
            }
            _ => {
                let model = match self.model {
                    CopilotChatModel::Gpt4o => open_ai::Model::FourOmni,
                    CopilotChatModel::Gpt4 => open_ai::Model::Four,
                    CopilotChatModel::Gpt4_1 => open_ai::Model::FourPointOne,
                    CopilotChatModel::Gpt3_5Turbo => open_ai::Model::ThreePointFiveTurbo,
                    CopilotChatModel::O1 => open_ai::Model::O1,
                    CopilotChatModel::O3Mini => open_ai::Model::O3Mini,
                    CopilotChatModel::O3 => open_ai::Model::O3,
                    CopilotChatModel::O4Mini => open_ai::Model::O4Mini,
                    CopilotChatModel::Claude3_5Sonnet
                    | CopilotChatModel::Claude3_7Sonnet
                    | CopilotChatModel::Claude3_7SonnetThinking
                    | CopilotChatModel::Gemini20Flash
                    | CopilotChatModel::Gemini25Pro => {
                        unreachable!()
                    }
                };
                count_open_ai_tokens(request, model, cx)
            }
        }
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            BoxStream<'static, Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>,
        >,
    > {
        if let Some(message) = request.messages.last() {
            if message.contents_empty() {
                const EMPTY_PROMPT_MSG: &str =
                    "Empty prompts aren't allowed. Please provide a non-empty prompt.";
                return futures::future::ready(Err(anyhow::anyhow!(EMPTY_PROMPT_MSG))).boxed();
            }

            // Copilot Chat has a restriction that the final message must be from the user.
            // While their API does return an error message for this, we can catch it earlier
            // and provide a more helpful error message.
            if !matches!(message.role, Role::User) {
                const USER_ROLE_MSG: &str = "The final message must be from the user. To provide a system prompt, you must provide the system prompt followed by a user prompt.";
                return futures::future::ready(Err(anyhow::anyhow!(USER_ROLE_MSG))).boxed();
            }
        }

        let copilot_request = match self.to_copilot_chat_request(request) {
            Ok(request) => request,
            Err(err) => return futures::future::ready(Err(err)).boxed(),
        };
        let is_streaming = copilot_request.stream;

        let request_limiter = self.request_limiter.clone();
        let future = cx.spawn(async move |cx| {
            let request = CopilotChat::stream_completion(copilot_request, cx.clone());
            request_limiter
                .stream(async move {
                    let response = request.await?;
                    Ok(map_to_language_model_completion_events(
                        response,
                        is_streaming,
                    ))
                })
                .await
        });
        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

pub fn map_to_language_model_completion_events(
    events: Pin<Box<dyn Send + Stream<Item = Result<ResponseEvent>>>>,
    is_streaming: bool,
) -> impl Stream<Item = Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
    #[derive(Default)]
    struct RawToolCall {
        id: String,
        name: String,
        arguments: String,
    }

    struct State {
        events: Pin<Box<dyn Send + Stream<Item = Result<ResponseEvent>>>>,
        tool_calls_by_index: HashMap<usize, RawToolCall>,
    }

    futures::stream::unfold(
        State {
            events,
            tool_calls_by_index: HashMap::default(),
        },
        move |mut state| async move {
            if let Some(event) = state.events.next().await {
                match event {
                    Ok(event) => {
                        let Some(choice) = event.choices.first() else {
                            return Some((
                                vec![Err(anyhow!("Response contained no choices").into())],
                                state,
                            ));
                        };

                        let delta = if is_streaming {
                            choice.delta.as_ref()
                        } else {
                            choice.message.as_ref()
                        };

                        let Some(delta) = delta else {
                            return Some((
                                vec![Err(anyhow!("Response contained no delta").into())],
                                state,
                            ));
                        };

                        let mut events = Vec::new();
                        if let Some(content) = delta.content.clone() {
                            events.push(Ok(LanguageModelCompletionEvent::Text(content)));
                        }

                        for tool_call in &delta.tool_calls {
                            let entry = state
                                .tool_calls_by_index
                                .entry(tool_call.index)
                                .or_default();

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

                        match choice.finish_reason.as_deref() {
                            Some("stop") => {
                                events.push(Ok(LanguageModelCompletionEvent::Stop(
                                    StopReason::EndTurn,
                                )));
                            }
                            Some("tool_calls") => {
                                events.extend(state.tool_calls_by_index.drain().map(
                                    |(_, tool_call)| match serde_json::Value::from_str(
                                        &tool_call.arguments,
                                    ) {
                                        Ok(input) => Ok(LanguageModelCompletionEvent::ToolUse(
                                            LanguageModelToolUse {
                                                id: tool_call.id.clone().into(),
                                                name: tool_call.name.as_str().into(),
                                                is_input_complete: true,
                                                input,
                                                raw_input: tool_call.arguments.clone(),
                                            },
                                        )),
                                        Err(error) => {
                                            Err(LanguageModelCompletionError::BadInputJson {
                                                id: tool_call.id.into(),
                                                tool_name: tool_call.name.as_str().into(),
                                                raw_input: tool_call.arguments.into(),
                                                json_parse_error: error.to_string(),
                                            })
                                        }
                                    },
                                ));

                                events.push(Ok(LanguageModelCompletionEvent::Stop(
                                    StopReason::ToolUse,
                                )));
                            }
                            Some(stop_reason) => {
                                log::error!("Unexpected Copilot Chat stop_reason: {stop_reason:?}");
                                events.push(Ok(LanguageModelCompletionEvent::Stop(
                                    StopReason::EndTurn,
                                )));
                            }
                            None => {}
                        }

                        return Some((events, state));
                    }
                    Err(err) => return Some((vec![Err(anyhow!(err).into())], state)),
                }
            }

            None
        },
    )
    .flat_map(futures::stream::iter)
}

impl CopilotChatLanguageModel {
    pub fn to_copilot_chat_request(
        &self,
        request: LanguageModelRequest,
    ) -> Result<CopilotChatRequest> {
        let model = self.model.clone();

        let mut request_messages: Vec<LanguageModelRequestMessage> = Vec::new();
        for message in request.messages {
            if let Some(last_message) = request_messages.last_mut() {
                if last_message.role == message.role {
                    last_message.content.extend(message.content);
                } else {
                    request_messages.push(message);
                }
            } else {
                request_messages.push(message);
            }
        }

        let mut tool_called = false;
        let mut messages: Vec<ChatMessage> = Vec::new();
        for message in request_messages {
            let text_content = {
                let mut buffer = String::new();
                for string in message.content.iter().filter_map(|content| match content {
                    MessageContent::Text(text) | MessageContent::Thinking { text, .. } => {
                        Some(text.as_str())
                    }
                    MessageContent::ToolUse(_)
                    | MessageContent::RedactedThinking(_)
                    | MessageContent::ToolResult(_)
                    | MessageContent::Image(_) => None,
                }) {
                    buffer.push_str(string);
                }

                buffer
            };

            match message.role {
                Role::User => {
                    for content in &message.content {
                        if let MessageContent::ToolResult(tool_result) = content {
                            messages.push(ChatMessage::Tool {
                                tool_call_id: tool_result.tool_use_id.to_string(),
                                content: tool_result.content.to_string(),
                            });
                        }
                    }

                    if !text_content.is_empty() {
                        messages.push(ChatMessage::User {
                            content: text_content,
                        });
                    }
                }
                Role::Assistant => {
                    let mut tool_calls = Vec::new();
                    for content in &message.content {
                        if let MessageContent::ToolUse(tool_use) = content {
                            tool_called = true;
                            tool_calls.push(ToolCall {
                                id: tool_use.id.to_string(),
                                content: copilot::copilot_chat::ToolCallContent::Function {
                                    function: copilot::copilot_chat::FunctionContent {
                                        name: tool_use.name.to_string(),
                                        arguments: serde_json::to_string(&tool_use.input)?,
                                    },
                                },
                            });
                        }
                    }

                    messages.push(ChatMessage::Assistant {
                        content: if text_content.is_empty() {
                            None
                        } else {
                            Some(text_content)
                        },
                        tool_calls,
                    });
                }
                Role::System => messages.push(ChatMessage::System {
                    content: message.string_contents(),
                }),
            }
        }

        let mut tools = request
            .tools
            .iter()
            .map(|tool| Tool::Function {
                function: copilot::copilot_chat::Function {
                    name: tool.name.clone(),
                    description: tool.description.clone(),
                    parameters: tool.input_schema.clone(),
                },
            })
            .collect::<Vec<_>>();

        // The API will return a Bad Request (with no error message) when tools
        // were used previously in the conversation but no tools are provided as
        // part of this request. Inserting a dummy tool seems to circumvent this
        // error.
        if tool_called && tools.is_empty() {
            tools.push(Tool::Function {
                function: copilot::copilot_chat::Function {
                    name: "noop".to_string(),
                    description: "No operation".to_string(),
                    parameters: serde_json::json!({}),
                },
            });
        }

        Ok(CopilotChatRequest {
            intent: true,
            n: 1,
            stream: model.uses_streaming(),
            temperature: 0.1,
            model,
            messages,
            tools,
            tool_choice: None,
        })
    }
}

struct ConfigurationView {
    copilot_status: Option<copilot::Status>,
    state: Entity<State>,
    _subscription: Option<Subscription>,
}

impl ConfigurationView {
    pub fn new(state: Entity<State>, cx: &mut Context<Self>) -> Self {
        let copilot = Copilot::global(cx);

        Self {
            copilot_status: copilot.as_ref().map(|copilot| copilot.read(cx).status()),
            state,
            _subscription: copilot.as_ref().map(|copilot| {
                cx.observe(copilot, |this, model, cx| {
                    this.copilot_status = Some(model.read(cx).status());
                    cx.notify();
                })
            }),
        }
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.state.read(cx).is_authenticated(cx) {
            h_flex()
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
                        .child(Label::new("Authorized")),
                )
                .child(
                    Button::new("sign_out", "Sign Out")
                        .label_size(LabelSize::Small)
                        .on_click(|_, window, cx| {
                            window.dispatch_action(copilot::SignOut.boxed_clone(), cx);
                        }),
                )
        } else {
            let loading_icon = Icon::new(IconName::ArrowCircle).with_animation(
                "arrow-circle",
                Animation::new(Duration::from_secs(4)).repeat(),
                |icon, delta| icon.transform(Transformation::rotate(percentage(delta))),
            );

            const ERROR_LABEL: &str = "Copilot Chat requires an active GitHub Copilot subscription. Please ensure Copilot is configured and try again, or use a different Assistant provider.";

            match &self.copilot_status {
                Some(status) => match status {
                    Status::Starting { task: _ } => h_flex()
                        .gap_2()
                        .child(loading_icon)
                        .child(Label::new("Starting Copilot…")),
                    Status::SigningIn { prompt: _ }
                    | Status::SignedOut {
                        awaiting_signing_in: true,
                    } => h_flex()
                        .gap_2()
                        .child(loading_icon)
                        .child(Label::new("Signing into Copilot…")),
                    Status::Error(_) => {
                        const LABEL: &str = "Copilot had issues starting. Please try restarting it. If the issue persists, try reinstalling Copilot.";
                        v_flex()
                            .gap_6()
                            .child(Label::new(LABEL))
                            .child(svg().size_8().path(IconName::CopilotError.path()))
                    }
                    _ => {
                        const LABEL: &str = "To use Zed's assistant with GitHub Copilot, you need to be logged in to GitHub. Note that your GitHub account must have an active Copilot Chat subscription.";
                        v_flex().gap_2().child(Label::new(LABEL)).child(
                            Button::new("sign_in", "Sign in to use GitHub Copilot")
                                .icon_color(Color::Muted)
                                .icon(IconName::Github)
                                .icon_position(IconPosition::Start)
                                .icon_size(IconSize::Medium)
                                .full_width()
                                .on_click(|_, window, cx| copilot::initiate_sign_in(window, cx)),
                        )
                    }
                },
                None => v_flex().gap_6().child(Label::new(ERROR_LABEL)),
            }
        }
    }
}
