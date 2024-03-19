use crate::{
    assistant_settings::ZedDotDevModel, count_open_ai_tokens, CompletionProvider,
    LanguageModelRequest,
};
use anyhow::{anyhow, Result};
use client::{proto, Client};
use futures::{future::BoxFuture, stream::BoxStream, FutureExt, StreamExt, TryFutureExt};
use gpui::{AnyView, AppContext, Task};
use std::{future, sync::Arc};
use ui::prelude::*;

pub struct ZedDotDevCompletionProvider {
    client: Arc<Client>,
    default_model: ZedDotDevModel,
    settings_version: usize,
    status: client::Status,
    _maintain_client_status: Task<()>,
}

impl ZedDotDevCompletionProvider {
    pub fn new(
        default_model: ZedDotDevModel,
        client: Arc<Client>,
        settings_version: usize,
        cx: &mut AppContext,
    ) -> Self {
        let mut status_rx = client.status();
        let status = *status_rx.borrow();
        let maintain_client_status = cx.spawn(|mut cx| async move {
            while let Some(status) = status_rx.next().await {
                let _ = cx.update_global::<CompletionProvider, _>(|provider, _cx| {
                    if let CompletionProvider::ZedDotDev(provider) = provider {
                        provider.status = status;
                    } else {
                        unreachable!()
                    }
                });
            }
        });
        Self {
            client,
            default_model,
            settings_version,
            status,
            _maintain_client_status: maintain_client_status,
        }
    }

    pub fn update(&mut self, default_model: ZedDotDevModel, settings_version: usize) {
        self.default_model = default_model;
        self.settings_version = settings_version;
    }

    pub fn settings_version(&self) -> usize {
        self.settings_version
    }

    pub fn default_model(&self) -> ZedDotDevModel {
        self.default_model.clone()
    }

    pub fn is_authenticated(&self) -> bool {
        self.status.is_connected()
    }

    pub fn authenticate(&self, cx: &AppContext) -> Task<Result<()>> {
        let client = self.client.clone();
        cx.spawn(move |cx| async move { client.authenticate_and_connect(true, &cx).await })
    }

    pub fn authentication_prompt(&self, cx: &mut WindowContext) -> AnyView {
        cx.new_view(|_cx| AuthenticationPrompt).into()
    }

    pub fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &AppContext,
    ) -> BoxFuture<'static, Result<usize>> {
        match request.model {
            crate::LanguageModel::OpenAi(_) => future::ready(Err(anyhow!("invalid model"))).boxed(),
            crate::LanguageModel::ZedDotDev(ZedDotDevModel::GptFour)
            | crate::LanguageModel::ZedDotDev(ZedDotDevModel::GptFourTurbo)
            | crate::LanguageModel::ZedDotDev(ZedDotDevModel::GptThreePointFiveTurbo) => {
                count_open_ai_tokens(request, cx.background_executor())
            }
            crate::LanguageModel::ZedDotDev(ZedDotDevModel::Custom(model)) => {
                let request = self.client.request(proto::CountTokensWithLanguageModel {
                    model,
                    messages: request
                        .messages
                        .iter()
                        .map(|message| message.to_proto())
                        .collect(),
                });
                async move {
                    let response = request.await?;
                    Ok(response.token_count as usize)
                }
                .boxed()
            }
        }
    }

    pub fn complete(
        &self,
        request: LanguageModelRequest,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        let request = proto::CompleteWithLanguageModel {
            model: request.model.id().to_string(),
            messages: request
                .messages
                .iter()
                .map(|message| message.to_proto())
                .collect(),
            stop: request.stop,
            temperature: request.temperature,
        };

        self.client
            .request_stream(request)
            .map_ok(|stream| {
                stream
                    .filter_map(|response| async move {
                        match response {
                            Ok(mut response) => Some(Ok(response.choices.pop()?.delta?.content?)),
                            Err(error) => Some(Err(error)),
                        }
                    })
                    .boxed()
            })
            .boxed()
    }
}

struct AuthenticationPrompt;

impl Render for AuthenticationPrompt {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        const LABEL: &str = "Generate and analyze code with language models. You can dialog with the assistant in this panel or transform code inline.";

        v_flex().gap_6().p_4().child(Label::new(LABEL)).child(
            v_flex()
                .gap_2()
                .child(
                    Button::new("sign_in", "Sign in")
                        .icon_color(Color::Muted)
                        .icon(IconName::Github)
                        .icon_position(IconPosition::Start)
                        .style(ButtonStyle::Filled)
                        .full_width()
                        .on_click(|_, cx| {
                            CompletionProvider::global(cx)
                                .authenticate(cx)
                                .detach_and_log_err(cx);
                        }),
                )
                .child(
                    div().flex().w_full().items_center().child(
                        Label::new("Sign in to enable collaboration.")
                            .color(Color::Muted)
                            .size(LabelSize::Small),
                    ),
                ),
        )
    }
}
