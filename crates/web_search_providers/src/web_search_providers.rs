mod open_ai;

use client::Client;
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
    registry: &mut WebSearchRegistry,
    client: Arc<Client>,
    cx: &mut Context<WebSearchRegistry>,
) {
    registry.register_provider(
        open_ai::OpenAiWebSearchProvider::new(client.http_client()),
        cx,
    );
}
