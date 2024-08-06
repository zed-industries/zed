use crate::{
    provider::{
        anthropic::AnthropicLanguageModelProvider, cloud::CloudLanguageModelProvider,
        copilot_chat::CopilotChatLanguageModelProvider, google::GoogleLanguageModelProvider,
        ollama::OllamaLanguageModelProvider, open_ai::OpenAiLanguageModelProvider,
    },
    LanguageModel, LanguageModelId, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderState,
};
use client::{Client, UserStore};
use collections::BTreeMap;
use gpui::{AppContext, EventEmitter, Global, Model, ModelContext};
use std::sync::Arc;
use ui::Context;

pub fn init(user_store: Model<UserStore>, client: Arc<Client>, cx: &mut AppContext) {
    let registry = cx.new_model(|cx| {
        let mut registry = LanguageModelRegistry::default();
        register_language_model_providers(&mut registry, user_store, client, cx);
        registry
    });
    cx.set_global(GlobalLanguageModelRegistry(registry));
}

fn register_language_model_providers(
    registry: &mut LanguageModelRegistry,
    user_store: Model<UserStore>,
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
    registry.register_provider(
        GoogleLanguageModelProvider::new(client.http_client(), cx),
        cx,
    );
    registry.register_provider(CopilotChatLanguageModelProvider::new(cx), cx);

    cx.observe_flag::<feature_flags::LanguageModels, _>(move |enabled, cx| {
        let user_store = user_store.clone();
        let client = client.clone();
        LanguageModelRegistry::global(cx).update(cx, move |registry, cx| {
            if enabled {
                registry.register_provider(
                    CloudLanguageModelProvider::new(user_store.clone(), client.clone(), cx),
                    cx,
                );
            } else {
                registry.unregister_provider(
                    LanguageModelProviderId::from(crate::provider::cloud::PROVIDER_ID.to_string()),
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
    active_model: Option<ActiveModel>,
    providers: BTreeMap<LanguageModelProviderId, Arc<dyn LanguageModelProvider>>,
}

pub struct ActiveModel {
    provider: Arc<dyn LanguageModelProvider>,
    model: Option<Arc<dyn LanguageModel>>,
}

pub enum Event {
    ActiveModelChanged,
    ProviderStateChanged,
    AddedProvider(LanguageModelProviderId),
    RemovedProvider(LanguageModelProviderId),
}

impl EventEmitter<Event> for LanguageModelRegistry {}

impl LanguageModelRegistry {
    pub fn global(cx: &AppContext) -> Model<Self> {
        cx.global::<GlobalLanguageModelRegistry>().0.clone()
    }

    pub fn read_global(cx: &AppContext) -> &Self {
        cx.global::<GlobalLanguageModelRegistry>().0.read(cx)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test(cx: &mut AppContext) -> crate::provider::fake::FakeLanguageModelProvider {
        let fake_provider = crate::provider::fake::FakeLanguageModelProvider;
        let registry = cx.new_model(|cx| {
            let mut registry = Self::default();
            registry.register_provider(fake_provider.clone(), cx);
            let model = fake_provider.provided_models(cx)[0].clone();
            registry.set_active_model(Some(model), cx);
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
        let id = provider.id();

        let subscription = provider.subscribe(cx, |_, cx| {
            cx.emit(Event::ProviderStateChanged);
        });
        if let Some(subscription) = subscription {
            subscription.detach();
        }

        self.providers.insert(id.clone(), Arc::new(provider));
        cx.emit(Event::AddedProvider(id));
    }

    pub fn unregister_provider(
        &mut self,
        id: LanguageModelProviderId,
        cx: &mut ModelContext<Self>,
    ) {
        if self.providers.remove(&id).is_some() {
            cx.emit(Event::RemovedProvider(id));
        }
    }

    pub fn providers(&self) -> Vec<Arc<dyn LanguageModelProvider>> {
        let zed_provider_id = LanguageModelProviderId(crate::provider::cloud::PROVIDER_ID.into());
        let mut providers = Vec::with_capacity(self.providers.len());
        if let Some(provider) = self.providers.get(&zed_provider_id) {
            providers.push(provider.clone());
        }
        providers.extend(self.providers.values().filter_map(|p| {
            if p.id() != zed_provider_id {
                Some(p.clone())
            } else {
                None
            }
        }));
        providers
    }

    pub fn available_models(&self, cx: &AppContext) -> Vec<Arc<dyn LanguageModel>> {
        self.providers
            .values()
            .flat_map(|provider| provider.provided_models(cx))
            .collect()
    }

    pub fn provider(&self, id: &LanguageModelProviderId) -> Option<Arc<dyn LanguageModelProvider>> {
        self.providers.get(id).cloned()
    }

    pub fn select_active_model(
        &mut self,
        provider: &LanguageModelProviderId,
        model_id: &LanguageModelId,
        cx: &mut ModelContext<Self>,
    ) {
        let Some(provider) = self.provider(&provider) else {
            return;
        };

        let models = provider.provided_models(cx);
        if let Some(model) = models.iter().find(|model| &model.id() == model_id).cloned() {
            self.set_active_model(Some(model), cx);
        }
    }

    pub fn set_active_provider(
        &mut self,
        provider: Option<Arc<dyn LanguageModelProvider>>,
        cx: &mut ModelContext<Self>,
    ) {
        self.active_model = provider.map(|provider| ActiveModel {
            provider,
            model: None,
        });
        cx.emit(Event::ActiveModelChanged);
    }

    pub fn set_active_model(
        &mut self,
        model: Option<Arc<dyn LanguageModel>>,
        cx: &mut ModelContext<Self>,
    ) {
        if let Some(model) = model {
            let provider_id = model.provider_id();
            if let Some(provider) = self.providers.get(&provider_id).cloned() {
                self.active_model = Some(ActiveModel {
                    provider,
                    model: Some(model),
                });
                cx.emit(Event::ActiveModelChanged);
            } else {
                log::warn!("Active model's provider not found in registry");
            }
        } else {
            self.active_model = None;
            cx.emit(Event::ActiveModelChanged);
        }
    }

    pub fn active_provider(&self) -> Option<Arc<dyn LanguageModelProvider>> {
        Some(self.active_model.as_ref()?.provider.clone())
    }

    pub fn active_model(&self) -> Option<Arc<dyn LanguageModel>> {
        self.active_model.as_ref()?.model.clone()
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
            registry.register_provider(FakeLanguageModelProvider, cx);
        });

        let providers = registry.read(cx).providers();
        assert_eq!(providers.len(), 1);
        assert_eq!(providers[0].id(), crate::provider::fake::provider_id());

        registry.update(cx, |registry, cx| {
            registry.unregister_provider(crate::provider::fake::provider_id(), cx);
        });

        let providers = registry.read(cx).providers();
        assert!(providers.is_empty());
    }
}
