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
use gpui::{AppContext, Task};
use settings::{Settings, SettingsStore};
use std::sync::Arc;

pub fn init(client: Arc<Client>, cx: &mut AppContext) {
    let provider = match &AssistantSettings::get_global(cx).provider {
        AssistantProvider::ZedDotDev { default_model } => CompletionProvider::ZedDotDev(
            ZedDotDevCompletionProvider::new(default_model.clone(), client.clone(), cx),
        ),
        AssistantProvider::OpenAi {
            default_model,
            api_url,
        } => CompletionProvider::OpenAi(OpenAiCompletionProvider::new(
            default_model.clone(),
            api_url.clone(),
            client.http_client(),
        )),
    };
    cx.set_global(provider);

    cx.observe_global::<SettingsStore>(move |cx| {
        cx.update_global::<CompletionProvider, _>(|provider, cx| {
            match (&mut *provider, &AssistantSettings::get_global(cx).provider) {
                (
                    CompletionProvider::OpenAi(provider),
                    AssistantProvider::OpenAi {
                        default_model,
                        api_url,
                    },
                ) => {
                    provider.update(default_model.clone(), api_url.clone());
                }
                (
                    CompletionProvider::ZedDotDev(provider),
                    AssistantProvider::ZedDotDev { default_model },
                ) => {
                    provider.update(default_model.clone());
                }
                (CompletionProvider::OpenAi(_), AssistantProvider::ZedDotDev { default_model }) => {
                    *provider = CompletionProvider::ZedDotDev(ZedDotDevCompletionProvider::new(
                        default_model.clone(),
                        client.clone(),
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
    pub fn global<'a>(cx: &'a AppContext) -> &'a Self {
        cx.global::<Self>()
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
