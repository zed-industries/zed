use crate::{assistant_settings::OpenAiModel, LanguageModel, LanguageModelRequest, Role};
use anyhow::{anyhow, Result};
use editor::Editor;
use futures::{future::BoxFuture, stream::BoxStream, FutureExt, StreamExt};
use gpui::{AnyView, AppContext, Task, View};
use open_ai::{stream_completion, Request, RequestMessage, Role as OpenAiRole};
use std::{env, sync::Arc};
use ui::prelude::*;
use util::{http::HttpClient, ResultExt};

pub struct OpenAiCompletionProvider {
    api_key: Option<String>,
    api_url: String,
    default_model: OpenAiModel,
    http_client: Arc<dyn HttpClient>,
}

impl OpenAiCompletionProvider {
    pub fn new(
        default_model: OpenAiModel,
        api_url: String,
        http_client: Arc<dyn HttpClient>,
    ) -> Self {
        Self {
            api_key: env::var("OPENAI_API_KEY").log_err(),
            api_url,
            default_model,
            http_client,
        }
    }

    pub fn update(&mut self, default_model: OpenAiModel, api_url: String) {
        self.default_model = default_model;
        self.api_url = api_url;
    }

    pub fn is_authenticated(&self) -> bool {
        self.api_key.is_some()
    }

    pub fn authenticate(&self, _cx: &AppContext) -> Task<Result<()>> {
        if self.is_authenticated() {
            Task::ready(Ok(()))
        } else {
            Task::ready(Err(anyhow!(
                "OPENAI_API_KEY environment variable not found"
            )))
        }
    }

    pub fn authentication_prompt(&self, cx: &mut WindowContext) -> AnyView {
        cx.new_view(|cx| AuthenticationPrompt::new(cx)).into()
    }

    pub fn default_model(&self) -> OpenAiModel {
        self.default_model.clone()
    }

    pub fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &AppContext,
    ) -> BoxFuture<'static, Result<usize>> {
        count_open_ai_tokens(request, cx.background_executor())
    }

    pub fn complete(
        &self,
        request: LanguageModelRequest,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        let request = self.to_open_ai_request(request);

        let http_client = self.http_client.clone();
        let api_key = self.api_key.clone();
        let api_url = self.api_url.clone();
        async move {
            let api_key = api_key.ok_or_else(|| anyhow!("missing api key"))?;
            let request = stream_completion(http_client.as_ref(), &api_url, &api_key, request);
            let response = request.await?;
            let stream = response
                .filter_map(|response| async move {
                    match response {
                        Ok(mut response) => Some(Ok(response.choices.pop()?.delta.content?)),
                        Err(error) => Some(Err(error)),
                    }
                })
                .boxed();
            Ok(stream)
        }
        .boxed()
    }

    fn to_open_ai_request(&self, request: LanguageModelRequest) -> Request {
        let model = match request.model {
            LanguageModel::ZedDotDev(_) => self.default_model(),
            LanguageModel::OpenAi(model) => model,
        };

        Request {
            model,
            messages: request
                .messages
                .into_iter()
                .map(|msg| RequestMessage {
                    role: msg.role.into(),
                    content: msg.content,
                })
                .collect(),
            stream: true,
            stop: request.stop,
            temperature: request.temperature,
        }
    }
}

pub fn count_open_ai_tokens(
    request: LanguageModelRequest,
    background_executor: &gpui::BackgroundExecutor,
) -> BoxFuture<'static, Result<usize>> {
    background_executor
        .spawn(async move {
            let messages = request
                .messages
                .into_iter()
                .map(|message| tiktoken_rs::ChatCompletionRequestMessage {
                    role: match message.role {
                        Role::User => "user".into(),
                        Role::Assistant => "assistant".into(),
                        Role::System => "system".into(),
                    },
                    content: Some(message.content),
                    name: None,
                    function_call: None,
                })
                .collect::<Vec<_>>();

            tiktoken_rs::num_tokens_from_messages(request.model.id(), &messages)
        })
        .boxed()
}

impl From<Role> for open_ai::Role {
    fn from(val: Role) -> Self {
        match val {
            Role::User => OpenAiRole::User,
            Role::Assistant => OpenAiRole::Assistant,
            Role::System => OpenAiRole::System,
        }
    }
}

struct AuthenticationPrompt {
    api_key: View<Editor>,
}

impl AuthenticationPrompt {
    fn new(cx: &mut WindowContext) -> Self {
        Self {
            api_key: cx.new_view(|cx| {
                let mut editor = Editor::single_line(cx);
                editor.set_placeholder_text(
                    "sk-000000000000000000000000000000000000000000000000",
                    cx,
                );
                editor
            }),
        }
    }
}

impl Render for AuthenticationPrompt {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .gap_6()
            .p_4()
            .child(Label::new(concat!(
                "To use the assistant with OpenAI, please assign an OPENAI_API_KEY ",
                "environment variable, then restart Zed.",
            )))
            .into_any()
    }
}
