use gpui::MouseButton;
use ui::{div, prelude::*};

use crate::{CsvPreviewView, KEY_CONTEXT_NAME, settings::FontType};

impl Render for CsvPreviewView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        v_flex()
            .w_full()
            .h_full()
            .p_4()
            .bg(theme.colors().editor_background)
            .key_context(KEY_CONTEXT_NAME)
            .on_action(cx.listener(Self::copy_selected))
            .on_action(cx.listener(Self::clear_selection))
            .child(self.render_settings_panel(window, cx))
            .child({
                if self.contents.headers.is_empty() {
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .h_32()
                        .text_ui(cx)
                        .map(|div| match self.settings.font_type {
                            FontType::Ui => div.font_ui(cx),
                            FontType::Monospace => div.font_buffer(cx),
                        })
                        .text_color(cx.theme().colors().text_muted)
                        .child("No CSV content to display")
                        .into_any_element()
                } else {
                    self.render_table_with_cols(cx)
                }
            })
            // Workaround to be able to `end_selection`, when cursor is not over selectable cell, but within the table
            .on_mouse_up(MouseButton::Left, {
                let view = cx.entity();
                move |_event, _window, cx| {
                    view.update(cx, |this, cx| {
                        this.selection.end_selection();
                        cx.notify();
                    });
                }
            })
    }
}
