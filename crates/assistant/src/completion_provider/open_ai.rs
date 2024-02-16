use crate::{
    assistant_settings::OpenAiModel, LanguageModel, LanguageModelChoiceDelta, LanguageModelRequest,
    LanguageModelUsage, Role,
};
use anyhow::{anyhow, Result};
use futures::{
    future::BoxFuture, io::BufReader, stream::BoxStream, AsyncBufReadExt, AsyncReadExt, FutureExt,
    Stream, StreamExt,
};
use gpui::{AppContext, BackgroundExecutor, Task};
use isahc::{http::StatusCode, Request, RequestExt};
use serde::{Deserialize, Serialize};
use std::{env, io};
use util::ResultExt;

pub struct OpenAiCompletionProvider {
    api_key: Option<String>,
    api_url: String,
    default_model: OpenAiModel,
    executor: BackgroundExecutor,
}

impl OpenAiCompletionProvider {
    pub fn new(default_model: OpenAiModel, api_url: String, cx: &AppContext) -> Self {
        Self {
            api_key: env::var("OPENAI_API_KEY").log_err(),
            api_url,
            default_model,
            executor: cx.background_executor().clone(),
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

        let api_key = self.api_key.clone();
        let api_url = self.api_url.clone();
        let executor = self.executor.clone();
        async move {
            let api_key = api_key.ok_or_else(|| anyhow!("missing api key"))?;
            let request = stream_completion(&api_key, &api_url, executor, request);
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

    fn to_open_ai_request(&self, request: LanguageModelRequest) -> OpenAiRequest {
        let model = match request.model {
            Some(LanguageModel::OpenAi(model)) => model,
            _ => self.default_model(),
        };
        OpenAiRequest {
            model,
            messages: request
                .messages
                .into_iter()
                .map(|msg| OpenAiRequestMessage {
                    role: msg.role,
                    content: msg.content,
                })
                .collect(),
            stream: request.stream,
            stop: request.stop,
            temperature: request.temperature,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct OpenAiRequest {
    pub model: OpenAiModel,
    pub messages: Vec<OpenAiRequestMessage>,
    pub stream: bool,
    pub stop: Vec<String>,
    pub temperature: f32,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct OpenAiRequestMessage {
    pub role: Role,
    pub content: String,
}

#[derive(Deserialize, Debug)]
struct OpenAiResponseStreamEvent {
    pub id: Option<String>,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<LanguageModelChoiceDelta>,
    pub usage: Option<LanguageModelUsage>,
}

async fn stream_completion(
    api_key: &str,
    api_url: &str,
    executor: BackgroundExecutor,
    request: OpenAiRequest,
) -> Result<impl Stream<Item = Result<OpenAiResponseStreamEvent>>> {
    let (tx, rx) = futures::channel::mpsc::unbounded::<Result<OpenAiResponseStreamEvent>>();

    let mut response = Request::post(format!("{api_url}/chat/completions"))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .body(dbg!(serde_json::to_string(&request).unwrap()))?
        .send_async()
        .await?;

    let status = response.status();
    if status == StatusCode::OK {
        executor
            .spawn(async move {
                let mut lines = BufReader::new(response.body_mut()).lines();

                fn parse_line(
                    line: Result<String, io::Error>,
                ) -> Result<Option<OpenAiResponseStreamEvent>> {
                    if let Some(data) = line?.strip_prefix("data: ") {
                        let event = serde_json::from_str(data)?;
                        Ok(Some(event))
                    } else {
                        Ok(None)
                    }
                }

                while let Some(line) = lines.next().await {
                    if let Some(event) = parse_line(line).transpose() {
                        let done = event.as_ref().map_or(false, |event| {
                            event
                                .choices
                                .last()
                                .map_or(false, |choice| choice.finish_reason.is_some())
                        });
                        if tx.unbounded_send(event).is_err() {
                            break;
                        }

                        if done {
                            break;
                        }
                    }
                }

                anyhow::Ok(())
            })
            .detach();

        Ok(rx)
    } else {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;

        #[derive(Deserialize)]
        struct OpenAiResponse {
            error: OpenAiError,
        }

        #[derive(Deserialize)]
        struct OpenAiError {
            message: String,
        }

        match serde_json::from_str::<OpenAiResponse>(&body) {
            Ok(response) if !response.error.message.is_empty() => Err(anyhow!(
                "Failed to connect to OpenAI API: {}",
                response.error.message,
            )),

            _ => Err(anyhow!(
                "Failed to connect to OpenAI API: {} {}",
                response.status(),
                body,
            )),
        }
    }
}
