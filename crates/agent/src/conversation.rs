use std::fmt::Write as _;
use std::ops::Range;
use std::sync::Arc;
use std::time::Instant;

use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use editor::display_map::CreaseMetadata;
use gpui::{AppContext, AsyncApp, SharedString};
use language_model::{
    LanguageModelRequestMessage, LanguageModelRequestTool, LanguageModelToolResult,
    LanguageModelToolUseId, MessageContent, Role, TokenUsage,
};
use serde::{Deserialize, Serialize};
use util::post_inc;

use crate::context::{AgentContextHandle, LoadedContext};
use crate::thread_store::{
    SerializedMessage, SerializedMessageSegment, SerializedToolResult,
    SerializedToolUse,
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
        self.segments.iter().all(|segment| segment.should_display())
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
                MessageSegment::Thinking { text, .. } => {
                    result.push_str("<think>\n");
                    result.push_str(text);
                    result.push_str("\n</think>");
                }
                MessageSegment::RedactedThinking(_) => {}
            }
        }

        result
    }
    
    /// Serialize the message for storage
    pub fn serialize(&self) -> SerializedMessage {
        SerializedMessage {
            id: self.id.0,
            role: self.role.clone(),
            segments: self.segments.iter().map(|s| s.serialize()).collect(),
            context: self.loaded_context.serialize(),
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
#[derive(Debug)]
pub struct Conversation {
    id: ConversationId,
    updated_at: DateTime<Utc>,
    title: Option<SharedString>,
    messages: Vec<Message>,
    next_message_id: MessageId,
    last_prompt_id: PromptId,
    request_token_usage: Vec<TokenUsage>,
    cumulative_token_usage: TokenUsage,
    last_received_chunk_at: Option<Instant>,
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
            last_prompt_id: PromptId::new(),
            request_token_usage: Vec::new(),
            cumulative_token_usage: TokenUsage::default(),
            last_received_chunk_at: None,
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
        self.last_prompt_id = PromptId::new();
    }
    
    /// Get the current prompt ID
    pub fn current_prompt_id(&self) -> &PromptId {
        &self.last_prompt_id
    }
    
    /// Check if the conversation is empty
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
    
    /// Get a specific message by ID
    pub fn message(&self, id: MessageId) -> Option<&Message> {
        self.messages.iter().find(|m| m.id == id)
    }
    
    /// Get all messages
    pub fn messages(&self) -> impl ExactSizeIterator<Item = &Message> {
        self.messages.iter()
    }
    
    /// Record that a chunk was received
    pub fn received_chunk(&mut self) {
        self.last_received_chunk_at = Some(Instant::now());
    }
    
    /// Insert a new user message
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
    
    /// Insert a new assistant message
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
    
    /// Insert a message with the specified role
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
    
    /// Edit an existing message
    pub fn edit_message(
        &mut self,
        id: MessageId,
        new_role: Role,
        new_segments: Vec<MessageSegment>,
        loaded_context: Option<LoadedContext>,
    ) -> bool {
        if let Some(message) = self.messages.iter_mut().find(|m| m.id == id) {
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
        let initial_len = self.messages.len();
        self.messages.retain(|m| m.id != id);
        let deleted = self.messages.len() < initial_len;
        
        if deleted {
            self.touch_updated_at();
        }
        
        deleted
    }
    
    /// Get the entire conversation as text
    pub fn text(&self) -> String {
        let mut result = String::new();
        
        for message in &self.messages {
            match message.role {
                Role::System => {
                    writeln!(result, "# System:").unwrap();
                }
                Role::User => {
                    writeln!(result, "# User:").unwrap();
                }
                Role::Assistant => {
                    writeln!(result, "# Assistant:").unwrap();
                }
                Role::Tool => {
                    writeln!(result, "# Tool:").unwrap();
                }
            }
            
            result.push_str(&message.to_string());
            result.push('\n');
        }
        
        result
    }
    
    /// Truncate the conversation up to a specific message
    pub fn truncate(&mut self, message_id: MessageId) {
        if let Some(index) = self.messages.iter().position(|m| m.id == message_id) {
            self.messages.truncate(index + 1);
            self.touch_updated_at();
        }
    }
    
    /// Convert the conversation to a series of language model request messages
    pub fn to_model_messages(&self) -> Vec<LanguageModelRequestMessage> {
        let mut result = Vec::new();
        
        for message in &self.messages {
            let content = match message.role {
                Role::System | Role::User | Role::Assistant => {
                    MessageContent::Text(message.to_string())
                }
                Role::Tool => continue, // Tool messages are handled specially
            };
            
            result.push(LanguageModelRequestMessage {
                role: message.role.clone(),
                content,
                name: None,
                cache: false,
                tool_calls: Vec::new(),
            });
        }
        
        result
    }
    
    /// Calculate the cumulative token usage
    pub fn cumulative_token_usage(&self) -> TokenUsage {
        self.cumulative_token_usage.clone()
    }
    
    /// Update the token usage with a new measurement
    pub fn update_token_usage(&mut self, token_usage: TokenUsage) {
        self.request_token_usage.push(token_usage.clone());
        self.cumulative_token_usage = self.cumulative_token_usage.add(&token_usage);
    }
} 