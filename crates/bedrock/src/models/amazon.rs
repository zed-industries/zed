use super::{BedrockModelMode, ModelCapabilities};

pub const NOVA_LITE: ModelCapabilities = ModelCapabilities {
    max_tokens: 300_000,
    max_output_tokens: 5_000,
    default_temperature: 1.0,
    supports_tool_use: true,
    supports_images: true,
    supports_thinking: false,
    supports_adaptive_thinking: false,
    cache_configuration: None,
    extended_context_token_count: None,
    thinking_mode: BedrockModelMode::Default,
};

pub const NOVA_PRO: ModelCapabilities = ModelCapabilities {
    max_tokens: 300_000,
    max_output_tokens: 5_000,
    default_temperature: 1.0,
    supports_tool_use: true,
    supports_images: true,
    supports_thinking: false,
    supports_adaptive_thinking: false,
    cache_configuration: None,
    extended_context_token_count: None,
    thinking_mode: BedrockModelMode::Default,
};

pub const NOVA_PREMIER: ModelCapabilities = ModelCapabilities {
    max_tokens: 1_000_000,
    max_output_tokens: 5_000,
    default_temperature: 1.0,
    supports_tool_use: true,
    supports_images: false,
    supports_thinking: false,
    supports_adaptive_thinking: false,
    cache_configuration: None,
    extended_context_token_count: None,
    thinking_mode: BedrockModelMode::Default,
};

pub const NOVA_2_LITE: ModelCapabilities = ModelCapabilities {
    max_tokens: 300_000,
    max_output_tokens: 5_000,
    default_temperature: 1.0,
    supports_tool_use: true,
    supports_images: false,
    supports_thinking: false,
    supports_adaptive_thinking: false,
    cache_configuration: None,
    extended_context_token_count: None,
    thinking_mode: BedrockModelMode::Default,
};
