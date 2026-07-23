// Provider serialization and configuration structures.

use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

pub use super::provider::open_ai_compatible::{
    OpenAiCompatibleModelConfig, OpenAiCompatibleProvider,
};

/// Runtime state maintained by the provider lifecycle trait.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LanguageModelProviderState {
    /// Models currently advertised (starts as the static list, may be
    /// overwritten/augmented by discovered models once discovery resolves).
    pub available_models: Vec<AvailableModel>,

    /// Idempotency guard: prevents concurrent discovery storms when
    /// `initialize` is invoked more than once for the same provider.
    ///
    /// This is an `Arc<AtomicBool>` (rather than a plain `bool`) so the
    /// detached background discovery task spawned in `initialize` can clone
    /// the cell and reset it to `false` once discovery resolves — without
    /// needing a handle back to this `&mut` borrow. Defaults to `false` via
    /// `derive(Default)` (`Arc::new(AtomicBool::new(false))`).
    pub discovery_in_progress: Arc<AtomicBool>,
}

/// A model entry mapped from the dynamic endpoint response.
#[derive(Clone, Debug, PartialEq)]
pub struct AvailableModel {
    pub name: gpui::SharedString,
    pub max_tokens: Option<u64>,
    pub supports_tools: Option<bool>,
    pub supports_images: Option<bool>,
}

/// Provider lifecycle trait (simplified for the recipe).
#[async_trait::async_trait]
pub trait LanguageModelProvider: Send + Sync + 'static {
    async fn initialize(
        &self,
        cx: &mut gpui::Context<Self>,
        state: &mut LanguageModelProviderState,
    ) -> Result<(), std::sync::Arc<anyhow::Error>>;
    async fn available_models(
        &self,
        cx: &mut gpui::Context<Self>,
    ) -> Result<Vec<AvailableModel>, std::sync::Arc<anyhow::Error>>;
    async fn complete(
        &self,
        cx: &mut gpui::Context<Self>,
        request: LanguageModelRequest,
    ) -> Result<LanguageModelResponse, std::sync::Arc<anyhow::Error>>;
}

/// Request / response types (stubs referencing real Zed definitions).
pub struct LanguageModelRequest;
pub struct LanguageModelResponse;

/// Top-level language-models configuration block read from
/// `settings.json` under the `language_models:` key.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LanguageModelsConfig {
    /// Provider-specific settings keyed by provider name.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub providers: Vec<ProviderEntry>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase", tag = "provider")]
pub enum ProviderEntry {
    #[serde(rename = "openai_compatible")]
    OpenAiCompatible(OpenAiCompatibleProvider),
}
