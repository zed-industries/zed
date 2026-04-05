use super::{BedrockModelMode, ModelCapabilities};

pub const DEEPSEEK_R1: ModelCapabilities = ModelCapabilities {
    max_tokens: 128_000,
    max_output_tokens: 16_000,
    default_temperature: 1.0,
    supports_tool_use: true,
    supports_images: false,
    supports_thinking: false,
    supports_adaptive_thinking: false,
    cache_configuration: None,
    extended_context_token_count: None,
    thinking_mode: BedrockModelMode::Default,
};

pub const DEEPSEEK_V3_1: ModelCapabilities = ModelCapabilities {
    max_tokens: 128_000,
    max_output_tokens: 16_000,
    default_temperature: 1.0,
    supports_tool_use: true,
    supports_images: false,
    supports_thinking: false,
    supports_adaptive_thinking: false,
    cache_configuration: None,
    extended_context_token_count: None,
    thinking_mode: BedrockModelMode::Default,
};

pub const DEEPSEEK_V3_2: ModelCapabilities = ModelCapabilities {
    max_tokens: 128_000,
    max_output_tokens: 16_000,
    default_temperature: 1.0,
    supports_tool_use: true,
    supports_images: false,
    supports_thinking: false,
    supports_adaptive_thinking: false,
    cache_configuration: None,
    extended_context_token_count: None,
    thinking_mode: BedrockModelMode::Default,
};
