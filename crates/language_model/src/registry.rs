use crate::{
    LanguageModel, LanguageModelId, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderState,
};
use collections::BTreeMap;
use gpui::{App, Context, Entity, EventEmitter, Global, prelude::*};
use std::{str::FromStr, sync::Arc};
use thiserror::Error;

pub fn init(cx: &mut App) {
    let registry = cx.new(|_cx| LanguageModelRegistry::default());
    cx.set_global(GlobalLanguageModelRegistry(registry));
}

struct GlobalLanguageModelRegistry(Entity<LanguageModelRegistry>);

impl Global for GlobalLanguageModelRegistry {}

#[derive(Error)]
pub enum ConfigurationError {
    #[error("Configure at least one LLM provider to start using the panel.")]
    NoProvider,
    #[error("LLM provider is not configured or does not support the configured model.")]
    ModelNotFound,
    #[error("{} LLM provider is not configured.", .0.name().0)]
    ProviderNotAuthenticated(Arc<dyn LanguageModelProvider>),
}

impl std::fmt::Debug for ConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoProvider => write!(f, "NoProvider"),
            Self::ModelNotFound => write!(f, "ModelNotFound"),
            Self::ProviderNotAuthenticated(provider) => {
                write!(f, "ProviderNotAuthenticated({})", provider.id())
            }
        }
    }
}

#[derive(Default)]
pub struct LanguageModelRegistry {
    default_model: Option<ConfiguredModel>,
    /// This model is automatically configured by a user's environment after
    /// authenticating all providers. It's only used when default_model is not available.
    environment_fallback_model: Option<ConfiguredModel>,
    inline_assistant_model: Option<ConfiguredModel>,
    commit_message_model: Option<ConfiguredModel>,
    thread_summary_model: Option<ConfiguredModel>,
    providers: BTreeMap<LanguageModelProviderId, Arc<dyn LanguageModelProvider>>,
    inline_alternatives: Vec<Arc<dyn LanguageModel>>,
}

#[derive(Debug)]
pub struct SelectedModel {
    pub provider: LanguageModelProviderId,
    pub model: LanguageModelId,
}

impl FromStr for SelectedModel {
    type Err = String;

    /// Parse string identifiers like `provider_id/model_id` into a `SelectedModel`
    fn from_str(id: &str) -> Result<SelectedModel, Self::Err> {
        let parts: Vec<&str> = id.split('/').collect();
        let [provider_id, model_id] = parts.as_slice() else {
            return Err(format!(
                "Invalid model identifier format: `{}`. Expected `provider_id/model_id`",
                id
            ));
        };

        if provider_id.is_empty() || model_id.is_empty() {
            return Err(format!("Provider and model ids can't be empty: `{}`", id));
        }

        Ok(SelectedModel {
            provider: LanguageModelProviderId(provider_id.to_string().into()),
            model: LanguageModelId(model_id.to_string().into()),
        })
    }
}

#[derive(Clone)]
pub struct ConfiguredModel {
    pub provider: Arc<dyn LanguageModelProvider>,
    pub model: Arc<dyn LanguageModel>,
}

impl ConfiguredModel {
    pub fn is_same_as(&self, other: &ConfiguredModel) -> bool {
        self.model.id() == other.model.id() && self.provider.id() == other.provider.id()
    }

    pub fn is_provided_by_zed(&self) -> bool {
        self.provider.id() == crate::ZED_CLOUD_PROVIDER_ID
    }
}

pub enum Event {
    DefaultModelChanged,
    ProviderStateChanged(LanguageModelProviderId),
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
        let fake_provider = crate::fake_provider::FakeLanguageModelProvider::default();
        let registry = cx.new(|cx| {
            let mut registry = Self::default();
            registry.register_provider(fake_provider.clone(), cx);
            let model = fake_provider.provided_models(cx)[0].clone();
            let configured_model = ConfiguredModel {
                provider: Arc::new(fake_provider.clone()),
                model,
            };
            registry.set_default_model(Some(configured_model), cx);
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

        let subscription = provider.subscribe(cx, {
            let id = id.clone();
            move |_, cx| {
                cx.emit(Event::ProviderStateChanged(id.clone()));
            }
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

    pub fn configuration_error(
        &self,
        model: Option<ConfiguredModel>,
        cx: &App,
    ) -> Option<ConfigurationError> {
        let Some(model) = model else {
            if !self.has_authenticated_provider(cx) {
                return Some(ConfigurationError::NoProvider);
            }
            return Some(ConfigurationError::ModelNotFound);
        };

        if !model.provider.is_authenticated(cx) {
            return Some(ConfigurationError::ProviderNotAuthenticated(model.provider));
        }

        None
    }

    /// Returns `true` if at least one provider that is authenticated.
    pub fn has_authenticated_provider(&self, cx: &App) -> bool {
        self.providers.values().any(|p| p.is_authenticated(cx))
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

    pub fn select_default_model(&mut self, model: Option<&SelectedModel>, cx: &mut Context<Self>) {
        let configured_model = model.and_then(|model| self.select_model(model, cx));
        self.set_default_model(configured_model, cx);
    }

    pub fn select_inline_assistant_model(
        &mut self,
        model: Option<&SelectedModel>,
        cx: &mut Context<Self>,
    ) {
        let configured_model = model.and_then(|model| self.select_model(model, cx));
        self.set_inline_assistant_model(configured_model);
    }

    pub fn select_commit_message_model(
        &mut self,
        model: Option<&SelectedModel>,
        cx: &mut Context<Self>,
    ) {
        let configured_model = model.and_then(|model| self.select_model(model, cx));
        self.set_commit_message_model(configured_model);
    }

    pub fn select_thread_summary_model(
        &mut self,
        model: Option<&SelectedModel>,
        cx: &mut Context<Self>,
    ) {
        let configured_model = model.and_then(|model| self.select_model(model, cx));
        self.set_thread_summary_model(configured_model);
    }

    /// Selects and sets the inline alternatives for language models based on
    /// provider name and id.
    pub fn select_inline_alternative_models(
        &mut self,
        alternatives: impl IntoIterator<Item = SelectedModel>,
        cx: &mut Context<Self>,
    ) {
        self.inline_alternatives = alternatives
            .into_iter()
            .flat_map(|alternative| {
                self.select_model(&alternative, cx)
                    .map(|configured_model| configured_model.model)
            })
            .collect::<Vec<_>>();
    }

    pub fn select_model(
        &mut self,
        selected_model: &SelectedModel,
        cx: &mut Context<Self>,
    ) -> Option<ConfiguredModel> {
        let provider = self.provider(&selected_model.provider)?;
        let model = provider
            .provided_models(cx)
            .iter()
            .find(|model| model.id() == selected_model.model)?
            .clone();
        Some(ConfiguredModel { provider, model })
    }

    pub fn set_default_model(&mut self, model: Option<ConfiguredModel>, cx: &mut Context<Self>) {
        match (self.default_model(), model.as_ref()) {
            (Some(old), Some(new)) if old.is_same_as(new) => {}
            (None, None) => {}
            _ => cx.emit(Event::DefaultModelChanged),
        }
        self.default_model = model;
    }

    pub fn set_environment_fallback_model(
        &mut self,
        model: Option<ConfiguredModel>,
        cx: &mut Context<Self>,
    ) {
        if self.default_model.is_none() {
            match (self.environment_fallback_model.as_ref(), model.as_ref()) {
                (Some(old), Some(new)) if old.is_same_as(new) => {}
                (None, None) => {}
                _ => cx.emit(Event::DefaultModelChanged),
            }
        }
        self.environment_fallback_model = model;
    }

    pub fn set_inline_assistant_model(&mut self, model: Option<ConfiguredModel>) {
        self.inline_assistant_model = model;
    }

    pub fn set_commit_message_model(&mut self, model: Option<ConfiguredModel>) {
        self.commit_message_model = model;
    }

    pub fn set_thread_summary_model(&mut self, model: Option<ConfiguredModel>) {
        self.thread_summary_model = model;
    }

    #[track_caller]
    pub fn default_model(&self) -> Option<ConfiguredModel> {
        #[cfg(debug_assertions)]
        if std::env::var("ZED_SIMULATE_NO_LLM_PROVIDER").is_ok() {
            return None;
        }

        self.default_model
            .clone()
            .or_else(|| self.environment_fallback_model.clone())
    }

    pub fn default_fast_model(&self, cx: &App) -> Option<ConfiguredModel> {
        let provider = self.default_model()?.provider;
        let fast_model = provider.default_fast_model(cx)?;
        Some(ConfiguredModel {
            provider,
            model: fast_model,
        })
    }

    pub fn inline_assistant_model(&self) -> Option<ConfiguredModel> {
        #[cfg(debug_assertions)]
        if std::env::var("ZED_SIMULATE_NO_LLM_PROVIDER").is_ok() {
            return None;
        }

        self.inline_assistant_model
            .clone()
            .or_else(|| self.default_model.clone())
    }

    pub fn commit_message_model(&self, cx: &App) -> Option<ConfiguredModel> {
        #[cfg(debug_assertions)]
        if std::env::var("ZED_SIMULATE_NO_LLM_PROVIDER").is_ok() {
            return None;
        }

        self.commit_message_model
            .clone()
            .or_else(|| self.default_fast_model(cx))
            .or_else(|| self.default_model.clone())
    }

    pub fn thread_summary_model(&self, cx: &App) -> Option<ConfiguredModel> {
        #[cfg(debug_assertions)]
        if std::env::var("ZED_SIMULATE_NO_LLM_PROVIDER").is_ok() {
            return None;
        }

        self.thread_summary_model
            .clone()
            .or_else(|| self.default_fast_model(cx))
            .or_else(|| self.default_model.clone())
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

        let provider = FakeLanguageModelProvider::default();
        registry.update(cx, |registry, cx| {
            registry.register_provider(provider.clone(), cx);
        });

        let providers = registry.read(cx).providers();
        assert_eq!(providers.len(), 1);
        assert_eq!(providers[0].id(), provider.id());

        registry.update(cx, |registry, cx| {
            registry.unregister_provider(provider.id(), cx);
        });

        let providers = registry.read(cx).providers();
        assert!(providers.is_empty());
    }

    #[gpui::test]
    async fn test_configure_environment_fallback_model(cx: &mut gpui::TestAppContext) {
        let registry = cx.new(|_| LanguageModelRegistry::default());

        let provider = FakeLanguageModelProvider::default();
        registry.update(cx, |registry, cx| {
            registry.register_provider(provider.clone(), cx);
        });

        cx.update(|cx| provider.authenticate(cx)).await.unwrap();

        registry.update(cx, |registry, cx| {
            let provider = registry.provider(&provider.id()).unwrap();

            registry.set_environment_fallback_model(
                Some(ConfiguredModel {
                    provider: provider.clone(),
                    model: provider.default_model(cx).unwrap(),
                }),
                cx,
            );

            let default_model = registry.default_model().unwrap();
            let fallback_model = registry.environment_fallback_model.clone().unwrap();

            assert_eq!(default_model.model.id(), fallback_model.model.id());
            assert_eq!(default_model.provider.id(), fallback_model.provider.id());
        });
    }
}
