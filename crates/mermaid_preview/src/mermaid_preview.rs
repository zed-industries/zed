use gpui::{App, actions};
use workspace::Workspace;

pub mod mermaid_preview_view;

pub use zed_actions::preview::mermaid::{OpenPreview, OpenPreviewToTheSide};

actions!(
    mermaid,
    [
        /// Opens a following Mermaid preview that syncs with the editor.
        OpenFollowingPreview
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        crate::mermaid_preview_view::MermaidPreviewView::register(workspace, window, cx);
    })
    .detach();
}
