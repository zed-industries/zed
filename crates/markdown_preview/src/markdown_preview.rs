use gpui::{actions, AppContext};
use workspace::Workspace;

pub mod markdown_elements;
pub mod markdown_parser;
pub mod markdown_preview_view;
pub mod markdown_renderer;

actions!(markdown, [OpenPreview, OpenPreviewToTheSide]);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(|workspace: &mut Workspace, cx| {
        markdown_preview_view::MarkdownPreviewView::register(workspace, cx);
    })
    .detach();
}
