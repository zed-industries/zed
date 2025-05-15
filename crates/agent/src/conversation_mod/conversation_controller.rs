use std::sync::Arc;

use anyhow::{Result, anyhow};
use assistant_tool::{ActionLog, ToolWorkingSet};
use gpui::{
    AnyWindowHandle, App, AsyncApp, Context, Entity, EventEmitter, SharedString, Task, WeakEntity, Subscription
};
use language_model::{
    ConfiguredModel, LanguageModel, LanguageModelRegistry, LanguageModelRequestTool,
    LanguageModelRequestMessage, LanguageModelToolResult, LanguageModelToolUseId, 
    Role, StopReason, TokenUsage,
};
use project::Project;
use prompt_store::PromptBuilder;

use crate::context::{AgentContext, AgentContextHandle, ContextLoadResult, LoadedContext};
use crate::conversation_mod::{
    Conversation, ConversationEvent, ConversationId, DetailedSummaryState, Message, 
    MessageId, CompletionEvent, CompletionService, DefaultCompletionService,
    ToolService, DefaultToolService, ContextService, DefaultContextService,
};
use crate::thread_store::SharedProjectContext;

/// Events emitted by the ConversationController
#[derive(Debug, Clone)]
pub enum ConversationEvent {
    MessageAdded(MessageId),
    MessageEdited(MessageId),
    MessageDeleted(MessageId),
    StreamedText {
        message_id: MessageId,
        text: String,
    },
    StreamedThinking {
        message_id: MessageId,
        text: String,
        signature: Option<String>,
    },
    ToolCall {
        tool_use_id: LanguageModelToolUseId,
        tool_name: Arc<str>,
        input: serde_json::Value,
    },
    ToolFinished {
        tool_use_id: LanguageModelToolUseId,
    },
    Stopped(Result<StopReason, Arc<anyhow::Error>>),
    Error(SharedString),
}

/// The ConversationController
pub struct ConversationController {
    pub conversation: Conversation,
    configured_model: Option<ConfiguredModel>,
    completion_service: Arc<DefaultCompletionService>,
    tool_service: Arc<DefaultToolService>,
    context_service: Arc<DefaultContextService>,
    is_generating: bool,
    generating_task: Option<Task<Result<()>>>,
    _subscriptions: Vec<Subscription>,
    project: Entity<Project>,
    tools: Entity<ToolWorkingSet>,
    action_log: Entity<ActionLog>,
    prompt_builder: Arc<PromptBuilder>,
    project_context: SharedProjectContext,
    current_message_id: Option<MessageId>,
    pending_tool_results: Vec<LanguageModelToolResult>,
}

impl EventEmitter<ConversationEvent> for ConversationController {}

impl ConversationController {
    pub fn new(
        conversation_id: Option<ConversationId>,
        project: Entity<Project>,
        tools: Entity<ToolWorkingSet>,
        action_log: Entity<ActionLog>,
        prompt_builder: Arc<PromptBuilder>,
        project_context: SharedProjectContext,
        cx: &mut Context<Self>,
    ) -> Self {
        let conversation = Conversation::new(conversation_id);
        let completion_service = Arc::new(DefaultCompletionService::new());
        let tool_service = Arc::new(DefaultToolService::new(tools.clone()));
        
        Self {
            conversation,
            completion_service,
            tool_service,
            project,
            tools,
            action_log,
            prompt_builder,
            project_context,
            configured_model: None,
            current_message_id: None,
            pending_tool_results: Vec::new(),
            is_generating: false,
            generating_task: None,
            _subscriptions: Vec::new(),
        }
    }
    
    pub fn conversation(&self) -> &Conversation {
        &self.conversation
    }
    
    pub fn conversation_mut(&mut self) -> &mut Conversation {
        &mut self.conversation
    }
    
    pub fn is_generating(&self) -> bool {
        self.is_generating
    }
    
    pub fn set_configured_model(&mut self, model: Option<ConfiguredModel>) {
        self.configured_model = model;
    }
    
    pub fn configured_model(&self) -> Option<&ConfiguredModel> {
        self.configured_model.as_ref()
    }
    
    pub fn insert_user_message(
        &mut self,
        text: impl Into<String>,
        loaded_context: ContextLoadResult,
        creases: Vec<MessageCrease>,
        cx: &mut Context<Self>,
    ) -> MessageId {
        let loaded_context = match loaded_context {
            ContextLoadResult::Loaded(ctx) => ctx,
            ContextLoadResult::Loading(_) => LoadedContext::default(),
        };
        
        let id = self.conversation.insert_user_message(
            text,
            loaded_context,
            creases,
        );
        
        cx.emit(ConversationEvent::MessageAdded(id));
        id
    }
    
    pub fn insert_assistant_message(
        &mut self,
        segments: Vec<MessageSegment>,
        cx: &mut Context<Self>,
    ) -> MessageId {
        let id = self.conversation.insert_assistant_message(segments);
        cx.emit(ConversationEvent::MessageAdded(id));
        id
    }
    
    pub fn edit_message(
        &mut self,
        id: MessageId,
        new_role: Role,
        new_segments: Vec<MessageSegment>,
        loaded_context: Option<LoadedContext>,
        cx: &mut Context<Self>,
    ) -> bool {
        let success = self.conversation.edit_message(
            id, 
            new_role, 
            new_segments, 
            loaded_context
        );
        
        if success {
            cx.emit(ConversationEvent::MessageEdited(id));
        }
        
        success
    }
    
    pub fn delete_message(
        &mut self,
        id: MessageId,
        cx: &mut Context<Self>,
    ) -> bool {
        let success = self.conversation.delete_message(id);
        
        if success {
            cx.emit(ConversationEvent::MessageDeleted(id));
        }
        
        success
    }
    
    pub fn truncate(
        &mut self,
        message_id: MessageId,
        cx: &mut Context<Self>,
    ) {
        self.conversation.truncate(message_id);
        // No need to emit an event as truncation just removes messages after the given one
    }
    
    pub fn cancel_completion(&mut self, window: Option<AnyWindowHandle>, cx: &mut Context<Self>) {
        if self.is_generating {
            self.is_generating = false;
            cx.emit(ConversationEvent::Stopped(Err(Arc::new(anyhow!("Canceled by user")))));
        }
    }
    
    pub fn send_to_model(
        &mut self,
        model: Arc<dyn LanguageModel>,
        window: Option<AnyWindowHandle>,
        cx: &mut Context<Self>,
    ) {
        if self.is_generating {
            // Already generating, cancel first
            self.cancel_completion(window.clone(), cx);
        }
        
        // Create a new assistant message to receive the response
        let message_id = self.insert_assistant_message(Vec::new(), cx);
        self.current_message_id = Some(message_id);
        
        // Set state to generating
        self.is_generating = true;
        
        // Get available tools
        let available_tools = self.tool_service.available_tools(model.clone(), cx);
        
        // Get self as weak entity for tasks
        let controller = cx.entity_id().downgrade();
        
        // Stream completion
        let completion_service = self.completion_service.clone();
        let tool_service = self.tool_service.clone();
        let conversation = self.conversation.clone();
        
        let _completion_task = cx.spawn(async move |cx| {
            // Start streaming
            let mut completion_stream = completion_service
                .stream_completion(
                    &conversation,
                    model.clone(),
                    available_tools,
                    message_id,
                    window.clone(),
                    cx,
                )
                .await;
            
            while let Some(result) = completion_stream.next().await {
                if let Some(controller) = controller.upgrade() {
                    match result {
                        Ok(CompletionEvent::TextChunk(text)) => {
                            controller.update(cx, |this, cx| {
                                // Add text to the current assistant message
                                if let Some(message) = this.conversation.message_mut(message_id) {
                                    message.push_text(&text);
                                }
                                
                                // Emit event
                                cx.emit(ConversationEvent::StreamedText {
                                    message_id,
                                    text,
                                });
                            })?;
                        }
                        Ok(CompletionEvent::ThinkingChunk { text, signature }) => {
                            controller.update(cx, |this, cx| {
                                // Add thinking to the current assistant message
                                if let Some(message) = this.conversation.message_mut(message_id) {
                                    message.push_thinking(&text, signature.clone());
                                }
                                
                                // Emit event
                                cx.emit(ConversationEvent::StreamedThinking {
                                    message_id,
                                    text,
                                    signature,
                                });
                            })?;
                        }
                        Ok(CompletionEvent::ToolCall { tool_use_id, tool_name, input }) => {
                            controller.update(cx, |this, cx| {
                                // Emit event
                                cx.emit(ConversationEvent::ToolCall {
                                    tool_use_id: tool_use_id.clone(),
                                    tool_name: tool_name.clone(),
                                    input: input.clone(),
                                });
                                
                                // Execute the tool
                                let window_handle = window.clone();
                                let tool_service = this.tool_service.clone();
                                let request = this.completion_service.prepare_request(
                                    &this.conversation,
                                    model.clone(),
                                    this.tool_service.available_tools(model.clone(), cx),
                                );
                                
                                cx.spawn(async move |cx| {
                                    let result = tool_service.run_tool(
                                        tool_use_id.clone(),
                                        tool_name,
                                        input,
                                        message_id,
                                        Arc::new(request),
                                        model.clone(),
                                        window_handle,
                                        cx,
                                    ).await;
                                    
                                    if let Some(controller) = controller.upgrade() {
                                        controller.update(cx, |this, cx| {
                                            match result {
                                                Ok(tool_result) => {
                                                    this.pending_tool_results.push(tool_result);
                                                    cx.emit(ConversationEvent::ToolFinished {
                                                        tool_use_id,
                                                    });
                                                }
                                                Err(err) => {
                                                    // Emit error event
                                                    cx.emit(ConversationEvent::Error(
                                                        format!("Tool execution error: {}", err).into()
                                                    ));
                                                }
                                            }
                                        })?;
                                    }
                                    
                                    Ok(())
                                }).detach();
                            })?;
                        }
                        Ok(CompletionEvent::Stopped(result)) => {
                            controller.update(cx, |this, cx| {
                                this.is_generating = false;
                                
                                // Process token usage if available
                                if let Ok(stop_reason) = &result {
                                    if let Some(token_usage) = stop_reason.token_usage() {
                                        this.conversation.update_token_usage(token_usage);
                                    }
                                }
                                
                                // Emit stopped event
                                cx.emit(ConversationEvent::Stopped(result));
                            })?;
                            
                            break;
                        }
                        Err(err) => {
                            controller.update(cx, |this, cx| {
                                this.is_generating = false;
                                
                                // Emit error event
                                cx.emit(ConversationEvent::Error(
                                    format!("Completion error: {}", err).into()
                                ));
                                
                                // Also emit stopped with error
                                cx.emit(ConversationEvent::Stopped(
                                    Err(Arc::new(anyhow!(err.to_string())))
                                ));
                            })?;
                            
                            break;
                        }
                    }
                } else {
                    // Controller was dropped, exit the loop
                    break;
                }
            }
            
            Ok(())
        });
    }
    
    pub fn project(&self) -> &Entity<Project> {
        &self.project
    }
    
    pub fn tools(&self) -> &Entity<ToolWorkingSet> {
        &self.tools
    }
    
    pub fn action_log(&self) -> &Entity<ActionLog> {
        &self.action_log
    }
    
    pub fn perform_continuous_thinking(
        &mut self,
        window: Option<AnyWindowHandle>,
        cx: &mut Context<Self>,
    ) {
        if self.is_generating {
            cx.emit(ConversationEvent::Error("Cannot start continuous thinking while already generating".into()));
            return;
        }
        
        if let Some(model) = self.get_model_for_request(cx) {
            // Create a special prompt for analysis
            let mut request = self.completion_service.prepare_request(
                &self.conversation,
                model.clone(),
                self.tool_service.available_tools(model.clone(), cx),
            );
            
            // Add our special continuous thinking instruction at the end
            request.messages.push(LanguageModelRequestMessage {
                role: Role::System,
                content: vec![MessageContent::Text(
                    "Analyze the conversation above. Has the user's goal been achieved? If not, \
                    continue the conversation with a different approach. Focus on solving the \
                    original problem in a new way. Do NOT summarize the conversation or explain \
                    what you're doing - just continue with your new approach.".into()
                )],
                cache: false,
            });
            
            // Create a new assistant message to receive the response
            let message_id = self.insert_assistant_message(Vec::new(), cx);
            self.current_message_id = Some(message_id);
            
            // Add a thinking segment to show processing
            cx.emit(ConversationEvent::StreamedThinking {
                message_id,
                text: "Analyzing conversation and trying a new approach...".to_string(),
                signature: Some("continuous_thinking".to_string()),
            });
            
            // Track token usage
            let token_usage_task = self.completion_service.calculate_token_usage(&request, &model);
            
            // Set state to generating
            self.is_generating = true;
            
            // Send to model with fallback mechanism
            let fallback_task = cx.spawn({
                let weak_controller = cx.entity().downgrade();
                let model_clone = model.clone();
                let window_clone = window.clone();
                
                async move |cx| {
                    // Try to calculate token usage and update if successful
                    if let Ok(token_usage) = token_usage_task.await {
                        if let Some(controller) = weak_controller.upgrade() {
                            controller.update(cx, |this, _cx| {
                                this.conversation.update_token_usage(token_usage);
                            })?;
                        }
                    }
                    
                    // If initial attempt fails, try with fallback model
                    if let Some(controller) = weak_controller.upgrade() {
                        let result = controller.update(cx, |this, cx| {
                            if let Some(message_id) = this.current_message_id {
                                if let Some(message) = this.conversation.message_mut(message_id) {
                                    // Check if we already have content - if not, something went wrong
                                    let has_content = message.segments.iter().any(|s| {
                                        matches!(s, MessageSegment::Text(t) if !t.is_empty())
                                    });
                                    
                                    if !has_content {
                                        // Try a fallback model if available
                                        let registry = LanguageModelRegistry::global(cx);
                                        let fallback_models = registry.read_with(cx, |registry, _| {
                                            registry.models().filter(|m| m.model.id() != model_clone.id()).collect::<Vec<_>>()
                                        });
                                        
                                        if !fallback_models.is_empty() {
                                            cx.emit(ConversationEvent::StreamedThinking {
                                                message_id,
                                                text: "\nFallback: Trying alternative model...".to_string(),
                                                signature: Some("continuous_thinking".to_string()),
                                            });
                                            
                                            // Use first available model different from current
                                            return Some(fallback_models[0].model.clone());
                                        }
                                    }
                                }
                            }
                            None
                        })?;
                        
                        if let Some(fallback_model) = result {
                            // Try with fallback model
                            controller.update(cx, |this, cx| {
                                this.stream_completion_with_request(
                                    request.clone(),
                                    fallback_model,
                                    message_id,
                                    window_clone,
                                    cx,
                                );
                            })?;
                        }
                    }
                    
                    Ok(())
                }
            });
            
            // Detach the fallback task so it runs independently
            fallback_task.detach_and_log_err(cx);
            
            // Send to model
            self.stream_completion_with_request(request, model, message_id, window, cx);
        } else {
            cx.emit(ConversationEvent::Error("No language model is configured".into()));
        }
    }
    
    pub fn get_model_for_request(&self, cx: &Context<Self>) -> Option<Arc<dyn LanguageModel>> {
        if let Some(configured_model) = &self.configured_model {
            LanguageModelRegistry::global(cx).read_with(cx, |registry, _| {
                registry.get_model(&configured_model.provider_id, &configured_model.model_id)
            })
        } else {
            LanguageModelRegistry::global(cx).read_with(cx, |registry, _| {
                registry.default_model().map(|cm| cm.model)
            })
        }
    }
    
    fn stream_completion_with_request(
        &mut self,
        request: LanguageModelRequest,
        model: Arc<dyn LanguageModel>,
        message_id: MessageId,
        window: Option<AnyWindowHandle>,
        cx: &mut Context<Self>,
    ) {
        // Get self as weak entity for tasks
        let controller = cx.entity_id().downgrade();
        
        // Stream completion
        let completion_service = self.completion_service.clone();
        let tool_service = self.tool_service.clone();
        
        let _completion_task = cx.spawn(async move |cx| {
            // Start streaming directly with request
            let mut completion_stream = completion_service
                .stream_completion_with_request(
                    request,
                    model.clone(),
                    message_id,
                    window.clone(),
                    cx,
                )
                .await;
            
            while let Some(result) = completion_stream.next().await {
                if let Some(controller) = controller.upgrade() {
                    match result {
                        Ok(CompletionEvent::TextChunk(text)) => {
                            controller.update(cx, |this, cx| {
                                // Add text to the current assistant message
                                if let Some(message) = this.conversation.message_mut(message_id) {
                                    message.push_text(&text);
                                }
                                
                                // Emit event
                                cx.emit(ConversationEvent::StreamedText {
                                    message_id,
                                    text,
                                });
                            })?;
                        }
                        Ok(CompletionEvent::ThinkingChunk { text, signature }) => {
                            controller.update(cx, |this, cx| {
                                // Add thinking to the current assistant message
                                if let Some(message) = this.conversation.message_mut(message_id) {
                                    message.push_thinking(&text, signature.clone());
                                }
                                
                                // Emit event
                                cx.emit(ConversationEvent::StreamedThinking {
                                    message_id,
                                    text,
                                    signature,
                                });
                            })?;
                        }
                        Ok(CompletionEvent::ToolCall { tool_use_id, tool_name, input }) => {
                            controller.update(cx, |this, cx| {
                                // Emit event
                                cx.emit(ConversationEvent::ToolCall {
                                    tool_use_id: tool_use_id.clone(),
                                    tool_name: tool_name.clone(),
                                    input: input.clone(),
                                });
                                
                                // Execute the tool
                                let window_handle = window.clone();
                                let tool_service = this.tool_service.clone();
                                
                                cx.spawn(async move |cx| {
                                    let result = tool_service.run_tool(
                                        tool_use_id.clone(),
                                        tool_name,
                                        input,
                                        message_id,
                                        Arc::new(request),
                                        model.clone(),
                                        window_handle,
                                        cx,
                                    ).await;
                                    
                                    if let Some(controller) = controller.upgrade() {
                                        controller.update(cx, |this, cx| {
                                            match result {
                                                Ok(tool_result) => {
                                                    this.pending_tool_results.push(tool_result);
                                                    cx.emit(ConversationEvent::ToolFinished {
                                                        tool_use_id,
                                                    });
                                                }
                                                Err(err) => {
                                                    // Emit error event
                                                    cx.emit(ConversationEvent::Error(
                                                        format!("Tool execution error: {}", err).into()
                                                    ));
                                                }
                                            }
                                        })?;
                                    }
                                    
                                    Ok(())
                                }).detach();
                            })?;
                        }
                        Ok(CompletionEvent::Stopped(result)) => {
                            controller.update(cx, |this, cx| {
                                this.is_generating = false;
                                
                                // Process token usage if available
                                if let Ok(stop_reason) = &result {
                                    if let Some(token_usage) = stop_reason.token_usage() {
                                        this.conversation.update_token_usage(token_usage);
                                    }
                                }
                                
                                // Emit stopped event
                                cx.emit(ConversationEvent::Stopped(result));
                            })?;
                            
                            break;
                        }
                        Err(err) => {
                            controller.update(cx, |this, cx| {
                                this.is_generating = false;
                                
                                // Emit error event
                                cx.emit(ConversationEvent::Error(
                                    format!("Completion error: {}", err).into()
                                ));
                                
                                // Also emit stopped with error
                                cx.emit(ConversationEvent::Stopped(
                                    Err(Arc::new(anyhow!(err.to_string())))
                                ));
                            })?;
                            
                            break;
                        }
                    }
                } else {
                    // Controller was dropped, exit the loop
                    break;
                }
            }
            
            Ok(())
        });
    }
    
    /// Set the conversation directly (used for deserialization)
    pub fn set_conversation(&mut self, conversation: Conversation) {
        self.conversation = conversation;
    }
}

impl Clone for ConversationController {
    fn clone(&self) -> Self {
        Self {
            conversation: self.conversation.clone(),
            completion_service: self.completion_service.clone(),
            tool_service: self.tool_service.clone(),
            project: self.project.clone(),
            tools: self.tools.clone(),
            action_log: self.action_log.clone(),
            prompt_builder: self.prompt_builder.clone(),
            project_context: self.project_context.clone(),
            configured_model: self.configured_model.clone(),
            current_message_id: self.current_message_id,
            pending_tool_results: self.pending_tool_results.clone(),
            is_generating: self.is_generating,
            generating_task: self.generating_task.clone(),
            _subscriptions: self._subscriptions.clone(),
        }
    }
} 