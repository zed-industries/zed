use std::collections::HashMap;
use std::sync::Mutex;
use zed_extension_api::{self as zed, *};

struct ExampleProvider {
    /// Active completion streams, keyed by stream ID
    streams: Mutex<HashMap<String, Vec<LlmCompletionEvent>>>,
    /// Counter for generating unique stream IDs
    next_stream_id: Mutex<u64>,
}

impl zed::Extension for ExampleProvider {
    fn new() -> Self {
        Self {
            streams: Mutex::new(HashMap::new()),
            next_stream_id: Mutex::new(0),
        }
    }

    fn llm_providers(&self) -> Vec<LlmProviderInfo> {
        vec![LlmProviderInfo {
            id: "example".into(),
            name: "Example Provider".into(),
            icon: None,
        }]
    }

    fn llm_provider_models(&self, _provider_id: &str) -> Result<Vec<LlmModelInfo>, String> {
        Ok(vec![
            LlmModelInfo {
                id: "example-fast".into(),
                name: "Example Fast".into(),
                max_token_count: 8192,
                max_output_tokens: Some(4096),
                capabilities: LlmModelCapabilities {
                    supports_images: false,
                    supports_tools: true,
                    supports_tool_choice_auto: true,
                    supports_tool_choice_any: true,
                    supports_tool_choice_none: true,
                    supports_thinking: false,
                    tool_input_format: LlmToolInputFormat::JsonSchema,
                },
                is_default: false,
                is_default_fast: true,
            },
            LlmModelInfo {
                id: "example-smart".into(),
                name: "Example Smart".into(),
                max_token_count: 32768,
                max_output_tokens: Some(8192),
                capabilities: LlmModelCapabilities {
                    supports_images: true,
                    supports_tools: true,
                    supports_tool_choice_auto: true,
                    supports_tool_choice_any: true,
                    supports_tool_choice_none: true,
                    supports_thinking: true,
                    tool_input_format: LlmToolInputFormat::JsonSchema,
                },
                is_default: true,
                is_default_fast: false,
            },
        ])
    }

    fn llm_provider_is_authenticated(&self, _provider_id: &str) -> bool {
        // Example provider is always authenticated for testing
        true
    }

    fn llm_provider_settings_markdown(&self, _provider_id: &str) -> Option<String> {
        Some(r#"# Example Provider Setup

Welcome to the **Example Provider**! This is a demonstration LLM provider for testing purposes.

## Features

- 🚀 **Fast responses** - Instant echo responses for testing
- 🛠️ **Tool support** - Full function calling capabilities
- 🖼️ **Image support** - Vision model available (Example Smart)

## Configuration

No API key is required for this example provider. It echoes back your messages for testing purposes.

## Models

- **Example Fast** - Quick responses, 8K context
- **Example Smart** - Extended features, 32K context, supports images and thinking

## Usage

Simply select this provider and start chatting! Your messages will be echoed back with the model name.
"#.to_string())
    }

    fn llm_provider_authenticate(&mut self, _provider_id: &str) -> Result<(), String> {
        // Example provider doesn't need authentication
        Ok(())
    }

    fn llm_stream_completion_start(
        &mut self,
        _provider_id: &str,
        model_id: &str,
        request: &LlmCompletionRequest,
    ) -> Result<String, String> {
        // Get the last user message to echo back
        let user_message = request
            .messages
            .iter()
            .filter(|m| matches!(m.role, LlmMessageRole::User))
            .last()
            .and_then(|m| {
                m.content.iter().find_map(|c| {
                    if let LlmMessageContent::Text(text) = c {
                        Some(text.clone())
                    } else {
                        None
                    }
                })
            })
            .unwrap_or_else(|| "Hello!".to_string());

        // Create a response based on the model
        let response_text = format!("Hello from {}! You said: \"{}\"", model_id, user_message);

        // Create events for the stream - simulate streaming by breaking into chunks
        let mut events = vec![LlmCompletionEvent::Started];

        // Stream the response in chunks
        for chunk in response_text.chars().collect::<Vec<_>>().chunks(10) {
            let text: String = chunk.iter().collect();
            events.push(LlmCompletionEvent::Text(text));
        }

        events.push(LlmCompletionEvent::Stop(LlmStopReason::EndTurn));
        events.push(LlmCompletionEvent::Usage(LlmTokenUsage {
            input_tokens: 10,
            output_tokens: response_text.len() as u64 / 4,
            cache_creation_input_tokens: None,
            cache_read_input_tokens: None,
        }));

        // Generate a unique stream ID
        let mut id_counter = self.next_stream_id.lock().unwrap();
        let stream_id = format!("example-stream-{}", *id_counter);
        *id_counter += 1;

        // Store the events
        self.streams
            .lock()
            .unwrap()
            .insert(stream_id.clone(), events);

        Ok(stream_id)
    }

    fn llm_stream_completion_next(
        &mut self,
        stream_id: &str,
    ) -> Result<Option<LlmCompletionEvent>, String> {
        let mut streams = self.streams.lock().unwrap();
        if let Some(events) = streams.get_mut(stream_id) {
            if events.is_empty() {
                Ok(None)
            } else {
                Ok(Some(events.remove(0)))
            }
        } else {
            Err(format!("Unknown stream: {}", stream_id))
        }
    }

    fn llm_stream_completion_close(&mut self, stream_id: &str) {
        self.streams.lock().unwrap().remove(stream_id);
    }
}

zed::register_extension!(ExampleProvider);
