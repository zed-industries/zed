use std::sync::Arc;

use client::{Client, UserStore};
use fs::Fs;
use gpui::{App, Context, Entity};
use language_model::{LanguageModelProviderId, LanguageModelRegistry, ZED_CLOUD_PROVIDER_ID};
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
pub use crate::settings::*;

pub fn init(user_store: Entity<UserStore>, client: Arc<Client>, fs: Arc<dyn Fs>, cx: &mut App) {
    crate::settings::init(fs, cx);
    let registry = LanguageModelRegistry::global(cx);
    registry.update(cx, |registry, cx| {
        register_language_model_providers(registry, user_store, client, cx);
    });
}

fn register_language_model_providers(
    registry: &mut LanguageModelRegistry,
    user_store: Entity<UserStore>,
    client: Arc<Client>,
    cx: &mut Context<LanguageModelRegistry>,
) {
    use feature_flags::FeatureFlagAppExt;

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
    registry.register_provider(CopilotChatLanguageModelProvider::new(cx), cx);

    cx.observe_flag::<feature_flags::LanguageModelsFeatureFlag, _>(move |enabled, cx| {
        let user_store = user_store.clone();
        let client = client.clone();
        LanguageModelRegistry::global(cx).update(cx, move |registry, cx| {
            if enabled {
                registry.register_provider(
                    CloudLanguageModelProvider::new(user_store.clone(), client.clone(), cx),
                    cx,
                );
            } else {
                registry.unregister_provider(
                    LanguageModelProviderId::from(ZED_CLOUD_PROVIDER_ID.to_string()),
                    cx,
                );
            }
        });
    })
    .detach();
}
