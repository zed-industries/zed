use anyhow::{anyhow, Result};
use futures::{future::BoxFuture, stream::BoxStream, FutureExt, StreamExt};
use gpui::{App, AsyncApp};
use http_client::HttpClient;
use language_model::{
    LanguageModel, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelId, LanguageModelName,
    LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelRequest, LanguageModelToolChoice, Role,
};
use settings::Settings;
use std::sync::Arc;

use crate::AllLanguageModelSettings;
use super::{PROVIDER_ID, PROVIDER_NAME, utils::LmStudioStreamMapper};
use lmstudio::{ChatCompletionRequest, ChatMessage};

pub struct LmStudioLanguageModel {
    pub(crate) id: LanguageModelId,
    pub(crate) model: lmstudio::Model,
    pub(crate) http_client: Arc<dyn HttpClient>,
}

impl LmStudioLanguageModel {
    fn get_server_url(&self, cx: &App) -> Result<String> {
        let settings = &AllLanguageModelSettings::get_global(cx).lmstudio;
        
        // If the model has a server_id, use that server's URL
        if let Some(server_id) = &self.model.server_id {
            for server in &settings.servers {
                if &server.id == server_id && server.enabled {
                    return Ok(server.api_url.clone());
                }
            }
            
            // If server was found but is disabled
            for server in &settings.servers {
                if &server.id == server_id {
                    return Err(anyhow!("The server '{}' is disabled", server.name));
                }
            }
            
            // Don't fallback to another server, require a server match
            return Err(anyhow!("Server not found for model {}", self.model.name));
        }
        
        // For backwards compatibility with models that don't have a server_id
        // Fallback to first enabled server
        if let Some(server) = settings.first_enabled_server() {
            log::warn!("Model {} has no server_id, using first enabled server", self.model.name);
            return Ok(server.api_url.clone());
        }
        
        // No servers configured
        Err(anyhow!("No enabled LM Studio servers found"))
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
            .map(|tool| lmstudio::LmStudioTool::Function {
                function: lmstudio::LmStudioFunctionTool {
                    name: tool.name,
                    description: Some(tool.description),
                    parameters: Some(tool.input_schema),
                },
            })
            .collect::<Vec<_>>();
        
        // Log the tools for debugging
        if !tools.is_empty() {
            log::debug!("LMStudio: Sending {} tools to model", tools.len());
            for tool in &tools_debug {
                log::debug!("  Tool: {}", tool.name);
            }
        }

        // Convert tool choice to LM Studio format
        let tool_choice = match request.tool_choice {
            Some(choice) => match choice {
                LanguageModelToolChoice::Auto => Some("auto"),
                LanguageModelToolChoice::Any => Some("any"),
                LanguageModelToolChoice::None => Some("none"),
            },
            None => if has_tools { Some("auto") } else { None },
        };

        ChatCompletionRequest {
            model: self.model.name.clone(),
            messages: request
                .messages
                .into_iter()
                .map(|msg| match msg.role {
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
        _cx: &App,
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
                                lmstudio::Model::estimate_tokens(&call.function.name) + 
                                lmstudio::Model::estimate_tokens(&call.function.arguments)
                            }).sum()
                        });
                        content_tokens + tool_call_tokens
                    },
                    lmstudio::ChatMessage::Tool { content, tool_call_id } => {
                        lmstudio::Model::estimate_tokens(content) + 
                        lmstudio::Model::estimate_tokens(tool_call_id)
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
                            Err(e) => Err(LanguageModelCompletionError::Other(e)),
                        }
                    }
                    Err(e) => Err(LanguageModelCompletionError::Other(anyhow!("{}", e))),
                }
            })
            .boxed();

            Ok(mapped_stream)
        }
        .boxed()
    }
} 