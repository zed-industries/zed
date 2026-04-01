use anyhow::Result;
use language_model_core::{LanguageModelRequest, Role};

use crate::Model;

/// Count tokens for an xAI model using tiktoken. This is synchronous;
/// callers should spawn it on a background thread if needed.
pub fn count_xai_tokens(request: LanguageModelRequest, model: Model) -> Result<u64> {
    let messages = request
        .messages
        .into_iter()
        .map(|message| tiktoken_rs::ChatCompletionRequestMessage {
            role: match message.role {
                Role::User => "user".into(),
                Role::Assistant => "assistant".into(),
                Role::System => "system".into(),
            },
            content: Some(message.string_contents()),
            name: None,
            function_call: None,
        })
        .collect::<Vec<_>>();

    let model_name = if model.max_token_count() >= 100_000 {
        "gpt-4o"
    } else {
        "gpt-4"
    };
    tiktoken_rs::num_tokens_from_messages(model_name, &messages).map(|tokens| tokens as u64)
}
