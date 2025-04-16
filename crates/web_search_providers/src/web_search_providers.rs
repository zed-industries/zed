mod cloud;

use client::Client;
use feature_flags::{FeatureFlagAppExt, ZedProWebSearchTool};
use gpui::{App, Context};
use std::sync::Arc;
use web_search::WebSearchRegistry;

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
    cx.observe_flag::<ZedProWebSearchTool, _>({
        let client = client.clone();
        move |is_enabled, cx| {
            if is_enabled {
                WebSearchRegistry::global(cx).update(cx, |registry, cx| {
                    registry.register_provider(
                        cloud::CloudWebSearchProvider::new(client.clone(), cx),
                        cx,
                    );
                });
            }
        }
    })
    .detach();
}
