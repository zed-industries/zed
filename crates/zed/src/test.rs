use crate::{assets::Assets, build_window_options, build_workspace, AppState};
use client::{test::FakeHttpClient, ChannelList, Client, UserStore};
use gpui::MutableAppContext;
use language::LanguageRegistry;
use project::fs::FakeFs;
use settings::Settings;
use std::sync::Arc;
use theme::ThemeRegistry;

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }
}

pub fn test_app_state(cx: &mut MutableAppContext) -> Arc<AppState> {
    let settings = Settings::test(cx);
    editor::init(cx);
    cx.set_global(settings);
    let themes = ThemeRegistry::new(Assets, cx.font_cache().clone());
    let http = FakeHttpClient::with_404_response();
    let client = Client::new(http.clone());
    let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http, cx));
    let languages = LanguageRegistry::test();
    languages.add(Arc::new(language::Language::new(
        language::LanguageConfig {
            name: "Rust".into(),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    )));
    Arc::new(AppState {
        themes,
        languages: Arc::new(languages),
        channel_list: cx.add_model(|cx| ChannelList::new(user_store.clone(), client.clone(), cx)),
        client,
        user_store,
        fs: FakeFs::new(cx.background().clone()),
        build_window_options: &build_window_options,
        build_workspace: &build_workspace,
    })
}
