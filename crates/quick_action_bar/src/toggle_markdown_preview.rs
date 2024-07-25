use gpui::{AnyElement, WeakView};
use markdown_preview::{
    markdown_preview_view::MarkdownPreviewView, OpenPreview, OpenPreviewToTheSide,
};
use ui::{prelude::*, IconButtonShape, Tooltip};
use workspace::Workspace;

use crate::QuickActionBar;

impl QuickActionBar {
    pub fn render_toggle_markdown_preview(
        &self,
        workspace: WeakView<Workspace>,
        cx: &mut ViewContext<Self>,
    ) -> Option<AnyElement> {
        let mut active_editor_is_markdown = false;

        if let Some(workspace) = self.workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                active_editor_is_markdown =
                    MarkdownPreviewView::resolve_active_item_as_markdown_editor(workspace, cx)
                        .is_some();
            });
        }

        if !active_editor_is_markdown {
            return None;
        }

        let preview_visible = false;

        let button = IconButton::new(
            "toggle-markdown-preview",
            if preview_visible {
                IconName::Code
            } else {
                IconName::FileText
            },
        )
        .shape(IconButtonShape::Square)
        .icon_size(IconSize::Small)
        .style(ButtonStyle::Subtle)
        .tooltip(move |cx| {
            Tooltip::with_meta(
                format!(
                    "{} Markdown Preview",
                    if preview_visible { "Hide" } else { "Show" }
                ),
                Some(&markdown_preview::OpenPreview),
                "Option+Click to open in a split",
                cx,
            )
        })
        .on_click(move |_, cx| {
            if let Some(workspace) = workspace.upgrade() {
                workspace.update(cx, |_, cx| {
                    if cx.modifiers().alt {
                        cx.dispatch_action(Box::new(OpenPreviewToTheSide));
                    } else {
                        cx.dispatch_action(Box::new(OpenPreview));
                    }
                });
            }
        });

        Some(button.into_any_element())
    }
}
