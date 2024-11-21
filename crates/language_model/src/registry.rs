use crate::{
    LanguageModel, LanguageModelId, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderState,
};
use collections::BTreeMap;
use gpui::{AppContext, EventEmitter, Global, Model, ModelContext};
use std::sync::Arc;
use ui::Context;

pub fn init(cx: &mut AppContext) {
    let registry = cx.new_model(|_cx| LanguageModelRegistry::default());
    cx.set_global(GlobalLanguageModelRegistry(registry));
}

struct GlobalLanguageModelRegistry(Model<LanguageModelRegistry>);

impl Global for GlobalLanguageModelRegistry {}

#[derive(Default)]
pub struct LanguageModelRegistry {
    active_model: Option<ActiveModel>,
    providers: BTreeMap<LanguageModelProviderId, Arc<dyn LanguageModelProvider>>,
    inline_alternatives: Vec<Arc<dyn LanguageModel>>,
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
    pub fn test(cx: &mut AppContext) -> crate::fake_provider::FakeLanguageModelProvider {
        let fake_provider = crate::fake_provider::FakeLanguageModelProvider;
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
        let zed_provider_id = LanguageModelProviderId("zed.dev".into());
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

    pub fn available_models<'a>(
        &'a self,
        cx: &'a AppContext,
    ) -> impl Iterator<Item = Arc<dyn LanguageModel>> + 'a {
        self.providers
            .values()
            .flat_map(|provider| provider.provided_models(cx))
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
        let Some(provider) = self.provider(provider) else {
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

    /// Selects and sets the inline alternatives for language models based on
    /// provider name and id.
    pub fn select_inline_alternative_models(
        &mut self,
        alternatives: impl IntoIterator<Item = (LanguageModelProviderId, LanguageModelId)>,
        cx: &mut ModelContext<Self>,
    ) {
        let mut selected_alternatives = Vec::new();

        for (provider_id, model_id) in alternatives {
            if let Some(provider) = self.providers.get(&provider_id) {
                if let Some(model) = provider
                    .provided_models(cx)
                    .iter()
                    .find(|m| m.id() == model_id)
                {
                    selected_alternatives.push(model.clone());
                }
            }
        }

        self.inline_alternatives = selected_alternatives;
    }

    /// The models to use for inline assists. Returns the union of the active
    /// model and all inline alternatives. When there are multiple models, the
    /// user will be able to cycle through results.
    pub fn inline_alternative_models(&self) -> &[Arc<dyn LanguageModel>] {
        &self.inline_alternatives
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake_provider::FakeLanguageModelProvider;

    #[gpui::test]
    fn test_register_providers(cx: &mut AppContext) {
        let registry = cx.new_model(|_| LanguageModelRegistry::default());

        registry.update(cx, |registry, cx| {
            registry.register_provider(FakeLanguageModelProvider, cx);
        });

        let providers = registry.read(cx).providers();
        assert_eq!(providers.len(), 1);
        assert_eq!(providers[0].id(), crate::fake_provider::provider_id());

        registry.update(cx, |registry, cx| {
            registry.unregister_provider(crate::fake_provider::provider_id(), cx);
        });

        let providers = registry.read(cx).providers();
        assert!(providers.is_empty());
    }
}
