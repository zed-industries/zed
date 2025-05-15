use std::fmt::Write as _;
use std::ops::Range;
use std::sync::Arc;
use std::time::Instant;
use std::collections::HashMap;

use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use editor::display_map::CreaseMetadata;
use gpui::{AppContext, AsyncApp, SharedString};
use language_model::{
    LanguageModelRequestMessage, LanguageModelRequestTool, LanguageModelToolResult,
    LanguageModelToolUseId, MessageContent, Role, TokenUsage, ConfiguredModel,
};
use project::ProjectSnapshot;
use serde::{Deserialize, Serialize};
use util::post_inc;

use crate::context::{AgentContextHandle, LoadedContext};
use crate::thread_store::{
    SerializedMessage, SerializedMessageSegment, SerializedToolResult,
    SerializedToolUse, SerializedThread, SerializedCrease, SerializedLanguageModel,
};

/// The ID of the conversation.
#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Serialize, Deserialize,
)]
pub struct ConversationId(Arc<str>);

impl ConversationId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4().to_string().into())
    }
}

impl std::fmt::Display for ConversationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for ConversationId {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

/// The ID of the user prompt that initiated a request.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Serialize, Deserialize)]
pub struct PromptId(Arc<str>);

impl PromptId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4().to_string().into())
    }
}

impl std::fmt::Display for PromptId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct MessageId(pub(crate) usize);

impl MessageId {
    fn post_inc(&mut self) -> Self {
        Self(post_inc(&mut self.0))
    }
}

/// Stored information that can be used to resurrect a context crease when creating an editor for a past message.
#[derive(Clone, Debug)]
pub struct MessageCrease {
    pub range: Range<usize>,
    pub metadata: CreaseMetadata,
    /// None for a deserialized message, Some otherwise.
    pub context: Option<AgentContextHandle>,
}

/// A message in a conversation.
#[derive(Debug, Clone)]
pub struct Message {
    pub id: MessageId,
    pub role: Role,
    pub segments: Vec<MessageSegment>,
    pub loaded_context: LoadedContext,
    pub creases: Vec<MessageCrease>,
    pub timestamp: DateTime<Utc>,
}

impl Message {
    /// Returns whether the message contains any meaningful text that should be displayed
    /// The model sometimes runs tool without producing any text or just a marker
    pub fn should_display_content(&self) -> bool {
        self.segments.iter().any(|segment| segment.should_display())
    }

    pub fn push_thinking(&mut self, text: &str, signature: Option<String>) {
        if let Some(MessageSegment::Thinking {
            text: segment,
            signature: current_signature,
        }) = self.segments.last_mut()
        {
            if let Some(signature) = signature {
                *current_signature = Some(signature);
            }
            segment.push_str(text);
        } else {
            self.segments.push(MessageSegment::Thinking {
                text: text.to_string(),
                signature,
            });
        }
    }

    pub fn push_text(&mut self, text: &str) {
        if let Some(MessageSegment::Text(segment)) = self.segments.last_mut() {
            segment.push_str(text);
        } else {
            self.segments.push(MessageSegment::Text(text.to_string()));
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();

        if !self.loaded_context.text.is_empty() {
            result.push_str(&self.loaded_context.text);
        }

        for segment in &self.segments {
            match segment {
                MessageSegment::Text(text) => result.push_str(text),
                MessageSegment::Thinking { text: content, .. } => {
                    result.push_str("<think>\n");
                    result.push_str(content);
                    result.push_str("\n</think>");
                }
                MessageSegment::RedactedThinking(_) => {}
            }
        }

        result
    }
    
    /// Serialize the message for storage
    pub fn serialize(&self, tool_uses: &[SerializedToolUse], tool_results: &[SerializedToolResult]) -> SerializedMessage {
        SerializedMessage {
            id: self.id,
            role: self.role,
            segments: self.segments.iter().map(|s| s.serialize()).collect(),
            context: self.loaded_context.text.clone(),
            creases: self.creases.iter().map(|crease| 
                SerializedCrease {
                    start: crease.range.start,
                    end: crease.range.end,
                    icon_path: crease.metadata.icon_path.clone(),
                    label: crease.metadata.label.clone(),
                }
            ).collect(),
            tool_uses: tool_uses.to_vec(),
            tool_results: tool_results.to_vec(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageSegment {
    Text(String),
    Thinking {
        text: String,
        signature: Option<String>,
    },
    RedactedThinking(Vec<u8>),
}

impl MessageSegment {
    pub fn should_display(&self) -> bool {
        match self {
            Self::Text(text) => !text.is_empty(),
            Self::Thinking { text, .. } => !text.is_empty(),
            Self::RedactedThinking(_) => false,
        }
    }
    
    /// Serialize the message segment for storage
    fn serialize(&self) -> SerializedMessageSegment {
        match self {
            Self::Text(text) => SerializedMessageSegment::Text(text.clone()),
            Self::Thinking { text, signature } => SerializedMessageSegment::Thinking {
                text: text.clone(),
                signature: signature.clone(),
            },
            Self::RedactedThinking(bytes) => SerializedMessageSegment::RedactedThinking(bytes.clone()),
        }
    }
}

/// Core model for a conversation 
#[derive(Debug, Clone)]
pub struct Conversation {
    id: ConversationId,
    updated_at: DateTime<Utc>,
    title: Option<SharedString>,
    messages: Vec<Message>,
    next_message_id: MessageId,
    prompt_id: PromptId,
    request_token_usage: Vec<TokenUsage>,
    cumulative_token_usage: TokenUsage,
    last_received_chunk_at: Option<Instant>,
    detailed_summary: Option<DetailedSummaryState>,
}

/// Detailed summary state, compatible with Thread's implementation
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum DetailedSummaryState {
    #[default]
    NotGenerated,
    Generating {
        message_id: MessageId,
    },
    Generated {
        text: SharedString,
        message_id: MessageId,
    },
}

impl Conversation {
    /// Create a new conversation
    pub fn new(id: Option<ConversationId>) -> Self {
        Self {
            id: id.unwrap_or_else(ConversationId::new),
            updated_at: Utc::now(),
            title: None,
            messages: Vec::new(),
            next_message_id: MessageId(0),
            prompt_id: PromptId::new(),
            request_token_usage: Vec::new(),
            cumulative_token_usage: TokenUsage::default(),
            last_received_chunk_at: None,
            detailed_summary: None,
        }
    }
    
    /// Create a conversation from serialized data
    pub fn from_serialized(id: ConversationId, serialized: SerializedThread) -> Self {
        let next_message_id = MessageId(
            serialized
                .messages
                .last()
                .map(|message| message.id.0 + 1)
                .unwrap_or(0),
        );
        
        let mut messages = Vec::with_capacity(serialized.messages.len());
        
        for message in serialized.messages {
            messages.push(Message {
                id: message.id,
                role: message.role,
                segments: message
                    .segments
                    .into_iter()
                    .map(|segment| match segment {
                        SerializedMessageSegment::Text { text } => MessageSegment::Text(text),
                        SerializedMessageSegment::Thinking { text, signature } => {
                            MessageSegment::Thinking { text, signature }
                        }
                        SerializedMessageSegment::RedactedThinking { data } => {
                            MessageSegment::RedactedThinking(data)
                        }
                    })
                    .collect(),
                loaded_context: LoadedContext {
                    contexts: Vec::new(),
                    text: message.context,
                    images: Vec::new(),
                },
                creases: message
                    .creases
                    .into_iter()
                    .map(|crease| MessageCrease {
                        range: crease.start..crease.end,
                        metadata: CreaseMetadata {
                            icon_path: crease.icon_path,
                            label: crease.label,
                        },
                        context: None,
                    })
                    .collect(),
                timestamp: Utc::now(), // We don't have timestamp in serialized data
            });
        }
        
        let detailed_summary = match serialized.detailed_summary_state {
            crate::thread_store::DetailedSummaryState::NotGenerated => DetailedSummaryState::NotGenerated,
            crate::thread_store::DetailedSummaryState::Generating { message_id } => {
                DetailedSummaryState::Generating { message_id }
            }
            crate::thread_store::DetailedSummaryState::Generated { text, message_id } => {
                DetailedSummaryState::Generated { text, message_id }
            }
        };
        
        Self {
            id,
            updated_at: serialized.updated_at,
            title: Some(serialized.summary),
            messages,
            next_message_id,
            prompt_id: PromptId::new(),
            request_token_usage: serialized.request_token_usage,
            cumulative_token_usage: serialized.cumulative_token_usage,
            last_received_chunk_at: None,
            detailed_summary: Some(detailed_summary),
        }
    }
    
    /// Serialize the conversation for storage
    pub fn serialize(&self, 
        tool_uses_by_message: HashMap<MessageId, Vec<SerializedToolUse>>,
        tool_results_by_message: HashMap<MessageId, Vec<SerializedToolResult>>,
        project_snapshot: Option<Arc<ProjectSnapshot>>,
        configured_model: Option<ConfiguredModel>,
    ) -> SerializedThread {
        let serialized_messages = self.messages.iter().map(|message| {
            let tool_uses = tool_uses_by_message.get(&message.id).cloned().unwrap_or_default();
            let tool_results = tool_results_by_message.get(&message.id).cloned().unwrap_or_default();
            message.serialize(&tool_uses, &tool_results)
        }).collect();
        
        let serialized_model = configured_model.map(|model| {
            SerializedLanguageModel {
                provider: model.provider_id.0.to_string(),
                model: model.model_id.0.to_string(),
            }
        });
        
        SerializedThread {
            version: SerializedThread::VERSION.to_string(),
            summary: self.title.clone().unwrap_or_else(|| SharedString::from("New Conversation")),
            updated_at: self.updated_at,
            messages: serialized_messages,
            request_token_usage: self.request_token_usage.clone(),
            cumulative_token_usage: self.cumulative_token_usage.clone(),
            detailed_summary_state: self.detailed_summary.clone().unwrap_or_default().to_thread_store_format(),
            initial_project_snapshot: project_snapshot,
            exceeded_window_error: None, // Not tracking this in Conversation
            model: serialized_model,
            completion_mode: None, // Not tracking this in Conversation yet
        }
    }
    
    /// Get the conversation id
    pub fn id(&self) -> &ConversationId {
        &self.id
    }
    
    /// Get the last update time
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
    
    /// Touch the updated_at timestamp
    pub fn touch_updated_at(&mut self) {
        self.updated_at = Utc::now();
    }
    
    /// Set the conversation title
    pub fn set_title(&mut self, title: impl Into<SharedString>) {
        self.title = Some(title.into());
        self.touch_updated_at();
    }
    
    /// Get the conversation title
    pub fn title(&self) -> Option<&SharedString> {
        self.title.as_ref()
    }
    
    /// Get the title or a default
    pub fn title_or_default(&self) -> SharedString {
        self.title.clone().unwrap_or_else(|| SharedString::new_static("New Conversation"))
    }
    
    /// Advance the prompt ID
    pub fn advance_prompt_id(&mut self) {
        self.prompt_id = PromptId::new();
    }
    
    /// Get the current prompt ID
    pub fn current_prompt_id(&self) -> &PromptId {
        &self.prompt_id
    }
    
    /// Check if the conversation is empty
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
    
    /// Get a message by ID
    pub fn message(&self, id: MessageId) -> Option<&Message> {
        self.messages.iter().find(|m| m.id == id)
    }
    
    /// Get a mutable message by ID
    pub fn message_mut(&mut self, id: MessageId) -> Option<&mut Message> {
        self.messages.iter_mut().find(|m| m.id == id)
    }
    
    /// Get all messages
    pub fn messages(&self) -> impl ExactSizeIterator<Item = &Message> {
        self.messages.iter()
    }
    
    /// Record that a chunk was received
    pub fn received_chunk(&mut self) {
        self.last_received_chunk_at = Some(Instant::now());
    }
    
    /// Insert a user message
    pub fn insert_user_message(
        &mut self,
        text: impl Into<String>,
        loaded_context: LoadedContext,
        creases: Vec<MessageCrease>,
    ) -> MessageId {
        self.insert_message(
            Role::User,
            vec![MessageSegment::Text(text.into())],
            loaded_context,
            creases,
        )
    }
    
    /// Insert an assistant message
    pub fn insert_assistant_message(
        &mut self,
        segments: Vec<MessageSegment>,
    ) -> MessageId {
        self.insert_message(
            Role::Assistant,
            segments,
            LoadedContext::default(),
            Vec::new(),
        )
    }
    
    /// Insert a message
    pub fn insert_message(
        &mut self,
        role: Role,
        segments: Vec<MessageSegment>,
        loaded_context: LoadedContext,
        creases: Vec<MessageCrease>,
    ) -> MessageId {
        let id = self.next_message_id.post_inc();
        
        self.messages.push(Message {
            id,
            role,
            segments,
            loaded_context,
            creases,
            timestamp: Utc::now(),
        });
        
        self.touch_updated_at();
        id
    }
    
    /// Edit a message
    pub fn edit_message(
        &mut self,
        id: MessageId,
        new_role: Role,
        new_segments: Vec<MessageSegment>,
        loaded_context: Option<LoadedContext>,
    ) -> bool {
        if let Some(message) = self.message_mut(id) {
            message.role = new_role;
            message.segments = new_segments;
            
            if let Some(context) = loaded_context {
                message.loaded_context = context;
            }
            
            self.touch_updated_at();
            true
        } else {
            false
        }
    }
    
    /// Delete a message
    pub fn delete_message(&mut self, id: MessageId) -> bool {
        let index = self.messages.iter().position(|m| m.id == id);
        
        if let Some(index) = index {
            self.messages.remove(index);
            self.touch_updated_at();
            true
        } else {
            false
        }
    }
    
    /// Get the conversation as a formatted text
    pub fn text(&self) -> String {
        let mut result = String::new();
        
        for message in &self.messages {
            // Add role header
            match message.role {
                Role::User => writeln!(result, "# User:").unwrap(),
                Role::Assistant => writeln!(result, "# Assistant:").unwrap(),
                Role::System => writeln!(result, "# System:").unwrap(),
                Role::Tool => writeln!(result, "# Tool:").unwrap(),
            }
            
            // Add message content
            writeln!(result, "{}", message.to_string()).unwrap();
            writeln!(result).unwrap();
        }
        
        result
    }
    
    /// Truncate the conversation after the given message ID
    pub fn truncate(&mut self, message_id: MessageId) {
        if let Some(index) = self.messages.iter().position(|m| m.id == message_id) {
            self.messages.truncate(index + 1);
            self.touch_updated_at();
        }
    }
    
    /// Convert the conversation to a sequence of language model messages
    pub fn to_model_messages(&self) -> Vec<LanguageModelRequestMessage> {
        self.messages
            .iter()
            .map(|message| {
                let mut content = Vec::new();
                
                // Add loaded context if available
                if !message.loaded_context.text.is_empty() {
                    content.push(MessageContent::Text(message.loaded_context.text.clone()));
                }
                
                // Add message segments
                for segment in &message.segments {
                    match segment {
                        MessageSegment::Text(text) => {
                            content.push(MessageContent::Text(text.clone()));
                        }
                        MessageSegment::Thinking { text, .. } => {
                            content.push(MessageContent::Thinking(text.clone()));
                        }
                        MessageSegment::RedactedThinking(_) => {}
                    }
                }
                
                LanguageModelRequestMessage {
                    role: message.role.clone(),
                    content,
                }
            })
            .collect()
    }
    
    /// Get the cumulative token usage
    pub fn cumulative_token_usage(&self) -> TokenUsage {
        self.cumulative_token_usage.clone()
    }
    
    /// Update token usage
    pub fn update_token_usage(&mut self, token_usage: TokenUsage) {
        self.request_token_usage.push(token_usage.clone());
        self.cumulative_token_usage.prompt_tokens += token_usage.prompt_tokens;
        self.cumulative_token_usage.completion_tokens += token_usage.completion_tokens;
        self.cumulative_token_usage.total_tokens += token_usage.total_tokens;
    }
    
    /// Set the detailed summary state
    pub fn set_detailed_summary(&mut self, state: DetailedSummaryState) {
        self.detailed_summary = Some(state);
    }
    
    /// Get the detailed summary state
    pub fn detailed_summary(&self) -> Option<&DetailedSummaryState> {
        self.detailed_summary.as_ref()
    }
}

impl DetailedSummaryState {
    /// Convert to thread_store format for serialization
    fn to_thread_store_format(&self) -> crate::thread::DetailedSummaryState {
        match self {
            Self::NotGenerated => crate::thread::DetailedSummaryState::NotGenerated,
            Self::Generating { message_id } => {
                crate::thread::DetailedSummaryState::Generating {
                    message_id: *message_id,
                }
            },
            Self::Generated { text, message_id } => {
                crate::thread::DetailedSummaryState::Generated {
                    text: text.clone(),
                    message_id: *message_id
                }
            }
        }
    }

    pub fn from_thread_detailed_summary(state: &crate::thread::DetailedSummaryState) -> Self {
        match state {
            crate::thread::DetailedSummaryState::NotGenerated => Self::NotGenerated,
            crate::thread::DetailedSummaryState::Generating { message_id } => {
                Self::Generating { message_id: *message_id }
            }
            crate::thread::DetailedSummaryState::Generated { text, message_id } => {
                Self::Generated { text: text.clone(), message_id: *message_id }
            }
        }
    }
} 