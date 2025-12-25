use collections::HashMap;
use extension::{
    ExtensionHostProxy, ExtensionLanguageModelProviderProxy, LanguageModelProviderRegistration,
};
use gpui::{App, Entity};
use language_model::{LanguageModelProviderId, LanguageModelRegistry};
use std::sync::{Arc, LazyLock};

/// Maps built-in provider IDs to their corresponding extension IDs.
/// When an extension with this ID is installed, the built-in provider should be hidden.
static BUILTIN_TO_EXTENSION_MAP: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(|| {
        let mut map = HashMap::default();
        map.insert("anthropic", "anthropic");
        map.insert("openai", "openai");
        map.insert("google", "google-ai");
        map.insert("openrouter", "openrouter");
        map.insert("copilot_chat", "copilot-chat");
        map
    });

/// Returns the extension ID that should hide the given built-in provider.
pub fn extension_for_builtin_provider(provider_id: &str) -> Option<&'static str> {
    BUILTIN_TO_EXTENSION_MAP.get(provider_id).copied()
}

/// Proxy that registers extension language model providers with the LanguageModelRegistry.
pub struct LanguageModelProviderRegistryProxy {
    registry: Entity<LanguageModelRegistry>,
}

impl LanguageModelProviderRegistryProxy {
    pub fn new(registry: Entity<LanguageModelRegistry>) -> Self {
        Self { registry }
    }
}

impl ExtensionLanguageModelProviderProxy for LanguageModelProviderRegistryProxy {
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

/// Initialize the extension language model provider proxy.
/// This must be called BEFORE extension_host::init to ensure the proxy is available
/// when extensions try to register their language model providers.
pub fn init_proxy(cx: &mut App) {
    let proxy = ExtensionHostProxy::default_global(cx);
    let registry = LanguageModelRegistry::global(cx);

    registry.update(cx, |registry, _cx| {
        registry.set_builtin_provider_hiding_fn(Box::new(extension_for_builtin_provider));
    });

    proxy.register_language_model_provider_proxy(LanguageModelProviderRegistryProxy::new(registry));
}
