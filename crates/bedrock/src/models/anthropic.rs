use super::{
    BedrockAdaptiveThinkingEffort, BedrockModelCacheConfiguration, BedrockModelMode,
    ModelCapabilities,
};

pub const CLAUDE_HAIKU_4_5: ModelCapabilities = ModelCapabilities {
    max_tokens: 200_000,
    max_output_tokens: 64_000,
    default_temperature: 1.0,
    supports_tool_use: true,
    supports_images: true,
    supports_thinking: true,
    supports_adaptive_thinking: false,
    cache_configuration: Some(BedrockModelCacheConfiguration {
        max_cache_anchors: 4,
        min_total_token: 2048,
    }),
    extended_context_token_count: None,
    thinking_mode: BedrockModelMode::Thinking {
        budget_tokens: Some(4096),
    },
};

pub const CLAUDE_SONNET_4: ModelCapabilities = ModelCapabilities {
    max_tokens: 200_000,
    max_output_tokens: 64_000,
    default_temperature: 1.0,
    supports_tool_use: true,
    supports_images: true,
    supports_thinking: true,
    supports_adaptive_thinking: false,
    cache_configuration: Some(BedrockModelCacheConfiguration {
        max_cache_anchors: 4,
        min_total_token: 1024,
    }),
    extended_context_token_count: Some(1_000_000),
    thinking_mode: BedrockModelMode::Thinking {
        budget_tokens: Some(4096),
    },
};

pub const CLAUDE_SONNET_4_5: ModelCapabilities = ModelCapabilities {
    max_tokens: 200_000,
    max_output_tokens: 64_000,
    default_temperature: 1.0,
    supports_tool_use: true,
    supports_images: true,
    supports_thinking: true,
    supports_adaptive_thinking: false,
    cache_configuration: Some(BedrockModelCacheConfiguration {
        max_cache_anchors: 4,
        min_total_token: 1024,
    }),
    extended_context_token_count: Some(1_000_000),
    thinking_mode: BedrockModelMode::Thinking {
        budget_tokens: Some(4096),
    },
};

pub const CLAUDE_OPUS_4_1: ModelCapabilities = ModelCapabilities {
    max_tokens: 200_000,
    max_output_tokens: 32_000,
    default_temperature: 1.0,
    supports_tool_use: true,
    supports_images: true,
    supports_thinking: true,
    supports_adaptive_thinking: false,
    cache_configuration: Some(BedrockModelCacheConfiguration {
        max_cache_anchors: 4,
        min_total_token: 1024,
    }),
    extended_context_token_count: None,
    thinking_mode: BedrockModelMode::Thinking {
        budget_tokens: Some(4096),
    },
};

pub const CLAUDE_OPUS_4_5: ModelCapabilities = ModelCapabilities {
    max_tokens: 200_000,
    max_output_tokens: 64_000,
    default_temperature: 1.0,
    supports_tool_use: true,
    supports_images: true,
    supports_thinking: true,
    supports_adaptive_thinking: false,
    cache_configuration: Some(BedrockModelCacheConfiguration {
        max_cache_anchors: 4,
        min_total_token: 1024,
    }),
    extended_context_token_count: Some(1_000_000),
    thinking_mode: BedrockModelMode::Thinking {
        budget_tokens: Some(4096),
    },
};

pub const CLAUDE_OPUS_4_6: ModelCapabilities = ModelCapabilities {
    max_tokens: 200_000,
    max_output_tokens: 128_000,
    default_temperature: 1.0,
    supports_tool_use: true,
    supports_images: true,
    supports_thinking: true,
    supports_adaptive_thinking: true,
    cache_configuration: Some(BedrockModelCacheConfiguration {
        max_cache_anchors: 4,
        min_total_token: 1024,
    }),
    extended_context_token_count: Some(1_000_000),
    thinking_mode: BedrockModelMode::AdaptiveThinking {
        effort: BedrockAdaptiveThinkingEffort::High,
    },
};

pub const CLAUDE_SONNET_4_6: ModelCapabilities = ModelCapabilities {
    max_tokens: 200_000,
    max_output_tokens: 64_000,
    default_temperature: 1.0,
    supports_tool_use: true,
    supports_images: true,
    supports_thinking: true,
    supports_adaptive_thinking: true,
    cache_configuration: Some(BedrockModelCacheConfiguration {
        max_cache_anchors: 4,
        min_total_token: 1024,
    }),
    extended_context_token_count: Some(1_000_000),
    thinking_mode: BedrockModelMode::AdaptiveThinking {
        effort: BedrockAdaptiveThinkingEffort::High,
    },
};
