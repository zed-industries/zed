use gpui::{AnyElement, Modifiers, WeakEntity};
use svg_preview::{OpenPreview, OpenPreviewToTheSide, svg_preview_view::SvgPreviewView};
use ui::{IconButtonShape, Tooltip, prelude::*, text_for_keystroke};
use workspace::Workspace;

use super::QuickActionBar;

impl QuickActionBar {
    pub fn render_toggle_svg_preview(
        &self,
        workspace: WeakEntity<Workspace>,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let mut active_editor_is_svg = false;

        if let Some(workspace) = self.workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                active_editor_is_svg =
                    SvgPreviewView::resolve_active_item_as_svg_editor(workspace, cx).is_some();
            });
        }

        if !active_editor_is_svg {
            return None;
        }

        let alt_click = gpui::Keystroke {
            key: "click".into(),
            modifiers: Modifiers::alt(),
            ..Default::default()
        };

        let button = IconButton::new("toggle-svg-preview", IconName::Eye)
            .shape(IconButtonShape::Square)
            .icon_size(IconSize::Small)
            .style(ButtonStyle::Subtle)
            .tooltip(move |window, cx| {
                Tooltip::with_meta(
                    "Preview SVG",
                    Some(&svg_preview::OpenPreview),
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
