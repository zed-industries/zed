//! Multi-model AI engine for CodeOrbit
//! 
//! This crate provides a unified interface for interacting with multiple AI models
//! including OpenAI, Google Gemini, Anthropic Claude, DeepSeek, and Alibaba Qwen.

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// Re-export the ModelProvider enum and AIModel trait for convenience
pub use crate::providers::{ModelProvider, AIModel, ModelConfig};

// Include the providers module
#[cfg(any(
    feature = "openai",
    feature = "gemini",
    feature = "claude",
    feature = "deepseek",
    feature = "qwen"
))]
mod providers;

/// Request structure for AI completions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIRequest {
    /// The model provider to use for this request
    pub model: ModelProvider,
    /// The prompt to send to the model
    pub prompt: String,
    /// Controls randomness (0.0 to 2.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Maximum number of tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    /// Controls diversity via nucleus sampling (0.0 to 1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// Stop sequences to end generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    /// Additional parameters specific to the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

/// Response structure for AI completions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse {
    /// The generated text content
    pub text: String,
    /// The model that generated the response
    pub model: String,
    /// Token usage information if available
    pub usage: Option<HashMap<String, u32>>,
    /// Additional metadata from the provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Error type for AI engine operations
#[derive(Error, Debug)]
pub enum AIEngineError {
    /// Error making HTTP request
    #[error("API request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    
    /// Error with environment variables
    #[error("Environment variable error: {0}")]
    EnvVarError(#[from] std::env::VarError),
    
    /// Error parsing JSON
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    /// Error from the API
    #[error("API error: {0}")]
    ApiError(String),
    
    /// Requested model is not available
    #[error("Model not available: {0}")]
    ModelNotAvailable(String),
    
    /// Authentication failed
    #[error("Authentication failed: {0}")]
    AuthError(String),
    
    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    ConfigError(String),
    
    /// Feature not enabled
    #[error("Feature not enabled: {0}")]
    FeatureNotEnabled(String),
}

// AIModel trait is now defined in providers/mod.rs

/// Main AI Engine that manages multiple model providers
pub struct AIEngine {
    /// HTTP client for making requests
    client: reqwest::Client,
    /// Map of available models
    models: HashMap<ModelProvider, Arc<dyn AIModel>>,
    /// Default model to use when none is specified
    default_model: ModelProvider,
    /// Default request timeout
    timeout: Duration,
}

impl AIEngine {
    /// Create a new AI Engine instance with default settings
    pub fn new() -> Result<Self, AIEngineError> {
        dotenv::dotenv().ok();
        
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(AIEngineError::RequestError)?;
            
        let default_model = env::var("DEFAULT_MODEL")
            .unwrap_or_else(|_| "openai".to_string())
            .parse()
            .map_err(|e: String| AIEngineError::ModelNotAvailable(e))?;
            
        let timeout = Duration::from_secs(
            env::var("REQUEST_TIMEOUT")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30)
        );
        
        let mut engine = Self {
            client,
            models: HashMap::new(),
            default_model,
            timeout,
        };
        
        // Initialize models
        engine.initialize_models()?;
        
        Ok(engine)
    }
    
    /// Create a new AI Engine with a custom client and configuration
    pub fn with_config(
        client: reqwest::Client,
        default_model: ModelProvider,
        timeout: Duration,
    ) -> Result<Self, AIEngineError> {
        let mut engine = Self {
            client,
            models: HashMap::new(),
            default_model,
            timeout,
        };
        
        engine.initialize_models()?;
        Ok(engine)
    }
    
    /// Initialize all available models based on enabled features
    fn initialize_models(&mut self) -> Result<(), AIEngineError> {
        // Initialize each model provider if its feature is enabled
        // and the required environment variables are set
        
        #[cfg(feature = "openai")]
        if let Ok(api_key) = env::var("OPENAI_API_KEY") {
            let config = ModelConfig {
                api_key,
                base_url: env::var("OPENAI_API_BASE").ok(),
                model_name: env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-4o".to_string()),
                timeout_seconds: self.timeout.as_secs(),
                extra: None,
            };
            
            if let Ok(model) = crate::providers::openai::OpenAIModel::new(config) {
                self.models.insert(ModelProvider::OpenAIGpt4o, Arc::new(model));
            }
        }
        
        // Similar initialization for other providers...
        // Gemini, Claude, DeepSeek, Qwen would go here
        
        Ok(())
    }
    
    /// Register a custom model provider
    pub fn register_model(
        &mut self,
        provider: ModelProvider,
        model: impl AIModel + 'static,
    ) -> Option<Arc<dyn AIModel>> {
        self.models.insert(provider, Arc::new(model))
    }
    
    /// Process a completion request using the specified model
    pub async fn complete(&self, request: AIRequest) -> Result<AIResponse, AIEngineError> {
        let model = request.model;
        
        match self.models.get(&model) {
            Some(model_impl) => {
                debug!("Processing request with model: {}", model_impl.get_model_name());
                model_impl.complete(&request).await
            }
            None => Err(AIEngineError::ModelNotAvailable(format!(
                "Model {} is not available. Make sure the corresponding feature is enabled and API keys are set.",
                model
            ))),
        }
    }
    
    /// Process a completion request using the default model
    pub async fn complete_with_default(
        &self,
        prompt: String,
    ) -> Result<AIResponse, AIEngineError> {
        let request = AIRequest {
            model: self.default_model,
            prompt,
            temperature: None,
            max_tokens: None,
            top_p: None,
            stop: None,
            extra: None,
        };
        
        self.complete(request).await
    }
    
    /// Get a list of available models
    pub fn available_models(&self) -> Vec<ModelProvider> {
        self.models.keys().cloned().collect()
    }
    
    /// Check if a specific model is available
    pub fn has_model(&self, provider: ModelProvider) -> bool {
        self.models.contains_key(&provider)
    }
    
    /// Get the default model
    pub fn default_model(&self) -> ModelProvider {
        self.default_model
    }
    
    /// Set the default model
    pub fn set_default_model(&mut self, provider: ModelProvider) -> Result<(), AIEngineError> {
        if self.models.contains_key(&provider) {
            self.default_model = provider;
            Ok(())
        } else {
            Err(AIEngineError::ModelNotAvailable(
                "Cannot set default to an unavailable model".to_string()
            ))
        }
    }
    
    /// Get a reference to the HTTP client
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }
    
    /// Get the current timeout duration
    pub fn timeout(&self) -> Duration {
        self.timeout
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use serde_json::json;
    use async_trait::async_trait;

    // Mock implementation of AIModel for testing
    struct MockModel {
        name: &'static str,
        response: String,
    }

    #[async_trait]
    impl AIModel for MockModel {
        async fn complete(&self, _request: &AIRequest) -> Result<AIResponse, AIEngineError> {
            Ok(AIResponse {
                text: self.response.clone(),
                model: self.name.to_string(),
                usage: None,
                metadata: None,
            })
        }
        
        fn get_model_name(&self) -> &'static str {
            self.name
        }
    }

    #[tokio::test]
    async fn test_ai_engine_initialization() {
        // Test basic initialization
        let engine = AIEngine::new();
        assert!(engine.is_ok());
        
        // Test with custom config
        let client = reqwest::Client::new();
        let engine = AIEngine::with_config(
            client,
            ModelProvider::OpenAIGpt4o,
            Duration::from_secs(10)
        );
        assert!(engine.is_ok());
    }

    #[tokio::test]
    async fn test_model_registration() {
        let mut engine = AIEngine::new().unwrap();
        let model = MockModel {
            name: "test-model",
            response: "test response".to_string(),
        };
        
        // Register the mock model
        engine.register_model(ModelProvider::OpenAIGpt4o, model);
        
        // Check that the model is available
        assert!(engine.has_model(ModelProvider::OpenAIGpt4o));
        
        // Test completion with the mock model
        let request = AIRequest {
            model: ModelProvider::OpenAIGpt4o,
            prompt: "Test prompt".to_string(),
            temperature: None,
            max_tokens: None,
            top_p: None,
            stop: None,
            extra: None,
        };
        
        let response = engine.complete(request).await;
        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response.text, "test response");
    }

    #[tokio::test]
    async fn test_default_model_handling() {
        let mut engine = AIEngine::new().unwrap();
        
        // Set up a mock model as default
        let model = MockModel {
            name: "default-model",
            response: "default response".to_string(),
        };
        
        engine.register_model(ModelProvider::OpenAIGpt4o, model);
        engine.set_default_model(ModelProvider::OpenAIGpt4o).unwrap();
        
        // Test completion with default model
        let response = engine.complete_with_default("Test prompt".to_string()).await;
        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response.text, "default response");
    }

    #[tokio::test]
    async fn test_model_not_available_error() {
        let engine = AIEngine::new().unwrap();
        
        // Try to use a model that hasn't been registered
        let request = AIRequest {
            model: ModelProvider::GoogleGemini15Pro,
            prompt: "Test prompt".to_string(),
            temperature: None,
            max_tokens: None,
            top_p: None,
            stop: None,
            extra: None,
        };
        
        let response = engine.complete(request).await;
        assert!(response.is_err());
        
        // Check that the error message is informative
        let error = response.unwrap_err().to_string();
        assert!(error.contains("not available"));
    }

    #[tokio::test]
    async fn test_openai_completion() {
        // Set up mock server for OpenAI API
        let mock_server = MockServer::start().await;
        
        // Mock the OpenAI API response
        let response_json = json!({
            "id": "cmpl-123",
            "object": "text_completion",
            "created": 1589478378,
            "model": "gpt-4o",
            "choices": [{
                "text": "This is a test response from the mock server.",
                "index": 0,
                "logprobs": null,
                "finish_reason": "length"
            }],
            "usage": {
                "prompt_tokens": 5,
                "completion_tokens": 10,
                "total_tokens": 15
            }
        });
        
        // Set up the mock to respond to POST /v1/completions
        Mock::given(wiremock::matchers::method("POST"))
            .and(wiremock::matchers::path("/v1/completions"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(&response_json)
                    .insert_header("content-type", "application/json")
            )
            .mount(&mock_server)
            .await;
        
        // Create a test client that points to our mock server
        let client = reqwest::Client::builder()
            .build()
            .expect("Failed to create HTTP client");
        
        // Create an AI engine with our test client
        let mut engine = AIEngine::with_config(
            client,
            ModelProvider::OpenAIGpt4o,
            Duration::from_secs(5)
        ).unwrap();
        
        // Register the OpenAI model with our mock server URL
        let config = ModelConfig {
            api_key: "test-api-key".to_string(),
            base_url: Some(mock_server.uri()),
            model_name: "gpt-4o".to_string(),
            timeout_seconds: 5,
            extra: None,
        };
        
        let openai_model = crate::providers::openai::OpenAIModel::new(config).unwrap();
        engine.register_model(ModelProvider::OpenAIGpt4o, openai_model);
        
        // Test completion with the mock model
        let request = AIRequest {
            model: ModelProvider::OpenAIGpt4o,
            prompt: "Test prompt".to_string(),
            temperature: Some(0.7),
            max_tokens: Some(50),
            top_p: Some(1.0),
            stop: Some(vec!["\n".to_string()]),
            extra: None,
        };
        
        let response = engine.complete(request).await;
        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response.text, "This is a test response from the mock server.");
        assert_eq!(response.model, "gpt-4o");
        assert!(response.usage.is_some());
    }
    
    #[tokio::test]
    async fn test_error_handling() {
        // Test invalid model configuration
        let config = ModelConfig {
            api_key: "".to_string(),
            base_url: None,
            model_name: "".to_string(),
            timeout_seconds: 0,
            extra: None,
        };
        
        let result = crate::providers::openai::OpenAIModel::new(config);
        assert!(result.is_err());
        
        // Test request with invalid parameters
        let mut engine = AIEngine::new().unwrap();
        let request = AIRequest {
            model: ModelProvider::OpenAIGpt4o,
            prompt: "".to_string(), // Empty prompt
            temperature: Some(2.5), // Invalid temperature
            max_tokens: Some(0),    // Invalid max_tokens
            top_p: None,
            stop: None,
            extra: None,
        };
        
        // This will fail because we haven't registered the model
        let response = engine.complete(request).await;
        assert!(response.is_err());
    }
    
    #[tokio::test]
    async fn test_model_selection() {
        let mut engine = AIEngine::new().unwrap();
        
        // Register multiple models
        let model1 = MockModel {
            name: "model-1",
            response: "response-1".to_string(),
        };
        let model2 = MockModel {
            name: "model-2",
            response: "response-2".to_string(),
        };
        
        engine.register_model(ModelProvider::OpenAIGpt4o, model1);
        engine.register_model(ModelProvider::GoogleGemini15Pro, model2);
        
        // Test model 1
        let request1 = AIRequest {
            model: ModelProvider::OpenAIGpt4o,
            prompt: "test".to_string(),
            ..Default::default()
        };
        let response1 = engine.complete(request1).await.unwrap();
        assert_eq!(response1.text, "response-1");
        
        // Test model 2
        let request2 = AIRequest {
            model: ModelProvider::GoogleGemini15Pro,
            prompt: "test".to_string(),
            ..Default::default()
        };
        let response2 = engine.complete(request2).await.unwrap();
        assert_eq!(response2.text, "response-2");
    }
}
