use gpui::{App, actions};
use workspace::Workspace;

pub mod svg_preview_view;

actions!(
    svg,
    [OpenPreview, OpenPreviewToTheSide, OpenFollowingPreview]
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
