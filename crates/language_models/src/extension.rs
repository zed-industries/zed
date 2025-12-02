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
        _provider_id: Arc<str>,
        register_fn: LanguageModelProviderRegistration,
        cx: &mut App,
    ) {
        register_fn(cx);
    }

    fn unregister_language_model_provider(&self, provider_id: Arc<str>, cx: &mut App) {
        self.registry.update(cx, |registry, cx| {
            registry.unregister_provider(LanguageModelProviderId::from(provider_id), cx);
        });
    }
}
