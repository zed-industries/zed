use gpui::{actions, AppContext, ClipboardItem, PromptLevel};
use workspace::Workspace;

pub mod markdown_preview_view;

actions!(markdown, [OpenPreview, SubmitFeedback]);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(|workspace: &mut Workspace, cx| {
        markdown_preview_view::MarkdownPreviewView::register(workspace, cx);
    })
    .detach();
}
