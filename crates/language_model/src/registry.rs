use crate::{
    LanguageModel, LanguageModelId, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderState,
};
use collections::BTreeMap;
use gpui::{App, Context, Entity, EventEmitter, Global, prelude::*};
use std::sync::Arc;

pub fn init(cx: &mut App) {
    let registry = cx.new(|_cx| LanguageModelRegistry::default());
    cx.set_global(GlobalLanguageModelRegistry(registry));
}

struct GlobalLanguageModelRegistry(Entity<LanguageModelRegistry>);

impl Global for GlobalLanguageModelRegistry {}

#[derive(Default)]
pub struct LanguageModelRegistry {
    default_model: Option<ConfiguredModel>,
    inline_assistant_model: Option<ConfiguredModel>,
    commit_message_model: Option<ConfiguredModel>,
    thread_summary_model: Option<ConfiguredModel>,
    providers: BTreeMap<LanguageModelProviderId, Arc<dyn LanguageModelProvider>>,
    inline_alternatives: Vec<Arc<dyn LanguageModel>>,
}

#[derive(Clone)]
pub struct ConfiguredModel {
    pub provider: Arc<dyn LanguageModelProvider>,
    pub model: Arc<dyn LanguageModel>,
}

pub enum Event {
    DefaultModelChanged,
    InlineAssistantModelChanged,
    CommitMessageModelChanged,
    ThreadSummaryModelChanged,
    ProviderStateChanged,
    AddedProvider(LanguageModelProviderId),
    RemovedProvider(LanguageModelProviderId),
}

impl EventEmitter<Event> for LanguageModelRegistry {}

impl LanguageModelRegistry {
    pub fn global(cx: &App) -> Entity<Self> {
        cx.global::<GlobalLanguageModelRegistry>().0.clone()
    }

    pub fn read_global(cx: &App) -> &Self {
        cx.global::<GlobalLanguageModelRegistry>().0.read(cx)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test(cx: &mut App) -> crate::fake_provider::FakeLanguageModelProvider {
        let fake_provider = crate::fake_provider::FakeLanguageModelProvider;
        let registry = cx.new(|cx| {
            let mut registry = Self::default();
            registry.register_provider(fake_provider.clone(), cx);
            let model = fake_provider.provided_models(cx)[0].clone();
            registry.set_default_model(Some(model), cx);
            registry
        });
        cx.set_global(GlobalLanguageModelRegistry(registry));
        fake_provider
    }

    pub fn register_provider<T: LanguageModelProvider + LanguageModelProviderState>(
        &mut self,
        provider: T,
        cx: &mut Context<Self>,
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

    pub fn unregister_provider(&mut self, id: LanguageModelProviderId, cx: &mut Context<Self>) {
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
        cx: &'a App,
    ) -> impl Iterator<Item = Arc<dyn LanguageModel>> + 'a {
        self.providers
            .values()
            .flat_map(|provider| provider.provided_models(cx))
    }

    pub fn provider(&self, id: &LanguageModelProviderId) -> Option<Arc<dyn LanguageModelProvider>> {
        self.providers.get(id).cloned()
    }

    pub fn select_default_model(
        &mut self,
        provider: &LanguageModelProviderId,
        model_id: &LanguageModelId,
        cx: &mut Context<Self>,
    ) {
        let Some(provider) = self.provider(provider) else {
            return;
        };

        let models = provider.provided_models(cx);
        if let Some(model) = models.iter().find(|model| &model.id() == model_id).cloned() {
            self.set_default_model(Some(model), cx);
        }
    }

    pub fn select_inline_assistant_model(
        &mut self,
        provider: &LanguageModelProviderId,
        model_id: &LanguageModelId,
        cx: &mut Context<Self>,
    ) {
        let Some(provider) = self.provider(provider) else {
            return;
        };

        let models = provider.provided_models(cx);
        if let Some(model) = models.iter().find(|model| &model.id() == model_id).cloned() {
            self.set_inline_assistant_model(Some(model), cx);
        }
    }

    pub fn select_commit_message_model(
        &mut self,
        provider: &LanguageModelProviderId,
        model_id: &LanguageModelId,
        cx: &mut Context<Self>,
    ) {
        let Some(provider) = self.provider(provider) else {
            return;
        };

        let models = provider.provided_models(cx);
        if let Some(model) = models.iter().find(|model| &model.id() == model_id).cloned() {
            self.set_commit_message_model(Some(model), cx);
        }
    }

    pub fn select_thread_summary_model(
        &mut self,
        provider: &LanguageModelProviderId,
        model_id: &LanguageModelId,
        cx: &mut Context<Self>,
    ) {
        let Some(provider) = self.provider(provider) else {
            return;
        };

        let models = provider.provided_models(cx);
        if let Some(model) = models.iter().find(|model| &model.id() == model_id).cloned() {
            self.set_thread_summary_model(Some(model), cx);
        }
    }

    pub fn set_default_model(
        &mut self,
        model: Option<Arc<dyn LanguageModel>>,
        cx: &mut Context<Self>,
    ) {
        if let Some(model) = model {
            let provider_id = model.provider_id();
            if let Some(provider) = self.providers.get(&provider_id).cloned() {
                self.default_model = Some(ConfiguredModel { provider, model });
                cx.emit(Event::DefaultModelChanged);
            } else {
                log::warn!("Active model's provider not found in registry");
            }
        } else {
            self.default_model = None;
            cx.emit(Event::DefaultModelChanged);
        }
    }

    pub fn set_inline_assistant_model(
        &mut self,
        model: Option<Arc<dyn LanguageModel>>,
        cx: &mut Context<Self>,
    ) {
        if let Some(model) = model {
            let provider_id = model.provider_id();
            if let Some(provider) = self.providers.get(&provider_id).cloned() {
                self.inline_assistant_model = Some(ConfiguredModel { provider, model });
                cx.emit(Event::InlineAssistantModelChanged);
            } else {
                log::warn!("Inline assistant model's provider not found in registry");
            }
        } else {
            self.inline_assistant_model = None;
            cx.emit(Event::InlineAssistantModelChanged);
        }
    }

    pub fn set_commit_message_model(
        &mut self,
        model: Option<Arc<dyn LanguageModel>>,
        cx: &mut Context<Self>,
    ) {
        if let Some(model) = model {
            let provider_id = model.provider_id();
            if let Some(provider) = self.providers.get(&provider_id).cloned() {
                self.commit_message_model = Some(ConfiguredModel { provider, model });
                cx.emit(Event::CommitMessageModelChanged);
            } else {
                log::warn!("Commit message model's provider not found in registry");
            }
        } else {
            self.commit_message_model = None;
            cx.emit(Event::CommitMessageModelChanged);
        }
    }

    pub fn set_thread_summary_model(
        &mut self,
        model: Option<Arc<dyn LanguageModel>>,
        cx: &mut Context<Self>,
    ) {
        if let Some(model) = model {
            let provider_id = model.provider_id();
            if let Some(provider) = self.providers.get(&provider_id).cloned() {
                self.thread_summary_model = Some(ConfiguredModel { provider, model });
                cx.emit(Event::ThreadSummaryModelChanged);
            } else {
                log::warn!("Thread summary model's provider not found in registry");
            }
        } else {
            self.thread_summary_model = None;
            cx.emit(Event::ThreadSummaryModelChanged);
        }
    }

    pub fn default_model(&self) -> Option<ConfiguredModel> {
        #[cfg(debug_assertions)]
        if std::env::var("ZED_SIMULATE_NO_LLM_PROVIDER").is_ok() {
            return None;
        }

        self.default_model.clone()
    }

    pub fn inline_assistant_model(&self) -> Option<ConfiguredModel> {
        self.inline_assistant_model
            .clone()
            .or_else(|| self.default_model())
    }

    pub fn commit_message_model(&self) -> Option<ConfiguredModel> {
        self.commit_message_model
            .clone()
            .or_else(|| self.default_model())
    }

    pub fn thread_summary_model(&self) -> Option<ConfiguredModel> {
        self.thread_summary_model
            .clone()
            .or_else(|| self.default_model())
    }

    /// Selects and sets the inline alternatives for language models based on
    /// provider name and id.
    pub fn select_inline_alternative_models(
        &mut self,
        alternatives: impl IntoIterator<Item = (LanguageModelProviderId, LanguageModelId)>,
        cx: &mut Context<Self>,
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
    fn test_register_providers(cx: &mut App) {
        let registry = cx.new(|_| LanguageModelRegistry::default());

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
