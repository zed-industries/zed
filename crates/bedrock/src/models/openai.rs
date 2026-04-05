use super::{BedrockModelMode, ModelCapabilities};

pub const GPT_OSS_20B: ModelCapabilities = ModelCapabilities {
    max_tokens: 128_000,
    max_output_tokens: 16_000,
    default_temperature: 1.0,
    supports_tool_use: false,
    supports_images: false,
    supports_thinking: false,
    supports_adaptive_thinking: false,
    cache_configuration: None,
    extended_context_token_count: None,
    thinking_mode: BedrockModelMode::Default,
};

pub const GPT_OSS_120B: ModelCapabilities = ModelCapabilities {
    max_tokens: 128_000,
    max_output_tokens: 16_000,
    default_temperature: 1.0,
    supports_tool_use: false,
    supports_images: false,
    supports_thinking: false,
    supports_adaptive_thinking: false,
    cache_configuration: None,
    extended_context_token_count: None,
    thinking_mode: BedrockModelMode::Default,
};
