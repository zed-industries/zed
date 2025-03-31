use std::pin::Pin;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use copilot::copilot_chat::{
    ChatMessage, CopilotChat, Model as CopilotChatModel, Request as CopilotChatRequest,
    Role as CopilotChatRole,
};
use copilot::{Copilot, Status};
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use futures::{FutureExt, Stream, StreamExt};
use gpui::{
    percentage, svg, Action, Animation, AnimationExt, AnyView, App, AsyncApp, Entity, Render,
    Subscription, Task, Transformation,
};
use language_model::{
    AuthenticateError, LanguageModel, LanguageModelCompletionEvent, LanguageModelId,
    LanguageModelName, LanguageModelProvider, LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelProviderState, LanguageModelRequest, LanguageModelToolUse, RateLimiter, Role,
};
use settings::SettingsStore;
use std::str::FromStr;
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
        log::error!("Streaming completion for model: {:?}", self.model);
        if let Some(message) = request.messages.last() {
            if message.contents_empty() {
                const EMPTY_PROMPT_MSG: &str =
                    "Empty prompts aren't allowed. Please provide a non-empty prompt.";
                return futures::future::ready(Err(anyhow::anyhow!(EMPTY_PROMPT_MSG))).boxed();
            }

            if !matches!(message.role, Role::User) {
                const USER_ROLE_MSG: &str = "The final message must be from the user. To provide a system prompt, you must provide the system prompt followed by a user prompt.";
                return futures::future::ready(Err(anyhow::anyhow!(USER_ROLE_MSG))).boxed();
            }
        }

        let copilot_request = self.to_copilot_chat_request(request);
        let request_limiter = self.request_limiter.clone();
        let future = cx.spawn(async move |cx| {
            let response = CopilotChat::stream_completion(copilot_request, cx.clone());
            request_limiter.stream(async move {
            let response = response.await?;

            struct State {
                stream: Pin<Box<dyn Send + Stream<Item = Result<copilot::copilot_chat::ResponseEvent, anyhow::Error>>>>,
                current_tool_name: Option<String>,
                current_tool_id: Option<String>,
                current_tool_input_json: String,
                pending_event: Option<LanguageModelCompletionEvent>,
            }

            let stream = futures::stream::unfold(
                State {
                stream: response,
                current_tool_id: None,
                current_tool_name: None,
                current_tool_input_json: String::new(),
                pending_event: None,
                },
                |mut state| async move {
                // If there is a pending event, yield it before processing new items.
                if let Some(event) = state.pending_event.take() {
                    log::error!("Yielding pending event: {:?}", event);
                    return Some((Ok(event), state));
                }
                while let Some(response) = state.stream.next().await {
                    log::error!("Got response part: {:?}", response);
                    match response {
                    Ok(result) => {
                        // log::error!("Processing result: {:?}", result);
                        let choice = match result.choices.first() {
                        Some(choice) => choice,
                        None => continue,
                        };
                        let delta = &choice.delta;

                        if let Some(finish_reason) = &choice.finish_reason {
                        log::error!("Got finish reason: {}", finish_reason);
                        match finish_reason.as_str() {
                            "stop" => {
                            return Some((
                                Ok(LanguageModelCompletionEvent::Stop(language_model::StopReason::EndTurn)),
                                state,
                            ));
                            }
                            "tool_calls" => {
                            if let (Some(tool_name), Some(tool_id), args) = (
                                state.current_tool_name.take(),
                                state.current_tool_id.take(),
                                state.current_tool_input_json.clone(),
                            ) {
                                if !args.is_empty() {
                                if let Ok(input) = serde_json::Value::from_str(&args) {
                                    let tool_use = LanguageModelCompletionEvent::ToolUse(
                                    LanguageModelToolUse {
                                        id: tool_id.into(),
                                        name: tool_name.into(),
                                        input,
                                    }
                                    );
                                    log::error!("Tool use: {:?}", tool_use);
                                    // Schedule a Stop event for the next iteration.
                                    state.pending_event = Some(LanguageModelCompletionEvent::Stop(language_model::StopReason::ToolUse));
                                    return Some((Ok(tool_use), state));
                                }
                                }
                            }
                            }
                            _ => {}
                        }
                        }

                        if let Some(tool_calls) = &delta.tool_calls {
                        log::error!("Processing tool calls: {:?}", tool_calls);
                        for tool_call in tool_calls {
                            if let Some(id) = &tool_call.id {
                            state.current_tool_id = Some(id.clone());
                            }
                            if let Some(function) = &tool_call.function {
                            if let Some(name) = &function.name {
                                state.current_tool_name = Some(name.clone());
                            }
                            if let Some(args) = &function.arguments {
                                state.current_tool_input_json.push_str(args);
                            }
                            }
                        }
                        continue;
                        }

                        if let Some(content) = &delta.content {
                        log::error!("Got content: {}", content);
                        return Some((Ok(LanguageModelCompletionEvent::Text(content.clone())), state));
                        }
                    }
                    Err(err) => {
                        log::error!("Got error: {:?}", err);
                        return Some((Err(err), state));
                    }
                    }
                }
                None
                },
            );

            Ok(stream
                .inspect(|event_result| match event_result {
                    Ok(event) => log::error!("LanguageModelCompletionEvent: {:?}", event),
                    Err(err) => log::error!("LanguageModelCompletionEvent error: {:?}", err),
                })
                .boxed())
            })
            .await
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }

    fn use_any_tool(
        &self,
        request: LanguageModelRequest,
        tool_name: String,
        tool_description: String,
        input_schema: serde_json::Value,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<futures::stream::BoxStream<'static, Result<String>>>> {
        log::error!(
            "Using tool '{}' with description: {}",
            tool_name,
            tool_description
        );

        // For tool use, we need to use a Claude model since they support tool use
        let mut copilot_request = self.to_copilot_chat_request(request);
        copilot_request.model = match self.model {
            CopilotChatModel::Claude3_5Sonnet => CopilotChatModel::Claude3_5Sonnet,
            CopilotChatModel::Claude3_7Sonnet => CopilotChatModel::Claude3_7Sonnet,
            CopilotChatModel::Claude3_7SonnetThinking => CopilotChatModel::Claude3_7SonnetThinking,
            _ => CopilotChatModel::Claude3_5Sonnet, // Default to Claude 3.5 Sonnet for other models
        };

        copilot_request.tools = vec![copilot::copilot_chat::ToolWrapper {
            function: copilot::copilot_chat::Tool {
                name: tool_name.clone(),
                description: tool_description,
                parameters: input_schema,
            },
            tool_type: "function".to_string(),
        }];

        // let tools = request.tools.clone();
        // // Add tools from the original request with proper function wrapper
        // copilot_request.tools = tools
        //     .into_iter()
        //     .map(|tool| {
        //         let name = tool.name.clone();
        //         let description = tool.description.clone();
        //         let parameters = tool.input_schema;
        //         copilot::copilot_chat::ToolWrapper {
        //             function: copilot::copilot_chat::Tool {
        //                 name,
        //                 description,
        //                 parameters: serde_json::json!({
        //                     "type": "object",
        //                     "properties": parameters.get("properties").unwrap_or(&serde_json::json!({})),
        //                     "required": parameters.get("required").unwrap_or(&serde_json::json!([])),
        //                 }),
        //             },
        //             tool_type: "function".to_string(),
        //         }
        //     })
        //     .collect();
        // copilot_request.tool_choice = Some(copilot::copilot_chat::ToolChoice::Tool {
        //     name: tool_name.clone(),
        // });

        let request_limiter = self.request_limiter.clone();
        log::error!("BEFORE CALLING CopilotChat::stream_completion");
        let response_stream = CopilotChat::stream_completion(copilot_request, cx.clone());
        log::error!("AFTER CALLING CopilotChat::stream_completion");

        let future = cx.spawn(async move |_cx| {
            log::error!("Entering cx.spawn for tool use processing");
            request_limiter
                .run(async move {
                    log::error!("Waiting for response stream from CopilotChat");
                    let stream = response_stream.await?;
                    log::error!("Response stream received successfully");

                    let tool_args_stream = stream
                        .filter_map(move |response| async move {
                            log::error!("Processing a new response");
                            match response {
                                Ok(response) => {
                                    for choice in response.choices {
                                        log::error!("Processing choice in response");
                                        if let Some(tool_calls) = choice.delta.tool_calls {
                                            log::error!("Tool calls found in choice");
                                            for tool_call in tool_calls {
                                                if let Some(function) = tool_call.function {
                                                    log::error!("Function found in tool call");
                                                    if let Some(args) = function.arguments {
                                                        log::error!("Function arguments extracted");
                                                        return Some(Ok(args));
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    log::error!("No tool arguments found in this response");
                                    None
                                }
                                Err(e) => {
                                    log::error!("Error processing response: {:?}", e);
                                    Some(Err(e))
                                }
                            }
                        })
                        .boxed();

                    log::error!("Returning tool arguments stream to caller");
                    Ok(tool_args_stream)
                })
                .await
        });

        async move { future.await }.boxed()
    }
}

pub fn map_to_language_model_completion_events(
    events: Pin<
        Box<dyn Send + Stream<Item = Result<copilot::copilot_chat::ResponseEvent, anyhow::Error>>>,
    >,
) -> impl Stream<Item = Result<LanguageModelCompletionEvent>> {
    events
        .filter_map(move |response| async move {
            match response {
                Ok(result) => {
                    let choice = result.choices.first();
                    match choice {
                        Some(choice) => {
                            if let Some(msg) = &choice.message {
                                let content = msg.content.clone().unwrap_or_default();
                                Some(Ok(LanguageModelCompletionEvent::StartMessage {
                                    message_id: result.id,
                                }))
                            // }
                            // else if let Some( msg) = &choice.delta.tool_calls {
                            // Some(Ok(LanguageModelCompletionEvent::ToolUse(content)))
                            } else if let Some(content) = &choice.delta.content {
                                Some(Ok(LanguageModelCompletionEvent::Text(content.clone())))
                            // } else if let Some(content) = &choice.finish_reason {
                            // Some(Ok(LanguageModelCompletionEvent::Stop(content)))
                            } else {
                                None
                            }
                        }
                        None => Some(Err(anyhow::anyhow!(
                            "The Copilot Chat API returned a response with no choices"
                        ))),
                    }
                }
                Err(err) => Some(Err(anyhow::anyhow!(err))),
            }
        })
        .boxed()
}

fn parse_content_to_completion_event(content: String) -> LanguageModelCompletionEvent {
    // Try to parse the content as a JSON object that might be a tool use
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
        if json.is_object() {
            if let Ok(tool_use) = serde_json::from_value::<LanguageModelToolUse>(json.clone()) {
                return LanguageModelCompletionEvent::ToolUse(tool_use);
            }
        }
    }
    // If not a valid tool use JSON, treat as regular text
    LanguageModelCompletionEvent::Text(content)
}

impl CopilotChatLanguageModel {
    pub fn to_copilot_chat_request(&self, request: LanguageModelRequest) -> CopilotChatRequest {
        let mut copilot_request = CopilotChatRequest::new(
            self.model.clone(),
            request
                .messages
                .into_iter()
                .map(|msg| ChatMessage {
                    role: match msg.role {
                        Role::User => CopilotChatRole::User,
                        Role::Assistant => CopilotChatRole::Assistant,
                        Role::System => CopilotChatRole::System,
                    },
                    content: msg.string_contents(),
                })
                .collect(),
        );

        // Add tools from the original request with proper function wrapper
        copilot_request.tools = request
            .tools
            .into_iter()
            .map(|tool| {
                let name = tool.name.clone();
                let description = tool.description.clone();
                let parameters = tool.input_schema;
                copilot::copilot_chat::ToolWrapper {
                    function: copilot::copilot_chat::Tool {
                        name,
                        description,
                        parameters: serde_json::json!({
                            "type": "object",
                            "properties": parameters.get("properties").unwrap_or(&serde_json::json!({})),
                            "required": parameters.get("required").unwrap_or(&serde_json::json!([])),
                        }),
                    },
                    tool_type: "function".to_string(),
                }
            })
            .collect();

        copilot_request
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
