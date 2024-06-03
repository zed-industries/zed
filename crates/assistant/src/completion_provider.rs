mod anthropic;
#[cfg(test)]
mod fake;
mod open_ai;
mod zed;

pub use anthropic::*;
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
use std::time::Duration;

pub fn init(client: Arc<Client>, cx: &mut AppContext) {
    let mut settings_version = 0;
    let provider = match &AssistantSettings::get_global(cx).provider {
        AssistantProvider::ZedDotDev { model } => CompletionProvider::ZedDotDev(
            ZedDotDevCompletionProvider::new(model.clone(), client.clone(), settings_version, cx),
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
                    CompletionProvider::ZedDotDev(provider),
                    AssistantProvider::ZedDotDev { model },
                ) => {
                    provider.update(model.clone(), settings_version);
                }
                (_, AssistantProvider::ZedDotDev { model }) => {
                    *provider = CompletionProvider::ZedDotDev(ZedDotDevCompletionProvider::new(
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
            }
        })
    })
    .detach();
}

pub enum CompletionProvider {
    OpenAi(OpenAiCompletionProvider),
    Anthropic(AnthropicCompletionProvider),
    ZedDotDev(ZedDotDevCompletionProvider),
    #[cfg(test)]
    Fake(FakeCompletionProvider),
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
            CompletionProvider::ZedDotDev(provider) => provider
                .available_models()
                .map(LanguageModel::ZedDotDev)
                .collect(),
            #[cfg(test)]
            CompletionProvider::Fake(_) => unimplemented!(),
        }
    }

    pub fn settings_version(&self) -> usize {
        match self {
            CompletionProvider::OpenAi(provider) => provider.settings_version(),
            CompletionProvider::Anthropic(provider) => provider.settings_version(),
            CompletionProvider::ZedDotDev(provider) => provider.settings_version(),
            #[cfg(test)]
            CompletionProvider::Fake(_) => unimplemented!(),
        }
    }

    pub fn is_authenticated(&self) -> bool {
        match self {
            CompletionProvider::OpenAi(provider) => provider.is_authenticated(),
            CompletionProvider::Anthropic(provider) => provider.is_authenticated(),
            CompletionProvider::ZedDotDev(provider) => provider.is_authenticated(),
            #[cfg(test)]
            CompletionProvider::Fake(_) => true,
        }
    }

    pub fn authenticate(&self, cx: &AppContext) -> Task<Result<()>> {
        match self {
            CompletionProvider::OpenAi(provider) => provider.authenticate(cx),
            CompletionProvider::Anthropic(provider) => provider.authenticate(cx),
            CompletionProvider::ZedDotDev(provider) => provider.authenticate(cx),
            #[cfg(test)]
            CompletionProvider::Fake(_) => Task::ready(Ok(())),
        }
    }

    pub fn authentication_prompt(&self, cx: &mut WindowContext) -> AnyView {
        match self {
            CompletionProvider::OpenAi(provider) => provider.authentication_prompt(cx),
            CompletionProvider::Anthropic(provider) => provider.authentication_prompt(cx),
            CompletionProvider::ZedDotDev(provider) => provider.authentication_prompt(cx),
            #[cfg(test)]
            CompletionProvider::Fake(_) => unimplemented!(),
        }
    }

    pub fn reset_credentials(&self, cx: &AppContext) -> Task<Result<()>> {
        match self {
            CompletionProvider::OpenAi(provider) => provider.reset_credentials(cx),
            CompletionProvider::Anthropic(provider) => provider.reset_credentials(cx),
            CompletionProvider::ZedDotDev(_) => Task::ready(Ok(())),
            #[cfg(test)]
            CompletionProvider::Fake(_) => Task::ready(Ok(())),
        }
    }

    pub fn model(&self) -> LanguageModel {
        match self {
            CompletionProvider::OpenAi(provider) => LanguageModel::OpenAi(provider.model()),
            CompletionProvider::Anthropic(provider) => LanguageModel::Anthropic(provider.model()),
            CompletionProvider::ZedDotDev(provider) => LanguageModel::ZedDotDev(provider.model()),
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
            CompletionProvider::ZedDotDev(provider) => provider.count_tokens(request, cx),
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
            CompletionProvider::ZedDotDev(provider) => provider.complete(request),
            #[cfg(test)]
            CompletionProvider::Fake(provider) => provider.complete(),
        }
    }
}
