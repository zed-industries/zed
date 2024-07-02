use anyhow::{anyhow, Context as _, Result};
use rpc::proto;
use util::ResultExt as _;

pub fn language_model_request_to_open_ai(
    request: proto::CompleteWithLanguageModel,
) -> Result<open_ai::Request> {
    Ok(open_ai::Request {
        model: open_ai::Model::from_id(&request.model).unwrap_or(open_ai::Model::FourTurbo),
        messages: request
            .messages
            .into_iter()
            .map(|message: proto::LanguageModelRequestMessage| {
                let role = proto::LanguageModelRole::from_i32(message.role)
                    .ok_or_else(|| anyhow!("invalid role {}", message.role))?;

                let openai_message = match role {
                    proto::LanguageModelRole::LanguageModelUser => open_ai::RequestMessage::User {
                        content: message.content,
                    },
                    proto::LanguageModelRole::LanguageModelAssistant => {
                        open_ai::RequestMessage::Assistant {
                            content: Some(message.content),
                            tool_calls: message
                                .tool_calls
                                .into_iter()
                                .filter_map(|call| {
                                    Some(open_ai::ToolCall {
                                        id: call.id,
                                        content: match call.variant? {
                                            proto::tool_call::Variant::Function(f) => {
                                                open_ai::ToolCallContent::Function {
                                                    function: open_ai::FunctionContent {
                                                        name: f.name,
                                                        arguments: f.arguments,
                                                    },
                                                }
                                            }
                                        },
                                    })
                                })
                                .collect(),
                        }
                    }
                    proto::LanguageModelRole::LanguageModelSystem => {
                        open_ai::RequestMessage::System {
                            content: message.content,
                        }
                    }
                    proto::LanguageModelRole::LanguageModelTool => open_ai::RequestMessage::Tool {
                        tool_call_id: message
                            .tool_call_id
                            .ok_or_else(|| anyhow!("tool message is missing tool call id"))?,
                        content: message.content,
                    },
                };

                Ok(openai_message)
            })
            .collect::<Result<Vec<open_ai::RequestMessage>>>()?,
        stream: true,
        stop: request.stop,
        temperature: request.temperature,
        tools: request
            .tools
            .into_iter()
            .filter_map(|tool| {
                Some(match tool.variant? {
                    proto::chat_completion_tool::Variant::Function(f) => {
                        open_ai::ToolDefinition::Function {
                            function: open_ai::FunctionDefinition {
                                name: f.name,
                                description: f.description,
                                parameters: if let Some(params) = &f.parameters {
                                    Some(
                                        serde_json::from_str(params)
                                            .context("failed to deserialize tool parameters")
                                            .log_err()?,
                                    )
                                } else {
                                    None
                                },
                            },
                        }
                    }
                })
            })
            .collect(),
        tool_choice: request.tool_choice,
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
            proto::LanguageModelRole::LanguageModelTool => {
                Err(anyhow!("we don't handle tool calls with google ai yet"))?
            }
        },
    })
}

pub fn cache_language_model_content_request_to_google_ai(
    request: proto::CacheLanguageModelContent,
) -> Result<google_ai::CreateCachedContentRequest> {
    Ok(google_ai::CreateCachedContentRequest {
        contents: request
            .messages
            .into_iter()
            .map(language_model_request_message_to_google_ai)
            .collect::<Result<Vec<_>>>()?,
        tools: if request.tools.is_empty() {
            None
        } else {
            Some(
                request
                    .tools
                    .into_iter()
                    .try_fold(Vec::new(), |mut acc, tool| {
                        if let Some(variant) = tool.variant {
                            match variant {
                                proto::chat_completion_tool::Variant::Function(f) => {
                                    let description = f.description.ok_or_else(|| {
                                        anyhow!("Function tool is missing a description")
                                    })?;
                                    let parameters = f.parameters.ok_or_else(|| {
                                        anyhow!("Function tool is missing parameters")
                                    })?;
                                    let parsed_parameters = serde_json::from_str(&parameters)
                                        .map_err(|e| {
                                            anyhow!("Failed to parse parameters: {}", e)
                                        })?;
                                    acc.push(google_ai::Tool {
                                        function_declarations: vec![
                                            google_ai::FunctionDeclaration {
                                                name: f.name,
                                                description,
                                                parameters: parsed_parameters,
                                            },
                                        ],
                                    });
                                }
                            }
                        }
                        anyhow::Ok(acc)
                    })?,
            )
        },
        ttl: request.ttl_seconds.map(|s| format!("{}s", s)),
        display_name: None,
        model: request.model,
        system_instruction: None,
        tool_config: None,
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
