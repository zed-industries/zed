use crate::{
    count_open_ai_tokens, CompletionProvider, LanguageModel, LanguageModelCompletionProvider,
    LanguageModelRequest,
};
use anyhow::Result;
use chrono::{NaiveDateTime, TimeDelta, Utc};
use copilot::{
    copilot_chat::{
        self, ChatMessage, Model as CopilotChatModel, Request as CopilotChatRequest,
        Role as CopilotChatRole, COPILOT_CHAT_AUTH_URL,
    },
    Copilot, Status,
};
use futures::{FutureExt, StreamExt};
use gpui::{AppContext, AsyncAppContext, IntoElement, Render, Subscription, Task};
use http::HttpClient;
use language_model::Role;
use std::{sync::Arc, time::Duration};
use strum::IntoEnumIterator;
use ui::{
    div, v_flex, BorrowAppContext, Button, ButtonCommon, Clickable, Color, FixedWidth, IconName,
    IconPosition, IconSize, Label, LabelCommon, ParentElement, Styled, ViewContext, VisualContext,
    WindowContext,
};

pub struct CopilotChatCompletionProvider {
    oauth_token: Option<String>,
    api_key: Option<String>,
    api_key_expiry: Option<NaiveDateTime>,
    completion_api_url: String,
    auth_api_url: String,
    model: CopilotChatModel,
    http_client: Arc<dyn HttpClient>,
    low_speed_timeout: Option<Duration>,
    settings_version: usize,
}

impl LanguageModelCompletionProvider for CopilotChatCompletionProvider {
    fn available_models(&self) -> Vec<crate::LanguageModel> {
        CopilotChatModel::iter()
            .map(LanguageModel::CopilotChat)
            .collect()
    }

    fn settings_version(&self) -> usize {
        self.settings_version
    }

    fn is_authenticated(&self) -> bool {
        self.oauth_token.is_some()
    }

    fn authenticate(&self, cx: &gpui::AppContext) -> Task<Result<()>> {
        if self.is_authenticated() {
            return Task::ready(Ok(()));
        } else {
            let Some(copilot) = Copilot::global(cx) else {
                return Task::ready(Err(anyhow::anyhow!(
                    "Copilot is not available. Please start Copilot and try again."
                )));
            };

            match copilot.read(cx).status() {
                Status::Authorized => cx.spawn(|mut cx| async move {
                    let oauth_token = copilot_chat::fetch_copilot_oauth_token().await?;

                    cx.update_global::<CompletionProvider, _>(|provider, _cx| {
                        provider.update_current_as::<_, CopilotChatCompletionProvider>(
                            |provider| {
                                provider.oauth_token = Some(oauth_token);
                            },
                        );
                    })

                }),
                _ => Task::ready(Err(anyhow::anyhow!("You are not authorized with Github Copilot. Please authorize first, then try again")))
            }
        }
    }

    fn authentication_prompt(&self, cx: &mut WindowContext) -> gpui::AnyView {
        cx.new_view(|cx| AuthenticationPrompt::new(cx)).into()
    }

    fn reset_credentials(&self, cx: &AppContext) -> Task<Result<()>> {
        let Some(copilot) = Copilot::global(cx) else {
            return Task::ready(Err(anyhow::anyhow!(
                "Copilot is not available. Please start Copilot and try again."
            )));
        };

        cx.spawn(|mut cx| async move {
            let task = copilot.update(&mut cx, |model, cx| model.sign_out(cx))?;

            task.await?;

            cx.update_global::<CompletionProvider, _>(|provider, _cx| {
                provider.update_current_as::<_, CopilotChatCompletionProvider>(|provider| {
                    provider.oauth_token = None;
                });
            })
        })
    }

    fn model(&self) -> crate::LanguageModel {
        LanguageModel::CopilotChat(self.model.clone())
    }

    fn count_tokens(
        &self,
        request: crate::LanguageModelRequest,
        cx: &gpui::AppContext,
    ) -> futures::future::BoxFuture<'static, http::Result<usize>> {
        count_open_ai_tokens(request, cx.background_executor())
    }

    fn stream_completion(
        &self,
        request: crate::LanguageModelRequest,
        cx: &AsyncAppContext,
    ) -> futures::future::BoxFuture<
        'static,
        Result<futures::stream::BoxStream<'static, Result<String>>>,
    > {
        let http_client = self.http_client.clone();
        let request: CopilotChatRequest = self.to_copilot_chat_request(request);
        let oauth_token = self.oauth_token.clone().unwrap();
        let expires_at = self.api_key_expiry.clone();
        let api_key = self.api_key.clone();
        let auth_api_url = self.auth_api_url.clone();
        let api_url = self.completion_api_url.clone();
        let low_speed_timeout = self.low_speed_timeout.clone();

        cx.spawn(|mut cx| async move {
            let api_key = if api_key.is_none()
                || expires_at.is_none()
                || (expires_at.unwrap() - Utc::now().naive_utc()) < TimeDelta::minutes(5)
            {
                let (api_key, expires_at) = copilot_chat::request_api_token(
                    &oauth_token,
                    &auth_api_url,
                    http_client.clone(),
                    low_speed_timeout,
                )
                .await?;

                cx.update_global::<CompletionProvider, _>(|provider, _cx| {
                    provider.update_current_as::<_, CopilotChatCompletionProvider>(|provider| {
                        provider.api_key = Some(api_key.clone());
                        provider.api_key_expiry = Some(expires_at);
                    });
                })?;

                api_key
            } else {
                api_key.unwrap()
            };
            let response = copilot_chat::stream_completion(
                http_client,
                api_url,
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

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl CopilotChatCompletionProvider {
    pub fn new(
        model: CopilotChatModel,
        api_url: String,
        http_client: Arc<dyn HttpClient>,
        low_speed_timeout: Option<Duration>,
        settings_version: usize,
    ) -> Self {
        Self {
            model,
            auth_api_url: COPILOT_CHAT_AUTH_URL.to_string(),
            api_key: None,
            oauth_token: None,
            http_client,
            low_speed_timeout,
            settings_version,
            api_key_expiry: None,
            completion_api_url: api_url,
        }
    }

    pub fn update(
        &mut self,
        model: CopilotChatModel,
        api_url: String,
        low_speed_timeout: Option<Duration>,
        settings_version: usize,
    ) {
        self.model = model;
        self.completion_api_url = api_url;
        self.low_speed_timeout = low_speed_timeout;
        self.settings_version = settings_version;
    }

    pub fn to_copilot_chat_request(&self, request: LanguageModelRequest) -> CopilotChatRequest {
        let model = match request.model {
            LanguageModel::CopilotChat(model) => model,
            _ => CopilotChatModel::default(),
        };

        CopilotChatRequest::new(
            model,
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
    _subscription: Option<Subscription>,
}

impl AuthenticationPrompt {
    pub fn new(cx: &mut AppContext) -> Self {
        fn update_auth_information(cx: &mut AppContext) {
            cx.spawn(|mut cx| async move {
                let oauth_token = copilot_chat::fetch_copilot_oauth_token().await?;
                cx.update_global::<CompletionProvider, _>(|provider, _cx| {
                    provider.update_current_as::<_, CopilotChatCompletionProvider>(|provider| {
                        provider.oauth_token = Some(oauth_token.clone());
                    });
                })
            })
            .detach_and_log_err(cx);
        }

        let sub = match Copilot::global(cx) {
            Some(copilot) => {
                // Fetch the token if the user is already signed in.
                match copilot.read(cx).status() {
                    Status::Authorized => update_auth_information(cx),
                    _ => {}
                }

                // Subscribe to copilot status changes so we get the token when the user signs in.
                Some(
                    cx.observe(&copilot, |model, cx| match model.read(cx).status() {
                        Status::Authorized => update_auth_information(cx),
                        Status::SignedOut => {
                            cx.update_global::<CompletionProvider, _>(|provider, _cx| {
                                provider.update_current_as::<_, CopilotChatCompletionProvider>(
                                    |provider| {
                                        provider.oauth_token = None;
                                        provider.api_key = None;
                                        provider.api_key_expiry = None;
                                    },
                                );
                            })
                        }
                        _ => {}
                    }),
                )
            }
            None => None,
        };

        Self { _subscription: sub }
    }
}

impl Render for AuthenticationPrompt {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        const LABEL: &str =
            "To use the assistant panel or inline assistant, you must login to Github Copilot. Your Github account must have an active Copilot Chat subscription.";

        const ERROR_LABEL: &str = "Copilot Chat requires the Copilot plugin to be available and running. Please ensure Copilot is running and try again, or use a different Assistant provider.";

        match self._subscription {
            Some(_) => v_flex().gap_6().p_4().child(Label::new(LABEL)).child(
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
            ),
            None => v_flex().p_4().child(Label::new(ERROR_LABEL)),
        }
    }
}
