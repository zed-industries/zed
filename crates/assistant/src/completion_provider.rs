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
use std::sync::Arc;
use std::time::Duration;

pub fn init(client: Arc<Client>, cx: &mut AppContext) {
    let mut settings_version = 0;
    let provider = match &AssistantSettings::get_global(cx).provider {
        AssistantProvider::ZedDotDev { model } => CompletionProvider::Cloud(
            CloudCompletionProvider::new(model.clone(), client.clone(), settings_version, cx),
        ),
        AssistantProvider::OpenAi {
            model,
            api_url,
            low_speed_timeout_in_seconds,
        } => CompletionProvider::OpenAi(OpenAiCompletionProvider::new(
            model.clone(),
            api_url.clone(),
            client.http_client(),
            low_speed_timeout_in_seconds.map(Duration::from_secs),
            settings_version,
        )),
        AssistantProvider::Anthropic {
            model,
            api_url,
            low_speed_timeout_in_seconds,
        } => CompletionProvider::Anthropic(AnthropicCompletionProvider::new(
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
        } => CompletionProvider::Ollama(OllamaCompletionProvider::new(
            model.clone(),
            api_url.clone(),
            client.http_client(),
            low_speed_timeout_in_seconds.map(Duration::from_secs),
            settings_version,
            cx,
        )),
    };
    cx.set_global(provider);

    cx.observe_global::<SettingsStore>(move |cx| {
        settings_version += 1;
        cx.update_global::<CompletionProvider, _>(|provider, cx| {
            match (&mut *provider, &AssistantSettings::get_global(cx).provider) {
                (
                    CompletionProvider::OpenAi(provider),
                    AssistantProvider::OpenAi {
                        model,
                        api_url,
                        low_speed_timeout_in_seconds,
                    },
                ) => {
                    provider.update(
                        model.clone(),
                        api_url.clone(),
                        low_speed_timeout_in_seconds.map(Duration::from_secs),
                        settings_version,
                    );
                }
                (
                    CompletionProvider::Anthropic(provider),
                    AssistantProvider::Anthropic {
                        model,
                        api_url,
                        low_speed_timeout_in_seconds,
                    },
                ) => {
                    provider.update(
                        model.clone(),
                        api_url.clone(),
                        low_speed_timeout_in_seconds.map(Duration::from_secs),
                        settings_version,
                    );
                }

                (
                    CompletionProvider::Ollama(provider),
                    AssistantProvider::Ollama {
                        model,
                        api_url,
                        low_speed_timeout_in_seconds,
                    },
                ) => {
                    provider.update(
                        model.clone(),
                        api_url.clone(),
                        low_speed_timeout_in_seconds.map(Duration::from_secs),
                        settings_version,
                        cx,
                    );
                }

                (CompletionProvider::Cloud(provider), AssistantProvider::ZedDotDev { model }) => {
                    provider.update(model.clone(), settings_version);
                }
                (_, AssistantProvider::ZedDotDev { model }) => {
                    *provider = CompletionProvider::Cloud(CloudCompletionProvider::new(
                        model.clone(),
                        client.clone(),
                        settings_version,
                        cx,
                    ));
                }
                (
                    _,
                    AssistantProvider::OpenAi {
                        model,
                        api_url,
                        low_speed_timeout_in_seconds,
                    },
                ) => {
                    *provider = CompletionProvider::OpenAi(OpenAiCompletionProvider::new(
                        model.clone(),
                        api_url.clone(),
                        client.http_client(),
                        low_speed_timeout_in_seconds.map(Duration::from_secs),
                        settings_version,
                    ));
                }
                (
                    _,
                    AssistantProvider::Anthropic {
                        model,
                        api_url,
                        low_speed_timeout_in_seconds,
                    },
                ) => {
                    *provider = CompletionProvider::Anthropic(AnthropicCompletionProvider::new(
                        model.clone(),
                        api_url.clone(),
                        client.http_client(),
                        low_speed_timeout_in_seconds.map(Duration::from_secs),
                        settings_version,
                    ));
                }
                (
                    _,
                    AssistantProvider::Ollama {
                        model,
                        api_url,
                        low_speed_timeout_in_seconds,
                    },
                ) => {
                    *provider = CompletionProvider::Ollama(OllamaCompletionProvider::new(
                        model.clone(),
                        api_url.clone(),
                        client.http_client(),
                        low_speed_timeout_in_seconds.map(Duration::from_secs),
                        settings_version,
                        cx,
                    ));
                }
            }
        })
    })
    .detach();
}

pub enum CompletionProvider {
    OpenAi(OpenAiCompletionProvider),
    Anthropic(AnthropicCompletionProvider),
    Cloud(CloudCompletionProvider),
    #[cfg(test)]
    Fake(FakeCompletionProvider),
    Ollama(OllamaCompletionProvider),
}

impl gpui::Global for CompletionProvider {}

impl CompletionProvider {
    pub fn global(cx: &AppContext) -> &Self {
        cx.global::<Self>()
    }

    pub fn available_models(&self) -> Vec<LanguageModel> {
        match self {
            CompletionProvider::OpenAi(provider) => provider
                .available_models()
                .map(LanguageModel::OpenAi)
                .collect(),
            CompletionProvider::Anthropic(provider) => provider
                .available_models()
                .map(LanguageModel::Anthropic)
                .collect(),
            CompletionProvider::Cloud(provider) => provider
                .available_models()
                .map(LanguageModel::Cloud)
                .collect(),
            CompletionProvider::Ollama(provider) => provider
                .available_models()
                .map(|model| LanguageModel::Ollama(model.clone()))
                .collect(),
            #[cfg(test)]
            CompletionProvider::Fake(_) => unimplemented!(),
        }
    }

    pub fn settings_version(&self) -> usize {
        match self {
            CompletionProvider::OpenAi(provider) => provider.settings_version(),
            CompletionProvider::Anthropic(provider) => provider.settings_version(),
            CompletionProvider::Cloud(provider) => provider.settings_version(),
            CompletionProvider::Ollama(provider) => provider.settings_version(),
            #[cfg(test)]
            CompletionProvider::Fake(_) => unimplemented!(),
        }
    }

    pub fn is_authenticated(&self) -> bool {
        match self {
            CompletionProvider::OpenAi(provider) => provider.is_authenticated(),
            CompletionProvider::Anthropic(provider) => provider.is_authenticated(),
            CompletionProvider::Cloud(provider) => provider.is_authenticated(),
            CompletionProvider::Ollama(provider) => provider.is_authenticated(),
            #[cfg(test)]
            CompletionProvider::Fake(_) => true,
        }
    }

    pub fn authenticate(&self, cx: &AppContext) -> Task<Result<()>> {
        match self {
            CompletionProvider::OpenAi(provider) => provider.authenticate(cx),
            CompletionProvider::Anthropic(provider) => provider.authenticate(cx),
            CompletionProvider::Cloud(provider) => provider.authenticate(cx),
            CompletionProvider::Ollama(provider) => provider.authenticate(cx),
            #[cfg(test)]
            CompletionProvider::Fake(_) => Task::ready(Ok(())),
        }
    }

    pub fn authentication_prompt(&self, cx: &mut WindowContext) -> AnyView {
        match self {
            CompletionProvider::OpenAi(provider) => provider.authentication_prompt(cx),
            CompletionProvider::Anthropic(provider) => provider.authentication_prompt(cx),
            CompletionProvider::Cloud(provider) => provider.authentication_prompt(cx),
            CompletionProvider::Ollama(provider) => provider.authentication_prompt(cx),
            #[cfg(test)]
            CompletionProvider::Fake(_) => unimplemented!(),
        }
    }

    pub fn reset_credentials(&self, cx: &AppContext) -> Task<Result<()>> {
        match self {
            CompletionProvider::OpenAi(provider) => provider.reset_credentials(cx),
            CompletionProvider::Anthropic(provider) => provider.reset_credentials(cx),
            CompletionProvider::Cloud(_) => Task::ready(Ok(())),
            CompletionProvider::Ollama(provider) => provider.reset_credentials(cx),
            #[cfg(test)]
            CompletionProvider::Fake(_) => Task::ready(Ok(())),
        }
    }

    pub fn model(&self) -> LanguageModel {
        match self {
            CompletionProvider::OpenAi(provider) => LanguageModel::OpenAi(provider.model()),
            CompletionProvider::Anthropic(provider) => LanguageModel::Anthropic(provider.model()),
            CompletionProvider::Cloud(provider) => LanguageModel::Cloud(provider.model()),
            CompletionProvider::Ollama(provider) => LanguageModel::Ollama(provider.model()),
            #[cfg(test)]
            CompletionProvider::Fake(_) => LanguageModel::default(),
        }
    }

    pub fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &AppContext,
    ) -> BoxFuture<'static, Result<usize>> {
        match self {
            CompletionProvider::OpenAi(provider) => provider.count_tokens(request, cx),
            CompletionProvider::Anthropic(provider) => provider.count_tokens(request, cx),
            CompletionProvider::Cloud(provider) => provider.count_tokens(request, cx),
            CompletionProvider::Ollama(provider) => provider.count_tokens(request, cx),
            #[cfg(test)]
            CompletionProvider::Fake(_) => futures::FutureExt::boxed(futures::future::ready(Ok(0))),
        }
    }

    pub fn complete(
        &self,
        request: LanguageModelRequest,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        match self {
            CompletionProvider::OpenAi(provider) => provider.complete(request),
            CompletionProvider::Anthropic(provider) => provider.complete(request),
            CompletionProvider::Cloud(provider) => provider.complete(request),
            CompletionProvider::Ollama(provider) => provider.complete(request),
            #[cfg(test)]
            CompletionProvider::Fake(provider) => provider.complete(),
        }
    }
}
