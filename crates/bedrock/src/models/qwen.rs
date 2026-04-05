use super::{BedrockModelMode, ModelCapabilities};

pub const QWEN3_32B: ModelCapabilities = ModelCapabilities {
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

pub const QWEN3_VL_235B: ModelCapabilities = ModelCapabilities {
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

pub const QWEN3_235B: ModelCapabilities = ModelCapabilities {
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

pub const QWEN3_NEXT_80B: ModelCapabilities = ModelCapabilities {
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

pub const QWEN3_CODER_30B: ModelCapabilities = ModelCapabilities {
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

pub const QWEN3_CODER_NEXT: ModelCapabilities = ModelCapabilities {
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

pub const QWEN3_CODER_480B: ModelCapabilities = ModelCapabilities {
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
