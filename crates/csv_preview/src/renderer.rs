use std::time::Instant;

use ui::{div, prelude::*};

use crate::{CsvPreviewView, KEY_CONTEXT_NAME, settings::FontType};

impl Render for CsvPreviewView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        let render_prep_start = Instant::now();
        let table_with_settings = v_flex()
            .w_full()
            .h_full()
            .p_4()
            .bg(theme.colors().editor_background)
            .key_context(KEY_CONTEXT_NAME)
            // Apparently, this should make newly created CSV preview to get focus automatically
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::copy_selected))
            .on_action(cx.listener(Self::clear_selection))
            .on_action(cx.listener(Self::select_up))
            .on_action(cx.listener(Self::select_down))
            .on_action(cx.listener(Self::select_left))
            .on_action(cx.listener(Self::select_right))
            .on_action(cx.listener(Self::extend_selection_up))
            .on_action(cx.listener(Self::extend_selection_down))
            .on_action(cx.listener(Self::extend_selection_left))
            .on_action(cx.listener(Self::extend_selection_right))
            .on_action(cx.listener(Self::select_all))
            .on_action(cx.listener(Self::select_at_top_edge))
            .on_action(cx.listener(Self::select_at_bottom_edge))
            .on_action(cx.listener(Self::select_at_left_edge))
            .on_action(cx.listener(Self::select_at_right_edge))
            .on_action(cx.listener(Self::extend_selection_to_top_edge))
            .on_action(cx.listener(Self::extend_selection_to_bottom_edge))
            .on_action(cx.listener(Self::extend_selection_to_left_edge))
            .on_action(cx.listener(Self::extend_selection_to_right_edge))
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
            });

        let render_prep_duration = render_prep_start.elapsed();
        self.performance_metrics.last_render_preparation_took = Some(render_prep_duration);

        div()
            .relative()
            .w_full()
            .h_full()
            .child(table_with_settings)
            .child(self.render_performance_metrics_overlay(cx))
    }
}
