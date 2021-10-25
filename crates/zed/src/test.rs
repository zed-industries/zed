use crate::{assets::Assets, AppState};
use client::{http::ServerResponse, test::FakeHttpClient, ChannelList, Client, UserStore};
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
    env_logger::init();
}

pub fn test_app_state(cx: &mut MutableAppContext) -> Arc<AppState> {
    let (settings_tx, settings) = watch::channel_with(build_settings(cx));
    let themes = ThemeRegistry::new(Assets, cx.font_cache().clone());
    let client = Client::new();
    let http = FakeHttpClient::new(|_| async move { Ok(ServerResponse::new(404)) });
    let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http, cx));
    Arc::new(AppState {
        settings_tx: Arc::new(Mutex::new(settings_tx)),
        settings,
        themes,
        languages: Arc::new(LanguageRegistry::new()),
        channel_list: cx.add_model(|cx| ChannelList::new(user_store.clone(), client.clone(), cx)),
        client,
        user_store,
        fs: Arc::new(FakeFs::new()),
    })
}

fn build_settings(cx: &gpui::AppContext) -> Settings {
    lazy_static::lazy_static! {
        static ref DEFAULT_THEME: parking_lot::Mutex<Option<Arc<Theme>>> = Default::default();
        static ref FONTS: Vec<Arc<Vec<u8>>> = Assets
            .list("fonts")
            .into_iter()
            .map(|f| Arc::new(Assets.load(&f).unwrap().to_vec()))
            .collect();
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

    Settings::new("Inconsolata", cx.font_cache(), theme).unwrap()
}
