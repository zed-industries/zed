use std::pin::Pin;
use std::str::FromStr as _;
use std::sync::Arc;

use anyhow::{Result, anyhow};
use collections::HashMap;
use copilot::copilot_chat::{
    ChatMessage, CopilotChat, Model as CopilotChatModel, Request as CopilotChatRequest,
    ResponseEvent, Tool,
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
    AuthenticateError, LanguageModel, LanguageModelCompletionEvent, LanguageModelId,
    LanguageModelName, LanguageModelProvider, LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelProviderState, LanguageModelRequest, LanguageModelToolUse, MessageContent,
    RateLimiter, Role,
};
use settings::SettingsStore;
use std::time::Duration;
use strum::IntoEnumIterator;
use ui::prelude::*;
use util::maybe;

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
        let model = CopilotChatModel::default();
        Some(Arc::new(CopilotChatLanguageModel {
            model,
            request_limiter: RateLimiter::new(4),
        }) as Arc<dyn LanguageModel>)
    }

    fn provided_models(&self, _cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        CopilotChatModel::iter()
            .map(|model| {
                Arc::new(CopilotChatLanguageModel {
                    model,
                    request_limiter: RateLimiter::new(4),
                }) as Arc<dyn LanguageModel>
            })
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
        true
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
            CopilotChatModel::Gemini20Flash => count_google_tokens(request, cx),
            _ => {
                let model = match self.model {
                    CopilotChatModel::Gpt4o => open_ai::Model::FourOmni,
                    CopilotChatModel::Gpt4 => open_ai::Model::Four,
                    CopilotChatModel::Gpt3_5Turbo => open_ai::Model::ThreePointFiveTurbo,
                    CopilotChatModel::O1 | CopilotChatModel::O3Mini => open_ai::Model::Four,
                    CopilotChatModel::Claude3_5Sonnet
                    | CopilotChatModel::Claude3_7Sonnet
                    | CopilotChatModel::Claude3_7SonnetThinking
                    | CopilotChatModel::Gemini20Flash => {
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
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<LanguageModelCompletionEvent>>>> {
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

        let copilot_request = self.to_copilot_chat_request(request);
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
) -> impl Stream<Item = Result<LanguageModelCompletionEvent>> {
    const NO_CHOICES_ERROR_MESSAGE: &str = "The Copilot Chat API returned a response with no choices, but hadn't finished the message yet. Please try again.";
    const NO_MESSAGE_CONTENT_ERROR_MESSAGE: &str =
        "The Copilot Chat API returned a response with no message content";

    #[derive(Default)]
    struct RawFunctionCall {
        id: String,
        name: String,
        arguments: String,
    }

    struct State {
        events: Pin<Box<dyn Send + Stream<Item = Result<ResponseEvent>>>>,
        function_calls_by_index: HashMap<usize, RawFunctionCall>,
        // usage: Usage,
        // stop_reason: StopReason,
    }

    futures::stream::unfold(
        State {
            events,
            function_calls_by_index: HashMap::default(),
        },
        move |mut state| async move {
            if let Some(event) = state.events.next().await {
                match event {
                    Ok(result) => {
                        let choice = result.choices.first();
                        match choice {
                            Some(choice) if !is_streaming => match &choice.message {
                                Some(msg) => {
                                    let mut events = Vec::new();
                                    if let Some(content) = msg.content.clone() {
                                        events
                                            .push(Ok(LanguageModelCompletionEvent::Text(content)));
                                    }

                                    Some((events, state))
                                }
                                None => Some((
                                    vec![Err(anyhow::anyhow!(NO_MESSAGE_CONTENT_ERROR_MESSAGE))],
                                    state,
                                )),
                            },
                            Some(choice) => {
                                let mut events = Vec::new();

                                if let Some(msg) = choice.delta.as_ref() {
                                    for tool_call in &msg.tool_calls {
                                        let tool = state
                                            .function_calls_by_index
                                            .entry(tool_call.index)
                                            .or_default();
                                        if let Some(tool_id) = tool_call.id.clone() {
                                            tool.id = tool_id;
                                        }
                                        if let Some(tool_name) = tool_call
                                            .function
                                            .as_ref()
                                            .and_then(|function| function.name.as_ref())
                                        {
                                            tool.name = tool_name.clone();
                                        }
                                        if let Some(arguments) = tool_call
                                            .function
                                            .as_ref()
                                            .and_then(|function| function.arguments.as_ref())
                                        {
                                            tool.arguments.push_str(&arguments);
                                        }
                                    }
                                    if let Some(content) = msg.content.clone() {
                                        events
                                            .push(Ok(LanguageModelCompletionEvent::Text(content)));
                                    }
                                }

                                match choice.finish_reason.as_deref() {
                                    Some("stop") => {
                                        events.push(Ok(LanguageModelCompletionEvent::Stop(
                                            language_model::StopReason::EndTurn,
                                        )));
                                    }
                                    Some("tool_calls") => {
                                        events.extend(state.function_calls_by_index.drain().map(
                                            |(_, function_call)| {
                                                maybe!({
                                                    Ok(LanguageModelCompletionEvent::ToolUse(
                                                        LanguageModelToolUse {
                                                            id: function_call.id.into(),
                                                            name: function_call
                                                                .name
                                                                .as_str()
                                                                .into(),
                                                            input: serde_json::Value::from_str(
                                                                &function_call.arguments,
                                                            )?,
                                                        },
                                                    ))
                                                })
                                            },
                                        ));

                                        events.push(Ok(LanguageModelCompletionEvent::Stop(
                                            language_model::StopReason::ToolUse,
                                        )));
                                    }
                                    _ => {}
                                }

                                Some((events, state))
                            }
                            None => {
                                Some((vec![Err(anyhow::anyhow!(NO_CHOICES_ERROR_MESSAGE))], state))
                            }
                        }
                    }
                    Err(err) => Some((vec![Err(err)], state)),
                }
            } else {
                None
            }
        },
    )
    .flat_map(futures::stream::iter)
}

impl CopilotChatLanguageModel {
    pub fn to_copilot_chat_request(&self, request: LanguageModelRequest) -> CopilotChatRequest {
        let model = self.model.clone();
        let messages = request
            .messages
            .into_iter()
            .flat_map(|message| {
                message
                    .content
                    .into_iter()
                    .filter_map(move |content| match content {
                        MessageContent::Text(text) => Some(match message.role {
                            Role::User => ChatMessage::User { content: text },
                            Role::Assistant => ChatMessage::Assistant { content: text },
                            Role::System => ChatMessage::System { content: text },
                        }),
                        MessageContent::Image(_) => None,
                        MessageContent::ToolUse(_tool_use) => None,
                        MessageContent::ToolResult(tool_result) => Some(ChatMessage::Tool {
                            tool_call_id: tool_result.tool_use_id.to_string(),
                            content: tool_result.content.to_string(),
                        }),
                    })
            })
            .collect();
        let tools = request
            .tools
            .iter()
            .map(|tool| Tool::Function {
                function: copilot::copilot_chat::Function {
                    name: tool.name.clone(),
                    description: tool.description.clone(),
                    parameters: tool.input_schema.clone(),
                },
            })
            .collect();

        CopilotChatRequest {
            intent: true,
            n: 1,
            stream: model.uses_streaming(),
            temperature: 0.1,
            model,
            messages,
            tools,
            tool_choice: None,
        }
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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.state.read(cx).is_authenticated(cx) {
            const LABEL: &str = "Authorized.";
            h_flex()
                .justify_between()
                .child(
                    h_flex()
                        .gap_1()
                        .child(Icon::new(IconName::Check).color(Color::Success))
                        .child(Label::new(LABEL)),
                )
                .child(
                    Button::new("sign_out", "Sign Out")
                        .style(ui::ButtonStyle::Filled)
                        .on_click(|_, window, cx| {
                            window.dispatch_action(copilot::SignOut.boxed_clone(), cx);
                        }),
                )
        } else {
            let loading_icon = svg()
                .size_8()
                .path(IconName::ArrowCircle.path())
                .text_color(window.text_style().color)
                .with_animation(
                    "icon_circle_arrow",
                    Animation::new(Duration::from_secs(2)).repeat(),
                    |svg, delta| svg.with_transformation(Transformation::rotate(percentage(delta))),
                );

            const ERROR_LABEL: &str = "Copilot Chat requires an active GitHub Copilot subscription. Please ensure Copilot is configured and try again, or use a different Assistant provider.";

            match &self.copilot_status {
                Some(status) => match status {
                    Status::Starting { task: _ } => {
                        const LABEL: &str = "Starting Copilot...";
                        v_flex()
                            .gap_6()
                            .justify_center()
                            .items_center()
                            .child(Label::new(LABEL))
                            .child(loading_icon)
                    }
                    Status::SigningIn { prompt: _ }
                    | Status::SignedOut {
                        awaiting_signing_in: true,
                    } => {
                        const LABEL: &str = "Signing in to Copilot...";
                        v_flex()
                            .gap_6()
                            .justify_center()
                            .items_center()
                            .child(Label::new(LABEL))
                            .child(loading_icon)
                    }
                    Status::Error(_) => {
                        const LABEL: &str = "Copilot had issues starting. Please try restarting it. If the issue persists, try reinstalling Copilot.";
                        v_flex()
                            .gap_6()
                            .child(Label::new(LABEL))
                            .child(svg().size_8().path(IconName::CopilotError.path()))
                    }
                    _ => {
                        const LABEL: &str = "To use Zed's assistant with GitHub Copilot, you need to be logged in to GitHub. Note that your GitHub account must have an active Copilot Chat subscription.";
                        v_flex().gap_6().child(Label::new(LABEL)).child(
                            v_flex()
                                .gap_2()
                                .child(
                                    Button::new("sign_in", "Sign In")
                                        .icon_color(Color::Muted)
                                        .icon(IconName::Github)
                                        .icon_position(IconPosition::Start)
                                        .icon_size(IconSize::Medium)
                                        .style(ui::ButtonStyle::Filled)
                                        .full_width()
                                        .on_click(|_, window, cx| {
                                            copilot::initiate_sign_in(window, cx)
                                        }),
                                )
                                .child(
                                    div().flex().w_full().items_center().child(
                                        Label::new("Sign in to start using Github Copilot Chat.")
                                            .color(Color::Muted)
                                            .size(ui::LabelSize::Small),
                                    ),
                                ),
                        )
                    }
                },
                None => v_flex().gap_6().child(Label::new(ERROR_LABEL)),
            }
        }
    }
}
