//! Model discovery shared by OpenCode clients.
//!
//! Applications own fetching and caching; this module applies the catalog's
//! subscription, capability, and transport rules consistently.

use anyhow::{Result, anyhow, bail};
use serde::Deserialize;
use std::collections::HashMap;

use crate::{ApiProtocol, Model, OpenCodeSubscription};
use language_model_core::ReasoningEffort;

pub const MODEL_CATALOG_URL: &str = "https://models.opencode.ai/api.json";

#[derive(Clone)]
pub struct DiscoveredModel {
    pub model: Model,
    pub supports_images: bool,
    pub supports_thinking: bool,
}

/// Parses compatible models, optionally restricted to a live endpoint's model IDs.
///
/// Deprecated models and zero-cost Zen models are excluded. An intentionally
/// filtered empty catalog is valid; unusable metadata is an error.
pub fn parse_model_catalog(
    catalog: &str,
    subscription: OpenCodeSubscription,
    available_models: Option<&[String]>,
) -> Result<Vec<DiscoveredModel>> {
    let provider_key = match subscription {
        OpenCodeSubscription::Zen => "opencode",
        OpenCodeSubscription::Go => "opencode-go",
    };
    let catalog = serde_json::from_str::<serde_json::Value>(catalog)?;
    let provider = catalog
        .get(provider_key)
        .ok_or_else(|| anyhow!("OpenCode catalog is missing {provider_key}"))?;
    let provider = serde_json::from_value::<CatalogProvider>(provider.clone())?;
    let model_ids = available_models.map_or_else(
        || {
            let mut ids = provider.models.keys().cloned().collect::<Vec<_>>();
            ids.sort();
            ids
        },
        |models| models.to_vec(),
    );
    let mut compatible_candidate_count = model_ids.len();
    let models = model_ids
        .into_iter()
        .filter_map(|model_id| {
            let metadata = provider.models.get(&model_id)?;
            let metadata = match CatalogModel::deserialize(metadata) {
                Ok(metadata) => metadata,
                Err(error) => {
                    log::warn!("Invalid OpenCode metadata for {model_id}: {error}");
                    return None;
                }
            };
            if metadata.status.as_deref() == Some("deprecated")
                || (subscription == OpenCodeSubscription::Zen
                    && metadata
                        .cost
                        .as_ref()
                        .is_some_and(|cost| cost.input == Some(0.0) && cost.output == Some(0.0)))
            {
                compatible_candidate_count -= 1;
                return None;
            }
            if metadata.id != model_id
                || !metadata.tool_call
                || !metadata
                    .modalities
                    .output
                    .iter()
                    .any(|value| value == "text")
                || metadata.limit.context == 0
            {
                return None;
            }
            let transport = metadata
                .provider
                .as_ref()
                .and_then(|provider| provider.npm.as_deref())
                .or(provider.npm.as_deref());
            let protocol = match transport? {
                "@ai-sdk/anthropic" => ApiProtocol::Anthropic,
                "@ai-sdk/openai" => ApiProtocol::OpenAiResponses,
                "@ai-sdk/openai-compatible" => ApiProtocol::OpenAiChat,
                "@ai-sdk/google" => ApiProtocol::Google,
                _ => return None,
            };
            let reasoning_effort_levels = metadata
                .reasoning
                .then(|| {
                    metadata
                        .reasoning_options
                        .as_deref()
                        .unwrap_or_default()
                        .iter()
                        .find(|option| option.kind == "effort")
                        .into_iter()
                        .flat_map(|option| &option.values)
                        .filter_map(|effort| normalize_reasoning_effort(effort))
                        .collect::<Vec<_>>()
                })
                .filter(|levels| !levels.is_empty());
            Some(DiscoveredModel {
                model: Model::new(
                    metadata.id,
                    Some(metadata.name),
                    metadata.limit.context,
                    metadata.limit.output,
                    protocol,
                    reasoning_effort_levels,
                    None,
                    metadata.interleaved == serde_json::Value::Bool(true)
                        || metadata.interleaved.is_object(),
                ),
                supports_images: metadata
                    .modalities
                    .input
                    .iter()
                    .any(|value| value == "image"),
                supports_thinking: metadata.reasoning,
            })
        })
        .collect::<Vec<_>>();
    if compatible_candidate_count > 0 && models.is_empty() {
        bail!("OpenCode model metadata did not contain any compatible models");
    }
    Ok(models)
}

pub fn normalize_reasoning_effort(effort: &str) -> Option<ReasoningEffort> {
    match effort.trim().to_ascii_lowercase().as_str() {
        "none" => Some(ReasoningEffort::None),
        "minimal" => Some(ReasoningEffort::Minimal),
        "low" => Some(ReasoningEffort::Low),
        "medium" => Some(ReasoningEffort::Medium),
        "high" => Some(ReasoningEffort::High),
        "xhigh" => Some(ReasoningEffort::XHigh),
        "max" => Some(ReasoningEffort::Max),
        _ => None,
    }
}

#[derive(Deserialize)]
struct CatalogProvider {
    npm: Option<String>,
    models: HashMap<String, serde_json::Value>,
}

#[derive(Deserialize)]
struct CatalogModel {
    id: String,
    name: String,
    #[serde(default)]
    reasoning: bool,
    reasoning_options: Option<Vec<ReasoningOption>>,
    #[serde(default)]
    tool_call: bool,
    #[serde(default)]
    interleaved: serde_json::Value,
    #[serde(default)]
    modalities: ModelModalities,
    limit: ModelLimits,
    provider: Option<ModelTransport>,
    cost: Option<ModelCost>,
    status: Option<String>,
}

#[derive(Deserialize)]
struct ModelCost {
    input: Option<f64>,
    output: Option<f64>,
}

#[derive(Default, Deserialize)]
struct ModelModalities {
    #[serde(default)]
    input: Vec<String>,
    #[serde(default)]
    output: Vec<String>,
}

#[derive(Deserialize)]
struct ModelLimits {
    context: u64,
    output: Option<u64>,
}

#[derive(Deserialize)]
struct ModelTransport {
    npm: Option<String>,
}

#[derive(Deserialize)]
struct ReasoningOption {
    #[serde(rename = "type")]
    kind: String,
    #[serde(default)]
    values: Vec<String>,
}
