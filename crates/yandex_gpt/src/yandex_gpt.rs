use anyhow::{anyhow, Result};
use futures::{io::BufReader, stream::BoxStream, AsyncBufReadExt, AsyncReadExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use util::http::{AsyncBody, HttpClient, Method, Request as HttpRequest};

pub const YANDEX_GPT_API_URL: &str = "https://llm.api.cloud.yandex.net/foundationModels/v1";

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
pub enum Model {
    #[serde(rename = "yandexgpt", alias = "yandex-gpt-pro")]
    #[default]
    YandexGptPro,
    #[serde(rename = "yandexgpt-lite", alias = "yandex-gpt-lite")]
    YandexGptLite,
}

impl Model {
    pub fn from_id(id: &str) -> Result<Self> {
        match id {
            "yandexgpt" => Ok(Self::YandexGptPro),
            "yandexgpt-lite" => Ok(Self::YandexGptLite),
            _ => Err(anyhow!("invalid model id")),
        }
    }

    pub fn id(&self) -> &'static str {
        match self {
            Self::YandexGptPro => "yandexgpt",
            Self::YandexGptLite => "yandexgpt-lite",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::YandexGptPro => "yandex-gpt-pro",
            Self::YandexGptLite => "yandex-gpt-lite",
        }
    }

    pub fn max_token_count(&self) -> usize {
        match self {
            Model::YandexGptLite => 8000,
            Model::YandexGptPro => 8000,
        }
    }

    pub fn get_uri(&self, folder_id: String) -> String {
        format!(
            "gpt://{folder_id}/{}/latest",
            match self {
                Self::YandexGptPro => "yandexgpt",
                Self::YandexGptLite => "yandexgpt-lite",
            }
        )
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub model_uri: String,
    pub messages: Vec<RequestMessage>,
    pub completion_options: CompletionOptions,
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionOptions {
    pub stream: bool,
    pub temperature: f32,
    pub max_tokens: usize,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct RequestMessage {
    pub role: Role,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ResponseMessage {
    pub role: Option<Role>,
    pub text: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    pub input_text_tokens: String,
    pub completion_tokens: String,
    pub total_tokens: String,
}

#[derive(Deserialize, Debug)]
pub struct Alternative {
    pub message: ResponseMessage,
    pub status: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseStreamEvent {
    pub model_version: String,
    pub alternatives: Vec<Alternative>,
    pub usage: Option<Usage>,
}

#[derive(Deserialize, Debug)]
pub struct ResponseResult {
    pub result: ResponseStreamEvent,
}

pub async fn stream_completion(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: Request,
) -> Result<BoxStream<'static, Result<ResponseStreamEvent>>> {
    let uri = format!("{api_url}/completion");
    let request = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Api-Key {}", api_key))
        .body(AsyncBody::from(serde_json::to_string(&request)?))?;

    let mut response = client.send(request).await?;
    if response.status().is_success() {
        let reader = BufReader::new(response.into_body());

        Ok(reader
            .lines()
            .filter_map(|line| async move {
                match line {
                    Ok(line) => match serde_json::from_str::<ResponseResult>(line.as_str()) {
                        Ok(response) => {
                            if response
                                .result
                                .alternatives
                                .iter()
                                .all(|a| a.status == "ALTERNATIVE_STATUS_FINAL")
                            {
                                Some(Ok(response.result))
                            } else {
                                None
                            }
                        }
                        Err(error) => Some(Err(anyhow!(error))),
                    },
                    Err(error) => Some(Err(anyhow!(error))),
                }
            })
            .boxed())
    } else {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;

        #[derive(Deserialize)]
        struct YandexGptResponse {
            error: YandexGptError,
        }

        #[derive(Deserialize)]
        struct YandexGptError {
            message: String,
        }

        match serde_json::from_str::<YandexGptResponse>(&body) {
            Ok(response) if !response.error.message.is_empty() => Err(anyhow!(
                "Failed to connect to YandexGPT API: {}",
                response.error.message,
            )),

            _ => Err(anyhow!(
                "Failed to connect to YandexGPT API: {} {}",
                response.status(),
                body,
            )),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct YdxToken {
    pub id: String,
    pub text: String,
    pub special: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct YandexGptTokenizerResponse {
    pub tokens: Vec<YdxToken>,
    pub model_version: String,
}

pub async fn tokenize_completion(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: Request,
) -> Result<YandexGptTokenizerResponse> {
    let uri = format!("{api_url}/tokenizeCompletion");
    let request = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Api-Key {}", api_key))
        .body(AsyncBody::from(serde_json::to_string(&request)?))?;
    let mut response = client.send(request).await?;
    if response.status().is_success() {
        let mut buf = String::default();
        let mut body = response.into_body();
        let reader = body.read_to_string(&mut buf);

        match reader.await {
            Ok(_) => match serde_json::from_str::<YandexGptTokenizerResponse>(buf.as_str()) {
                Ok(response) => Ok(response),
                Err(error) => Err(anyhow!(error)),
            },
            Err(error) => Err(anyhow!(error)),
        }
    } else {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;

        #[derive(Deserialize)]
        struct YandexGptResponse {
            error: YandexGptError,
        }

        #[derive(Deserialize)]
        struct YandexGptError {
            message: String,
        }

        match serde_json::from_str::<YandexGptResponse>(&body) {
            Ok(response) if !response.error.message.is_empty() => Err(anyhow!(
                "Failed to connect to YandexGPT API: {}",
                response.error.message,
            )),

            _ => Err(anyhow!(
                "Failed to connect to YandexGPT API: {} {}",
                response.status(),
                body,
            )),
        }
    }
}

// TODO: impl embeddings
// #[derive(Copy, Clone, Serialize, Deserialize)]
// pub enum YandexGptEmbeddingModel {
//     #[serde(rename = "text-search-query")]
//     TextSearchQuery,
//     #[serde(rename = "text-search-doc")]
//     TextSearchDoc,
// }

// #[derive(Serialize)]
// struct YandexGptEmbeddingRequest<'a> {
//     model: YandexGptEmbeddingModel,
//     text: &'a str,
// }

// #[derive(Deserialize)]
// pub struct YandexGptEmbedding {
//     pub embedding: Vec<f32>,
//     pub num_tokens: String,
//     pub model_version: String,
// }

// pub fn embed<'a>(
//     client: &dyn HttpClient,
//     api_url: &str,
//     api_key: &str,
//     model: YandexGptEmbeddingModel,
//     texts: impl IntoIterator<Item = &'a str>,
// ) -> impl 'static + Future<Output = Result<YandexGptEmbedding>> {
//     let uri = format!("{api_url}/textEmbedding");

//     let request = YandexGptEmbeddingRequest {
//         model,
//         text: texts.into_iter().collect(),
//     };
//     let body = AsyncBody::from(serde_json::to_string(&request).unwrap());
//     let request = HttpRequest::builder()
//         .method(Method::POST)
//         .uri(uri)
//         .header("Content-Type", "application/json")
//         .header("Authorization", format!("Api-Key {}", api_key))
//         .body(body)
//         .map(|request| client.send(request));

//     async move {
//         let mut response = request?.await?;
//         let mut body = String::new();
//         response.body_mut().read_to_string(&mut body).await?;

//         if response.status().is_success() {
//             let response: YandexGptEmbedding = serde_json::from_str(&body)
//                 .context("failed to parse YandexGPT embedding response")?;
//             Ok(response)
//         } else {
//             Err(anyhow!(
//                 "error during embedding, status: {:?}, body: {:?}",
//                 response.status(),
//                 body
//             ))
//         }
//     }
// }
