use super::*;
use anthropic::preprocess_anthropic_request;
use client::Client;
use futures::{FutureExt, StreamExt, TryFutureExt};
use gpui::{ModelContext, Render};
use std::sync::Arc;
use strum::{EnumIter, IntoEnumIterator};
use ui::{prelude::*, IntoElement, ViewContext};

use crate::LanguageModelProvider;

#[derive(Clone, Debug, Default, PartialEq, EnumIter)]
pub enum CloudModel {
    Gpt3Point5Turbo,
    Gpt4,
    Gpt4Turbo,
    #[default]
    Gpt4Omni,
    Claude3_5Sonnet,
    Claude3Opus,
    Claude3Sonnet,
    Claude3Haiku,
    Custom(String),
}

impl From<&str> for CloudModel {
    fn from(value: &str) -> Self {
        match value {
            "gpt-3.5-turbo" => Self::Gpt3Point5Turbo,
            "gpt-4" => Self::Gpt4,
            "gpt-4-turbo-preview" => Self::Gpt4Turbo,
            "gpt-4o" => Self::Gpt4Omni,
            "claude-3-5-sonnet" => Self::Claude3_5Sonnet,
            "claude-3-opus" => Self::Claude3Opus,
            "claude-3-sonnet" => Self::Claude3Sonnet,
            "claude-3-haiku" => Self::Claude3Haiku,
            _ => Self::Custom(value.to_string()),
        }
    }
}

impl CloudModel {
    pub fn id(&self) -> &str {
        match self {
            Self::Gpt3Point5Turbo => "gpt-3.5-turbo",
            Self::Gpt4 => "gpt-4",
            Self::Gpt4Turbo => "gpt-4-turbo-preview",
            Self::Gpt4Omni => "gpt-4o",
            Self::Claude3_5Sonnet => "claude-3-5-sonnet",
            Self::Claude3Opus => "claude-3-opus",
            Self::Claude3Sonnet => "claude-3-sonnet",
            Self::Claude3Haiku => "claude-3-haiku",
            Self::Custom(id) => id,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Gpt3Point5Turbo => "GPT 3.5 Turbo",
            Self::Gpt4 => "GPT 4",
            Self::Gpt4Turbo => "GPT 4 Turbo",
            Self::Gpt4Omni => "GPT 4 Omni",
            Self::Claude3_5Sonnet => "Claude 3.5 Sonnet",
            Self::Claude3Opus => "Claude 3 Opus",
            Self::Claude3Sonnet => "Claude 3 Sonnet",
            Self::Claude3Haiku => "Claude 3 Haiku",
            Self::Custom(id) => id.as_str(),
        }
    }
}

pub struct CloudLanguageModelProvider {
    client: Arc<Client>,
    status: client::Status,
    _maintain_client_status: Task<()>,
}

impl CloudLanguageModelProvider {
    pub fn new(client: Arc<Client>, cx: &mut ModelContext<Self>) -> Self {
        let mut status_rx = client.status();
        let status = *status_rx.borrow();

        let maintain_client_status = cx.spawn(|this, mut cx| async move {
            while let Some(status) = status_rx.next().await {
                if let Some(this) = this.upgrade() {
                    _ = this.update(&mut cx, |this, _| {
                        this.status = status;
                    });
                } else {
                    break;
                }
            }
        });

        Self {
            client,
            status,
            _maintain_client_status: maintain_client_status,
        }
    }
}

impl LanguageModelProvider for CloudLanguageModelProvider {
    fn name(&self, _cx: &AppContext) -> LanguageModelProviderName {
        LanguageModelProviderName("Cloud".into())
    }

    fn provided_models(&self, _cx: &AppContext) -> Vec<ProvidedLanguageModel> {
        CloudModel::iter()
            .map(|model| ProvidedLanguageModel {
                id: LanguageModelId::from(model.id().to_string()),
                name: LanguageModelName::from(model.display_name().to_string()),
            })
            .collect()
    }

    fn is_authenticated(&self, _cx: &AppContext) -> bool {
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

    fn model(&self, id: LanguageModelId, _cx: &AppContext) -> Result<Arc<dyn LanguageModel>> {
        Ok(Arc::new(CloudLanguageModel {
            id,
            client: self.client.clone(),
        }))
    }
}

pub struct CloudLanguageModel {
    id: LanguageModelId,
    client: Arc<Client>,
}

impl LanguageModel for CloudLanguageModel {
    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &AppContext,
    ) -> BoxFuture<'static, Result<usize>> {
        todo!()
    }

    fn complete(
        &self,
        mut request: LanguageModelRequest,
        _cx: &mut AppContext,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        match CloudModel::from(self.id.0.as_ref()) {
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
                            //TODO auth
                            // CompletionProvider::global(cx)
                            //     .authenticate(cx)
                            //     .detach_and_log_err(cx);
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
