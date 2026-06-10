use std::sync::Arc;

use ::settings::{Settings, SettingsStore};
use client::{Client, UserStore};
use collections::{HashMap, HashSet};
use gpui::{App, Context, Entity};
use language_model::{LanguageModelProviderId, LanguageModelRegistry};
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
    let built_in_provider_ids = registry.update(cx, |registry, cx| {
        register_language_model_providers(registry, user_store, client.clone(), cx);
        registry
            .providers()
            .into_iter()
            .map(|provider| provider.id())
            .collect::<HashSet<_>>()
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

    let mut compatible_provider_settings = CompatibleProviderSettings::global(cx);
    let mut registered_compatible_providers = HashMap::default();

    registry.update(cx, |registry, cx| {
        registered_compatible_providers = reconcile_compatible_providers(
            registry,
            std::mem::take(&mut registered_compatible_providers),
            &compatible_provider_settings,
            &built_in_provider_ids,
            &client,
            cx,
        );
    });
    cx.observe_global::<SettingsStore>(move |cx| {
        let compatible_provider_settings_new = CompatibleProviderSettings::global(cx);

        if compatible_provider_settings_new != compatible_provider_settings {
            registry.update(cx, |registry, cx| {
                registered_compatible_providers = reconcile_compatible_providers(
                    registry,
                    std::mem::take(&mut registered_compatible_providers),
                    &compatible_provider_settings_new,
                    &built_in_provider_ids,
                    &client,
                    cx,
                );
            });
            compatible_provider_settings = compatible_provider_settings_new;
        }
    })
    .detach();
}

#[derive(PartialEq, Eq)]
struct CompatibleProviderSettings {
    openai_compatible_provider_ids: HashSet<Arc<str>>,
    anthropic_compatible_provider_ids: HashSet<Arc<str>>,
}

impl CompatibleProviderSettings {
    fn global(cx: &App) -> Self {
        let settings = AllLanguageModelSettings::get_global(cx);
        Self {
            openai_compatible_provider_ids: settings.openai_compatible.keys().cloned().collect(),
            anthropic_compatible_provider_ids: settings
                .anthropic_compatible
                .keys()
                .cloned()
                .collect(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CompatibleProviderKind {
    OpenAi,
    Anthropic,
}

impl CompatibleProviderKind {
    fn name(self) -> &'static str {
        match self {
            Self::OpenAi => "OpenAI",
            Self::Anthropic => "Anthropic",
        }
    }

    fn register_provider(
        self,
        registry: &mut LanguageModelRegistry,
        provider_id: Arc<str>,
        client: &Arc<Client>,
        cx: &mut Context<LanguageModelRegistry>,
    ) {
        match self {
            Self::OpenAi => registry.register_provider(
                Arc::new(OpenAiCompatibleLanguageModelProvider::new(
                    provider_id,
                    client.http_client(),
                    cx,
                )),
                cx,
            ),
            Self::Anthropic => registry.register_provider(
                Arc::new(AnthropicCompatibleLanguageModelProvider::new(
                    provider_id,
                    client.http_client(),
                    cx,
                )),
                cx,
            ),
        }
    }
}

fn reconcile_compatible_providers(
    registry: &mut LanguageModelRegistry,
    registered: HashMap<Arc<str>, CompatibleProviderKind>,
    settings: &CompatibleProviderSettings,
    built_in_provider_ids: &HashSet<LanguageModelProviderId>,
    client: &Arc<Client>,
    cx: &mut Context<LanguageModelRegistry>,
) -> HashMap<Arc<str>, CompatibleProviderKind> {
    let desired = desired_compatible_providers(settings, built_in_provider_ids);

    for (provider_id, provider_kind) in &registered {
        if desired.get(provider_id) != Some(provider_kind) {
            registry.unregister_provider(LanguageModelProviderId::from(provider_id.clone()), cx);
        }
    }

    for (provider_id, provider_kind) in &desired {
        if registered.get(provider_id) == Some(provider_kind) {
            continue;
        }

        provider_kind.register_provider(registry, provider_id.clone(), client, cx);
    }

    desired
}

fn desired_compatible_providers(
    settings: &CompatibleProviderSettings,
    built_in_provider_ids: &HashSet<LanguageModelProviderId>,
) -> HashMap<Arc<str>, CompatibleProviderKind> {
    let mut desired = HashMap::default();
    insert_compatible_provider_settings(
        &mut desired,
        &settings.anthropic_compatible_provider_ids,
        CompatibleProviderKind::Anthropic,
        built_in_provider_ids,
    );
    insert_compatible_provider_settings(
        &mut desired,
        &settings.openai_compatible_provider_ids,
        CompatibleProviderKind::OpenAi,
        built_in_provider_ids,
    );
    desired
}

fn insert_compatible_provider_settings(
    desired: &mut HashMap<Arc<str>, CompatibleProviderKind>,
    provider_ids: &HashSet<Arc<str>>,
    provider_kind: CompatibleProviderKind,
    built_in_provider_ids: &HashSet<LanguageModelProviderId>,
) {
    for provider_id in provider_ids {
        let language_model_provider_id = LanguageModelProviderId::from(provider_id.clone());
        if built_in_provider_ids.contains(&language_model_provider_id) {
            log::warn!(
                "Ignoring {}-compatible provider `{provider_id}` because it conflicts with a built-in language model provider",
                provider_kind.name()
            );
            continue;
        }

        if let Some(previous_provider_kind) = desired.insert(provider_id.clone(), provider_kind) {
            log::warn!(
                "Using {}-compatible provider `{provider_id}` instead of {}-compatible provider with the same id",
                provider_kind.name(),
                previous_provider_kind.name()
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
