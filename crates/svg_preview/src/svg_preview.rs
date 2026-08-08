use gpui::{App, actions};
use workspace::Workspace;

pub mod svg_preview_view;

pub use zed_actions::preview::svg::{OpenPreview, OpenPreviewToTheSide};

actions!(
    svg,
    [
        /// Opens a following SVG preview that syncs with the editor.
        OpenFollowingPreview,
        /// Zoom in the SVG preview.
        ZoomIn,
        /// Zoom out the SVG preview.
        ZoomOut,
        /// Reset zoom to 100%.
        ResetZoom,
        /// Fit the SVG to view.
        FitToView,
        /// Zoom to actual size (100%).
        ZoomToActualSize,
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
