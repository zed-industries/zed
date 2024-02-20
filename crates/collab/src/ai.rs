use anyhow::{anyhow, Result};
use rpc::proto;

pub fn language_model_request_to_open_ai(
    request: proto::CompleteWithLanguageModel,
) -> Result<open_ai::Request> {
    Ok(open_ai::Request {
        model: open_ai::Model::from_id(&request.model).unwrap_or(open_ai::Model::FourTurbo),
        messages: request
            .messages
            .into_iter()
            .map(|message| {
                let role = proto::LanguageModelRole::from_i32(message.role)
                    .ok_or_else(|| anyhow!("invalid role {}", message.role))?;
                Ok(open_ai::RequestMessage {
                    role: match role {
                        proto::LanguageModelRole::LanguageModelUser => open_ai::Role::User,
                        proto::LanguageModelRole::LanguageModelAssistant => {
                            open_ai::Role::Assistant
                        }
                        proto::LanguageModelRole::LanguageModelSystem => open_ai::Role::System,
                    },
                    content: message.content,
                })
            })
            .collect::<Result<Vec<open_ai::RequestMessage>>>()?,
        stream: true,
        stop: request.stop,
        temperature: request.temperature,
    })
}

pub fn language_model_request_to_google_ai(
    request: proto::CompleteWithLanguageModel,
) -> Result<google_ai::GenerateContentRequest> {
    Ok(google_ai::GenerateContentRequest {
        contents: request
            .messages
            .into_iter()
            .map(language_model_request_message_to_google_ai)
            .collect::<Result<Vec<_>>>()?,
        generation_config: None,
        safety_settings: None,
    })
}

pub fn language_model_request_message_to_google_ai(
    message: proto::LanguageModelRequestMessage,
) -> Result<google_ai::Content> {
    let role = proto::LanguageModelRole::from_i32(message.role)
        .ok_or_else(|| anyhow!("invalid role {}", message.role))?;

    Ok(google_ai::Content {
        parts: vec![google_ai::Part::TextPart(google_ai::TextPart {
            text: message.content,
        })],
        role: match role {
            proto::LanguageModelRole::LanguageModelUser => google_ai::Role::User,
            proto::LanguageModelRole::LanguageModelAssistant => google_ai::Role::Model,
            proto::LanguageModelRole::LanguageModelSystem => google_ai::Role::User,
        },
    })
}

pub fn count_tokens_request_to_google_ai(
    request: proto::CountTokensWithLanguageModel,
) -> Result<google_ai::CountTokensRequest> {
    Ok(google_ai::CountTokensRequest {
        contents: request
            .messages
            .into_iter()
            .map(language_model_request_message_to_google_ai)
            .collect::<Result<Vec<_>>>()?,
    })
}
