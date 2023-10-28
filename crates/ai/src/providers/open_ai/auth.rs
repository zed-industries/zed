use std::env;

use gpui::AppContext;
use util::ResultExt;

use crate::auth::{CredentialProvider, ProviderCredential};
use crate::providers::open_ai::OPENAI_API_URL;

#[derive(Clone)]
pub struct OpenAICredentialProvider {}

impl CredentialProvider for OpenAICredentialProvider {
    fn retrieve_credentials(&self, cx: &AppContext) -> ProviderCredential {
        let api_key = if let Ok(api_key) = env::var("OPENAI_API_KEY") {
            Some(api_key)
        } else if let Some((_, api_key)) = cx
            .platform()
            .read_credentials(OPENAI_API_URL)
            .log_err()
            .flatten()
        {
            String::from_utf8(api_key).log_err()
        } else {
            None
        };

        if let Some(api_key) = api_key {
            ProviderCredential::Credentials { api_key }
        } else {
            ProviderCredential::NoCredentials
        }
    }
    fn save_credentials(&self, cx: &AppContext, credential: ProviderCredential) {
        match credential {
            ProviderCredential::Credentials { api_key } => {
                cx.platform()
                    .write_credentials(OPENAI_API_URL, "Bearer", api_key.as_bytes())
                    .log_err();
            }
            _ => {}
        }
    }
    fn delete_credentials(&self, cx: &AppContext) {
        cx.platform().delete_credentials(OPENAI_API_URL).log_err();
    }
}
