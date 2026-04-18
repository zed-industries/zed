use editor::Editor;
use gpui::{App, actions};
use settings::Settings;
use workspace::{AutoPreviewSetting, Workspace, WorkspaceSettings};

pub mod markdown_preview_view;

pub use zed_actions::preview::markdown::{OpenPreview, OpenPreviewToTheSide};

actions!(
    markdown,
    [
        /// Scrolls up by one page in the markdown preview.
        #[action(deprecated_aliases = ["markdown::MovePageUp"])]
        ScrollPageUp,
        /// Scrolls down by one page in the markdown preview.
        #[action(deprecated_aliases = ["markdown::MovePageDown"])]
        ScrollPageDown,
        /// Scrolls up by approximately one visual line.
        ScrollUp,
        /// Scrolls down by approximately one visual line.
        ScrollDown,
        /// Scrolls up by one markdown element in the markdown preview
        ScrollUpByItem,
        /// Scrolls down by one markdown element in the markdown preview
        ScrollDownByItem,
        /// Scrolls to the top of the markdown preview.
        ScrollToTop,
        /// Scrolls to the bottom of the markdown preview.
        ScrollToBottom,
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

        let workspace_handle = cx.entity();
        cx.subscribe_in(
            &workspace_handle,
            window,
            |_workspace, _, event: &workspace::Event, window, cx| {
                if let workspace::Event::ItemAdded { item } = event {
                    let auto_preview = WorkspaceSettings::get_global(cx).auto_preview;
                    if auto_preview == AutoPreviewSetting::Disabled {
                        return;
                    }
                    if let Some(editor) = item.act_as::<Editor>(cx) {
                        if markdown_preview_view::MarkdownPreviewView::is_markdown_file(&editor, cx)
                        {
                            match auto_preview {
                                AutoPreviewSetting::Preview => {
                                    window.dispatch_action(Box::new(OpenPreview), cx);
                                }
                                AutoPreviewSetting::PreviewToSide => {
                                    window.dispatch_action(Box::new(OpenPreviewToTheSide), cx);
                                }
                                AutoPreviewSetting::Disabled => {}
                            }
                        }
                    }
                }
            },
        )
        .detach();
    })
    .detach();
}
