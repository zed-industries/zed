use super::open_ai::count_open_ai_tokens;
use crate::{
    LanguageModel, LanguageModelId, LanguageModelName, LanguageModelProviderName,
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
use schemars::{
    schema::{InstanceType, Metadata, Schema, SchemaObject},
    JsonSchema,
};
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt, sync::Arc};
use strum::{EnumIter, IntoEnumIterator};
use ui::{prelude::*, IntoElement, ViewContext};

use crate::LanguageModelProvider;

use super::anthropic::{count_anthropic_tokens, preprocess_anthropic_request};

const PROVIDER_NAME: &str = "zed.dev";

#[derive(Clone, Debug, Default, PartialEq, EnumIter)]
pub enum CloudModel {
    Gpt3Point5Turbo,
    Gpt4,
    Gpt4Turbo,
    #[default]
    Gpt4Omni,
    Gpt4OmniMini,
    Claude3_5Sonnet,
    Claude3Opus,
    Claude3Sonnet,
    Claude3Haiku,
    Gemini15Pro,
    Gemini15Flash,
    Custom(String),
}

impl Serialize for CloudModel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.id())
    }
}

impl<'de> Deserialize<'de> for CloudModel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ZedDotDevModelVisitor;

        impl<'de> Visitor<'de> for ZedDotDevModelVisitor {
            type Value = CloudModel;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string for a ZedDotDevModel variant or a custom model")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let model = CloudModel::iter()
                    .find(|model| model.id() == value)
                    .unwrap_or_else(|| CloudModel::Custom(value.to_string()));
                Ok(model)
            }
        }

        deserializer.deserialize_str(ZedDotDevModelVisitor)
    }
}

impl JsonSchema for CloudModel {
    fn schema_name() -> String {
        "ZedDotDevModel".to_owned()
    }

    fn json_schema(_generator: &mut schemars::gen::SchemaGenerator) -> Schema {
        let variants = CloudModel::iter()
            .filter_map(|model| {
                let id = model.id();
                if id.is_empty() {
                    None
                } else {
                    Some(id.to_string())
                }
            })
            .collect::<Vec<_>>();
        Schema::Object(SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            enum_values: Some(variants.iter().map(|s| s.clone().into()).collect()),
            metadata: Some(Box::new(Metadata {
                title: Some("ZedDotDevModel".to_owned()),
                default: Some(CloudModel::default().id().into()),
                examples: variants.into_iter().map(Into::into).collect(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}

impl CloudModel {
    pub fn id(&self) -> &str {
        match self {
            Self::Gpt3Point5Turbo => "gpt-3.5-turbo",
            Self::Gpt4 => "gpt-4",
            Self::Gpt4Turbo => "gpt-4-turbo-preview",
            Self::Gpt4Omni => "gpt-4o",
            Self::Gpt4OmniMini => "gpt-4o-mini",
            Self::Claude3_5Sonnet => "claude-3-5-sonnet",
            Self::Claude3Opus => "claude-3-opus",
            Self::Claude3Sonnet => "claude-3-sonnet",
            Self::Claude3Haiku => "claude-3-haiku",
            Self::Gemini15Pro => "gemini-1.5-pro",
            Self::Gemini15Flash => "gemini-1.5-flash",
            Self::Custom(id) => id,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Gpt3Point5Turbo => "GPT 3.5 Turbo",
            Self::Gpt4 => "GPT 4",
            Self::Gpt4Turbo => "GPT 4 Turbo",
            Self::Gpt4Omni => "GPT 4 Omni",
            Self::Gpt4OmniMini => "GPT 4 Omni Mini",
            Self::Claude3_5Sonnet => "Claude 3.5 Sonnet",
            Self::Claude3Opus => "Claude 3 Opus",
            Self::Claude3Sonnet => "Claude 3 Sonnet",
            Self::Claude3Haiku => "Claude 3 Haiku",
            Self::Gemini15Pro => "Gemini 1.5 Pro",
            Self::Gemini15Flash => "Gemini 1.5 Flash",
            Self::Custom(id) => id.as_str(),
        }
    }

    pub fn max_token_count(&self) -> usize {
        match self {
            Self::Gpt3Point5Turbo => 2048,
            Self::Gpt4 => 4096,
            Self::Gpt4Turbo | Self::Gpt4Omni => 128000,
            Self::Gpt4OmniMini => 128000,
            Self::Claude3_5Sonnet
            | Self::Claude3Opus
            | Self::Claude3Sonnet
            | Self::Claude3Haiku => 200000,
            Self::Gemini15Pro => 128000,
            Self::Gemini15Flash => 32000,
            Self::Custom(_) => 4096, // TODO: Make this configurable
        }
    }
}

impl From<&str> for CloudModel {
    fn from(value: &str) -> Self {
        match value {
            "gpt-3.5-turbo" => Self::Gpt3Point5Turbo,
            "gpt-4" => Self::Gpt4,
            "gpt-4-turbo-preview" => Self::Gpt4Turbo,
            "gpt-4o" => Self::Gpt4Omni,
            "gpt-4o-mini" => Self::Gpt4OmniMini,
            "claude-3-5-sonnet" => Self::Claude3_5Sonnet,
            "claude-3-opus" => Self::Claude3Opus,
            "claude-3-sonnet" => Self::Claude3Sonnet,
            "claude-3-haiku" => Self::Claude3Haiku,
            "gemini-1.5-pro" => Self::Gemini15Pro,
            "gemini-1.5-flash" => Self::Gemini15Flash,
            _ => Self::Custom(value.to_string()),
        }
    }
}

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
