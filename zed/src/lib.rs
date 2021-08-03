pub mod assets;
pub mod editor;
pub mod file_finder;
pub mod fs;
pub mod language;
pub mod menus;
mod operation_queue;
pub mod rpc;
pub mod settings;
mod sum_tree;
#[cfg(any(test, feature = "test-support"))]
pub mod test;
pub mod theme_selector;
mod time;
mod util;
pub mod workspace;
pub mod worktree;

pub use settings::Settings;

use futures::lock::Mutex;
use postage::watch;
use std::sync::Arc;
use zrpc::ForegroundRouter;

pub struct AppState {
    pub settings_tx: Arc<Mutex<watch::Sender<Settings>>>,
    pub settings: watch::Receiver<Settings>,
    pub languages: Arc<language::LanguageRegistry>,
    pub themes: Arc<settings::ThemeRegistry>,
    pub rpc_router: Arc<ForegroundRouter>,
    pub rpc: rpc::Client,
    pub fs: Arc<dyn fs::Fs>,
}

pub fn init(cx: &mut gpui::MutableAppContext) {
    cx.add_global_action("app:quit", quit);
}

fn quit(_: &(), cx: &mut gpui::MutableAppContext) {
    cx.platform().quit();
}
