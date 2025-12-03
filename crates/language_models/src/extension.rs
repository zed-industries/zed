use extension::{ExtensionLanguageModelProviderProxy, LanguageModelProviderRegistration};
use gpui::{App, Entity};
use language_model::{LanguageModelProviderId, LanguageModelRegistry};
use std::sync::Arc;

/// Proxy implementation that registers extension-based language model providers
/// with the LanguageModelRegistry.
pub struct ExtensionLanguageModelProxy {
    registry: Entity<LanguageModelRegistry>,
}

impl ExtensionLanguageModelProxy {
    pub fn new(registry: Entity<LanguageModelRegistry>) -> Self {
        Self { registry }
    }
}

impl ExtensionLanguageModelProviderProxy for ExtensionLanguageModelProxy {
    fn register_language_model_provider(
        &self,
        provider_id: Arc<str>,
        register_fn: LanguageModelProviderRegistration,
        cx: &mut App,
    ) {
        eprintln!(
            "ExtensionLanguageModelProxy::register_language_model_provider called for {}",
            provider_id
        );
        register_fn(cx);
        eprintln!(
            "ExtensionLanguageModelProxy::register_language_model_provider completed for {}",
            provider_id
        );
    }

    fn unregister_language_model_provider(&self, provider_id: Arc<str>, cx: &mut App) {
        self.registry.update(cx, |registry, cx| {
            registry.unregister_provider(LanguageModelProviderId::from(provider_id), cx);
        });
    }
}
