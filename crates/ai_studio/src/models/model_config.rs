// Model configuration types are defined in mod.rs
// This file exists to satisfy module declaration 

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

/// Core model configuration that replaces hardcoded language_model crate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub model_type: ModelType,
    pub provider: ModelProvider,
    pub parameters: ModelParameters,
    pub capabilities: Vec<ModelCapability>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelType {
    LanguageModel,
    CodeModel,
    EmbeddingModel,
    ImageModel,
    AudioModel,
    MultiModal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelProvider {
    OpenAI {
        api_key: String,
        base_url: Option<String>,
        organization: Option<String>,
    },
    Anthropic {
        api_key: String,
    },
    Local {
        model_path: String,
        executable_path: Option<String>,
    },
    Ollama {
        base_url: String,
        model_name: String,
    },
    HuggingFace {
        api_key: Option<String>,
        model_id: String,
    },
    Custom {
        name: String,
        config: serde_json::Value,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelParameters {
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<u32>,
    pub frequency_penalty: Option<f32>,
    pub presence_penalty: Option<f32>,
    pub stop_sequences: Option<Vec<String>>,
    pub context_window: Option<u32>,
    pub custom_parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ModelCapability {
    TextGeneration,
    CodeGeneration,
    TextEmbedding,
    ImageGeneration,
    ImageUnderstanding,
    AudioGeneration,
    AudioTranscription,
    FunctionCalling,
    ToolUse,
    StreamingResponse,
    ConversationMemory,
    MultiModal,
}

impl Default for ModelParameters {
    fn default() -> Self {
        Self {
            max_tokens: Some(2048),
            temperature: Some(0.7),
            top_p: Some(0.9),
            top_k: None,
            frequency_penalty: None,
            presence_penalty: None,
            stop_sequences: None,
            context_window: Some(4096),
            custom_parameters: HashMap::new(),
        }
    }
}

impl ModelConfig {
    pub fn new(name: String, model_type: ModelType, provider: ModelProvider) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description: String::new(),
            model_type,
            provider,
            parameters: ModelParameters::default(),
            capabilities: Vec::new(),
            created_at: now,
            updated_at: now,
            is_active: true,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn with_parameters(mut self, parameters: ModelParameters) -> Self {
        self.parameters = parameters;
        self
    }

    pub fn with_capabilities(mut self, capabilities: Vec<ModelCapability>) -> Self {
        self.capabilities = capabilities;
        self
    }

    pub fn update_timestamp(&mut self) {
        self.updated_at = chrono::Utc::now();
    }

    pub fn supports_capability(&self, capability: &ModelCapability) -> bool {
        self.capabilities.contains(capability)
    }
} 