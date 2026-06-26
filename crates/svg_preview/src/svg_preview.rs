use gpui::{App, actions};
use workspace::{Workspace, register_auto_preview_provider};

pub mod svg_preview_view;

use crate::svg_preview_view::SvgAutoPreviewProvider;

pub use zed_actions::preview::svg::{OpenPreview, OpenPreviewToTheSide};

actions!(
    svg,
    [
        /// Opens a following SVG preview that syncs with the editor.
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

    register_auto_preview_provider(SvgAutoPreviewProvider, cx);
}
