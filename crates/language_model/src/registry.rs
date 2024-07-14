use super::*;
use collections::HashMap;
use gpui::{AppContext, Global, Model};

#[derive(Default)]
pub struct LanguageModelRegistry {
    providers: HashMap<LanguageModelProviderName, Arc<dyn LanguageModelProvider>>,
}

impl Global for LanguageModelRegistry {}

impl LanguageModelRegistry {
    pub fn register_provider<T: LanguageModelProvider>(
        &mut self,
        provider: Model<T>,
        cx: &mut AppContext,
    ) {
        let name = provider.name(cx);

        self.providers.insert(name, Arc::new(provider));
    }

    pub fn available_models(&self, cx: &AppContext) -> Vec<AvailableLanguageModel> {
        self.providers
            .values()
            .flat_map(|provider| {
                provider
                    .provided_models(cx)
                    .into_iter()
                    .map(|model| AvailableLanguageModel {
                        provider: provider.name(cx),
                        model,
                    })
            })
            .collect()
    }

    pub fn model(
        &mut self,
        requested: &AvailableLanguageModel,
        cx: &mut AppContext,
    ) -> Result<Arc<dyn LanguageModel>> {
        let provider = self.providers.get(&requested.provider).ok_or_else(|| {
            anyhow::anyhow!("No provider found for name: {:?}", requested.provider)
        })?;

        provider.model(requested.model.id.clone(), cx)
    }

    pub fn provider(
        &self,
        name: &LanguageModelProviderName,
    ) -> Option<Arc<dyn LanguageModelProvider>> {
        self.providers.get(name).cloned()
    }
}
