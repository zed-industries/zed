//! Reads what Bedrock's listing APIs do not report, such as token limits,
//! tool use and reasoning controls, from the public models.dev catalog. The
//! catalog also names the models to offer when Bedrock itself cannot be asked
//! which ones exist.

use crate::{BedrockAdaptiveThinkingEffort, ConverseModel, ConverseThinking};
use anyhow::{Context as _, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::num::NonZeroU64;

pub const CAPABILITY_CATALOG_URL: &str = "https://models.dev/api.json";

const DEFAULT_CONTEXT_WINDOW: u64 = 128_000;

pub(crate) const GLOBAL_GEOGRAPHY: &str = "global";
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
/// The catalog lists the same effort and budget options for models that accept
/// adaptive thinking and for these models, which require budget thinking.
const BUDGET_THINKING_ONLY: &[&str] = &["anthropic.claude-opus-4-5"];
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
pub(crate) fn split_profile_id(profile_id: &str) -> Option<(&str, &str)> {
    profile_id
        .split_once('.')
        .filter(|(geography, _)| GEOGRAPHIES.contains(geography))
}

/// The model or profile id in `request_id`, which can also be an ARN.
fn profile_id(request_id: &str) -> &str {
    if request_id.starts_with("arn:")
        && let Some((resource, id)) = request_id.split_once('/')
        && (resource.ends_with(":inference-profile") || resource.ends_with(":foundation-model"))
    {
        id
    } else {
        request_id
    }
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
    reasoning_options: Option<Vec<ReasoningOption>>,
    #[serde(default)]
    tool_call: bool,
    #[serde(default = "default_true")]
    temperature: bool,
    #[serde(default)]
    modalities: Modalities,
    limit: Limits,
    status: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
struct ReasoningOption {
    #[serde(rename = "type")]
    kind: String,
    #[serde(default)]
    values: Vec<String>,
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
    context: NonZeroU64,
    output: Option<u64>,
}

impl ModelCapabilities {
    fn parse(value: &serde_json::Value) -> Result<Self> {
        let mut model = Self::deserialize(value)?;
        // Some entries repeat the context window as the output limit.
        model.limit.output = model.limit.output.filter(|&output| {
            output > 0 && output < model.limit.context.get() && i32::try_from(output).is_ok()
        });
        Ok(model)
    }

    fn reasoning_options(&self) -> &[ReasoningOption] {
        self.reasoning_options.as_deref().unwrap_or_default()
    }

    fn has_reasoning_option(&self, kind: &str) -> bool {
        self.reasoning_options()
            .iter()
            .any(|option| option.kind == kind)
    }

    fn effort_values(&self) -> impl Iterator<Item = &str> {
        self.reasoning_options()
            .iter()
            .filter(|option| option.kind == "effort")
            .flat_map(|option| &option.values)
            .map(String::as_str)
    }

    fn efforts(
        &self,
        default_effort: BedrockAdaptiveThinkingEffort,
    ) -> Option<(
        Vec<BedrockAdaptiveThinkingEffort>,
        BedrockAdaptiveThinkingEffort,
    )> {
        let efforts = self
            .effort_values()
            .filter_map(BedrockAdaptiveThinkingEffort::from_value)
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
            if BUDGET_THINKING_ONLY.iter().any(|id| {
                base_model_id
                    .strip_prefix(id)
                    .is_some_and(|suffix| suffix.is_empty() || suffix.starts_with(['-', ':']))
            }) {
                return Some(ConverseThinking::budget(None));
            }
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
                supports_none: self.effort_values().any(|value| value == "none"),
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

#[derive(Deserialize, Serialize)]
struct CatalogEnvelope {
    #[serde(rename = "amazon-bedrock")]
    bedrock: ProviderCapabilities,
}

#[derive(Deserialize, Serialize)]
struct ProviderCapabilities {
    models: BTreeMap<String, serde_json::Value>,
}

impl CapabilityCatalog {
    /// Parses the catalog, or a part of it that holds the Bedrock provider.
    /// An entry that cannot be read is skipped, so that one malformed model
    /// does not hide the others.
    pub fn parse(catalog: &str) -> Result<Self> {
        let catalog = serde_json::from_str::<CatalogEnvelope>(catalog)
            .context("parsing the model capability catalog")?;
        Self::from_provider(&catalog.bedrock)
    }

    /// Decodes only the Bedrock section, validates it, and serializes that same
    /// section for the disk cache without decoding the response again.
    pub fn parse_with_cache(catalog: &str) -> Result<(Self, String)> {
        let catalog = serde_json::from_str::<CatalogEnvelope>(catalog)
            .context("parsing the model capability catalog")?;
        let capabilities = Self::from_provider(&catalog.bedrock)?;
        Ok((capabilities, serde_json::to_string(&catalog)?))
    }

    fn from_provider(provider: &ProviderCapabilities) -> Result<Self> {
        let models = provider
            .models
            .iter()
            .filter_map(
                |(model_id, capabilities)| match ModelCapabilities::parse(capabilities) {
                    Ok(capabilities) => Some((model_id.clone(), capabilities)),
                    Err(error) => {
                        log::warn!("Skipping Bedrock model {model_id} in the catalog: {error}");
                        None
                    }
                },
            )
            .collect::<BTreeMap<_, _>>();
        // An empty section is an error so that it never overwrites a usable cache.
        anyhow::ensure!(
            !models.is_empty(),
            "the amazon-bedrock capability catalog contains no usable models"
        );
        Ok(Self { models })
    }

    pub fn is_empty(&self) -> bool {
        self.models.is_empty()
    }

    /// Prefers the requested profile, then the foundation model, then the
    /// lexicographically first profile of the same model.
    fn capabilities(&self, base_model_id: &str, request_id: &str) -> Option<&ModelCapabilities> {
        self.models
            .get(profile_id(request_id))
            .or_else(|| self.models.get(base_model_id))
            .or_else(|| {
                self.models.iter().find_map(|(id, capabilities)| {
                    (split_profile_id(id)?.1 == base_model_id).then_some(capabilities)
                })
            })
    }

    pub fn describe(&self, base_model_id: &str, request_id: &str) -> ConverseModel {
        ConverseModel::from_capabilities(
            base_model_id,
            request_id,
            self.capabilities(base_model_id, request_id),
        )
    }

    /// Describes the model that `request_id` invokes, where `request_id` is a
    /// foundation model id or the id or ARN of an inference profile, if the
    /// catalog knows it.
    pub fn model_for_request_id(&self, request_id: &str) -> Option<ConverseModel> {
        let base_model_id = base_model_id(request_id);
        let capabilities = self.capabilities(base_model_id, request_id)?;
        Some(ConverseModel {
            id: request_id.to_string(),
            ..ConverseModel::from_capabilities(base_model_id, request_id, Some(capabilities))
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
                // Global profiles only serve commercial AWS Regions.
                let global = if region.starts_with("us-gov-") {
                    None
                } else {
                    profile_ids.get(GLOBAL_GEOGRAPHY)
                };
                let profile_id = if allow_global {
                    global.or(geographic)
                } else {
                    geographic.or(global)
                }?;
                Some(self.describe(base_model_id, profile_id))
            })
            .collect()
    }
}

impl ConverseModel {
    fn from_capabilities(
        base_model_id: &str,
        request_id: &str,
        capabilities: Option<&ModelCapabilities>,
    ) -> Self {
        let thinking = capabilities.and_then(|capabilities| capabilities.thinking(base_model_id));
        Self {
            id: base_model_id.to_string(),
            request_id: request_id.to_string(),
            base_model_id: base_model_id.to_string(),
            foundation_model_known: capabilities.is_some(),
            route: crate::ConverseModelRoute::Automatic,
            display_name: capabilities
                .map_or(base_model_id, |capabilities| &capabilities.name)
                .to_string(),
            max_tokens: capabilities.map_or(DEFAULT_CONTEXT_WINDOW, |capabilities| {
                capabilities.limit.context.get()
            }),
            max_output_tokens: capabilities.and_then(|capabilities| capabilities.limit.output),
            default_temperature: None,
            supports_temperature: capabilities.is_some_and(|capabilities| capabilities.temperature)
                && !thinking
                    .as_ref()
                    .is_some_and(ConverseThinking::is_always_on),
            // Newly listed streaming text models can precede the public catalog. Without
            // tools the agent cannot act; if the model rejects tools, Bedrock explains why.
            supports_tool_use: capabilities.is_none_or(|capabilities| capabilities.tool_call)
                && !has_prefix_in(base_model_id, STREAMING_TOOL_USE_UNSUPPORTED),
            supports_images: capabilities.is_some_and(|capabilities| {
                capabilities
                    .modalities
                    .input
                    .iter()
                    .any(|input| input == "image")
            }),
            // Claude 4 and later name the family before the version. The catalog omits cache points.
            supports_caching: base_model_id
                .strip_prefix("anthropic.claude-")
                .and_then(|rest| rest.split_once('-'))
                .is_some_and(|(family, version)| {
                    !family.is_empty()
                        && family != "instant"
                        && family.bytes().all(|letter| letter.is_ascii_alphabetic())
                        && version
                            .split(['-', ':'])
                            .next()
                            .and_then(|major| major.parse::<u32>().ok())
                            .is_some_and(|major| major >= 4)
                }),
            thinking,
            thinking_disabled: false,
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use BedrockAdaptiveThinkingEffort::{High, Low, Medium, XHigh};
    use serde_json::json;

    pub(crate) fn catalog(models: serde_json::Value) -> Result<CapabilityCatalog> {
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
        let catalog = CapabilityCatalog::default();
        let model = catalog.describe("vendor.chat", "us.vendor.chat");

        assert_eq!(model.id, "vendor.chat");
        assert_eq!(model.display_name, "vendor.chat");
        assert_eq!(model.max_tokens, DEFAULT_CONTEXT_WINDOW);
        assert_eq!(model.max_output_tokens, None);
        assert_eq!(model.thinking, None);
        assert!(model.supports_tool_use);
        assert!(!model.supports_temperature);
        assert!(!model.supports_images && !model.supports_caching);
        assert!(
            !catalog
                .describe("meta.llama-example", "us.meta.llama-example")
                .supports_tool_use
        );
    }

    #[test]
    fn cache_points_require_a_family_first_claude_id() {
        let catalog = CapabilityCatalog::default();
        for (id, expected) in [
            ("anthropic.claude-example-4-v1:0", true),
            ("anthropic.claude-3-example", false),
            ("vendor.claude-example-9", false),
        ] {
            assert_eq!(catalog.describe(id, id).supports_caching, expected, "{id}");
        }
    }

    #[test]
    fn reads_capabilities_from_the_exact_profile_then_the_bare_model() -> Result<()> {
        let catalog = catalog(json!({
            "global.vendor.chat": entry("Chat (Global)", json!({
                "limit": {"context": 500_000, "output": 64_000},
                "modalities": {"input": ["text", "image"], "output": ["text"]},
                "temperature": false,
                "tool_call": false,
            })),
            "vendor.chat": entry("Chat", json!({})),
        }))?;

        let by_profile = catalog.describe("vendor.chat", "global.vendor.chat");
        assert_eq!(by_profile.display_name, "Chat (Global)");
        assert_eq!(by_profile.max_tokens, 500_000);
        assert_eq!(by_profile.max_output_tokens, Some(64_000));
        assert!(by_profile.supports_images);
        assert!(!by_profile.supports_temperature && !by_profile.supports_tool_use);

        let by_model_id = catalog.describe("vendor.chat", "eu.vendor.chat");
        assert_eq!(by_model_id.display_name, "Chat");
        assert_eq!(by_model_id.max_output_tokens, Some(8_000));
        Ok(())
    }

    #[test]
    fn ignores_invalid_output_limits_without_dropping_the_model() -> Result<()> {
        let catalog = catalog(json!({
            "vendor.chat": entry("Chat", json!({"limit": {"context": 100, "output": 100}})),
        }))?;
        let model = catalog
            .model_for_request_id("vendor.chat")
            .context("readable context window")?;
        assert_eq!(model.max_tokens, 100);
        assert_eq!(model.max_output_tokens, None);
        Ok(())
    }

    #[test]
    fn cache_contains_only_a_usable_bedrock_section() -> Result<()> {
        let response = json!({
            "unrelated": {"models": {"vendor.unrelated": {"name": "Other"}}, "large": [1, 2, 3]},
            "amazon-bedrock": {"models": {"us.vendor.chat": entry("Chat", json!({}))}},
        })
        .to_string();
        let (catalog, cache) = CapabilityCatalog::parse_with_cache(&response)?;
        assert!(catalog.model_for_request_id("us.vendor.chat").is_some());
        assert!(!cache.contains("unrelated"));
        assert_eq!(catalog, CapabilityCatalog::parse(&cache)?);
        Ok(())
    }

    #[test]
    fn maps_reasoning_controls_to_the_request_shape_of_the_vendor() -> Result<()> {
        let efforts = json!({"type": "effort", "values": ["none", "low", "high", "xhigh"]});
        let toggle = json!({"type": "toggle"});
        let catalog = catalog(json!({
            "anthropic.always": entry("", reasoning(json!([efforts]))),
            "anthropic.optional": entry("", reasoning(json!([toggle, efforts]))),
            "anthropic.budget": entry("", reasoning(json!([toggle, {"type": "budget_tokens"}]))),
            "anthropic.claude-opus-4-5-v1": entry("", reasoning(json!([toggle, efforts]))),
            "anthropic.uncontrolled": entry("", reasoning(json!([]))),
            "anthropic.plain": entry("", json!({})),
            "openai.effort": entry("", reasoning(json!([efforts]))),
            "openai.default": entry("", reasoning(json!([
                {"type": "effort", "values": ["low", "medium", "high"]}
            ]))),
            "vendor.effort": entry("", reasoning(json!([efforts]))),
        }))?;
        let thinking = |model_id| catalog.describe(model_id, model_id).thinking;

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
        // The catalog lists efforts for this model, which rejects adaptive thinking.
        assert_eq!(
            thinking("anthropic.claude-opus-4-5-v1"),
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
                supports_none: true,
            })
        );
        assert_eq!(thinking("vendor.effort"), None);
        assert_eq!(
            thinking("openai.default"),
            Some(ConverseThinking::Reasoning {
                efforts: vec![Low, Medium, High],
                default_effort: Medium,
                supports_none: false,
            })
        );

        assert!(
            !catalog
                .describe("anthropic.always", "anthropic.always")
                .supports_temperature
        );
        assert!(
            catalog
                .describe("anthropic.optional", "anthropic.optional")
                .supports_temperature
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
        for unknown in [
            "team-a/vendor.bare",
            "arn:aws:bedrock:us-east-1:123456789012:application-inference-profile/vendor.bare",
        ] {
            assert_eq!(base_model_id(unknown), unknown);
            assert!(catalog.model_for_request_id(unknown).is_none());
        }
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
            "au.vendor.pacific": entry("Pacific (AU)", json!({})),
            "jp.vendor.pacific": entry("Pacific (JP)", json!({})),
            "apac.vendor.pacific": entry("Pacific (APAC)", json!({})),
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
        for (region, id, request_id) in [
            ("eu-west-1", "vendor.european", "eu.vendor.european"),
            ("ap-southeast-2", "vendor.pacific", "au.vendor.pacific"),
            ("ap-northeast-1", "vendor.pacific", "jp.vendor.pacific"),
            ("ap-southeast-1", "vendor.pacific", "apac.vendor.pacific"),
        ] {
            assert!(
                offered(region, false).contains(&pair(id, request_id)),
                "{region}"
            );
        }
        Ok(())
    }

    #[test]
    fn offers_a_govcloud_region_its_own_geography() -> Result<()> {
        let catalog = catalog(json!({
            "us.vendor.chat": entry("Chat (US)", json!({})),
            "us-gov.vendor.chat": entry("Chat (GovCloud)", json!({})),
            "global.vendor.chat": entry("Chat (Global)", json!({})),
            "global.vendor.global-only": entry("Global only", json!({})),
        }))?;
        let request_ids = |region, allow_global| {
            catalog
                .models_for_region(region, allow_global)
                .into_iter()
                .map(|model| model.request_id)
                .collect::<Vec<_>>()
        };

        for allow_global in [false, true] {
            assert_eq!(
                request_ids("us-gov-west-1", allow_global),
                ["us-gov.vendor.chat"]
            );
        }
        assert_eq!(
            request_ids("us-west-2", false),
            ["us.vendor.chat", "global.vendor.global-only"]
        );
        Ok(())
    }
}
