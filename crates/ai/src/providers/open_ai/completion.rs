use anyhow::{anyhow, Result};
use futures::{
    future::BoxFuture, io::BufReader, stream::BoxStream, AsyncBufReadExt, AsyncReadExt, FutureExt,
    Stream, StreamExt,
};
use gpui::{AppContext, BackgroundExecutor};
use isahc::{http::StatusCode, Request, RequestExt};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::{
    env,
    fmt::{self, Display},
    io,
    sync::Arc,
};
use util::ResultExt;

use crate::{
    auth::{CredentialProvider, ProviderCredential},
    completion::{CompletionProvider, CompletionRequest},
    models::LanguageModel,
};

use crate::providers::open_ai::{OpenAILanguageModel, OPENAI_API_URL};

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
    pub stop: Vec<String>,
    pub temperature: f32,
}

impl CompletionRequest for OpenAIRequest {
    fn data(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
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
    credential: ProviderCredential,
    executor: BackgroundExecutor,
    request: Box<dyn CompletionRequest>,
) -> Result<impl Stream<Item = Result<OpenAIResponseStreamEvent>>> {
    let api_key = match credential {
        ProviderCredential::Credentials { api_key } => api_key,
        _ => {
            return Err(anyhow!("no credentials provider for completion"));
        }
    };

    let (tx, rx) = futures::channel::mpsc::unbounded::<Result<OpenAIResponseStreamEvent>>();

    let json_data = request.data()?;
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

#[derive(Clone)]
pub struct OpenAICompletionProvider {
    model: OpenAILanguageModel,
    credential: Arc<RwLock<ProviderCredential>>,
    executor: BackgroundExecutor,
}

impl OpenAICompletionProvider {
    pub async fn new(model_name: String, executor: BackgroundExecutor) -> Self {
        let model = executor
            .spawn(async move { OpenAILanguageModel::load(&model_name) })
            .await;
        let credential = Arc::new(RwLock::new(ProviderCredential::NoCredentials));
        Self {
            model,
            credential,
            executor,
        }
    }
}

impl CredentialProvider for OpenAICompletionProvider {
    fn has_credentials(&self) -> bool {
        match *self.credential.read() {
            ProviderCredential::Credentials { .. } => true,
            _ => false,
        }
    }

    fn retrieve_credentials(&self, cx: &mut AppContext) -> BoxFuture<ProviderCredential> {
        let existing_credential = self.credential.read().clone();
        let retrieved_credential = match existing_credential {
            ProviderCredential::Credentials { .. } => {
                return async move { existing_credential }.boxed()
            }
            _ => {
                if let Some(api_key) = env::var("OPENAI_API_KEY").log_err() {
                    async move { ProviderCredential::Credentials { api_key } }.boxed()
                } else {
                    let credentials = cx.read_credentials(OPENAI_API_URL);
                    async move {
                        if let Some(Some((_, api_key))) = credentials.await.log_err() {
                            if let Some(api_key) = String::from_utf8(api_key).log_err() {
                                ProviderCredential::Credentials { api_key }
                            } else {
                                ProviderCredential::NoCredentials
                            }
                        } else {
                            ProviderCredential::NoCredentials
                        }
                    }
                    .boxed()
                }
            }
        };

        async move {
            let retrieved_credential = retrieved_credential.await;
            *self.credential.write() = retrieved_credential.clone();
            retrieved_credential
        }
        .boxed()
    }

    fn save_credentials(
        &self,
        cx: &mut AppContext,
        credential: ProviderCredential,
    ) -> BoxFuture<()> {
        *self.credential.write() = credential.clone();
        let credential = credential.clone();
        let write_credentials = match credential {
            ProviderCredential::Credentials { api_key } => {
                Some(cx.write_credentials(OPENAI_API_URL, "Bearer", api_key.as_bytes()))
            }
            _ => None,
        };

        async move {
            if let Some(write_credentials) = write_credentials {
                write_credentials.await.log_err();
            }
        }
        .boxed()
    }

    fn delete_credentials(&self, cx: &mut AppContext) -> BoxFuture<()> {
        *self.credential.write() = ProviderCredential::NoCredentials;
        let delete_credentials = cx.delete_credentials(OPENAI_API_URL);
        async move {
            delete_credentials.await.log_err();
        }
        .boxed()
    }
}

impl CompletionProvider for OpenAICompletionProvider {
    fn base_model(&self) -> Box<dyn LanguageModel> {
        let model: Box<dyn LanguageModel> = Box::new(self.model.clone());
        model
    }
    fn complete(
        &self,
        prompt: Box<dyn CompletionRequest>,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        // Currently the CompletionRequest for OpenAI, includes a 'model' parameter
        // This means that the model is determined by the CompletionRequest and not the CompletionProvider,
        // which is currently model based, due to the language model.
        // At some point in the future we should rectify this.
        let credential = self.credential.read().clone();
        let request = stream_completion(credential, self.executor.clone(), prompt);
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
    fn box_clone(&self) -> Box<dyn CompletionProvider> {
        Box::new((*self).clone())
    }
}
