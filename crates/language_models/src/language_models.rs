use std::sync::Arc;

use ::settings::{Settings, SettingsStore};
use client::{Client, UserStore};
use collections::HashSet;
use gpui::{App, Context, Entity};
use http_client::HttpClient;
use language_model::{
    LanguageModelProvider, LanguageModelProviderId, LanguageModelProviderState,
    LanguageModelRegistry,
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
use crate::provider::vercel::VercelLanguageModelProvider;
use crate::provider::vercel_ai_gateway::VercelAiGatewayLanguageModelProvider;
use crate::provider::x_ai::XAiLanguageModelProvider;
pub use crate::settings::*;

pub fn init(user_store: Entity<UserStore>, client: Arc<Client>, cx: &mut App) {
    let registry = LanguageModelRegistry::global(cx);
    registry.update(cx, |registry, cx| {
        register_language_model_providers(registry, user_store, client.clone(), cx);
    });

    // Subscribe to extension store events to track LLM extension installations
    if let Some(extension_store) = extension_host::ExtensionStore::try_global(cx) {
        cx.subscribe(&extension_store, {
            let registry = registry.clone();
            move |extension_store, event, cx| match event {
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
                    for (extension_id, entry) in extension_store.read(cx).installed_extensions() {
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

    let mut openai_compatible_provider_settings = AllLanguageModelSettings::get_global(cx)
        .openai_compatible
        .keys()
        .cloned()
        .collect::<HashSet<_>>();
    let mut anthropic_compatible_provider_settings = AllLanguageModelSettings::get_global(cx)
        .anthropic_compatible
        .keys()
        .cloned()
        .collect::<HashSet<_>>();
    let mut registered_openai_compatible_providers = HashSet::default();
    let mut registered_anthropic_compatible_providers = HashSet::default();

    registry.update(cx, |registry, cx| {
        registered_openai_compatible_providers = register_new_compatible_providers(
            registry,
            std::mem::take(&mut registered_openai_compatible_providers),
            &openai_compatible_provider_settings,
            &client,
            "OpenAI",
            OpenAiCompatibleLanguageModelProvider::new,
            cx,
        );
        registered_anthropic_compatible_providers = register_new_compatible_providers(
            registry,
            std::mem::take(&mut registered_anthropic_compatible_providers),
            &anthropic_compatible_provider_settings,
            &client,
            "Anthropic",
            AnthropicCompatibleLanguageModelProvider::new,
            cx,
        );
    });
    cx.observe_global::<SettingsStore>(move |cx| {
        let openai_compatible_provider_settings_new = AllLanguageModelSettings::get_global(cx)
            .openai_compatible
            .keys()
            .cloned()
            .collect::<HashSet<_>>();
        let anthropic_compatible_provider_settings_new = AllLanguageModelSettings::get_global(cx)
            .anthropic_compatible
            .keys()
            .cloned()
            .collect::<HashSet<_>>();

        if openai_compatible_provider_settings_new != openai_compatible_provider_settings
            || anthropic_compatible_provider_settings_new != anthropic_compatible_provider_settings
        {
            registry.update(cx, |registry, cx| {
                registered_openai_compatible_providers = unregister_removed_compatible_providers(
                    registry,
                    &registered_openai_compatible_providers,
                    &openai_compatible_provider_settings_new,
                    cx,
                );
                registered_anthropic_compatible_providers = unregister_removed_compatible_providers(
                    registry,
                    &registered_anthropic_compatible_providers,
                    &anthropic_compatible_provider_settings_new,
                    cx,
                );

                registered_openai_compatible_providers = register_new_compatible_providers(
                    registry,
                    std::mem::take(&mut registered_openai_compatible_providers),
                    &openai_compatible_provider_settings_new,
                    &client,
                    "OpenAI",
                    OpenAiCompatibleLanguageModelProvider::new,
                    cx,
                );
                registered_anthropic_compatible_providers = register_new_compatible_providers(
                    registry,
                    std::mem::take(&mut registered_anthropic_compatible_providers),
                    &anthropic_compatible_provider_settings_new,
                    &client,
                    "Anthropic",
                    AnthropicCompatibleLanguageModelProvider::new,
                    cx,
                );
            });
            openai_compatible_provider_settings = openai_compatible_provider_settings_new;
            anthropic_compatible_provider_settings = anthropic_compatible_provider_settings_new;
        }
    })
    .detach();
}

fn unregister_removed_compatible_providers(
    registry: &mut LanguageModelRegistry,
    registered: &HashSet<Arc<str>>,
    settings: &HashSet<Arc<str>>,
    cx: &mut Context<LanguageModelRegistry>,
) -> HashSet<Arc<str>> {
    let mut remaining = HashSet::default();
    for provider_id in registered {
        if settings.contains(provider_id) {
            remaining.insert(provider_id.clone());
        } else {
            registry.unregister_provider(LanguageModelProviderId::from(provider_id.clone()), cx);
        }
    }
    remaining
}

fn register_new_compatible_providers<T: LanguageModelProvider + LanguageModelProviderState>(
    registry: &mut LanguageModelRegistry,
    mut registered: HashSet<Arc<str>>,
    settings: &HashSet<Arc<str>>,
    client: &Arc<Client>,
    provider_kind: &'static str,
    new_provider: fn(Arc<str>, Arc<dyn HttpClient>, &mut App) -> T,
    cx: &mut Context<LanguageModelRegistry>,
) -> HashSet<Arc<str>> {
    for provider_id in settings {
        if registered.contains(provider_id) {
            continue;
        }

        let language_model_provider_id = LanguageModelProviderId::from(provider_id.clone());
        if registry.provider(&language_model_provider_id).is_some() {
            log::warn!(
                "Ignoring {provider_kind}-compatible provider `{provider_id}` because another language model provider is already registered with that id"
            );
            continue;
        }

        registry.register_provider(
            Arc::new(new_provider(provider_id.clone(), client.http_client(), cx)),
            cx,
        );
        registered.insert(provider_id.clone());
    }
    registered
}

fn register_language_model_providers(
    registry: &mut LanguageModelRegistry,
    user_store: Entity<UserStore>,
    client: Arc<Client>,
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
            cx,
        )),
        cx,
    );
    registry.register_provider(
        Arc::new(OpenAiLanguageModelProvider::new(client.http_client(), cx)),
        cx,
    );
    registry.register_provider(
        Arc::new(OllamaLanguageModelProvider::new(client.http_client(), cx)),
        cx,
    );
    registry.register_provider(
        Arc::new(LmStudioLanguageModelProvider::new(client.http_client(), cx)),
        cx,
    );
    registry.register_provider(
        Arc::new(DeepSeekLanguageModelProvider::new(client.http_client(), cx)),
        cx,
    );
    registry.register_provider(
        Arc::new(GoogleLanguageModelProvider::new(client.http_client(), cx)),
        cx,
    );
    registry.register_provider(
        MistralLanguageModelProvider::global(client.http_client(), cx),
        cx,
    );
    registry.register_provider(
        Arc::new(BedrockLanguageModelProvider::new(client.http_client(), cx)),
        cx,
    );
    registry.register_provider(
        Arc::new(OpenRouterLanguageModelProvider::new(
            client.http_client(),
            cx,
        )),
        cx,
    );
    registry.register_provider(
        Arc::new(VercelLanguageModelProvider::new(client.http_client(), cx)),
        cx,
    );
    registry.register_provider(
        Arc::new(VercelAiGatewayLanguageModelProvider::new(
            client.http_client(),
            cx,
        )),
        cx,
    );
    registry.register_provider(
        Arc::new(XAiLanguageModelProvider::new(client.http_client(), cx)),
        cx,
    );
    registry.register_provider(Arc::new(CopilotChatLanguageModelProvider::new(cx)), cx);
}
