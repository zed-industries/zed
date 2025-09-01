use std::sync::Arc;

use ::settings::{Settings, SettingsStore};
use client::{Client, UserStore};
use collections::HashSet;
use gpui::{App, Context, Entity};
use language_model::{LanguageModelProviderId, LanguageModelRegistry};
use provider::deepseek::DeepSeekLanguageModelProvider;

pub mod provider;
mod settings;
pub mod ui;

use crate::provider::anthropic::AnthropicLanguageModelProvider;
use crate::provider::bedrock::BedrockLanguageModelProvider;
use crate::provider::cloud::CloudLanguageModelProvider;
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
