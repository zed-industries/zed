pub mod context_server_log_view;

use gpui::App;

pub use context_server_log_view::{
    ContextServerLogToolbarItemView, ContextServerLogView, open_server_logs,
};

pub fn init(cx: &mut App) {
    context_server_log_view::init(cx);
}
