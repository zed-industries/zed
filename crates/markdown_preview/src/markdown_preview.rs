use editor::Editor;
use gpui::{App, Focusable, actions};
use settings::{AutoPreviewMode, Settings};
use workspace::item::PreviewTabsSettings;
use workspace::{Event as WorkspaceEvent, Workspace};

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
        cx.subscribe_in(&workspace_handle, window, move |_workspace, _, event, window, cx| {
            let WorkspaceEvent::ItemAdded { item } = event else {
                return;
            };
            let auto_preview = PreviewTabsSettings::get_global(cx).auto_preview;
            if matches!(auto_preview, AutoPreviewMode::Off) {
                return;
            }
            let Some(editor) = item.act_as::<Editor>(cx) else {
                return;
            };
            if !is_markdown_file_by_extension(&editor, cx) {
                return;
            }

            let weak_workspace = cx.entity().downgrade();
            window.defer(cx, move |window, cx| {
                let Some(workspace) = weak_workspace.upgrade() else {
                    return;
                };
                workspace.update(cx, |workspace, cx| {
                    let view = markdown_preview_view::MarkdownPreviewView::create_markdown_view(
                        workspace,
                        editor.clone(),
                        window,
                        cx,
                    );

                    match auto_preview {
                        AutoPreviewMode::SamePane => {
                            workspace.active_pane().update(cx, |pane, cx| {
                                if markdown_preview_view::MarkdownPreviewView::find_existing_independent_preview_item_idx(pane, &editor, cx).is_none() {
                                    pane.add_item(Box::new(view), true, true, None, window, cx);
                                }
                            });
                        }
                        AutoPreviewMode::ToSide => {
                            let pane = workspace
                                .find_pane_in_direction(workspace::SplitDirection::Right, cx)
                                .unwrap_or_else(|| {
                                    workspace.split_pane(
                                        workspace.active_pane().clone(),
                                        workspace::SplitDirection::Right,
                                        window,
                                        cx,
                                    )
                                });
                            pane.update(cx, |pane, cx| {
                                if markdown_preview_view::MarkdownPreviewView::find_existing_independent_preview_item_idx(pane, &editor, cx).is_none() {
                                    pane.add_item(Box::new(view), false, false, None, window, cx);
                                }
                            });
                            editor.focus_handle(cx).focus(window, cx);
                        }
                        AutoPreviewMode::Off => {}
                    }
                    cx.notify();
                });
            });
        })
        .detach();
    })
    .detach();
}

fn is_markdown_file_by_extension(editor: &gpui::Entity<Editor>, cx: &gpui::App) -> bool {
    editor
        .read(cx)
        .buffer()
        .read(cx)
        .as_singleton()
        .and_then(|buffer| buffer.read(cx).file())
        .is_some_and(|file| {
            std::path::Path::new(file.file_name(cx))
                .extension()
                .is_some_and(|ext| {
                    ext.eq_ignore_ascii_case("md") || ext.eq_ignore_ascii_case("markdown")
                })
        })
}
