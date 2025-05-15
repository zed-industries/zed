use std::sync::Arc;

use anyhow::{Result, anyhow};
use futures::{StreamExt, Stream};
use gpui::{AnyWindowHandle, AsyncApp, Context, Entity, Task};
use language_model::{
    ConfiguredModel, LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelId, LanguageModelKnownError, LanguageModelRegistry, LanguageModelRequest,
    LanguageModelRequestMessage, LanguageModelRequestTool, RequestUsage, Role, StopReason,
    TokenUsage,
};
use postage::stream::Stream as _;

use crate::conversation::{Conversation, MessageId, MessageSegment};
use crate::thread::{ThreadError, ThreadEvent};

/// Events produced during completion
#[derive(Debug)]
pub enum CompletionEvent {
    TextChunk(String),
    ThinkingChunk {
        text: String,
        signature: Option<String>,
    },
    ToolCall {
        tool_name: Arc<str>,
        tool_id: Arc<str>,
        arguments: serde_json::Value,
    },
    Stopped(Result<StopReason, Arc<anyhow::Error>>),
    Error(CompletionError),
}

/// Errors that can occur during completions
#[derive(Debug, thiserror::Error)]
pub enum CompletionError {
    #[error("Payment required")]
    PaymentRequired,
    
    #[error("Model request limit reached")]
    ModelRequestLimitReached { 
        plan: proto::Plan 
    },
    
    #[error("Message {header}: {message}")]
    Message {
        header: gpui::SharedString,
        message: gpui::SharedString,
    },
    
    #[error("Error during completion: {0}")]
    Other(#[from] anyhow::Error),
}

impl From<LanguageModelCompletionError> for CompletionError {
    fn from(error: LanguageModelCompletionError) -> Self {
        match error {
            LanguageModelCompletionError::Known(known_error) => match known_error {
                LanguageModelKnownError::PaymentRequired(_) => Self::PaymentRequired,
                LanguageModelKnownError::ModelRequestLimitReached(e) => {
                    Self::ModelRequestLimitReached { plan: e.plan }
                }
            },
            LanguageModelCompletionError::Other(e) => Self::Other(e),
        }
    }
}

/// The completion service interface
pub trait CompletionService {
    /// Stream a completion from the language model
    fn stream_completion(
        &self,
        conversation: &Conversation,
        model: Arc<dyn LanguageModel>,
        tools: Vec<LanguageModelRequestTool>,
        message_id: MessageId,
        window: Option<AnyWindowHandle>,
        cx: &mut AsyncApp,
    ) -> Task<impl Stream<Item = Result<CompletionEvent, CompletionError>>>;
    
    /// Prepare a completion request from a conversation
    fn prepare_request(
        &self,
        conversation: &Conversation,
        model: Arc<dyn LanguageModel>,
        tools: Vec<LanguageModelRequestTool>,
    ) -> LanguageModelRequest;
    
    /// Get the token usage for a request
    fn calculate_token_usage(
        &self,
        request: &LanguageModelRequest,
        model: &Arc<dyn LanguageModel>,
    ) -> Task<Result<TokenUsage>>;
}

/// Default implementation of the completion service
pub struct DefaultCompletionService;

impl DefaultCompletionService {
    pub fn new() -> Self {
        Self
    }
    
    fn map_completion_event(
        event: Result<LanguageModelCompletionEvent, String>,
        message_id: MessageId,
    ) -> Result<CompletionEvent, CompletionError> {
        match event {
            Ok(LanguageModelCompletionEvent::ContentBlock { content, .. }) => {
                Ok(CompletionEvent::TextChunk(content))
            }
            
            Ok(LanguageModelCompletionEvent::ThinkingBlock { content, end_turn, signature, .. }) => {
                Ok(CompletionEvent::ThinkingChunk {
                    text: content,
                    signature,
                })
            }
            
            Ok(LanguageModelCompletionEvent::ToolCall {
                tool_name,
                tool_call_id,
                arguments,
                ..
            }) => {
                Ok(CompletionEvent::ToolCall {
                    tool_name,
                    tool_id: tool_call_id,
                    arguments,
                })
            }
            
            Ok(LanguageModelCompletionEvent::StreamEnd { stop_reason, .. }) => {
                Ok(CompletionEvent::Stopped(Ok(stop_reason)))
            }
            
            Err(error_message) => {
                Err(CompletionError::Message {
                    header: "Error".into(),
                    message: error_message.into(),
                })
            }
        }
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
    ) -> Task<impl Stream<Item = Result<CompletionEvent, CompletionError>>> {
        let request = self.prepare_request(conversation, model.clone(), tools);
        
        cx.spawn(|cx| async move {
            let mut completion_stream = match model.complete(&request).await {
                Ok(stream) => stream,
                Err(err) => return Box::pin(futures::stream::once(async move { Err(err.into()) })),
            };
            
            let mut output_stream = postage::stream::Stream::new(16);
            
            cx.background_executor().spawn(async move {
                while let Some(event) = completion_stream.next().await {
                    let mapped_event = Self::map_completion_event(event, message_id);
                    if output_stream.send(mapped_event).await.is_err() {
                        break;
                    }
                }
            });
            
            output_stream
        })
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
        }
    }
    
    fn calculate_token_usage(
        &self,
        request: &LanguageModelRequest,
        model: &Arc<dyn LanguageModel>,
    ) -> Task<Result<TokenUsage>> {
        let request = request.clone();
        let model = model.clone();
        
        Task::spawn(async move {
            model.count_tokens(&request).await
        })
    }
}

/// Adapter to bridge between old and new completion implementations
pub struct CompletionAdapter {
    service: Arc<dyn CompletionService>,
}

impl CompletionAdapter {
    pub fn new(service: Arc<dyn CompletionService>) -> Self {
        Self { service }
    }
    
    pub fn stream_completion_to_thread(
        self: Arc<Self>,
        thread: Entity<crate::thread::Thread>,
        conversation: &Conversation,
        model: Arc<dyn LanguageModel>,
        tools: Vec<LanguageModelRequestTool>,
        message_id: MessageId,
        window: Option<AnyWindowHandle>,
        cx: &mut Context<crate::thread::Thread>,
    ) {
        let request = self.service.prepare_request(conversation, model.clone(), tools.clone());
        
        // Calculate token usage
        let token_count_task = self.service.calculate_token_usage(&request, &model);
        
        // Stream the completion
        let completion_task = cx.spawn_in_background(|cx| async move {
            let mut completion_stream = self.service
                .stream_completion(conversation, model, tools, message_id, window.clone(), cx)
                .await;
            
            while let Some(event) = completion_stream.next().await {
                match event {
                    Ok(CompletionEvent::TextChunk(text)) => {
                        thread.update(cx, |thread, cx| {
                            cx.emit(ThreadEvent::StreamedAssistantText(message_id, text));
                        })?;
                    }
                    
                    Ok(CompletionEvent::ThinkingChunk { text, signature }) => {
                        thread.update(cx, |thread, cx| {
                            cx.emit(ThreadEvent::StreamedAssistantThinking(message_id, text));
                        })?;
                    }
                    
                    Ok(CompletionEvent::ToolCall { tool_name, tool_id, arguments }) => {
                        thread.update(cx, |thread, cx| {
                            cx.emit(ThreadEvent::StreamedToolUse {
                                tool_use_id: tool_id.into(),
                                ui_text: tool_name,
                                input: arguments,
                            });
                        })?;
                    }
                    
                    Ok(CompletionEvent::Stopped(stop_reason)) => {
                        thread.update(cx, |thread, cx| {
                            cx.emit(ThreadEvent::Stopped(stop_reason));
                        })?;
                    }
                    
                    Err(error) => {
                        thread.update(cx, |thread, cx| {
                            match error {
                                CompletionError::PaymentRequired => {
                                    cx.emit(ThreadEvent::ShowError(ThreadError::PaymentRequired));
                                }
                                
                                CompletionError::ModelRequestLimitReached { plan } => {
                                    cx.emit(ThreadEvent::ShowError(
                                        ThreadError::ModelRequestLimitReached { plan }
                                    ));
                                }
                                
                                CompletionError::Message { header, message } => {
                                    cx.emit(ThreadEvent::ShowError(ThreadError::Message {
                                        header,
                                        message,
                                    }));
                                }
                                
                                CompletionError::Other(e) => {
                                    cx.emit(ThreadEvent::Stopped(Err(Arc::new(e))));
                                }
                            }
                        })?;
                    }
                }
            }
            
            Ok(())
        });
        
        // Update token usage when available
        cx.spawn_in_background(|cx| async move {
            if let Ok(token_usage) = token_count_task.await {
                thread.update(cx, |thread, cx| {
                    // Update token usage in thread
                })?;
            }
            
            Ok(())
        }).detach();
        
        // Detach the completion task
        completion_task.detach();
    }
} 