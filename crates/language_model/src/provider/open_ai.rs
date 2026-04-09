use crate::{LanguageModelCompletionError, LanguageModelProviderId, LanguageModelProviderName};
use http_client::http;
use std::time::Duration;

pub const OPEN_AI_PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("openai");
pub const OPEN_AI_PROVIDER_NAME: LanguageModelProviderName =
    LanguageModelProviderName::new("OpenAI");

impl From<open_ai::RequestError> for LanguageModelCompletionError {
    fn from(error: open_ai::RequestError) -> Self {
        match error {
            open_ai::RequestError::HttpResponseError {
                provider,
                status_code,
                body,
                headers,
            } => {
                let retry_after = headers
                    .get(http::header::RETRY_AFTER)
                    .and_then(|val| val.to_str().ok()?.parse::<u64>().ok())
                    .map(Duration::from_secs);

                Self::from_http_status(provider.into(), status_code, body, retry_after)
            }
            open_ai::RequestError::Other(e) => Self::Other(e),
        }
    }
}
