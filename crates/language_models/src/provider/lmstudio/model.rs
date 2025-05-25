use std::sync::Arc;
use anyhow::{Result, anyhow};
use futures::{StreamExt, stream::BoxStream, future::BoxFuture};
use gpui::{
    prelude::*,
    AppContext, AsyncApp, Context, Entity, Task, Window,
};
use http_client::HttpClient;
use language_model::{
    LanguageModel, LanguageModelProvider,
    AllLanguageModelSettings, Settings,
    LanguageModelId, LanguageModelName, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelRequest, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelToolChoice, Role,
};
use language_model::settings::Settings;
use lmstudio::{ChatCompletionRequest, ChatMessage};
use ui::{
    Button, ButtonCommon, ButtonStyle, Clickable, IconButton, IconName, Indicator, Label,
    LabelCommon, LabelSize, List, ListDirection, Switch, ToggleState,
};
use util::ResultExt;
use futures::FutureExt;

use crate::AllLanguageModelSettings;
use super::{PROVIDER_ID, PROVIDER_NAME, utils::LmStudioStreamMapper};

pub struct LmStudioLanguageModel {
    pub(crate) id: LanguageModelId,
    pub(crate) model: lmstudio::Model,
    pub(crate) http_client: Arc<dyn HttpClient>,
}

impl LmStudioLanguageModel {
    fn get_server_url(&self, cx: &AsyncApp) -> Result<String> {
        let settings = &AllLanguageModelSettings::get_global(cx).lmstudio;
        let server = settings
            .servers
            .iter()
            .find(|server| server.id == self.model.server_id)
            .ok_or_else(|| anyhow::anyhow!("Server not found"))?;
        Ok(server.api_url.clone())
    }

    fn to_lmstudio_request(&self, request: LanguageModelRequest) -> ChatCompletionRequest {
        // Make a deep clone of the tools for debugging and to preserve them
        let tools_debug = request.tools.clone();
        
        // Check if tools are empty before moving them
        let has_tools = !request.tools.is_empty();
        
        // Convert tools to LM Studio format
        let tools = request
            .tools
            .into_iter()
            .map(|tool| {
                log::debug!("Converting tool to LM Studio format: {}", tool.name);
                lmstudio::LmStudioTool::Function {
                    function: lmstudio::LmStudioFunctionTool {
                        name: tool.name,
                        description: Some(tool.description),
                        parameters: Some(tool.input_schema),
                    },
                }
            })
            .collect::<Vec<_>>();
        
        // Log the tools for debugging
        if !tools.is_empty() {
            log::debug!("LMStudio: Sending {} tools to model", tools.len());
            for tool in &tools_debug {
                log::debug!("  Tool: {}", tool.name);
            }
        }

        // Convert tool choice to LM Studio format with better handling
        let tool_choice = match request.tool_choice {
            Some(choice) => {
                log::debug!("LMStudio: Using explicit tool choice: {:?}", choice);
                match choice {
                    LanguageModelToolChoice::Auto => Some("auto"),
                    LanguageModelToolChoice::Any => Some("any"),
                    LanguageModelToolChoice::None => Some("none"),
                }
            },
            None => {
                if has_tools {
                    log::debug!("LMStudio: No tool choice specified, defaulting to 'auto' since tools are present");
                    Some("auto")
                } else {
                    log::debug!("LMStudio: No tool choice specified and no tools present, not setting tool_choice");
                    None
                }
            }
        };

        // Log the final tool choice
        if let Some(choice) = &tool_choice {
            log::debug!("LMStudio: Final tool choice: {}", choice);
        }

        ChatCompletionRequest {
            model: self.model.name.clone(),
            messages: request
                .messages
                .into_iter()
                .map(|msg| {
                    log::debug!("LMStudio: Converting message with role: {:?}", msg.role);
                    match msg.role {
                        Role::User => ChatMessage::User {
                            content: msg.string_contents(),
                        },
                        Role::Assistant => ChatMessage::Assistant {
                            content: Some(msg.string_contents()),
                            tool_calls: None,
                        },
                        Role::System => ChatMessage::System {
                            content: msg.string_contents(),
                        },
                    }
                })
                .collect(),
            stream: true,
            max_tokens: Some(-1),
            stop: Some(request.stop),
            temperature: request.temperature.or(Some(0.0)),
            tools,
            tool_choice,
        }
    }
}

impl LanguageModel for LmStudioLanguageModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(self.model.display_name().to_string())
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        LanguageModelProviderId(PROVIDER_ID.into())
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn supports_tools(&self) -> bool {
        self.model.supports_tools.unwrap_or(true)
    }

    fn supports_images(&self) -> bool {
        false
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto | LanguageModelToolChoice::Any => self.supports_tools(),
            LanguageModelToolChoice::None => true
        }
    }

    fn telemetry_id(&self) -> String {
        format!("lmstudio/{}", self.model.id())
    }

    fn max_token_count(&self) -> usize {
        self.model.max_token_count()
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        _cx: &gpui::App,
    ) -> BoxFuture<'static, Result<usize>> {
        // Convert LanguageModelRequest to ChatMessage for token counting
        let messages = self.to_lmstudio_request(request).messages;
        
        // Use the model's own token counting method
        let total_tokens = messages.iter()
            .map(|msg| {
                match msg {
                    lmstudio::ChatMessage::User { content } => 
                        lmstudio::Model::estimate_tokens(content),
                    lmstudio::ChatMessage::System { content } => 
                        lmstudio::Model::estimate_tokens(content),
                    lmstudio::ChatMessage::Assistant { content, tool_calls } => {
                        let content_tokens = content.as_ref()
                            .map_or(0, |c| lmstudio::Model::estimate_tokens(c));
                        let tool_call_tokens = tool_calls.as_ref().map_or(0, |calls| {
                            calls.iter().map(|call| {
                                // Estimate tokens for function name, arguments, and any additional metadata
                                lmstudio::Model::estimate_tokens(&call.function.name) + 
                                lmstudio::Model::estimate_tokens(&call.function.arguments) +
                                // Add some overhead for the tool call structure
                                10
                            }).sum()
                        });
                        content_tokens + tool_call_tokens
                    },
                    lmstudio::ChatMessage::Tool { content, tool_call_id } => {
                        // Estimate tokens for tool response content and ID
                        lmstudio::Model::estimate_tokens(content) + 
                        lmstudio::Model::estimate_tokens(tool_call_id) +
                        // Add some overhead for the tool response structure
                        5
                    }
                }
            })
            .sum::<usize>();
        
        // Check if we're approaching the model's token limit
        let max_tokens = self.model.max_token_count();
        if total_tokens > max_tokens * 9 / 10 {
            log::warn!(
                "LM Studio token count is approaching model limit: {}/{} ({}%)",
                total_tokens,
                max_tokens,
                (total_tokens as f64 / max_tokens as f64 * 100.0) as usize
            );
        }
        
        async move { Ok(total_tokens) }.boxed()
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            BoxStream<'static, Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>,
        >,
    > {
        // Clone needed variables for the async block
        let http_client = self.http_client.clone();
        let model_name = self.model.name.clone();
        let lmstudio_request = self.to_lmstudio_request(request);
        let model_id = self.id.0.clone();
        
        // Get the server URL upfront before the async block
        let server_url_result = cx.update(|app| {
            self.get_server_url(app)
        });
        
        async move {
            // First get the server URL, which may fail if no servers are available
            let server_url = match server_url_result {
                Ok(url_result) => match url_result {
                    Ok(url) => url,
                    Err(e) => {
                        log::error!("Failed to get server URL for model {}: {}", model_id, e);
                        return Err(anyhow!("No available LM Studio server for model {}: {}", model_id, e));
                    }
                },
                Err(e) => return Err(anyhow!("Failed during server URL lookup: {}", e)),
            };
            
            log::info!("Streaming completion from LM Studio model {} at {}", model_name, server_url);

            // Create stream mapper to handle the response
            let mut stream_mapper = LmStudioStreamMapper::new();

            // Get streaming response from LM Studio
            let stream = match lmstudio::stream_chat_completion(
                &*http_client,
                &server_url,
                lmstudio_request,
            )
            .await {
                Ok(stream) => stream,
                Err(err) => {
                    log::error!("Error streaming from LM Studio: {}", err);
                    return Err(anyhow!("Error connecting to LM Studio: {}", err));
                }
            };

            // Map the stream to LanguageModelCompletionEvent
            let mapped_stream = stream.map(move |fragment| {
                match fragment {
                    Ok(chat_response) => {
                        match stream_mapper.process_fragment(chat_response) {
                            Ok(Some(event)) => Ok(event),
                            Ok(None) => Ok(LanguageModelCompletionEvent::Text(String::new())), // Send empty text for fragments that don't produce events
                            Err(e) => {
                                log::error!("Error processing LM Studio response fragment: {}", e);
                                Err(LanguageModelCompletionError::Other(e))
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Error receiving LM Studio response: {}", e);
                        Err(LanguageModelCompletionError::Other(anyhow!("{}", e)))
                    }
                }
            })
            .boxed();

            Ok(mapped_stream)
        }
        .boxed()
    }
} 