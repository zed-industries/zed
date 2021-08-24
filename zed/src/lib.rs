pub mod assets;
pub mod channel;
pub mod chat_panel;
pub mod editor;
pub mod file_finder;
pub mod fs;
mod fuzzy;
pub mod language;
pub mod menus;
pub mod project_browser;
pub mod rpc;
pub mod settings;
#[cfg(any(test, feature = "test-support"))]
pub mod test;
pub mod theme;
pub mod theme_selector;
mod time;
mod util;
pub mod workspace;
pub mod worktree;

use crate::util::TryFutureExt;
use channel::ChannelList;
use gpui::{action, ModelHandle};
use parking_lot::Mutex;
use postage::watch;
use std::sync::Arc;

pub use settings::Settings;

action!(About);
action!(Quit);
action!(Authenticate);

pub struct AppState {
    pub settings_tx: Arc<Mutex<watch::Sender<Settings>>>,
    pub settings: watch::Receiver<Settings>,
    pub languages: Arc<language::LanguageRegistry>,
    pub themes: Arc<settings::ThemeRegistry>,
    pub rpc: Arc<rpc::Client>,
    pub fs: Arc<dyn fs::Fs>,
    pub channel_list: ModelHandle<ChannelList>,
}

pub fn init(app_state: &Arc<AppState>, cx: &mut gpui::MutableAppContext) {
    cx.add_global_action(quit);

    cx.add_global_action({
        let rpc = app_state.rpc.clone();
        move |_: &Authenticate, cx| {
            let rpc = rpc.clone();
            cx.spawn(|cx| async move { rpc.authenticate_and_connect(cx).log_err().await })
                .detach();
        }
    });
}

fn quit(_: &Quit, cx: &mut gpui::MutableAppContext) {
    cx.platform().quit();
}
