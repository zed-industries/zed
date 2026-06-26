use anyhow::{Context as _, Result, anyhow};
use aws_http_client::sign_request_sigv4;
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::{
    AsyncBody, CustomHeaders, HttpClient, Method, Request as HttpRequest, RequestBuilderExt,
    StatusCode,
    http::{HeaderMap, HeaderValue, header::AUTHORIZATION},
};
pub use language_model_core::ReasoningEffort;
use open_ai::responses as MantleResponses;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{convert::TryFrom, future::Future};
use strum::EnumIter;
use thiserror::Error;

const PROVIDER_NAME: &str = "bedrock-mantle";

/// SigV4 signing name for the Bedrock Mantle service.
const MANTLE_SIGNING_SERVICE: &str = "bedrock-mantle";

/// Authentication for Bedrock Mantle requests.
///
/// Unlike the regular Bedrock path (which goes through the AWS SDK client and
/// lets the SDK handle signing), Mantle requests are issued directly through an
/// [`HttpClient`], so we have to apply auth to each request ourselves. The
/// provider resolves its higher-level auth configuration down to one of these
/// two concrete schemes before calling into this module.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MantleAuth {
    /// Bearer token (Bedrock API key) auth.
    ApiKey { api_key: String },
    /// AWS SigV4 request signing with concrete IAM credentials. The provider is
    /// responsible for resolving profile/SSO/automatic credential chains into
    /// these concrete values before reaching this module.
    SigV4 {
        access_key_id: String,
        secret_access_key: String,
        session_token: Option<String>,
    },
}

impl MantleAuth {
    /// Applies this auth scheme to an already-built request, mutating it in place.
    /// For SigV4 the full request body bytes are required to compute the payload hash.
    fn apply(
        &self,
        request: &mut HttpRequest<AsyncBody>,
        body: &[u8],
        region: &str,
    ) -> Result<(), RequestError> {
        match self {
            MantleAuth::ApiKey { api_key } => {
                let value = HeaderValue::from_str(&format!("Bearer {}", api_key.trim()))
                    .map_err(|error| RequestError::Other(error.into()))?;
                request.headers_mut().insert(AUTHORIZATION, value);
            }
            MantleAuth::SigV4 {
                access_key_id,
                secret_access_key,
                session_token,
            } => {
                sign_request_sigv4(
                    request,
                    body,
                    access_key_id,
                    secret_access_key,
                    session_token.as_deref(),
                    region,
                    MANTLE_SIGNING_SERVICE,
                )
                .map_err(RequestError::Other)?;
            }
        }
        Ok(())
    }
}

fn is_none_or_empty<T: AsRef<[U]>, U>(opt: &Option<T>) -> bool {
    opt.as_ref().is_none_or(|v| v.as_ref().is_empty())
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
    Tool,
}

impl TryFrom<String> for Role {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self> {
        match value.as_str() {
            "user" => Ok(Self::User),
            "assistant" => Ok(Self::Assistant),
            "system" => Ok(Self::System),
            "tool" => Ok(Self::Tool),
            _ => anyhow::bail!("invalid role '{value}'"),
        }
    }
}

impl From<Role> for String {
    fn from(val: Role) -> Self {
        match val {
            Role::User => "user".to_owned(),
            Role::Assistant => "assistant".to_owned(),
            Role::System => "system".to_owned(),
            Role::Tool => "tool".to_owned(),
        }
    }
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum MantleModel {
    #[serde(rename = "gpt-5.4")]
    Gpt5_4,
    #[serde(rename = "gpt-5.5")]
    #[default]
    Gpt5_5,
    #[serde(rename = "gemma-4-31b")]
    Gemma4_31B,
    #[serde(rename = "gemma-4-26b")]
    Gemma4_26B,
    #[serde(rename = "gemma-4-e2b")]
    Gemma4E2b,
    #[serde(rename = "grok-4-3")]
    Grok4_3,
    #[serde(rename = "custom")]
    Custom {
        name: String,
        /// The name displayed in the UI, such as in the agent panel model dropdown menu.
        display_name: Option<String>,
        max_tokens: u64,
        max_output_tokens: Option<u64>,
        max_completion_tokens: Option<u64>,
        reasoning_effort: Option<ReasoningEffort>,
        #[serde(default = "default_supports_chat_completions")]
        supports_chat_completions: bool,
        #[serde(default = "default_supports_images")]
        supports_images: bool,
    },
}

const fn default_supports_chat_completions() -> bool {
    true
}

const fn default_supports_images() -> bool {
    true
}

impl MantleModel {
    pub fn default_fast() -> Self {
        Self::Gemma4E2b
    }

    pub fn from_id(id: &str) -> Result<Self> {
        match id {
            "gpt-5.4" => Ok(Self::Gpt5_4),
            "gpt-5.5" => Ok(Self::Gpt5_5),
            "google.gemma-4-31b" => Ok(Self::Gemma4_31B),
            "google.gemma-4-26b-a4b" => Ok(Self::Gemma4_26B),
            "google.gemma-4-e2b" => Ok(Self::Gemma4E2b),
            "xai.grok-4.3" => Ok(Self::Grok4_3),
            invalid_id => anyhow::bail!("invalid model id '{invalid_id}'"),
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Self::Gpt5_4 => "gpt-5.4",
            Self::Gpt5_5 => "gpt-5.5",
            // Gemma and Grok are served on the `bedrock-mantle` endpoint and
            // expect their fully-qualified Bedrock model IDs as `request.model`.
            Self::Gemma4_31B => "google.gemma-4-31b",
            Self::Gemma4_26B => "google.gemma-4-26b-a4b",
            Self::Gemma4E2b => "google.gemma-4-e2b",
            Self::Grok4_3 => "xai.grok-4.3",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Gpt5_4 => "gpt-5.4",
            Self::Gpt5_5 => "gpt-5.5",
            Self::Gemma4_31B => "Gemma 4 31B",
            Self::Gemma4_26B => "Gemma 4 26B-A4B",
            Self::Gemma4E2b => "Gemma 4 E2B",
            Self::Grok4_3 => "Grok 4.3",
            Self::Custom { display_name, .. } => display_name.as_deref().unwrap_or(self.id()),
        }
    }

    pub fn max_token_count(&self) -> u64 {
        match self {
            Self::Gpt5_4 => 1_050_000,
            Self::Gpt5_5 => 1_050_000,
            // Context windows verified against the `bedrock-mantle` endpoint:
            // it caps `max_completion_tokens` at exactly the context window.
            Self::Grok4_3 => 1_048_576,
            Self::Gemma4_31B | Self::Gemma4_26B => 262_144,
            Self::Gemma4E2b => 131_072,
            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self) -> Option<u64> {
        match self {
            Self::Custom {
                max_output_tokens, ..
            } => *max_output_tokens,
            Self::Gpt5_4 => Some(128_000),
            Self::Gpt5_5 => Some(128_000),
            // Empirically probed against `bedrock-mantle`: the endpoint accepts
            // `max_completion_tokens` up to the full context window and rejects
            // anything larger ("exceeds model maximum (N)"). Grok's documented
            // 131072 is only the *default*, not the ceiling.
            Self::Grok4_3 => Some(1_048_576),
            Self::Gemma4_31B | Self::Gemma4_26B => Some(262_144),
            Self::Gemma4E2b => Some(131_072),
        }
    }

    pub fn reasoning_effort(&self) -> Option<ReasoningEffort> {
        match self {
            Self::Custom {
                reasoning_effort, ..
            } => reasoning_effort.to_owned(),
            Self::Gpt5_4 => Some(ReasoningEffort::None),
            Self::Gpt5_5 => Some(ReasoningEffort::Medium),
            // Grok 4.3 reasoning is always on and defaults to low effort.
            Self::Grok4_3 => Some(ReasoningEffort::Low),
            Self::Gemma4_31B | Self::Gemma4_26B => Some(ReasoningEffort::Medium),
            // Gemma 4 E2B is recommended to run with high reasoning effort so
            // that its extensive reasoning stays in the dedicated channel.
            Self::Gemma4E2b => Some(ReasoningEffort::High),
        }
    }

    pub fn supported_reasoning_efforts(&self) -> &'static [ReasoningEffort] {
        match self {
            Self::Custom {
                reasoning_effort: Some(effort),
                ..
            } => match effort {
                ReasoningEffort::None => &[ReasoningEffort::None],
                ReasoningEffort::Minimal => &[ReasoningEffort::Minimal],
                ReasoningEffort::Low => &[ReasoningEffort::Low],
                ReasoningEffort::Medium => &[ReasoningEffort::Medium],
                ReasoningEffort::High => &[ReasoningEffort::High],
                ReasoningEffort::XHigh => &[ReasoningEffort::XHigh],
                ReasoningEffort::Max => &[ReasoningEffort::Max],
            },
            Self::Custom {
                reasoning_effort: None,
                ..
            } => &[],
            Self::Gpt5_4 | Self::Gpt5_5 => &[
                ReasoningEffort::None,
                ReasoningEffort::Low,
                ReasoningEffort::Medium,
                ReasoningEffort::High,
                ReasoningEffort::XHigh,
            ],
            // Grok 4.3 supports disabling reasoning ("none") plus low/medium/high.
            Self::Grok4_3 => &[
                ReasoningEffort::None,
                ReasoningEffort::Low,
                ReasoningEffort::Medium,
                ReasoningEffort::High,
            ],
            // Gemma 4 models honor reasoning effort but do not document a
            // "none" option, so we only expose low/medium/high.
            Self::Gemma4_31B | Self::Gemma4_26B | Self::Gemma4E2b => &[
                ReasoningEffort::Low,
                ReasoningEffort::Medium,
                ReasoningEffort::High,
            ],
        }
    }

    pub fn uses_responses_api(&self) -> bool {
        match self {
            Self::Custom {
                supports_chat_completions,
                ..
            } => !*supports_chat_completions,
            _ => true,
        }
    }

    /// Returns whether the given model supports the `parallel_tool_calls` parameter.
    ///
    /// If the model does not support the parameter, do not pass it up, or the API will return an error.
    pub fn supports_parallel_tool_calls(&self) -> bool {
        match self {
            Self::Gpt5_4 | Self::Gpt5_5 => true,
            // Grok 4.3 does not document a parallel tool call restriction.
            Self::Grok4_3 => true,
            // Gemma 4 models explicitly only support one tool call per turn.
            Self::Gemma4_31B | Self::Gemma4_26B | Self::Gemma4E2b => false,
            Self::Custom { .. } => false,
        }
    }

    /// Returns whether the given model supports the `prompt_cache_key` parameter.
    ///
    /// If the model does not support the parameter, do not pass it up.
    pub fn supports_prompt_cache_key(&self) -> bool {
        true
    }

    /// Whether this model supports server-side compaction via the
    /// `context_management` request parameter. OpenAI doesn't publish a
    /// support matrix, but the GPT-5.5 guide notes compaction is a feature
    /// shared with GPT-5.4, and the compaction docs exercise the GPT-5.3
    /// Codex line, so we treat everything from GPT-5.3 onward as supported.
    ///
    /// <https://developers.openai.com/api/docs/guides/compaction>
    pub fn supports_compaction(&self) -> bool {
        match self {
            Self::Gpt5_4 | Self::Gpt5_5 => true,
            Self::Gemma4_31B | Self::Gemma4_26B | Self::Gemma4E2b | Self::Grok4_3 => false,
            Self::Custom { .. } => false,
        }
    }

    /// Whether OpenAI's Priority processing tier is available for this model.
    /// Sourced from <https://openai.com/api-priority-processing/>. The `*-pro`,
    /// `*-nano`, and legacy `gpt-4` variants are not eligible.
    pub fn supports_priority(&self) -> bool {
        match self {
            Self::Gpt5_4 | Self::Gpt5_5 => true,
            // Gemma 4 and Grok 4.3 are all available on the Priority tier.
            Self::Gemma4_31B | Self::Gemma4_26B | Self::Gemma4E2b | Self::Grok4_3 => true,
            Self::Custom { .. } => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MantleModel, ReasoningEffort};

    #[test]
    fn newer_frontier_models_support_none_reasoning() {
        let expected_efforts = [
            ReasoningEffort::None,
            ReasoningEffort::Low,
            ReasoningEffort::Medium,
            ReasoningEffort::High,
            ReasoningEffort::XHigh,
        ];

        assert_eq!(
            MantleModel::Gpt5_4.reasoning_effort(),
            Some(ReasoningEffort::None)
        );
        assert_eq!(
            MantleModel::Gpt5_4.supported_reasoning_efforts(),
            expected_efforts.as_slice()
        );
        assert_eq!(
            MantleModel::Gpt5_5.reasoning_effort(),
            Some(ReasoningEffort::Medium)
        );
        assert_eq!(
            MantleModel::Gpt5_5.supported_reasoning_efforts(),
            expected_efforts.as_slice()
        );
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamOptions {
    pub include_usage: bool,
}

impl Default for StreamOptions {
    fn default() -> Self {
        Self {
            include_usage: true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub model: String,
    pub messages: Vec<RequestMessage>,
    pub stream: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stop: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    /// Whether to enable parallel function calling during tool use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<ToolDefinition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<ReasoningEffort>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ServiceTier>,
}

/// Service tier for OpenAI requests. Maps to the top-level `service_tier`
/// field on Responses and Chat Completions. We only ever send `Priority`
/// today (in response to Fast Mode being enabled); the other variants are
/// included for symmetry with the API and so deserialization of echoed
/// values does not fail.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ServiceTier {
    Auto,
    Default,
    Flex,
    Scale,
    Priority,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoice {
    Auto,
    Required,
    None,
    #[serde(untagged)]
    Other(ToolDefinition),
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolDefinition {
    #[allow(dead_code)]
    Function { function: FunctionDefinition },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Option<Value>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum RequestMessage {
    Assistant {
        content: Option<MessageContent>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        tool_calls: Vec<ToolCall>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        reasoning_content: Option<String>,
    },
    User {
        content: MessageContent,
    },
    System {
        content: MessageContent,
    },
    Tool {
        content: MessageContent,
        tool_call_id: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(untagged)]
pub enum MessageContent {
    Plain(String),
    Multipart(Vec<MessagePart>),
}

impl MessageContent {
    pub fn empty() -> Self {
        MessageContent::Multipart(vec![])
    }

    pub fn push_part(&mut self, part: MessagePart) {
        match self {
            MessageContent::Plain(text) => {
                *self =
                    MessageContent::Multipart(vec![MessagePart::Text { text: text.clone() }, part]);
            }
            MessageContent::Multipart(parts) if parts.is_empty() => match part {
                MessagePart::Text { text } => *self = MessageContent::Plain(text),
                MessagePart::Image { .. } => *self = MessageContent::Multipart(vec![part]),
            },
            MessageContent::Multipart(parts) => parts.push(part),
        }
    }
}

impl From<Vec<MessagePart>> for MessageContent {
    fn from(mut parts: Vec<MessagePart>) -> Self {
        if let [MessagePart::Text { text }] = parts.as_mut_slice() {
            MessageContent::Plain(std::mem::take(text))
        } else {
            MessageContent::Multipart(parts)
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum MessagePart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    Image { image_url: ImageUrl },
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ImageUrl {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ToolCall {
    pub id: String,
    #[serde(flatten)]
    pub content: ToolCallContent,
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ToolCallContent {
    Function { function: FunctionContent },
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct FunctionContent {
    pub name: String,
    pub arguments: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Choice {
    pub index: u32,
    pub message: RequestMessage,
    pub finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ResponseMessageDelta {
    pub role: Option<Role>,
    pub content: Option<String>,
    pub reasoning: Option<String>,
    #[serde(default, skip_serializing_if = "is_none_or_empty")]
    pub tool_calls: Option<Vec<ToolCallChunk>>,
    #[serde(default, skip_serializing_if = "is_none_or_empty")]
    pub reasoning_content: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ToolCallChunk {
    pub index: usize,
    pub id: Option<String>,

    // There is also an optional `type` field that would determine if a
    // function is there. Sometimes this streams in with the `function` before
    // it streams in the `type`
    pub function: Option<FunctionChunk>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct FunctionChunk {
    pub name: Option<String>,
    pub arguments: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: Option<u64>,
    pub completion_tokens: Option<u64>,
    pub total_tokens: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChoiceDelta {
    pub index: u32,
    pub delta: Option<ResponseMessageDelta>,
    pub finish_reason: Option<String>,
}

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("HTTP response error from {provider}'s API: status {status_code} - {body:?}")]
    HttpResponseError {
        provider: String,
        status_code: StatusCode,
        body: String,
        headers: HeaderMap<HeaderValue>,
    },
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseStreamError {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ResponseStreamResult {
    Ok(ResponseStreamEvent),
    Err { error: ResponseStreamError },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseStreamEvent {
    pub choices: Vec<ChoiceDelta>,
    pub usage: Option<Usage>,
}

/// Builds the chat-completions URL for a Mantle model.
///
/// Bedrock-hosted third-party models (Gemma, Grok — whose IDs are namespaced
/// like `google.gemma-4-31b` or `xai.grok-4.3`) are served on the OpenAI-shaped
/// `/openai/v1` path, whereas the first-party GPT ids use the bare `/v1` path.
/// See the per-model "Programmatic Access" sections of the Bedrock model cards.
fn chat_completions_uri(region: &str, model: &str) -> String {
    let region_url = format!("https://bedrock-mantle.{region}.api.aws");
    if model.contains('.') {
        format!("{region_url}/openai/v1/chat/completions")
    } else {
        format!("{region_url}/v1/chat/completions")
    }
}

pub async fn non_streaming_completion(
    client: &dyn HttpClient,
    region: &str,
    auth: &MantleAuth,
    request: Request,
) -> Result<Response, RequestError> {
    let uri = chat_completions_uri(region, &request.model);
    let body = serde_json::to_vec(&request).map_err(|e| RequestError::Other(e.into()))?;

    let mut http_request = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .body(AsyncBody::from(body.clone()))
        .map_err(|e| RequestError::Other(e.into()))?;

    auth.apply(&mut http_request, &body, region)?;

    let mut response = client.send(http_request).await?;
    if response.status().is_success() {
        let mut body = String::new();
        response
            .body_mut()
            .read_to_string(&mut body)
            .await
            .map_err(|e| RequestError::Other(e.into()))?;

        serde_json::from_str(&body).map_err(|e| RequestError::Other(e.into()))
    } else {
        let mut body = String::new();
        response
            .body_mut()
            .read_to_string(&mut body)
            .await
            .map_err(|e| RequestError::Other(e.into()))?;

        Err(RequestError::HttpResponseError {
            provider: PROVIDER_NAME.to_owned(),
            status_code: response.status(),
            body,
            headers: response.headers().clone(),
        })
    }
}

pub async fn stream_completion(
    client: &dyn HttpClient,
    region: &str,
    auth: &MantleAuth,
    request: Request,
    extra_headers: &CustomHeaders,
) -> Result<BoxStream<'static, Result<ResponseStreamEvent>>, RequestError> {
    let uri = chat_completions_uri(region, &request.model);
    let body = serde_json::to_vec(&request).map_err(|e| RequestError::Other(e.into()))?;

    let mut http_request = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .extra_headers(extra_headers)
        .body(AsyncBody::from(body.clone()))
        .map_err(|e| RequestError::Other(e.into()))?;

    auth.apply(&mut http_request, &body, region)?;

    let mut response = client.send(http_request).await?;
    if response.status().is_success() {
        let reader = BufReader::new(response.into_body());
        Ok(reader
            .lines()
            .filter_map(|line| async move {
                match line {
                    Ok(line) => {
                        let line = line.strip_prefix("data: ").or_else(|| line.strip_prefix("data:"))?;
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
        response
            .body_mut()
            .read_to_string(&mut body)
            .await
            .map_err(|e| RequestError::Other(e.into()))?;

        Err(RequestError::HttpResponseError {
            provider: PROVIDER_NAME.to_owned(),
            status_code: response.status(),
            body,
            headers: response.headers().clone(),
        })
    }
}

// -- Conversions to `language_model_core` types --

impl From<RequestError> for language_model_core::LanguageModelCompletionError {
    fn from(error: RequestError) -> Self {
        match error {
            RequestError::HttpResponseError {
                provider,
                status_code,
                body,
                headers,
            } => {
                let retry_after = headers
                    .get(http_client::http::header::RETRY_AFTER)
                    .and_then(|val| val.to_str().ok()?.parse::<u64>().ok())
                    .map(std::time::Duration::from_secs);

                Self::from_http_status(provider.into(), status_code, body, retry_after)
            }
            RequestError::Other(e) => Self::Other(e),
        }
    }
}
