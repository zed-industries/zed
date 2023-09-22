use anyhow::{anyhow, Result};
use futures::{
    future::BoxFuture, io::BufReader, stream::BoxStream, AsyncBufReadExt, AsyncReadExt, FutureExt,
    Stream, StreamExt,
};
use gpui::executor::Background;
use isahc::{http::StatusCode, Request, RequestExt};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display},
    io,
    sync::Arc,
};

pub const OPENAI_API_URL: &'static str = "https://api.openai.com/v1";

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
}

impl Role {
    pub fn cycle(&mut self) {
        *self = match self {
            Role::User => Role::Assistant,
            Role::Assistant => Role::System,
            Role::System => Role::User,
        }
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::User => write!(f, "User"),
            Role::Assistant => write!(f, "Assistant"),
            Role::System => write!(f, "System"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct RequestMessage {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Default, Serialize)]
pub struct OpenAIRequest {
    pub model: String,
    pub messages: Vec<RequestMessage>,
    pub stream: bool,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ResponseMessage {
    pub role: Option<Role>,
    pub content: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct OpenAIUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Deserialize, Debug)]
pub struct ChatChoiceDelta {
    pub index: u32,
    pub delta: ResponseMessage,
    pub finish_reason: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct OpenAIResponseStreamEvent {
    pub id: Option<String>,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<ChatChoiceDelta>,
    pub usage: Option<OpenAIUsage>,
}

pub async fn stream_completion(
    api_key: String,
    executor: Arc<Background>,
    mut request: OpenAIRequest,
) -> Result<impl Stream<Item = Result<OpenAIResponseStreamEvent>>> {
    request.stream = true;

    let (tx, rx) = futures::channel::mpsc::unbounded::<Result<OpenAIResponseStreamEvent>>();

    let json_data = serde_json::to_string(&request)?;
    let mut response = Request::post(format!("{OPENAI_API_URL}/chat/completions"))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .body(json_data)?
        .send_async()
        .await?;

    let status = response.status();
    if status == StatusCode::OK {
        executor
            .spawn(async move {
                let mut lines = BufReader::new(response.body_mut()).lines();

                fn parse_line(
                    line: Result<String, io::Error>,
                ) -> Result<Option<OpenAIResponseStreamEvent>> {
                    if let Some(data) = line?.strip_prefix("data: ") {
                        let event = serde_json::from_str(&data)?;
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
        struct OpenAIResponse {
            error: OpenAIError,
        }

        #[derive(Deserialize)]
        struct OpenAIError {
            message: String,
        }

        match serde_json::from_str::<OpenAIResponse>(&body) {
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

pub trait CompletionProvider {
    fn complete(
        &self,
        prompt: OpenAIRequest,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>>;
}

pub struct OpenAICompletionProvider {
    api_key: String,
    executor: Arc<Background>,
}

impl OpenAICompletionProvider {
    pub fn new(api_key: String, executor: Arc<Background>) -> Self {
        Self { api_key, executor }
    }
}

impl CompletionProvider for OpenAICompletionProvider {
    fn complete(
        &self,
        prompt: OpenAIRequest,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        let request = stream_completion(self.api_key.clone(), self.executor.clone(), prompt);
        async move {
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
