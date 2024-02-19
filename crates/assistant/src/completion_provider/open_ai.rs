use crate::{assistant_settings::OpenAiModel, LanguageModel, LanguageModelRequest, Role};
use anyhow::{anyhow, Result};
use futures::{future::BoxFuture, stream::BoxStream, FutureExt, StreamExt};
use gpui::{AppContext, Task};
use open_ai::{stream_completion, Request, RequestMessage, Role as OpenAiRole};
use std::{env, sync::Arc};
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
        // todo!("validate api key")

        if self.is_authenticated() {
            Task::ready(Ok(()))
        } else {
            Task::ready(Err(anyhow!(
                "OPENAI_API_KEY environment variable not found"
            )))
        }
    }

    pub fn default_model(&self) -> OpenAiModel {
        self.default_model.clone()
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
            Some(LanguageModel::OpenAi(model)) => model,
            _ => self.default_model(),
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

impl Into<open_ai::Role> for Role {
    fn into(self) -> open_ai::Role {
        match self {
            Role::User => OpenAiRole::User,
            Role::Assistant => OpenAiRole::Assistant,
            Role::System => OpenAiRole::System,
        }
    }
}
