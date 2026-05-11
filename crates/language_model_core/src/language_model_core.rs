mod provider;
mod rate_limiter;
mod request;
mod role;
pub mod tool_schema;
pub mod util;

use anyhow::{Result, anyhow};
use cloud_llm_client::CompletionRequestStatus;
use http_client::{StatusCode, http};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use std::{fmt, io};
use thiserror::Error;
fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}

pub use crate::provider::*;
pub use crate::rate_limiter::*;
pub use crate::request::*;
pub use crate::role::*;
pub use crate::tool_schema::LanguageModelToolSchemaFormat;
pub use crate::util::{fix_streamed_json, parse_prompt_too_long, parse_tool_arguments};
pub use gpui_shared_string::SharedString;

#[derive(Clone, Debug)]
pub struct LanguageModelCacheConfiguration {
    pub max_cache_anchors: usize,
    pub should_speculate: bool,
    pub min_total_token: u64,
}

/// A completion event from a language model.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum LanguageModelCompletionEvent {
    Queued {
        position: usize,
    },
    Started,
    Stop(StopReason),
    Text(String),
    Thinking {
        text: String,
        signature: Option<String>,
    },
    RedactedThinking {
        data: String,
    },
    ToolUse(LanguageModelToolUse),
    ToolUseJsonParseError {
        id: LanguageModelToolUseId,
        tool_name: Arc<str>,
        raw_input: Arc<str>,
        json_parse_error: String,
    },
    StartMessage {
        message_id: String,
    },
    ReasoningDetails(serde_json::Value),
    UsageUpdate(TokenUsage),
}

impl LanguageModelCompletionEvent {
    pub fn from_completion_request_status(
        status: CompletionRequestStatus,
        upstream_provider: LanguageModelProviderName,
    ) -> Result<Option<Self>, LanguageModelCompletionError> {
        match status {
            CompletionRequestStatus::Queued { position } => {
                Ok(Some(LanguageModelCompletionEvent::Queued { position }))
            }
            CompletionRequestStatus::Started => Ok(Some(LanguageModelCompletionEvent::Started)),
            CompletionRequestStatus::Unknown | CompletionRequestStatus::StreamEnded => Ok(None),
            CompletionRequestStatus::Failed {
                code,
                message,
                request_id: _,
                retry_after,
            } => Err(LanguageModelCompletionError::from_cloud_failure(
                upstream_provider,
                code,
                message,
                retry_after.map(Duration::from_secs_f64),
            )),
        }
    }
}

#[derive(Error, Debug)]
pub enum LanguageModelCompletionError {
    #[error("prompt too large for context window")]
    PromptTooLarge { tokens: Option<u64> },
    #[error("missing {provider} API key")]
    NoApiKey { provider: LanguageModelProviderName },
    #[error("{provider}'s API rate limit exceeded")]
    RateLimitExceeded {
        provider: LanguageModelProviderName,
        retry_after: Option<Duration>,
    },
    #[error("{provider}'s API servers are overloaded right now")]
    ServerOverloaded {
        provider: LanguageModelProviderName,
        retry_after: Option<Duration>,
    },
    #[error("{provider}'s API server reported an internal server error: {message}")]
    ApiInternalServerError {
        provider: LanguageModelProviderName,
        message: String,
    },
    #[error("{message}")]
    UpstreamProviderError {
        message: String,
        status: StatusCode,
        retry_after: Option<Duration>,
    },
    #[error("HTTP response error from {provider}'s API: status {status_code} - {message:?}")]
    HttpResponseError {
        provider: LanguageModelProviderName,
        status_code: StatusCode,
        message: String,
    },
    #[error("invalid request format to {provider}'s API: {message}")]
    BadRequestFormat {
        provider: LanguageModelProviderName,
        message: String,
    },
    #[error("authentication error with {provider}'s API: {message}")]
    AuthenticationError {
        provider: LanguageModelProviderName,
        message: String,
    },
    #[error("Permission error with {provider}'s API: {message}")]
    PermissionError {
        provider: LanguageModelProviderName,
        message: String,
    },
    #[error("language model provider API endpoint not found")]
    ApiEndpointNotFound { provider: LanguageModelProviderName },
    #[error("I/O error reading response from {provider}'s API")]
    ApiReadResponseError {
        provider: LanguageModelProviderName,
        #[source]
        error: io::Error,
    },
    #[error("error serializing request to {provider} API")]
    SerializeRequest {
        provider: LanguageModelProviderName,
        #[source]
        error: serde_json::Error,
    },
    #[error("error building request body to {provider} API")]
    BuildRequestBody {
        provider: LanguageModelProviderName,
        #[source]
        error: http::Error,
    },
    #[error("error sending HTTP request to {provider} API")]
    HttpSend {
        provider: LanguageModelProviderName,
        #[source]
        error: anyhow::Error,
    },
    #[error("error deserializing {provider} API response")]
    DeserializeResponse {
        provider: LanguageModelProviderName,
        #[source]
        error: serde_json::Error,
    },
    #[error("stream from {provider} ended unexpectedly")]
    StreamEndedUnexpectedly { provider: LanguageModelProviderName },
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl LanguageModelCompletionError {
    fn parse_upstream_error_json(message: &str) -> Option<(StatusCode, String)> {
        let error_json = serde_json::from_str::<serde_json::Value>(message).ok()?;
        let upstream_status = error_json
            .get("upstream_status")
            .and_then(|v| v.as_u64())
            .and_then(|status| u16::try_from(status).ok())
            .and_then(|status| StatusCode::from_u16(status).ok())?;
        let inner_message = error_json
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or(message)
            .to_string();
        Some((upstream_status, inner_message))
    }

    pub fn from_cloud_failure(
        upstream_provider: LanguageModelProviderName,
        code: String,
        message: String,
        retry_after: Option<Duration>,
    ) -> Self {
        if let Some(tokens) = parse_prompt_too_long(&message) {
            Self::PromptTooLarge {
                tokens: Some(tokens),
            }
        } else if code == "upstream_http_error" {
            if let Some((upstream_status, inner_message)) =
                Self::parse_upstream_error_json(&message)
            {
                return Self::from_http_status(
                    upstream_provider,
                    upstream_status,
                    inner_message,
                    retry_after,
                );
            }
            anyhow!("completion request failed, code: {code}, message: {message}").into()
        } else if let Some(status_code) = code
            .strip_prefix("upstream_http_")
            .and_then(|code| StatusCode::from_str(code).ok())
        {
            Self::from_http_status(upstream_provider, status_code, message, retry_after)
        } else if let Some(status_code) = code
            .strip_prefix("http_")
            .and_then(|code| StatusCode::from_str(code).ok())
        {
            Self::from_http_status(ZED_CLOUD_PROVIDER_NAME, status_code, message, retry_after)
        } else {
            anyhow!("completion request failed, code: {code}, message: {message}").into()
        }
    }

    pub fn from_http_status(
        provider: LanguageModelProviderName,
        status_code: StatusCode,
        message: String,
        retry_after: Option<Duration>,
    ) -> Self {
        match status_code {
            StatusCode::BAD_REQUEST => Self::BadRequestFormat { provider, message },
            StatusCode::UNAUTHORIZED => Self::AuthenticationError { provider, message },
            StatusCode::FORBIDDEN => Self::PermissionError { provider, message },
            StatusCode::NOT_FOUND => Self::ApiEndpointNotFound { provider },
            StatusCode::PAYLOAD_TOO_LARGE => Self::PromptTooLarge {
                tokens: parse_prompt_too_long(&message),
            },
            StatusCode::TOO_MANY_REQUESTS => Self::RateLimitExceeded {
                provider,
                retry_after,
            },
            StatusCode::INTERNAL_SERVER_ERROR => Self::ApiInternalServerError { provider, message },
            StatusCode::SERVICE_UNAVAILABLE => Self::ServerOverloaded {
                provider,
                retry_after,
            },
            _ if status_code.as_u16() == 529 => Self::ServerOverloaded {
                provider,
                retry_after,
            },
            _ => Self::HttpResponseError {
                provider,
                status_code,
                message,
            },
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StopReason {
    EndTurn,
    MaxTokens,
    ToolUse,
    Refusal,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Default)]
pub struct TokenUsage {
    #[serde(default, skip_serializing_if = "is_default")]
    pub input_tokens: u64,
    #[serde(default, skip_serializing_if = "is_default")]
    pub output_tokens: u64,
    #[serde(default, skip_serializing_if = "is_default")]
    pub cache_creation_input_tokens: u64,
    #[serde(default, skip_serializing_if = "is_default")]
    pub cache_read_input_tokens: u64,
}

impl TokenUsage {
    pub fn total_tokens(&self) -> u64 {
        self.input_tokens
            + self.output_tokens
            + self.cache_read_input_tokens
            + self.cache_creation_input_tokens
    }
}

impl Add<TokenUsage> for TokenUsage {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            input_tokens: self.input_tokens + other.input_tokens,
            output_tokens: self.output_tokens + other.output_tokens,
            cache_creation_input_tokens: self.cache_creation_input_tokens
                + other.cache_creation_input_tokens,
            cache_read_input_tokens: self.cache_read_input_tokens + other.cache_read_input_tokens,
        }
    }
}

impl Sub<TokenUsage> for TokenUsage {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            input_tokens: self.input_tokens - other.input_tokens,
            output_tokens: self.output_tokens - other.output_tokens,
            cache_creation_input_tokens: self.cache_creation_input_tokens
                - other.cache_creation_input_tokens,
            cache_read_input_tokens: self.cache_read_input_tokens - other.cache_read_input_tokens,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct LanguageModelToolUseId(Arc<str>);

impl fmt::Display for LanguageModelToolUseId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> From<T> for LanguageModelToolUseId
where
    T: Into<Arc<str>>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct LanguageModelToolUse {
    pub id: LanguageModelToolUseId,
    pub name: Arc<str>,
    pub raw_input: String,
    pub input: serde_json::Value,
    pub is_input_complete: bool,
    /// Thought signature the model sent us. Some models require that this
    /// signature be preserved and sent back in conversation history for validation.
    pub thought_signature: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LanguageModelEffortLevel {
    pub name: SharedString,
    pub value: SharedString,
    pub is_default: bool,
}

/// An error that occurred when trying to authenticate the language model provider.
#[derive(Debug, Error)]
pub enum AuthenticateError {
    #[error("connection refused")]
    ConnectionRefused,
    #[error("credentials not found")]
    CredentialsNotFound,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd, Serialize, Deserialize)]
pub struct LanguageModelId(pub SharedString);

#[derive(Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
pub struct LanguageModelName(pub SharedString);

#[derive(Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
pub struct LanguageModelProviderId(pub SharedString);

#[derive(Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
pub struct LanguageModelProviderName(pub SharedString);

impl LanguageModelProviderId {
    pub const fn new(id: &'static str) -> Self {
        Self(SharedString::new_static(id))
    }
}

impl LanguageModelProviderName {
    pub const fn new(id: &'static str) -> Self {
        Self(SharedString::new_static(id))
    }
}

impl fmt::Display for LanguageModelProviderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for LanguageModelProviderName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for LanguageModelId {
    fn from(value: String) -> Self {
        Self(SharedString::from(value))
    }
}

impl From<String> for LanguageModelName {
    fn from(value: String) -> Self {
        Self(SharedString::from(value))
    }
}

impl From<String> for LanguageModelProviderId {
    fn from(value: String) -> Self {
        Self(SharedString::from(value))
    }
}

impl From<String> for LanguageModelProviderName {
    fn from(value: String) -> Self {
        Self(SharedString::from(value))
    }
}

impl From<Arc<str>> for LanguageModelProviderId {
    fn from(value: Arc<str>) -> Self {
        Self(SharedString::from(value))
    }
}

impl From<Arc<str>> for LanguageModelProviderName {
    fn from(value: Arc<str>) -> Self {
        Self(SharedString::from(value))
    }
}

/// Settings-layer–free model mode enum.
///
/// Mirrors the shape of `settings_content::ModelMode` but lives here so that
/// crates below the settings layer can reference it.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ModelMode {
    #[default]
    Default,
    Thinking {
        budget_tokens: Option<u32>,
    },
}

/// Settings-layer–free reasoning-effort enum.
///
/// Mirrors the shape of `settings_content::OpenAiReasoningEffort` but lives
/// here so that crates below the settings layer can reference it.
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, strum::EnumString,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum ReasoningEffort {
    Minimal,
    Low,
    Medium,
    High,
    XHigh,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cloud_failure_with_upstream_http_error() {
        let error = LanguageModelCompletionError::from_cloud_failure(
            String::from("anthropic").into(),
            "upstream_http_error".to_string(),
            r#"{"code":"upstream_http_error","message":"Received an error from the Anthropic API: upstream connect error or disconnect/reset before headers. reset reason: connection timeout","upstream_status":503}"#.to_string(),
            None,
        );

        match error {
            LanguageModelCompletionError::ServerOverloaded { provider, .. } => {
                assert_eq!(provider.0, "anthropic");
            }
            _ => panic!(
                "Expected ServerOverloaded error for 503 status, got: {:?}",
                error
            ),
        }

        let error = LanguageModelCompletionError::from_cloud_failure(
            String::from("anthropic").into(),
            "upstream_http_error".to_string(),
            r#"{"code":"upstream_http_error","message":"Internal server error","upstream_status":500}"#.to_string(),
            None,
        );

        match error {
            LanguageModelCompletionError::ApiInternalServerError { provider, message } => {
                assert_eq!(provider.0, "anthropic");
                assert_eq!(message, "Internal server error");
            }
            _ => panic!(
                "Expected ApiInternalServerError for 500 status, got: {:?}",
                error
            ),
        }
    }

    #[test]
    fn test_from_cloud_failure_with_standard_format() {
        let error = LanguageModelCompletionError::from_cloud_failure(
            String::from("anthropic").into(),
            "upstream_http_503".to_string(),
            "Service unavailable".to_string(),
            None,
        );

        match error {
            LanguageModelCompletionError::ServerOverloaded { provider, .. } => {
                assert_eq!(provider.0, "anthropic");
            }
            _ => panic!("Expected ServerOverloaded error for upstream_http_503"),
        }
    }

    #[test]
    fn test_upstream_http_error_connection_timeout() {
        let error = LanguageModelCompletionError::from_cloud_failure(
            String::from("anthropic").into(),
            "upstream_http_error".to_string(),
            r#"{"code":"upstream_http_error","message":"Received an error from the Anthropic API: upstream connect error or disconnect/reset before headers. reset reason: connection timeout","upstream_status":503}"#.to_string(),
            None,
        );

        match error {
            LanguageModelCompletionError::ServerOverloaded { provider, .. } => {
                assert_eq!(provider.0, "anthropic");
            }
            _ => panic!(
                "Expected ServerOverloaded error for connection timeout with 503 status, got: {:?}",
                error
            ),
        }

        let error = LanguageModelCompletionError::from_cloud_failure(
            String::from("anthropic").into(),
            "upstream_http_error".to_string(),
            r#"{"code":"upstream_http_error","message":"Received an error from the Anthropic API: upstream connect error or disconnect/reset before headers. reset reason: connection timeout","upstream_status":500}"#.to_string(),
            None,
        );

        match error {
            LanguageModelCompletionError::ApiInternalServerError { provider, message } => {
                assert_eq!(provider.0, "anthropic");
                assert_eq!(
                    message,
                    "Received an error from the Anthropic API: upstream connect error or disconnect/reset before headers. reset reason: connection timeout"
                );
            }
            _ => panic!(
                "Expected ApiInternalServerError for connection timeout with 500 status, got: {:?}",
                error
            ),
        }
    }

    #[test]
    fn test_language_model_tool_use_serializes_with_signature() {
        use serde_json::json;

        let tool_use = LanguageModelToolUse {
            id: LanguageModelToolUseId::from("test_id"),
            name: "test_tool".into(),
            raw_input: json!({"arg": "value"}).to_string(),
            input: json!({"arg": "value"}),
            is_input_complete: true,
            thought_signature: Some("test_signature".to_string()),
        };

        let serialized = serde_json::to_value(&tool_use).unwrap();

        assert_eq!(serialized["id"], "test_id");
        assert_eq!(serialized["name"], "test_tool");
        assert_eq!(serialized["thought_signature"], "test_signature");
    }

    #[test]
    fn test_language_model_tool_use_deserializes_with_missing_signature() {
        use serde_json::json;

        let json = json!({
            "id": "test_id",
            "name": "test_tool",
            "raw_input": "{\"arg\":\"value\"}",
            "input": {"arg": "value"},
            "is_input_complete": true
        });

        let tool_use: LanguageModelToolUse = serde_json::from_value(json).unwrap();

        assert_eq!(tool_use.id, LanguageModelToolUseId::from("test_id"));
        assert_eq!(tool_use.name.as_ref(), "test_tool");
        assert_eq!(tool_use.thought_signature, None);
    }

    #[test]
    fn test_language_model_tool_use_round_trip_with_signature() {
        use serde_json::json;

        let original = LanguageModelToolUse {
            id: LanguageModelToolUseId::from("round_trip_id"),
            name: "round_trip_tool".into(),
            raw_input: json!({"key": "value"}).to_string(),
            input: json!({"key": "value"}),
            is_input_complete: true,
            thought_signature: Some("round_trip_sig".to_string()),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: LanguageModelToolUse = serde_json::from_value(serialized).unwrap();

        assert_eq!(deserialized.id, original.id);
        assert_eq!(deserialized.name, original.name);
        assert_eq!(deserialized.thought_signature, original.thought_signature);
    }

    #[test]
    fn test_language_model_tool_use_round_trip_without_signature() {
        use serde_json::json;

        let original = LanguageModelToolUse {
            id: LanguageModelToolUseId::from("no_sig_id"),
            name: "no_sig_tool".into(),
            raw_input: json!({"arg": "value"}).to_string(),
            input: json!({"arg": "value"}),
            is_input_complete: true,
            thought_signature: None,
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: LanguageModelToolUse = serde_json::from_value(serialized).unwrap();

        assert_eq!(deserialized.id, original.id);
        assert_eq!(deserialized.name, original.name);
        assert_eq!(deserialized.thought_signature, None);
    }
}
