//! Model provider definitions and implementations

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Supported AI model providers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModelProvider {
    /// OpenAI's GPT-4o model
    OpenAIGpt4o,
    /// Google's Gemini 1.5 Pro model
    GoogleGemini15Pro,
    /// Anthropic's Claude 3 Sonnet model
    AnthropicClaudeSonnet,
    /// DeepSeek Coder model
    DeepSeekCoder,
    /// Alibaba's Qwen model
    AlibabaQwen,
}

impl FromStr for ModelProvider {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "openai" | "gpt-4o" => Ok(ModelProvider::OpenAIGpt4o),
            "gemini" | "gemini-1.5-pro" => Ok(ModelProvider::GoogleGemini15Pro),
            "claude" | "claude-sonnet" | "claude-3-sonnet" => Ok(ModelProvider::AnthropicClaudeSonnet),
            "deepseek" | "deepseek-coder" => Ok(ModelProvider::DeepSeekCoder),
            "qwen" | "alibaba-qwen" | "qwen-max" => Ok(ModelProvider::AlibabaQwen),
            _ => Err(format!("Unknown model provider: {}", s)),
        }
    }
}

impl fmt::Display for ModelProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ModelProvider::OpenAIGpt4o => "gpt-4o",
            ModelProvider::GoogleGemini15Pro => "gemini-1.5-pro",
            ModelProvider::AnthropicClaudeSonnet => "claude-3-sonnet",
            ModelProvider::DeepSeekCoder => "deepseek-coder",
            ModelProvider::AlibabaQwen => "qwen-max",
        };
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_model_provider_parsing() {
        assert_eq!(
            ModelProvider::from_str("openai").unwrap(),
            ModelProvider::OpenAIGpt4o
        );
        assert_eq!(
            ModelProvider::from_str("gpt-4o").unwrap(),
            ModelProvider::OpenAIGpt4o
        );
        assert_eq!(
            ModelProvider::from_str("gemini").unwrap(),
            ModelProvider::GoogleGemini15Pro
        );
        assert_eq!(
            ModelProvider::from_str("claude").unwrap(),
            ModelProvider::AnthropicClaudeSonnet
        );
        assert!(ModelProvider::from_str("unknown").is_err());
    }

    #[test]
    fn test_model_provider_display() {
        assert_eq!(ModelProvider::OpenAIGpt4o.to_string(), "gpt-4o");
        assert_eq!(
            ModelProvider::GoogleGemini15Pro.to_string(),
            "gemini-1.5-pro"
        );
        assert_eq!(
            ModelProvider::AnthropicClaudeSonnet.to_string(),
            "claude-3-sonnet"
        );
    }
}
