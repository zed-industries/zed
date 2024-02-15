use crate::{LanguageModelChoiceDelta, LanguageModelRequest, LanguageModelUsage};
use anyhow::{anyhow, Result};
use futures::{
    future::BoxFuture, io::BufReader, stream::BoxStream, AsyncBufReadExt, AsyncReadExt, FutureExt,
    Stream, StreamExt,
};
use gpui::{AppContext, BackgroundExecutor, Task};
use isahc::{http::StatusCode, Request, RequestExt};
use parking_lot::Mutex;
use serde::Deserialize;
use std::{env, io, sync::Arc};

pub const OPEN_AI_API_URL: &'static str = "https://api.openai.com/v1";

#[derive(Clone)]
pub struct OpenAiCompletionProvider {
    api_key: Arc<Mutex<Option<String>>>,
    // todo!("move api_key_editor here")
    executor: BackgroundExecutor,
}

impl OpenAiCompletionProvider {
    pub fn is_authenticated(&self) -> bool {
        self.api_key.lock().is_some()
    }

    pub fn authenticate(&self, cx: &AppContext) -> Task<Result<()>> {
        if let Ok(api_key) = env::var("OPENAI_API_KEY") {
            *self.api_key.lock() = Some(api_key);
            Task::ready(Ok(()))
        } else {
            Task::ready(Err(anyhow!(
                "OPENAI_API_KEY environment variable not found"
            )))
        }
    }

    pub fn complete(
        &self,
        request: LanguageModelRequest,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        let api_key = self.api_key.lock().clone();
        let executor = self.executor.clone();
        async move {
            let api_key = api_key.ok_or_else(|| anyhow!("missing api key"))?;
            let request = stream_completion(api_key, executor, request);
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
}

#[derive(Deserialize, Debug)]
pub struct OpenAiResponseStreamEvent {
    pub id: Option<String>,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<LanguageModelChoiceDelta>,
    pub usage: Option<LanguageModelUsage>,
}

pub async fn stream_completion(
    api_key: String,
    executor: BackgroundExecutor,
    request: LanguageModelRequest,
) -> Result<impl Stream<Item = Result<OpenAiResponseStreamEvent>>> {
    let (tx, rx) = futures::channel::mpsc::unbounded::<Result<OpenAiResponseStreamEvent>>();

    let mut response = Request::post(format!("{OPEN_AI_API_URL}/chat/completions"))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .body(serde_json::to_string(&request).unwrap())?
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
