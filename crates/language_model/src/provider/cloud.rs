use super::open_ai::count_open_ai_tokens;
use crate::{
    settings::AllLanguageModelSettings, CloudModel, LanguageModel, LanguageModelId,
    LanguageModelName, LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelProviderState, LanguageModelRequest, RateLimiter,
};
use anyhow::{anyhow, Context as _, Result};
use client::Client;
use collections::BTreeMap;
use futures::{future::BoxFuture, stream::BoxStream, FutureExt, StreamExt};
use gpui::{AnyView, AppContext, AsyncAppContext, ModelContext, Subscription, Task};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use std::{future, sync::Arc};
use strum::IntoEnumIterator;
use ui::prelude::*;

use crate::LanguageModelProvider;

use super::anthropic::count_anthropic_tokens;

pub const PROVIDER_ID: &str = "zed.dev";
pub const PROVIDER_NAME: &str = "zed.dev";

#[derive(Default, Clone, Debug, PartialEq)]
pub struct ZedDotDevSettings {
    pub available_models: Vec<AvailableModel>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum AvailableProvider {
    Anthropic,
    OpenAi,
    Google,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AvailableModel {
    provider: AvailableProvider,
    name: String,
    max_tokens: usize,
    tool_override: Option<String>,
}

pub struct CloudLanguageModelProvider {
    client: Arc<Client>,
    state: gpui::Model<State>,
    _maintain_client_status: Task<()>,
}

pub struct State {
    client: Arc<Client>,
    status: client::Status,
    _subscription: Subscription,
}

impl State {
    fn authenticate(&self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        let client = self.client.clone();
        cx.spawn(move |this, mut cx| async move {
            client.authenticate_and_connect(true, &cx).await?;
            this.update(&mut cx, |_, cx| cx.notify())
        })
    }
}

impl CloudLanguageModelProvider {
    pub fn new(client: Arc<Client>, cx: &mut AppContext) -> Self {
        let mut status_rx = client.status();
        let status = *status_rx.borrow();

        let state = cx.new_model(|cx| State {
            client: client.clone(),
            status,
            _subscription: cx.observe_global::<SettingsStore>(|_, cx| {
                cx.notify();
            }),
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
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<gpui::Model<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for CloudLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        LanguageModelProviderId(PROVIDER_ID.into())
    }

    fn name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn provided_models(&self, cx: &AppContext) -> Vec<Arc<dyn LanguageModel>> {
        let mut models = BTreeMap::default();

        for model in anthropic::Model::iter() {
            if !matches!(model, anthropic::Model::Custom { .. }) {
                models.insert(model.id().to_string(), CloudModel::Anthropic(model));
            }
        }
        for model in open_ai::Model::iter() {
            if !matches!(model, open_ai::Model::Custom { .. }) {
                models.insert(model.id().to_string(), CloudModel::OpenAi(model));
            }
        }
        for model in google_ai::Model::iter() {
            if !matches!(model, google_ai::Model::Custom { .. }) {
                models.insert(model.id().to_string(), CloudModel::Google(model));
            }
        }

        // Override with available models from settings
        for model in &AllLanguageModelSettings::get_global(cx)
            .zed_dot_dev
            .available_models
        {
            let model = match model.provider {
                AvailableProvider::Anthropic => CloudModel::Anthropic(anthropic::Model::Custom {
                    name: model.name.clone(),
                    max_tokens: model.max_tokens,
                    tool_override: model.tool_override.clone(),
                }),
                AvailableProvider::OpenAi => CloudModel::OpenAi(open_ai::Model::Custom {
                    name: model.name.clone(),
                    max_tokens: model.max_tokens,
                }),
                AvailableProvider::Google => CloudModel::Google(google_ai::Model::Custom {
                    name: model.name.clone(),
                    max_tokens: model.max_tokens,
                }),
            };
            models.insert(model.id().to_string(), model.clone());
        }

        models
            .into_values()
            .map(|model| {
                Arc::new(CloudLanguageModel {
                    id: LanguageModelId::from(model.id().to_string()),
                    model,
                    client: self.client.clone(),
                    request_limiter: RateLimiter::new(4),
                }) as Arc<dyn LanguageModel>
            })
            .collect()
    }

    fn is_authenticated(&self, cx: &AppContext) -> bool {
        self.state.read(cx).status.is_connected()
    }

    fn authenticate(&self, cx: &mut AppContext) -> Task<Result<()>> {
        self.state.update(cx, |state, cx| state.authenticate(cx))
    }

    fn authentication_prompt(&self, cx: &mut WindowContext) -> AnyView {
        cx.new_view(|_cx| AuthenticationPrompt {
            state: self.state.clone(),
        })
        .into()
    }

    fn reset_credentials(&self, _cx: &mut AppContext) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }
}

pub struct CloudLanguageModel {
    id: LanguageModelId,
    model: CloudModel,
    client: Arc<Client>,
    request_limiter: RateLimiter,
}

impl LanguageModel for CloudLanguageModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
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
        match self.model.clone() {
            CloudModel::Anthropic(_) => count_anthropic_tokens(request, cx),
            CloudModel::OpenAi(model) => count_open_ai_tokens(request, model, cx),
            CloudModel::Google(model) => {
                let client = self.client.clone();
                let request = request.into_google(model.id().into());
                let request = google_ai::CountTokensRequest {
                    contents: request.contents,
                };
                async move {
                    let request = serde_json::to_string(&request)?;
                    let response = client
                        .request(proto::CountLanguageModelTokens {
                            provider: proto::LanguageModelProvider::Google as i32,
                            request,
                        })
                        .await?;
                    Ok(response.token_count as usize)
                }
                .boxed()
            }
        }
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        _: &AsyncAppContext,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        match &self.model {
            CloudModel::Anthropic(model) => {
                let client = self.client.clone();
                let request = request.into_anthropic(model.id().into());
                let future = self.request_limiter.stream(async move {
                    let request = serde_json::to_string(&request)?;
                    let stream = client
                        .request_stream(proto::StreamCompleteWithLanguageModel {
                            provider: proto::LanguageModelProvider::Anthropic as i32,
                            request,
                        })
                        .await?;
                    Ok(anthropic::extract_text_from_events(
                        stream.map(|item| Ok(serde_json::from_str(&item?.event)?)),
                    ))
                });
                async move { Ok(future.await?.boxed()) }.boxed()
            }
            CloudModel::OpenAi(model) => {
                let client = self.client.clone();
                let request = request.into_open_ai(model.id().into());
                let future = self.request_limiter.stream(async move {
                    let request = serde_json::to_string(&request)?;
                    let stream = client
                        .request_stream(proto::StreamCompleteWithLanguageModel {
                            provider: proto::LanguageModelProvider::OpenAi as i32,
                            request,
                        })
                        .await?;
                    Ok(open_ai::extract_text_from_events(
                        stream.map(|item| Ok(serde_json::from_str(&item?.event)?)),
                    ))
                });
                async move { Ok(future.await?.boxed()) }.boxed()
            }
            CloudModel::Google(model) => {
                let client = self.client.clone();
                let request = request.into_google(model.id().into());
                let future = self.request_limiter.stream(async move {
                    let request = serde_json::to_string(&request)?;
                    let stream = client
                        .request_stream(proto::StreamCompleteWithLanguageModel {
                            provider: proto::LanguageModelProvider::Google as i32,
                            request,
                        })
                        .await?;
                    Ok(google_ai::extract_text_from_events(
                        stream.map(|item| Ok(serde_json::from_str(&item?.event)?)),
                    ))
                });
                async move { Ok(future.await?.boxed()) }.boxed()
            }
        }
    }

    fn use_any_tool(
        &self,
        request: LanguageModelRequest,
        tool_name: String,
        tool_description: String,
        input_schema: serde_json::Value,
        _cx: &AsyncAppContext,
    ) -> BoxFuture<'static, Result<serde_json::Value>> {
        match &self.model {
            CloudModel::Anthropic(model) => {
                let client = self.client.clone();
                let mut request = request.into_anthropic(model.tool_model_id().into());
                request.tool_choice = Some(anthropic::ToolChoice::Tool {
                    name: tool_name.clone(),
                });
                request.tools = vec![anthropic::Tool {
                    name: tool_name.clone(),
                    description: tool_description,
                    input_schema,
                }];

                self.request_limiter
                    .run(async move {
                        let request = serde_json::to_string(&request)?;
                        let response = client
                            .request(proto::CompleteWithLanguageModel {
                                provider: proto::LanguageModelProvider::Anthropic as i32,
                                request,
                            })
                            .await?;
                        let response: anthropic::Response =
                            serde_json::from_str(&response.completion)?;
                        response
                            .content
                            .into_iter()
                            .find_map(|content| {
                                if let anthropic::Content::ToolUse { name, input, .. } = content {
                                    if name == tool_name {
                                        Some(input)
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            })
                            .context("tool not used")
                    })
                    .boxed()
            }
            CloudModel::OpenAi(_) => {
                future::ready(Err(anyhow!("tool use not implemented for OpenAI"))).boxed()
            }
            CloudModel::Google(_) => {
                future::ready(Err(anyhow!("tool use not implemented for Google AI"))).boxed()
            }
        }
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
