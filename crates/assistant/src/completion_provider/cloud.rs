use crate::{
    assistant_settings::CloudModel, count_open_ai_tokens, CompletionProvider, LanguageModel,
    LanguageModelCompletionProvider, LanguageModelRequest,
};
use anyhow::{anyhow, Result};
use client::{proto, Client};
use futures::{future::BoxFuture, stream::BoxStream, FutureExt, StreamExt, TryFutureExt};
use gpui::{AnyView, AppContext, Task};
use std::{future, sync::Arc};
use strum::IntoEnumIterator;
use ui::prelude::*;

use super::LanguageModelSettings;

pub struct CloudCompletionProvider {
    client: Arc<Client>,
    status: client::Status,
    model: CloudModel,
    _maintain_client_status: Task<()>,
}

impl CloudCompletionProvider {
    pub fn new(client: Arc<Client>, cx: &mut AppContext) -> Self {
        //TODO(completion_provider) is this status really needed?
        let mut status_rx = client.status();
        let status = *status_rx.borrow();
        let maintain_client_status = cx.spawn(|mut cx| async move {
            while let Some(status) = status_rx.next().await {
                let _ = cx.update_global::<CompletionProvider, _>(|provider, _cx| {
                    provider.update_provider_of_type::<_, Self>(|provider| {
                        provider.status = status;
                    });
                });
            }
        });
        Self {
            client,
            model: CloudModel::default(),
            status,
            _maintain_client_status: maintain_client_status,
        }
    }
}

impl LanguageModelSettings for () {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn boxed(&self) -> Box<dyn LanguageModelSettings> {
        Box::new(self.clone())
    }
}

impl LanguageModelCompletionProvider for CloudCompletionProvider {
    type Settings = ();

    fn update(&mut self, _settings: &Self::Settings, _cx: &AppContext) {}

    fn set_model(&mut self, model: LanguageModel, _cx: &mut AppContext) {
        match model {
            LanguageModel::Cloud(model) => {
                self.model = model;
            }
            _ => {}
        }
    }

    fn available_models(&self, _cx: &AppContext) -> Vec<LanguageModel> {
        let mut custom_model = if let CloudModel::Custom(custom_model) = self.model.clone() {
            Some(custom_model)
        } else {
            None
        };
        CloudModel::iter()
            .filter_map(move |model| {
                if let CloudModel::Custom(_) = model {
                    Some(CloudModel::Custom(custom_model.take()?))
                } else {
                    Some(model)
                }
            })
            .map(LanguageModel::Cloud)
            .collect()
    }

    fn is_authenticated(&self) -> bool {
        self.status.is_connected()
    }

    fn authenticate(&self, cx: &AppContext) -> Task<Result<()>> {
        let client = self.client.clone();
        cx.spawn(move |cx| async move { client.authenticate_and_connect(true, &cx).await })
    }

    fn authentication_prompt(&self, cx: &mut WindowContext) -> AnyView {
        cx.new_view(|_cx| AuthenticationPrompt).into()
    }

    fn reset_credentials(&self, _cx: &AppContext) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &AppContext,
    ) -> BoxFuture<'static, Result<usize>> {
        match request.model {
            LanguageModel::Cloud(CloudModel::Gpt4)
            | LanguageModel::Cloud(CloudModel::Gpt4Turbo)
            | LanguageModel::Cloud(CloudModel::Gpt4Omni)
            | LanguageModel::Cloud(CloudModel::Gpt3Point5Turbo) => {
                count_open_ai_tokens(request, cx.background_executor())
            }
            LanguageModel::Cloud(
                CloudModel::Claude3_5Sonnet
                | CloudModel::Claude3Opus
                | CloudModel::Claude3Sonnet
                | CloudModel::Claude3Haiku,
            ) => {
                // Can't find a tokenizer for Claude 3, so for now just use the same as OpenAI's as an approximation.
                count_open_ai_tokens(request, cx.background_executor())
            }
            LanguageModel::Cloud(CloudModel::Custom(model)) => {
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
            _ => future::ready(Err(anyhow!("invalid model"))).boxed(),
        }
    }

    fn complete(
        &self,
        mut request: LanguageModelRequest,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        request.preprocess();

        let request = proto::CompleteWithLanguageModel {
            model: request.model.id().to_string(),
            messages: request
                .messages
                .iter()
                .map(|message| message.to_proto())
                .collect(),
            stop: request.stop,
            temperature: request.temperature,
            tools: Vec::new(),
            tool_choice: None,
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
