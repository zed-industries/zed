use std::sync::Arc;

use client::{Client, UserStore};
use fs::Fs;
use gpui::{App, Context, Entity};
use language_model::LanguageModelRegistry;

pub mod provider;
mod settings;
pub mod ui;

use crate::provider::anthropic::AnthropicLanguageModelProvider;

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
    _user_store: Entity<UserStore>,
    client: Arc<Client>,
    cx: &mut Context<LanguageModelRegistry>,
) {
    registry.register_provider(
        AnthropicLanguageModelProvider::new(client.http_client(), cx),
        cx,
    );
}
