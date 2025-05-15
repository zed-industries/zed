use std::sync::Arc;

use anyhow::{Result, anyhow};
use assistant_tool::{ActionLog, ToolWorkingSet};
use gpui::{App, Context, Entity, EventEmitter, Task, AnyWindowHandle, WeakEntity};
use language_model::{
    LanguageModel, LanguageModelRegistry, ConfiguredModel, StopReason, TokenUsage, Role,
    LanguageModelToolUseId, LanguageModelToolResult,
};
use project::Project;
use prompt_store::PromptBuilder;

use crate::conversation_mod::{
    Conversation, ConversationController, ConversationEvent, ConversationId, 
    CompletionService, DefaultCompletionService, ToolService, DefaultToolService,
    Message as ConversationMessage, MessageId as ConversationMessageId,
    MessageSegment as ConversationMessageSegment,
};
use crate::context::{AgentContextHandle, ContextLoadResult, LoadedContext};
use crate::thread_store::SharedProjectContext;
use crate::thread::{Thread, ThreadId, ThreadEvent, Message, MessageId, MessageSegment};

/// Adapter for interfacing the new conversation system with the existing Thread system
pub struct ConversationAdapter {
    controller: Entity<ConversationController>,
    adapter_context: Arc<AdapterContext>,
}

struct AdapterContext {
    project: Entity<Project>,
    tools: Entity<ToolWorkingSet>,
    action_log: Entity<ActionLog>,
    prompt_builder: Arc<PromptBuilder>,
    project_context: SharedProjectContext,
}

impl ConversationAdapter {
    pub fn new(
        thread_id: Option<ThreadId>,
        project: Entity<Project>,
        tools: Entity<ToolWorkingSet>,
        action_log: Entity<ActionLog>,
        prompt_builder: Arc<PromptBuilder>,
        project_context: SharedProjectContext,
        cx: &mut App,
    ) -> Self {
        let conversation_id = thread_id.map(|id| ConversationId::from(id.to_string().as_str()));
        
        let adapter_context = Arc::new(AdapterContext {
            project: project.clone(),
            tools: tools.clone(),
            action_log: action_log.clone(),
            prompt_builder: prompt_builder.clone(),
            project_context: project_context.clone(),
        });
        
        let controller = cx.new_entity(|cx| {
            ConversationController::new(
                conversation_id,
                project.clone(),
                tools.clone(),
                action_log.clone(),
                prompt_builder.clone(),
                project_context.clone(),
                cx,
            )
        });
        
        Self {
            controller,
            adapter_context,
        }
    }
    
    pub fn to_thread(&self, cx: &App) -> Thread {
        let conversation = self.controller.read(cx).conversation();
        
        // Create a new thread from the conversation
        let thread_id = ThreadId::from_string(conversation.id().to_string());
        let mut thread = Thread::new(thread_id);
        
        // Copy entity references
        thread.project = Some(self.adapter_context.project.clone());
        thread.tools = Some(self.adapter_context.tools.clone());
        thread.action_log = Some(self.adapter_context.action_log.clone());
        thread.prompt_builder = self.adapter_context.prompt_builder.clone();
        thread.project_context = self.adapter_context.project_context.clone();
        
        // Set configured model if available
        if let Some(model) = self.controller.read(cx).configured_model() {
            thread.configured_model = Some(model.clone());
        }
        
        // Convert messages
        for message in conversation.messages() {
            // Convert the message
            let thread_message = self.convert_message_to_thread(message);
            
            // Add message to thread
            thread.messages.push(thread_message);
        }
        
        // Copy token usage
        thread.cumulative_token_usage = conversation.cumulative_token_usage();
        
        thread
    }
    
    fn convert_message_to_thread(&self, message: &ConversationMessage) -> Message {
        // Convert message segments
        let segments = message.segments.iter().map(|segment| {
            match segment {
                ConversationMessageSegment::Text(text) => {
                    MessageSegment::Text(text.clone())
                },
                ConversationMessageSegment::Thinking { text, signature } => {
                    MessageSegment::Thinking {
                        text: text.clone(),
                        signature: signature.clone(),
                    }
                },
                ConversationMessageSegment::RedactedThinking(bytes) => {
                    MessageSegment::RedactedThinking(bytes.clone())
                },
            }
        }).collect();
        
        Message {
            id: MessageId(message.id.0),
            role: message.role.clone(),
            segments,
            loaded_context: message.loaded_context.clone(),
            creases: message.creases.clone(),
            timestamp: message.timestamp,
        }
    }
    
    pub fn from_thread(thread: &Thread, cx: &mut App) -> Result<Self> {
        // Create a new adapter with the thread ID
        let adapter = Self::new(
            Some(thread.id),
            thread.project.clone().ok_or_else(|| anyhow!("Thread has no project"))?,
            thread.tools.clone().ok_or_else(|| anyhow!("Thread has no tools"))?,
            thread.action_log.clone().ok_or_else(|| anyhow!("Thread has no action log"))?,
            thread.prompt_builder.clone(),
            thread.project_context.clone(),
            cx,
        );
        
        // Set configured model if available
        if let Some(model) = &thread.configured_model {
            adapter.controller.update(cx, |controller, cx| {
                controller.set_configured_model(Some(model.clone()));
            });
        }
        
        // Convert messages from thread to conversation
        for message in &thread.messages {
            let loaded_context = message.loaded_context.clone();
            
            // Convert message segments
            let segments: Vec<ConversationMessageSegment> = message.segments.iter().map(|segment| {
                match segment {
                    MessageSegment::Text(text) => {
                        ConversationMessageSegment::Text(text.clone())
                    },
                    MessageSegment::Thinking { text, signature } => {
                        ConversationMessageSegment::Thinking {
                            text: text.clone(),
                            signature: signature.clone(),
                        }
                    },
                    MessageSegment::RedactedThinking(bytes) => {
                        ConversationMessageSegment::RedactedThinking(bytes.clone())
                    },
                }
            }).collect();
            
            // Add message to conversation
            adapter.controller.update(cx, |controller, cx| {
                controller.edit_message(
                    ConversationMessageId(message.id.0),
                    message.role.clone(),
                    segments,
                    Some(loaded_context),
                    cx,
                );
            });
        }
        
        Ok(adapter)
    }
    
    pub fn insert_user_message(
        &self,
        text: impl Into<String>,
        loaded_context: ContextLoadResult,
        creases: Vec<crate::context::MessageCrease>,
        cx: &mut App,
    ) -> MessageId {
        let id = self.controller.update(cx, |controller, cx| {
            controller.insert_user_message(text, loaded_context, creases, cx)
        });
        
        MessageId(id.0)
    }
    
    pub fn send_to_model(
        &self,
        model: Arc<dyn LanguageModel>,
        window: Option<AnyWindowHandle>,
        cx: &mut App,
    ) {
        self.controller.update(cx, |controller, cx| {
            controller.send_to_model(model, window, cx)
        });
    }
    
    pub fn insert_assistant_message(
        &self,
        segments: Vec<MessageSegment>,
        cx: &mut App,
    ) -> MessageId {
        // Convert segments
        let conv_segments = segments.into_iter().map(|segment| {
            match segment {
                MessageSegment::Text(text) => {
                    ConversationMessageSegment::Text(text)
                }
                MessageSegment::Thinking { text, signature } => {
                    ConversationMessageSegment::Thinking { text, signature }
                }
                MessageSegment::RedactedThinking(bytes) => {
                    ConversationMessageSegment::RedactedThinking(bytes)
                }
            }
        }).collect();
        
        let id = self.controller.update(cx, |controller, cx| {
            controller.insert_assistant_message(conv_segments, cx)
        });
        
        MessageId(id.0)
    }
    
    pub fn edit_message(
        &self,
        id: MessageId,
        new_role: Role,
        new_segments: Vec<MessageSegment>,
        loaded_context: Option<LoadedContext>,
        cx: &mut App,
    ) -> bool {
        // Convert segments
        let conv_segments = new_segments.into_iter().map(|segment| {
            match segment {
                MessageSegment::Text(text) => {
                    ConversationMessageSegment::Text(text)
                }
                MessageSegment::Thinking { text, signature } => {
                    ConversationMessageSegment::Thinking { text, signature }
                }
                MessageSegment::RedactedThinking(bytes) => {
                    ConversationMessageSegment::RedactedThinking(bytes)
                }
            }
        }).collect();
        
        self.controller.update(cx, |controller, cx| {
            controller.edit_message(
                ConversationMessageId(id.0),
                new_role,
                conv_segments,
                loaded_context,
                cx,
            )
        })
    }
    
    pub fn delete_message(
        &self,
        id: MessageId,
        cx: &mut App,
    ) -> bool {
        self.controller.update(cx, |controller, cx| {
            controller.delete_message(
                ConversationMessageId(id.0),
                cx,
            )
        })
    }
    
    pub fn truncate(
        &self,
        message_id: MessageId,
        cx: &mut App,
    ) {
        self.controller.update(cx, |controller, cx| {
            controller.truncate(
                ConversationMessageId(message_id.0),
                cx,
            )
        });
    }
    
    pub fn cancel_completion(
        &self,
        window: Option<AnyWindowHandle>,
        cx: &mut App,
    ) {
        self.controller.update(cx, |controller, cx| {
            controller.cancel_completion(window, cx)
        });
    }
    
    pub fn is_generating(&self, cx: &App) -> bool {
        self.controller.read(cx).is_generating()
    }
    
    pub fn project(&self) -> &Entity<Project> {
        &self.adapter_context.project
    }
    
    pub fn tools(&self) -> &Entity<ToolWorkingSet> {
        &self.adapter_context.tools
    }
    
    pub fn action_log(&self) -> &Entity<ActionLog> {
        &self.adapter_context.action_log
    }
    
    pub fn prompt_builder(&self) -> &Arc<PromptBuilder> {
        &self.adapter_context.prompt_builder
    }
    
    pub fn project_context(&self) -> &SharedProjectContext {
        &self.adapter_context.project_context
    }
    
    pub fn set_configured_model(&self, model: Option<ConfiguredModel>, cx: &mut App) {
        self.controller.update(cx, |controller, cx| {
            controller.set_configured_model(model);
        });
    }
    
    pub fn configured_model(&self, cx: &App) -> Option<ConfiguredModel> {
        self.controller.read(cx).configured_model().cloned()
    }
    
    pub fn token_usage(&self, cx: &App) -> TokenUsage {
        self.controller.read(cx).conversation().cumulative_token_usage()
    }
    
    pub fn id(&self, cx: &App) -> ThreadId {
        let conversation_id = self.controller.read(cx).conversation().id().to_string();
        ThreadId::from_string(conversation_id)
    }
    
    pub fn as_entity(&self) -> &Entity<ConversationController> {
        &self.controller
    }
    
    pub fn perform_continuous_thinking(
        &self,
        window: Option<AnyWindowHandle>,
        cx: &mut App,
    ) {
        self.controller.update(cx, |controller, cx| {
            controller.perform_continuous_thinking(window, cx);
        });
        
        // Set up subscription to bridge events from controller to thread events
        let subscription = self.controller.subscribe(
            self.controller.entity_id(),
            move |adapter, controller, event, cx| {
                match event {
                    ConversationEvent::Error(error) => {
                        cx.emit(ThreadEvent::Error(error.clone()));
                    },
                    ConversationEvent::ToolCall { tool_use_id, tool_name, input } => {
                        cx.emit(ThreadEvent::ToolCall {
                            tool_use_id: tool_use_id.clone(),
                            tool_name: tool_name.clone(),
                            input: input.clone(),
                        });
                    },
                    ConversationEvent::StreamedThinking { message_id, text, .. } => {
                        cx.emit(ThreadEvent::StreamedAssistantThinking(
                            MessageId(message_id.0),
                            text.clone(),
                        ));
                    },
                    ConversationEvent::StreamedText { message_id, text } => {
                        cx.emit(ThreadEvent::StreamedAssistantMessage(
                            MessageId(message_id.0),
                            text.clone(),
                        ));
                    },
                    _ => {},
                }
            },
            cx,
        );
        
        // Store subscription so it doesn't get dropped
        cx.update_global(|global_state: &mut crate::thread_store::ThreadStore, _| {
            global_state
                .continuous_thinking_subscriptions
                .retain(|sub| !sub.is_disposed());
                
            global_state
                .continuous_thinking_subscriptions
                .push(subscription);
        });
    }
    
    /// Save the conversation using the thread store
    pub fn save(&self, cx: &mut App) -> Task<Result<()>> {
        // Convert to thread and save
        let thread = self.to_thread(cx);
        let thread_id = thread.id;
        
        let task = cx.spawn(async move |mut cx| {
            // Get thread store
            let thread_store = cx.global::<crate::thread_store::ThreadStore>();
            
            // Create a serialized thread
            let thread_data = cx.read_global(move |thread_store: &crate::thread_store::ThreadStore, _| {
                thread.serialize(thread_store, &cx)
            })?;
            
            // Save using the database
            let database = crate::thread_store::ThreadsDatabase::global(&mut cx).await?;
            database.save_thread(thread_id, thread_data).await?;
            
            Ok(())
        });
        
        task
    }
    
    /// Load a conversation from the thread store
    pub fn load(
        thread_id: crate::thread::ThreadId,
        project: Entity<Project>,
        tools: Entity<ToolWorkingSet>,
        action_log: Entity<ActionLog>,
        prompt_builder: Arc<PromptBuilder>,
        project_context: SharedProjectContext,
        cx: &mut App,
    ) -> Task<Result<Self>> {
        let task = cx.spawn(async move |mut cx| {
            // Get the database
            let database = crate::thread_store::ThreadsDatabase::global(&mut cx).await?;
            
            // Load the thread data
            let thread_data = database.try_find_thread(thread_id).await?
                .ok_or_else(|| anyhow!("Thread not found: {}", thread_id))?;
            
            // Create a conversation adapter from the thread data
            let adapter = Self::new(
                Some(thread_id),
                project,
                tools,
                action_log,
                prompt_builder,
                project_context,
                &mut cx,
            );
            
            // Deserialize the thread data into the conversation
            adapter.controller.update(&mut cx, |controller, cx| {
                let conversation_id = ConversationId::from(thread_id.to_string().as_str());
                controller.set_conversation(Conversation::from_serialized(conversation_id, thread_data));
            });
            
            Ok(adapter)
        });
        
        task
    }
    
    /// Check if the adapter system can handle this thread
    pub fn can_handle_thread(thread: &Thread, cx: &App) -> bool {
        // For now, we don't have any specific compatibility limits
        // In the future, we might want to check for certain features or thread states
        // that aren't yet supported by the new system
        true
    }
}

impl EventEmitter<ThreadEvent> for ConversationAdapter {
    fn subscribe_to_events(
        &self,
        subscriber_id: gpui::SubscriberId,
        callback: Box<dyn FnMut(&ConversationAdapter, &ThreadEvent) + 'static>,
        cx: &mut gpui::WindowContext,
    ) -> gpui::Subscription {
        let subscriber_id_clone = subscriber_id;
        let adapter = self.clone();
        
        // Subscribe to conversation events and map them to thread events
        let subscription = self.controller.subscribe(
            subscriber_id,
            move |_, controller, event, cx| {
                let thread_event = match event {
                    ConversationEvent::MessageAdded(id) => {
                        ThreadEvent::MessageAdded(MessageId(id.0))
                    },
                    ConversationEvent::MessageEdited(id) => {
                        ThreadEvent::MessageEdited(MessageId(id.0))
                    },
                    ConversationEvent::MessageDeleted(id) => {
                        ThreadEvent::MessageDeleted(MessageId(id.0))
                    },
                    ConversationEvent::StreamedText { message_id, text } => {
                        ThreadEvent::ReceivedText {
                            message_id: MessageId(message_id.0),
                            text: text.clone(),
                        }
                    },
                    ConversationEvent::StreamedThinking { message_id, text, signature } => {
                        ThreadEvent::ReceivedThinking {
                            message_id: MessageId(message_id.0),
                            text: text.clone(),
                            signature: signature.clone(),
                        }
                    },
                    ConversationEvent::ToolCall { tool_use_id, tool_name, input } => {
                        ThreadEvent::ToolCalled {
                            tool_use_id: tool_use_id.clone(),
                            tool_name: tool_name.clone(),
                            params: input.clone(),
                        }
                    },
                    ConversationEvent::ToolFinished { tool_use_id } => {
                        ThreadEvent::ToolFinished {
                            tool_use_id: tool_use_id.clone(),
                        }
                    },
                    ConversationEvent::Stopped(result) => {
                        ThreadEvent::Stopped(result.clone())
                    },
                    ConversationEvent::Error(message) => {
                        ThreadEvent::ReceivedError(message.clone())
                    },
                };
                
                callback(&adapter, &thread_event);
            },
            cx,
        );
        
        subscription
    }
}

impl Clone for ConversationAdapter {
    fn clone(&self) -> Self {
        Self {
            controller: self.controller.clone(),
            adapter_context: self.adapter_context.clone(),
        }
    }
} 