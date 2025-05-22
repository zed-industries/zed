use gpui::{AnyElement, Modifiers, WeakEntity};
use markdown_preview::{
    OpenPreview, OpenPreviewToTheSide, markdown_preview_view::MarkdownPreviewView,
};
use ui::{IconButtonShape, Tooltip, prelude::*, text_for_keystroke};
use workspace::Workspace;

use super::QuickActionBar;

impl QuickActionBar {
    pub fn render_toggle_markdown_preview(
        &self,
        workspace: WeakEntity<Workspace>,
        cx: &mut Context<Self>,
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

        let alt_click = gpui::Keystroke {
            key: "click".into(),
            modifiers: Modifiers::alt(),
            ..Default::default()
        };

        let button = IconButton::new("toggle-markdown-preview", IconName::Eye)
            .shape(IconButtonShape::Square)
            .icon_size(IconSize::Small)
            .style(ButtonStyle::Subtle)
            .tooltip(move |window, cx| {
                Tooltip::with_meta(
                    "Preview Markdown",
                    Some(&markdown_preview::OpenPreview),
                    format!("{} to open in a split", text_for_keystroke(&alt_click, cx)),
                    window,
                    cx,
                )
            })
            .on_click(move |_, window, cx| {
                if let Some(workspace) = workspace.upgrade() {
                    workspace.update(cx, |_, cx| {
                        if window.modifiers().alt {
                            window.dispatch_action(Box::new(OpenPreviewToTheSide), cx);
                        } else {
                            window.dispatch_action(Box::new(OpenPreview), cx);
                        }
                    });
                }
            });

        Some(button.into_any_element())
    }
}
