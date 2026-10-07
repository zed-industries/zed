use std::sync::Arc;

use ::settings::{Settings, SettingsStore};
use client::{Client, UserStore};
use collections::{HashMap, HashSet};
use credentials_provider::CredentialsProvider;
use futures::{channel::oneshot, future::Shared};
use gpui::{App, Context, Entity, Task};
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
use crate::provider::llama_cpp::LlamaCppLanguageModelProvider;
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
use crate::provider::x_ai_subscribed::XAiSubscribedProvider;
pub use crate::settings::*;

pub fn init(
    user_store: Entity<UserStore>,
    client: Arc<Client>,
    shell_env_loaded: Option<Shared<oneshot::Receiver<()>>>,
    cx: &mut App,
) {
    if let Some(shell_env_loaded) = shell_env_loaded {
        cx.spawn(async move |cx| {
            shell_env_loaded.await.ok();

            cx.update(|cx| init_registry(user_store, client, cx));
        })
        .detach();
    } else {
        init_registry(user_store, client, cx);
    }
}

fn init_registry(user_store: Entity<UserStore>, client: Arc<Client>, cx: &mut App) {
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

    authenticate_all_language_model_providers(cx).detach();
}

fn authenticate_all_language_model_providers(cx: &mut App) -> Task<()> {
    let authenticate_all_providers = LanguageModelRegistry::global(cx)
        .read(cx)
        .visible_providers()
        .iter()
        .map(|provider| (provider.id(), provider.name(), provider.authenticate(cx)))
        .collect::<Vec<_>>();

    cx.spawn(async move |cx| {
        for (provider_id, provider_name, authenticate_task) in authenticate_all_providers {
            if let Err(err) = authenticate_task.await {
                match err {
                    language_model::AuthenticateError::CredentialsNotFound => {
                        // Since we're authenticating these providers in the
                        // background for the purposes of populating the
                        // language selector, we don't care about providers
                        // where the credentials are not found.
                    }
                    language_model::AuthenticateError::ConnectionRefused => {
                        // Not logging connection refused errors as they are mostly from LM Studio's noisy auth failures.
                        // LM Studio only has one auth method (endpoint call) which fails for users who haven't enabled it.
                        // TODO: Better manage LM Studio auth logic to avoid these noisy failures.
                    }
                    _ => {
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
                                    "Failed to authenticate provider: {}: {err:#}",
                                    provider_name.0
                                );
                            }
                        }
                    }
                }
            }
        }

        cx.update(|cx| {
            LanguageModelRegistry::global(cx)
                .update(cx, |registry, cx| registry.refresh_fallback_model(cx))
        });
    })
}

#[derive(Default, PartialEq, Eq)]
struct CompatibleProviders(HashMap<Arc<str>, CompatibleProviderKind>);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum CompatibleProviderKind {
    OpenAi,
    Anthropic,
}

impl CompatibleProviders {
    fn from_settings(cx: &App) -> Self {
        let settings = AllLanguageModelSettings::get_global(cx);
        let mut providers: HashMap<Arc<str>, CompatibleProviderKind> = settings
            .openai_compatible
            .keys()
            .map(|id| (id.clone(), CompatibleProviderKind::OpenAi))
            .collect();
        for id in settings.anthropic_compatible.keys() {
            // The registry has a single provider ID namespace, so a name can
            // only refer to one provider. OpenAI-compatible entries win
            // collisions because they predate Anthropic-compatible ones, so
            // existing configurations keep working.
            if providers.contains_key(id) {
                log::warn!(
                    "ignoring `anthropic_compatible` provider `{id}`: \
                     an `openai_compatible` provider with the same name exists"
                );
            } else {
                providers.insert(id.clone(), CompatibleProviderKind::Anthropic);
            }
        }
        Self(providers)
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
    for (provider_id, old_kind) in &old.0 {
        if new.0.get(provider_id) != Some(old_kind) {
            registry.unregister_provider(LanguageModelProviderId::from(provider_id.clone()), cx);
        }
    }

    for (provider_id, kind) in &new.0 {
        if old.0.get(provider_id) != Some(kind) {
            match kind {
                CompatibleProviderKind::OpenAi => registry.register_provider(
                    Arc::new(OpenAiCompatibleLanguageModelProvider::new(
                        provider_id.clone(),
                        client.http_client(),
                        credentials_provider.clone(),
                        cx,
                    )),
                    cx,
                ),
                CompatibleProviderKind::Anthropic => registry.register_provider(
                    Arc::new(AnthropicCompatibleLanguageModelProvider::new(
                        provider_id.clone(),
                        client.http_client(),
                        credentials_provider.clone(),
                        cx,
                    )),
                    cx,
                ),
            }
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
        Arc::new(LlamaCppLanguageModelProvider::new(
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
        Arc::new(XAiSubscribedProvider::new(
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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use client::RefreshLlmTokenListener;
    use clock::FakeSystemClock;
    use feature_flags::FeatureFlagAppExt as _;
    use futures::FutureExt as _;
    use gpui::{AppContext as _, AsyncApp, BorrowAppContext as _, TestAppContext};
    use http_client::FakeHttpClient;
    use language_model::{
        IconOrSvg, LanguageModelProviderName, OPEN_AI_PROVIDER_ID,
        fake_provider::FakeLanguageModelProvider,
    };
    use release_channel::AppVersion;
    use std::future::Future;
    use std::pin::Pin;
    use ui::IconName;

    struct FakeCredentialsProvider;

    impl CredentialsProvider for FakeCredentialsProvider {
        fn read_credentials<'a>(
            &'a self,
            _url: &'a str,
            _cx: &'a AsyncApp,
        ) -> Pin<Box<dyn Future<Output = Result<Option<(String, Vec<u8>)>>> + 'a>> {
            Box::pin(async { Ok(None) })
        }

        fn write_credentials<'a>(
            &'a self,
            _url: &'a str,
            _username: &'a str,
            _password: &'a [u8],
            _cx: &'a AsyncApp,
        ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>> {
            Box::pin(async { Ok(()) })
        }

        fn delete_credentials<'a>(
            &'a self,
            _url: &'a str,
            _cx: &'a AsyncApp,
        ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>> {
            Box::pin(async { Ok(()) })
        }
    }

    fn init_test(cx: &mut App) -> (Arc<Client>, Arc<dyn CredentialsProvider>) {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        cx.set_global(db::AppDatabase::test_new());
        let app_version = AppVersion::global(cx);
        release_channel::init_test(app_version, release_channel::ReleaseChannel::Dev, cx);
        gpui_tokio::init(cx);
        cx.update_flags(false, Vec::new());

        let client = Client::new(
            Arc::new(FakeSystemClock::new()),
            FakeHttpClient::with_404_response(),
            cx,
        );
        (client, Arc::new(FakeCredentialsProvider))
    }

    fn update_compatible_provider_settings(
        openai: &[&str],
        anthropic: &[&str],
        cx: &mut App,
    ) -> CompatibleProviders {
        fn section(ids: &[&str]) -> serde_json::Value {
            ids.iter()
                .map(|id| {
                    (
                        id.to_string(),
                        serde_json::json!({
                            "api_url": "https://example.com",
                            "available_models": [],
                        }),
                    )
                })
                .collect::<serde_json::Map<String, serde_json::Value>>()
                .into()
        }

        let content = serde_json::json!({
            "language_models": {
                "openai_compatible": section(openai),
                "anthropic_compatible": section(anthropic),
            }
        })
        .to_string();
        cx.update_global::<SettingsStore, _>(|store, cx| {
            store
                .set_user_settings(&content, cx)
                .expect("failed to parse test settings");
        });
        CompatibleProviders::from_settings(cx)
    }

    fn provider_icons(registry: &LanguageModelRegistry, id: &str) -> Vec<IconOrSvg> {
        registry
            .providers()
            .into_iter()
            .filter(|provider| provider.id().0.as_ref() == id)
            .map(|provider| provider.icon())
            .collect()
    }

    #[gpui::test]
    async fn test_waits_for_shell_environment_before_initializing(cx: &mut TestAppContext) {
        let (client, user_store) = cx.update(|cx| {
            let (client, _) = init_test(cx);
            language_model::init(cx);
            <dyn fs::Fs>::set_global(fs::FakeFs::new(cx.background_executor().clone()), cx);
            let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
            RefreshLlmTokenListener::register(client.clone(), user_store.clone(), cx);
            (client, user_store)
        });
        let provider = Arc::new(FakeLanguageModelProvider::new(
            LanguageModelProviderId::from("test-provider".to_string()),
            LanguageModelProviderName::from("Test Provider".to_string()),
        ));
        let (shell_env_loaded_tx, shell_env_loaded_rx) = oneshot::channel();
        cx.update(|cx| {
            init(user_store, client, Some(shell_env_loaded_rx.shared()), cx);
            LanguageModelRegistry::global(cx).update(cx, |registry, cx| {
                registry.register_provider(provider.clone(), cx);
            });
        });
        cx.run_until_parked();

        assert_eq!(provider.authentication_count(), 0);
        assert!(cx.read(|cx| {
            LanguageModelRegistry::global(cx)
                .read(cx)
                .provider(&OPEN_AI_PROVIDER_ID)
                .is_none()
        }));

        shell_env_loaded_tx.send(()).unwrap();
        cx.run_until_parked();

        assert_eq!(provider.authentication_count(), 1);
        assert!(cx.read(|cx| {
            LanguageModelRegistry::global(cx)
                .read(cx)
                .provider(&OPEN_AI_PROVIDER_ID)
                .is_some()
        }));
    }

    #[gpui::test]
    fn test_compatible_provider_id_collision_resolves_when_one_entry_is_removed(cx: &mut App) {
        let (client, credentials_provider) = init_test(cx);
        let registry = cx.new(|_| LanguageModelRegistry::default());

        // The same provider name is configured in both `openai_compatible`
        // and `anthropic_compatible` settings sections; the OpenAI-compatible
        // entry wins the collision.
        let both = update_compatible_provider_settings(&["acme"], &["acme"], cx);
        registry.update(cx, |registry, cx| {
            register_compatible_providers(
                registry,
                &CompatibleProviders::default(),
                &both,
                &client,
                &credentials_provider,
                cx,
            );
        });
        assert_eq!(
            registry.read_with(cx, |registry, _| provider_icons(registry, "acme")),
            vec![IconOrSvg::Icon(IconName::AiOpenAiCompat)],
            "the OpenAI-compatible provider should win the name collision"
        );

        // The user removes the `anthropic_compatible` entry; the remaining
        // `openai_compatible` entry must stay registered.
        let openai_only = update_compatible_provider_settings(&["acme"], &[], cx);
        registry.update(cx, |registry, cx| {
            register_compatible_providers(
                registry,
                &both,
                &openai_only,
                &client,
                &credentials_provider,
                cx,
            );
        });
        assert_eq!(
            registry.read_with(cx, |registry, _| provider_icons(registry, "acme")),
            vec![IconOrSvg::Icon(IconName::AiOpenAiCompat)],
            "the provider registered for `acme` should be the OpenAI-compatible one"
        );
    }

    #[gpui::test]
    fn test_compatible_provider_changes_kind_and_unregisters(cx: &mut App) {
        let (client, credentials_provider) = init_test(cx);
        let registry = cx.new(|_| LanguageModelRegistry::default());

        let both = update_compatible_provider_settings(&["acme"], &["acme"], cx);
        registry.update(cx, |registry, cx| {
            register_compatible_providers(
                registry,
                &CompatibleProviders::default(),
                &both,
                &client,
                &credentials_provider,
                cx,
            );
        });

        // Removing the `openai_compatible` entry hands the name over to the
        // remaining `anthropic_compatible` entry.
        let anthropic_only = update_compatible_provider_settings(&[], &["acme"], cx);
        registry.update(cx, |registry, cx| {
            register_compatible_providers(
                registry,
                &both,
                &anthropic_only,
                &client,
                &credentials_provider,
                cx,
            );
        });
        assert_eq!(
            registry.read_with(cx, |registry, _| provider_icons(registry, "acme")),
            vec![IconOrSvg::Icon(IconName::AiAnthropicCompat)],
            "after removing the openai_compatible entry, the anthropic_compatible provider should be registered"
        );

        // Removing the last entry unregisters the provider entirely.
        let none = update_compatible_provider_settings(&[], &[], cx);
        registry.update(cx, |registry, cx| {
            register_compatible_providers(
                registry,
                &anthropic_only,
                &none,
                &client,
                &credentials_provider,
                cx,
            );
        });
        assert_eq!(
            registry.read_with(cx, |registry, _| provider_icons(registry, "acme")),
            Vec::new(),
            "removing all entries should unregister the provider"
        );
    }
}
