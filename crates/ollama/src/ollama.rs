use anyhow::{anyhow, Context, Result};
use futures::{io::BufReader, stream::BoxStream, AsyncBufReadExt, AsyncReadExt, StreamExt};
use http::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use isahc::config::Configurable;
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, time::Duration};

pub const OLLAMA_API_URL: &str = "http://localhost:11434";

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
}

impl TryFrom<String> for Role {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self> {
        match value.as_str() {
            "user" => Ok(Self::User),
            "assistant" => Ok(Self::Assistant),
            "system" => Ok(Self::System),
            _ => Err(anyhow!("invalid role '{value}'")),
        }
    }
}

impl From<Role> for String {
    fn from(val: Role) -> Self {
        match val {
            Role::User => "user".to_owned(),
            Role::Assistant => "assistant".to_owned(),
            Role::System => "system".to_owned(),
        }
    }
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Model {
    pub name: String,
    pub parameter_size: String,
    pub max_tokens: usize,
    pub keep_alive: Option<String>,
}

impl Model {
    pub fn new(name: &str, parameter_size: &str) -> Self {
        Self {
            name: name.to_owned(),
            parameter_size: parameter_size.to_owned(),
            // todo: determine if there's an endpoint to find the max tokens
            //       I'm not seeing it in the API docs but it's on the model cards
            max_tokens: 2048,
            keep_alive: Some("10m".to_owned()),
        }
    }

    pub fn id(&self) -> &str {
        &self.name
    }

    pub fn display_name(&self) -> &str {
        &self.name
    }

    pub fn max_token_count(&self) -> usize {
        self.max_tokens
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum ChatMessage {
    Assistant { content: String },
    User { content: String },
    System { content: String },
}

#[derive(Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub stream: bool,
    pub keep_alive: Option<String>,
    pub options: Option<ChatOptions>,
}

// https://github.com/ollama/ollama/blob/main/docs/modelfile.md#valid-parameters-and-values
#[derive(Serialize, Default)]
pub struct ChatOptions {
    pub num_ctx: Option<usize>,
    pub num_predict: Option<isize>,
    pub stop: Option<Vec<String>>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
}

#[derive(Deserialize)]
pub struct ChatResponseDelta {
    #[allow(unused)]
    pub model: String,
    #[allow(unused)]
    pub created_at: String,
    pub message: ChatMessage,
    #[allow(unused)]
    pub done_reason: Option<String>,
    #[allow(unused)]
    pub done: bool,
}

#[derive(Serialize, Deserialize)]
pub struct LocalModelsResponse {
    pub models: Vec<LocalModelListing>,
}

#[derive(Serialize, Deserialize)]
pub struct LocalModelListing {
    pub name: String,
    pub modified_at: String,
    pub size: u64,
    pub digest: String,
    pub details: ModelDetails,
}

#[derive(Serialize, Deserialize)]
pub struct LocalModel {
    pub modelfile: String,
    pub parameters: String,
    pub template: String,
    pub details: ModelDetails,
}

#[derive(Serialize, Deserialize)]
pub struct ModelDetails {
    pub format: String,
    pub family: String,
    pub families: Option<Vec<String>>,
    pub parameter_size: String,
    pub quantization_level: String,
}

pub async fn stream_chat_completion(
    client: &dyn HttpClient,
    api_url: &str,
    request: ChatRequest,
    low_speed_timeout: Option<Duration>,
) -> Result<BoxStream<'static, Result<ChatResponseDelta>>> {
    let uri = format!("{api_url}/api/chat");
    let mut request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json");

    if let Some(low_speed_timeout) = low_speed_timeout {
        request_builder = request_builder.low_speed_timeout(100, low_speed_timeout);
    };

    let request = request_builder.body(AsyncBody::from(serde_json::to_string(&request)?))?;
    let mut response = client.send(request).await?;
    if response.status().is_success() {
        let reader = BufReader::new(response.into_body());

        Ok(reader
            .lines()
            .filter_map(|line| async move {
                match line {
                    Ok(line) => {
                        Some(serde_json::from_str(&line).context("Unable to parse chat response"))
                    }
                    Err(e) => Some(Err(e.into())),
                }
            })
            .boxed())
    } else {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;

        Err(anyhow!(
            "Failed to connect to Ollama API: {} {}",
            response.status(),
            body,
        ))
    }
}

pub async fn get_models(
    client: &dyn HttpClient,
    api_url: &str,
    low_speed_timeout: Option<Duration>,
) -> Result<Vec<LocalModelListing>> {
    let uri = format!("{api_url}/api/tags");
    let mut request_builder = HttpRequest::builder()
        .method(Method::GET)
        .uri(uri)
        .header("Accept", "application/json");

    if let Some(low_speed_timeout) = low_speed_timeout {
        request_builder = request_builder.low_speed_timeout(100, low_speed_timeout);
    };

    let request = request_builder.body(AsyncBody::default())?;

    let mut response = client.send(request).await?;

    let mut body = String::new();
    response.body_mut().read_to_string(&mut body).await?;

    if response.status().is_success() {
        let response: LocalModelsResponse =
            serde_json::from_str(&body).context("Unable to parse Ollama tag listing")?;

        Ok(response.models)
    } else {
        Err(anyhow!(
            "Failed to connect to Ollama API: {} {}",
            response.status(),
            body,
        ))
    }
}
