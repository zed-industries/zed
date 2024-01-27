use gpui::{actions, AppContext, ClipboardItem, PromptLevel};
use workspace::Workspace;

pub mod markdown_preview_modal;

actions!(markdown, [OpenPreview, SubmitFeedback]);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(|workspace: &mut Workspace, cx| {
        markdown_preview_modal::MarkdownPreviewModal::register(workspace, cx);
    })
    .detach();
}
