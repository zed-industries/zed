use super::{BedrockModelMode, ModelCapabilities};

pub const MAGISTRAL_SMALL: ModelCapabilities = ModelCapabilities {
    max_tokens: 128_000,
    max_output_tokens: 8_192,
    default_temperature: 1.0,
    supports_tool_use: true,
    supports_images: false,
    supports_thinking: false,
    supports_adaptive_thinking: false,
    cache_configuration: None,
    extended_context_token_count: None,
    thinking_mode: BedrockModelMode::Default,
};

pub const MISTRAL_LARGE_3: ModelCapabilities = ModelCapabilities {
    max_tokens: 128_000,
    max_output_tokens: 8_192,
    default_temperature: 1.0,
    supports_tool_use: true,
    supports_images: false,
    supports_thinking: false,
    supports_adaptive_thinking: false,
    cache_configuration: None,
    extended_context_token_count: None,
    thinking_mode: BedrockModelMode::Default,
};

pub const PIXTRAL_LARGE: ModelCapabilities = ModelCapabilities {
    max_tokens: 128_000,
    max_output_tokens: 8_192,
    default_temperature: 1.0,
    supports_tool_use: true,
    supports_images: true,
    supports_thinking: false,
    supports_adaptive_thinking: false,
    cache_configuration: None,
    extended_context_token_count: None,
    thinking_mode: BedrockModelMode::Default,
};
