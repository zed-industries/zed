use crate::{LanguageModelCompletionError, LanguageModelProviderId, LanguageModelProviderName};
use anthropic::AnthropicError;
pub use anthropic::parse_prompt_too_long;

pub const ANTHROPIC_PROVIDER_ID: LanguageModelProviderId =
    LanguageModelProviderId::new("anthropic");
pub const ANTHROPIC_PROVIDER_NAME: LanguageModelProviderName =
    LanguageModelProviderName::new("Anthropic");

impl From<AnthropicError> for LanguageModelCompletionError {
    fn from(error: AnthropicError) -> Self {
        let provider = ANTHROPIC_PROVIDER_NAME;
        match error {
            AnthropicError::SerializeRequest(error) => Self::SerializeRequest { provider, error },
            AnthropicError::BuildRequestBody(error) => Self::BuildRequestBody { provider, error },
            AnthropicError::HttpSend(error) => Self::HttpSend { provider, error },
            AnthropicError::DeserializeResponse(error) => {
                Self::DeserializeResponse { provider, error }
            }
            AnthropicError::ReadResponse(error) => Self::ApiReadResponseError { provider, error },
            AnthropicError::HttpResponseError {
                status_code,
                message,
            } => Self::HttpResponseError {
                provider,
                status_code,
                message,
            },
            AnthropicError::RateLimit { retry_after } => Self::RateLimitExceeded {
                provider,
                retry_after: Some(retry_after),
            },
            AnthropicError::ServerOverloaded { retry_after } => Self::ServerOverloaded {
                provider,
                retry_after,
            },
            AnthropicError::ApiError(api_error) => api_error.into(),
        }
    }
}

impl From<anthropic::ApiError> for LanguageModelCompletionError {
    fn from(error: anthropic::ApiError) -> Self {
        use anthropic::ApiErrorCode::*;
        let provider = ANTHROPIC_PROVIDER_NAME;
        match error.code() {
            Some(code) => match code {
                InvalidRequestError => Self::BadRequestFormat {
                    provider,
                    message: error.message,
                },
                AuthenticationError => Self::AuthenticationError {
                    provider,
                    message: error.message,
                },
                PermissionError => Self::PermissionError {
                    provider,
                    message: error.message,
                },
                NotFoundError => Self::ApiEndpointNotFound { provider },
                RequestTooLarge => Self::PromptTooLarge {
                    tokens: parse_prompt_too_long(&error.message),
                },
                RateLimitError => Self::RateLimitExceeded {
                    provider,
                    retry_after: None,
                },
                ApiError => Self::ApiInternalServerError {
                    provider,
                    message: error.message,
                },
                OverloadedError => Self::ServerOverloaded {
                    provider,
                    retry_after: None,
                },
            },
            None => Self::Other(error.into()),
        }
    }
}
