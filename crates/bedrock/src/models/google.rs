use super::{BedrockModelMode, ModelCapabilities};

pub const GEMMA_3_4B: ModelCapabilities = ModelCapabilities {
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

pub const GEMMA_3_12B: ModelCapabilities = ModelCapabilities {
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

pub const GEMMA_3_27B: ModelCapabilities = ModelCapabilities {
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
