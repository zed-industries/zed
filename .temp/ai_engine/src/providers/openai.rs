use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use log::{debug, error};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    AIModel, AIEngineError, AIRequest, AIResponse, ModelProvider
};

/// OpenAI API configuration
#[derive(Debug, Clone)]
pub struct OpenAIConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub timeout_seconds: u64,
}

impl Default for OpenAIConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://api.openai.com/v1".to_string(),
            model: "gpt-4o".to_string(),
            timeout_seconds: 30,
        }
    }
}

/// OpenAI model implementation
pub struct OpenAIModel {
    client: Client,
    config: OpenAIConfig,
}

impl OpenAIModel {
    /// Create a new OpenAI model instance
    pub fn new(api_key: String) -> Result<Self, AIEngineError> {
        let config = OpenAIConfig {
            api_key,
            ..Default::default()
        };
        
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()
            .map_err(AIEngineError::RequestError)?;
            
        Ok(Self { client, config })
    }
    
    /// Create a new OpenAI model with custom configuration
    pub fn with_config(config: OpenAIConfig) -> Result<Self, AIEngineError> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()
            .map_err(AIEngineError::RequestError)?;
            
        Ok(Self { client, config })
    }
    
    /// Build the request body for the OpenAI API
    fn build_request_body(&self, request: &AIRequest) -> serde_json::Value {
        let mut messages = vec![json!({ "role": "user", "content": request.prompt })];
        
        let mut body = json!({ 
            "model": &self.config.model,
            "messages": messages,
        });
        
        // Add optional parameters if they exist
        if let Some(temp) = request.temperature {
            body["temperature"] = json!(temp);
        }
        
        if let Some(max_tokens) = request.max_tokens {
            body["max_tokens"] = json!(max_tokens);
        }
        
        if let Some(top_p) = request.top_p {
            body["top_p"] = json!(top_p);
        }
        
        if let Some(stop) = &request.stop {
            body["stop"] = json!(stop);
        }
        
        body
    }
    
    /// Parse the response from the OpenAI API
    fn parse_response(&self, response: &str) -> Result<AIResponse, AIEngineError> {
        let json: serde_json::Value = serde_json::from_str(response)
            .map_err(AIEngineError::JsonError)?;
        
        // Check for API errors
        if let Some(error) = json.get("error") {
            let error_message = error["message"].as_str().unwrap_or("Unknown error");
            return Err(AIEngineError::ApiError(error_message.to_string()));
        }
        
        // Extract the response text
        let text = json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| AIEngineError::ApiError("Invalid response format".to_string()))?
            .to_string();
            
        // Extract usage information if available
        let usage = if let Some(usage) = json.get("usage") {
            let mut usage_map = HashMap::new();
            if let Some(prompt_tokens) = usage["prompt_tokens"].as_u64() {
                usage_map.insert("prompt_tokens".to_string(), prompt_tokens as u32);
            }
            if let Some(completion_tokens) = usage["completion_tokens"].as_u64() {
                usage_map.insert("completion_tokens".to_string(), completion_tokens as u32);
            }
            if let Some(total_tokens) = usage["total_tokens"].as_u64() {
                usage_map.insert("total_tokens".to_string(), total_tokens as u32);
            }
            if !usage_map.is_empty() {
                Some(usage_map)
            } else {
                None
            }
        } else {
            None
        };
        
        Ok(AIResponse {
            text,
            model: self.config.model.clone(),
            usage,
        })
    }
}

#[async_trait]
impl AIModel for OpenAIModel {
    async fn complete(&self, request: &AIRequest) -> Result<AIResponse, AIEngineError> {
        let url = format!("{}/chat/completions", self.config.base_url);
        let body = self.build_request_body(request);
        
        debug!("Sending request to OpenAI API: {}", url);
        
        let response = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .json(&body)
            .send()
            .await
            .map_err(AIEngineError::RequestError)?;
            
        let status = response.status();
        let response_text = response.text().await
            .map_err(AIEngineError::RequestError)?;
            
        if !status.is_success() {
            error!("OpenAI API error ({}): {}", status, response_text);
            return Err(AIEngineError::ApiError(format!(
                "API request failed with status {}: {}",
                status, response_text
            )));
        }
        
        self.parse_response(&response_text)
    }
    
    fn get_model_name(&self) -> &'static str {
        "OpenAI GPT-4o"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::{method, header, body_json};
    
    #[tokio::test]
    async fn test_openai_completion() {
        // Set up mock server
        let mock_server = MockServer::start().await;
        
        // Mock the OpenAI API response
        let mock_response = json!({
            "id": "chatcmpl-123",
            "object": "chat.completion",
            "created": 1677652288,
            "model": "gpt-4o",
            "choices": [{
                "index": 0,
                "message": {
                    "role": "assistant",
                    "content": "Hello, world!"
                },
                "finish_reason": "stop"
            }],
            "usage": {
                "prompt_tokens": 9,
                "completion_tokens": 12,
                "total_tokens": 21
            }
        });
        
        // Set up the mock
        Mock::given(method("POST"))
            .and(header("Content-Type", "application/json"))
            .and(header("Authorization", "Bearer test_key"))
            .and(body_json(json!({
                "model": "gpt-4o",
                "messages": [{"role": "user", "content": "Test prompt"}]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
            .mount(&mock_server)
            .await;
        
        // Create a test config
        let config = OpenAIConfig {
            api_key: "test_key".to_string(),
            base_url: mock_server.uri(),
            model: "gpt-4o".to_string(),
            timeout_seconds: 10,
        };
        
        // Create the model
        let model = OpenAIModel::with_config(config).unwrap();
        
        // Create a test request
        let request = AIRequest {
            model: ModelProvider::OpenAIGpt4o,
            prompt: "Test prompt".to_string(),
            temperature: None,
            max_tokens: None,
            top_p: None,
            stop: None,
        };
        
        // Make the request
        let response = model.complete(&request).await.unwrap();
        
        // Verify the response
        assert_eq!(response.text, "Hello, world!");
        assert_eq!(response.model, "gpt-4o");
        assert!(response.usage.is_some());
        let usage = response.usage.unwrap();
        assert_eq!(usage.get("prompt_tokens"), Some(&9));
        assert_eq!(usage.get("completion_tokens"), Some(&12));
        assert_eq!(usage.get("total_tokens"), Some(&21));
    }
    
    #[tokio::test]
    async fn test_openai_error_handling() {
        // Set up mock server
        let mock_server = MockServer::start().await;
        
        // Mock an error response
        let error_response = json!({
            "error": {
                "message": "Invalid API key",
                "type": "invalid_request_error",
                "param": null,
                "code": "invalid_api_key"
            }
        });
        
        // Set up the mock
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(401).set_body_json(&error_response))
            .mount(&mock_server)
            .await;
        
        // Create a test config
        let config = OpenAIConfig {
            api_key: "invalid_key".to_string(),
            base_url: mock_server.uri(),
            model: "gpt-4o".to_string(),
            timeout_seconds: 10,
        };
        
        // Create the model
        let model = OpenAIModel::with_config(config).unwrap();
        
        // Create a test request
        let request = AIRequest {
            model: ModelProvider::OpenAIGpt4o,
            prompt: "Test prompt".to_string(),
            temperature: None,
            max_tokens: None,
            top_p: None,
            stop: None,
        };
        
        // Make the request and expect an error
        let result = model.complete(&request).await;
        assert!(result.is_err());
        
        // Check the error type and message
        if let Err(AIEngineError::ApiError(message)) = result {
            assert!(message.contains("Invalid API key"));
        } else {
            panic!("Expected ApiError, got {:?}", result);
        }
    }
}
