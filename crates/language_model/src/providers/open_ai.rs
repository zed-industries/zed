use anyhow::Result;
use futures::{future::BoxFuture, FutureExt};
use gpui::AppContext;

use crate::{LanguageModelRequest, Role};

pub fn count_open_ai_tokens(
    request: LanguageModelRequest,
    model: open_ai::Model,
    cx: &AppContext,
) -> BoxFuture<'static, Result<usize>> {
    cx.background_executor()
        .spawn(async move {
            let messages = request
                .messages
                .into_iter()
                .map(|message| tiktoken_rs::ChatCompletionRequestMessage {
                    role: match message.role {
                        Role::User => "user".into(),
                        Role::Assistant => "assistant".into(),
                        Role::System => "system".into(),
                    },
                    content: Some(message.content),
                    name: None,
                    function_call: None,
                })
                .collect::<Vec<_>>();

            if let open_ai::Model::Custom { .. } = model {
                tiktoken_rs::num_tokens_from_messages("gpt-4", &messages)
            } else {
                tiktoken_rs::num_tokens_from_messages(model.id(), &messages)
            }
        })
        .boxed()
}
