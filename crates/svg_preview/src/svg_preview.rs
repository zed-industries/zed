use gpui::{App, actions};
use workspace::Workspace;

pub mod svg_preview_view;

actions!(
    svg,
    [
        /// Open an SVG preview for the current file
        OpenPreview,
        /// Open an SVG preview in a split pane
        OpenPreviewToTheSide,
        /// Open a following SVG preview that syncs with the editor
        OpenFollowingPreview
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        crate::svg_preview_view::SvgPreviewView::register(workspace, window, cx);
    })
    .detach();
}
