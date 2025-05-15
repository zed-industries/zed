use std::fmt;
use std::sync::Arc;
use std::pin::Pin;
use std::future::Future;

use anyhow::{Result, anyhow};
use futures::{Stream, StreamExt};
use gpui::{AsyncApp, AppContext};
use language_model::{
    ConfiguredModel, LanguageModel, LanguageModelCompletionEvent, LanguageModelRequest,
    LanguageModelToolResult, LanguageModelToolUseId, Role, StopReason, TokenUsage,
};
use tokio::sync::mpsc;

use crate::conversation_mod::conversation::{Conversation, MessageId};

/// CompletionError type for handling errors in completion process
#[derive(Debug, thiserror::Error)]
pub enum CompletionError {
    #[error("{header}: {message}")]
    Message {
        header: String,
        message: String,
    },
    
    #[error("{0}")]
    Other(#[from] anyhow::Error),
}

impl CompletionError {
    pub fn message(header: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Message {
            header: header.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for CompletionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Message { header, message } => {
                write!(f, "{}: {}", header, message)
            }
            Self::Other(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for CompletionError {}

/// Events emitted by the completion service
#[derive(Debug, Clone)]
pub enum CompletionEvent {
    TextChunk(String),
    ThinkingChunk { 
        text: String,
        signature: Option<String>,
    },
    ToolCall {
        tool_use_id: LanguageModelToolUseId,
        tool_name: Arc<str>,
        input: serde_json::Value,
    },
    Stopped(Result<StopReason, Arc<anyhow::Error>>),
}

/// Service responsible for interacting with language models
pub trait CompletionService: Send + Sync {
    /// Stream a completion
    fn stream_completion(
        &self,
        conversation: &Conversation,
        model: Arc<dyn LanguageModel>,
        tools: Vec<LanguageModelRequestTool>,
        message_id: MessageId,
        window: Option<AnyWindowHandle>,
        cx: &mut AsyncApp,
    ) -> Pin<Box<dyn Stream<Item = Result<CompletionEvent, CompletionError>> + Send>>;

    /// Stream a completion with a custom request
    fn stream_completion_with_request(
        &self,
        request: LanguageModelRequest,
        model: Arc<dyn LanguageModel>,
        message_id: MessageId,
        window: Option<AnyWindowHandle>,
        cx: &mut AsyncApp,
    ) -> Pin<Box<dyn Stream<Item = Result<CompletionEvent, CompletionError>> + Send>>;

    /// Calculate token usage for a request
    fn calculate_token_usage(
        &self,
        request: &LanguageModelRequest,
        model: Arc<dyn LanguageModel>,
        cx: &mut AsyncApp,
    ) -> Pin<Box<dyn Future<Output = Result<TokenUsage>> + Send>>;
}

/// Default implementation of CompletionService
pub struct DefaultCompletionService;

impl DefaultCompletionService {
    pub fn new() -> Self {
        Self
    }
}

impl CompletionService for DefaultCompletionService {
    fn stream_completion(
        &self,
        conversation: &Conversation,
        model: Arc<dyn LanguageModel>,
        tools: Vec<LanguageModelRequestTool>,
        message_id: MessageId,
        window: Option<AnyWindowHandle>,
        cx: &mut AsyncApp,
    ) -> Pin<Box<dyn Stream<Item = Result<CompletionEvent, CompletionError>> + Send>> {
        let request = self.prepare_request(conversation, model.clone(), tools);
        self.stream_completion_with_request(request, model, message_id, window, cx)
    }
    
    fn stream_completion_with_request(
        &self,
        request: LanguageModelRequest,
        model: Arc<dyn LanguageModel>,
        message_id: MessageId,
        window: Option<AnyWindowHandle>,
        cx: &mut AsyncApp,
    ) -> Pin<Box<dyn Stream<Item = Result<CompletionEvent, CompletionError>> + Send>> {
        let (tx, rx) = mpsc::channel(10);
        
        cx.background_spawn(async move {
            // Call the model
            let mut model_stream = match model.complete(&request).await {
                Ok(stream) => stream,
                Err(err) => {
                    let _ = tx.send(Err(CompletionError::Other(err))).await;
                    return;
                }
            };
            
            // Process model response events
            while let Some(event) = model_stream.next().await {
                match event {
                    Ok(event) => {
                        match event {
                            LanguageModelCompletionEvent::Text(text) => {
                                let _ = tx.send(Ok(CompletionEvent::TextChunk(text))).await;
                            }
                            LanguageModelCompletionEvent::ToolUse(tool_use) => {
                                let _ = tx.send(Ok(CompletionEvent::ToolCall {
                                    tool_use_id: tool_use.id,
                                    tool_name: tool_use.name,
                                    input: tool_use.input,
                                })).await;
                            }
                            LanguageModelCompletionEvent::Finished(stop_reason) => {
                                let _ = tx.send(Ok(CompletionEvent::Stopped(Ok(stop_reason)))).await;
                                break;
                            }
                            LanguageModelCompletionEvent::Thinking { text, signature } => {
                                let _ = tx.send(Ok(CompletionEvent::ThinkingChunk {
                                    text, 
                                    signature,
                                })).await;
                            }
                        }
                    }
                    Err(err) => {
                        let _ = tx.send(Err(CompletionError::Other(err))).await;
                        break;
                    }
                }
            }
        });
        
        Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx))
    }
    
    fn prepare_request(
        &self,
        conversation: &Conversation,
        model: Arc<dyn LanguageModel>,
        tools: Vec<LanguageModelRequestTool>,
    ) -> LanguageModelRequest {
        let messages = conversation.to_model_messages();
        
        // Create a request with system prompt and messages
        LanguageModelRequest {
            thread_id: conversation.id().to_string(),
            prompt_id: conversation.current_prompt_id().to_string(),
            mode: language_model::CompletionMode::Chat,
            messages,
            tools,
            tool_results: Vec::new(), // Initialize with empty tool results
            project: None,            // Will be set by caller if needed
            action_log: None,         // Will be set by caller if needed
        }
    }
    
    fn calculate_token_usage(
        &self,
        request: &LanguageModelRequest,
        model: Arc<dyn LanguageModel>,
        cx: &mut AsyncApp,
    ) -> Pin<Box<dyn Future<Output = Result<TokenUsage>> + Send>> {
        Box::pin(async move {
            model.calculate_tokens(request).await
        })
    }
}

// Helper function to map model events to completion events
fn map_model_event(
    event: Result<LanguageModelCompletionEvent, anyhow::Error>,
    message_id: MessageId,
) -> Result<CompletionEvent, CompletionError> {
    match event {
        Ok(event) => match event {
            LanguageModelCompletionEvent::Text(text) => {
                Ok(CompletionEvent::TextChunk(text))
            }
            LanguageModelCompletionEvent::ToolUse(tool_use) => {
                Ok(CompletionEvent::ToolCall {
                    tool_use_id: tool_use.id,
                    tool_name: tool_use.name,
                    input: tool_use.input,
                })
            }
            LanguageModelCompletionEvent::Thinking { text, signature } => {
                Ok(CompletionEvent::ThinkingChunk { text, signature })
            }
            LanguageModelCompletionEvent::Finished(stop_reason) => {
                Ok(CompletionEvent::Stopped(Ok(stop_reason)))
            }
        },
        Err(err) => Err(CompletionError::Other(err)),
    }
} 