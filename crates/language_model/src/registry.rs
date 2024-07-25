use client::Client;
use collections::BTreeMap;
use gpui::{AppContext, Global, Model, ModelContext};
use std::sync::Arc;
use ui::Context;

use crate::{
    provider::{
        anthropic::AnthropicLanguageModelProvider, cloud::CloudLanguageModelProvider,
        ollama::OllamaLanguageModelProvider, open_ai::OpenAiLanguageModelProvider,
    },
    LanguageModel, LanguageModelProvider, LanguageModelProviderId, LanguageModelProviderState,
};

pub fn init(client: Arc<Client>, cx: &mut AppContext) {
    let registry = cx.new_model(|cx| {
        let mut registry = LanguageModelRegistry::default();
        register_language_model_providers(&mut registry, client, cx);
        registry
    });
    cx.set_global(GlobalLanguageModelRegistry(registry));
}

fn register_language_model_providers(
    registry: &mut LanguageModelRegistry,
    client: Arc<Client>,
    cx: &mut ModelContext<LanguageModelRegistry>,
) {
    use feature_flags::FeatureFlagAppExt;

    registry.register_provider(
        AnthropicLanguageModelProvider::new(client.http_client(), cx),
        cx,
    );
    registry.register_provider(
        OpenAiLanguageModelProvider::new(client.http_client(), cx),
        cx,
    );
    registry.register_provider(
        OllamaLanguageModelProvider::new(client.http_client(), cx),
        cx,
    );

    cx.observe_flag::<feature_flags::LanguageModels, _>(move |enabled, cx| {
        let client = client.clone();
        LanguageModelRegistry::global(cx).update(cx, move |registry, cx| {
            if enabled {
                registry.register_provider(CloudLanguageModelProvider::new(client.clone(), cx), cx);
            } else {
                registry.unregister_provider(
                    &LanguageModelProviderId::from(
                        crate::provider::cloud::PROVIDER_NAME.to_string(),
                    ),
                    cx,
                );
            }
        });
    })
    .detach();
}

struct GlobalLanguageModelRegistry(Model<LanguageModelRegistry>);

impl Global for GlobalLanguageModelRegistry {}

#[derive(Default)]
pub struct LanguageModelRegistry {
    providers: BTreeMap<LanguageModelProviderId, Arc<dyn LanguageModelProvider>>,
}

impl LanguageModelRegistry {
    pub fn global(cx: &AppContext) -> Model<Self> {
        cx.global::<GlobalLanguageModelRegistry>().0.clone()
    }

    pub fn read_global(cx: &AppContext) -> &Self {
        cx.global::<GlobalLanguageModelRegistry>().0.read(cx)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test(cx: &mut AppContext) -> crate::provider::fake::FakeLanguageModelProvider {
        let fake_provider = crate::provider::fake::FakeLanguageModelProvider::default();
        let registry = cx.new_model(|cx| {
            let mut registry = Self::default();
            registry.register_provider(fake_provider.clone(), cx);
            registry
        });
        cx.set_global(GlobalLanguageModelRegistry(registry));
        fake_provider
    }

    pub fn register_provider<T: LanguageModelProvider + LanguageModelProviderState>(
        &mut self,
        provider: T,
        cx: &mut ModelContext<Self>,
    ) {
        let name = provider.id();

        if let Some(subscription) = provider.subscribe(cx) {
            subscription.detach();
        }

        self.providers.insert(name, Arc::new(provider));
        cx.notify();
    }

    pub fn unregister_provider(
        &mut self,
        name: &LanguageModelProviderId,
        cx: &mut ModelContext<Self>,
    ) {
        if self.providers.remove(name).is_some() {
            cx.notify();
        }
    }

    pub fn providers(&self) -> impl Iterator<Item = &Arc<dyn LanguageModelProvider>> {
        self.providers.values()
    }

    pub fn available_models(&self, cx: &AppContext) -> Vec<Arc<dyn LanguageModel>> {
        self.providers
            .values()
            .flat_map(|provider| provider.provided_models(cx))
            .collect()
    }

    pub fn provider(
        &self,
        name: &LanguageModelProviderId,
    ) -> Option<Arc<dyn LanguageModelProvider>> {
        self.providers.get(name).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::fake::FakeLanguageModelProvider;

    #[gpui::test]
    fn test_register_providers(cx: &mut AppContext) {
        let registry = cx.new_model(|_| LanguageModelRegistry::default());

        registry.update(cx, |registry, cx| {
            registry.register_provider(FakeLanguageModelProvider::default(), cx);
        });

        let providers = registry.read(cx).providers().collect::<Vec<_>>();
        assert_eq!(providers.len(), 1);
        assert_eq!(providers[0].id(), crate::provider::fake::provider_id());

        registry.update(cx, |registry, cx| {
            registry.unregister_provider(&crate::provider::fake::provider_id(), cx);
        });

        let providers = registry.read(cx).providers().collect::<Vec<_>>();
        assert!(providers.is_empty());
    }
}
