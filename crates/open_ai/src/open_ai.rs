pub mod batches;
pub mod completion;
pub mod responses;

use anyhow::{Context as _, Result, anyhow};
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::{
    AsyncBody, CustomHeaders, HttpClient, Method, Request as HttpRequest, RequestBuilderExt,
    StatusCode,
    http::{HeaderMap, HeaderValue},
};
pub use language_model_core::ReasoningEffort;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{convert::TryFrom, future::Future};
use strum::EnumIter;
use thiserror::Error;

pub const OPEN_AI_API_URL: &str = "https://api.openai.com/v1";

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
pub enum Model {
    #[serde(rename = "gpt-4")]
    Four,
    #[serde(rename = "gpt-4o-mini")]
    FourOmniMini,
    #[serde(rename = "o3")]
    O3,
    #[serde(rename = "gpt-5")]
    Five,
    #[serde(rename = "gpt-5-mini")]
    FiveMini,
    #[serde(rename = "gpt-5-nano")]
    FiveNano,
    #[serde(rename = "gpt-5.1")]
    FivePointOne,
    #[serde(rename = "gpt-5.2")]
    FivePointTwo,
    #[serde(rename = "gpt-5.3-codex")]
    FivePointThreeCodex,
    #[serde(rename = "gpt-5.4-nano")]
    FivePointFourNano,
    #[serde(rename = "gpt-5.4-mini")]
    FivePointFourMini,
    #[serde(rename = "gpt-5.4")]
    FivePointFour,
    #[serde(rename = "gpt-5.4-pro")]
    FivePointFourPro,
    #[serde(rename = "gpt-5.5")]
    FivePointFive,
    #[serde(rename = "gpt-5.5-pro")]
    FivePointFivePro,
    #[serde(rename = "gpt-5.6-sol")]
    #[default]
    FivePointSixSol,
    #[serde(rename = "gpt-5.6-terra")]
    FivePointSixTerra,
    #[serde(rename = "gpt-5.6-luna")]
    FivePointSixLuna,
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

impl Model {
    pub fn default_fast() -> Self {
        Self::FivePointSixLuna
    }

    pub fn from_id(id: &str) -> Result<Self> {
        match id {
            "gpt-4" => Ok(Self::Four),
            "gpt-4o-mini" => Ok(Self::FourOmniMini),
            "o3" => Ok(Self::O3),
            "gpt-5" => Ok(Self::Five),
            "gpt-5-mini" => Ok(Self::FiveMini),
            "gpt-5-nano" => Ok(Self::FiveNano),
            "gpt-5.1" => Ok(Self::FivePointOne),
            "gpt-5.2" => Ok(Self::FivePointTwo),
            "gpt-5.3-codex" => Ok(Self::FivePointThreeCodex),
            "gpt-5.4-nano" => Ok(Self::FivePointFourNano),
            "gpt-5.4-mini" => Ok(Self::FivePointFourMini),
            "gpt-5.4" => Ok(Self::FivePointFour),
            "gpt-5.4-pro" => Ok(Self::FivePointFourPro),
            "gpt-5.5" => Ok(Self::FivePointFive),
            "gpt-5.5-pro" => Ok(Self::FivePointFivePro),
            "gpt-5.6-sol" => Ok(Self::FivePointSixSol),
            "gpt-5.6-terra" => Ok(Self::FivePointSixTerra),
            "gpt-5.6-luna" => Ok(Self::FivePointSixLuna),
            invalid_id => anyhow::bail!("invalid model id '{invalid_id}'"),
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Self::Four => "gpt-4",
            Self::FourOmniMini => "gpt-4o-mini",
            Self::O3 => "o3",
            Self::Five => "gpt-5",
            Self::FiveMini => "gpt-5-mini",
            Self::FiveNano => "gpt-5-nano",
            Self::FivePointOne => "gpt-5.1",
            Self::FivePointTwo => "gpt-5.2",
            Self::FivePointThreeCodex => "gpt-5.3-codex",
            Self::FivePointFourNano => "gpt-5.4-nano",
            Self::FivePointFourMini => "gpt-5.4-mini",
            Self::FivePointFour => "gpt-5.4",
            Self::FivePointFourPro => "gpt-5.4-pro",
            Self::FivePointFive => "gpt-5.5",
            Self::FivePointFivePro => "gpt-5.5-pro",
            Self::FivePointSixSol => "gpt-5.6-sol",
            Self::FivePointSixTerra => "gpt-5.6-terra",
            Self::FivePointSixLuna => "gpt-5.6-luna",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Four => "gpt-4",
            Self::FourOmniMini => "gpt-4o-mini",
            Self::O3 => "o3",
            Self::Five => "gpt-5",
            Self::FiveMini => "gpt-5-mini",
            Self::FiveNano => "gpt-5-nano",
            Self::FivePointOne => "gpt-5.1",
            Self::FivePointTwo => "gpt-5.2",
            Self::FivePointThreeCodex => "gpt-5.3-codex",
            Self::FivePointFourNano => "gpt-5.4-nano",
            Self::FivePointFourMini => "gpt-5.4-mini",
            Self::FivePointFour => "gpt-5.4",
            Self::FivePointFourPro => "gpt-5.4-pro",
            Self::FivePointFive => "gpt-5.5",
            Self::FivePointFivePro => "gpt-5.5-pro",
            Self::FivePointSixSol => "gpt-5.6-sol",
            Self::FivePointSixTerra => "gpt-5.6-terra",
            Self::FivePointSixLuna => "gpt-5.6-luna",
            Self::Custom { display_name, .. } => display_name.as_deref().unwrap_or(&self.id()),
        }
    }

    pub fn max_token_count(&self) -> u64 {
        match self {
            Self::Four => 8_192,
            Self::FourOmniMini => 128_000,
            Self::O3 => 200_000,
            Self::Five => 272_000,
            Self::FiveMini => 400_000,
            Self::FiveNano => 400_000,
            Self::FivePointOne => 400_000,
            Self::FivePointTwo => 400_000,
            Self::FivePointThreeCodex => 400_000,
            Self::FivePointFourNano => 400_000,
            Self::FivePointFourMini => 400_000,
            Self::FivePointFour => 1_050_000,
            Self::FivePointFourPro => 1_050_000,
            Self::FivePointFive => 1_050_000,
            Self::FivePointFivePro => 1_050_000,
            Self::FivePointSixSol => 1_050_000,
            Self::FivePointSixTerra => 1_050_000,
            Self::FivePointSixLuna => 1_050_000,
            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self) -> Option<u64> {
        match self {
            Self::Custom {
                max_output_tokens, ..
            } => *max_output_tokens,
            Self::Four => Some(8_192),
            Self::FourOmniMini => Some(16_384),
            Self::O3 => Some(100_000),
            Self::Five => Some(128_000),
            Self::FiveMini => Some(128_000),
            Self::FiveNano => Some(128_000),
            Self::FivePointOne => Some(128_000),
            Self::FivePointTwo => Some(128_000),
            Self::FivePointThreeCodex => Some(128_000),
            Self::FivePointFourNano => Some(128_000),
            Self::FivePointFourMini => Some(128_000),
            Self::FivePointFour => Some(128_000),
            Self::FivePointFourPro => Some(128_000),
            Self::FivePointFive => Some(128_000),
            Self::FivePointFivePro => Some(128_000),
            Self::FivePointSixSol => Some(128_000),
            Self::FivePointSixTerra => Some(128_000),
            Self::FivePointSixLuna => Some(128_000),
        }
    }

    pub fn reasoning_effort(&self) -> Option<ReasoningEffort> {
        match self {
            Self::Custom {
                reasoning_effort, ..
            } => reasoning_effort.to_owned(),
            Self::FivePointOne
            | Self::FivePointTwo
            | Self::FivePointFour
            | Self::FivePointFourMini
            | Self::FivePointFourNano => Some(ReasoningEffort::None),
            Self::FivePointSixSol => Some(ReasoningEffort::Low),
            Self::O3
            | Self::Five
            | Self::FiveMini
            | Self::FiveNano
            | Self::FivePointThreeCodex
            | Self::FivePointFourPro
            | Self::FivePointFive
            | Self::FivePointFivePro
            | Self::FivePointSixTerra
            | Self::FivePointSixLuna => Some(ReasoningEffort::Medium),
            _ => None,
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
            Self::O3 => &[
                ReasoningEffort::Low,
                ReasoningEffort::Medium,
                ReasoningEffort::High,
            ],
            Self::FivePointOne => &[
                ReasoningEffort::None,
                ReasoningEffort::Low,
                ReasoningEffort::Medium,
                ReasoningEffort::High,
            ],
            Self::Five | Self::FiveMini | Self::FiveNano => &[
                ReasoningEffort::Minimal,
                ReasoningEffort::Low,
                ReasoningEffort::Medium,
                ReasoningEffort::High,
            ],
            Self::FivePointFourPro | Self::FivePointFivePro => &[
                ReasoningEffort::Medium,
                ReasoningEffort::High,
                ReasoningEffort::XHigh,
            ],
            Self::FivePointThreeCodex => &[
                ReasoningEffort::Low,
                ReasoningEffort::Medium,
                ReasoningEffort::High,
                ReasoningEffort::XHigh,
            ],
            Self::FivePointSixSol | Self::FivePointSixTerra | Self::FivePointSixLuna => &[
                ReasoningEffort::None,
                ReasoningEffort::Low,
                ReasoningEffort::Medium,
                ReasoningEffort::High,
                ReasoningEffort::XHigh,
                ReasoningEffort::Max,
            ],
            Self::FivePointTwo
            | Self::FivePointFour
            | Self::FivePointFive
            | Self::FivePointFourMini
            | Self::FivePointFourNano => &[
                ReasoningEffort::None,
                ReasoningEffort::Low,
                ReasoningEffort::Medium,
                ReasoningEffort::High,
                ReasoningEffort::XHigh,
            ],
            _ => &[],
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
            Self::Four
            | Self::FourOmniMini
            | Self::Five
            | Self::FiveMini
            | Self::FivePointOne
            | Self::FivePointTwo
            | Self::FivePointThreeCodex
            | Self::FivePointFour
            | Self::FivePointFourMini
            | Self::FivePointFourNano
            | Self::FivePointFourPro
            | Self::FivePointFive
            | Self::FivePointFivePro
            | Self::FivePointSixSol
            | Self::FivePointSixTerra
            | Self::FivePointSixLuna
            | Self::FiveNano => true,
            Self::O3 | Model::Custom { .. } => false,
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
            Self::FivePointThreeCodex
            | Self::FivePointFourNano
            | Self::FivePointFourMini
            | Self::FivePointFour
            | Self::FivePointFourPro
            | Self::FivePointFive
            | Self::FivePointFivePro
            | Self::FivePointSixSol
            | Self::FivePointSixTerra
            | Self::FivePointSixLuna => true,
            Self::Four
            | Self::FourOmniMini
            | Self::O3
            | Self::Five
            | Self::FiveMini
            | Self::FiveNano
            | Self::FivePointOne
            | Self::FivePointTwo
            | Self::Custom { .. } => false,
        }
    }

    /// Whether OpenAI's Priority processing tier is available for this model.
    /// Sourced from <https://openai.com/api-priority-processing/>. The `*-pro`,
    /// `*-nano`, and legacy `gpt-4` variants are not eligible.
    pub fn supports_priority(&self) -> bool {
        match self {
            Self::FourOmniMini
            | Self::O3
            | Self::Five
            | Self::FiveMini
            | Self::FivePointOne
            | Self::FivePointTwo
            | Self::FivePointThreeCodex
            | Self::FivePointFourMini
            | Self::FivePointFour
            | Self::FivePointFive
            | Self::FivePointSixSol
            | Self::FivePointSixTerra
            | Self::FivePointSixLuna => true,
            Self::Four
            | Self::FiveNano
            | Self::FivePointFourNano
            | Self::FivePointFourPro
            | Self::FivePointFivePro
            | Self::Custom { .. } => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Model, ReasoningEffort};

    #[test]
    fn gpt_5_1_uses_none_reasoning_by_default() {
        let expected_efforts = [
            ReasoningEffort::None,
            ReasoningEffort::Low,
            ReasoningEffort::Medium,
            ReasoningEffort::High,
        ];

        assert_eq!(
            Model::FivePointOne.reasoning_effort(),
            Some(ReasoningEffort::None)
        );
        assert_eq!(
            Model::FivePointOne.supported_reasoning_efforts(),
            expected_efforts.as_slice()
        );
    }

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
            Model::FivePointTwo.reasoning_effort(),
            Some(ReasoningEffort::None)
        );
        assert_eq!(
            Model::FivePointTwo.supported_reasoning_efforts(),
            expected_efforts.as_slice()
        );
        assert_eq!(
            Model::FivePointFour.reasoning_effort(),
            Some(ReasoningEffort::None)
        );
        assert_eq!(
            Model::FivePointFour.supported_reasoning_efforts(),
            expected_efforts.as_slice()
        );
        assert_eq!(
            Model::FivePointFive.reasoning_effort(),
            Some(ReasoningEffort::Medium)
        );
        assert_eq!(
            Model::FivePointFive.supported_reasoning_efforts(),
            expected_efforts.as_slice()
        );
    }

    #[test]
    fn newer_codex_models_support_low_reasoning_effort() {
        let expected_efforts = [
            ReasoningEffort::Low,
            ReasoningEffort::Medium,
            ReasoningEffort::High,
            ReasoningEffort::XHigh,
        ];

        assert_eq!(
            Model::FivePointThreeCodex.supported_reasoning_efforts(),
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

pub async fn non_streaming_completion(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: Request,
) -> Result<Response, RequestError> {
    let uri = format!("{api_url}/chat/completions");
    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key.trim()));

    let request = request_builder
        .body(AsyncBody::from(
            serde_json::to_string(&request).map_err(|e| RequestError::Other(e.into()))?,
        ))
        .map_err(|e| RequestError::Other(e.into()))?;

    let mut response = client.send(request).await?;
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
            provider: "openai".to_owned(),
            status_code: response.status(),
            body,
            headers: response.headers().clone(),
        })
    }
}

pub async fn stream_completion(
    client: &dyn HttpClient,
    provider_name: &str,
    api_url: &str,
    api_key: &str,
    request: Request,
    extra_headers: &CustomHeaders,
) -> Result<BoxStream<'static, Result<ResponseStreamEvent>>, RequestError> {
    let uri = format!("{api_url}/chat/completions");
    let request = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key.trim()))
        .extra_headers(extra_headers)
        .body(AsyncBody::from(
            serde_json::to_string(&request).map_err(|e| RequestError::Other(e.into()))?,
        ))
        .map_err(|e| RequestError::Other(e.into()))?;

    let mut response = client.send(request).await?;
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
            provider: provider_name.to_owned(),
            status_code: response.status(),
            body,
            headers: response.headers().clone(),
        })
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum OpenAiEmbeddingModel {
    #[serde(rename = "text-embedding-3-small")]
    TextEmbedding3Small,
    #[serde(rename = "text-embedding-3-large")]
    TextEmbedding3Large,
}

#[derive(Serialize)]
struct OpenAiEmbeddingRequest<'a> {
    model: OpenAiEmbeddingModel,
    input: Vec<&'a str>,
}

#[derive(Deserialize)]
pub struct OpenAiEmbeddingResponse {
    pub data: Vec<OpenAiEmbedding>,
}

#[derive(Deserialize)]
pub struct OpenAiEmbedding {
    pub embedding: Vec<f32>,
}

pub fn embed<'a>(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    model: OpenAiEmbeddingModel,
    texts: impl IntoIterator<Item = &'a str>,
) -> impl 'static + Future<Output = Result<OpenAiEmbeddingResponse>> {
    let uri = format!("{api_url}/embeddings");

    let request = OpenAiEmbeddingRequest {
        model,
        input: texts.into_iter().collect(),
    };
    let body = AsyncBody::from(serde_json::to_string(&request).unwrap());
    let request = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key.trim()))
        .body(body)
        .map(|request| client.send(request));

    async move {
        let mut response = request?.await?;
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;

        anyhow::ensure!(
            response.status().is_success(),
            "error during embedding, status: {:?}, body: {:?}",
            response.status(),
            body
        );
        let response: OpenAiEmbeddingResponse =
            serde_json::from_str(&body).context("failed to parse OpenAI embedding response")?;
        Ok(response)
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
