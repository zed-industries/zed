use crate::{LanguageModelCompletionError, LanguageModelProviderName};
use http_client::StatusCode;
use open_router::OpenRouterError;

impl From<OpenRouterError> for LanguageModelCompletionError {
    fn from(error: OpenRouterError) -> Self {
        let provider = LanguageModelProviderName::new("OpenRouter");
        match error {
            OpenRouterError::SerializeRequest(error) => Self::SerializeRequest { provider, error },
            OpenRouterError::BuildRequestBody(error) => Self::BuildRequestBody { provider, error },
            OpenRouterError::HttpSend(error) => Self::HttpSend { provider, error },
            OpenRouterError::DeserializeResponse(error) => {
                Self::DeserializeResponse { provider, error }
            }
            OpenRouterError::ReadResponse(error) => Self::ApiReadResponseError { provider, error },
            OpenRouterError::RateLimit { retry_after } => Self::RateLimitExceeded {
                provider,
                retry_after: Some(retry_after),
            },
            OpenRouterError::ServerOverloaded { retry_after } => Self::ServerOverloaded {
                provider,
                retry_after,
            },
            OpenRouterError::ApiError(api_error) => api_error.into(),
        }
    }
}

impl From<open_router::ApiError> for LanguageModelCompletionError {
    fn from(error: open_router::ApiError) -> Self {
        use open_router::ApiErrorCode::*;
        let provider = LanguageModelProviderName::new("OpenRouter");
        match error.code {
            InvalidRequestError => Self::BadRequestFormat {
                provider,
                message: error.message,
            },
            AuthenticationError => Self::AuthenticationError {
                provider,
                message: error.message,
            },
            PaymentRequiredError => Self::AuthenticationError {
                provider,
                message: format!("Payment required: {}", error.message),
            },
            PermissionError => Self::PermissionError {
                provider,
                message: error.message,
            },
            RequestTimedOut => Self::HttpResponseError {
                provider,
                status_code: StatusCode::REQUEST_TIMEOUT,
                message: error.message,
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
        }
    }
}
