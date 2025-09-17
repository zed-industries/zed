pub mod oauth;

use anyhow::{Result, anyhow};
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use log;
use open_ai::ReasoningEffort;
use open_ai::{OpenAiError, Request, ResponseStreamEvent, ResponseStreamResult};
use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Model {
    #[default]
    #[serde(rename = "gpt-4.1")]
    FourPointOne,
    #[serde(rename = "openai-o3")]
    O3,
    #[serde(rename = "gpt-5")]
    Five,
    #[serde(rename = "grok3")]
    Grok3,
    #[serde(rename = "grok4")]
    Grok4,
    #[serde(rename = "grok-code-fast-1")]
    GrokCodeFast1,
    #[serde(rename = "llama4")]
    Llama4,
    #[serde(rename = "custom")]
    Custom {
        name: String,
        /// The name displayed in the UI, such as in the assistant panel model dropdown menu.
        display_name: Option<String>,
        max_tokens: u64,
        max_output_tokens: Option<u64>,
        max_completion_tokens: Option<u64>,
        reasoning_effort: Option<ReasoningEffort>,
    },
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum ModelVendor {
    OpenAI,
    XAi,
    Meta,
}

impl Model {
    pub fn default_fast() -> Self {
        Self::FourPointOne
    }

    pub fn from_id(id: &str) -> Result<Self> {
        match id {
            "oca/gpt-4.1" => Ok(Self::FourPointOne),
            "oca/openai-o3" => Ok(Self::O3),
            "oca/grok3" => Ok(Self::Grok3),
            "oca/grok4" => Ok(Self::Grok4),
            "oca/llama4" => Ok(Self::Llama4),
            invalid_id => anyhow::bail!("invalid model id '{invalid_id}'"),
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Self::FourPointOne => "oca/gpt-4.1",
            Self::O3 => "oca/openai-o3",
            Self::Five => "oca/gpt5",
            Self::Grok3 => "oca/grok3",
            Self::Grok4 => "oca/grok4",
            Self::GrokCodeFast1 => "oca/grok-code-fast-1",
            Self::Llama4 => "oca/llama4",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::FourPointOne => "OpenAI GPT-4.1",
            Self::O3 => "OpenAI O3",
            Self::Five => "OpenAI GPT 5",
            Self::Grok3 => "Grok 3",
            Self::Grok4 => "Grok 4",
            Self::GrokCodeFast1 => "Grok Code Fast 1",
            Self::Llama4 => "Llama 4",
            Self::Custom {
                name, display_name, ..
            } => display_name.as_ref().unwrap_or(name),
        }
    }

    pub fn model_vendor(&self) -> ModelVendor {
        match self {
            Self::FourPointOne | Self::O3 | Self::Five => ModelVendor::OpenAI,
            Self::Grok3 | Self::Grok4 | Self::GrokCodeFast1 => ModelVendor::XAi,
            Self::Llama4 => ModelVendor::Meta,
            // Assume custom models are OpenAI compatible
            Self::Custom { .. } => ModelVendor::OpenAI,
        }
    }

    pub fn max_token_count(&self) -> u64 {
        match self {
            Self::FourPointOne => 1_047_576,
            Self::O3 => 200_000,
            Self::Five => 272_000,
            Self::Grok3 => 131_072,
            Self::Grok4 | Self::GrokCodeFast1 => 256_000,
            Self::Llama4 => 128_000,
            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self) -> Option<u64> {
        match self {
            Self::Custom {
                max_output_tokens, ..
            } => *max_output_tokens,
            Self::FourPointOne => Some(32_768),
            Self::O3 => Some(100_000),
            Self::Five => Some(128_000),
            Self::Grok3 => Some(8_192),
            Self::Grok4 | Self::GrokCodeFast1 => Some(64_000),
            Self::Llama4 => None,
        }
    }

    pub fn reasoning_effort(&self) -> Option<ReasoningEffort> {
        match self {
            Self::Custom {
                reasoning_effort, ..
            } => reasoning_effort.to_owned(),
            _ => None,
        }
    }

    /// Returns whether the given model supports the `parallel_tool_calls` parameter.
    ///
    /// If the model does not support the parameter, do not pass it up, or the API will return an error.
    pub fn supports_parallel_tool_calls(&self) -> bool {
        match self {
            Self::FourPointOne | Self::Grok3 | Self::Grok4 | Self::Llama4 | Self::Five => true,
            Self::GrokCodeFast1 | Self::O3 | Model::Custom { .. } => false,
        }
    }

    /// Returns whether the given model supports the `prompt_cache_key` parameter.
    ///
    /// If the model does not support the parameter, do not pass it up.
    pub fn supports_prompt_cache_key(&self) -> bool {
        match self {
            Self::FourPointOne | Self::O3 | Self::Five => true,
            Self::GrokCodeFast1
            | Self::Grok3
            | Self::Grok4
            | Self::Llama4
            | Model::Custom { .. } => false,
        }
    }
}

pub async fn stream_completion(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: Request,
) -> Result<BoxStream<'static, Result<ResponseStreamEvent>>> {
    let uri = format!("{api_url}/chat/completions");
    let client_name = "Zed";
    let client_version = format!(
        "{}/{}",
        client_name,
        option_env!("CARGO_PKG_VERSION").unwrap_or("unknown")
    );

    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("client", client_name)
        .header("client-version", client_version.as_str())
        .header("client-ide", client_name)
        .header("client-ide-version", client_version)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key));

    let request = request_builder.body(AsyncBody::from(serde_json::to_string(&request)?))?;
    let mut response = client.send(request).await?;
    if response.status().is_success() {
        let reader = BufReader::new(response.into_body());
        Ok(reader
            .lines()
            .filter_map(|line| async move {
                match line {
                    Ok(line) => {
                        let line = line.strip_prefix("data: ")?;
                        if line == "[DONE]" {
                            None
                        } else {
                            match serde_json::from_str(line) {
                                Ok(ResponseStreamResult::Ok(response)) => Some(Ok(response)),
                                Ok(ResponseStreamResult::Err { error }) => {
                                    Some(Err(anyhow!(error.message)))
                                }
                                Err(error) => {
                                    log::error!(
                                        "Failed to parse OpenAI response into ResponseStreamResult: `{}`\n\
                                        Response: `{}`",
                                        error,
                                        line,
                                    );
                                    Some(Err(anyhow!(error)))
                                }
                            }
                        }
                    }
                    Err(error) => Some(Err(anyhow!(error))),
                }
            })
            .boxed())
    } else {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;

        #[derive(Deserialize)]
        struct OpenAiResponse {
            error: OpenAiError,
        }

        match serde_json::from_str::<OpenAiResponse>(&body) {
            Ok(response) if !response.error.message.is_empty() => Err(anyhow!(
                "API request to {} failed: {}",
                api_url,
                response.error.message,
            )),

            _ => anyhow::bail!(
                "API request to {} failed with status {}: {}",
                api_url,
                response.status(),
                body,
            ),
        }
    }
}
