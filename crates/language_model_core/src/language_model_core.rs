mod provider;
mod rate_limiter;
mod request;
mod role;
pub mod tool_schema;
pub mod util;

use anyhow::{Context as _, Result, anyhow};
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
pub use crate::util::{
    fix_streamed_json, is_context_window_exceeded_message, parse_prompt_too_long,
    parse_tool_arguments,
};
pub use gpui_shared_string::SharedString;

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
    Compaction(CompactionUpdate),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum CompactionUpdate {
    /// A streamed response has started producing replacement context.
    Started,
    /// A chunk of a natural-language summary, suitable for incremental display.
    SummaryDelta(Arc<str>),
    /// The complete context to persist and use in subsequent requests.
    Finished(CompactedContext),
    /// The provider abandoned the compaction without producing replacement
    /// context. This is a documented outcome, not a protocol error: the
    /// conversation simply continues on the uncompacted transcript.
    Failed,
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

/// Normalized semantic classification of a provider-originated rejection.
///
/// Each language model provider reports failures with its own wire format
/// (Anthropic's `error.type` strings, OpenRouter's numeric codes, Zed cloud's
/// `upstream_http_*` codes, ...). Callers that need to react to a rejection's
/// meaning — deciding whether to retry, which message to show — shouldn't
/// have to match on every provider's raw vocabulary. Wire-level parsing stays
/// local to each provider crate, but is mapped once into this shared
/// category so the rest of Zed only needs to understand one vocabulary.
///
/// A category does not replace `ProviderRejection`'s `status`, `code`, and
/// `message` fields: those are always preserved verbatim alongside it so a
/// caller can fall back to them for a category that doesn't need special
/// handling (`Other`) or for diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderErrorCategory {
    /// The request exceeded the model's context window, optionally including
    /// the provider-reported token count.
    PromptTooLarge {
        tokens: Option<u64>,
    },
    /// The provider rejected previously-generated opaque content (e.g.
    /// encrypted reasoning state) as invalid or unreadable.
    InvalidEncryptedContent,
    Authentication,
    Permission,
    EndpointNotFound,
    PaymentRequired,
    RateLimit,
    Overloaded,
    InvalidRequest,
    Conflict,
    Timeout,
    InternalServer,
    /// No known semantic applies; callers should fall back to `status`/`code`.
    Other,
}

impl ProviderErrorCategory {
    /// Classifies a rejection from an HTTP status that the provider actually
    /// returned, with message inspection for ambiguous bad-request responses.
    pub fn from_http_status(status: StatusCode, message: &str) -> Self {
        match status {
            StatusCode::BAD_REQUEST if is_invalid_encrypted_content_message(message) => {
                Self::InvalidEncryptedContent
            }
            StatusCode::BAD_REQUEST if is_context_window_exceeded_message(message) => {
                Self::PromptTooLarge { tokens: None }
            }
            StatusCode::UNAUTHORIZED => Self::Authentication,
            StatusCode::FORBIDDEN => Self::Permission,
            StatusCode::NOT_FOUND => Self::EndpointNotFound,
            StatusCode::PAYMENT_REQUIRED => Self::PaymentRequired,
            StatusCode::PAYLOAD_TOO_LARGE => Self::PromptTooLarge {
                tokens: parse_prompt_too_long(message),
            },
            StatusCode::BAD_REQUEST => Self::InvalidRequest,
            StatusCode::CONFLICT => Self::Conflict,
            StatusCode::REQUEST_TIMEOUT | StatusCode::GATEWAY_TIMEOUT => Self::Timeout,
            StatusCode::TOO_MANY_REQUESTS => Self::RateLimit,
            StatusCode::SERVICE_UNAVAILABLE => Self::Overloaded,
            // There is no `StatusCode` variant for the unofficial HTTP 529
            // ("the service is overloaded"), but providers such as
            // Anthropic send it in practice. See https://http.dev/529
            status_code if status_code.as_u16() == 529 => Self::Overloaded,
            status_code if status_code.is_server_error() => Self::InternalServer,
            _ => Self::Other,
        }
    }
}

#[derive(Error, Debug)]
pub enum LanguageModelCompletionError {
    /// The model requires the user to consent to the upstream provider
    /// retaining inference logs (see `LanguageModel::requires_data_retention`)
    /// and that consent has not been given.
    #[error(
        "{model_name} cannot be offered with Zero Data Retention. \
        Anthropic will retain inference logs."
    )]
    DataRetentionConsentRequired { model_name: String },
    #[error("missing {provider} API key")]
    NoApiKey { provider: LanguageModelProviderName },
    /// A rejection reported by the language model provider itself, as
    /// opposed to a transport or plumbing failure on Zed's side.
    ///
    /// `status` and `code` are preserved verbatim from the provider's
    /// response even when `category` already gives them a known meaning, so
    /// callers that need the raw wire details (diagnostics, provider-specific
    /// workarounds) never lose them.
    #[error("{message}")]
    ProviderRejection {
        provider: LanguageModelProviderName,
        status: Option<StatusCode>,
        code: Option<String>,
        message: String,
        retry_after: Option<Duration>,
        category: ProviderErrorCategory,
    },
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
    #[error("error sending HTTP request to {host} for {provider}")]
    HttpSend {
        provider: LanguageModelProviderName,
        host: String,
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
        if code == "upstream_http_error" {
            if let Some((upstream_status, inner_message)) =
                Self::parse_upstream_error_json(&message)
            {
                let category =
                    ProviderErrorCategory::from_http_status(upstream_status, &inner_message);
                return Self::from_provider_response(
                    upstream_provider,
                    Some(upstream_status),
                    Some(code),
                    inner_message,
                    retry_after,
                    category,
                );
            }
            let category = category_from_cloud_failure(&code, &message);
            Self::from_provider_response(
                upstream_provider,
                None,
                Some(code),
                message,
                retry_after,
                category,
            )
        } else if let Some(status_code) = code
            .strip_prefix("upstream_http_")
            .and_then(|code| StatusCode::from_str(code).ok())
        {
            let category = ProviderErrorCategory::from_http_status(status_code, &message);
            Self::from_provider_response(
                upstream_provider,
                Some(status_code),
                Some(code),
                message,
                retry_after,
                category,
            )
        } else if let Some(status_code) = code
            .strip_prefix("http_")
            .and_then(|code| StatusCode::from_str(code).ok())
        {
            let category = ProviderErrorCategory::from_http_status(status_code, &message);
            Self::from_provider_response(
                ZED_CLOUD_PROVIDER_NAME,
                Some(status_code),
                Some(code),
                message,
                retry_after,
                category,
            )
        } else {
            let category = category_from_cloud_failure(&code, &message);
            Self::from_provider_response(
                upstream_provider,
                None,
                Some(code),
                message,
                retry_after,
                category,
            )
        }
    }

    pub fn from_http_status(
        provider: LanguageModelProviderName,
        status_code: StatusCode,
        message: String,
        retry_after: Option<Duration>,
    ) -> Self {
        let category = ProviderErrorCategory::from_http_status(status_code, &message);
        Self::from_provider_response(
            provider,
            Some(status_code),
            None,
            message,
            retry_after,
            category,
        )
    }

    /// Builds a [`LanguageModelCompletionError::ProviderRejection`] from raw
    /// provider response fields and the category assigned by its adapter.
    pub fn from_provider_response(
        provider: LanguageModelProviderName,
        status: Option<StatusCode>,
        code: Option<String>,
        message: String,
        retry_after: Option<Duration>,
        category: ProviderErrorCategory,
    ) -> Self {
        Self::ProviderRejection {
            provider,
            status,
            code,
            message,
            retry_after,
            category,
        }
    }

    /// Returns the delay before a retry attempt, honoring a provider-supplied
    /// delay before falling back to exponential backoff from five to forty
    /// seconds.
    ///
    /// `attempt` is one-based. Provider rejections that are not classified as
    /// transient, and error kinds without shared retry semantics, return
    /// `None`.
    pub fn retry_delay(&self, attempt: usize) -> Option<Duration> {
        if attempt == 0 {
            return None;
        }

        match self {
            Self::ProviderRejection {
                status,
                retry_after,
                category,
                ..
            } if status.is_some_and(is_retryable_provider_status)
                || matches!(
                    category,
                    ProviderErrorCategory::RateLimit
                        | ProviderErrorCategory::Overloaded
                        | ProviderErrorCategory::Timeout
                        | ProviderErrorCategory::InternalServer
                )
                || retry_after.is_some() =>
            {
                (*retry_after).or_else(|| exponential_backoff(attempt))
            }
            Self::ApiReadResponseError { .. } | Self::HttpSend { .. } => {
                exponential_backoff(attempt)
            }
            Self::DataRetentionConsentRequired { .. }
            | Self::NoApiKey { .. }
            | Self::ProviderRejection { .. }
            | Self::SerializeRequest { .. }
            | Self::BuildRequestBody { .. }
            | Self::DeserializeResponse { .. }
            | Self::StreamEndedUnexpectedly { .. }
            | Self::Other(_) => None,
        }
    }
}

fn category_from_cloud_failure(code: &str, message: &str) -> ProviderErrorCategory {
    if let Some(tokens) = parse_prompt_too_long(message) {
        return ProviderErrorCategory::PromptTooLarge {
            tokens: Some(tokens),
        };
    }
    if is_context_window_exceeded_message(message) {
        return ProviderErrorCategory::PromptTooLarge { tokens: None };
    }
    if code == "invalid_encrypted_content" || is_invalid_encrypted_content_message(message) {
        return ProviderErrorCategory::InvalidEncryptedContent;
    }

    match code {
        "context_length_exceeded" | "request_too_large" => {
            ProviderErrorCategory::PromptTooLarge { tokens: None }
        }
        "invalid_request_error" => ProviderErrorCategory::InvalidRequest,
        "authentication_error" => ProviderErrorCategory::Authentication,
        "billing_error" | "payment_required_error" => ProviderErrorCategory::PaymentRequired,
        "permission_error" => ProviderErrorCategory::Permission,
        "not_found_error" => ProviderErrorCategory::EndpointNotFound,
        "conflict_error" => ProviderErrorCategory::Conflict,
        "rate_limit_error" | "rate_limit_exceeded" => ProviderErrorCategory::RateLimit,
        "timeout_error" | "request_timed_out" => ProviderErrorCategory::Timeout,
        "api_error" | "internal_server_error" => ProviderErrorCategory::InternalServer,
        "overloaded_error" => ProviderErrorCategory::Overloaded,
        _ => ProviderErrorCategory::Other,
    }
}

fn is_retryable_provider_status(status: StatusCode) -> bool {
    status.is_server_error() || matches!(status.as_u16(), 408 | 425 | 429)
}

fn exponential_backoff(attempt: usize) -> Option<Duration> {
    const INITIAL_BACKOFF: Duration = Duration::from_secs(5);
    const MAXIMUM_BACKOFF: Duration = Duration::from_secs(40);

    let exponent = u32::try_from(attempt.checked_sub(1)?).unwrap_or(u32::MAX);
    let multiplier = 2_u32.checked_pow(exponent).unwrap_or(u32::MAX);
    Some(
        INITIAL_BACKOFF
            .checked_mul(multiplier)
            .unwrap_or(MAXIMUM_BACKOFF)
            .min(MAXIMUM_BACKOFF),
    )
}

fn is_invalid_encrypted_content_message(message: &str) -> bool {
    let Ok(response) = serde_json::from_str::<serde_json::Value>(message) else {
        return false;
    };
    response
        .get("error")
        .and_then(|error| error.get("code"))
        .or_else(|| response.get("code"))
        .and_then(serde_json::Value::as_str)
        == Some("invalid_encrypted_content")
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
    pub input: LanguageModelToolUseInput,
    pub is_input_complete: bool,
    /// Thought signature the model sent us. Some models require that this
    /// signature be preserved and sent back in conversation history for validation.
    pub thought_signature: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum LanguageModelToolUseInput {
    Json(serde_json::Value),
    Text(String),
}

impl Serialize for LanguageModelToolUseInput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("LanguageModelToolUseInput", 2)?;
        match self {
            Self::Json(input) => {
                state.serialize_field("type", "json")?;
                state.serialize_field("value", input)?;
            }
            Self::Text(input) => {
                state.serialize_field("type", "text")?;
                state.serialize_field("value", input)?;
            }
        }
        state.end()
    }
}

impl<'de> Deserialize<'de> for LanguageModelToolUseInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        if let Some(object) = value.as_object()
            && object.len() == 2
            && let Some(input_type) = object.get("type").and_then(|value| value.as_str())
            && let Some(input) = object.get("value")
        {
            return match input_type {
                "json" => Ok(Self::Json(input.clone())),
                "text" => input
                    .as_str()
                    .map(|input| Self::Text(input.to_string()))
                    .ok_or_else(|| serde::de::Error::custom("text tool input must be a string")),
                _ => Ok(Self::Json(value)),
            };
        }

        Ok(Self::Json(value))
    }
}

impl LanguageModelToolUseInput {
    pub fn as_json(&self) -> Option<&serde_json::Value> {
        match self {
            Self::Json(input) => Some(input),
            Self::Text(_) => None,
        }
    }

    /// Typed parsing for JSON tool inputs; freeform (Text) inputs always error.
    ///
    /// Callers wanting the raw value should use [`Self::as_json`] or [`Self::into_json`].
    pub fn parse<T: serde::de::DeserializeOwned>(&self) -> Result<T> {
        match self {
            Self::Json(input) => {
                serde_json::from_value(input.clone()).context("failed to parse JSON tool input")
            }
            Self::Text(_) => Err(anyhow!("custom tool text input cannot be parsed as JSON")),
        }
    }

    pub fn into_json(self) -> Result<serde_json::Value> {
        match self {
            Self::Json(input) => Ok(input),
            Self::Text(_) => Err(anyhow!("custom tool text input cannot be used as JSON")),
        }
    }

    pub fn to_display_json(&self) -> serde_json::Value {
        match self {
            Self::Json(input) => input.clone(),
            Self::Text(input) => serde_json::Value::String(input.clone()),
        }
    }
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

#[derive(Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd, Serialize, Deserialize)]
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
    Adaptive,
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
    None,
    Minimal,
    Low,
    Medium,
    High,
    XHigh,
    Max,
}

impl ReasoningEffort {
    pub const OPENAI_COMPATIBLE_SELECTABLE: [Self; 6] = [
        Self::Minimal,
        Self::Low,
        Self::Medium,
        Self::High,
        Self::XHigh,
        Self::Max,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Minimal => "Minimal",
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::XHigh => "Extra High",
            Self::Max => "Max",
        }
    }

    pub fn value(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Minimal => "minimal",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::XHigh => "xhigh",
            Self::Max => "max",
        }
    }
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
            LanguageModelCompletionError::ProviderRejection {
                provider,
                category: ProviderErrorCategory::Overloaded,
                ..
            } => {
                assert_eq!(provider.0, "anthropic");
            }
            _ => panic!(
                "Expected Overloaded category for 503 status, got: {:?}",
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
            LanguageModelCompletionError::ProviderRejection {
                provider,
                status,
                code,
                message,
                category,
                ..
            } => {
                assert_eq!(provider.0, "anthropic");
                assert_eq!(status, Some(StatusCode::INTERNAL_SERVER_ERROR));
                assert_eq!(code.as_deref(), Some("upstream_http_error"));
                assert_eq!(message, "Internal server error");
                assert_eq!(category, ProviderErrorCategory::InternalServer);
            }
            _ => panic!(
                "Expected ProviderRejection for 500 status, got: {:?}",
                error
            ),
        }
    }

    #[test]
    fn test_from_http_status_maps_context_length_exceeded_to_prompt_too_large() {
        let error = LanguageModelCompletionError::from_http_status(
            String::from("OpenAI").into(),
            StatusCode::BAD_REQUEST,
            r#"{"error":{"type":"invalid_request_error","code":"context_length_exceeded","message":"Your input exceeds the context window of this model. Please adjust your input and try again.","param":"input"}}"#.to_string(),
            None,
        );

        assert!(matches!(
            error,
            LanguageModelCompletionError::ProviderRejection {
                category: ProviderErrorCategory::PromptTooLarge { tokens: None },
                ..
            }
        ));

        let error = LanguageModelCompletionError::from_http_status(
            String::from("OpenAI").into(),
            StatusCode::BAD_REQUEST,
            "Invalid request.".to_string(),
            None,
        );

        assert!(matches!(
            error,
            LanguageModelCompletionError::ProviderRejection {
                status: Some(StatusCode::BAD_REQUEST),
                category: ProviderErrorCategory::InvalidRequest,
                ..
            }
        ));
    }

    #[test]
    fn test_from_http_status_maps_invalid_encrypted_content() {
        let message = r#"{"error":{"type":"invalid_request_error","code":"invalid_encrypted_content","message":"The encrypted content is invalid.","param":"input"}}"#;
        let error = LanguageModelCompletionError::from_http_status(
            String::from("OpenAI").into(),
            StatusCode::BAD_REQUEST,
            message.to_string(),
            None,
        );

        assert!(matches!(
            error,
            LanguageModelCompletionError::ProviderRejection {
                provider,
                message: error_message,
                category: ProviderErrorCategory::InvalidEncryptedContent,
                ..
            } if provider.0 == "OpenAI" && error_message == message
        ));
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
            LanguageModelCompletionError::ProviderRejection {
                provider,
                category: ProviderErrorCategory::Overloaded,
                ..
            } => {
                assert_eq!(provider.0, "anthropic");
            }
            _ => panic!("Expected Overloaded category for upstream_http_503"),
        }
    }

    #[test]
    fn test_from_cloud_failure_preserves_unknown_provider_rejection() {
        let error = LanguageModelCompletionError::from_cloud_failure(
            OPEN_AI_PROVIDER_NAME,
            "cyber_policy".to_string(),
            "This content was flagged as potentially violating our terms of use.".to_string(),
            None,
        );

        assert!(matches!(
            error,
            LanguageModelCompletionError::ProviderRejection {
                provider,
                status: None,
                code: Some(code),
                message,
                retry_after: None,
                category: ProviderErrorCategory::Other,
            } if provider == OPEN_AI_PROVIDER_NAME
                && code == "cyber_policy"
                && message == "This content was flagged as potentially violating our terms of use."
        ));
    }

    #[test]
    fn test_provider_category_preserves_status_less_rejection_code() {
        let error = LanguageModelCompletionError::from_provider_response(
            ANTHROPIC_PROVIDER_NAME,
            None,
            Some("rate_limit_error".to_string()),
            "Rate limit exceeded".to_string(),
            None,
            ProviderErrorCategory::RateLimit,
        );

        assert!(matches!(
            error,
            LanguageModelCompletionError::ProviderRejection {
                status: None,
                code: Some(code),
                category: ProviderErrorCategory::RateLimit,
                ..
            } if code == "rate_limit_error"
        ));
    }

    #[test]
    fn test_retry_delay_uses_provider_delay_or_bounded_exponential_backoff() {
        let retry_after = Duration::from_secs(17);
        let error = LanguageModelCompletionError::from_http_status(
            ANTHROPIC_PROVIDER_NAME,
            StatusCode::TOO_MANY_REQUESTS,
            "Rate limit exceeded".to_string(),
            Some(retry_after),
        );
        assert_eq!(error.retry_delay(1), Some(retry_after));

        let error = LanguageModelCompletionError::from_http_status(
            ANTHROPIC_PROVIDER_NAME,
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
            None,
        );
        assert_eq!(error.retry_delay(0), None);
        assert_eq!(error.retry_delay(1), Some(Duration::from_secs(5)));
        assert_eq!(error.retry_delay(2), Some(Duration::from_secs(10)));
        assert_eq!(error.retry_delay(4), Some(Duration::from_secs(40)));
        assert_eq!(error.retry_delay(20), Some(Duration::from_secs(40)));
    }

    #[test]
    fn test_retry_delay_retries_statusless_transient_provider_errors() {
        for category in [
            ProviderErrorCategory::Timeout,
            ProviderErrorCategory::InternalServer,
        ] {
            let error = LanguageModelCompletionError::from_provider_response(
                ANTHROPIC_PROVIDER_NAME,
                None,
                Some("provider_error".to_string()),
                "Transient provider error".to_string(),
                None,
                category,
            );

            assert_eq!(error.retry_delay(1), Some(Duration::from_secs(5)));
        }
    }

    #[test]
    fn test_retry_delay_rejects_permanent_provider_error() {
        let error = LanguageModelCompletionError::from_provider_response(
            OPEN_AI_PROVIDER_NAME,
            None,
            Some("cyber_policy".to_string()),
            "This content was flagged as potentially violating our terms of use.".to_string(),
            None,
            ProviderErrorCategory::Other,
        );

        assert_eq!(error.retry_delay(1), None);
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
            LanguageModelCompletionError::ProviderRejection {
                provider,
                category: ProviderErrorCategory::Overloaded,
                ..
            } => {
                assert_eq!(provider.0, "anthropic");
            }
            _ => panic!(
                "Expected Overloaded category for connection timeout with 503 status, got: {:?}",
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
            LanguageModelCompletionError::ProviderRejection {
                provider,
                status,
                code,
                message,
                category,
                ..
            } => {
                assert_eq!(provider.0, "anthropic");
                assert_eq!(status, Some(StatusCode::INTERNAL_SERVER_ERROR));
                assert_eq!(code.as_deref(), Some("upstream_http_error"));
                assert_eq!(
                    message,
                    "Received an error from the Anthropic API: upstream connect error or disconnect/reset before headers. reset reason: connection timeout"
                );
                assert_eq!(category, ProviderErrorCategory::InternalServer);
            }
            _ => panic!(
                "Expected ProviderRejection for connection timeout with 500 status, got: {:?}",
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
            input: LanguageModelToolUseInput::Json(json!({"arg": "value"})),
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
        assert_eq!(
            tool_use.input,
            LanguageModelToolUseInput::Json(json!({"arg": "value"}))
        );
        assert_eq!(tool_use.thought_signature, None);
    }

    #[test]
    fn test_language_model_tool_use_input_round_trips_json() {
        use serde_json::json;

        let input = LanguageModelToolUseInput::Json(json!({"arg": "value"}));
        let serialized = serde_json::to_value(&input).unwrap();
        assert_eq!(
            serialized,
            json!({
                "type": "json",
                "value": {"arg": "value"}
            })
        );

        let deserialized: LanguageModelToolUseInput = serde_json::from_value(serialized).unwrap();
        assert_eq!(deserialized, input);
    }

    #[test]
    fn test_language_model_tool_use_input_round_trips_text() {
        use serde_json::json;

        let input = LanguageModelToolUseInput::Text("raw custom input".to_string());
        let serialized = serde_json::to_value(&input).unwrap();
        assert_eq!(
            serialized,
            json!({
                "type": "text",
                "value": "raw custom input"
            })
        );

        let deserialized: LanguageModelToolUseInput = serde_json::from_value(serialized).unwrap();
        assert_eq!(deserialized, input);
    }

    #[test]
    fn test_language_model_tool_use_input_parse() {
        use serde_json::json;

        #[derive(Debug, Deserialize, PartialEq)]
        struct TestInput {
            arg: String,
        }

        let parsed: TestInput = LanguageModelToolUseInput::Json(json!({"arg": "value"}))
            .parse()
            .unwrap();
        assert_eq!(
            parsed,
            TestInput {
                arg: "value".to_string()
            }
        );

        let error = LanguageModelToolUseInput::Text("raw custom input".to_string())
            .parse::<TestInput>()
            .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("custom tool text input cannot be parsed as JSON")
        );
    }

    #[test]
    fn test_language_model_tool_use_input_deserializes_legacy_plain_json_as_json() {
        use serde_json::json;

        let deserialized: LanguageModelToolUseInput =
            serde_json::from_value(json!({"arg": "value"})).unwrap();
        assert_eq!(
            deserialized,
            LanguageModelToolUseInput::Json(json!({"arg": "value"}))
        );

        let deserialized: LanguageModelToolUseInput =
            serde_json::from_value(json!("legacy string argument")).unwrap();
        assert_eq!(
            deserialized,
            LanguageModelToolUseInput::Json(json!("legacy string argument"))
        );
    }

    #[test]
    fn test_language_model_tool_use_round_trip_with_signature() {
        use serde_json::json;

        let original = LanguageModelToolUse {
            id: LanguageModelToolUseId::from("round_trip_id"),
            name: "round_trip_tool".into(),
            raw_input: json!({"key": "value"}).to_string(),
            input: LanguageModelToolUseInput::Json(json!({"key": "value"})),
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
            input: LanguageModelToolUseInput::Json(json!({"arg": "value"})),
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
