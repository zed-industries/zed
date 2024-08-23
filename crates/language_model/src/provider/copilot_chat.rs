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
    percentage, svg, Animation, AnimationExt, AnyView, AppContext, AsyncAppContext, Model, Render,
    Subscription, Task, Transformation,
};
use settings::{Settings, SettingsStore};
use std::time::Duration;
use strum::IntoEnumIterator;
use ui::{
    div, h_flex, v_flex, Button, ButtonCommon, Clickable, Color, Context, FixedWidth, Icon,
    IconName, IconPosition, IconSize, IntoElement, Label, LabelCommon, ParentElement, Styled,
    ViewContext, VisualContext, WindowContext,
};

use crate::settings::AllLanguageModelSettings;
use crate::LanguageModelProviderState;
use crate::{
    LanguageModel, LanguageModelId, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelRequest, RateLimiter, Role,
};

use super::open_ai::count_open_ai_tokens;

const PROVIDER_ID: &str = "copilot_chat";
const PROVIDER_NAME: &str = "GitHub Copilot Chat";

#[derive(Default, Clone, Debug, PartialEq)]
pub struct CopilotChatSettings {
    pub low_speed_timeout: Option<Duration>,
}

pub struct CopilotChatLanguageModelProvider {
    state: Model<State>,
}

pub struct State {
    _copilot_chat_subscription: Option<Subscription>,
    _settings_subscription: Subscription,
}

impl State {
    fn is_authenticated(&self, cx: &AppContext) -> bool {
        CopilotChat::global(cx)
            .map(|m| m.read(cx).is_authenticated())
            .unwrap_or(false)
    }
}

impl CopilotChatLanguageModelProvider {
    pub fn new(cx: &mut AppContext) -> Self {
        let state = cx.new_model(|cx| {
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

    fn observable_entity(&self) -> Option<gpui::Model<Self::ObservableEntity>> {
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

    fn provided_models(&self, _cx: &AppContext) -> Vec<Arc<dyn LanguageModel>> {
        CopilotChatModel::iter()
            .map(|model| {
                Arc::new(CopilotChatLanguageModel {
                    model,
                    request_limiter: RateLimiter::new(4),
                }) as Arc<dyn LanguageModel>
            })
            .collect()
    }

    fn is_authenticated(&self, cx: &AppContext) -> bool {
        self.state.read(cx).is_authenticated(cx)
    }

    fn authenticate(&self, cx: &mut AppContext) -> Task<Result<()>> {
        let result = if self.is_authenticated(cx) {
            Ok(())
        } else if let Some(copilot) = Copilot::global(cx) {
            let error_msg = match copilot.read(cx).status() {
                Status::Disabled => anyhow::anyhow!("Copilot must be enabled for Copilot Chat to work. Please enable Copilot and try again."),
                Status::Error(e) => anyhow::anyhow!(format!("Received the following error while signing into Copilot: {e}")),
                Status::Starting { task: _ } => anyhow::anyhow!("Copilot is still starting, please wait for Copilot to start then try again"),
                Status::Unauthorized => anyhow::anyhow!("Unable to authorize with Copilot. Please make sure that you have an active Copilot and Copilot Chat subscription."),
                Status::Authorized => return Task::ready(Ok(())),
                Status::SignedOut => anyhow::anyhow!("You have signed out of Copilot. Please sign in to Copilot and try again."),
                Status::SigningIn { prompt: _ } => anyhow::anyhow!("Still signing into Copilot..."),
            };
            Err(error_msg)
        } else {
            Err(anyhow::anyhow!(
                "Copilot must be enabled for Copilot Chat to work. Please enable Copilot and try again."
            ))
        };
        Task::ready(result)
    }

    fn configuration_view(&self, cx: &mut WindowContext) -> AnyView {
        let state = self.state.clone();
        cx.new_view(|cx| ConfigurationView::new(state, cx)).into()
    }

    fn reset_credentials(&self, _cx: &mut AppContext) -> Task<Result<()>> {
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
        cx: &AppContext,
    ) -> BoxFuture<'static, Result<usize>> {
        let model = match self.model {
            CopilotChatModel::Gpt4o => open_ai::Model::FourOmni,
            CopilotChatModel::Gpt4 => open_ai::Model::Four,
            CopilotChatModel::Gpt3_5Turbo => open_ai::Model::ThreePointFiveTurbo,
        };

        count_open_ai_tokens(request, model, cx)
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncAppContext,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
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

        let request = self.to_copilot_chat_request(request);
        let Ok(low_speed_timeout) = cx.update(|cx| {
            AllLanguageModelSettings::get_global(cx)
                .copilot_chat
                .low_speed_timeout
        }) else {
            return futures::future::ready(Err(anyhow::anyhow!("App state dropped"))).boxed();
        };

        let request_limiter = self.request_limiter.clone();
        let future = cx.spawn(|cx| async move {
            let response = CopilotChat::stream_completion(request, low_speed_timeout, cx);
            request_limiter.stream(async move {
                let response = response.await?;
                let stream = response
                    .filter_map(|response| async move {
                        match response {
                            Ok(result) => {
                                let choice = result.choices.first();
                                match choice {
                                    Some(choice) => Some(Ok(choice.delta.content.clone().unwrap_or_default())),
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

        async move { Ok(future.await?.boxed()) }.boxed()
    }

    fn use_any_tool(
        &self,
        _request: LanguageModelRequest,
        _name: String,
        _description: String,
        _schema: serde_json::Value,
        _cx: &AsyncAppContext,
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
    state: Model<State>,
    _subscription: Option<Subscription>,
}

impl ConfigurationView {
    pub fn new(state: Model<State>, cx: &mut ViewContext<Self>) -> Self {
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
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
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
                .text_color(cx.text_style().color)
                .with_animation(
                    "icon_circle_arrow",
                    Animation::new(Duration::from_secs(2)).repeat(),
                    |svg, delta| svg.with_transformation(Transformation::rotate(percentage(delta))),
                );

            const ERROR_LABEL: &str = "Copilot Chat requires the Copilot plugin to be available and running. Please ensure Copilot is running and try again, or use a different Assistant provider.";

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
                    "To use the assistant panel or inline assistant, you must login to GitHub Copilot. Your GitHub account must have an active Copilot Chat subscription.";
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
                                        .on_click(|_, cx| {
                                            inline_completion_button::initiate_sign_in(cx)
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
