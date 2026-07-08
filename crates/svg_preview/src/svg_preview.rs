use gpui::{App, actions};
use workspace::Workspace;

pub mod svg_preview_view;

pub use zed_actions::preview::svg::{OpenPreview, OpenPreviewToTheSide};

actions!(
    svg,
    [
        /// Opens a following SVG preview that syncs with the editor.
        OpenFollowingPreview
    ]
);

pub fn init(cx: &mut App) {
    workspace::register_project_item::<svg_preview_view::SvgPreviewView>(cx);
    workspace::register_auto_preview_provider(
        svg_preview_view::SvgPreviewView::auto_preview_provider(),
        cx,
    );
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        crate::svg_preview_view::SvgPreviewView::register(workspace, window, cx);
    })
    .detach();
}
