use gpui::{AnyElement, Modifiers, WeakView};
use markdown_preview::{
    markdown_preview_view::MarkdownPreviewView, OpenPreview, OpenPreviewToTheSide,
};
use ui::{prelude::*, text_for_keystroke, IconButtonShape, Tooltip};
use workspace::Workspace;

use super::QuickActionBar;

impl QuickActionBar {
    pub fn render_toggle_markdown_preview(
        &self,
        workspace: WeakModel<Workspace>,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) -> Option<AnyElement> {
        let mut active_editor_is_markdown = false;

        if let Some(workspace) = self.workspace.upgrade() {
            workspace.update(cx, |workspace, model, cx| {
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
                    format!(
                        "{} to open in a split",
                        text_for_keystroke(&alt_click, PlatformStyle::platform())
                    ),
                    model,
                    cx,
                )
            })
            .on_click(move |_, cx| {
                if let Some(workspace) = workspace.upgrade() {
                    workspace.update(cx, |_, model, cx| {
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
