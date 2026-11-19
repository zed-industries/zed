//! Reads what Bedrock's listing APIs do not report, such as token limits,
//! tool use and reasoning controls, from the public models.dev catalog. The
//! catalog also names the models to offer when Bedrock itself cannot be asked
//! which ones exist.

use crate::{BedrockAdaptiveThinkingEffort, ConverseModel, ConverseThinking};
use anyhow::{Context as _, Result};
use serde::Deserialize;
use std::collections::BTreeMap;

pub const CAPABILITY_CATALOG_URL: &str = "https://models.dev/api.json";
/// The catalog's key for the models that Bedrock serves.
pub const CAPABILITY_CATALOG_PROVIDER: &str = "amazon-bedrock";

const DEFAULT_CONTEXT_WINDOW: u64 = 128_000;

const GLOBAL_GEOGRAPHY: &str = "global";
/// The geographies of cross-Region inference profiles, which prefix the ids
/// of those profiles.
const GEOGRAPHIES: &[&str] = &[
    "us",
    "us-gov",
    "ca",
    "eu",
    "au",
    "jp",
    "in",
    "apac",
    GLOBAL_GEOGRAPHY,
];

/// Models that reject tools on `ConverseStream`, which is the only Converse
/// operation Zed uses. The catalog records tool use for the model as a whole.
///
/// <https://docs.aws.amazon.com/bedrock/latest/userguide/tool-use-client-side.html>
const STREAMING_TOOL_USE_UNSUPPORTED: &[&str] = &[
    "meta.llama",
    "mistral.pixtral-large",
    "writer.palmyra",
    "deepseek.r1",
];

const ANTHROPIC_MODEL_ID_PREFIX: &str = "anthropic.";
/// Claude models other than Claude 3 take cache points, which the catalog
/// does not record.
const CLAUDE_MODEL_ID_PREFIX: &str = "anthropic.claude";
const CLAUDE_3_MODEL_ID_PREFIX: &str = "anthropic.claude-3";
/// The vendors whose models take an effort through `reasoning.effort`.
const REASONING_EFFORT_MODEL_ID_PREFIXES: &[&str] = &["openai.", "xai."];

/// The geographies whose inference profiles serve `region`, nearest first.
fn geographies(region: &str) -> &'static [&'static str] {
    match region {
        "ap-southeast-2" | "ap-southeast-4" => &["au", "apac"],
        "ap-northeast-1" | "ap-northeast-3" => &["jp", "apac"],
        "ap-south-1" | "ap-south-2" => &["in", "apac"],
        _ if region.starts_with("us-gov-") => &["us-gov"],
        _ if region.starts_with("us-") || region.starts_with("sa-") => &["us"],
        _ if region.starts_with("ca-") => &["ca", "us"],
        _ if region.starts_with("eu-") => &["eu"],
        _ if region.starts_with("ap-") || region.starts_with("me-") => &["apac"],
        _ => &[],
    }
}

/// Splits an inference profile id into its geography and the id of the
/// foundation model behind it.
fn split_profile_id(profile_id: &str) -> Option<(&str, &str)> {
    profile_id
        .split_once('.')
        .filter(|(geography, _)| GEOGRAPHIES.contains(geography))
}

/// The model or profile id in `request_id`, which can also be an ARN.
fn profile_id(request_id: &str) -> &str {
    request_id
        .rsplit_once('/')
        .map_or(request_id, |(_, profile_id)| profile_id)
}

/// The id of the foundation model that `request_id` names, where
/// `request_id` is a model id or the id or ARN of a cross-Region inference
/// profile. Any other id, such as that of an application inference profile,
/// is returned as it is.
pub fn base_model_id(request_id: &str) -> &str {
    let profile_id = profile_id(request_id);
    split_profile_id(profile_id).map_or(profile_id, |(_, base_model_id)| base_model_id)
}

fn has_prefix_in(model_id: &str, prefixes: &[&str]) -> bool {
    prefixes.iter().any(|prefix| model_id.starts_with(prefix))
}

fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
struct ModelCapabilities {
    name: String,
    #[serde(default)]
    reasoning: bool,
    #[serde(default)]
    reasoning_options: Option<Vec<ReasoningOption>>,
    #[serde(default)]
    tool_call: bool,
    #[serde(default = "default_true")]
    temperature: bool,
    #[serde(default)]
    modalities: Modalities,
    limit: Limits,
    #[serde(default)]
    status: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
struct ReasoningOption {
    #[serde(rename = "type")]
    kind: String,
    #[serde(default)]
    values: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
struct Modalities {
    #[serde(default)]
    input: Vec<String>,
    #[serde(default)]
    output: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
struct Limits {
    context: u64,
    #[serde(default)]
    output: Option<u64>,
}

impl ModelCapabilities {
    fn reasoning_options(&self) -> &[ReasoningOption] {
        self.reasoning_options.as_deref().unwrap_or_default()
    }

    fn has_reasoning_option(&self, kind: &str) -> bool {
        self.reasoning_options()
            .iter()
            .any(|option| option.kind == kind)
    }

    /// The efforts the model can be asked for, with `default_effort` as the
    /// default when the model offers it.
    fn efforts(
        &self,
        default_effort: BedrockAdaptiveThinkingEffort,
    ) -> Option<(
        Vec<BedrockAdaptiveThinkingEffort>,
        BedrockAdaptiveThinkingEffort,
    )> {
        let efforts = self
            .reasoning_options()
            .iter()
            .filter(|option| option.kind == "effort")
            .flat_map(|option| option.values.as_deref().unwrap_or_default())
            .filter_map(|effort| BedrockAdaptiveThinkingEffort::from_value(effort))
            .collect::<Vec<_>>();
        let lowest_effort = *efforts.first()?;
        let default_effort = if efforts.contains(&default_effort) {
            default_effort
        } else {
            lowest_effort
        };
        Some((efforts, default_effort))
    }

    /// How Zed drives the reasoning of `base_model_id`. The catalog says
    /// which controls a model has. The request fields that carry them depend
    /// on the vendor, and Zed sends only the two shapes it knows.
    fn thinking(&self, base_model_id: &str) -> Option<ConverseThinking> {
        if !self.reasoning {
            return None;
        }
        if base_model_id.starts_with(ANTHROPIC_MODEL_ID_PREFIX) {
            if let Some((efforts, default_effort)) =
                self.efforts(BedrockAdaptiveThinkingEffort::High)
            {
                return Some(ConverseThinking::Adaptive {
                    efforts,
                    default_effort,
                    always_on: !self.has_reasoning_option("toggle"),
                });
            }
            return self
                .has_reasoning_option("budget_tokens")
                .then(|| ConverseThinking::budget(None));
        }
        if has_prefix_in(base_model_id, REASONING_EFFORT_MODEL_ID_PREFIXES) {
            let (efforts, default_effort) = self.efforts(BedrockAdaptiveThinkingEffort::Medium)?;
            return Some(ConverseThinking::Reasoning {
                efforts,
                default_effort,
            });
        }
        None
    }

    fn can_converse(&self) -> bool {
        self.status.as_deref() != Some("deprecated")
            && self.modalities.output.iter().any(|output| output == "text")
    }
}

/// What the models.dev catalog says about the models Bedrock serves, keyed by
/// the model id or inference profile id that invokes each of them.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CapabilityCatalog {
    models: BTreeMap<String, ModelCapabilities>,
}

impl CapabilityCatalog {
    /// Parses the catalog, or a part of it that holds the Bedrock provider.
    /// An entry that cannot be read is skipped, so that one malformed model
    /// does not hide the others.
    pub fn parse(catalog: &str) -> Result<Self> {
        let catalog = serde_json::from_str::<serde_json::Value>(catalog)
            .context("parsing the model capability catalog")?;
        let models = catalog
            .get(CAPABILITY_CATALOG_PROVIDER)
            .and_then(|provider| provider.get("models"))
            .and_then(serde_json::Value::as_object)
            .with_context(|| {
                format!("the model capability catalog has no {CAPABILITY_CATALOG_PROVIDER} models")
            })?;
        let models = models
            .iter()
            .filter_map(|(model_id, capabilities)| {
                match ModelCapabilities::deserialize(capabilities) {
                    Ok(capabilities) => Some((model_id.clone(), capabilities)),
                    Err(error) => {
                        log::warn!("Skipping Bedrock model {model_id} in the catalog: {error}");
                        None
                    }
                }
            })
            .collect();
        Ok(Self { models })
    }

    pub fn is_empty(&self) -> bool {
        self.models.is_empty()
    }

    /// Describes the foundation model `base_model_id` from the first of
    /// `request_ids`, or else the model id itself, that the catalog knows. A
    /// model the catalog does not know gets no optional capability beyond
    /// tool use.
    pub fn describe<'a>(
        &self,
        base_model_id: &'a str,
        request_ids: impl IntoIterator<Item = &'a str>,
    ) -> ConverseModel {
        let capabilities = request_ids
            .into_iter()
            .chain([base_model_id])
            .find_map(|request_id| self.models.get(request_id));
        let thinking = capabilities.and_then(|capabilities| capabilities.thinking(base_model_id));
        let max_tokens = capabilities
            .map(|capabilities| capabilities.limit.context)
            .filter(|context_window| *context_window > 0)
            .unwrap_or(DEFAULT_CONTEXT_WINDOW);
        ConverseModel {
            id: base_model_id.to_string(),
            request_id: base_model_id.to_string(),
            base_model_id: base_model_id.to_string(),
            display_name: capabilities
                .map_or(base_model_id, |capabilities| capabilities.name.as_str())
                .to_string(),
            max_tokens,
            // The catalog gives some models their context window as the
            // output limit, which Bedrock would reject.
            max_output_tokens: capabilities
                .and_then(|capabilities| capabilities.limit.output)
                .filter(|max_output_tokens| *max_output_tokens < max_tokens),
            default_temperature: None,
            supports_temperature: capabilities.is_none_or(|capabilities| capabilities.temperature)
                && !thinking
                    .as_ref()
                    .is_some_and(ConverseThinking::is_always_on),
            supports_tool_use: capabilities.is_none_or(|capabilities| capabilities.tool_call)
                && !has_prefix_in(base_model_id, STREAMING_TOOL_USE_UNSUPPORTED),
            supports_images: capabilities.is_some_and(|capabilities| {
                capabilities
                    .modalities
                    .input
                    .iter()
                    .any(|input| input == "image")
            }),
            supports_caching: base_model_id.starts_with(CLAUDE_MODEL_ID_PREFIX)
                && !base_model_id.starts_with(CLAUDE_3_MODEL_ID_PREFIX),
            thinking,
        }
    }

    /// Describes the model that `request_id` invokes, where `request_id` is a
    /// foundation model id or the id or ARN of an inference profile, if the
    /// catalog knows it.
    pub fn model_for_request_id(&self, request_id: &str) -> Option<ConverseModel> {
        let profile_id = profile_id(request_id);
        let base_model_id = base_model_id(request_id);
        [profile_id, base_model_id]
            .iter()
            .any(|id| self.models.contains_key(*id))
            .then(|| ConverseModel {
                id: request_id.to_string(),
                request_id: request_id.to_string(),
                ..self.describe(base_model_id, [profile_id])
            })
    }

    /// The models that the catalog says have an inference profile serving
    /// `region`. This stands in for Bedrock's own listing when the
    /// credentials in use may not ask for it. It can name a profile that the
    /// Region does not offer, and such a request fails when it is sent.
    pub fn models_for_region(&self, region: &str, allow_global: bool) -> Vec<ConverseModel> {
        let mut profile_ids = BTreeMap::<&str, BTreeMap<&str, &str>>::new();
        for (profile_id, capabilities) in &self.models {
            if !capabilities.can_converse() {
                continue;
            }
            if let Some((geography, base_model_id)) = split_profile_id(profile_id) {
                profile_ids
                    .entry(base_model_id)
                    .or_default()
                    .insert(geography, profile_id);
            }
        }

        profile_ids
            .into_iter()
            .filter_map(|(base_model_id, profile_ids)| {
                let geographic = geographies(region)
                    .iter()
                    .find_map(|geography| profile_ids.get(geography));
                let global = profile_ids.get(GLOBAL_GEOGRAPHY);
                let profile_id = if allow_global {
                    global.or(geographic)
                } else {
                    geographic.or(global)
                }?;
                Some(ConverseModel {
                    request_id: profile_id.to_string(),
                    ..self.describe(base_model_id, [*profile_id])
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use BedrockAdaptiveThinkingEffort::{High, Low, Medium, XHigh};
    use serde_json::json;

    fn catalog(models: serde_json::Value) -> Result<CapabilityCatalog> {
        CapabilityCatalog::parse(&json!({ "amazon-bedrock": { "models": models } }).to_string())
    }

    fn entry(name: &str, members: serde_json::Value) -> serde_json::Value {
        let mut entry = json!({
            "name": name,
            "tool_call": true,
            "modalities": {"input": ["text"], "output": ["text"]},
            "limit": {"context": 200_000, "output": 8_000},
        });
        if let (Some(entry), Some(members)) = (entry.as_object_mut(), members.as_object()) {
            entry.extend(members.clone());
        }
        entry
    }

    fn reasoning(options: serde_json::Value) -> serde_json::Value {
        json!({"reasoning": true, "reasoning_options": options})
    }

    #[test]
    fn describes_a_model_the_catalog_does_not_know_conservatively() {
        let model = CapabilityCatalog::default().describe("vendor.chat", ["us.vendor.chat"]);

        assert_eq!(model.id, "vendor.chat");
        assert_eq!(model.display_name, "vendor.chat");
        assert_eq!(model.max_tokens, DEFAULT_CONTEXT_WINDOW);
        assert_eq!(model.max_output_tokens, None);
        assert_eq!(model.thinking, None);
        assert!(model.supports_tool_use && model.supports_temperature);
        assert!(!model.supports_images && !model.supports_caching);
    }

    #[test]
    fn reads_limits_and_modalities_from_the_first_known_request_id() -> Result<()> {
        let catalog = catalog(json!({
            "global.vendor.chat": entry("Chat (Global)", json!({
                "limit": {"context": 500_000, "output": 64_000},
                "modalities": {"input": ["text", "image"], "output": ["text"]},
                "temperature": false,
                "tool_call": false,
            })),
            "vendor.chat": entry("Chat", json!({})),
        }))?;

        let by_profile = catalog.describe("vendor.chat", ["us.vendor.chat", "global.vendor.chat"]);
        assert_eq!(by_profile.display_name, "Chat (Global)");
        assert_eq!(by_profile.max_tokens, 500_000);
        assert_eq!(by_profile.max_output_tokens, Some(64_000));
        assert!(by_profile.supports_images);
        assert!(!by_profile.supports_temperature && !by_profile.supports_tool_use);

        let by_model_id = catalog.describe("vendor.chat", ["eu.vendor.chat"]);
        assert_eq!(by_model_id.display_name, "Chat");
        assert_eq!(by_model_id.max_output_tokens, Some(8_000));
        Ok(())
    }

    #[test]
    fn ignores_an_output_limit_that_is_not_below_the_context_window() -> Result<()> {
        let catalog = catalog(json!({
            "vendor.chat": entry("Chat", json!({"limit": {"context": 100_000, "output": 100_000}})),
        }))?;

        assert_eq!(catalog.describe("vendor.chat", []).max_output_tokens, None);
        Ok(())
    }

    #[test]
    fn skips_an_entry_it_cannot_read() -> Result<()> {
        let catalog = catalog(json!({
            "vendor.broken": {"name": "No limits"},
            "vendor.chat": entry("Chat", json!({})),
        }))?;

        assert_eq!(
            catalog.describe("vendor.broken", []).display_name,
            "vendor.broken"
        );
        assert_eq!(catalog.describe("vendor.chat", []).display_name, "Chat");
        Ok(())
    }

    #[test]
    fn requires_the_bedrock_provider() {
        assert!(CapabilityCatalog::parse(r#"{"another-provider": {"models": {}}}"#).is_err());
        assert!(CapabilityCatalog::parse("not json").is_err());
    }

    #[test]
    fn maps_reasoning_controls_to_the_request_shape_of_the_vendor() -> Result<()> {
        let efforts = json!({"type": "effort", "values": ["none", "low", "high", "xhigh"]});
        let toggle = json!({"type": "toggle"});
        let catalog = catalog(json!({
            "anthropic.always": entry("", reasoning(json!([efforts]))),
            "anthropic.optional": entry("", reasoning(json!([toggle, efforts]))),
            "anthropic.budget": entry("", reasoning(json!([toggle, {"type": "budget_tokens"}]))),
            "anthropic.uncontrolled": entry("", reasoning(json!([]))),
            "anthropic.plain": entry("", json!({})),
            "openai.effort": entry("", reasoning(json!([efforts]))),
            "vendor.effort": entry("", reasoning(json!([efforts]))),
        }))?;
        let thinking = |model_id| catalog.describe(model_id, []).thinking;

        assert_eq!(
            thinking("anthropic.always"),
            Some(ConverseThinking::Adaptive {
                efforts: vec![Low, High, XHigh],
                default_effort: High,
                always_on: true,
            })
        );
        assert_eq!(
            thinking("anthropic.optional"),
            Some(ConverseThinking::Adaptive {
                efforts: vec![Low, High, XHigh],
                default_effort: High,
                always_on: false,
            })
        );
        assert_eq!(
            thinking("anthropic.budget"),
            Some(ConverseThinking::budget(None))
        );
        assert_eq!(thinking("anthropic.uncontrolled"), None);
        assert_eq!(thinking("anthropic.plain"), None);
        // `medium` is not offered, so the lowest effort is the default.
        assert_eq!(
            thinking("openai.effort"),
            Some(ConverseThinking::Reasoning {
                efforts: vec![Low, High, XHigh],
                default_effort: Low,
            })
        );
        assert_eq!(thinking("vendor.effort"), None);

        assert!(
            !catalog
                .describe("anthropic.always", [])
                .supports_temperature
        );
        assert!(
            catalog
                .describe("anthropic.optional", [])
                .supports_temperature
        );
        assert!(!catalog.describe("openai.effort", []).supports_temperature);
        Ok(())
    }

    #[test]
    fn prefers_the_default_effort_when_the_model_offers_it() -> Result<()> {
        let catalog = catalog(json!({
            "openai.effort": entry("", reasoning(json!([
                {"type": "effort", "values": ["low", "medium", "high"]}
            ]))),
        }))?;

        assert_eq!(
            catalog.describe("openai.effort", []).thinking,
            Some(ConverseThinking::Reasoning {
                efforts: vec![Low, Medium, High],
                default_effort: Medium,
            })
        );
        Ok(())
    }

    #[test]
    fn describes_a_request_id_by_the_model_behind_it() -> Result<()> {
        let catalog = catalog(json!({
            "eu.vendor.chat": entry("Chat (EU)", json!({})),
            "vendor.bare": entry("Bare", json!({})),
        }))?;

        for request_id in [
            "eu.vendor.chat",
            "arn:aws:bedrock:eu-west-1:123456789012:inference-profile/eu.vendor.chat",
        ] {
            let model = catalog
                .model_for_request_id(request_id)
                .with_context(|| format!("{request_id} is not in the catalog"))?;
            assert_eq!(model.id, request_id);
            assert_eq!(model.request_id, request_id);
            assert_eq!(model.base_model_id, "vendor.chat");
            assert_eq!(model.display_name, "Chat (EU)");
        }

        // A profile the catalog lacks is described by the model behind it.
        let other_profile = catalog
            .model_for_request_id("us.vendor.bare")
            .context("the model behind us.vendor.bare is in the catalog")?;
        assert_eq!(other_profile.request_id, "us.vendor.bare");
        assert_eq!(other_profile.display_name, "Bare");

        assert_eq!(catalog.model_for_request_id("vendor.other-1.5"), None);
        assert_eq!(base_model_id("vendor.other-1.5"), "vendor.other-1.5");
        assert_eq!(
            base_model_id(
                "arn:aws:bedrock:us-east-1:123456789012:application-inference-profile/abc123"
            ),
            "abc123"
        );
        Ok(())
    }

    #[test]
    fn offers_the_profiles_that_serve_a_region() -> Result<()> {
        let catalog = catalog(json!({
            "us.vendor.everywhere": entry("Everywhere (US)", json!({})),
            "ca.vendor.everywhere": entry("Everywhere (CA)", json!({})),
            "global.vendor.everywhere": entry("Everywhere (Global)", json!({})),
            "us.vendor.american": entry("American (US)", json!({})),
            "global.vendor.global-only": entry("Global only", json!({})),
            "eu.vendor.european": entry("European (EU)", json!({})),
            "vendor.no-profile": entry("No profile", json!({})),
            "us.vendor.retired": entry("Retired (US)", json!({"status": "deprecated"})),
            "us.vendor.embedding": entry("Embedding (US)", json!({
                "modalities": {"input": ["text"], "output": ["embedding"]},
            })),
        }))?;
        let offered = |region, allow_global| {
            catalog
                .models_for_region(region, allow_global)
                .into_iter()
                .map(|model| (model.id, model.request_id))
                .collect::<Vec<_>>()
        };
        let pair = |id: &str, request_id: &str| (id.to_string(), request_id.to_string());

        assert_eq!(
            offered("ca-central-1", false),
            [
                pair("vendor.american", "us.vendor.american"),
                pair("vendor.everywhere", "ca.vendor.everywhere"),
                pair("vendor.global-only", "global.vendor.global-only"),
            ]
        );
        assert_eq!(
            offered("us-east-1", true),
            [
                pair("vendor.american", "us.vendor.american"),
                pair("vendor.everywhere", "global.vendor.everywhere"),
                pair("vendor.global-only", "global.vendor.global-only"),
            ]
        );
        // A Region outside every geography reaches only global profiles.
        assert_eq!(
            offered("af-south-1", false),
            [
                pair("vendor.everywhere", "global.vendor.everywhere"),
                pair("vendor.global-only", "global.vendor.global-only"),
            ]
        );
        Ok(())
    }
}
