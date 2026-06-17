use std::sync::Arc;

use ::settings::{Settings, SettingsStore};
use client::{Client, UserStore};
use collections::{HashMap, HashSet};
use credentials_provider::CredentialsProvider;
use gpui::{App, Context, Entity};
use language_model::{
    ConfiguredModel, LanguageModelProviderId, LanguageModelRegistry, ZED_CLOUD_PROVIDER_ID,
};
use provider::deepseek::DeepSeekLanguageModelProvider;

pub mod provider;
mod settings;

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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use clock::FakeSystemClock;
    use feature_flags::FeatureFlagAppExt as _;
    use gpui::{AppContext as _, AsyncApp, BorrowAppContext as _};
    use http_client::FakeHttpClient;
    use language_model::IconOrSvg;
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
