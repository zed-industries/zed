#[cfg(test)]
mod fake;
mod open_ai;
mod zed;

#[cfg(test)]
pub use fake::*;
pub use open_ai::*;
pub use zed::*;

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

pub fn init(client: Arc<Client>, cx: &mut AppContext) {
    let mut settings_version = 0;
    let provider = match &AssistantSettings::get_global(cx).provider {
        AssistantProvider::ZedDotDev { default_model } => {
            CompletionProvider::ZedDotDev(ZedDotDevCompletionProvider::new(
                default_model.clone(),
                client.clone(),
                settings_version,
                cx,
            ))
        }
        AssistantProvider::OpenAi {
            default_model,
            api_url,
        } => CompletionProvider::OpenAi(OpenAiCompletionProvider::new(
            default_model.clone(),
            api_url.clone(),
            client.http_client(),
            settings_version,
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
                        default_model,
                        api_url,
                    },
                ) => {
                    provider.update(default_model.clone(), api_url.clone(), settings_version);
                }
                (
                    CompletionProvider::ZedDotDev(provider),
                    AssistantProvider::ZedDotDev { default_model },
                ) => {
                    provider.update(default_model.clone(), settings_version);
                }
                (CompletionProvider::OpenAi(_), AssistantProvider::ZedDotDev { default_model }) => {
                    *provider = CompletionProvider::ZedDotDev(ZedDotDevCompletionProvider::new(
                        default_model.clone(),
                        client.clone(),
                        settings_version,
                        cx,
                    ));
                }
                (
                    CompletionProvider::ZedDotDev(_),
                    AssistantProvider::OpenAi {
                        default_model,
                        api_url,
                    },
                ) => {
                    *provider = CompletionProvider::OpenAi(OpenAiCompletionProvider::new(
                        default_model.clone(),
                        api_url.clone(),
                        client.http_client(),
                        settings_version,
                    ));
                }
                #[cfg(test)]
                (CompletionProvider::Fake(_), _) => unimplemented!(),
            }
        })
    })
    .detach();
}

pub enum CompletionProvider {
    OpenAi(OpenAiCompletionProvider),
    ZedDotDev(ZedDotDevCompletionProvider),
    #[cfg(test)]
    Fake(FakeCompletionProvider),
}

impl gpui::Global for CompletionProvider {}

impl CompletionProvider {
    pub fn global(cx: &AppContext) -> &Self {
        cx.global::<Self>()
    }

    pub fn settings_version(&self) -> usize {
        match self {
            CompletionProvider::OpenAi(provider) => provider.settings_version(),
            CompletionProvider::ZedDotDev(provider) => provider.settings_version(),
            #[cfg(test)]
            CompletionProvider::Fake(_) => unimplemented!(),
        }
    }

    pub fn is_authenticated(&self) -> bool {
        match self {
            CompletionProvider::OpenAi(provider) => provider.is_authenticated(),
            CompletionProvider::ZedDotDev(provider) => provider.is_authenticated(),
            #[cfg(test)]
            CompletionProvider::Fake(_) => true,
        }
    }

    pub fn authenticate(&self, cx: &AppContext) -> Task<Result<()>> {
        match self {
            CompletionProvider::OpenAi(provider) => provider.authenticate(cx),
            CompletionProvider::ZedDotDev(provider) => provider.authenticate(cx),
            #[cfg(test)]
            CompletionProvider::Fake(_) => Task::ready(Ok(())),
        }
    }

    pub fn authentication_prompt(&self, cx: &mut WindowContext) -> AnyView {
        match self {
            CompletionProvider::OpenAi(provider) => provider.authentication_prompt(cx),
            CompletionProvider::ZedDotDev(provider) => provider.authentication_prompt(cx),
            #[cfg(test)]
            CompletionProvider::Fake(_) => unimplemented!(),
        }
    }

    pub fn reset_credentials(&self, cx: &AppContext) -> Task<Result<()>> {
        match self {
            CompletionProvider::OpenAi(provider) => provider.reset_credentials(cx),
            CompletionProvider::ZedDotDev(_) => Task::ready(Ok(())),
            #[cfg(test)]
            CompletionProvider::Fake(_) => Task::ready(Ok(())),
        }
    }

    pub fn default_model(&self) -> LanguageModel {
        match self {
            CompletionProvider::OpenAi(provider) => LanguageModel::OpenAi(provider.default_model()),
            CompletionProvider::ZedDotDev(provider) => {
                LanguageModel::ZedDotDev(provider.default_model())
            }
            #[cfg(test)]
            CompletionProvider::Fake(_) => unimplemented!(),
        }
    }

    pub fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &AppContext,
    ) -> BoxFuture<'static, Result<usize>> {
        match self {
            CompletionProvider::OpenAi(provider) => provider.count_tokens(request, cx),
            CompletionProvider::ZedDotDev(provider) => provider.count_tokens(request, cx),
            #[cfg(test)]
            CompletionProvider::Fake(_) => unimplemented!(),
        }
    }

    pub fn complete(
        &self,
        request: LanguageModelRequest,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        match self {
            CompletionProvider::OpenAi(provider) => provider.complete(request),
            CompletionProvider::ZedDotDev(provider) => provider.complete(request),
            #[cfg(test)]
            CompletionProvider::Fake(provider) => provider.complete(),
        }
    }
}
