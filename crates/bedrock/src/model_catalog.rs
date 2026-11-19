//! Builds the Converse model list from two Bedrock control plane calls.
//! `ListInferenceProfiles` says which models can be invoked from a Region and
//! through which inference profile. `ListFoundationModels` gives each model's
//! name and modalities.
//!
//! Neither call reports token limits, tool use or reasoning support, so those
//! come from the rules in this file. The rules name model families and the
//! older models that differ from their family, so a new model in a known
//! family needs no change here.

use crate::{BedrockAdaptiveThinkingEffort, ConverseModel, ConverseThinking};
use anyhow::{Result, anyhow};
use aws_sdk_bedrock::Client;
use aws_sdk_bedrock::error::{DisplayErrorContext, ProvideErrorMetadata, SdkError};
use aws_sdk_bedrock::types::{
    FoundationModelSummary, InferenceProfileStatus, InferenceProfileSummary, InferenceProfileType,
    ModelModality,
};
use std::collections::BTreeMap;

const GLOBAL_PROFILE_ID_PREFIX: &str = "global.";

/// Context windows by foundation model id prefix; the first match wins.
const CONTEXT_WINDOWS: &[(&str, u64)] = &[
    ("anthropic.claude-3", 200_000),
    ("anthropic.claude-opus-4-1", 200_000),
    ("anthropic.claude", 1_000_000),
    ("amazon.nova-micro", 128_000),
    ("amazon.nova-premier", 1_000_000),
    ("amazon.nova", 300_000),
    ("openai.gpt-6", 1_050_000),
    ("openai.gpt-5.6", 1_000_000),
    ("openai.gpt-5", 272_000),
    ("xai.grok", 1_000_000),
    ("zai.glm", 203_000),
];
const DEFAULT_CONTEXT_WINDOW: u64 = 128_000;

/// Models that reject tools on `ConverseStream`, which is the only Converse
/// operation Zed uses.
///
/// <https://docs.aws.amazon.com/bedrock/latest/userguide/tool-use-client-side.html>
const STREAMING_TOOL_USE_UNSUPPORTED: &[&str] = &[
    "meta.llama",
    "mistral.pixtral-large",
    "writer.palmyra",
    "deepseek.r1",
];

/// Models with an inference profile that the Converse API does not serve.
const CONVERSE_UNSUPPORTED: &[&str] = &["twelvelabs.pegasus"];

const CLAUDE_MODEL_ID_PREFIX: &str = "anthropic.claude";
/// Zed sends Claude 3 models neither thinking fields nor cache points.
const CLAUDE_3_MODEL_ID_PREFIX: &str = "anthropic.claude-3";

/// Claude models that think within a token budget, by model id prefix.
/// Together with the next list this is every Claude 4 and later model that
/// predates adaptive thinking on every request, so neither list grows with
/// new models.
const CLAUDE_BUDGET_THINKING_MODELS: &[&str] = &[
    "anthropic.claude-opus-4-2025",
    "anthropic.claude-opus-4-1",
    "anthropic.claude-opus-4-5",
    "anthropic.claude-sonnet-4-2025",
    "anthropic.claude-sonnet-4-5",
    "anthropic.claude-haiku-4-5",
];

/// Claude models on which adaptive thinking can be turned off, and whether
/// they accept the `xhigh` effort. The ids are matched whole, because a later
/// model's id can extend an earlier one's.
const CLAUDE_OPTIONAL_ADAPTIVE_THINKING_MODELS: &[(&str, bool)] = &[
    ("anthropic.claude-opus-4-6-v1", false),
    ("anthropic.claude-sonnet-4-6", false),
    ("anthropic.claude-opus-4-7", false),
    ("anthropic.claude-opus-4-8", true),
    ("anthropic.claude-opus-5", true),
    ("anthropic.claude-sonnet-5", true),
    ("anthropic.claude-fable-5", true),
];

const OPENAI_GPT_MODEL_ID_PREFIX: &str = "openai.gpt-";
const OPENAI_OPEN_WEIGHT_MODEL_ID_PREFIX: &str = "openai.gpt-oss";
/// The last GPT generation without the `max` reasoning effort.
const OPENAI_GPT_5_MODEL_ID_PREFIX: &str = "openai.gpt-5";

fn has_prefix_in(model_id: &str, prefixes: &[&str]) -> bool {
    prefixes.iter().any(|prefix| model_id.starts_with(prefix))
}

fn context_window(model_id: &str) -> Option<u64> {
    CONTEXT_WINDOWS
        .iter()
        .find(|(prefix, _)| model_id.starts_with(prefix))
        .map(|(_, context_window)| *context_window)
}

fn thinking(model_id: &str) -> Option<ConverseThinking> {
    if model_id.starts_with(OPENAI_GPT_MODEL_ID_PREFIX) {
        if model_id.starts_with(OPENAI_OPEN_WEIGHT_MODEL_ID_PREFIX) {
            return None;
        }
        let mut efforts = vec![
            BedrockAdaptiveThinkingEffort::Low,
            BedrockAdaptiveThinkingEffort::Medium,
            BedrockAdaptiveThinkingEffort::High,
            BedrockAdaptiveThinkingEffort::XHigh,
        ];
        if !model_id.starts_with(OPENAI_GPT_5_MODEL_ID_PREFIX) {
            efforts.push(BedrockAdaptiveThinkingEffort::Max);
        }
        return Some(ConverseThinking::Reasoning {
            efforts,
            default_effort: BedrockAdaptiveThinkingEffort::Medium,
        });
    }

    if !model_id.starts_with(CLAUDE_MODEL_ID_PREFIX)
        || model_id.starts_with(CLAUDE_3_MODEL_ID_PREFIX)
    {
        return None;
    }
    if has_prefix_in(model_id, CLAUDE_BUDGET_THINKING_MODELS) {
        return Some(ConverseThinking::budget(None));
    }
    let optional_adaptive_thinking = CLAUDE_OPTIONAL_ADAPTIVE_THINKING_MODELS
        .iter()
        .find(|(optional_model_id, _)| *optional_model_id == model_id);
    Some(match optional_adaptive_thinking {
        Some((_, supports_xhigh)) => ConverseThinking::adaptive(*supports_xhigh, false),
        None => ConverseThinking::adaptive(true, true),
    })
}

/// Whether the model can hold a text conversation over `ConverseStream`.
fn supports_converse_stream(model: &FoundationModelSummary) -> bool {
    model.input_modalities().contains(&ModelModality::Text)
        && model.output_modalities() == [ModelModality::Text]
        && model.response_streaming_supported() == Some(true)
        && !has_prefix_in(model.model_id(), CONVERSE_UNSUPPORTED)
}

/// What the rules say about the foundation model `model_id`. Its name and
/// modalities are not part of the rules.
fn describe_model(model_id: &str) -> ConverseModel {
    let thinking = thinking(model_id);
    ConverseModel {
        id: model_id.to_string(),
        request_id: model_id.to_string(),
        base_model_id: model_id.to_string(),
        display_name: model_id.to_string(),
        max_tokens: context_window(model_id).unwrap_or(DEFAULT_CONTEXT_WINDOW),
        max_output_tokens: None,
        default_temperature: None,
        supports_temperature: !thinking
            .as_ref()
            .is_some_and(ConverseThinking::is_always_on),
        supports_tool_use: !has_prefix_in(model_id, STREAMING_TOOL_USE_UNSUPPORTED),
        supports_images: false,
        supports_caching: model_id.starts_with(CLAUDE_MODEL_ID_PREFIX)
            && !model_id.starts_with(CLAUDE_3_MODEL_ID_PREFIX),
        thinking,
    }
}

fn converse_model(model: &FoundationModelSummary) -> ConverseModel {
    let model_id = model.model_id();
    ConverseModel {
        display_name: model.model_name().unwrap_or(model_id).to_string(),
        supports_images: model.input_modalities().contains(&ModelModality::Image),
        ..describe_model(model_id)
    }
}

/// Describes the model behind `request_id` when Bedrock did not list it,
/// where `request_id` is a foundation model id or the id or ARN of an
/// inference profile. An id the rules do not recognize describes a model of
/// an unknown family.
pub fn describe_unlisted_model(request_id: &str) -> ConverseModel {
    let profile_id = request_id
        .rsplit_once('/')
        .map_or(request_id, |(_, profile_id)| profile_id);
    // A profile id is a foundation model id behind a geography, and the set
    // of geographies is open. Drop leading segments until the rules
    // recognize what remains.
    let base_model_id = std::iter::successors(Some(profile_id), |model_id| {
        model_id.split_once('.').map(|(_, rest)| rest)
    })
    .find(|model_id| {
        context_window(model_id).is_some()
            || has_prefix_in(model_id, STREAMING_TOOL_USE_UNSUPPORTED)
    })
    .unwrap_or(profile_id);
    ConverseModel {
        id: request_id.to_string(),
        request_id: request_id.to_string(),
        display_name: request_id.to_string(),
        ..describe_model(base_model_id)
    }
}

/// The foundation model a profile routes to. Every destination Region of a
/// profile serves the same model.
fn foundation_model_id(profile: &InferenceProfileSummary) -> Option<&str> {
    profile.models().iter().find_map(|model| {
        let (_, model_id) = model.model_arn()?.split_once("foundation-model/")?;
        Some(model_id)
    })
}

/// The models that can be invoked through an inference profile from one
/// Region, with the profiles that route to each of them.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ModelCatalog {
    models: Vec<CatalogModel>,
}

#[derive(Clone, Debug, PartialEq)]
struct CatalogModel {
    model: ConverseModel,
    geographic_profile: Option<GeographicProfile>,
    global_profile_id: Option<String>,
    /// Every profile id and ARN that routes to this model.
    routes: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
struct GeographicProfile {
    id: String,
    destination_count: usize,
}

impl CatalogModel {
    fn request_id(&self, allow_global: bool) -> &str {
        let geographic = self
            .geographic_profile
            .as_ref()
            .map(|profile| profile.id.as_str());
        let global = self.global_profile_id.as_deref();
        let profile_id = if allow_global {
            global.or(geographic)
        } else {
            geographic.or(global)
        };
        profile_id.unwrap_or(&self.model.request_id)
    }
}

impl ModelCatalog {
    pub fn new(
        inference_profiles: &[InferenceProfileSummary],
        foundation_models: &[FoundationModelSummary],
    ) -> Self {
        let foundation_models = foundation_models
            .iter()
            .filter(|model| supports_converse_stream(model))
            .map(|model| (model.model_id(), converse_model(model)))
            .collect::<BTreeMap<_, _>>();

        let mut system_defined = BTreeMap::<&str, CatalogModel>::new();
        let mut application = Vec::new();
        for profile in inference_profiles {
            if profile.status() != &InferenceProfileStatus::Active {
                continue;
            }
            let Some((model_id, model)) = foundation_model_id(profile)
                .and_then(|model_id| Some((model_id, foundation_models.get(model_id)?)))
            else {
                continue;
            };
            let profile_id = profile.inference_profile_id();
            let profile_arn = profile.inference_profile_arn();
            let routes = vec![profile_id.to_string(), profile_arn.to_string()];
            match profile.r#type() {
                InferenceProfileType::SystemDefined => {
                    let catalog_model =
                        system_defined
                            .entry(model_id)
                            .or_insert_with(|| CatalogModel {
                                model: model.clone(),
                                geographic_profile: None,
                                global_profile_id: None,
                                routes: Vec::new(),
                            });
                    catalog_model.routes.extend(routes);
                    if profile_id.starts_with(GLOBAL_PROFILE_ID_PREFIX) {
                        catalog_model.global_profile_id = Some(profile_id.to_string());
                        continue;
                    }
                    // A Region can offer several geographies for one model.
                    // Prefer the one that keeps requests closest to it.
                    let destination_count = profile.models().len();
                    let is_narrower =
                        catalog_model
                            .geographic_profile
                            .as_ref()
                            .is_none_or(|current| {
                                (destination_count, profile_id)
                                    < (current.destination_count, current.id.as_str())
                            });
                    if is_narrower {
                        catalog_model.geographic_profile = Some(GeographicProfile {
                            id: profile_id.to_string(),
                            destination_count,
                        });
                    }
                }
                // An application profile can only be invoked by its ARN.
                InferenceProfileType::Application => application.push(CatalogModel {
                    model: ConverseModel {
                        id: profile_arn.to_string(),
                        request_id: profile_arn.to_string(),
                        display_name: profile.inference_profile_name().to_string(),
                        ..model.clone()
                    },
                    geographic_profile: None,
                    global_profile_id: None,
                    routes,
                }),
                _ => {}
            }
        }

        let mut models = system_defined.into_values().collect::<Vec<_>>();
        models.extend(application);
        Self { models }
    }

    /// Every model in the catalog. A model reachable through both a
    /// geographic and a global profile is routed through the global one only
    /// when `allow_global` is set.
    pub fn models(&self, allow_global: bool) -> impl Iterator<Item = ConverseModel> + '_ {
        self.models.iter().map(move |catalog_model| ConverseModel {
            request_id: catalog_model.request_id(allow_global).to_string(),
            ..catalog_model.model.clone()
        })
    }

    /// The model that `request_id` resolves to, where `request_id` is a
    /// foundation model id or the id or ARN of an inference profile.
    pub fn model_for_request_id(&self, request_id: &str) -> Option<&ConverseModel> {
        self.models
            .iter()
            .find(|catalog_model| {
                catalog_model.model.id == request_id
                    || catalog_model.routes.iter().any(|route| route == request_id)
            })
            .map(|catalog_model| &catalog_model.model)
    }
}

fn list_error<Error, Response>(operation: &str, error: SdkError<Error, Response>) -> anyhow::Error
where
    Error: ProvideErrorMetadata + std::error::Error + 'static,
    Response: std::fmt::Debug,
{
    let details = DisplayErrorContext(&error);
    if error.code() == Some("AccessDeniedException") {
        anyhow!(
            "{operation} was denied. Listing models requires the `bedrock:{operation}` \
             permission: {details}"
        )
    } else {
        anyhow!("{operation} failed: {details}")
    }
}

async fn list_inference_profiles(
    client: &Client,
    profile_type: InferenceProfileType,
) -> Result<Vec<InferenceProfileSummary>> {
    client
        .list_inference_profiles()
        .type_equals(profile_type)
        .into_paginator()
        .items()
        .send()
        .collect::<Result<Vec<_>, _>>()
        .await
        .map_err(|error| list_error("ListInferenceProfiles", error))
}

/// Lists the models that `client` can reach through an inference profile in
/// its Region.
pub async fn list_models(client: &Client) -> Result<ModelCatalog> {
    let foundation_models = async {
        client
            .list_foundation_models()
            .send()
            .await
            .map_err(|error| list_error("ListFoundationModels", error))
    };
    // An unfiltered listing is not documented to include the profiles
    // created in the account, so each kind is requested by name.
    let (mut inference_profiles, application_inference_profiles, foundation_models) = futures::try_join!(
        list_inference_profiles(client, InferenceProfileType::SystemDefined),
        list_inference_profiles(client, InferenceProfileType::Application),
        foundation_models,
    )?;
    inference_profiles.extend(application_inference_profiles);
    Ok(ModelCatalog::new(
        &inference_profiles,
        foundation_models.model_summaries(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context as _;
    use aws_sdk_bedrock::types::InferenceProfileModel;

    const PROFILE_ARN_PREFIX: &str = "arn:aws:bedrock:us-east-1:123456789012:";

    /// A text model that streams, taking the extra `input_modalities`.
    fn foundation_model(
        model_id: &str,
        input_modalities: &[ModelModality],
    ) -> Result<FoundationModelSummary> {
        let mut builder = FoundationModelSummary::builder()
            .model_arn(format!(
                "arn:aws:bedrock:us-east-1::foundation-model/{model_id}"
            ))
            .model_id(model_id)
            .model_name(format!("Name of {model_id}"))
            .input_modalities(ModelModality::Text)
            .output_modalities(ModelModality::Text)
            .response_streaming_supported(true);
        for modality in input_modalities {
            builder = builder.input_modalities(modality.clone());
        }
        Ok(builder.build()?)
    }

    fn profile_of_type(
        profile_type: InferenceProfileType,
        profile_id: &str,
        profile_arn: String,
        model_id: &str,
        destination_count: usize,
    ) -> Result<InferenceProfileSummary> {
        let mut builder = InferenceProfileSummary::builder()
            .inference_profile_id(profile_id)
            .inference_profile_arn(profile_arn)
            .inference_profile_name(format!("Name of {profile_id}"))
            .status(InferenceProfileStatus::Active)
            .r#type(profile_type);
        for index in 0..destination_count {
            builder = builder.models(
                InferenceProfileModel::builder()
                    .model_arn(format!(
                        "arn:aws:bedrock:region-{index}::foundation-model/{model_id}"
                    ))
                    .build(),
            );
        }
        Ok(builder.build()?)
    }

    fn profile(
        profile_id: &str,
        model_id: &str,
        destination_count: usize,
    ) -> Result<InferenceProfileSummary> {
        profile_of_type(
            InferenceProfileType::SystemDefined,
            profile_id,
            format!("{PROFILE_ARN_PREFIX}inference-profile/{profile_id}"),
            model_id,
            destination_count,
        )
    }

    fn only_model(catalog: &ModelCatalog, allow_global: bool) -> Result<ConverseModel> {
        let mut models = catalog.models(allow_global);
        let model = models.next().context("the catalog is empty")?;
        anyhow::ensure!(models.next().is_none(), "the catalog has several models");
        Ok(model)
    }

    /// The model a catalog lists for a foundation model with one profile.
    fn listed_model(model_id: &str) -> Result<ConverseModel> {
        let catalog = ModelCatalog::new(
            &[profile("us.model", model_id, 1)?],
            &[foundation_model(model_id, &[])?],
        );
        only_model(&catalog, false)
    }

    #[test]
    fn lists_a_model_once_under_its_foundation_model_id() -> Result<()> {
        let catalog = ModelCatalog::new(
            &[
                profile("us.vendor.chat", "vendor.chat", 3)?,
                profile("global.vendor.chat", "vendor.chat", 2)?,
            ],
            &[foundation_model("vendor.chat", &[ModelModality::Image])?],
        );
        let model = only_model(&catalog, false)?;

        assert_eq!(model.id, "vendor.chat");
        assert_eq!(model.base_model_id, "vendor.chat");
        assert_eq!(model.display_name, "Name of vendor.chat");
        assert!(model.supports_images);
        Ok(())
    }

    #[test]
    fn skips_profiles_that_cannot_serve_a_streamed_text_conversation() -> Result<()> {
        let embedding = FoundationModelSummary::builder()
            .model_arn("arn:aws:bedrock:us-east-1::foundation-model/vendor.embedding")
            .model_id("vendor.embedding")
            .input_modalities(ModelModality::Text)
            .output_modalities(ModelModality::Embedding)
            .response_streaming_supported(true)
            .build()?;
        let blocking = FoundationModelSummary::builder()
            .model_arn("arn:aws:bedrock:us-east-1::foundation-model/vendor.blocking")
            .model_id("vendor.blocking")
            .input_modalities(ModelModality::Text)
            .output_modalities(ModelModality::Text)
            .response_streaming_supported(false)
            .build()?;
        let inactive = InferenceProfileSummary::builder()
            .inference_profile_id("us.vendor.chat")
            .inference_profile_arn(format!(
                "{PROFILE_ARN_PREFIX}inference-profile/us.vendor.chat"
            ))
            .inference_profile_name("Inactive")
            .models(
                InferenceProfileModel::builder()
                    .model_arn("arn:aws:bedrock:us-east-1::foundation-model/vendor.chat")
                    .build(),
            )
            .status(InferenceProfileStatus::from("RETIRED"))
            .r#type(InferenceProfileType::SystemDefined)
            .build()?;
        let catalog = ModelCatalog::new(
            &[
                profile("us.vendor.embedding", "vendor.embedding", 1)?,
                profile("us.vendor.blocking", "vendor.blocking", 1)?,
                profile("us.vendor.unlisted", "vendor.unlisted", 1)?,
                inactive,
            ],
            &[embedding, blocking, foundation_model("vendor.chat", &[])?],
        );

        assert_eq!(catalog.models(false).count(), 0);
        Ok(())
    }

    #[test]
    fn routes_through_the_global_profile_only_when_allowed() -> Result<()> {
        let catalog = ModelCatalog::new(
            &[
                profile("global.vendor.chat", "vendor.chat", 2)?,
                profile("us.vendor.chat", "vendor.chat", 3)?,
            ],
            &[foundation_model("vendor.chat", &[])?],
        );

        assert_eq!(only_model(&catalog, false)?.request_id, "us.vendor.chat");
        assert_eq!(only_model(&catalog, true)?.request_id, "global.vendor.chat");
        Ok(())
    }

    #[test]
    fn falls_back_to_the_only_profile_a_model_has() -> Result<()> {
        let models = [foundation_model("vendor.chat", &[])?];

        let global_only =
            ModelCatalog::new(&[profile("global.vendor.chat", "vendor.chat", 2)?], &models);
        assert_eq!(
            only_model(&global_only, false)?.request_id,
            "global.vendor.chat"
        );

        let geographic_only =
            ModelCatalog::new(&[profile("eu.vendor.chat", "vendor.chat", 2)?], &models);
        assert_eq!(
            only_model(&geographic_only, true)?.request_id,
            "eu.vendor.chat"
        );
        Ok(())
    }

    #[test]
    fn prefers_the_geographic_profile_with_the_fewest_destinations() -> Result<()> {
        let catalog = ModelCatalog::new(
            &[
                profile("wide.vendor.chat", "vendor.chat", 4)?,
                profile("narrow.vendor.chat", "vendor.chat", 2)?,
                profile("wider.vendor.chat", "vendor.chat", 5)?,
            ],
            &[foundation_model("vendor.chat", &[])?],
        );

        assert_eq!(
            only_model(&catalog, false)?.request_id,
            "narrow.vendor.chat"
        );
        Ok(())
    }

    #[test]
    fn lists_an_application_profile_as_its_own_model_invoked_by_arn() -> Result<()> {
        let arn = format!("{PROFILE_ARN_PREFIX}application-inference-profile/abc123");
        let catalog = ModelCatalog::new(
            &[profile_of_type(
                InferenceProfileType::Application,
                "abc123",
                arn.clone(),
                "vendor.chat",
                1,
            )?],
            &[foundation_model("vendor.chat", &[])?],
        );
        let model = only_model(&catalog, true)?;

        assert_eq!(model.id, arn);
        assert_eq!(model.request_id, arn);
        assert_eq!(model.display_name, "Name of abc123");
        assert_eq!(model.base_model_id, "vendor.chat");
        Ok(())
    }

    #[test]
    fn resolves_model_ids_profile_ids_and_profile_arns() -> Result<()> {
        let catalog = ModelCatalog::new(
            &[profile("us.vendor.chat", "vendor.chat", 1)?],
            &[foundation_model("vendor.chat", &[])?],
        );
        let profile_arn = format!("{PROFILE_ARN_PREFIX}inference-profile/us.vendor.chat");

        for request_id in ["vendor.chat", "us.vendor.chat", profile_arn.as_str()] {
            let model = catalog
                .model_for_request_id(request_id)
                .with_context(|| format!("{request_id} did not resolve"))?;
            assert_eq!(model.base_model_id, "vendor.chat");
        }
        assert!(catalog.model_for_request_id("vendor.other").is_none());
        Ok(())
    }

    #[test]
    fn describes_a_model_of_an_unknown_family_conservatively() -> Result<()> {
        let model = listed_model("vendor.chat")?;

        assert_eq!(model.max_tokens, DEFAULT_CONTEXT_WINDOW);
        assert_eq!(model.max_output_tokens, None);
        assert_eq!(model.default_temperature, None);
        assert_eq!(model.thinking, None);
        assert!(model.supports_tool_use && model.supports_temperature);
        assert!(!model.supports_images && !model.supports_caching);
        Ok(())
    }

    #[test]
    fn treats_a_new_model_like_the_newest_of_its_family() -> Result<()> {
        let claude = listed_model("anthropic.claude-not-yet-released")?;
        assert!(claude.supports_caching && !claude.supports_temperature);
        assert_eq!(
            claude.thinking,
            Some(ConverseThinking::adaptive(true, true))
        );

        let gpt = listed_model("openai.gpt-not-yet-released")?;
        assert!(!gpt.supports_temperature);
        assert!(matches!(
            gpt.thinking,
            Some(ConverseThinking::Reasoning {
                default_effort: BedrockAdaptiveThinkingEffort::Medium,
                ..
            })
        ));
        Ok(())
    }

    #[test]
    fn keeps_older_models_on_the_thinking_they_were_released_with() -> Result<()> {
        for model_id_prefix in CLAUDE_BUDGET_THINKING_MODELS {
            let revision = format!("{model_id_prefix}-later-revision");
            for model_id in [*model_id_prefix, revision.as_str()] {
                assert_eq!(
                    listed_model(model_id)?.thinking,
                    Some(ConverseThinking::budget(None)),
                    "{model_id}"
                );
            }
        }
        for (model_id, supports_xhigh) in CLAUDE_OPTIONAL_ADAPTIVE_THINKING_MODELS {
            let model = listed_model(model_id)?;
            assert_eq!(
                model.thinking,
                Some(ConverseThinking::adaptive(*supports_xhigh, false)),
                "{model_id}"
            );
            assert!(model.supports_temperature);

            let successor = listed_model(&format!("{model_id}-5"))?;
            assert!(
                successor
                    .thinking
                    .is_some_and(|thinking| thinking.is_always_on()),
                "a model whose id extends {model_id} is a different model"
            );
        }
        Ok(())
    }

    #[test]
    fn describes_an_unlisted_model_by_the_model_its_id_names() {
        for request_id in [
            "anthropic.claude-not-yet-released",
            "us.anthropic.claude-not-yet-released",
            "arn:aws:bedrock:us-east-1:123456789012:inference-profile/eu.anthropic.claude-not-yet-released",
        ] {
            let model = describe_unlisted_model(request_id);
            assert_eq!(model.id, request_id);
            assert_eq!(model.request_id, request_id);
            assert_eq!(model.base_model_id, "anthropic.claude-not-yet-released");
            assert!(
                model
                    .thinking
                    .is_some_and(|thinking| thinking.is_always_on())
            );
        }

        let unknown = describe_unlisted_model("us.vendor.chat-1.5");
        assert_eq!(unknown.base_model_id, "us.vendor.chat-1.5");
        assert_eq!(unknown.thinking, None);
        assert_eq!(unknown.max_tokens, DEFAULT_CONTEXT_WINDOW);
    }
}
