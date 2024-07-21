use super::open_ai::count_open_ai_tokens;
use crate::{
    CloudModel, LanguageModel, LanguageModelId, LanguageModelName, LanguageModelProviderName,
    LanguageModelProviderState, LanguageModelRequest, ProvidedLanguageModel,
};
use anyhow::{anyhow, Result};
use client::Client;
use futures::{
    future::{self, BoxFuture},
    stream::BoxStream,
    FutureExt, StreamExt, TryFutureExt,
};
use gpui::{AnyView, AppContext, Render, Task};
use std::sync::Arc;
use strum::IntoEnumIterator;
use ui::{prelude::*, IntoElement, ViewContext};

use crate::LanguageModelProvider;

use super::anthropic::{count_anthropic_tokens, preprocess_anthropic_request};

const PROVIDER_NAME: &str = "zed.dev";

pub struct CloudLanguageModelProvider {
    client: Arc<Client>,
    state: gpui::Model<State>,
    _maintain_client_status: Task<()>,
}

struct State {
    client: Arc<Client>,
    status: client::Status,
}

impl State {
    fn authenticate(&self, cx: &AppContext) -> Task<Result<()>> {
        let client = self.client.clone();
        cx.spawn(move |cx| async move { client.authenticate_and_connect(true, &cx).await })
    }
}

impl CloudLanguageModelProvider {
    pub fn new(client: Arc<Client>, cx: &mut AppContext) -> Self {
        let mut status_rx = client.status();
        let status = *status_rx.borrow();

        let state = cx.new_model(|_| State {
            client: client.clone(),
            status,
        });

        let state_ref = state.downgrade();
        let maintain_client_status = cx.spawn(|mut cx| async move {
            while let Some(status) = status_rx.next().await {
                if let Some(this) = state_ref.upgrade() {
                    _ = this.update(&mut cx, |this, cx| {
                        this.status = status;
                        cx.notify();
                    });
                } else {
                    break;
                }
            }
        });

        Self {
            client,
            state,
            _maintain_client_status: maintain_client_status,
        }
    }
}

impl LanguageModelProviderState for CloudLanguageModelProvider {
    fn subscribe<T: 'static>(&self, cx: &mut gpui::ModelContext<T>) -> gpui::Subscription {
        cx.observe(&self.state, |_, _, cx| {
            cx.notify();
        })
    }
}

impl LanguageModelProvider for CloudLanguageModelProvider {
    fn name(&self, _cx: &AppContext) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn provided_models(&self, _cx: &AppContext) -> Vec<ProvidedLanguageModel> {
        CloudModel::iter()
            .filter(|model| !matches!(model, CloudModel::Custom(_)))
            .map(|model| ProvidedLanguageModel {
                id: LanguageModelId::from(model.id().to_string()),
                name: LanguageModelName::from(model.display_name().to_string()),
            })
            .collect()
    }

    fn is_authenticated(&self, cx: &AppContext) -> bool {
        self.state.read(cx).status.is_connected()
    }

    fn authenticate(&self, cx: &AppContext) -> Task<Result<()>> {
        self.state.read(cx).authenticate(cx)
    }

    fn authentication_prompt(&self, cx: &mut WindowContext) -> AnyView {
        cx.new_view(|_cx| AuthenticationPrompt {
            state: self.state.clone(),
        })
        .into()
    }

    fn reset_credentials(&self, _cx: &AppContext) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn model(&self, id: LanguageModelId, _cx: &AppContext) -> Result<Arc<dyn LanguageModel>> {
        let model = CloudModel::from(id.0.as_ref());
        Ok(Arc::new(CloudLanguageModel {
            id,
            model,
            client: self.client.clone(),
        }))
    }
}

pub struct CloudLanguageModel {
    id: LanguageModelId,
    model: CloudModel,
    client: Arc<Client>,
}

impl LanguageModel for CloudLanguageModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(self.model.display_name().to_string())
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn telemetry_id(&self) -> String {
        format!("zed.dev/{}", self.model.id())
    }

    fn max_token_count(&self) -> usize {
        self.model.max_token_count()
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &AppContext,
    ) -> BoxFuture<'static, Result<usize>> {
        let model = CloudModel::from(self.id.0.as_ref());
        match model {
            CloudModel::Gpt3Point5Turbo => {
                count_open_ai_tokens(request, open_ai::Model::ThreePointFiveTurbo, cx)
            }
            CloudModel::Gpt4 => count_open_ai_tokens(request, open_ai::Model::Four, cx),
            CloudModel::Gpt4Turbo => count_open_ai_tokens(request, open_ai::Model::FourTurbo, cx),
            CloudModel::Gpt4Omni => count_open_ai_tokens(request, open_ai::Model::FourOmni, cx),
            CloudModel::Gpt4OmniMini => {
                count_open_ai_tokens(request, open_ai::Model::FourOmniMini, cx)
            }
            CloudModel::Claude3_5Sonnet
            | CloudModel::Claude3Opus
            | CloudModel::Claude3Sonnet
            | CloudModel::Claude3Haiku => count_anthropic_tokens(request, cx),
            CloudModel::Custom(model) => {
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
            //TODO: how to handle Gemini models?
            _ => future::ready(Err(anyhow!("invalid model"))).boxed(),
        }
    }

    fn stream_completion(
        &self,
        mut request: LanguageModelRequest,
        _cx: &AppContext,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        match self.model {
            CloudModel::Claude3Opus
            | CloudModel::Claude3Sonnet
            | CloudModel::Claude3Haiku
            | CloudModel::Claude3_5Sonnet => preprocess_anthropic_request(&mut request),
            _ => {}
        }

        let request = proto::CompleteWithLanguageModel {
            model: self.id.0.to_string(),
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

struct AuthenticationPrompt {
    state: gpui::Model<State>,
}

impl Render for AuthenticationPrompt {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
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
                        .on_click(cx.listener(move |this, _, cx| {
                            this.state.update(cx, |provider, cx| {
                                provider.authenticate(cx).detach_and_log_err(cx);
                                cx.notify();
                            });
                        })),
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
