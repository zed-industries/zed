use zrpc::ForegroundRouter;

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
mod time;
mod util;
pub mod workspace;
pub mod worktree;

pub use settings::Settings;

pub struct AppState {
    pub settings: postage::watch::Receiver<Settings>,
    pub languages: std::sync::Arc<language::LanguageRegistry>,
    pub themes: std::sync::Arc<settings::ThemeRegistry>,
    pub rpc_router: std::sync::Arc<ForegroundRouter>,
    pub rpc: rpc::Client,
    pub fs: std::sync::Arc<dyn fs::Fs>,
}

pub fn init(cx: &mut gpui::MutableAppContext) {
    cx.add_global_action("app:quit", quit);
}

fn quit(_: &(), cx: &mut gpui::MutableAppContext) {
    cx.platform().quit();
}
