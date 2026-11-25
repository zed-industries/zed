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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConverseModelRoute {
    Automatic,
    Pinned,
}

/// A model served by the Converse API on `bedrock-runtime`.
#[derive(Clone, Debug, PartialEq)]
pub struct ConverseModel {
    /// The id Zed stores for the model. For a model discovered through its
    /// inference profiles this is the foundation model id, which stays the
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
