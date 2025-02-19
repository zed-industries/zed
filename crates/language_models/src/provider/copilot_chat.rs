use std::future;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use copilot::copilot_chat::{
    ChatMessage, CopilotChat, Model as CopilotChatModel, Request as CopilotChatRequest,
    Role as CopilotChatRole,
};
use copilot::{Copilot, Status};
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use futures::{FutureExt, StreamExt};
use gpui::{
    percentage, svg, Animation, AnimationExt, AnyView, App, AsyncApp, Entity, Render, Subscription,
    Task, Transformation,
};
use language_model::{
    AuthenticateError, LanguageModel, LanguageModelCompletionEvent, LanguageModelId,
    LanguageModelName, LanguageModelProvider, LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelProviderState, LanguageModelRequest, RateLimiter, Role,
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
            Status::Disabled => anyhow!("Copilot must be enabled for Copilot Chat to work. Please enable Copilot and try again."),
            Status::Error(err) => anyhow!(format!("Received the following error while signing into Copilot: {err}")),
            Status::Starting { task: _ } => anyhow!("Copilot is still starting, please wait for Copilot to start then try again"),
            Status::Unauthorized => anyhow!("Unable to authorize with Copilot. Please make sure that you have an active Copilot and Copilot Chat subscription."),
            Status::SignedOut => anyhow!("You have signed out of Copilot. Please sign in to Copilot and try again."),
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
            CopilotChatModel::Gemini20Flash => count_google_tokens(request, cx),
            _ => {
                let model = match self.model {
                    CopilotChatModel::Gpt4o => open_ai::Model::FourOmni,
                    CopilotChatModel::Gpt4 => open_ai::Model::Four,
                    CopilotChatModel::Gpt3_5Turbo => open_ai::Model::ThreePointFiveTurbo,
                    CopilotChatModel::O1 | CopilotChatModel::O3Mini => open_ai::Model::Four,
                    CopilotChatModel::Claude3_5Sonnet | CopilotChatModel::Gemini20Flash => {
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
        let future = cx.spawn(|cx| async move {
            let response = CopilotChat::stream_completion(copilot_request, cx);
            request_limiter.stream(async move {
                let response = response.await?;
                let stream = response
                    .filter_map(move |response| async move {
                        match response {
                            Ok(result) => {
                                let choice = result.choices.first();
                                match choice {
                                    Some(choice) if !is_streaming => {
                                        match &choice.message {
                                            Some(msg) => Some(Ok(msg.content.clone().unwrap_or_default())),
                                            None => Some(Err(anyhow::anyhow!(
                                                "The Copilot Chat API returned a response with no message content"
                                            ))),
                                        }
                                    },
                                    Some(choice) => {
                                        match &choice.delta {
                                            Some(delta) => Some(Ok(delta.content.clone().unwrap_or_default())),
                                            None => Some(Err(anyhow::anyhow!(
                                                "The Copilot Chat API returned a response with no delta content"
                                            ))),
                                        }
                                    },
                                    None => Some(Err(anyhow::anyhow!(
                                        "The Copilot Chat API returned a response with no choices, but hadn't finished the message yet. Please try again."
                                    ))),
                                }
                            }
                            Err(err) => Some(Err(err)),
                        }
                    })
                    .boxed();
                Ok(stream)
            }).await
        });

        async move {
            Ok(future
                .await?
                .map(|result| result.map(LanguageModelCompletionEvent::Text))
                .boxed())
        }
        .boxed()
    }

    fn use_any_tool(
        &self,
        _request: LanguageModelRequest,
        _name: String,
        _description: String,
        _schema: serde_json::Value,
        _cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        future::ready(Err(anyhow!("not implemented"))).boxed()
    }
}

impl CopilotChatLanguageModel {
    pub fn to_copilot_chat_request(&self, request: LanguageModelRequest) -> CopilotChatRequest {
        CopilotChatRequest::new(
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
        )
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
                .gap_1()
                .child(Icon::new(IconName::Check).color(Color::Success))
                .child(Label::new(LABEL))
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
                    Status::Disabled => v_flex().gap_6().p_4().child(Label::new(ERROR_LABEL)),
                    Status::Starting { task: _ } => {
                        const LABEL: &str = "Starting Copilot...";
                        v_flex()
                            .gap_6()
                            .justify_center()
                            .items_center()
                            .child(Label::new(LABEL))
                            .child(loading_icon)
                    }
                    Status::SigningIn { prompt: _ } => {
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
                        const LABEL: &str =
                    "To use Zed's assistant with GitHub Copilot, you need to be logged in to GitHub. Note that your GitHub account must have an active Copilot Chat subscription.";
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
