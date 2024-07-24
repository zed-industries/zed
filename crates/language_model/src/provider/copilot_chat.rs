use std::sync::Arc;

use chrono::{NaiveDateTime, TimeDelta, Utc};
use copilot::copilot_chat::{
    self, fetch_copilot_oauth_token, ChatMessage, Model as CopilotChatModel,
    Request as CopilotChatRequest, Role as CopilotChatRole, COPILOT_CHAT_AUTH_URL,
};
use copilot::{Copilot, Status};
use futures::{FutureExt, StreamExt};
use gpui::{AppContext, AsyncAppContext, Model, Render, Subscription, Task, WeakModel};
use http_client::HttpClient;
use settings::{Settings, SettingsStore};
use std::time::Duration;
use strum::IntoEnumIterator;
use ui::{
    div, v_flex, Button, ButtonCommon, Clickable, Color, Context, FixedWidth, IconName,
    IconPosition, IconSize, IntoElement, Label, LabelCommon, ParentElement, Styled, ViewContext,
    VisualContext,
};

use crate::{settings::AllLanguageModelSettings, LanguageModelProviderState};
use crate::{
    LanguageModel, LanguageModelId, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderName, LanguageModelRequest, Role,
};

use super::open_ai::count_open_ai_tokens;

const PROVIDER_NAME: &str = "copilot_chat";

#[derive(Default, Clone, Debug, PartialEq)]
pub struct CopilotChatSettings {
    pub low_speed_timeout: Option<Duration>,
}

pub struct CopilotChatLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Model<State>,
}

pub struct State {
    oauth_token: Option<String>,
    api_key: Option<(String, NaiveDateTime)>,
    settings: CopilotChatSettings,
    _settings_subscription: Subscription,
    _oauth_token_subscription: Subscription,
}

impl CopilotChatLanguageModelProvider {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut AppContext) -> Self {
        let state = cx.new_model(|cx| {
            let _oauth_token_subscription = cx.subscribe(
                &Copilot::global(cx).unwrap(),
                |_, _, event, cx| match event {
                    // Ensures that the API key is reset when the user signs out, as well as deleted from the keychain.
                    copilot::Event::CopilotAuthSignedOut => {
                        let reset = cx.delete_credentials(COPILOT_CHAT_AUTH_URL);
                        cx.spawn(|this: WeakModel<State>, mut cx| async move {
                            reset.await?;

                            this.update(&mut cx, |this, cx| {
                                this.oauth_token = None;
                                this.api_key = None;
                                cx.notify();
                            })
                        })
                        .detach_and_log_err(cx);
                    }
                    _ => {}
                },
            );

            State {
                oauth_token: None,
                api_key: None,
                settings: CopilotChatSettings::default(),
                _settings_subscription: cx.observe_global::<SettingsStore>(
                    |this: &mut State, cx| {
                        this.settings = AllLanguageModelSettings::get_global(cx)
                            .copilot_chat
                            .clone();
                        cx.notify();
                    },
                ),
                _oauth_token_subscription,
            }
        });

        Self { http_client, state }
    }

    async fn get_new_api_token(
        cx: &mut AsyncAppContext,
        oauth_token: String,
        http_client: Arc<dyn HttpClient>,
        low_speed_timeout: Option<Duration>,
        state: &Model<State>,
    ) -> Result<String, anyhow::Error> {
        let (api_key, expires_at) =
            copilot_chat::request_api_token(&oauth_token, http_client.clone(), low_speed_timeout)
                .await?;

        cx.update_model(state, |state, cx| {
            state.api_key = Some((api_key.clone(), expires_at));
            cx.notify();
        })?;

        Ok(api_key)
    }
}

impl LanguageModelProviderState for CopilotChatLanguageModelProvider {
    fn subscribe<T: 'static>(&self, cx: &mut gpui::ModelContext<T>) -> Option<gpui::Subscription> {
        Some(cx.observe(&self.state, |_, _, cx| {
            cx.notify();
        }))
    }
}

impl LanguageModelProvider for CopilotChatLanguageModelProvider {
    fn name(&self) -> crate::LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn provided_models(&self, _cx: &AppContext) -> Vec<Arc<dyn crate::LanguageModel>> {
        CopilotChatModel::iter()
            .map(|model| {
                Arc::new(CopilotChatLanguageModel {
                    id: LanguageModelId::from(model.id().to_string()),
                    model,
                    state: self.state.clone(),
                    http_client: self.http_client.clone(),
                }) as Arc<dyn LanguageModel>
            })
            .collect()
    }

    fn is_authenticated(&self, cx: &AppContext) -> bool {
        self.state.read(cx).oauth_token.is_some()
    }

    fn authenticate(&self, cx: &AppContext) -> gpui::Task<gpui::Result<()>> {
        if self.is_authenticated(cx) {
            return Task::ready(Ok(()));
        } else {
            let Some(copilot) = Copilot::global(cx) else {
                return Task::ready(Err(anyhow::anyhow!(
                    "Copilot is not available. Please start Copilot and try again."
                )));
            };

            let state = self.state.clone();

            match copilot.read(cx).status() {
                Status::Authorized => cx.spawn(|mut cx| async move {
                    let oauth_token = match cx.update(|cx| cx.read_credentials(&COPILOT_CHAT_AUTH_URL.to_string()))?.await? {
                        Some((_, creds)) => {
                            String::from_utf8(creds)?
                        },
                        None => {
                            let oauth_token = fetch_copilot_oauth_token().await?;

                            cx.update(|cx| cx.write_credentials(COPILOT_CHAT_AUTH_URL, "Bearer", oauth_token.as_bytes()))?.await?;
                            oauth_token
                        }
                    };

                    state.update(&mut cx, |this, cx| {
                        this.oauth_token = Some(oauth_token);
                        cx.notify();
                    })
                }),

                _ => Task::ready(Err(anyhow::anyhow!("You are not authorized with Github Copilot. Please authorize first, then try again")))
            }
        }
    }

    fn authentication_prompt(&self, cx: &mut ui::WindowContext) -> gpui::AnyView {
        cx.new_view(|cx| AuthenticationPrompt::new(cx)).into()
    }

    fn reset_credentials(&self, cx: &AppContext) -> gpui::Task<gpui::Result<()>> {
        if Copilot::global(cx).is_none() {
            return Task::ready(Err(anyhow::anyhow!(
                "Copilot is not available. Please ensure Copilot is enabled and running and try again."
            )));
        }

        let state = self.state.clone();
        let copilot = Copilot::global(cx).clone();
        let reset = cx.delete_credentials(COPILOT_CHAT_AUTH_URL);

        cx.spawn(|mut cx| async move {
            reset.await?;

            cx.update_model(&copilot.unwrap(), |model, cx| model.sign_out(cx))?
                .await?;

            cx.update_model(&state, |this, cx| {
                this.oauth_token = None;
                this.api_key = None;
                cx.notify();
            })?;

            Ok(())
        })
    }
}

pub struct CopilotChatLanguageModel {
    id: LanguageModelId,
    model: CopilotChatModel,
    state: Model<State>,
    http_client: Arc<dyn HttpClient>,
}

impl LanguageModel for CopilotChatLanguageModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> crate::LanguageModelName {
        LanguageModelName::from(self.model.display_name().to_string())
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
        request: crate::LanguageModelRequest,
        cx: &AppContext,
    ) -> futures::future::BoxFuture<'static, gpui::Result<usize>> {
        let model = match self.model {
            CopilotChatModel::Gpt4 => open_ai::Model::Four,
            CopilotChatModel::Gpt3_5Turbo => open_ai::Model::ThreePointFiveTurbo,
        };

        count_open_ai_tokens(request, model, cx)
    }

    fn stream_completion(
        &self,
        request: crate::LanguageModelRequest,
        cx: &AsyncAppContext,
    ) -> futures::future::BoxFuture<
        'static,
        gpui::Result<futures::stream::BoxStream<'static, gpui::Result<String>>>,
    > {
        let state = self.state.clone();
        let http_client = self.http_client.clone();
        let request = self.to_copilot_chat_request(request);
        let Ok((oauth_token, api_key, low_speed_timeout)) =
            cx.read_model(&self.state, |state, _| {
                (
                    state.oauth_token.clone().unwrap(),
                    state.api_key.clone(),
                    state.settings.low_speed_timeout,
                )
            })
        else {
            return futures::future::ready(Err(anyhow::anyhow!("App state dropped"))).boxed();
        };

        cx.spawn(|mut cx| async move {

            let api_key = match api_key {
                Some((key, expires_at)) => {
                    if expires_at - Utc::now().naive_utc() < TimeDelta::minutes(5) {
                        CopilotChatLanguageModelProvider::get_new_api_token(&mut cx, oauth_token, http_client.clone(), low_speed_timeout, &state ).await?
                    } else {
                        key
                    }
                },
                None => CopilotChatLanguageModelProvider::get_new_api_token(&mut cx, oauth_token, http_client.clone(), low_speed_timeout, &state).await?
            };
            let response = copilot_chat::stream_completion(
                http_client,
                api_key,
                request,
                low_speed_timeout,
            )
            .await?;
            let stream = response
                .filter_map(|response| async move {
                    match response {
                        Ok(result) => {
                            let choice = result.choices.first();
                            match choice {
                                Some(choice) => Some(Ok(choice.delta.content.clone().unwrap())),
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
        })
        .boxed()
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
                    content: msg.content,
                })
                .collect(),
        )
    }
}

struct AuthenticationPrompt {
    copilot: Option<Model<Copilot>>,
}

impl AuthenticationPrompt {
    pub fn new(cx: &mut AppContext) -> Self {
        Self {
            copilot: Copilot::global(cx),
        }
    }

    pub fn copilot_disabled(&self, cx: &mut AppContext) -> bool {
        self.copilot.is_none()
            || self
                .copilot
                .clone()
                .unwrap()
                .read(cx)
                .status()
                .is_disabled()
    }
}

impl Render for AuthenticationPrompt {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        const LABEL: &str =
            "To use the assistant panel or inline assistant, you must login to GitHub Copilot. Your GitHub account must have an active Copilot Chat subscription.";

        const ERROR_LABEL: &str = "Copilot Chat requires the Copilot plugin to be available and running. Please ensure Copilot is running and try again, or use a different Assistant provider.";

        if self.copilot_disabled(cx) {
            return v_flex().gap_6().p_4().child(Label::new(ERROR_LABEL));
        }

        v_flex().gap_6().p_4().child(Label::new(LABEL)).child(
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
                        // I don't love that using the inline_completion_button module here, but it's the best way to share the logic between the two buttons,
                        // without a bunch of refactoring.
                        .on_click(|_, cx| inline_completion_button::initiate_sign_in(cx)),
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
}
