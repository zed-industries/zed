mod key_context_view;
mod lsp_log;
pub mod lsp_tool;
mod syntax_tree_view;

#[cfg(test)]
mod lsp_log_tests;

use gpui::App;

pub use lsp_log::{LogStore, LspLogToolbarItemView, LspLogView};
pub use syntax_tree_view::{SyntaxTreeToolbarItemView, SyntaxTreeView};

pub fn init(cx: &mut App) {
    lsp_log::init(cx);
    syntax_tree_view::init(cx);
    key_context_view::init(cx);
}
