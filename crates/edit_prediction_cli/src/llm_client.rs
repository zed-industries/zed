//! Shared LLM client abstraction for Anthropic and OpenAI.
//!
//! This module provides a unified interface for making LLM requests,
//! supporting both synchronous and batch modes.

use crate::BatchProvider;
use crate::anthropic_client::AnthropicClient;
use crate::openai_client::OpenAiClient;
use crate::paths::LLM_CACHE_DB;
use anyhow::Result;

/// A unified LLM client that wraps either Anthropic or OpenAI.
pub enum LlmClient {
    Anthropic(AnthropicClient),
    OpenAi(OpenAiClient),
}

impl LlmClient {
    /// Create a new LLM client for the given backend.
    ///
    /// If `batched` is true, requests will be queued for batch processing.
    /// Otherwise, requests are made synchronously.
    pub fn new(backend: BatchProvider, batched: bool) -> Result<Self> {
        match backend {
            BatchProvider::Anthropic => {
                if batched {
                    Ok(LlmClient::Anthropic(AnthropicClient::batch(&LLM_CACHE_DB)?))
                } else {
                    Ok(LlmClient::Anthropic(AnthropicClient::plain()?))
                }
            }
            BatchProvider::Openai => {
                if batched {
                    Ok(LlmClient::OpenAi(OpenAiClient::batch(&LLM_CACHE_DB)?))
                } else {
                    Ok(LlmClient::OpenAi(OpenAiClient::plain()?))
                }
            }
        }
    }

    /// Generate a response from the LLM.
    ///
    /// Returns `Ok(None)` if the request was queued for batch processing
    /// and results are not yet available.
    pub async fn generate(
        &self,
        model: &str,
        max_tokens: u64,
        prompt: &str,
    ) -> Result<Option<String>> {
        match self {
            LlmClient::Anthropic(client) => {
                let messages = vec![anthropic::Message {
                    role: anthropic::Role::User,
                    content: vec![anthropic::RequestContent::Text {
                        text: prompt.to_string(),
                        cache_control: None,
                    }],
                }];
                let response = client.generate(model, max_tokens, messages, None).await?;
                Ok(response.map(|r| {
                    r.content
                        .iter()
                        .filter_map(|c| match c {
                            anthropic::ResponseContent::Text { text } => Some(text.as_str()),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                        .join("")
                }))
            }
            LlmClient::OpenAi(client) => {
                let messages = vec![open_ai::RequestMessage::User {
                    content: open_ai::MessageContent::Plain(prompt.to_string()),
                }];
                let response = client.generate(model, max_tokens, messages, None).await?;
                Ok(response.map(|r| {
                    r.choices
                        .into_iter()
                        .filter_map(|choice| match choice.message {
                            open_ai::RequestMessage::Assistant { content, .. } => {
                                content.map(|c| match c {
                                    open_ai::MessageContent::Plain(text) => text,
                                    open_ai::MessageContent::Multipart(parts) => parts
                                        .into_iter()
                                        .filter_map(|p| match p {
                                            open_ai::MessagePart::Text { text } => Some(text),
                                            _ => None,
                                        })
                                        .collect::<Vec<_>>()
                                        .join(""),
                                })
                            }
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                        .join("")
                }))
            }
        }
    }

    /// Sync pending batches - upload queued requests and download completed results.
    pub async fn sync_batches(&self) -> Result<()> {
        match self {
            LlmClient::Anthropic(client) => client.sync_batches().await,
            LlmClient::OpenAi(client) => client.sync_batches().await,
        }
    }
}

/// Get the model name for a given backend.
pub fn model_for_backend(backend: BatchProvider) -> &'static str {
    match backend {
        BatchProvider::Anthropic => "claude-sonnet-4-5",
        BatchProvider::Openai => "gpt-5.2",
    }
}
