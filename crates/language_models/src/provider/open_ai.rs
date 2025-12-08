use anyhow::{Result, anyhow};
use collections::HashMap;
use futures::{FutureExt, Stream, future::BoxFuture};
use gpui::{App, AppContext as _};
use language_model::{
    LanguageModelCompletionError, LanguageModelCompletionEvent, LanguageModelRequest,
    LanguageModelToolChoice, LanguageModelToolUse, MessageContent, Role, StopReason, TokenUsage,
};
use open_ai::{ImageUrl, Model, ReasoningEffort, ResponseStreamEvent};
pub use settings::OpenAiAvailableModel as AvailableModel;
use std::pin::Pin;
use std::str::FromStr;

use language_model::LanguageModelToolResultContent;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct OpenAiSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
}

pub fn into_open_ai(
    request: LanguageModelRequest,
    model_id: &str,
    supports_parallel_tool_calls: bool,
    supports_prompt_cache_key: bool,
    max_output_tokens: Option<u64>,
    reasoning_effort: Option<ReasoningEffort>,
) -> open_ai::Request {
    let stream = !model_id.starts_with("o1-");

    let mut messages = Vec::new();
    for message in request.messages {
        for content in message.content {
            match content {
                MessageContent::Text(text) | MessageContent::Thinking { text, .. } => {
                    if !text.trim().is_empty() {
                        add_message_content_part(
                            open_ai::MessagePart::Text { text },
                            message.role,
                            &mut messages,
                        );
                    }
                }
                MessageContent::RedactedThinking(_) => {}
                MessageContent::Image(image) => {
                    add_message_content_part(
                        open_ai::MessagePart::Image {
                            image_url: ImageUrl {
                                url: image.to_base64_url(),
                                detail: None,
                            },
                        },
                        message.role,
                        &mut messages,
                    );
                }
                MessageContent::ToolUse(tool_use) => {
                    let tool_call = open_ai::ToolCall {
                        id: tool_use.id.to_string(),
                        content: open_ai::ToolCallContent::Function {
                            function: open_ai::FunctionContent {
                                name: tool_use.name.to_string(),
                                arguments: serde_json::to_string(&tool_use.input)
                                    .unwrap_or_default(),
                            },
                        },
                    };

                    if let Some(open_ai::RequestMessage::Assistant { tool_calls, .. }) =
                        messages.last_mut()
                    {
                        tool_calls.push(tool_call);
                    } else {
                        messages.push(open_ai::RequestMessage::Assistant {
                            content: None,
                            tool_calls: vec![tool_call],
                        });
                    }
                }
                MessageContent::ToolResult(tool_result) => {
                    let content = match &tool_result.content {
                        LanguageModelToolResultContent::Text(text) => {
                            vec![open_ai::MessagePart::Text {
                                text: text.to_string(),
                            }]
                        }
                        LanguageModelToolResultContent::Image(image) => {
                            vec![open_ai::MessagePart::Image {
                                image_url: ImageUrl {
                                    url: image.to_base64_url(),
                                    detail: None,
                                },
                            }]
                        }
                    };

                    messages.push(open_ai::RequestMessage::Tool {
                        content: content.into(),
                        tool_call_id: tool_result.tool_use_id.to_string(),
                    });
                }
            }
        }
    }

    open_ai::Request {
        model: model_id.into(),
        messages,
        stream,
        stop: request.stop,
        temperature: request.temperature.unwrap_or(1.0),
        max_completion_tokens: max_output_tokens,
        parallel_tool_calls: if supports_parallel_tool_calls && !request.tools.is_empty() {
            Some(false)
        } else {
            None
        },
        prompt_cache_key: if supports_prompt_cache_key {
            request.thread_id
        } else {
            None
        },
        tools: request
            .tools
            .into_iter()
            .map(|tool| open_ai::ToolDefinition::Function {
                function: open_ai::FunctionDefinition {
                    name: tool.name,
                    description: Some(tool.description),
                    parameters: Some(tool.input_schema),
                },
            })
            .collect(),
        tool_choice: request.tool_choice.map(|choice| match choice {
            LanguageModelToolChoice::Auto => open_ai::ToolChoice::Auto,
            LanguageModelToolChoice::Any => open_ai::ToolChoice::Required,
            LanguageModelToolChoice::None => open_ai::ToolChoice::None,
        }),
        reasoning_effort,
    }
}

fn add_message_content_part(
    new_part: open_ai::MessagePart,
    role: Role,
    messages: &mut Vec<open_ai::RequestMessage>,
) {
    match (role, messages.last_mut()) {
        (Role::User, Some(open_ai::RequestMessage::User { content }))
        | (
            Role::Assistant,
            Some(open_ai::RequestMessage::Assistant {
                content: Some(content),
                ..
            }),
        )
        | (Role::System, Some(open_ai::RequestMessage::System { content, .. })) => {
            content.push_part(new_part);
        }
        _ => {
            messages.push(match role {
                Role::User => open_ai::RequestMessage::User {
                    content: open_ai::MessageContent::from(vec![new_part]),
                },
                Role::Assistant => open_ai::RequestMessage::Assistant {
                    content: Some(open_ai::MessageContent::from(vec![new_part])),
                    tool_calls: Vec::new(),
                },
                Role::System => open_ai::RequestMessage::System {
                    content: open_ai::MessageContent::from(vec![new_part]),
                },
            });
        }
    }
}

pub struct OpenAiEventMapper {
    tool_calls_by_index: HashMap<usize, RawToolCall>,
}

impl OpenAiEventMapper {
    pub fn new() -> Self {
        Self {
            tool_calls_by_index: HashMap::default(),
        }
    }

    pub fn map_stream(
        mut self,
        events: Pin<Box<dyn Send + Stream<Item = Result<ResponseStreamEvent>>>>,
    ) -> impl Stream<Item = Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>
    {
        use futures::StreamExt;
        events.flat_map(move |event| {
            futures::stream::iter(match event {
                Ok(event) => self.map_event(event),
                Err(error) => vec![Err(LanguageModelCompletionError::from(anyhow!(error)))],
            })
        })
    }

    pub fn map_event(
        &mut self,
        event: ResponseStreamEvent,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        let mut events = Vec::new();
        if let Some(usage) = event.usage {
            events.push(Ok(LanguageModelCompletionEvent::UsageUpdate(TokenUsage {
                input_tokens: usage.prompt_tokens,
                output_tokens: usage.completion_tokens,
                cache_creation_input_tokens: 0,
                cache_read_input_tokens: 0,
            })));
        }

        let Some(choice) = event.choices.first() else {
            return events;
        };

        if let Some(delta) = choice.delta.as_ref() {
            if let Some(content) = delta.content.clone() {
                events.push(Ok(LanguageModelCompletionEvent::Text(content)));
            }

            if let Some(tool_calls) = delta.tool_calls.as_ref() {
                for tool_call in tool_calls {
                    let entry = self.tool_calls_by_index.entry(tool_call.index).or_default();

                    if let Some(tool_id) = tool_call.id.clone() {
                        entry.id = tool_id;
                    }

                    if let Some(function) = tool_call.function.as_ref() {
                        if let Some(name) = function.name.clone() {
                            entry.name = name;
                        }

                        if let Some(arguments) = function.arguments.clone() {
                            entry.arguments.push_str(&arguments);
                        }
                    }
                }
            }
        }

        match choice.finish_reason.as_deref() {
            Some("stop") => {
                events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::EndTurn)));
            }
            Some("tool_calls") => {
                events.extend(self.tool_calls_by_index.drain().map(|(_, tool_call)| {
                    match serde_json::Value::from_str(&tool_call.arguments) {
                        Ok(input) => Ok(LanguageModelCompletionEvent::ToolUse(
                            LanguageModelToolUse {
                                id: tool_call.id.clone().into(),
                                name: tool_call.name.as_str().into(),
                                is_input_complete: true,
                                input,
                                raw_input: tool_call.arguments.clone(),
                                thought_signature: None,
                            },
                        )),
                        Err(error) => Ok(LanguageModelCompletionEvent::ToolUseJsonParseError {
                            id: tool_call.id.into(),
                            tool_name: tool_call.name.into(),
                            raw_input: tool_call.arguments.clone().into(),
                            json_parse_error: error.to_string(),
                        }),
                    }
                }));

                events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::ToolUse)));
            }
            Some(stop_reason) => {
                log::error!("Unexpected OpenAI stop_reason: {stop_reason:?}",);
                events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::EndTurn)));
            }
            None => {}
        }

        events
    }
}

#[derive(Default)]
struct RawToolCall {
    id: String,
    name: String,
    arguments: String,
}

pub(crate) fn collect_tiktoken_messages(
    request: LanguageModelRequest,
) -> Vec<tiktoken_rs::ChatCompletionRequestMessage> {
    request
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
        .collect::<Vec<_>>()
}

pub fn count_open_ai_tokens(
    request: LanguageModelRequest,
    model: Model,
    cx: &App,
) -> BoxFuture<'static, Result<u64>> {
    cx.background_spawn(async move {
        let messages = collect_tiktoken_messages(request);
        match model {
            Model::Custom { max_tokens, .. } => {
                let model = if max_tokens >= 100_000 {
                    "gpt-4o"
                } else {
                    "gpt-4"
                };
                tiktoken_rs::num_tokens_from_messages(model, &messages)
            }
            Model::ThreePointFiveTurbo
            | Model::Four
            | Model::FourTurbo
            | Model::FourOmni
            | Model::FourOmniMini
            | Model::FourPointOne
            | Model::FourPointOneMini
            | Model::FourPointOneNano
            | Model::O1
            | Model::O3
            | Model::O3Mini
            | Model::O4Mini
            | Model::Five
            | Model::FiveMini
            | Model::FiveNano => tiktoken_rs::num_tokens_from_messages(model.id(), &messages),
            Model::FivePointOne => tiktoken_rs::num_tokens_from_messages("gpt-5", &messages),
        }
        .map(|tokens| tokens as u64)
    })
    .boxed()
}

#[cfg(test)]
mod tests {
    use gpui::TestAppContext;
    use language_model::LanguageModelRequestMessage;
    use strum::IntoEnumIterator;

    use super::*;

    #[gpui::test]
    fn tiktoken_rs_support(cx: &TestAppContext) {
        let request = LanguageModelRequest {
            thread_id: None,
            prompt_id: None,
            intent: None,
            mode: None,
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text("message".into())],
                cache: false,
                reasoning_details: None,
            }],
            tools: vec![],
            tool_choice: None,
            stop: vec![],
            temperature: None,
            thinking_allowed: true,
        };

        for model in Model::iter() {
            let count = cx
                .executor()
                .block(count_open_ai_tokens(
                    request.clone(),
                    model,
                    &cx.app.borrow(),
                ))
                .unwrap();
            assert!(count > 0);
        }
    }
}
