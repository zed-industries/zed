use std::sync::Arc;

use client::{Client, UserStore};
use fs::Fs;
use gpui::{AppContext, Model, ModelContext};
use language_model::{LanguageModelProviderId, LanguageModelRegistry, ZED_CLOUD_PROVIDER_ID};

mod logging;
pub mod provider;
mod settings;

use crate::provider::anthropic::AnthropicLanguageModelProvider;
use crate::provider::cloud::CloudLanguageModelProvider;
pub use crate::provider::cloud::LlmApiToken;
pub use crate::provider::cloud::RefreshLlmTokenListener;
use crate::provider::copilot_chat::CopilotChatLanguageModelProvider;
use crate::provider::google::GoogleLanguageModelProvider;
use crate::provider::ollama::OllamaLanguageModelProvider;
use crate::provider::open_ai::OpenAiLanguageModelProvider;
pub use crate::settings::*;
pub use logging::report_assistant_event;

pub fn init(
    user_store: Model<UserStore>,
    client: Arc<Client>,
    fs: Arc<dyn Fs>,
    cx: &mut AppContext,
) {
    crate::settings::init(fs, cx);
    let registry = LanguageModelRegistry::global(cx);
    registry.update(cx, |registry, cx| {
        register_language_model_providers(registry, user_store, client, cx);
    });
}

fn register_language_model_providers(
    registry: &mut LanguageModelRegistry,
    user_store: Model<UserStore>,
    client: Arc<Client>,
    cx: &mut ModelContext<LanguageModelRegistry>,
) {
    use feature_flags::FeatureFlagAppExt;

    RefreshLlmTokenListener::register(client.clone(), cx);

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
        GoogleLanguageModelProvider::new(client.http_client(), cx),
        cx,
    );
    registry.register_provider(CopilotChatLanguageModelProvider::new(cx), cx);

    cx.observe_flag::<feature_flags::LanguageModels, _>(move |enabled, cx| {
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
