use gpui::AppContext;

mod outputs;
mod runtime_manager;
mod runtime_panel;
mod runtime_session;
mod runtime_settings;
mod runtimes;
mod stdio;

pub use runtime_manager::RuntimeManager;
pub use runtime_panel::RuntimePanel;
pub use runtime_session::Session;

pub fn init(cx: &mut AppContext) {
    runtime_panel::init(cx)
}
