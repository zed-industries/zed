pub mod assets;
pub mod editor;
pub mod file_finder;
pub mod language;
pub mod menus;
mod operation_queue;
mod rpc_client;
pub mod settings;
mod sum_tree;
#[cfg(test)]
mod test;
mod time;
mod util;
pub mod workspace;
mod worktree;

#[derive(Clone)]
pub struct AppState {
    pub settings: postage::watch::Receiver<settings::Settings>,
    pub language_registry: std::sync::Arc<language::LanguageRegistry>,
}

pub fn init(cx: &mut gpui::MutableAppContext) {
    cx.add_global_action("app:quit", quit);
}

fn quit(_: &(), cx: &mut gpui::MutableAppContext) {
    cx.platform().quit();
}
