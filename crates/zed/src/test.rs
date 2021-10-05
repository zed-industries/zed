use crate::{
    assets::Assets,
    language,
    settings::{self, ThemeRegistry},
    AppState,
};
use buffer::LanguageRegistry;
use client::{http::ServerResponse, test::FakeHttpClient, ChannelList, Client, UserStore};
use gpui::MutableAppContext;
use parking_lot::Mutex;
use project::fs::FakeFs;
use std::sync::Arc;

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    env_logger::init();
}

pub fn test_app_state(cx: &mut MutableAppContext) -> Arc<AppState> {
    let (settings_tx, settings) = settings::test(cx);
    let mut languages = LanguageRegistry::new();
    languages.add(Arc::new(language::rust()));
    let themes = ThemeRegistry::new(Assets, cx.font_cache().clone());
    let client = Client::new();
    let http = FakeHttpClient::new(|_| async move { Ok(ServerResponse::new(404)) });
    let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http, cx));
    Arc::new(AppState {
        settings_tx: Arc::new(Mutex::new(settings_tx)),
        settings,
        themes,
        languages: Arc::new(languages),
        channel_list: cx.add_model(|cx| ChannelList::new(user_store.clone(), client.clone(), cx)),
        client,
        user_store,
        fs: Arc::new(FakeFs::new()),
    })
}
