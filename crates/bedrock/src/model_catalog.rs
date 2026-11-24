//! Lists the models that can be invoked through an inference profile, from
//! two Bedrock control plane calls. `ListInferenceProfiles` says which models
//! a Region reaches and through which profile. `ListFoundationModels` gives
//! each model's name and modalities.
//!
//! Neither call reports token limits, tool use or reasoning support. Those
//! come from the [`CapabilityCatalog`].

use crate::{CapabilityCatalog, ConverseModel, GLOBAL_GEOGRAPHY, split_profile_id};
use anyhow::{Result, anyhow};
use aws_sdk_bedrock::Client;
use aws_sdk_bedrock::error::{DisplayErrorContext, ProvideErrorMetadata, SdkError};
use aws_sdk_bedrock::types::{
    FoundationModelLifecycleStatus, FoundationModelSummary, InferenceProfileStatus,
    InferenceProfileSummary, InferenceProfileType, ModelModality,
};
use futures::FutureExt;
use std::collections::BTreeMap;

/// Models with an inference profile that the Converse API does not serve.
const CONVERSE_UNSUPPORTED: &[&str] = &["twelvelabs.pegasus"];

fn supports_converse_stream(model: &FoundationModelSummary) -> bool {
    model.input_modalities().contains(&ModelModality::Text)
        && model.output_modalities() == [ModelModality::Text]
        && model.response_streaming_supported() == Some(true)
        && !CONVERSE_UNSUPPORTED
            .iter()
            .any(|prefix| model.model_id().starts_with(prefix))
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
    models: Vec<ListedModel>,
}

#[derive(Clone, Debug, PartialEq)]
struct ListedModel {
    /// The id Zed stores for the model: the foundation model id, or the ARN
    /// of an application inference profile.
    id: String,
    base_model_id: String,
    display_name: String,
    supports_images: bool,
    is_active: bool,
    geographic_profile_id: Option<String>,
    global_profile_id: Option<String>,
    /// Every profile id and ARN that routes to this model.
    routes: Vec<String>,
}

impl ListedModel {
    fn new(id: &str, display_name: &str, model: &FoundationModelSummary) -> Self {
        Self {
            id: id.to_string(),
            base_model_id: model.model_id().to_string(),
            display_name: display_name.to_string(),
            supports_images: model.input_modalities().contains(&ModelModality::Image),
            is_active: model.model_lifecycle().is_some_and(|lifecycle| {
                lifecycle.status() == &FoundationModelLifecycleStatus::Active
            }),
            geographic_profile_id: None,
            global_profile_id: None,
            routes: Vec::new(),
        }
    }

    fn request_id(&self, allow_global: bool) -> &str {
        let geographic = self.geographic_profile_id.as_deref();
        let global = self.global_profile_id.as_deref();
        let profile_id = if allow_global {
            global.or(geographic)
        } else {
            geographic.or(global)
        };
        profile_id.unwrap_or(&self.id)
    }

    fn converse_model(
        &self,
        request_id: &str,
        capability_catalog: &CapabilityCatalog,
    ) -> ConverseModel {
        ConverseModel {
            id: self.id.clone(),
            foundation_model_known: true,
            route: if self.id == self.base_model_id {
                crate::ConverseModelRoute::Automatic
            } else {
                crate::ConverseModelRoute::Pinned
            },
            display_name: self.display_name.clone(),
            supports_images: self.supports_images,
            ..capability_catalog.describe(&self.base_model_id, request_id)
        }
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
            .map(|model| (model.model_id(), model))
            .collect::<BTreeMap<_, _>>();

        // A Region can offer several geographies for one model. In this order,
        // the first is the one that keeps requests closest to the Region.
        let mut inference_profiles = inference_profiles.iter().collect::<Vec<_>>();
        inference_profiles
            .sort_by_key(|profile| (profile.models().len(), profile.inference_profile_id()));

        let mut system_defined = BTreeMap::<&str, ListedModel>::new();
        let mut application = Vec::new();
        for profile in inference_profiles {
            if profile.status() != &InferenceProfileStatus::Active {
                continue;
            }
            let Some((model_id, model)) = foundation_model_id(profile)
                .and_then(|model_id| Some((model_id, *foundation_models.get(model_id)?)))
            else {
                continue;
            };
            let profile_id = profile.inference_profile_id();
            let profile_arn = profile.inference_profile_arn();
            let routes = vec![profile_id.to_string(), profile_arn.to_string()];
            match profile.r#type() {
                InferenceProfileType::SystemDefined => {
                    let listed_model = system_defined.entry(model_id).or_insert_with(|| {
                        ListedModel::new(model_id, model.model_name().unwrap_or(model_id), model)
                    });
                    listed_model.routes.extend(routes);
                    let preferred =
                        if matches!(split_profile_id(profile_id), Some((GLOBAL_GEOGRAPHY, _))) {
                            &mut listed_model.global_profile_id
                        } else {
                            &mut listed_model.geographic_profile_id
                        };
                    preferred.get_or_insert_with(|| profile_id.to_string());
                }
                // An application profile can only be invoked by its ARN.
                InferenceProfileType::Application => application.push(ListedModel {
                    routes,
                    ..ListedModel::new(profile_arn, profile.inference_profile_name(), model)
                }),
                _ => {}
            }
        }

        let mut models = system_defined.into_values().collect::<Vec<_>>();
        models.extend(application);
        Self { models }
    }

    /// Every model in the catalog, described by `capability_catalog`. A model
    /// reachable through both a geographic and a global profile is routed
    /// through the global one only when `allow_global` is set.
    pub fn models<'a>(
        &'a self,
        allow_global: bool,
        capability_catalog: &'a CapabilityCatalog,
    ) -> impl Iterator<Item = ConverseModel> + 'a {
        self.models.iter().map(move |model| {
            model.converse_model(model.request_id(allow_global), capability_catalog)
        })
    }

    /// Whether Bedrock lists the model with this id as active rather than legacy.
    pub fn is_active(&self, model_id: &str) -> bool {
        self.models
            .iter()
            .any(|model| model.id == model_id && model.is_active)
    }

    /// The model that `request_id` resolves to, where `request_id` is a
    /// foundation model id or the id or ARN of an inference profile.
    pub fn model_for_request_id(
        &self,
        request_id: &str,
        capability_catalog: &CapabilityCatalog,
    ) -> Option<ConverseModel> {
        self.models
            .iter()
            .find(|model| model.id == request_id || model.routes.iter().any(|id| id == request_id))
            .map(|model| ConverseModel {
                id: request_id.to_string(),
                ..model.converse_model(request_id, capability_catalog)
            })
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
    let operation = format!("ListInferenceProfiles ({profile_type})");
    client
        .list_inference_profiles()
        .type_equals(profile_type)
        .into_paginator()
        .items()
        .send()
        .collect::<Result<Vec<_>, _>>()
        .await
        .map_err(|error| list_error("ListInferenceProfiles", error).context(operation))
}

/// Lists the models that `client` can reach through an inference profile in
/// its Region.
pub async fn list_models(
    client: &Client,
    deadline: impl Future<Output = ()> + Send,
) -> Result<ModelCatalog> {
    let foundation_models = async {
        client
            .list_foundation_models()
            .send()
            .await
            .map(|response| response.model_summaries.unwrap_or_default())
            .map_err(|error| list_error("ListFoundationModels", error))
    };
    // An unfiltered listing is not documented to include the profiles
    // created in the account, so each kind is requested by name.
    let listing = async {
        let (foundation_models, mut system, application) = futures::try_join!(
            foundation_models,
            list_inference_profiles(client, InferenceProfileType::SystemDefined),
            list_inference_profiles(client, InferenceProfileType::Application),
        )?;
        system.extend(application);
        Ok(ModelCatalog::new(&system, &foundation_models))
    };
    futures::select_biased! {
        result = listing.fuse() => result,
        _ = deadline.fuse() => Err(anyhow!("model listing deadline exceeded")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability_catalog::tests::catalog as capability_catalog;
    use anyhow::Context as _;
    use aws_sdk_bedrock::types::InferenceProfileModel;
    use serde_json::json;

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
        let capability_catalog = CapabilityCatalog::default();
        let mut models = catalog.models(allow_global, &capability_catalog);
        let model = models.next().context("the catalog is empty")?;
        anyhow::ensure!(models.next().is_none(), "the catalog has several models");
        Ok(model)
    }

    #[test]
    fn skips_profiles_that_cannot_serve_a_streamed_text_conversation() -> Result<()> {
        let mut embedding = foundation_model("vendor.embedding", &[])?;
        embedding.output_modalities = Some(vec![ModelModality::Embedding]);
        let mut blocking = foundation_model("vendor.blocking", &[])?;
        blocking.response_streaming_supported = Some(false);
        let mut inactive = profile("us.vendor.chat", "vendor.chat", 1)?;
        inactive.status = InferenceProfileStatus::from("RETIRED");
        let catalog = ModelCatalog::new(
            &[
                profile("us.vendor.embedding", "vendor.embedding", 1)?,
                profile("us.vendor.blocking", "vendor.blocking", 1)?,
                profile("us.vendor.unlisted", "vendor.unlisted", 1)?,
                inactive,
            ],
            &[embedding, blocking, foundation_model("vendor.chat", &[])?],
        );

        assert_eq!(
            catalog.models(false, &CapabilityCatalog::default()).count(),
            0
        );
        Ok(())
    }

    #[test]
    fn routes_through_the_global_profile_only_when_allowed() -> Result<()> {
        let models = [foundation_model("vendor.chat", &[])?];
        let profiles = [
            profile("global.vendor.chat", "vendor.chat", 2)?,
            profile("us.vendor.chat", "vendor.chat", 3)?,
        ];
        for (profiles, allow_global, request_id) in [
            (&profiles[..], false, "us.vendor.chat"),
            (&profiles[..], true, "global.vendor.chat"),
            // The only profile a model has serves it either way.
            (&profiles[..1], false, "global.vendor.chat"),
            (&profiles[1..], true, "us.vendor.chat"),
        ] {
            let model = only_model(&ModelCatalog::new(profiles, &models), allow_global)?;
            assert_eq!(model.id, "vendor.chat");
            assert_eq!(model.base_model_id, "vendor.chat");
            assert_eq!(model.request_id, request_id);
        }
        Ok(())
    }

    #[test]
    fn prefers_the_geographic_profile_with_the_fewest_destinations() -> Result<()> {
        let catalog = ModelCatalog::new(
            &[
                profile("aaa.vendor.chat", "vendor.chat", 4)?,
                profile("zzz.vendor.chat", "vendor.chat", 2)?,
                profile("mmm.vendor.chat", "vendor.chat", 5)?,
            ],
            &[foundation_model("vendor.chat", &[])?],
        );

        assert_eq!(only_model(&catalog, false)?.request_id, "zzz.vendor.chat");
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
    fn resolves_exact_and_sibling_limits_without_changing_the_requested_profile() -> Result<()> {
        let application_arn = format!("{PROFILE_ARN_PREFIX}application-inference-profile/example");
        let catalog = ModelCatalog::new(
            &[
                profile("us.vendor.chat", "vendor.chat", 1)?,
                profile("global.vendor.chat", "vendor.chat", 1)?,
                profile_of_type(
                    InferenceProfileType::Application,
                    "example",
                    application_arn.clone(),
                    "vendor.chat",
                    1,
                )?,
            ],
            &[foundation_model("vendor.chat", &[ModelModality::Image])?],
        );
        let capabilities = capability_catalog(json!({
            "us.vendor.chat": {"name": "US", "limit": {"context": 300}},
            "global.vendor.chat": {"name": "Global", "limit": {"context": 500}},
        }))?;
        let global_arn = format!("{PROFILE_ARN_PREFIX}inference-profile/global.vendor.chat");
        let geographic_arn = format!("{PROFILE_ARN_PREFIX}inference-profile/us.vendor.chat");
        for (request_id, context) in [
            ("vendor.chat", 500),
            ("global.vendor.chat", 500),
            (&global_arn, 500),
            ("us.vendor.chat", 300),
            (&geographic_arn, 300),
            (&application_arn, 500),
        ] {
            let model = catalog
                .model_for_request_id(request_id, &capabilities)
                .context("listed profile")?;
            assert_eq!(model.request_id, request_id);
            assert_eq!(model.base_model_id, "vendor.chat");
            assert_eq!(model.max_tokens, context, "{request_id}");
        }
        assert!(
            catalog
                .model_for_request_id("vendor.other", &capabilities)
                .is_none()
        );
        // Bedrock's own listing is the authority on the name and modalities.
        let model = catalog
            .model_for_request_id("us.vendor.chat", &capabilities)
            .context("listed profile")?;
        assert_eq!(model.display_name, "Name of vendor.chat");
        assert!(model.supports_images);
        Ok(())
    }
}
