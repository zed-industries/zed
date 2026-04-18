use gpui::{App, actions};
use multi_buffer::MultiBuffer;
use settings::Settings;
use workspace::{AutoPreviewSetting, Workspace, WorkspaceSettings};

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
        cx.subscribe_in(
            &workspace_handle,
            window,
            |_workspace, _, event: &workspace::Event, window, cx| {
                if let workspace::Event::ItemAdded { item } = event {
                    let auto_preview = WorkspaceSettings::get_global(cx).auto_preview;
                    if auto_preview == AutoPreviewSetting::Disabled {
                        return;
                    }
                    if let Some(buffer) = item.act_as::<MultiBuffer>(cx) {
                        if crate::svg_preview_view::SvgPreviewView::is_svg_file(&buffer, cx) {
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
