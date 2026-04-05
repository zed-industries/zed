use super::{BedrockModelMode, ModelCapabilities};

pub const LLAMA_4_SCOUT_17B: ModelCapabilities = ModelCapabilities {
    max_tokens: 128_000,
    max_output_tokens: 8_192,
    default_temperature: 1.0,
    supports_tool_use: false,
    supports_images: false,
    supports_thinking: false,
    supports_adaptive_thinking: false,
    cache_configuration: None,
    extended_context_token_count: None,
    thinking_mode: BedrockModelMode::Default,
};

pub const LLAMA_4_MAVERICK_17B: ModelCapabilities = ModelCapabilities {
    max_tokens: 128_000,
    max_output_tokens: 8_192,
    default_temperature: 1.0,
    supports_tool_use: false,
    supports_images: false,
    supports_thinking: false,
    supports_adaptive_thinking: false,
    cache_configuration: None,
    extended_context_token_count: None,
    thinking_mode: BedrockModelMode::Default,
};
