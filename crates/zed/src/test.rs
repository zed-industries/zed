use crate::{assets::Assets, build_window_options, build_workspace, AppState};
use client::{test::FakeHttpClient, ChannelList, Client, UserStore};
use gpui::{AssetSource, MutableAppContext};
use language::LanguageRegistry;
use parking_lot::Mutex;
use postage::watch;
use project::fs::FakeFs;
use std::sync::Arc;
use theme::{Theme, ThemeRegistry, DEFAULT_THEME_NAME};
use workspace::Settings;

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }
}

pub fn test_app_state(cx: &mut MutableAppContext) -> Arc<AppState> {
    let mut path_openers = Vec::new();
    editor::init(cx, &mut path_openers);
    let (settings_tx, settings) = watch::channel_with(build_settings(cx));
    let themes = ThemeRegistry::new(Assets, cx.font_cache().clone());
    let http = FakeHttpClient::with_404_response();
    let client = Client::new(http.clone());
    let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http, cx));
    let languages = LanguageRegistry::new();
    languages.add(Arc::new(language::Language::new(
        language::LanguageConfig {
            name: "Rust".into(),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    )));
    Arc::new(AppState {
        settings_tx: Arc::new(Mutex::new(settings_tx)),
        settings,
        themes,
        languages: Arc::new(languages),
        channel_list: cx.add_model(|cx| ChannelList::new(user_store.clone(), client.clone(), cx)),
        client,
        user_store,
        fs: FakeFs::new(cx.background().clone()),
        path_openers: Arc::from(path_openers),
        build_window_options: &build_window_options,
        build_workspace: &build_workspace,
    })
}

fn build_settings(cx: &gpui::AppContext) -> Settings {
    lazy_static::lazy_static! {
        static ref DEFAULT_THEME: parking_lot::Mutex<Option<Arc<Theme>>> = Default::default();
        static ref FONTS: Vec<Arc<Vec<u8>>> = vec![
            Assets.load("fonts/zed-sans/zed-sans-regular.ttf").unwrap().to_vec().into()
        ];
    }

    cx.platform().fonts().add_fonts(&FONTS).unwrap();

    let mut theme_guard = DEFAULT_THEME.lock();
    let theme = if let Some(theme) = theme_guard.as_ref() {
        theme.clone()
    } else {
        let theme = ThemeRegistry::new(Assets, cx.font_cache().clone())
            .get(DEFAULT_THEME_NAME)
            .expect("failed to load default theme in tests");
        *theme_guard = Some(theme.clone());
        theme
    };

    Settings::new("Zed Sans", cx.font_cache(), theme).unwrap()
}
