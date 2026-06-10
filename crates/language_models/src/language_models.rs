use std::sync::Arc;

use ::settings::{Settings, SettingsStore};
use client::{Client, UserStore};
use collections::HashSet;
use credentials_provider::CredentialsProvider;
use gpui::{App, Context, Entity};
use language_model::{
    ConfiguredModel, LanguageModelProviderId, LanguageModelRegistry, ZED_CLOUD_PROVIDER_ID,
};
use provider::deepseek::DeepSeekLanguageModelProvider;

pub mod extension;
pub mod provider;
mod settings;

pub use crate::extension::init_proxy as init_extension_proxy;

use crate::provider::anthropic::AnthropicLanguageModelProvider;
use crate::provider::anthropic_compatible::AnthropicCompatibleLanguageModelProvider;
use crate::provider::bedrock::BedrockLanguageModelProvider;
use crate::provider::cloud::CloudLanguageModelProvider;
use crate::provider::copilot_chat::CopilotChatLanguageModelProvider;
use crate::provider::google::GoogleLanguageModelProvider;
use crate::provider::lmstudio::LmStudioLanguageModelProvider;
pub use crate::provider::mistral::MistralLanguageModelProvider;
use crate::provider::ollama::OllamaLanguageModelProvider;
use crate::provider::open_ai::OpenAiLanguageModelProvider;
use crate::provider::open_ai_compatible::OpenAiCompatibleLanguageModelProvider;
use crate::provider::open_router::OpenRouterLanguageModelProvider;
use crate::provider::openai_subscribed::OpenAiSubscribedProvider;
use crate::provider::opencode::OpenCodeLanguageModelProvider;
use crate::provider::vercel_ai_gateway::VercelAiGatewayLanguageModelProvider;
use crate::provider::x_ai::XAiLanguageModelProvider;
pub use crate::settings::*;

pub fn init(user_store: Entity<UserStore>, client: Arc<Client>, cx: &mut App) {
    let credentials_provider = client.credentials_provider();
    let registry = LanguageModelRegistry::global(cx);
    registry.update(cx, |registry, cx| {
        register_language_model_providers(
            registry,
            user_store,
            client.clone(),
            credentials_provider.clone(),
            cx,
        );
    });

    // Subscribe to extension store events to track LLM extension installations
    if let Some(extension_store) = extension_host::ExtensionStore::try_global(cx) {
        cx.subscribe(&extension_store, {
            let registry = registry.downgrade();
            move |extension_store, event, cx| {
                let Some(registry) = registry.upgrade() else {
                    return;
                };
                match event {
                    extension_host::Event::ExtensionInstalled(extension_id) => {
                        if let Some(manifest) = extension_store
                            .read(cx)
                            .extension_manifest_for_id(extension_id)
                        {
                            if !manifest.language_model_providers.is_empty() {
                                registry.update(cx, |registry, cx| {
                                    registry.extension_installed(extension_id.clone(), cx);
                                });
                            }
                        }
                    }
                    extension_host::Event::ExtensionUninstalled(extension_id) => {
                        registry.update(cx, |registry, cx| {
                            registry.extension_uninstalled(extension_id, cx);
                        });
                    }
                    extension_host::Event::ExtensionsUpdated => {
                        let mut new_ids = HashSet::default();
                        for (extension_id, entry) in extension_store.read(cx).installed_extensions()
                        {
                            if !entry.manifest.language_model_providers.is_empty() {
                                new_ids.insert(extension_id.clone());
                            }
                        }
                        registry.update(cx, |registry, cx| {
                            registry.sync_installed_llm_extensions(new_ids, cx);
                        });
                    }
                    _ => {}
                }
            }
        })
        .detach();

        // Initialize with currently installed extensions
        registry.update(cx, |registry, cx| {
            let mut initial_ids = HashSet::default();
            for (extension_id, entry) in extension_store.read(cx).installed_extensions() {
                if !entry.manifest.language_model_providers.is_empty() {
                    initial_ids.insert(extension_id.clone());
                }
            }
            registry.sync_installed_llm_extensions(initial_ids, cx);
        });
    }

    let mut compatible_providers = CompatibleProviders::from_settings(cx);

    registry.update(cx, |registry, cx| {
        register_compatible_providers(
            registry,
            &CompatibleProviders::default(),
            &compatible_providers,
            &client,
            &credentials_provider,
            cx,
        );
    });

    let registry = registry.downgrade();
    cx.observe_global::<SettingsStore>(move |cx| {
        let Some(registry) = registry.upgrade() else {
            return;
        };
        let compatible_providers_new = CompatibleProviders::from_settings(cx);
        if compatible_providers_new != compatible_providers {
            registry.update(cx, |registry, cx| {
                register_compatible_providers(
                    registry,
                    &compatible_providers,
                    &compatible_providers_new,
                    &client,
                    &credentials_provider,
                    cx,
                );
            });
            compatible_providers = compatible_providers_new;
        }
    })
    .detach();
}

/// Recomputes and sets the [`LanguageModelRegistry`]'s environment fallback
/// model based on currently authenticated providers.
///
/// Prefers the Zed cloud provider so that, once the user is signed in, we
/// always pick a Zed-hosted model over models from other authenticated
/// providers in the environment. If the Zed cloud provider is authenticated
/// but hasn't finished loading its models yet, we don't fall back to another
/// provider to avoid flickering between providers during sign in.
pub fn update_environment_fallback_model(cx: &mut App) {
    let registry = LanguageModelRegistry::global(cx);
    let fallback_model = {
        let registry = registry.read(cx);
        let cloud_provider = registry.provider(&ZED_CLOUD_PROVIDER_ID);
        if cloud_provider
            .as_ref()
            .is_some_and(|provider| provider.is_authenticated(cx))
        {
            cloud_provider.and_then(|provider| {
                let model = provider
                    .default_model(cx)
                    .or_else(|| provider.recommended_models(cx).first().cloned())?;
                Some(ConfiguredModel { provider, model })
            })
        } else {
            registry
                .providers()
                .iter()
                .filter(|provider| provider.is_authenticated(cx))
                .find_map(|provider| {
                    let model = provider
                        .default_model(cx)
                        .or_else(|| provider.recommended_models(cx).first().cloned())?;
                    Some(ConfiguredModel {
                        provider: provider.clone(),
                        model,
                    })
                })
        }
    };
    registry.update(cx, |registry, cx| {
        registry.set_environment_fallback_model(fallback_model, cx);
    });
}

#[derive(Default, PartialEq, Eq)]
struct CompatibleProviders {
    openai: HashSet<Arc<str>>,
    anthropic: HashSet<Arc<str>>,
}

impl CompatibleProviders {
    fn from_settings(cx: &App) -> Self {
        let settings = AllLanguageModelSettings::get_global(cx);
        Self {
            openai: settings.openai_compatible.keys().cloned().collect(),
            anthropic: settings.anthropic_compatible.keys().cloned().collect(),
        }
    }

    fn contains(&self, provider_id: &Arc<str>) -> bool {
        self.openai.contains(provider_id) || self.anthropic.contains(provider_id)
    }
}

fn register_compatible_providers(
    registry: &mut LanguageModelRegistry,
    old: &CompatibleProviders,
    new: &CompatibleProviders,
    client: &Arc<Client>,
    credentials_provider: &Arc<dyn CredentialsProvider>,
    cx: &mut Context<LanguageModelRegistry>,
) {
    for provider_id in old.openai.iter().chain(&old.anthropic) {
        if !new.contains(provider_id) {
            registry.unregister_provider(LanguageModelProviderId::from(provider_id.clone()), cx);
        }
    }

    for provider_id in &new.openai {
        if !old.openai.contains(provider_id) {
            registry.register_provider(
                Arc::new(OpenAiCompatibleLanguageModelProvider::new(
                    provider_id.clone(),
                    client.http_client(),
                    credentials_provider.clone(),
                    cx,
                )),
                cx,
            );
        }
    }

    for provider_id in &new.anthropic {
        if !old.anthropic.contains(provider_id) {
            registry.register_provider(
                Arc::new(AnthropicCompatibleLanguageModelProvider::new(
                    provider_id.clone(),
                    client.http_client(),
                    credentials_provider.clone(),
                    cx,
                )),
                cx,
            );
        }
    }
}

fn register_language_model_providers(
    registry: &mut LanguageModelRegistry,
    user_store: Entity<UserStore>,
    client: Arc<Client>,
    credentials_provider: Arc<dyn CredentialsProvider>,
    cx: &mut Context<LanguageModelRegistry>,
) {
    registry.register_provider(
        Arc::new(CloudLanguageModelProvider::new(
            user_store,
            client.clone(),
            cx,
        )),
        cx,
    );
    registry.register_provider(
        Arc::new(AnthropicLanguageModelProvider::new(
            client.http_client(),
            credentials_provider.clone(),
            cx,
        )),
        cx,
    );
    registry.register_provider(
        Arc::new(OpenAiLanguageModelProvider::new(
            client.http_client(),
            credentials_provider.clone(),
            cx,
        )),
        cx,
    );
    registry.register_provider(
        Arc::new(OllamaLanguageModelProvider::new(
            client.http_client(),
            credentials_provider.clone(),
            cx,
        )),
        cx,
    );
    registry.register_provider(
        Arc::new(LmStudioLanguageModelProvider::new(
            client.http_client(),
            credentials_provider.clone(),
            cx,
        )),
        cx,
    );
    registry.register_provider(
        Arc::new(DeepSeekLanguageModelProvider::new(
            client.http_client(),
            credentials_provider.clone(),
            cx,
        )),
        cx,
    );
    registry.register_provider(
        Arc::new(GoogleLanguageModelProvider::new(
            client.http_client(),
            credentials_provider.clone(),
            cx,
        )),
        cx,
    );
    registry.register_provider(
        MistralLanguageModelProvider::global(
            client.http_client(),
            credentials_provider.clone(),
            cx,
        ),
        cx,
    );
    registry.register_provider(
        Arc::new(BedrockLanguageModelProvider::new(
            client.http_client(),
            credentials_provider.clone(),
            cx,
        )),
        cx,
    );
    registry.register_provider(
        Arc::new(OpenRouterLanguageModelProvider::new(
            client.http_client(),
            credentials_provider.clone(),
            cx,
        )),
        cx,
    );
    registry.register_provider(
        Arc::new(VercelAiGatewayLanguageModelProvider::new(
            client.http_client(),
            credentials_provider.clone(),
            cx,
        )),
        cx,
    );
    registry.register_provider(
        Arc::new(XAiLanguageModelProvider::new(
            client.http_client(),
            credentials_provider.clone(),
            cx,
        )),
        cx,
    );
    registry.register_provider(
        Arc::new(OpenCodeLanguageModelProvider::new(
            client.http_client(),
            credentials_provider.clone(),
            cx,
        )),
        cx,
    );
    registry.register_provider(Arc::new(CopilotChatLanguageModelProvider::new(cx)), cx);
    registry.register_provider(
        Arc::new(OpenAiSubscribedProvider::new(
            client.http_client(),
            credentials_provider,
            cx,
        )),
        cx,
    );
}
