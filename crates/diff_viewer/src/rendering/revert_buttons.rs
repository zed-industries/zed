use gpui::{Context, IntoElement, Window, div, prelude::*, px};
use ui::{ButtonCommon, ButtonSize, Clickable, IconButton, IconName, IconSize};

use crate::connector::ConnectorKind;
use crate::constants::CRUSHED_BLOCK_HEIGHT;
use crate::viewer::DiffViewer;

impl DiffViewer {
    pub fn render_left_editor_revert_buttons(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<impl IntoElement> {
        let mut buttons = Vec::new();
        let mut deleted_lines_above = 0usize;

        let rem_size = window.rem_size();
        let icon_height = IconSize::Small.rems().to_pixels(rem_size);
        let button_height = ButtonSize::Compact.rems().to_pixels(rem_size);

        let (current_line_height, current_scroll_pixels) =
            self.left_editor.update(cx, |editor, cx| {
                let line_height = editor
                    .style()
                    .map(|style| f32::from(style.text.line_height_in_pixels(window.rem_size())))
                    .unwrap_or(self.line_height);
                let scroll_rows = editor.scroll_position(cx).y;
                let scroll_pixels = (scroll_rows as f32) * line_height;
                (line_height, scroll_pixels)
            });

        for (index, curve) in self.connector_curves.iter().enumerate() {
            let left_len = curve.left_end.saturating_sub(curve.left_start) + 1;
            let right_len = curve.right_end.saturating_sub(curve.right_start) + 1;

            if curve.right_crushed {
                deleted_lines_above += left_len;
            } else if !curve.left_crushed && right_len < left_len {
                deleted_lines_above += left_len - right_len;
            }

            if !matches!(
                curve.kind,
                ConnectorKind::Modify | ConnectorKind::Delete | ConnectorKind::Insert
            ) {
                continue;
            }

            let block_index = curve.block_index;
            let is_left_empty = curve.left_crushed;
            let left_offset_rows = if is_left_empty {
                deleted_lines_above as f32
            } else {
                0.0
            };

            let left_row = if is_left_empty {
                curve.focus_line as f32 + left_offset_rows
            } else {
                curve.left_start as f32
            };

            let left_y = (left_row * current_line_height) - current_scroll_pixels;

            let minimal_block_height = CRUSHED_BLOCK_HEIGHT;
            let left_bottom = if is_left_empty {
                left_y + minimal_block_height
            } else {
                ((curve.left_end as f32 + 1.0) * current_line_height - current_scroll_pixels)
                    .max(left_y + minimal_block_height)
            };

            let block_height = left_bottom - left_y;
            let block_center_y = left_y + block_height / 2.0;

            let container_height = block_height
                .max(button_height.into())
                .max(icon_height.into());
            let container_top = block_center_y - container_height / 2.0;

            if container_top + container_height > 0.0 {
                let button = div()
                    .absolute()
                    .right(px(8.0))
                    .top(px(container_top))
                    .h(px(container_height))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        IconButton::new(("revert-btn", index), IconName::ArrowRight)
                            .icon_size(IconSize::Small)
                            .size(ButtonSize::Compact)
                            .on_click(cx.listener(move |this, _event, _window, cx| {
                                this.handle_revert_block(block_index, cx);
                            })),
                    );

                buttons.push(button);
            }
        }

        buttons
    }
}
