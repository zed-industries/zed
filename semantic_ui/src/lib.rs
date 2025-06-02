//! # Semantic UI
//! 
//! A dynamic, voice-aware UI generation system that creates interfaces from semantic schemas
//! rather than hardcoded types. Supports natural language commands and AI-driven interactions.

pub mod schema;
pub mod voice;
pub mod storage;
pub mod engine;

// Re-export main types for convenience
pub use schema::{
    SemanticSchema, Schema, FieldDefinition, FieldType, WidgetConfig,
    SemanticMetadata, ActionMetadata, ParameterMetadata
};

pub use voice::{
    VoiceCommandProcessor, ParsedCommand, Intent, Filter,
    ConversationContext, ExtractedEntities
};

pub use storage::{
    ContextualDataStore, Entity, Value, SemanticIndex
};

pub use engine::{
    PredictiveEngine, Suggestion, MultiModalProcessor, InputModality
};

// Error handling
#[derive(thiserror::Error, Debug)]
pub enum SemanticUIError {
    #[error("Schema validation error: {message}")]
    SchemaValidation { message: String },
    
    #[error("Voice processing error: {message}")]
    VoiceProcessing { message: String },
    
    #[error("Storage error: {message}")]
    Storage { message: String },
    
    #[error("Semantic analysis error: {message}")]
    SemanticAnalysis { message: String },
    
    #[error("Configuration error: {message}")]
    Configuration { message: String },
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Generic error: {0}")]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, SemanticUIError>; 