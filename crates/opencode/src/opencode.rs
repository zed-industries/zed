use anyhow::{Result, anyhow};
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::{
    AsyncBody, CustomHeaders, HttpClient, Method, Request as HttpRequest, RequestBuilderExt,
};
use language_model_core::ReasoningEffort;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub const OPENCODE_API_URL: &str = "https://opencode.ai/zen";

pub const MODELS_DEV_API_URL: &str = "https://models.dev/api.json";
pub const MODELS_DEV_FETCH_TIMEOUT: Duration = Duration::from_secs(30);
pub const MODELS_DEV_MAX_FETCH_ATTEMPTS: u32 = 4;
pub const MODELS_DEV_MAX_RESPONSE_SIZE: u64 = 10 * 1024 * 1024; // 10MB

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum ApiProtocol {
    #[default]
    #[serde(rename = "anthropic")]
    Anthropic,
    #[serde(rename = "openai_responses", alias = "open_ai_responses")]
    OpenAiResponses,
    #[serde(rename = "openai_chat", alias = "open_ai_chat")]
    OpenAiChat,
    #[serde(rename = "google")]
    Google,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum OpenCodeSubscription {
    Zen,
    Go,
    Free,
}

impl OpenCodeSubscription {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Zen => "Zen",
            Self::Go => "Go",
            Self::Free => "Free",
        }
    }

    pub fn id_prefix(&self) -> &'static str {
        match self {
            Self::Zen => "zen",
            Self::Go => "go",
            Self::Free => "free",
        }
    }

    pub fn api_path_suffix(&self) -> &'static str {
        match self {
            Self::Zen | Self::Free => "",
            Self::Go => "/go",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Model {
    pub id: String,
    pub name: String,
    pub max_tokens: u64,
    pub max_output_tokens: Option<u64>,
    pub protocol: ApiProtocol,
    pub supports_images: bool,
    pub supports_tools: bool,
    pub reasoning_effort_levels: Option<Vec<ReasoningEffort>>,
    pub interleaved_reasoning: bool,
    pub cost_input: Option<f64>,
    pub cost_output: Option<f64>,
    pub custom_api_url: Option<String>,
    pub disabled: Option<String>,
}

impl Model {
    pub fn display_name(&self) -> &str {
        &self.name
    }

    pub fn is_free(&self) -> bool {
        self.cost_input == Some(0.0) && self.cost_output == Some(0.0)
    }

    pub fn new_disabled(id: &str, name: &str, reason: String) -> Self {
        // Placeholder model to show in case of parsing errors
        Self {
            id: id.to_string(),
            name: name.to_string(),
            max_tokens: 123_456,
            max_output_tokens: Some(12_345),
            protocol: ApiProtocol::OpenAiChat,
            supports_images: false,
            supports_tools: false,
            reasoning_effort_levels: None,
            interleaved_reasoning: false,
            cost_input: None,
            cost_output: None,
            custom_api_url: None,
            disabled: Some(reason),
        }
    }
}

pub(crate) enum ModelParseResult {
    Success(Model),
    Deprecated,
    Failed,
}

mod models_dev {
    // See https://github.com/anomalyco/models.dev#schema-reference
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub(super) struct Model {
        pub(super) name: String,
        #[serde(default)]
        pub(super) status: Option<String>,
        #[serde(default)]
        pub(super) provider: Option<Protocol>,
        #[serde(default)]
        pub(super) limit: Option<Limit>,
        #[serde(default)]
        pub(super) reasoning: Option<bool>,
        #[serde(default)]
        pub(super) reasoning_options: Option<Vec<Reasoning>>,
        #[serde(default, deserialize_with = "deserialize_interleaved")]
        pub(super) interleaved: bool,
        #[serde(default)]
        pub(super) tool_call: Option<bool>,
        #[serde(default)]
        #[allow(dead_code)]
        pub(super) attachment: bool,
        #[serde(default)]
        pub(super) modalities: Option<Modalities>,
        #[serde(default)]
        pub(super) cost: Option<Cost>,
    }

    #[derive(Debug, Deserialize)]
    pub(super) struct Protocol {
        #[serde(default)]
        pub(super) npm: Option<String>,
    }

    #[derive(Debug, Deserialize)]
    pub(super) struct Limit {
        #[serde(default)]
        pub(super) context: Option<u64>,
        #[serde(default)]
        pub(super) input: Option<u64>,
        #[serde(default)]
        pub(super) output: Option<u64>,
    }

    #[derive(Debug, Deserialize)]
    pub(super) struct Reasoning {
        #[serde(rename = "type")]
        pub(super) opt_type: String,
        #[serde(default)]
        pub(super) values: Option<Vec<String>>,
    }

    fn deserialize_interleaved<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        Ok(match value {
            serde_json::Value::Bool(b) => b,
            serde_json::Value::Object(_) => true,
            _ => false,
        })
    }

    #[derive(Debug, Deserialize)]
    pub(super) struct Modalities {
        #[serde(default)]
        pub(super) input: Vec<String>,
    }

    #[derive(Debug, Deserialize)]
    pub(super) struct Cost {
        #[serde(default)]
        pub(super) input: Option<f64>,
        #[serde(default)]
        pub(super) output: Option<f64>,
    }
}

pub(crate) fn extract_model_config(
    model_id: &str,
    entry: serde_json::Value,
    parent_npm: Option<&str>,
) -> ModelParseResult {
    let entry: models_dev::Model = match serde_json::from_value(entry) {
        Ok(e) => e,
        Err(err) => {
            log::error!("Failed to parse model entry {model_id}: {err}");
            return ModelParseResult::Failed;
        }
    };

    if entry.status.as_deref() == Some("deprecated") {
        return ModelParseResult::Deprecated;
    }

    let effective_npm = entry
        .provider
        .as_ref()
        .and_then(|p| p.npm.as_deref())
        .or(parent_npm);

    let protocol = match effective_npm {
        Some("@ai-sdk/anthropic") => ApiProtocol::Anthropic,
        Some("@ai-sdk/google") => ApiProtocol::Google,
        Some("@ai-sdk/openai") => ApiProtocol::OpenAiResponses,
        Some("@ai-sdk/openai-compatible") => ApiProtocol::OpenAiChat,
        _ => ApiProtocol::OpenAiChat,
    };

    let max_tokens = entry
        .limit
        .as_ref()
        .and_then(|l| l.input.or(l.context))
        .unwrap_or(200_000);

    let max_output_tokens = entry.limit.as_ref().and_then(|l| l.output);

    let reasoning_effort_levels =
        compute_reasoning_efforts(entry.reasoning, &entry.reasoning_options);

    let interleaved_reasoning = entry.interleaved;

    ModelParseResult::Success(Model {
        id: model_id.to_string(),
        name: entry.name,
        max_tokens,
        max_output_tokens,
        protocol,
        supports_images: entry
            .modalities
            .as_ref()
            .map(|m| m.input.iter().any(|s| s == "image"))
            .unwrap_or(false),
        supports_tools: entry.tool_call.unwrap_or(true),
        reasoning_effort_levels,
        interleaved_reasoning,
        cost_input: entry.cost.as_ref().and_then(|c| c.input),
        cost_output: entry.cost.as_ref().and_then(|c| c.output),
        custom_api_url: None,
        disabled: None,
    })
}

fn compute_reasoning_efforts(
    supports_reasoning: Option<bool>,
    reasoning_options: &Option<Vec<models_dev::Reasoning>>,
) -> Option<Vec<ReasoningEffort>> {
    if supports_reasoning == Some(false) {
        return None;
    }

    let options = reasoning_options.as_deref().unwrap_or(&[]);
    if options.is_empty() {
        return supports_reasoning.map(|_| Vec::new());
    }

    let mut levels: Vec<ReasoningEffort> = Vec::new();
    let mut has_reasoning_toggle = false;

    for opt in options {
        match opt.opt_type.as_str() {
            "effort" => {
                if let Some(ref values) = opt.values {
                    for value in values {
                        if let Ok(effort) = value.parse::<ReasoningEffort>() {
                            if !levels.contains(&effort) {
                                levels.push(effort);
                            }
                        }
                    }
                }
            }
            "toggle" => {
                has_reasoning_toggle = true;
            }
            "budget_tokens" => {}
            _ => {}
        }
    }

    if has_reasoning_toggle && !levels.contains(&ReasoningEffort::None) {
        levels.push(ReasoningEffort::None);
    }

    if levels.is_empty() {
        supports_reasoning.map(|_| Vec::new())
    } else {
        Some(levels)
    }
}

#[derive(Debug, Deserialize)]
pub struct ModelsDevResponse {
    pub opencode: Option<ProviderEntries>,
    #[serde(rename = "opencode-go")]
    pub opencode_go: Option<ProviderEntries>,
}

#[derive(Debug, Deserialize)]
pub struct ProviderEntries {
    #[serde(default)]
    pub npm: Option<String>,
    #[serde(default)]
    pub models: Option<serde_json::Map<String, serde_json::Value>>,
}

pub fn parse_models_json(
    response: ModelsDevResponse,
) -> Vec<(String, Model, OpenCodeSubscription)> {
    let mut entries: Vec<(String, Model, OpenCodeSubscription)> = Vec::new();

    if let Some(opencode_data) = response.opencode {
        if let Some(models_map) = opencode_data.models {
            let parent_npm = opencode_data.npm.as_deref();
            for (model_id, entry) in models_map {
                let model_name = entry
                    .get("name")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_owned())
                    .unwrap_or_else(|| model_id.clone());
                match extract_model_config(&model_id, entry, parent_npm) {
                    ModelParseResult::Success(model) => {
                        let subscription = if model.is_free() {
                            OpenCodeSubscription::Free
                        } else {
                            OpenCodeSubscription::Zen
                        };
                        entries.push((model_id, model, subscription));
                    }
                    ModelParseResult::Deprecated => {}
                    ModelParseResult::Failed => {
                        log::error!(
                            "model {model_name} ({model_id}) failed to parse, showing as disabled"
                        );
                        let disabled_model = Model::new_disabled(
                            &model_id,
                            &model_name,
                            "Failed parsing models.dev data".to_string(),
                        );
                        entries.push((model_id, disabled_model, OpenCodeSubscription::Zen));
                    }
                }
            }
        }
    }

    if let Some(opencode_go_data) = response.opencode_go {
        if let Some(models_map) = opencode_go_data.models {
            let parent_npm = opencode_go_data.npm.as_deref();
            for (model_id, entry) in models_map {
                let model_name = entry
                    .get("name")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_owned())
                    .unwrap_or_else(|| model_id.clone());
                match extract_model_config(&model_id, entry, parent_npm) {
                    ModelParseResult::Success(model) => {
                        entries.push((model_id.clone(), model, OpenCodeSubscription::Go));
                    }
                    ModelParseResult::Deprecated => {}
                    ModelParseResult::Failed => {
                        log::error!(
                            "Go model {model_name} ({model_id}) failed to parse, showing as disabled"
                        );
                        let disabled_model = Model::new_disabled(
                            &model_id,
                            &model_name,
                            "Failed parsing models.dev data".to_string(),
                        );
                        entries.push((model_id, disabled_model, OpenCodeSubscription::Go));
                    }
                }
            }
        }
    }

    entries
}

/// Stream generate content for Google models via OpenCode.
///
/// Unlike `google_ai::stream_generate_content()`, this uses:
/// - `/v1/models/{model}` path (not `/v1beta/models/{model}`)
/// - `Authorization: Bearer` header (not `key=` query param)
pub async fn stream_generate_content(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: google_ai::GenerateContentRequest,
    extra_headers: &CustomHeaders,
) -> Result<BoxStream<'static, Result<google_ai::GenerateContentResponse>>> {
    let api_key = api_key.trim();

    let model_id = &request.model.model_id;

    let uri = format!("{api_url}/v1/models/{model_id}:streamGenerateContent?alt=sse");

    let request = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {api_key}"))
        .extra_headers(extra_headers)
        .body(AsyncBody::from(serde_json::to_string(&request)?))?;
    let mut response = client.send(request).await?;
    if response.status().is_success() {
        let reader = BufReader::new(response.into_body());
        Ok(reader
            .lines()
            .filter_map(|line| async move {
                match line {
                    Ok(line) => {
                        if let Some(line) = line.strip_prefix("data: ") {
                            match serde_json::from_str(line) {
                                Ok(response) => Some(Ok(response)),
                                Err(error) => {
                                    Some(Err(anyhow!("Error parsing JSON: {error:?}\n{line:?}")))
                                }
                            }
                        } else {
                            None
                        }
                    }
                    Err(error) => Some(Err(anyhow!(error))),
                }
            })
            .boxed())
    } else {
        let mut text = String::new();
        response.body_mut().read_to_string(&mut text).await?;
        Err(anyhow!(
            "error during streamGenerateContent via OpenCode, status code: {:?}, body: {}",
            response.status(),
            text
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_model_all_fields_present() {
        let json = serde_json::json!({
            "name": "Full Featured Model",
            "attachment": true,
            "tool_call": true,
            "reasoning": true,
            "reasoning_options": [
                { "type": "effort", "values": ["low", "medium", "high", "xhigh"] },
                { "type": "toggle" },
            ],
            "interleaved": { "field": "reasoning_content" },
            "limit": { "context": 500_000, "output": 128_000 },
            "provider": { "npm": "@ai-sdk/openai-compatible" },
            "cost": { "input": 5.0, "output": 25.0 },
            "modalities": { "input": ["text", "image"] }
        });

        let ModelParseResult::Success(model) =
            extract_model_config("full-featured-model", json, None)
        else {
            panic!("expected successful parse");
        };

        assert_eq!(model.id, "full-featured-model");
        assert_eq!(model.name, "Full Featured Model");
        assert_eq!(model.max_tokens, 500_000);
        assert_eq!(model.max_output_tokens, Some(128_000));
        assert_eq!(model.protocol, ApiProtocol::OpenAiChat);
        assert!(model.supports_images);
        assert!(model.supports_tools);
        assert_eq!(
            model.reasoning_effort_levels,
            Some(vec![
                ReasoningEffort::Low,
                ReasoningEffort::Medium,
                ReasoningEffort::High,
                ReasoningEffort::XHigh,
                ReasoningEffort::None,
            ])
        );
        assert!(model.interleaved_reasoning);
        assert_eq!(model.cost_input, Some(5.0));
        assert_eq!(model.cost_output, Some(25.0));
        assert!(!model.is_free());
        assert!(model.custom_api_url.is_none());
    }

    #[test]
    fn test_parse_model_anthropic_protocol() {
        let json = serde_json::json!({
            "name": "Claude Sonnet 4.6",
            "attachment": true,
            "tool_call": true,
            "reasoning": true,
            "interleaved": true,
            "limit": { "context": 1_000_000, "output": 64_000 },
            "provider": { "npm": "@ai-sdk/anthropic" },
            "cost": { "input": 3.0, "output": 15.0 }
        });

        let ModelParseResult::Success(model) =
            extract_model_config("claude-sonnet-4-6", json, None)
        else {
            panic!("expected successful parse");
        };

        assert_eq!(model.id, "claude-sonnet-4-6");
        assert_eq!(model.name, "Claude Sonnet 4.6");
        assert_eq!(model.protocol, ApiProtocol::Anthropic);
        assert_eq!(model.max_tokens, 1_000_000);
        assert_eq!(model.max_output_tokens, Some(64_000));
        assert!(!model.supports_images);
        assert!(model.supports_tools);
        assert!(model.interleaved_reasoning);
        assert_eq!(model.cost_input, Some(3.0));
        assert_eq!(model.cost_output, Some(15.0));
        assert!(!model.is_free());
    }

    #[test]
    fn test_parse_model_openai_responses_protocol() {
        let json = serde_json::json!({
            "name": "GPT-5 Nano",
            "attachment": true,
            "tool_call": true,
            "limit": { "context": 400_000, "input": 272_000, "output": 128_000 },
            "provider": { "npm": "@ai-sdk/openai" },
            "cost": { "input": 0.05, "output": 0.4 }
        });

        let ModelParseResult::Success(model) = extract_model_config("gpt-5-nano", json, None)
        else {
            panic!("expected successful parse");
        };

        assert_eq!(model.id, "gpt-5-nano");
        assert_eq!(model.name, "GPT-5 Nano");
        assert_eq!(model.protocol, ApiProtocol::OpenAiResponses);
        assert_eq!(model.max_tokens, 272_000);
        assert_eq!(model.max_output_tokens, Some(128_000));
        assert!(!model.supports_images);
        assert!(model.supports_tools);
        assert!(!model.interleaved_reasoning);
        assert_eq!(model.cost_input, Some(0.05));
        assert_eq!(model.cost_output, Some(0.4));
        assert!(!model.is_free());
        assert!(model.reasoning_effort_levels.is_none());
    }

    #[test]
    fn test_parse_model_google_protocol() {
        let json = serde_json::json!({
            "name": "Gemini 3 Flash",
            "attachment": true,
            "tool_call": true,
            "limit": { "context": 1_048_576, "output": 65_536 },
            "provider": { "npm": "@ai-sdk/google" },
            "cost": { "input": 1.5, "output": 9.0 }
        });

        let ModelParseResult::Success(model) = extract_model_config("gemini-3-flash", json, None)
        else {
            panic!("expected successful parse");
        };

        assert_eq!(model.id, "gemini-3-flash");
        assert_eq!(model.name, "Gemini 3 Flash");
        assert_eq!(model.protocol, ApiProtocol::Google);
        assert_eq!(model.max_tokens, 1_048_576);
        assert_eq!(model.max_output_tokens, Some(65_536));
        assert!(!model.supports_images);
        assert!(model.supports_tools);
        assert!(!model.interleaved_reasoning);
        assert_eq!(model.cost_input, Some(1.5));
        assert_eq!(model.cost_output, Some(9.0));
        assert!(!model.is_free());
    }

    #[test]
    fn test_parse_model_default_openai_chat_protocol() {
        let json = serde_json::json!({
            "name": "DeepSeek V4 Pro",
            "attachment": false,
            "tool_call": true,
            "interleaved": { "field": "reasoning_content" },
            "limit": { "context": 1_000_000, "output": 384_000 },
            "cost": { "input": 1.74, "output": 3.84 }
        });

        let ModelParseResult::Success(model) =
            extract_model_config("deepseek-v4-pro", json, Some("@ai-sdk/openai-compatible"))
        else {
            panic!("expected successful parse");
        };

        assert_eq!(model.id, "deepseek-v4-pro");
        assert_eq!(model.name, "DeepSeek V4 Pro");
        assert_eq!(model.protocol, ApiProtocol::OpenAiChat);
        assert_eq!(model.max_tokens, 1_000_000);
        assert_eq!(model.max_output_tokens, Some(384_000));
        assert!(!model.supports_images);
        assert!(model.supports_tools);
        assert!(model.interleaved_reasoning);
        assert_eq!(model.cost_input, Some(1.74));
        assert_eq!(model.cost_output, Some(3.84));
        assert!(!model.is_free());
        assert!(model.reasoning_effort_levels.is_none());
    }

    #[test]
    fn test_parse_model_zero_cost_is_free() {
        let json = serde_json::json!({
            "name": "Big Pickle",
            "attachment": false,
            "tool_call": true,
            "limit": { "context": 200_000, "input": 160_000, "output": 32_000 },
            "cost": { "input": 0, "output": 0 }
        });

        let ModelParseResult::Success(model) = extract_model_config("big-pickle", json, None)
        else {
            panic!("expected successful parse");
        };

        assert_eq!(model.id, "big-pickle");
        assert_eq!(model.name, "Big Pickle");
        assert_eq!(model.max_tokens, 160_000);
        assert_eq!(model.max_output_tokens, Some(32_000));
        assert_eq!(model.protocol, ApiProtocol::OpenAiChat);
        assert!(!model.supports_images);
        assert!(model.supports_tools);
        assert!(model.is_free());
        assert_eq!(model.cost_input, Some(0.0));
        assert_eq!(model.cost_output, Some(0.0));
    }

    #[test]
    fn test_parse_model_effort_levels() {
        let json = serde_json::json!({
            "name": "Claude Opus 4.8",
            "attachment": true,
            "tool_call": true,
            "reasoning_options": [
                { "type": "effort", "values": ["low", "medium", "high", "xhigh"] }
            ],
            "limit": { "context": 1_000_000, "output": 128_000 },
            "provider": { "npm": "@ai-sdk/anthropic" },
            "cost": { "input": 15.0, "output": 75.0 }
        });

        let ModelParseResult::Success(model) = extract_model_config("claude-opus-4-8", json, None)
        else {
            panic!("expected successful parse");
        };

        assert_eq!(model.id, "claude-opus-4-8");
        assert_eq!(model.name, "Claude Opus 4.8");
        assert_eq!(model.protocol, ApiProtocol::Anthropic);
        assert_eq!(model.max_tokens, 1_000_000);
        assert_eq!(model.max_output_tokens, Some(128_000));
        assert!(!model.supports_images);
        assert!(model.supports_tools);
        assert_eq!(
            model.reasoning_effort_levels,
            Some(vec![
                ReasoningEffort::Low,
                ReasoningEffort::Medium,
                ReasoningEffort::High,
                ReasoningEffort::XHigh,
            ])
        );
        assert_eq!(model.cost_input, Some(15.0));
        assert_eq!(model.cost_output, Some(75.0));
    }

    #[test]
    fn test_parse_model_toggle_adds_none_effort() {
        let json = serde_json::json!({
            "name": "MiniMax M3",
            "attachment": false,
            "tool_call": true,
            "reasoning": true,
            "reasoning_options": [
                { "type": "toggle" }
            ],
            "limit": { "context": 512_000, "output": 131_072 },
            "provider": { "npm": "@ai-sdk/openai-compatible" },
            "cost": { "input": 0.1, "output": 0.4 }
        });

        let ModelParseResult::Success(model) = extract_model_config("minimax-m3", json, None)
        else {
            panic!("expected successful parse");
        };

        assert_eq!(model.id, "minimax-m3");
        assert_eq!(model.name, "MiniMax M3");
        assert_eq!(model.protocol, ApiProtocol::OpenAiChat);
        assert_eq!(model.max_tokens, 512_000);
        assert_eq!(model.max_output_tokens, Some(131_072));
        assert!(!model.supports_images);
        assert!(model.supports_tools);
        assert_eq!(
            model.reasoning_effort_levels,
            Some(vec![ReasoningEffort::None])
        );
        assert_eq!(model.cost_input, Some(0.1));
        assert_eq!(model.cost_output, Some(0.4));
    }

    #[test]
    fn test_parse_model_reasoning_with_only_budget_tokens() {
        let json = serde_json::json!({
            "name": "Claude Sonnet 4",
            "attachment": true,
            "tool_call": true,
            "reasoning": true,
            "reasoning_options": [
                { "type": "budget_tokens" }
            ],
            "limit": { "context": 200_000, "output": 64_000 },
            "provider": { "npm": "@ai-sdk/anthropic" },
            "cost": { "input": 3.0, "output": 15.0 }
        });

        let ModelParseResult::Success(model) = extract_model_config("claude-sonnet-4", json, None)
        else {
            panic!("expected successful parse");
        };

        assert_eq!(model.id, "claude-sonnet-4");
        assert_eq!(model.name, "Claude Sonnet 4");
        assert_eq!(model.reasoning_effort_levels, Some(vec![]));
    }

    #[test]
    fn test_parse_model_toggle_and_efforts_combined() {
        let json = serde_json::json!({
            "name": "DeepSeek V4 Pro",
            "attachment": false,
            "tool_call": true,
            "reasoning": true,
            "reasoning_options": [
                { "type": "toggle" },
                { "type": "effort", "values": ["high", "max"] }
            ],
            "limit": { "context": 1_000_000, "output": 384_000 },
            "cost": { "input": 1.74, "output": 3.84 }
        });

        let ModelParseResult::Success(model) = extract_model_config("deepseek-v4-pro", json, None)
        else {
            panic!("expected successful parse");
        };

        assert_eq!(model.id, "deepseek-v4-pro");
        assert_eq!(model.name, "DeepSeek V4 Pro");
        assert_eq!(model.protocol, ApiProtocol::OpenAiChat);
        assert_eq!(model.max_tokens, 1_000_000);
        assert_eq!(model.max_output_tokens, Some(384_000));
        assert!(!model.supports_images);
        assert!(model.supports_tools);
        assert_eq!(
            model.reasoning_effort_levels,
            Some(vec![
                ReasoningEffort::High,
                ReasoningEffort::Max,
                ReasoningEffort::None,
            ])
        );
        assert_eq!(model.cost_input, Some(1.74));
        assert_eq!(model.cost_output, Some(3.84));
    }

    #[test]
    fn test_parse_model_reasoning_always_on_no_none_effort() {
        let json = serde_json::json!({
            "name": "GPT-5.4 Pro",
            "attachment": false,
            "tool_call": true,
            "reasoning": true,
            "reasoning_options": [],
            "limit": { "context": 400_000, "output": 128_000 },
            "provider": { "npm": "@ai-sdk/openai" },
            "cost": { "input": 2.5, "output": 15.0 }
        });

        let ModelParseResult::Success(model) = extract_model_config("gpt-5.4-pro", json, None)
        else {
            panic!("expected successful parse");
        };

        assert_eq!(model.id, "gpt-5.4-pro");
        assert_eq!(model.name, "GPT-5.4 Pro");
        assert_eq!(model.protocol, ApiProtocol::OpenAiResponses);
        assert_eq!(model.max_tokens, 400_000);
        assert_eq!(model.max_output_tokens, Some(128_000));
        assert!(!model.supports_images);
        assert!(model.supports_tools);
        let levels = model.reasoning_effort_levels.unwrap();
        assert!(
            !levels.contains(&ReasoningEffort::None),
            "models without toggle should not have None effort"
        );
        assert!(!model.interleaved_reasoning);
        assert_eq!(model.cost_input, Some(2.5));
        assert_eq!(model.cost_output, Some(15.0));
    }

    #[test]
    fn test_parse_model_interleaved_object_is_truthy() {
        let json = serde_json::json!({
            "name": "DeepSeek V4 Flash",
            "attachment": false,
            "tool_call": true,
            "interleaved": { "field": "reasoning_content" },
            "limit": { "context": 1_000_000, "output": 384_000 },
            "cost": { "input": 0.45, "output": 1.8 }
        });

        let ModelParseResult::Success(model) =
            extract_model_config("deepseek-v4-flash", json, None)
        else {
            panic!("expected successful parse");
        };

        assert_eq!(model.id, "deepseek-v4-flash");
        assert_eq!(model.name, "DeepSeek V4 Flash");
        assert_eq!(model.protocol, ApiProtocol::OpenAiChat);
        assert_eq!(model.max_tokens, 1_000_000);
        assert_eq!(model.max_output_tokens, Some(384_000));
        assert!(!model.supports_images);
        assert!(model.supports_tools);
        assert!(model.interleaved_reasoning);
        assert_eq!(model.cost_input, Some(0.45));
        assert_eq!(model.cost_output, Some(1.8));
    }

    #[test]
    fn test_parse_model_deprecated_status_skipped() {
        let json = serde_json::json!({
            "name": "Claude Haiku 3.5",
            "attachment": true,
            "tool_call": true,
            "limit": { "context": 200_000, "output": 8_192 },
            "status": "deprecated",
            "provider": { "npm": "@ai-sdk/anthropic" },
            "cost": { "input": 0.8, "output": 4.0 }
        });

        assert!(matches!(
            extract_model_config("claude-3-5-haiku", json, None),
            ModelParseResult::Deprecated
        ));
    }

    #[test]
    fn test_parse_model_failed_returns_failed() {
        let json = serde_json::json!({
            "attachment": true,
            "tool_call": true,
            "limit": { "context": 100_000 }
        });

        assert!(matches!(
            extract_model_config("broken-model", json, None),
            ModelParseResult::Failed
        ));
    }

    #[test]
    fn test_parse_model_missing_context_uses_default() {
        let json = serde_json::json!({
            "name": "No Limit Model",
            "attachment": false,
            "tool_call": true,
            "limit": { "output": 16_000 },
            "cost": { "input": 0.1, "output": 0.5 }
        });

        let ModelParseResult::Success(model) = extract_model_config("no-limit-model", json, None)
        else {
            panic!("expected successful parse");
        };

        assert_eq!(model.max_tokens, 200_000);
        assert_eq!(model.max_output_tokens, Some(16_000));
    }

    #[test]
    fn test_parse_model_modalities_drives_supports_images() {
        let json = serde_json::json!({
            "name": "Vision Model",
            "attachment": false,
            "tool_call": true,
            "limit": { "context": 100_000 },
            "modalities": { "input": ["text", "image", "audio"] },
            "cost": { "input": 1.0, "output": 5.0 }
        });

        let ModelParseResult::Success(model) = extract_model_config("vision-model", json, None)
        else {
            panic!("expected successful parse");
        };

        assert_eq!(model.id, "vision-model");
        assert!(model.supports_images);
    }

    #[test]
    fn test_parse_models_tier_assignment() {
        let json = serde_json::json!({
            "opencode": {
                "npm": "@ai-sdk/openai-compatible",
                "models": {
                    "deepseek-v4-pro": {
                        "name": "DeepSeek V4 Pro",
                        "attachment": false,
                        "tool_call": true,
                        "interleaved": { "field": "reasoning_content" },
                        "limit": { "context": 1_000_000, "output": 384_000 },
                        "cost": { "input": 1.74, "output": 3.84 }
                    },
                    "big-pickle": {
                        "name": "Big Pickle",
                        "attachment": false,
                        "tool_call": true,
                        "limit": { "context": 200_000, "input": 160_000, "output": 32_000 },
                        "cost": { "input": 0, "output": 0 }
                    },
                    "claude-3-5-haiku": {
                        "name": "Claude Haiku 3.5",
                        "attachment": true,
                        "tool_call": true,
                        "limit": { "context": 200_000, "output": 8_192 },
                        "status": "deprecated",
                        "provider": { "npm": "@ai-sdk/anthropic" },
                        "cost": { "input": 0.8, "output": 4.0 }
                    }
                }
            },
            "opencode-go": {
                "npm": "@ai-sdk/openai-compatible",
                "models": {
                    "minimax-m3": {
                        "name": "MiniMax M3",
                        "attachment": false,
                        "tool_call": true,
                        "limit": { "context": 512_000, "output": 131_072 },
                        "provider": { "npm": "@ai-sdk/anthropic" },
                        "cost": { "input": 0.1, "output": 0.4 }
                    }
                }
            }
        });

        let response: ModelsDevResponse = serde_json::from_value(json).unwrap();
        let models = parse_models_json(response);

        let zen_deepseek = models
            .iter()
            .find(|(id, _, sub)| id == "deepseek-v4-pro" && *sub == OpenCodeSubscription::Zen);
        assert!(zen_deepseek.is_some(), "deepseek-v4-pro should be Zen");

        let free_big_pickle = models
            .iter()
            .find(|(id, _, sub)| id == "big-pickle" && *sub == OpenCodeSubscription::Free);
        assert!(free_big_pickle.is_some(), "big-pickle should be Free");

        let go_minimax = models
            .iter()
            .find(|(id, _, sub)| id == "minimax-m3" && *sub == OpenCodeSubscription::Go);
        assert!(go_minimax.is_some(), "minimax-m3 should be Go");
        let (_, minimax_model, _) = go_minimax.unwrap();
        assert_eq!(minimax_model.protocol, ApiProtocol::Anthropic);

        let deprecated = models.iter().find(|(id, _, _)| id == "claude-3-5-haiku");
        assert!(deprecated.is_none(), "deprecated model should be skipped");

        assert_eq!(models.len(), 3);
    }

    #[test]
    fn test_parse_models_dual_tier_independent_parsing() {
        let json = serde_json::json!({
            "opencode": {
                "npm": "@ai-sdk/openai-compatible",
                "models": {
                    "minimax-m2.5": {
                        "name": "MiniMax M2.5",
                        "attachment": true,
                        "tool_call": true,
                        "limit": { "context": 1_000_000, "output": 64_000 },
                        "provider": { "npm": "@ai-sdk/anthropic" },
                        "cost": { "input": 0.5, "output": 2.0 }
                    },
                    "glm-5": {
                        "name": "GLM-5",
                        "attachment": false,
                        "tool_call": true,
                        "limit": { "context": 200_000, "output": 8_192 },
                        "cost": { "input": 1.0, "output": 4.0 }
                    }
                }
            },
            "opencode-go": {
                "npm": "@ai-sdk/openai",
                "models": {
                    "minimax-m2.5": {
                        "name": "MiniMax M2.5",
                        "attachment": false,
                        "tool_call": true,
                        "limit": { "context": 512_000, "output": 131_072 },
                        "provider": { "npm": "@ai-sdk/openai" },
                        "cost": { "input": 0.3, "output": 1.2 }
                    },
                    "glm-5": {
                        "name": "GLM-5",
                        "attachment": false,
                        "tool_call": true,
                        "limit": { "context": 200_000, "output": 8_192 },
                        "status": "deprecated",
                        "cost": { "input": 0.5, "output": 2.0 }
                    }
                }
            }
        });

        let response: ModelsDevResponse = serde_json::from_value(json).unwrap();
        let models = parse_models_json(response);

        let zen_minimax = models
            .iter()
            .find(|(id, _, sub)| id == "minimax-m2.5" && *sub == OpenCodeSubscription::Zen);
        assert!(
            zen_minimax.is_some(),
            "minimax-m2.5 should have a Zen entry"
        );
        let (_, zen_model, _) = zen_minimax.unwrap();
        assert_eq!(zen_model.name, "MiniMax M2.5");
        assert_eq!(zen_model.protocol, ApiProtocol::Anthropic);
        assert!(!zen_model.supports_images);
        assert_eq!(zen_model.max_tokens, 1_000_000);
        assert_eq!(zen_model.cost_input, Some(0.5));

        let go_minimax = models
            .iter()
            .find(|(id, _, sub)| id == "minimax-m2.5" && *sub == OpenCodeSubscription::Go);
        assert!(go_minimax.is_some(), "minimax-m2.5 should have a Go entry");
        let (_, go_model, _) = go_minimax.unwrap();
        assert_eq!(go_model.name, "MiniMax M2.5");
        assert_eq!(go_model.protocol, ApiProtocol::OpenAiResponses);
        assert!(!go_model.supports_images);
        assert_eq!(go_model.max_tokens, 512_000);
        assert_eq!(go_model.cost_input, Some(0.3));

        let zen_glm = models
            .iter()
            .find(|(id, _, sub)| id == "glm-5" && *sub == OpenCodeSubscription::Zen);
        assert!(zen_glm.is_some(), "glm-5 should have a Zen entry");
        let (_, zen_glm_model, _) = zen_glm.unwrap();
        assert_eq!(zen_glm_model.name, "GLM-5");

        let go_glm = models
            .iter()
            .find(|(id, _, sub)| id == "glm-5" && *sub == OpenCodeSubscription::Go);
        assert!(
            go_glm.is_none(),
            "glm-5 should NOT have a Go entry (deprecated in Go)"
        );

        assert_eq!(models.len(), 3);
    }

    #[test]
    fn test_parse_models_failed_model_added_as_disabled() {
        let json = serde_json::json!({
            "opencode": {
                "npm": "@ai-sdk/openai-compatible",
                "models": {
                    "broken-model": {
                        "attachment": true,
                        "tool_call": true,
                        "limit": { "context": 100_000 }
                    }
                }
            },
            "opencode-go": {
                "npm": "@ai-sdk/openai-compatible",
                "models": {
                    "broken-go-model": {
                        "attachment": true,
                        "tool_call": true,
                        "limit": { "context": 200_000 }
                    }
                }
            }
        });

        let response: ModelsDevResponse = serde_json::from_value(json).unwrap();
        let models = parse_models_json(response);

        assert_eq!(
            models.len(),
            2,
            "both failed models should be included as disabled"
        );

        let zen_broken = models
            .iter()
            .find(|(id, _, sub)| id == "broken-model" && *sub == OpenCodeSubscription::Zen);
        let (_, zen_model, zen_sub) = zen_broken.expect("broken-model should be in output as Zen");
        assert_eq!(*zen_sub, OpenCodeSubscription::Zen);
        assert_eq!(
            zen_model.name, "broken-model",
            "name should fall back to model ID when 'name' field is missing"
        );
        assert_eq!(
            zen_model.disabled,
            Some("Failed parsing models.dev data".to_string())
        );

        let go_broken = models
            .iter()
            .find(|(id, _, sub)| id == "broken-go-model" && *sub == OpenCodeSubscription::Go);
        let (_, go_model, go_sub) = go_broken.expect("broken-go-model should be in output as Go");
        assert_eq!(*go_sub, OpenCodeSubscription::Go);
        assert_eq!(go_model.name, "broken-go-model");
        assert_eq!(
            go_model.disabled,
            Some("Failed parsing models.dev data".to_string())
        );
    }

    #[test]
    fn test_parse_model_input_limit_precedence() {
        let json = serde_json::json!({
            "name": "Tighter Input Model",
            "attachment": false,
            "tool_call": true,
            "limit": { "context": 400_000, "input": 272_000, "output": 128_000 },
            "provider": { "npm": "@ai-sdk/openai" },
            "cost": { "input": 1.0, "output": 8.0 }
        });

        let ModelParseResult::Success(model) =
            extract_model_config("tighter-input-model", json, None)
        else {
            panic!("expected successful parse");
        };

        assert_eq!(model.max_tokens, 272_000);
        assert_eq!(model.max_output_tokens, Some(128_000));
    }

    #[test]
    fn test_parse_model_no_input_falls_back_to_context() {
        let json = serde_json::json!({
            "name": "Context Only Model",
            "attachment": false,
            "tool_call": true,
            "limit": { "context": 500_000, "output": 64_000 },
            "cost": { "input": 0.5, "output": 2.0 }
        });

        let ModelParseResult::Success(model) =
            extract_model_config("context-only-model", json, None)
        else {
            panic!("expected successful parse");
        };

        assert_eq!(model.max_tokens, 500_000);
        assert_eq!(model.max_output_tokens, Some(64_000));
    }
}
