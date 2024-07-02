mod anthropic;
mod cloud;
#[cfg(test)]
mod fake;
mod ollama;
mod open_ai;

pub use anthropic::*;
pub use cloud::*;
#[cfg(test)]
pub use fake::*;
pub use ollama::*;
pub use open_ai::*;

use crate::{
    assistant_settings::{AssistantProvider, AssistantSettings},
    LanguageModel, LanguageModelRequest,
};
use anyhow::Result;
use client::Client;
use futures::{future::BoxFuture, stream::BoxStream};
use gpui::{AnyView, AppContext, BorrowAppContext, Task, WindowContext};
use settings::{Settings, SettingsStore};
use std::time::Duration;
use std::{any::Any, sync::Arc};

/// Choose which model to use for openai provider.
/// If the model is not available, try to use the first available model, or fallback to the original model.
fn choose_openai_model(
    model: &::open_ai::Model,
    available_models: &[::open_ai::Model],
) -> ::open_ai::Model {
    available_models
        .iter()
        .find(|&m| m == model)
        .or_else(|| available_models.first())
        .unwrap_or_else(|| model)
        .clone()
}

pub fn init(client: Arc<Client>, cx: &mut AppContext) {
    let provider = create_provider_from_settings(client.clone(), 0, cx);
    cx.set_global(CompletionProvider {
        provider,
        client: Some(client),
    });

    let mut settings_version = 0;
    cx.observe_global::<SettingsStore>(move |cx| {
        settings_version += 1;
        cx.update_global::<CompletionProvider, _>(|provider, cx| {
            provider.update_settings(settings_version, cx);
        })
    })
    .detach();
}

pub trait LanguageModelCompletionProvider {
    fn available_models(&self, cx: &AppContext) -> Vec<LanguageModel>;
    fn settings_version(&self) -> usize;
    fn is_authenticated(&self) -> bool;
    fn authenticate(&self, cx: &AppContext) -> Task<Result<()>>;
    fn authentication_prompt(&self, cx: &mut WindowContext) -> AnyView;
    fn reset_credentials(&self, cx: &AppContext) -> Task<Result<()>>;
    fn model(&self) -> LanguageModel;
    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &AppContext,
    ) -> BoxFuture<'static, Result<usize>>;
    fn complete(
        &self,
        request: LanguageModelRequest,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>>;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct CompletionProvider {
    provider: Box<dyn LanguageModelCompletionProvider>,
    client: Option<Arc<Client>>,
}

impl CompletionProvider {
    pub fn available_models(&self, cx: &AppContext) -> Vec<LanguageModel> {
        self.provider.available_models(cx)
    }

    pub fn settings_version(&self) -> usize {
        self.provider.settings_version()
    }

    pub fn is_authenticated(&self) -> bool {
        self.provider.is_authenticated()
    }

    pub fn authenticate(&self, cx: &AppContext) -> Task<Result<()>> {
        self.provider.authenticate(cx)
    }

    pub fn authentication_prompt(&self, cx: &mut WindowContext) -> AnyView {
        self.provider.authentication_prompt(cx)
    }

    pub fn reset_credentials(&self, cx: &AppContext) -> Task<Result<()>> {
        self.provider.reset_credentials(cx)
    }

    pub fn model(&self) -> LanguageModel {
        self.provider.model()
    }

    pub fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &AppContext,
    ) -> BoxFuture<'static, Result<usize>> {
        self.provider.count_tokens(request, cx)
    }

    pub fn complete(
        &self,
        request: LanguageModelRequest,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        self.provider.complete(request)
    }
}

impl gpui::Global for CompletionProvider {}

impl CompletionProvider {
    pub fn global(cx: &AppContext) -> &Self {
        cx.global::<Self>()
    }

    pub fn current_provider_as<T: LanguageModelCompletionProvider + 'static>(
        &mut self,
    ) -> Option<&mut T> {
        self.provider.as_any_mut().downcast_mut::<T>()
    }

    pub fn update_settings(&mut self, version: usize, cx: &mut AppContext) {
        let result = match &AssistantSettings::get_global(cx).provider {
            AssistantProvider::ZedDotDev { model } => self
                .current_provider_as::<CloudCompletionProvider>()
                .map(|provider| provider.update(model.clone(), version)),
            AssistantProvider::OpenAi {
                model,
                api_url,
                low_speed_timeout_in_seconds,
                available_models,
            } => self
                .current_provider_as::<OpenAiCompletionProvider>()
                .map(|provider| {
                    provider.update(
                        choose_openai_model(&model, &available_models),
                        api_url.clone(),
                        low_speed_timeout_in_seconds.map(Duration::from_secs),
                        version,
                    )
                }),
            AssistantProvider::Anthropic {
                model,
                api_url,
                low_speed_timeout_in_seconds,
            } => self
                .current_provider_as::<AnthropicCompletionProvider>()
                .map(|provider| {
                    provider.update(
                        model.clone(),
                        api_url.clone(),
                        low_speed_timeout_in_seconds.map(Duration::from_secs),
                        version,
                    )
                }),
            AssistantProvider::Ollama {
                model,
                api_url,
                low_speed_timeout_in_seconds,
            } => self
                .current_provider_as::<OllamaCompletionProvider>()
                .map(|provider| {
                    provider.update(
                        model.clone(),
                        api_url.clone(),
                        low_speed_timeout_in_seconds.map(Duration::from_secs),
                        version,
                        cx,
                    )
                }),
        };

        // new providers
        if result.is_none() {
            if let Some(client) = self.client.clone() {
                self.provider = create_provider_from_settings(client, version, cx);
            } else {
                log::warn!("completion provider cannot be created because client is not set");
            }
        }
    }
}

fn create_provider_from_settings(
    client: Arc<Client>,
    settings_version: usize,
    cx: &mut AppContext,
) -> Box<dyn LanguageModelCompletionProvider> {
    match &AssistantSettings::get_global(cx).provider {
        AssistantProvider::ZedDotDev { model } => Box::new(CloudCompletionProvider::new(
            model.clone(),
            client.clone(),
            settings_version,
            cx,
        )),
        AssistantProvider::OpenAi {
            model,
            api_url,
            low_speed_timeout_in_seconds,
            available_models,
        } => Box::new(OpenAiCompletionProvider::new(
            choose_openai_model(&model, &available_models),
            api_url.clone(),
            client.http_client(),
            low_speed_timeout_in_seconds.map(Duration::from_secs),
            settings_version,
        )),
        AssistantProvider::Anthropic {
            model,
            api_url,
            low_speed_timeout_in_seconds,
        } => Box::new(AnthropicCompletionProvider::new(
            model.clone(),
            api_url.clone(),
            client.http_client(),
            low_speed_timeout_in_seconds.map(Duration::from_secs),
            settings_version,
        )),
        AssistantProvider::Ollama {
            model,
            api_url,
            low_speed_timeout_in_seconds,
        } => Box::new(OllamaCompletionProvider::new(
            model.clone(),
            api_url.clone(),
            client.http_client(),
            low_speed_timeout_in_seconds.map(Duration::from_secs),
            settings_version,
            cx,
        )),
    }
}

// pub enum CompletionProvider {
//     OpenAi(OpenAiCompletionProvider),
//     Anthropic(AnthropicCompletionProvider),
//     Cloud(CloudCompletionProvider),
//     #[cfg(test)]
//     Fake(FakeCompletionProvider),
//     Ollama(OllamaCompletionProvider),
// }

// impl gpui::Global for CompletionProvider {}

// impl CompletionProvider {
//     pub fn global(cx: &AppContext) -> &Self {
//         cx.global::<Self>()
//     }

//     pub fn available_models(&self, cx: &AppContext) -> Vec<LanguageModel> {
//         match self {
//             CompletionProvider::OpenAi(provider) => provider
//                 .available_models(cx)
//                 .map(LanguageModel::OpenAi)
//                 .collect(),
//             CompletionProvider::Anthropic(provider) => provider
//                 .available_models()
//                 .map(LanguageModel::Anthropic)
//                 .collect(),
//             CompletionProvider::Cloud(provider) => provider
//                 .available_models()
//                 .map(LanguageModel::Cloud)
//                 .collect(),
//             CompletionProvider::Ollama(provider) => provider
//                 .available_models()
//                 .map(|model| LanguageModel::Ollama(model.clone()))
//                 .collect(),
//             #[cfg(test)]
//             CompletionProvider::Fake(_) => unimplemented!(),
//         }
//     }

//     pub fn settings_version(&self) -> usize {
//         match self {
//             CompletionProvider::OpenAi(provider) => provider.settings_version(),
//             CompletionProvider::Anthropic(provider) => provider.settings_version(),
//             CompletionProvider::Cloud(provider) => provider.settings_version(),
//             CompletionProvider::Ollama(provider) => provider.settings_version(),
//             #[cfg(test)]
//             CompletionProvider::Fake(_) => unimplemented!(),
//         }
//     }

//     pub fn is_authenticated(&self) -> bool {
//         match self {
//             CompletionProvider::OpenAi(provider) => provider.is_authenticated(),
//             CompletionProvider::Anthropic(provider) => provider.is_authenticated(),
//             CompletionProvider::Cloud(provider) => provider.is_authenticated(),
//             CompletionProvider::Ollama(provider) => provider.is_authenticated(),
//             #[cfg(test)]
//             CompletionProvider::Fake(_) => true,
//         }
//     }

//     pub fn authenticate(&self, cx: &AppContext) -> Task<Result<()>> {
//         match self {
//             CompletionProvider::OpenAi(provider) => provider.authenticate(cx),
//             CompletionProvider::Anthropic(provider) => provider.authenticate(cx),
//             CompletionProvider::Cloud(provider) => provider.authenticate(cx),
//             CompletionProvider::Ollama(provider) => provider.authenticate(cx),
//             #[cfg(test)]
//             CompletionProvider::Fake(_) => Task::ready(Ok(())),
//         }
//     }

//     pub fn authentication_prompt(&self, cx: &mut WindowContext) -> AnyView {
//         match self {
//             CompletionProvider::OpenAi(provider) => provider.authentication_prompt(cx),
//             CompletionProvider::Anthropic(provider) => provider.authentication_prompt(cx),
//             CompletionProvider::Cloud(provider) => provider.authentication_prompt(cx),
//             CompletionProvider::Ollama(provider) => provider.authentication_prompt(cx),
//             #[cfg(test)]
//             CompletionProvider::Fake(_) => unimplemented!(),
//         }
//     }

//     pub fn reset_credentials(&self, cx: &AppContext) -> Task<Result<()>> {
//         match self {
//             CompletionProvider::OpenAi(provider) => provider.reset_credentials(cx),
//             CompletionProvider::Anthropic(provider) => provider.reset_credentials(cx),
//             CompletionProvider::Cloud(_) => Task::ready(Ok(())),
//             CompletionProvider::Ollama(provider) => provider.reset_credentials(cx),
//             #[cfg(test)]
//             CompletionProvider::Fake(_) => Task::ready(Ok(())),
//         }
//     }

//     pub fn model(&self) -> LanguageModel {
//         match self {
//             CompletionProvider::OpenAi(provider) => LanguageModel::OpenAi(provider.model()),
//             CompletionProvider::Anthropic(provider) => LanguageModel::Anthropic(provider.model()),
//             CompletionProvider::Cloud(provider) => LanguageModel::Cloud(provider.model()),
//             CompletionProvider::Ollama(provider) => LanguageModel::Ollama(provider.model()),
//             #[cfg(test)]
//             CompletionProvider::Fake(_) => LanguageModel::default(),
//         }
//     }

//     pub fn count_tokens(
//         &self,
//         request: LanguageModelRequest,
//         cx: &AppContext,
//     ) -> BoxFuture<'static, Result<usize>> {
//         match self {
//             CompletionProvider::OpenAi(provider) => provider.count_tokens(request, cx),
//             CompletionProvider::Anthropic(provider) => provider.count_tokens(request, cx),
//             CompletionProvider::Cloud(provider) => provider.count_tokens(request, cx),
//             CompletionProvider::Ollama(provider) => provider.count_tokens(request, cx),
//             #[cfg(test)]
//             CompletionProvider::Fake(_) => futures::FutureExt::boxed(futures::future::ready(Ok(0))),
//         }
//     }

//     pub fn complete(
//         &self,
//         request: LanguageModelRequest,
//     ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
//         match self {
//             CompletionProvider::OpenAi(provider) => provider.complete(request),
//             CompletionProvider::Anthropic(provider) => provider.complete(request),
//             CompletionProvider::Cloud(provider) => provider.complete(request),
//             CompletionProvider::Ollama(provider) => provider.complete(request),
//             #[cfg(test)]
//             CompletionProvider::Fake(provider) => provider.complete(),
//         }
//     }
// }
