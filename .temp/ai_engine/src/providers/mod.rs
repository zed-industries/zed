//! AI model providers implementation

mod model_provider;
pub use model_provider::ModelProvider;

#[cfg(feature = "openai")]
pub mod openai;

#[cfg(feature = "gemini")]
pub mod gemini;

#[cfg(feature = "claude")]
pub mod claude;

#[cfg(feature = "deepseek")]
pub mod deepseek;

#[cfg(feature = "qwen")]
pub mod qwen;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{AIEngineError, AIRequest, AIResponse};

/// Trait for AI model providers
#[async_trait]
pub trait AIModel: Send + Sync {
    /// Process a completion request
    async fn complete(&self, request: &AIRequest) -> Result<AIResponse, AIEngineError>;
    
    /// Get the name of the model
    fn get_model_name(&self) -> &'static str;
}

/// Configuration for initializing model providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// API key for the model provider
    pub api_key: String,
    /// Base URL for the API (optional, uses default if not provided)
    pub base_url: Option<String>,
    /// Model name or ID
    pub model_name: String,
    /// Request timeout in seconds
    pub timeout_seconds: u64,
    /// Additional provider-specific configuration
    pub extra: Option<HashMap<String, String>>,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: None,
            model_name: String::new(),
            timeout_seconds: 30,
            extra: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_config_default() {
        let config = ModelConfig::default();
        assert_eq!(config.api_key, "");
        assert!(config.base_url.is_none());
        assert_eq!(config.model_name, "");
        assert_eq!(config.timeout_seconds, 30);
        assert!(config.extra.is_none());
    }
}
