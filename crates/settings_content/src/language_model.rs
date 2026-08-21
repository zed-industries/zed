use crate::merge_from::MergeFrom;
use collections::HashMap;
use language_model_core::ReasoningEffort;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings_macros::{MergeFrom, with_fallible_options};

use std::sync::Arc;

/// Different settings for specific language models.
#[with_fallible_options]
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema, MergeFrom)]
pub struct AllLanguageModelSettingsContent {
    /// Settings for the Anthropic provider.
    pub anthropic: Option<AnthropicSettingsContent>,
    /// Settings for Anthropic-compatible providers, keyed by provider name.
    pub anthropic_compatible: Option<HashMap<Arc<str>, AnthropicCompatibleSettingsContent>>,
    /// Settings for the Amazon Bedrock provider.
    pub bedrock: Option<AmazonBedrockSettingsContent>,
    /// Settings for the DeepSeek provider.
    pub deepseek: Option<DeepseekSettingsContent>,
    /// Settings for the Google AI provider.
    pub google: Option<GoogleSettingsContent>,
    /// Settings for the llama.cpp provider.
    #[serde(rename = "llama.cpp")]
    pub llama_cpp: Option<LlamaCppSettingsContent>,
    /// Settings for the LM Studio provider.
    pub lmstudio: Option<LmStudioSettingsContent>,
    /// Settings for the Mistral provider.
    pub mistral: Option<MistralSettingsContent>,
    /// Settings for the Ollama provider.
    pub ollama: Option<OllamaSettingsContent>,
    /// Settings for the OpenCode provider.
    pub opencode: Option<OpenCodeSettingsContent>,
    /// Settings for the OpenRouter provider.
    pub open_router: Option<OpenRouterSettingsContent>,
    /// Settings for the OpenAI provider.
    pub openai: Option<OpenAiSettingsContent>,
    /// Settings for OpenAI-compatible providers, keyed by provider name.
    pub openai_compatible: Option<HashMap<Arc<str>, OpenAiCompatibleSettingsContent>>,
    /// Settings for the Vercel AI Gateway provider.
    pub vercel_ai_gateway: Option<VercelAiGatewaySettingsContent>,
    /// Settings for the xAI provider.
    pub x_ai: Option<XAiSettingsContent>,
    /// Settings for the Zed hosted models provider.
    #[serde(rename = "zed.dev")]
    pub zed_dot_dev: Option<ZedDotDevSettingsContent>,
}

/// Settings for the Anthropic language model provider.
#[with_fallible_options]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema, MergeFrom)]
pub struct AnthropicSettingsContent {
    /// The API URL to use for this provider.
    ///
    /// Default: https://api.anthropic.com
    pub api_url: Option<String>,
    /// Custom models to make available for this provider.
    pub available_models: Option<Vec<AnthropicAvailableModel>>,
    /// Custom HTTP headers to include in requests to this provider's API.
    pub custom_headers: Option<HashMap<String, String>>,
}

/// Settings for an Anthropic-compatible language model provider.
#[with_fallible_options]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema, MergeFrom)]
pub struct AnthropicCompatibleSettingsContent {
    /// The API URL to use for this provider.
    pub api_url: String,
    /// The models available for this provider.
    pub available_models: Vec<AnthropicCompatibleAvailableModel>,
    /// Custom HTTP headers to include in requests to this provider's API.
    pub custom_headers: Option<HashMap<String, String>>,
}

/// A custom model to make available for an Anthropic-compatible provider.
#[with_fallible_options]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct AnthropicCompatibleAvailableModel {
    /// The model's name in the provider's API. e.g. claude-3-5-sonnet-latest
    pub name: String,
    /// The model's name in Zed's UI, such as in the model selector dropdown menu in the assistant panel.
    pub display_name: Option<String>,
    /// The model's context window size.
    pub max_tokens: u64,
    /// A model `name` to substitute when calling tools, in case the primary model doesn't support tool calling.
    pub tool_override: Option<String>,
    /// The maximum number of output tokens allowed by the model.
    pub max_output_tokens: Option<u64>,
    /// The default temperature to use for this model.
    #[serde(serialize_with = "crate::serialize_optional_f32_with_two_decimal_places")]
    pub default_temperature: Option<f32>,
    /// Any extra beta headers to provide when using the model.
    #[serde(default)]
    pub extra_beta_headers: Vec<String>,
    /// The model's mode (e.g. thinking)
    pub mode: Option<ModelMode>,
    /// The capabilities this model supports.
    #[serde(default)]
    pub capabilities: AnthropicCompatibleModelCapabilities,
}

/// The capabilities of an Anthropic-compatible model.
#[with_fallible_options]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct AnthropicCompatibleModelCapabilities {
    /// Whether the model supports tool calls.
    ///
    /// Default: true
    pub tools: bool,
    /// Whether the model supports image inputs.
    ///
    /// Default: false
    pub images: bool,
    /// Whether to send explicit `cache_control` breakpoints for prompt caching.
    /// Leave disabled if the provider rejects requests containing them.
    pub prompt_caching: bool,
}

impl Default for AnthropicCompatibleModelCapabilities {
    fn default() -> Self {
        Self {
            tools: true,
            images: false,
            prompt_caching: false,
        }
    }
}

/// A custom model to make available for the Anthropic provider.
#[with_fallible_options]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct AnthropicAvailableModel {
    /// The model's name in the Anthropic API. e.g. claude-3-5-sonnet-latest, claude-3-opus-20240229, etc
    pub name: String,
    /// The model's name in Zed's UI, such as in the model selector dropdown menu in the agent panel.
    pub display_name: Option<String>,
    /// The model's context window size.
    pub max_tokens: u64,
    /// A model `name` to substitute when calling tools, in case the primary model doesn't support tool calling.
    pub tool_override: Option<String>,
    /// Configuration of Anthropic's caching API.
    pub cache_configuration: Option<LanguageModelCacheConfiguration>,
    /// The maximum number of output tokens allowed by the model.
    pub max_output_tokens: Option<u64>,
    /// The default temperature to use for this model.
    #[serde(serialize_with = "crate::serialize_optional_f32_with_two_decimal_places")]
    pub default_temperature: Option<f32>,
    /// Any extra beta headers to provide when using the model.
    #[serde(default)]
    pub extra_beta_headers: Vec<String>,
    /// Whether Anthropic's fast mode is available for this model.
    pub supports_fast_mode: Option<bool>,
    /// The model's mode (e.g. thinking)
    pub mode: Option<ModelMode>,
}

/// Settings for the Amazon Bedrock language model provider.
#[with_fallible_options]
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema, MergeFrom)]
pub struct AmazonBedrockSettingsContent {
    /// Custom models to make available for this provider.
    pub available_models: Option<Vec<BedrockAvailableModel>>,
    /// Custom models served through the `bedrock-mantle` endpoint's
    /// OpenAI-compatible APIs, in addition to the built-in Mantle models
    /// (GPT-5.6 Sol, GPT-5.6 Terra, GPT-5.6 Luna, GPT-5.5, GPT-5.4, Grok 4.3).
    pub mantle_available_models: Option<Vec<BedrockMantleAvailableModel>>,
    /// Custom HTTP headers to include in requests to this provider's API.
    pub custom_headers: Option<HashMap<String, String>>,
    /// A custom endpoint URL to use for Bedrock API requests.
    pub endpoint_url: Option<String>,
    /// The AWS region to use for Bedrock API requests.
    pub region: Option<String>,
    /// The AWS profile name to use for authentication.
    pub profile: Option<String>,
    /// The authentication method to use for Bedrock API requests.
    pub authentication_method: Option<BedrockAuthMethodContent>,
    /// Whether to use the global cross-region inference profile for models that support it.
    pub allow_global: Option<bool>,
    /// The guardrail identifier (ARN or ID) to apply to Bedrock API requests.
    pub guardrail_identifier: Option<String>,
    /// The guardrail version to use. Defaults to "DRAFT" if not specified.
    pub guardrail_version: Option<String>,
}

/// A custom model to make available for the Amazon Bedrock provider.
#[with_fallible_options]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct BedrockAvailableModel {
    /// The model's name in the Bedrock API.
    pub name: String,
    /// The model's name in Zed's UI, such as in the model selector dropdown menu in the agent panel.
    pub display_name: Option<String>,
    /// The model's context window size.
    pub max_tokens: u64,
    /// Configuration of Bedrock's prompt caching.
    pub cache_configuration: Option<LanguageModelCacheConfiguration>,
    /// The maximum number of output tokens allowed by the model.
    pub max_output_tokens: Option<u64>,
    /// The default temperature to use for this model.
    #[serde(serialize_with = "crate::serialize_optional_f32_with_two_decimal_places")]
    pub default_temperature: Option<f32>,
    /// The model's mode (e.g. thinking)
    pub mode: Option<ModelMode>,
}

/// A custom model served through the Bedrock Mantle endpoint.
#[with_fallible_options]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct BedrockMantleAvailableModel {
    /// The model id as expected in Bedrock Mantle request bodies, e.g. `openai.gpt-5.5`.
    pub name: String,
    /// The model's name in Zed's UI, such as in the model selector dropdown menu in the agent panel.
    pub display_name: Option<String>,
    /// The model's context window size.
    pub max_tokens: u64,
    /// The maximum number of output tokens allowed by the model.
    pub max_output_tokens: Option<u64>,
    /// The OpenAI-compatible API this model must be called through.
    pub protocol: BedrockMantleProtocolContent,
    /// Whether the model supports tool calls.
    pub supports_tools: Option<bool>,
    /// Whether the model supports image inputs.
    pub supports_images: Option<bool>,
    /// Whether this custom Mantle model supports OpenAI reasoning effort parameters.
    pub supports_thinking: Option<bool>,
}

/// The OpenAI-compatible API a Bedrock Mantle model is called through.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub enum BedrockMantleProtocolContent {
    /// The OpenAI Chat Completions API (`/chat/completions`).
    #[serde(rename = "chat_completions")]
    ChatCompletions,
    /// The OpenAI Responses API (`/responses`).
    #[serde(rename = "responses")]
    Responses,
}

/// The authentication method to use for Amazon Bedrock.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub enum BedrockAuthMethodContent {
    /// Use AWS named profile from ~/.aws/credentials or ~/.aws/config.
    #[serde(rename = "named_profile")]
    NamedProfile,
    /// Use AWS SSO profile.
    #[serde(rename = "sso")]
    SingleSignOn,
    /// Use Bedrock API Key (bearer token authentication).
    #[serde(rename = "api_key")]
    ApiKey,
    /// IMDSv2, PodIdentity, env vars, etc.
    #[serde(rename = "default")]
    Automatic,
}

/// Settings for the Ollama language model provider.
#[with_fallible_options]
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema, MergeFrom)]
pub struct OllamaSettingsContent {
    /// The API URL to use for this provider.
    ///
    /// Default: http://localhost:11434
    pub api_url: Option<String>,
    /// Whether to automatically discover models served by the Ollama server.
    ///
    /// Default: true
    pub auto_discover: Option<bool>,
    /// Custom models to make available for this provider.
    pub available_models: Option<Vec<OllamaAvailableModel>>,
    /// Overrides the context window size for Ollama models.
    pub context_window: Option<u64>,
    /// Custom HTTP headers to include in requests to this provider's API.
    pub custom_headers: Option<HashMap<String, String>>,
}

/// A custom model to make available for the Ollama provider.
#[with_fallible_options]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct OllamaAvailableModel {
    /// The model name in the Ollama API (e.g. "llama3.2:latest")
    pub name: String,
    /// The model's name in Zed's UI, such as in the model selector dropdown menu in the agent panel.
    pub display_name: Option<String>,
    /// The Context Length parameter to the model (aka num_ctx or n_ctx)
    pub max_tokens: u64,
    /// The number of seconds to keep the connection open after the last request
    pub keep_alive: Option<KeepAlive>,
    /// Whether the model supports tools
    pub supports_tools: Option<bool>,
    /// Whether the model supports vision
    pub supports_images: Option<bool>,
    /// Whether to enable think mode
    pub supports_thinking: Option<bool>,
}

/// How long Ollama keeps a model loaded after the last request.
#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq, JsonSchema, MergeFrom)]
#[serde(untagged)]
pub enum KeepAlive {
    /// Keep model alive for N seconds
    Seconds(isize),
    /// Keep model alive for a fixed duration. Accepts durations like "5m", "10m", "1h", "1d", etc.
    Duration(String),
}

impl KeepAlive {
    /// Keep model alive until a new model is loaded or until Ollama shuts down
    pub fn indefinite() -> Self {
        Self::Seconds(-1)
    }
}

impl Default for KeepAlive {
    fn default() -> Self {
        Self::indefinite()
    }
}

/// Settings for the OpenCode language model provider.
#[with_fallible_options]
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema, MergeFrom)]
pub struct OpenCodeSettingsContent {
    /// The API URL to use for this provider.
    ///
    /// Default: https://opencode.ai/zen
    pub api_url: Option<String>,
    /// Custom models to make available for this provider.
    pub available_models: Option<Vec<OpenCodeAvailableModel>>,
    /// Custom HTTP headers to include in requests to this provider's API.
    pub custom_headers: Option<HashMap<String, String>>,
    /// Whether to show OpenCode Zen models. Defaults to true.
    pub show_zen_models: Option<bool>,
    /// Whether to show OpenCode Go models. Defaults to true.
    pub show_go_models: Option<bool>,
}

/// The API protocol to use for an OpenCode model.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub enum OpenCodeApiProtocol {
    /// The Anthropic Messages API.
    #[serde(rename = "anthropic")]
    Anthropic,
    /// The OpenAI Responses API.
    #[serde(rename = "openai_responses", alias = "open_ai_responses")]
    OpenAiResponses,
    /// The OpenAI Chat Completions API.
    #[serde(rename = "openai_chat", alias = "open_ai_chat")]
    OpenAiChat,
    /// The Google Gemini API.
    #[serde(rename = "google")]
    Google,
}

/// The OpenCode subscription a model belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
pub enum OpenCodeModelSubscription {
    /// The OpenCode Zen subscription.
    Zen,
    /// The OpenCode Go subscription.
    Go,
}

/// A custom model to make available for the OpenCode provider.
#[with_fallible_options]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct OpenCodeAvailableModel {
    /// The model's name in the OpenCode API.
    pub name: String,
    /// The model's name in Zed's UI, such as in the model selector dropdown menu in the agent panel.
    pub display_name: Option<String>,
    /// The model's context window size.
    pub max_tokens: u64,
    /// The maximum number of output tokens allowed by the model.
    pub max_output_tokens: Option<u64>,
    /// The API protocol to use for this model: "anthropic", "openai_responses", "openai_chat", or "google". Defaults to "openai_chat".
    pub protocol: Option<OpenCodeApiProtocol>,
    /// The subscription for this model: "zen" or "go". Defaults to Zen.
    pub subscription: Option<OpenCodeModelSubscription>,
    /// Custom Model API URL to use for this model.
    pub custom_model_api_url: Option<String>,
    /// Supported reasoning effort levels, for example `["low", "medium", "high"].
    pub reasoning_effort_levels: Option<Vec<ReasoningEffort>>,
    /// When using OpenAiChat protocol, whether thinking tokens are sent as a dedicated `reasoning_content` field or inline in message text.
    #[serde(default)]
    pub interleaved_reasoning: bool,
}

/// Settings for the LM Studio language model provider.
#[with_fallible_options]
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema, MergeFrom)]
pub struct LmStudioSettingsContent {
    /// The API URL to use for this provider.
    ///
    /// Default: http://localhost:1234/api/v0
    pub api_url: Option<String>,
    /// The API key to use when connecting to the LM Studio server.
    pub api_key: Option<String>,
    /// Custom models to make available for this provider.
    pub available_models: Option<Vec<LmStudioAvailableModel>>,
    /// Custom HTTP headers to include in requests to this provider's API.
    pub custom_headers: Option<HashMap<String, String>>,
}

/// A custom model to make available for the LM Studio provider.
#[with_fallible_options]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct LmStudioAvailableModel {
    /// The model's name in the LM Studio API.
    pub name: String,
    /// The model's name in Zed's UI, such as in the model selector dropdown menu in the agent panel.
    pub display_name: Option<String>,
    /// The model's context window size.
    pub max_tokens: u64,
    /// Whether the model supports tool calls.
    pub supports_tool_calls: bool,
    /// Whether the model supports image inputs.
    pub supports_images: bool,
}

/// Settings for the llama.cpp language model provider.
#[with_fallible_options]
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema, MergeFrom)]
pub struct LlamaCppSettingsContent {
    /// The API URL to use for this provider.
    ///
    /// Default: http://localhost:8080
    pub api_url: Option<String>,
    /// Whether to automatically discover models served by the llama.cpp server.
    /// Defaults to true.
    pub auto_discover: Option<bool>,
    /// Custom models to make available for this provider.
    pub available_models: Option<Vec<LlamaCppAvailableModel>>,
    /// Overrides the context length reported for every llama.cpp model.
    pub context_window: Option<u64>,
    /// Custom HTTP headers to include in requests to this provider's API.
    pub custom_headers: Option<HashMap<String, String>>,
}

/// A custom model to make available for the llama.cpp provider.
#[with_fallible_options]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct LlamaCppAvailableModel {
    /// The model id reported by the llama.cpp server (its `--alias` or the model file path).
    pub name: String,
    /// The model's name in Zed's UI, such as in the model selector dropdown menu in the agent panel.
    pub display_name: Option<String>,
    /// The Context Length parameter to the model (aka n_ctx).
    pub max_tokens: u64,
    /// Whether the model supports tools.
    pub supports_tools: Option<bool>,
    /// Whether the model supports vision.
    pub supports_images: Option<bool>,
    /// Whether the model emits reasoning/thinking content.
    pub supports_thinking: Option<bool>,
}

/// Settings for the DeepSeek language model provider.
#[with_fallible_options]
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema, MergeFrom)]
pub struct DeepseekSettingsContent {
    /// The API URL to use for this provider.
    ///
    /// Default: https://api.deepseek.com/v1
    pub api_url: Option<String>,
    /// Custom models to make available for this provider.
    pub available_models: Option<Vec<DeepseekAvailableModel>>,
    /// Custom HTTP headers to include in requests to this provider's API.
    pub custom_headers: Option<HashMap<String, String>>,
}

/// A custom model to make available for the DeepSeek provider.
#[with_fallible_options]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct DeepseekAvailableModel {
    /// The model's name in the DeepSeek API.
    pub name: String,
    /// The model's name in Zed's UI, such as in the model selector dropdown menu in the agent panel.
    pub display_name: Option<String>,
    /// The model's context window size.
    pub max_tokens: u64,
    /// The maximum number of output tokens allowed by the model.
    pub max_output_tokens: Option<u64>,
}

/// Settings for the Mistral language model provider.
#[with_fallible_options]
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema, MergeFrom)]
pub struct MistralSettingsContent {
    /// The API URL to use for this provider.
    ///
    /// Default: https://api.mistral.ai/v1
    pub api_url: Option<String>,
    /// Custom models to make available for this provider.
    pub available_models: Option<Vec<MistralAvailableModel>>,
    /// Custom HTTP headers to include in requests to this provider's API.
    pub custom_headers: Option<HashMap<String, String>>,
}

/// A custom model to make available for the Mistral provider.
#[with_fallible_options]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct MistralAvailableModel {
    /// The model's name in the Mistral API.
    pub name: String,
    /// The model's name in Zed's UI, such as in the model selector dropdown menu in the agent panel.
    pub display_name: Option<String>,
    /// The model's context window size.
    pub max_tokens: u64,
    /// The maximum number of output tokens allowed by the model.
    pub max_output_tokens: Option<u64>,
    /// The maximum number of completion tokens allowed by the model.
    pub max_completion_tokens: Option<u64>,
    /// Whether the model supports tool calls.
    pub supports_tools: Option<bool>,
    /// Whether the model supports image inputs.
    pub supports_images: Option<bool>,
    /// Whether the model emits reasoning/thinking content.
    pub supports_thinking: Option<bool>,
}

/// Settings for the OpenAI language model provider.
#[with_fallible_options]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema, MergeFrom)]
pub struct OpenAiSettingsContent {
    /// The API URL to use for this provider.
    ///
    /// Default: https://api.openai.com/v1
    pub api_url: Option<String>,
    /// Custom models to make available for this provider.
    pub available_models: Option<Vec<OpenAiAvailableModel>>,
    /// Custom HTTP headers to include in requests to this provider's API.
    pub custom_headers: Option<HashMap<String, String>>,
}

/// A custom model to make available for the OpenAI provider.
#[with_fallible_options]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct OpenAiAvailableModel {
    /// The model's name in the OpenAI API.
    pub name: String,
    /// The model's name in Zed's UI, such as in the model selector dropdown menu in the agent panel.
    pub display_name: Option<String>,
    /// The model's context window size.
    pub max_tokens: u64,
    /// The maximum number of output tokens allowed by the model.
    pub max_output_tokens: Option<u64>,
    /// The maximum number of completion tokens allowed by the model (o1-* only)
    pub max_completion_tokens: Option<u64>,
    /// The reasoning effort to use for this model.
    pub reasoning_effort: Option<OpenAiReasoningEffort>,
    /// The capabilities this model supports.
    #[serde(default)]
    pub capabilities: OpenAiModelCapabilities,
}

pub use language_model_core::ReasoningEffort as OpenAiReasoningEffort;

impl MergeFrom for OpenAiReasoningEffort {
    fn merge_from(&mut self, other: &Self) {
        *self = *other;
    }
}

/// Settings for an OpenAI-compatible language model provider.
#[with_fallible_options]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema, MergeFrom)]
pub struct OpenAiCompatibleSettingsContent {
    /// The API URL to use for this provider.
    pub api_url: String,
    /// The models available for this provider.
    pub available_models: Vec<OpenAiCompatibleAvailableModel>,
    /// Custom HTTP headers to include in requests to this provider's API.
    pub custom_headers: Option<HashMap<String, String>>,
}

/// The capabilities of an OpenAI model.
#[with_fallible_options]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct OpenAiModelCapabilities {
    /// Whether to call this model through the Chat Completions API. When false, the Responses API is used instead.
    ///
    /// Default: true
    #[serde(default = "default_true")]
    pub chat_completions: bool,
    /// Whether the model supports image inputs.
    ///
    /// Default: true
    #[serde(default = "default_true")]
    pub images: bool,
}

impl Default for OpenAiModelCapabilities {
    fn default() -> Self {
        Self {
            chat_completions: default_true(),
            images: default_true(),
        }
    }
}

/// A custom model to make available for an OpenAI-compatible provider.
#[with_fallible_options]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct OpenAiCompatibleAvailableModel {
    /// The model's name in the provider's API.
    pub name: String,
    /// The model's name in Zed's UI, such as in the model selector dropdown menu in the agent panel.
    pub display_name: Option<String>,
    /// The model's context window size.
    pub max_tokens: u64,
    /// The maximum number of output tokens allowed by the model.
    pub max_output_tokens: Option<u64>,
    /// The maximum number of completion tokens allowed by the model.
    pub max_completion_tokens: Option<u64>,
    /// The reasoning effort to use for this model.
    pub reasoning_effort: Option<OpenAiReasoningEffort>,
    /// The capabilities this model supports.
    #[serde(default)]
    pub capabilities: OpenAiCompatibleModelCapabilities,
}

/// The capabilities of an OpenAI-compatible model.
#[with_fallible_options]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct OpenAiCompatibleModelCapabilities {
    /// Whether the model supports tool calls.
    ///
    /// Default: true
    pub tools: bool,
    /// Whether the model supports image inputs.
    ///
    /// Default: false
    pub images: bool,
    /// Whether the model supports the `parallel_tool_calls` parameter.
    ///
    /// Default: false
    pub parallel_tool_calls: bool,
    /// Whether the model supports the `prompt_cache_key` parameter.
    ///
    /// Default: false
    pub prompt_cache_key: bool,
    /// Whether to call this model through the Chat Completions API. When false, the Responses API is used instead.
    ///
    /// Default: true
    #[serde(default = "default_true")]
    pub chat_completions: bool,
    /// Whether reasoning content is sent back to the model as reasoning details in Chat Completions requests.
    ///
    /// Default: false
    #[serde(default)]
    pub interleaved_reasoning: bool,
    /// Whether to send the `max_tokens` parameter instead of `max_completion_tokens` in Chat Completions requests.
    ///
    /// Default: false
    #[serde(default)]
    pub max_tokens_parameter: bool,
}

impl Default for OpenAiCompatibleModelCapabilities {
    fn default() -> Self {
        Self {
            tools: true,
            images: false,
            parallel_tool_calls: false,
            prompt_cache_key: false,
            chat_completions: default_true(),
            interleaved_reasoning: false,
            max_tokens_parameter: false,
        }
    }
}

/// Settings for the Vercel AI Gateway language model provider.
#[with_fallible_options]
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema, MergeFrom)]
pub struct VercelAiGatewaySettingsContent {
    /// The API URL to use for this provider.
    ///
    /// Default: https://ai-gateway.vercel.sh/v1
    pub api_url: Option<String>,
    /// Custom models to make available for this provider.
    pub available_models: Option<Vec<VercelAiGatewayAvailableModel>>,
    /// Custom HTTP headers to include in requests to this provider's API.
    pub custom_headers: Option<HashMap<String, String>>,
}

/// A custom model to make available for the Vercel AI Gateway provider.
#[with_fallible_options]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct VercelAiGatewayAvailableModel {
    /// The model's name in the Vercel AI Gateway API.
    pub name: String,
    /// The model's name in Zed's UI, such as in the model selector dropdown menu in the agent panel.
    pub display_name: Option<String>,
    /// The model's context window size.
    pub max_tokens: u64,
    /// The maximum number of output tokens allowed by the model.
    pub max_output_tokens: Option<u64>,
    /// The maximum number of completion tokens allowed by the model.
    pub max_completion_tokens: Option<u64>,
    /// The capabilities this model supports.
    #[serde(default)]
    pub capabilities: OpenAiCompatibleModelCapabilities,
}

/// Settings for the Google AI language model provider.
#[with_fallible_options]
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema, MergeFrom)]
pub struct GoogleSettingsContent {
    /// The API URL to use for this provider.
    ///
    /// Default: https://generativelanguage.googleapis.com
    pub api_url: Option<String>,
    /// Custom models to make available for this provider.
    pub available_models: Option<Vec<GoogleAvailableModel>>,
    /// Custom HTTP headers to include in requests to this provider's API.
    pub custom_headers: Option<HashMap<String, String>>,
}

/// A custom model to make available for the Google AI provider.
#[with_fallible_options]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct GoogleAvailableModel {
    /// The model's name in the Google AI API.
    pub name: String,
    /// The model's name in Zed's UI, such as in the model selector dropdown menu in the agent panel.
    pub display_name: Option<String>,
    /// The model's context window size.
    pub max_tokens: u64,
    /// The model's mode (e.g. thinking)
    pub mode: Option<ModelMode>,
}

/// Settings for the xAI language model provider.
#[with_fallible_options]
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema, MergeFrom)]
pub struct XAiSettingsContent {
    /// The API URL to use for this provider.
    ///
    /// Default: https://api.x.ai/v1
    pub api_url: Option<String>,
    /// Custom models to make available for this provider.
    pub available_models: Option<Vec<XaiAvailableModel>>,
    /// Custom HTTP headers to include in requests to this provider's API.
    pub custom_headers: Option<HashMap<String, String>>,
}

/// A custom model to make available for the xAI provider.
#[with_fallible_options]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct XaiAvailableModel {
    /// The model's name in the xAI API.
    pub name: String,
    /// The model's name in Zed's UI, such as in the model selector dropdown menu in the agent panel.
    pub display_name: Option<String>,
    /// The model's context window size.
    pub max_tokens: u64,
    /// The maximum number of output tokens allowed by the model.
    pub max_output_tokens: Option<u64>,
    /// The maximum number of completion tokens allowed by the model.
    pub max_completion_tokens: Option<u64>,
    /// Whether the model supports image inputs.
    pub supports_images: Option<bool>,
    /// Whether the model supports tool calls.
    pub supports_tools: Option<bool>,
    /// Whether the model supports the `parallel_tool_calls` parameter.
    pub parallel_tool_calls: Option<bool>,
}

/// Settings for the Zed hosted models provider.
#[with_fallible_options]
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema, MergeFrom)]
pub struct ZedDotDevSettingsContent {
    /// Custom models to make available for this provider.
    pub available_models: Option<Vec<ZedDotDevAvailableModel>>,
}

/// A custom model to make available for the Zed hosted models provider.
#[with_fallible_options]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct ZedDotDevAvailableModel {
    /// The provider of the language model.
    pub provider: ZedDotDevAvailableProvider,
    /// The model's name in the provider's API. e.g. claude-3-5-sonnet-20240620
    pub name: String,
    /// The name displayed in the UI, such as in the agent panel model dropdown menu.
    pub display_name: Option<String>,
    /// The size of the context window, indicating the maximum number of tokens the model can process.
    pub max_tokens: usize,
    /// The maximum number of output tokens allowed by the model.
    pub max_output_tokens: Option<u64>,
    /// The maximum number of completion tokens allowed by the model (o1-* only)
    pub max_completion_tokens: Option<u64>,
    /// Override this model with a different Anthropic model for tool calls.
    pub tool_override: Option<String>,
    /// Indicates whether this custom model supports caching.
    pub cache_configuration: Option<LanguageModelCacheConfiguration>,
    /// The default temperature to use for this model.
    #[serde(serialize_with = "crate::serialize_optional_f32_with_two_decimal_places")]
    pub default_temperature: Option<f32>,
    /// Any extra beta headers to provide when using the model.
    #[serde(default)]
    pub extra_beta_headers: Vec<String>,
    /// The model's mode (e.g. thinking)
    pub mode: Option<ModelMode>,
}

/// The upstream provider of a Zed hosted model.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "lowercase")]
pub enum ZedDotDevAvailableProvider {
    /// A model hosted by Anthropic.
    Anthropic,
    /// A model hosted by OpenAI.
    OpenAi,
    /// A model hosted by Google.
    Google,
}

/// Settings for the OpenRouter language model provider.
#[with_fallible_options]
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema, MergeFrom)]
pub struct OpenRouterSettingsContent {
    /// The API URL to use for this provider.
    ///
    /// Default: https://openrouter.ai/api/v1
    pub api_url: Option<String>,
    /// Custom models to make available for this provider.
    pub available_models: Option<Vec<OpenRouterAvailableModel>>,
    /// Custom HTTP headers to include in requests to this provider's API.
    pub custom_headers: Option<HashMap<String, String>>,
}

/// A custom model to make available for the OpenRouter provider.
#[with_fallible_options]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct OpenRouterAvailableModel {
    /// The model's name in the OpenRouter API.
    pub name: String,
    /// The model's name in Zed's UI, such as in the model selector dropdown menu in the agent panel.
    pub display_name: Option<String>,
    /// The model's context window size.
    pub max_tokens: u64,
    /// The maximum number of output tokens allowed by the model.
    pub max_output_tokens: Option<u64>,
    /// The maximum number of completion tokens allowed by the model.
    pub max_completion_tokens: Option<u64>,
    /// Whether the model supports tool calls.
    pub supports_tools: Option<bool>,
    /// Whether the model supports image inputs.
    pub supports_images: Option<bool>,
    /// The model's mode (e.g. thinking)
    pub mode: Option<ModelMode>,
    /// The provider routing preferences to use for this model.
    pub provider: Option<OpenRouterProvider>,
}

/// OpenRouter provider routing preferences for a model.
#[with_fallible_options]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct OpenRouterProvider {
    order: Option<Vec<String>>,
    #[serde(default = "default_true")]
    allow_fallbacks: bool,
    #[serde(default)]
    require_parameters: bool,
    #[serde(default)]
    data_collection: DataCollection,
    only: Option<Vec<String>>,
    ignore: Option<Vec<String>>,
    quantizations: Option<Vec<String>>,
    sort: Option<String>,
}

/// Whether OpenRouter may route requests to providers that collect data.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "lowercase")]
pub enum DataCollection {
    /// Allow the use of providers that may collect data.
    #[default]
    Allow,
    /// Disallow the use of providers that collect data.
    Disallow,
}

fn default_true() -> bool {
    true
}

/// Configuration for caching language model messages.
#[with_fallible_options]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct LanguageModelCacheConfiguration {
    /// The maximum number of cache anchors to set in a request.
    pub max_cache_anchors: usize,
    /// Whether to speculatively cache the conversation while the next message is composed.
    pub should_speculate: bool,
    /// The minimum number of total tokens required before caching is used.
    pub min_total_token: u64,
}

pub use language_model_core::ModelMode;

impl MergeFrom for ModelMode {
    fn merge_from(&mut self, other: &Self) {
        *self = *other;
    }
}

impl AllLanguageModelSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            anthropic: Some(AnthropicSettingsContent::defaults()),
            anthropic_compatible: Some(HashMap::default()),
            bedrock: Some(AmazonBedrockSettingsContent::defaults()),
            deepseek: Some(DeepseekSettingsContent::defaults()),
            google: Some(GoogleSettingsContent::defaults()),
            llama_cpp: Some(LlamaCppSettingsContent::defaults()),
            lmstudio: Some(LmStudioSettingsContent::defaults()),
            mistral: Some(MistralSettingsContent::defaults()),
            ollama: Some(OllamaSettingsContent::defaults()),
            opencode: Some(OpenCodeSettingsContent::defaults()),
            open_router: Some(OpenRouterSettingsContent::defaults()),
            openai: Some(OpenAiSettingsContent::defaults()),
            openai_compatible: Some(HashMap::default()),
            vercel_ai_gateway: Some(VercelAiGatewaySettingsContent::defaults()),
            x_ai: Some(XAiSettingsContent::defaults()),
            zed_dot_dev: Some(ZedDotDevSettingsContent::defaults()),
        }
    }
}

impl AnthropicSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            api_url: Some(String::from("https://api.anthropic.com")),
            available_models: None,
            custom_headers: None,
        }
    }
}

impl AmazonBedrockSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            available_models: None,
            mantle_available_models: None,
            custom_headers: None,
            endpoint_url: None,
            region: None,
            profile: None,
            authentication_method: None,
            allow_global: None,
            guardrail_identifier: None,
            guardrail_version: None,
        }
    }
}

impl DeepseekSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            api_url: Some(String::from("https://api.deepseek.com/v1")),
            available_models: None,
            custom_headers: None,
        }
    }
}

impl GoogleSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            api_url: Some(String::from("https://generativelanguage.googleapis.com")),
            available_models: None,
            custom_headers: None,
        }
    }
}

impl LlamaCppSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            api_url: Some(String::from("http://localhost:8080")),
            auto_discover: None,
            available_models: None,
            context_window: None,
            custom_headers: None,
        }
    }
}

impl LmStudioSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            api_url: Some(String::from("http://localhost:1234/api/v0")),
            api_key: None,
            available_models: None,
            custom_headers: None,
        }
    }
}

impl MistralSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            api_url: Some(String::from("https://api.mistral.ai/v1")),
            available_models: None,
            custom_headers: None,
        }
    }
}

impl OllamaSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            api_url: Some(String::from("http://localhost:11434")),
            auto_discover: None,
            available_models: None,
            context_window: None,
            custom_headers: None,
        }
    }
}

impl OpenCodeSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            api_url: Some(String::from("https://opencode.ai/zen")),
            available_models: None,
            custom_headers: None,
            show_zen_models: None,
            show_go_models: None,
        }
    }
}

impl OpenRouterSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            api_url: Some(String::from("https://openrouter.ai/api/v1")),
            available_models: None,
            custom_headers: None,
        }
    }
}

impl OpenAiSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            api_url: Some(String::from("https://api.openai.com/v1")),
            available_models: None,
            custom_headers: None,
        }
    }
}

impl VercelAiGatewaySettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            api_url: Some(String::from("https://ai-gateway.vercel.sh/v1")),
            available_models: None,
            custom_headers: None,
        }
    }
}

impl XAiSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            api_url: Some(String::from("https://api.x.ai/v1")),
            available_models: None,
            custom_headers: None,
        }
    }
}

impl ZedDotDevSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            available_models: None,
        }
    }
}
