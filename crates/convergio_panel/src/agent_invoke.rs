//! Agent Invocation Module
//!
//! Handles invoking Convergio agents via the Anthropic API
//! and streaming responses back to the database.

use anyhow::{anyhow, Result};
use anthropic::{
    Event, Message, Request, RequestContent, Role, ANTHROPIC_API_URL,
};
use futures::StreamExt;
use gpui::{BackgroundExecutor, Task};
use http_client::HttpClient;
use std::sync::Arc;

use crate::convergio_db::{ChatMessage, ConvergioDb, MessageType};

/// Default model for Convergio agents
const DEFAULT_MODEL: &str = "claude-sonnet-4-5";
const MAX_TOKENS: u64 = 8192;

/// Agent definition with name and system prompt
pub struct AgentDefinition {
    pub name: &'static str,
    pub display_name: &'static str,
    pub system_prompt: &'static str,
}

/// Get the system prompt for an agent by name
pub fn get_agent_prompt(agent_name: &str) -> Option<&'static AgentDefinition> {
    AGENTS.iter().find(|a| a.name == agent_name)
}

/// Invoke an agent with a user message
/// Returns a task that streams the response and updates the database
pub fn invoke_agent(
    db: Arc<ConvergioDb>,
    http_client: Arc<dyn HttpClient>,
    api_key: String,
    session_id: String,
    agent_name: String,
    messages: Vec<ChatMessage>,
    executor: BackgroundExecutor,
) -> Task<Result<()>> {
    executor.spawn(async move {
        // Get agent definition
        let agent = get_agent_prompt(&agent_name)
            .ok_or_else(|| anyhow!("Unknown agent: {}", agent_name))?;

        // Build message history for API
        let api_messages: Vec<Message> = messages
            .iter()
            .filter(|m| m.message_type == MessageType::User || m.message_type == MessageType::Assistant)
            .map(|m| Message {
                role: if m.message_type == MessageType::User {
                    Role::User
                } else {
                    Role::Assistant
                },
                content: vec![RequestContent::Text {
                    text: m.content.clone(),
                    cache_control: None,
                }],
            })
            .collect();

        // Build request
        let request = Request {
            model: DEFAULT_MODEL.to_string(),
            max_tokens: MAX_TOKENS,
            messages: api_messages,
            tools: vec![],
            thinking: None,
            tool_choice: None,
            system: Some(anthropic::StringOrContents::String(agent.system_prompt.to_string())),
            metadata: None,
            stop_sequences: vec![],
            temperature: Some(0.7),
            top_k: None,
            top_p: None,
        };

        // Stream completion
        let mut stream = anthropic::stream_completion(
            http_client.as_ref(),
            ANTHROPIC_API_URL,
            &api_key,
            request,
            None,
        )
        .await
        .map_err(|e| anyhow!("Failed to start stream: {:?}", e))?;

        // Collect response text
        let mut response_text = String::new();
        let mut input_tokens: i64 = 0;
        let mut output_tokens: i64 = 0;

        while let Some(event_result) = stream.next().await {
            match event_result {
                Ok(event) => match event {
                    Event::ContentBlockDelta { delta, .. } => {
                        if let anthropic::ContentDelta::TextDelta { text } = delta {
                            response_text.push_str(&text);
                        }
                    }
                    Event::MessageDelta { usage, .. } => {
                        if let Some(tokens) = usage.output_tokens {
                            output_tokens = tokens as i64;
                        }
                    }
                    Event::MessageStart { message } => {
                        if let Some(tokens) = message.usage.input_tokens {
                            input_tokens = tokens as i64;
                        }
                    }
                    Event::MessageStop => {
                        // Done streaming
                        break;
                    }
                    Event::Error { error } => {
                        log::error!("API error: {:?}", error);
                        return Err(anyhow!("API error: {:?}", error));
                    }
                    _ => {}
                },
                Err(e) => {
                    log::error!("Stream error: {:?}", e);
                    return Err(anyhow!("Stream error: {:?}", e));
                }
            }
        }

        // Insert assistant message into database
        if !response_text.is_empty() {
            // Calculate approximate cost (Claude Sonnet 4.5 pricing)
            let cost_usd = (input_tokens as f64 * 0.003 / 1000.0)
                + (output_tokens as f64 * 0.015 / 1000.0);

            db.insert_assistant_message(
                &session_id,
                &agent_name,
                &response_text,
                input_tokens,
                output_tokens,
                cost_usd,
            )?;
        }

        Ok(())
    })
}

/// Get the Anthropic API key from environment
pub fn get_api_key() -> Option<String> {
    std::env::var("ANTHROPIC_API_KEY").ok()
}

// Agent definitions with system prompts
// These are simplified versions - full prompts are in the CLI
static AGENTS: &[AgentDefinition] = &[
    AgentDefinition {
        name: "ali-chief-of-staff",
        display_name: "Ali - Chief of Staff",
        system_prompt: include_str!("prompts/ali-chief-of-staff.md"),
    },
    AgentDefinition {
        name: "rex-code-reviewer",
        display_name: "Rex - Code Reviewer",
        system_prompt: include_str!("prompts/rex-code-reviewer.md"),
    },
    AgentDefinition {
        name: "dario-debugger",
        display_name: "Dario - Debugger",
        system_prompt: include_str!("prompts/dario-debugger.md"),
    },
    AgentDefinition {
        name: "baccio-tech-architect",
        display_name: "Baccio - Tech Architect",
        system_prompt: include_str!("prompts/baccio-tech-architect.md"),
    },
    AgentDefinition {
        name: "paolo-best-practices-enforcer",
        display_name: "Paolo - Best Practices",
        system_prompt: include_str!("prompts/paolo-best-practices-enforcer.md"),
    },
    AgentDefinition {
        name: "marcello-pm",
        display_name: "Marcello - Product Manager",
        system_prompt: include_str!("prompts/marcello-pm.md"),
    },
];
