mod cloud;

use client::Client;
use gpui::{App, Context, Entity};
use language_model::LanguageModelRegistry;
use std::sync::Arc;
use web_search::{WebSearchProviderId, WebSearchRegistry};

pub fn init(client: Arc<Client>, cx: &mut App) {
    let registry = WebSearchRegistry::global(cx);
    registry.update(cx, |registry, cx| {
        register_web_search_providers(registry, client, cx);
    });
}

fn register_web_search_providers(
    registry: &mut WebSearchRegistry,
    client: Arc<Client>,
    cx: &mut Context<WebSearchRegistry>,
) {
    register_zed_web_search_provider(
        registry,
        client.clone(),
        &LanguageModelRegistry::global(cx),
        cx,
    );

    cx.subscribe(
        &LanguageModelRegistry::global(cx),
        move |this, registry, event, cx| match event {
            language_model::Event::DefaultModelChanged => {
                register_zed_web_search_provider(this, client.clone(), &registry, cx)
            }
            _ => {}
        },
    )
    .detach();
}

fn register_zed_web_search_provider(
    registry: &mut WebSearchRegistry,
    client: Arc<Client>,
    language_model_registry: &Entity<LanguageModelRegistry>,
    cx: &mut Context<WebSearchRegistry>,
) {
    let using_zed_provider = language_model_registry
        .read(cx)
        .default_model()
        .map_or(false, |default| default.is_provided_by_zed());
    if using_zed_provider {
        registry.register_provider(cloud::CloudWebSearchProvider::new(client, cx), cx)
    } else {
        registry.unregister_provider(WebSearchProviderId(
            cloud::ZED_WEB_SEARCH_PROVIDER_ID.into(),
        ));
    }
}
