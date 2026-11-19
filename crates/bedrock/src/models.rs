use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum BedrockAdaptiveThinkingEffort {
    Low,
    Medium,
    #[default]
    High,
    XHigh,
    Max,
}

impl BedrockAdaptiveThinkingEffort {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::XHigh => "xhigh",
            Self::Max => "max",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::XHigh => "XHigh",
            Self::Max => "Max",
        }
    }

    pub fn from_value(value: &str) -> Option<Self> {
        match value {
            "low" => Some(Self::Low),
            "medium" => Some(Self::Medium),
            "high" => Some(Self::High),
            "xhigh" => Some(Self::XHigh),
            "max" => Some(Self::Max),
            _ => None,
        }
    }
}

/// How a Converse model reasons, which decides the
/// `additionalModelRequestFields` sent with a request.
#[derive(Clone, Debug, PartialEq)]
pub enum ConverseThinking {
    /// Anthropic extended thinking with a token budget.
    Budget { budget_tokens: u64 },
    /// Anthropic adaptive thinking steered by an effort level. When
    /// `always_on`, the model rejects requests that turn thinking off.
    Adaptive {
        efforts: Vec<BedrockAdaptiveThinkingEffort>,
        default_effort: BedrockAdaptiveThinkingEffort,
        always_on: bool,
    },
    /// Reasoning steered through `reasoning.effort`.
    Reasoning {
        efforts: Vec<BedrockAdaptiveThinkingEffort>,
        default_effort: BedrockAdaptiveThinkingEffort,
        supports_none: bool,
    },
}

const DEFAULT_THINKING_BUDGET_TOKENS: u64 = 4_096;

impl ConverseThinking {
    pub fn budget(budget_tokens: Option<u64>) -> Self {
        Self::Budget {
            budget_tokens: budget_tokens.unwrap_or(DEFAULT_THINKING_BUDGET_TOKENS),
        }
    }

    /// Adaptive thinking with the `low`, `medium`, `high` and `max` efforts,
    /// and `xhigh` for the models that accept it.
    pub fn adaptive(supports_xhigh: bool, always_on: bool) -> Self {
        let mut efforts = vec![
            BedrockAdaptiveThinkingEffort::Low,
            BedrockAdaptiveThinkingEffort::Medium,
            BedrockAdaptiveThinkingEffort::High,
        ];
        if supports_xhigh {
            efforts.push(BedrockAdaptiveThinkingEffort::XHigh);
        }
        efforts.push(BedrockAdaptiveThinkingEffort::Max);
        Self::Adaptive {
            efforts,
            default_effort: BedrockAdaptiveThinkingEffort::default(),
            always_on,
        }
    }

    /// Whether every response from the model carries reasoning.
    pub fn is_always_on(&self) -> bool {
        match self {
            Self::Budget { .. } => false,
            Self::Adaptive { always_on, .. } => *always_on,
            Self::Reasoning { supports_none, .. } => !supports_none,
        }
    }

    pub fn efforts(&self) -> &[BedrockAdaptiveThinkingEffort] {
        match self {
            Self::Budget { .. } => &[],
            Self::Adaptive { efforts, .. } | Self::Reasoning { efforts, .. } => efforts,
        }
    }

    pub fn default_effort(&self) -> Option<BedrockAdaptiveThinkingEffort> {
        match self {
            Self::Budget { .. } => None,
            Self::Adaptive { default_effort, .. } | Self::Reasoning { default_effort, .. } => {
                Some(*default_effort)
            }
        }
    }
}

// This frozen table exists only so selections saved by earlier versions keep
// resolving. New models must never be added to it.
pub const LEGACY_MODEL_IDS: &[(&str, &str)] = &[
    ("claude-fable-5-1", "anthropic.claude-fable-5-1"),
    ("claude-fable-5", "anthropic.claude-fable-5"),
    ("claude-opus-5-5", "anthropic.claude-opus-5-5"),
    ("claude-opus-5", "anthropic.claude-opus-5"),
    ("claude-opus-4-8", "anthropic.claude-opus-4-8"),
    ("claude-opus-4-7", "anthropic.claude-opus-4-7"),
    ("claude-opus-4-6", "anthropic.claude-opus-4-6-v1"),
    ("claude-opus-4-5", "anthropic.claude-opus-4-5-20251101-v1:0"),
    ("claude-opus-4-1", "anthropic.claude-opus-4-1-20250805-v1:0"),
    ("claude-sonnet-5", "anthropic.claude-sonnet-5"),
    ("claude-sonnet-4-6", "anthropic.claude-sonnet-4-6"),
    (
        "claude-sonnet-4-5",
        "anthropic.claude-sonnet-4-5-20250929-v1:0",
    ),
    ("claude-sonnet-4", "anthropic.claude-sonnet-4-20250514-v1:0"),
    (
        "claude-haiku-4-5",
        "anthropic.claude-haiku-4-5-20251001-v1:0",
    ),
    ("llama-4-scout-17b", "meta.llama4-scout-17b-instruct-v1:0"),
    (
        "llama-4-maverick-17b",
        "meta.llama4-maverick-17b-instruct-v1:0",
    ),
    ("gemma-3-4b", "google.gemma-3-4b-it"),
    ("gemma-3-12b", "google.gemma-3-12b-it"),
    ("gemma-3-27b", "google.gemma-3-27b-it"),
    ("magistral-small", "mistral.magistral-small-2509"),
    ("mistral-large-3", "mistral.mistral-large-3-675b-instruct"),
    ("pixtral-large", "mistral.pixtral-large-2502-v1:0"),
    ("devstral-2-123b", "mistral.devstral-2-123b"),
    ("ministral-14b", "mistral.ministral-3-14b-instruct"),
    ("qwen3-32b", "qwen.qwen3-32b-v1:0"),
    ("qwen3-vl-235b", "qwen.qwen3-vl-235b-a22b"),
    ("qwen3-235b", "qwen.qwen3-235b-a22b-2507-v1:0"),
    ("qwen3-next-80b", "qwen.qwen3-next-80b-a3b"),
    ("qwen3-coder-30b", "qwen.qwen3-coder-30b-a3b-v1:0"),
    ("qwen3-coder-next", "qwen.qwen3-coder-next"),
    ("qwen3-coder-480b", "qwen.qwen3-coder-480b-a35b-v1:0"),
    ("nova-lite", "amazon.nova-lite-v1:0"),
    ("nova-pro", "amazon.nova-pro-v1:0"),
    ("nova-premier", "amazon.nova-premier-v1:0"),
    ("nova-2-lite", "amazon.nova-2-lite-v1:0"),
    ("gpt-6-astra", "openai.gpt-6-astra"),
    ("gpt-oss-20b", "openai.gpt-oss-20b-1:0"),
    ("gpt-oss-120b", "openai.gpt-oss-120b-1:0"),
    ("nemotron-super-3-120b", "nvidia.nemotron-super-3-120b"),
    ("nemotron-nano-3-30b", "nvidia.nemotron-nano-3-30b"),
    ("minimax-m2", "minimax.minimax-m2"),
    ("minimax-m2-1", "minimax.minimax-m2.1"),
    ("minimax-m2-5", "minimax.minimax-m2.5"),
    ("glm-5", "zai.glm-5"),
    ("glm-4-7", "zai.glm-4.7"),
    ("glm-4-7-flash", "zai.glm-4.7-flash"),
    ("kimi-k2-thinking", "moonshot.kimi-k2-thinking"),
    ("kimi-k2-5", "moonshotai.kimi-k2.5"),
    ("deepseek-r1", "deepseek.r1-v1:0"),
    ("deepseek-v3", "deepseek.v3-v1:0"),
    ("deepseek-v3-2", "deepseek.v3.2"),
    ("gpt-5.6-sol", "openai.gpt-5.6-sol"),
    ("gpt-5.6-terra", "openai.gpt-5.6-terra"),
    ("gpt-5.6-luna", "openai.gpt-5.6-luna"),
    ("gpt-5.5", "openai.gpt-5.5"),
    ("gpt-5.4", "openai.gpt-5.4"),
    ("grok-4.3", "xai.grok-4.3"),
];

pub fn legacy_model_id(base_model_id: &str) -> Option<&'static str> {
    LEGACY_MODEL_IDS
        .iter()
        .find_map(|(legacy, base)| (*base == base_model_id).then_some(*legacy))
}

pub fn stable_model_id(base_model_id: &str) -> &str {
    legacy_model_id(base_model_id).unwrap_or(base_model_id)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConverseModelRoute {
    Automatic,
    Pinned,
}

/// A model served by the Converse API on `bedrock-runtime`.
#[derive(Clone, Debug, PartialEq)]
pub struct ConverseModel {
    /// The id Zed stores for the model. For a model discovered through its
    /// inference profiles this is the legacy or foundation model id, which stays the
    /// same whichever profile requests are routed through.
    pub id: String,
    /// The model id, inference profile id or ARN sent with each request.
    pub request_id: String,
    /// The foundation model that `request_id` resolves to, as far as it is
    /// known. Model-specific request handling is keyed on this.
    pub base_model_id: String,
    /// Established by the listing or public catalog, never inferred from an id's shape.
    pub foundation_model_known: bool,
    pub route: ConverseModelRoute,
    pub display_name: String,
    pub max_tokens: u64,
    /// The output limit, when known. Without one, requests leave the limit to
    /// Bedrock, which applies its own default.
    pub max_output_tokens: Option<u64>,
    pub default_temperature: Option<f32>,
    /// Whether the model accepts a `temperature` at all.
    pub supports_temperature: bool,
    pub supports_tool_use: bool,
    pub supports_images: bool,
    pub supports_caching: bool,
    pub thinking: Option<ConverseThinking>,
    /// A settings entry may disable optional thinking while retaining the
    /// model's wire mode so the request can send its explicit off value.
    pub thinking_disabled: bool,
}

/// The wire protocol used to talk to a [`MantleModel`] on the `bedrock-mantle` endpoint.
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum MantleProtocol {
    /// The OpenAI Chat Completions API (`/chat/completions`).
    #[default]
    ChatCompletions,
    /// The OpenAI Responses API (`/responses`).
    Responses,
}

/// A model served by the `bedrock-mantle` endpoint's OpenAI-compatible APIs.
#[derive(Clone, Debug, PartialEq)]
pub struct MantleModel {
    pub name: String,
    pub known_foundation_model_id: Option<String>,
    pub display_name: Option<String>,
    pub max_tokens: u64,
    pub max_output_tokens: Option<u64>,
    pub protocol: MantleProtocol,
    pub supports_tools: bool,
    pub supports_images: bool,
    pub supports_thinking: bool,
}

impl MantleModel {
    /// The model id Zed uses internally (also used as the `name` in settings).
    pub fn id(&self) -> &str {
        &self.name
    }

    /// The model id as expected in Bedrock Mantle request bodies, e.g. `openai.gpt-5.5`.
    pub fn request_id(&self) -> &str {
        &self.name
    }

    pub fn display_name(&self) -> &str {
        self.display_name.as_deref().unwrap_or(self.name.as_str())
    }

    /// Which OpenAI-compatible API this model must be called through.
    pub fn protocol(&self) -> MantleProtocol {
        self.protocol
    }

    pub fn max_token_count(&self) -> u64 {
        self.max_tokens
    }

    pub fn max_output_tokens(&self) -> u64 {
        self.max_output_tokens.unwrap_or(4_096)
    }

    pub fn supports_tools(&self) -> bool {
        self.supports_tools
    }

    pub fn supports_images(&self) -> bool {
        self.supports_images
    }

    pub fn supports_thinking(&self) -> bool {
        self.supports_thinking
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_selections_have_unique_ids_and_foundation_models() {
        let mut ids = std::collections::HashSet::new();
        let mut bases = std::collections::HashSet::new();
        for (id, base) in LEGACY_MODEL_IDS {
            assert!(ids.insert(id), "duplicate legacy id: {id}");
            assert!(bases.insert(base), "duplicate foundation id: {base}");
            assert_eq!(stable_model_id(base), *id);
        }
        assert_eq!(stable_model_id("vendor.chat"), "vendor.chat");
    }
}
