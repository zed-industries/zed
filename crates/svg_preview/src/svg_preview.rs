use gpui::{App, actions};
use multi_buffer::MultiBuffer;
use settings::{AutoPreviewMode, Settings};
use workspace::item::PreviewTabsSettings;
use workspace::{Event as WorkspaceEvent, Workspace};

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
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        crate::svg_preview_view::SvgPreviewView::register(workspace, window, cx);

        let workspace_handle = cx.entity();
        cx.subscribe_in(&workspace_handle, window, move |_workspace, _, event, window, cx| {
            let WorkspaceEvent::ItemAdded { item } = event else {
                return;
            };
            let auto_preview = PreviewTabsSettings::get_global(cx).auto_preview;
            if matches!(auto_preview, AutoPreviewMode::Off) {
                return;
            }
            let Some(buffer) = item.act_as::<MultiBuffer>(cx) else {
                return;
            };
            if !svg_preview_view::SvgPreviewView::is_svg_file(&buffer, cx) {
                return;
            }

            let weak_workspace = cx.entity().downgrade();
            window.defer(cx, move |window, cx| {
                let Some(workspace) = weak_workspace.upgrade() else {
                    return;
                };
                workspace.update(cx, |workspace, cx| {
                    let view = svg_preview_view::SvgPreviewView::create_svg_view(
                        svg_preview_view::SvgPreviewMode::Default,
                        workspace,
                        buffer.clone(),
                        window,
                        cx,
                    );

                    match auto_preview {
                        AutoPreviewMode::SamePane => {
                            workspace.active_pane().update(cx, |pane, cx| {
                                if svg_preview_view::SvgPreviewView::find_existing_preview_item_idx(pane, &buffer, cx).is_none() {
                                    pane.add_item(Box::new(view), true, true, None, window, cx);
                                }
                            });
                        }
                        AutoPreviewMode::ToSide => {
                            let active_pane = workspace.active_pane().clone();
                            let pane = workspace
                                .find_pane_in_direction(workspace::SplitDirection::Right, cx)
                                .or_else(|| {
                                    workspace.panes().iter().find(|p| **p != active_pane).cloned()
                                })
                                .unwrap_or_else(|| {
                                    workspace.split_pane(
                                        active_pane,
                                        workspace::SplitDirection::Right,
                                        window,
                                        cx,
                                    )
                                });
                            pane.update(cx, |pane, cx| {
                                if svg_preview_view::SvgPreviewView::find_existing_preview_item_idx(pane, &buffer, cx).is_none() {
                                    pane.add_item(Box::new(view), false, false, None, window, cx);
                                }
                            });
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
