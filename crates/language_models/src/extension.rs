use collections::HashMap;
use extension::{ExtensionLanguageModelProviderProxy, LanguageModelProviderRegistration};
use gpui::{App, Entity};
use language_model::{LanguageModelProviderId, LanguageModelRegistry};
use std::sync::{Arc, LazyLock};

/// Maps built-in provider IDs to their corresponding extension IDs.
/// When an extension with this ID is installed, the built-in provider should be hidden.
pub static BUILTIN_TO_EXTENSION_MAP: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(|| {
        let mut map = HashMap::default();
        map.insert("anthropic", "anthropic");
        map.insert("openai", "openai");
        map.insert("google", "google-ai");
        map.insert("open_router", "open-router");
        map.insert("copilot_chat", "copilot-chat");
        map
    });

/// Returns the extension ID that should hide the given built-in provider.
pub fn extension_for_builtin_provider(provider_id: &str) -> Option<&'static str> {
    BUILTIN_TO_EXTENSION_MAP.get(provider_id).copied()
}

/// Returns true if the given provider ID is a built-in provider that can be hidden by an extension.
pub fn is_hideable_builtin_provider(provider_id: &str) -> bool {
    BUILTIN_TO_EXTENSION_MAP.contains_key(provider_id)
}

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
        let _ = provider_id;
        register_fn(cx);
    }

    fn unregister_language_model_provider(&self, provider_id: Arc<str>, cx: &mut App) {
        self.registry.update(cx, |registry, cx| {
            registry.unregister_provider(LanguageModelProviderId::from(provider_id), cx);
        });
    }
}
