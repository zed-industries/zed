mod cloud;

use client::Client;
use gpui::{App, Context};
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
    _registry: &mut WebSearchRegistry,
    client: Arc<Client>,
    cx: &mut Context<WebSearchRegistry>,
) {
    cx.subscribe(
        &LanguageModelRegistry::global(cx),
        move |this, registry, event, cx| match event {
            language_model::Event::DefaultModelChanged => {
                let using_zed_provider = registry
                    .read(cx)
                    .default_model()
                    .map_or(false, |default| default.is_provided_by_zed());
                if using_zed_provider {
                    dbg!("Registered cloud web search provider");
                    this.register_provider(
                        cloud::CloudWebSearchProvider::new(client.clone(), cx),
                        cx,
                    )
                } else {
                    dbg!("Unregistered cloud web search provider");
                    this.unregister_provider(WebSearchProviderId(
                        cloud::ZED_WEB_SEARCH_PROVIDER_ID.into(),
                    ));
                }
            }
            _ => {}
        },
    )
    .detach();
}
