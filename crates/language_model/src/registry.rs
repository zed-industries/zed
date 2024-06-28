use crate::{LanguageModel, LanguageModelId, LanguageModelName, LanguageModelProviderName};
use anyhow::Result;
use collections::HashMap;
use gpui::{AppContext, Global, Model, ModelContext};
use std::sync::Arc;

#[derive(Clone)]
pub struct ProvidedLanguageModel {
    pub id: LanguageModelId,
    pub name: LanguageModelName,
}

#[derive(Clone)]
struct AvailableLanguageModel {
    pub provider: LanguageModelProviderName,
    pub model: ProvidedLanguageModel,
}

pub trait LanguageModelProvider: 'static {
    fn name(&self, cx: &AppContext) -> LanguageModelProviderName;

    fn provided_models(&self, cx: &AppContext) -> Vec<ProvidedLanguageModel>;

    fn model(&self, id: LanguageModelId, cx: &AppContext) -> Result<Arc<dyn LanguageModel>>;
}

impl<T: LanguageModelProvider> LanguageModelProvider for Model<T> {
    fn name(&self, cx: &AppContext) -> LanguageModelProviderName {
        self.read(cx).name(cx)
    }

    fn provided_models(&self, cx: &AppContext) -> Vec<ProvidedLanguageModel> {
        self.read(cx).provided_models(cx)
    }

    fn model(&self, id: LanguageModelId, cx: &AppContext) -> Result<Arc<dyn LanguageModel>> {
        self.read(cx).model(id, cx)
    }
}

pub struct LanguageModelRegistry {
    providers: HashMap<LanguageModelProviderName, Box<dyn LanguageModelProvider>>,
}

impl Global for LanguageModelRegistry {}

impl LanguageModelRegistry {
    pub fn new() -> Self {
        Self {
            providers: HashMap::default(),
        }
    }

    pub fn register<T>(&mut self, provider: Model<T>, cx: &mut ModelContext<Self>)
    where
        T: LanguageModelProvider,
    {
        cx.observe(&provider, |_, _, cx| {
            cx.notify();
        })
        .detach();

        if self
            .providers
            .insert(provider.name(cx), Box::new(provider.clone()))
            .is_some()
        {
            panic!(
                "A provider with the name {} already exists",
                provider.name(cx).0
            );
        }
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
        info: &AvailableLanguageModel,
        cx: &mut AppContext,
    ) -> Result<Arc<dyn LanguageModel>> {
        let provider = self
            .providers
            .get(&info.provider)
            .ok_or_else(|| anyhow::anyhow!("No provider found for name: {:?}", info.provider))?;

        provider.model(info.model.id.clone(), cx)
    }
}
