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
        available_tools: Vec<LanguageModelRequestTool>,
        cx: &mut AsyncApp,
    ) -> Pin<Box<dyn Stream<Item = Result<CompletionEvent, CompletionError>> + Send>>;

    /// Stream a completion with a custom request
    fn stream_completion_with_request(
        &self,
        request: LanguageModelRequest,
        model: Arc<dyn LanguageModel>,
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
        available_tools: Vec<LanguageModelRequestTool>,
        cx: &mut AsyncApp,
    ) -> Pin<Box<dyn Stream<Item = Result<CompletionEvent, CompletionError>> + Send>> {
        let request = self.prepare_request(conversation, model.clone(), available_tools);
        self.stream_completion_with_request(request, model, cx)
    }
    
    fn stream_completion_with_request(
        &self,
        request: LanguageModelRequest,
        model: Arc<dyn LanguageModel>,
        cx: &mut AsyncApp,
    ) -> Pin<Box<dyn Stream<Item = Result<CompletionEvent, CompletionError>> + Send>> {
        let (tx, rx) = mpsc::channel(10);
        
        cx.background_spawn(async move {
            let completion_result = model.complete(&request).await;
            
            match completion_result {
                Ok(mut stream) => {
                    while let Some(event_result) = stream.next().await {
                        match event_result {
                            Ok(event) => {
                                match event {
                                    LanguageModelCompletionEvent::ContentBlock { content, is_final, .. } => {
                                        if !content.is_empty() {
                                            if let Err(e) = tx.clone().try_send(Ok(CompletionEvent::TextChunk(content))) {
                                                if !e.is_disconnected() {
                                                    // Only log if the error isn't because the receiver was dropped
                                                    log::error!("Error sending content chunk: {}", e);
                                                }
                                                break;
                                            }
                                        }
                                    }
                                    LanguageModelCompletionEvent::ThinkingBlock { content, signature, .. } => {
                                        if !content.is_empty() {
                                            if let Err(e) = tx.clone().try_send(Ok(CompletionEvent::ThinkingChunk { 
                                                text: content,
                                                signature,
                                            })) {
                                                if !e.is_disconnected() {
                                                    log::error!("Error sending thinking chunk: {}", e);
                                                }
                                                break;
                                            }
                                        }
                                    }
                                    LanguageModelCompletionEvent::ToolCall { tool_name, tool_call_id, arguments, .. } => {
                                        if let Err(e) = tx.clone().try_send(Ok(CompletionEvent::ToolCall { 
                                            tool_use_id: tool_call_id,
                                            tool_name,
                                            input: arguments,
                                        })) {
                                            if !e.is_disconnected() {
                                                log::error!("Error sending tool call: {}", e);
                                            }
                                            break;
                                        }
                                    }
                                    LanguageModelCompletionEvent::StreamEnd { stop_reason } => {
                                        let _ = tx.clone().try_send(Ok(CompletionEvent::Stopped(Ok(stop_reason))));
                                        break;
                                    }
                                }
                            }
                            Err(error) => {
                                let _ = tx.clone().try_send(Err(CompletionError::message("Error", error)));
                                break;
                            }
                        }
                    }
                }
                Err(error) => {
                    let _ = tx.try_send(Err(CompletionError::message("Error", error.to_string())));
                }
            }
        });
        
        rx
    }
    
    fn prepare_request(
        &self,
        conversation: &Conversation,
        model: Arc<dyn LanguageModel>,
        available_tools: Vec<LanguageModelRequestTool>,
    ) -> LanguageModelRequest {
        LanguageModelRequest {
            thread_id: conversation.id().to_string(),
            messages: conversation.to_model_messages(),
            tools: available_tools,
            temperature: None,
            top_p: None,
            top_k: None,
            presence_penalty: None,
            frequency_penalty: None,
            stop: None,
            max_tokens: None,
            extra: None,
        }
    }
    
    fn calculate_token_usage(
        &self,
        request: &LanguageModelRequest,
        model: Arc<dyn LanguageModel>,
        cx: &mut AsyncApp,
    ) -> Pin<Box<dyn Future<Output = Result<TokenUsage>> + Send>> {
        let request = request.clone();
        Box::pin(async move {
            let count = model.count_tokens(request, cx.as_ref()).await?;
            Ok(TokenUsage {
                input_tokens: count,
                output_tokens: 0,
                cache_creation_input_tokens: 0,
                cache_read_input_tokens: 0,
            })
        })
    }
} 