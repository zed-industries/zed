use gpui::{App, actions};
use workspace::Workspace;

pub mod markdown_elements;
mod markdown_minifier;
pub mod markdown_parser;
pub mod markdown_preview_view;
pub mod markdown_renderer;

actions!(
    markdown,
    [
        /// Scrolls up by one page in the markdown preview.
        MovePageUp,
        /// Scrolls down by one page in the markdown preview.
        MovePageDown,
        /// Opens a markdown preview for the current file.
        OpenPreview,
        /// Opens a markdown preview in a split pane.
        OpenPreviewToTheSide,
        /// Opens a following markdown preview that syncs with the editor.
        OpenFollowingPreview
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        markdown_preview_view::MarkdownPreviewView::register(workspace, window, cx);
    })
    .detach();
}
