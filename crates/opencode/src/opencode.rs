//! OpenCode API types and streaming clients.
//!
//! This crate describes dynamically discovered OpenCode models and sends requests using
//! each model's advertised protocol and capabilities.

use anyhow::{Result, anyhow};
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::{
    AsyncBody, CustomHeaders, HttpClient, Method, Request as HttpRequest, RequestBuilderExt,
};
use language_model_core::ReasoningEffort;
use serde::{Deserialize, Serialize};

pub const OPENCODE_API_URL: &str = "https://opencode.ai/zen";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum ApiProtocol {
    #[default]
    Anthropic,
    OpenAiResponses,
    OpenAiChat,
    Google,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum OpenCodeSubscription {
    Zen,
    Go,
}

impl OpenCodeSubscription {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Zen => "Zen",
            Self::Go => "Go",
        }
    }

    pub fn id_prefix(&self) -> &'static str {
        match self {
            Self::Zen => "zen",
            Self::Go => "go",
        }
    }

    pub fn api_path_suffix(&self) -> &'static str {
        match self {
            Self::Zen => "",
            Self::Go => "/go",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Model {
    name: String,
    display_name: Option<String>,
    max_tokens: u64,
    max_output_tokens: Option<u64>,
    protocol: ApiProtocol,
    reasoning_effort_levels: Option<Vec<ReasoningEffort>>,
    custom_model_api_url: Option<String>,
    interleaved_reasoning: bool,
}

impl Model {
    pub fn new(
        name: String,
        display_name: Option<String>,
        max_tokens: u64,
        max_output_tokens: Option<u64>,
        protocol: ApiProtocol,
        reasoning_effort_levels: Option<Vec<ReasoningEffort>>,
        custom_model_api_url: Option<String>,
        interleaved_reasoning: bool,
    ) -> Self {
        Self {
            name,
            display_name,
            max_tokens,
            max_output_tokens,
            protocol,
            reasoning_effort_levels,
            custom_model_api_url,
            interleaved_reasoning,
        }
    }

    pub fn id(&self) -> &str {
        &self.name
    }

    pub fn display_name(&self) -> &str {
        self.display_name.as_deref().unwrap_or(&self.name)
    }

    pub fn protocol(&self) -> ApiProtocol {
        self.protocol
    }

    pub fn interleaved_reasoning(&self) -> bool {
        self.interleaved_reasoning
    }

    pub fn max_token_count(&self) -> u64 {
        self.max_tokens
    }

    pub fn max_output_tokens(&self) -> Option<u64> {
        self.max_output_tokens
    }

    pub fn supports_tools(&self) -> bool {
        true
    }

    pub fn supported_reasoning_effort_levels(&self) -> Option<&[ReasoningEffort]> {
        self.reasoning_effort_levels.as_deref()
    }

    pub fn custom_model_api_url(&self) -> Option<&str> {
        self.custom_model_api_url.as_deref()
    }
}

/// Streams Google generate-content responses through OpenCode.
pub async fn stream_generate_content(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: google_ai::GenerateContentRequest,
    extra_headers: &CustomHeaders,
) -> Result<BoxStream<'static, Result<google_ai::GenerateContentResponse>>> {
    let api_key = api_key.trim();
    let model_id = &request.model.model_id;
    let uri = format!("{api_url}/v1/models/{model_id}:streamGenerateContent?alt=sse");
    let request = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {api_key}"))
        .extra_headers(extra_headers)
        .body(AsyncBody::from(serde_json::to_string(&request)?))?;
    let mut response = client.send(request).await?;
    if response.status().is_success() {
        let reader = BufReader::new(response.into_body());
        Ok(reader
            .lines()
            .filter_map(|line| async move {
                match line {
                    Ok(line) => {
                        if let Some(line) = line.strip_prefix("data: ") {
                            match serde_json::from_str(line) {
                                Ok(response) => Some(Ok(response)),
                                Err(error) => {
                                    Some(Err(anyhow!("Error parsing JSON: {error:?}\n{line:?}")))
                                }
                            }
                        } else {
                            None
                        }
                    }
                    Err(error) => Some(Err(anyhow!(error))),
                }
            })
            .boxed())
    } else {
        let mut text = String::new();
        response.body_mut().read_to_string(&mut text).await?;
        Err(anyhow!(
            "error during streamGenerateContent via OpenCode, status code: {:?}, body: {}",
            response.status(),
            text
        ))
    }
}
