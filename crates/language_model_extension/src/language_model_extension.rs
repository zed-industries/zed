use std::sync::Arc;

use extension::{Extension, ExtensionHostProxy, ExtensionLanguageModelProxy};
use gpui::App;
use language_model::{
    LanguageModelProvider, LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelRegistry,
};

struct LanguageModelRegistryProxy;

pub fn init(extension_host_proxy: Arc<ExtensionHostProxy>) {
    extension_host_proxy.register_language_model_proxy(LanguageModelRegistryProxy);
}

impl ExtensionLanguageModelProxy for LanguageModelRegistryProxy {
    fn register_language_model_provider(
        &self,
        extension: Arc<dyn Extension>,
        provider_id: LanguageModelProviderId,
        cx: &mut App,
    ) {
        todo!()
    }
    fn remove_language_model_provider(&self, provider_id: LanguageModelProviderId, cx: &mut App) {
        LanguageModelRegistry::global(cx).update(cx, |registry, cx| {
            registry.unregister_provider(provider_id, cx);
        });
    }
}

struct ExtensionLanguageModelAdapter {
    extension: Arc<dyn Extension>,
    provider_id: LanguageModelProviderId,
    provider_name: LanguageModelProviderName,
}

impl LanguageModelProvider for ExtensionLanguageModelAdapter {
    fn id(&self) -> LanguageModelProviderId {
        self.language_model_id.clone()
    }

    fn name(&self) -> LanguageModelProviderName {
        self.language_model_name.clone()
    }

    fn default_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {}
}
