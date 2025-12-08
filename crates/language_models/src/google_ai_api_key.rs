use anyhow::Result;
use credentials_provider::CredentialsProvider;
use gpui::{App, Task};

const GEMINI_API_KEY_VAR_NAME: &str = "GEMINI_API_KEY";
const GOOGLE_AI_API_KEY_VAR_NAME: &str = "GOOGLE_AI_API_KEY";
const GOOGLE_AI_EXTENSION_CREDENTIAL_KEY: &str = "extension-llm-google-ai:google-ai";

/// Returns the Google AI API key for use by the Gemini CLI.
///
/// This function checks the following sources in order:
/// 1. `GEMINI_API_KEY` environment variable
/// 2. `GOOGLE_AI_API_KEY` environment variable
/// 3. Extension credential store (`extension-llm-google-ai:google-ai`)
pub fn api_key_for_gemini_cli(cx: &mut App) -> Task<Result<String>> {
    if let Ok(key) = std::env::var(GEMINI_API_KEY_VAR_NAME) {
        if !key.is_empty() {
            return Task::ready(Ok(key));
        }
    }

    if let Ok(key) = std::env::var(GOOGLE_AI_API_KEY_VAR_NAME) {
        if !key.is_empty() {
            return Task::ready(Ok(key));
        }
    }

    let credentials_provider = <dyn CredentialsProvider>::global(cx);

    cx.spawn(async move |cx| {
        let credential = credentials_provider
            .read_credentials(GOOGLE_AI_EXTENSION_CREDENTIAL_KEY, &cx)
            .await?;

        match credential {
            Some((_, key_bytes)) => {
                let key = String::from_utf8(key_bytes)?;
                Ok(key)
            }
            None => Err(anyhow::anyhow!("No Google AI API key found")),
        }
    })
}
