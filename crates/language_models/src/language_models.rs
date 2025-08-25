use std::sync::Arc;

use ::settings::{Settings, SettingsStore};
use client::{Client, UserStore};
use collections::HashSet;
use futures::future;
use gpui::{App, AppContext as _, Context, Entity};
use language_model::{
    AuthenticateError, ConfiguredModel, LanguageModelProviderId, LanguageModelRegistry,
};
use project::DisableAiSettings;
use provider::deepseek::DeepSeekLanguageModelProvider;

pub mod provider;
mod settings;
pub mod ui;

use crate::provider::anthropic::AnthropicLanguageModelProvider;
use crate::provider::bedrock::BedrockLanguageModelProvider;
use crate::provider::cloud::{self, CloudLanguageModelProvider};
use crate::provider::copilot_chat::CopilotChatLanguageModelProvider;
use crate::provider::google::GoogleLanguageModelProvider;
use crate::provider::lmstudio::LmStudioLanguageModelProvider;
use crate::provider::mistral::MistralLanguageModelProvider;
use crate::provider::ollama::OllamaLanguageModelProvider;
use crate::provider::open_ai::OpenAiLanguageModelProvider;
use crate::provider::open_ai_compatible::OpenAiCompatibleLanguageModelProvider;
use crate::provider::open_router::OpenRouterLanguageModelProvider;
use crate::provider::vercel::VercelLanguageModelProvider;
use crate::provider::x_ai::XAiLanguageModelProvider;
pub use crate::settings::*;

pub fn init(user_store: Entity<UserStore>, client: Arc<Client>, cx: &mut App) {
    crate::settings::init_settings(cx);
    let registry = LanguageModelRegistry::global(cx);
    registry.update(cx, |registry, cx| {
        register_language_model_providers(registry, user_store, client.clone(), cx);
    });

    let mut openai_compatible_providers = AllLanguageModelSettings::get_global(cx)
        .openai_compatible
        .keys()
        .cloned()
        .collect::<HashSet<_>>();

    registry.update(cx, |registry, cx| {
        register_openai_compatible_providers(
            registry,
            &HashSet::default(),
            &openai_compatible_providers,
            client.clone(),
            cx,
        );
    });

    let mut already_authenticated = false;
    if !DisableAiSettings::get_global(cx).disable_ai {
        authenticate_all_providers(registry.clone(), cx);
        already_authenticated = true;
    }

    cx.observe_global::<SettingsStore>(move |cx| {
        let openai_compatible_providers_new = AllLanguageModelSettings::get_global(cx)
            .openai_compatible
            .keys()
            .cloned()
            .collect::<HashSet<_>>();
        if openai_compatible_providers_new != openai_compatible_providers {
            registry.update(cx, |registry, cx| {
                register_openai_compatible_providers(
                    registry,
                    &openai_compatible_providers,
                    &openai_compatible_providers_new,
                    client.clone(),
                    cx,
                );
            });
            openai_compatible_providers = openai_compatible_providers_new;
            already_authenticated = false;
        }

        if !DisableAiSettings::get_global(cx).disable_ai && !already_authenticated {
            authenticate_all_providers(registry.clone(), cx);
            already_authenticated = true;
        }
    })
    .detach();
}

fn register_openai_compatible_providers(
    registry: &mut LanguageModelRegistry,
    old: &HashSet<Arc<str>>,
    new: &HashSet<Arc<str>>,
    client: Arc<Client>,
    cx: &mut Context<LanguageModelRegistry>,
) {
    for provider_id in old {
        if !new.contains(provider_id) {
            registry.unregister_provider(LanguageModelProviderId::from(provider_id.clone()), cx);
        }
    }

    for provider_id in new {
        if !old.contains(provider_id) {
            registry.register_provider(
                OpenAiCompatibleLanguageModelProvider::new(
                    provider_id.clone(),
                    client.http_client(),
                    cx,
                ),
                cx,
            );
        }
    }
}

fn register_language_model_providers(
    registry: &mut LanguageModelRegistry,
    user_store: Entity<UserStore>,
    client: Arc<Client>,
    cx: &mut Context<LanguageModelRegistry>,
) {
    registry.register_provider(
        CloudLanguageModelProvider::new(user_store, client.clone(), cx),
        cx,
    );

    registry.register_provider(
        AnthropicLanguageModelProvider::new(client.http_client(), cx),
        cx,
    );
    registry.register_provider(
        OpenAiLanguageModelProvider::new(client.http_client(), cx),
        cx,
    );
    registry.register_provider(
        OllamaLanguageModelProvider::new(client.http_client(), cx),
        cx,
    );
    registry.register_provider(
        LmStudioLanguageModelProvider::new(client.http_client(), cx),
        cx,
    );
    registry.register_provider(
        DeepSeekLanguageModelProvider::new(client.http_client(), cx),
        cx,
    );
    registry.register_provider(
        GoogleLanguageModelProvider::new(client.http_client(), cx),
        cx,
    );
    registry.register_provider(
        MistralLanguageModelProvider::new(client.http_client(), cx),
        cx,
    );
    registry.register_provider(
        BedrockLanguageModelProvider::new(client.http_client(), cx),
        cx,
    );
    registry.register_provider(
        OpenRouterLanguageModelProvider::new(client.http_client(), cx),
        cx,
    );
    registry.register_provider(
        VercelLanguageModelProvider::new(client.http_client(), cx),
        cx,
    );
    registry.register_provider(XAiLanguageModelProvider::new(client.http_client(), cx), cx);
    registry.register_provider(CopilotChatLanguageModelProvider::new(cx), cx);
}

/// Authenticates all providers in the [`LanguageModelRegistry`].
///
/// We do this so that we can populate the language selector with all of the
/// models from the configured providers.
///
/// This function won't do anything if AI is disabled.
fn authenticate_all_providers(registry: Entity<LanguageModelRegistry>, cx: &mut App) {
    let providers_to_authenticate = registry
        .read(cx)
        .providers()
        .iter()
        .map(|provider| (provider.id(), provider.name(), provider.authenticate(cx)))
        .collect::<Vec<_>>();

    let mut tasks = Vec::with_capacity(providers_to_authenticate.len());

    for (provider_id, provider_name, authenticate_task) in providers_to_authenticate {
        tasks.push(cx.background_spawn(async move {
            if let Err(err) = authenticate_task.await {
                if matches!(err, AuthenticateError::CredentialsNotFound) {
                    // Since we're authenticating these providers in the
                    // background for the purposes of populating the
                    // language selector, we don't care about providers
                    // where the credentials are not found.
                } else {
                    // Some providers have noisy failure states that we
                    // don't want to spam the logs with every time the
                    // language model selector is initialized.
                    //
                    // Ideally these should have more clear failure modes
                    // that we know are safe to ignore here, like what we do
                    // with `CredentialsNotFound` above.
                    match provider_id.0.as_ref() {
                        "lmstudio" | "ollama" => {
                            // LM Studio and Ollama both make fetch requests to the local APIs to determine if they are "authenticated".
                            //
                            // These fail noisily, so we don't log them.
                        }
                        "copilot_chat" => {
                            // Copilot Chat returns an error if Copilot is not enabled, so we don't log those errors.
                        }
                        _ => {
                            log::error!(
                                "Failed to authenticate provider: {}: {err}",
                                provider_name.0
                            );
                        }
                    }
                }
            }
        }));
    }

    let all_authenticated_future = future::join_all(tasks);

    cx.spawn(async move |cx| {
        all_authenticated_future.await;

        registry
            .update(cx, |registry, cx| {
                let cloud_provider = registry.provider(&cloud::PROVIDER_ID);
                let fallback_model = cloud_provider
                    .iter()
                    .chain(registry.providers().iter())
                    .find(|provider| provider.is_authenticated(cx))
                    .and_then(|provider| {
                        Some(ConfiguredModel {
                            provider: provider.clone(),
                            model: provider
                                .default_model(cx)
                                .or_else(|| provider.recommended_models(cx).first().cloned())?,
                        })
                    });
                registry.set_environment_fallback_model(fallback_model, cx);
            })
            .ok();
    })
    .detach();
}
