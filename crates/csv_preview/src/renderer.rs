use std::time::Instant;

use ui::{ScrollAxes, WithScrollbar, div, prelude::*};

use crate::{CELL_EDITOR_CONTEXT_NAME, CsvPreviewView, TABLE_CONTEXT_NAME, settings::FontType};

impl Render for CsvPreviewView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        let render_prep_start = Instant::now();
        let table_with_settings = v_flex()
            .size_full()
            .p_4()
            .bg(theme.colors().editor_background)
            // Apparently, this should make newly created CSV preview to get focus automatically
            .track_focus(&self.focus_handle)
            .key_context(TABLE_CONTEXT_NAME)
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
            // Cell editor
            .on_action(cx.listener(Self::start_cell_editing))
            .on_action(cx.listener(Self::finish_cell_editing))
            .on_action(cx.listener(Self::cancel_cell_editing_handler))
            .child(self.render_settings_panel(window, cx))
            .when(self.settings.show_cell_editor_row, |div| {
                div.child(self.render_cell_editor(cx))
            })
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
                    // Wrapping into div to enable horizontal scrolling.
                    // This is super stinky solution, but unfortunatelly I don't know how to do better
                    div()
                        .id("table-div") // enables scrolling api
                        .size_full()
                        .overflow_x_scroll() // Allow the element to grow, so there's something to scroll
                        .track_scroll(&self.scroll_handle) // draws scrollbars
                        .custom_scrollbars(
                            // draws scrollbars when track_scroll is provided. Is utterly broken :D
                            ui::Scrollbars::new(ScrollAxes::Horizontal)
                                .tracked_scroll_handle(&self.scroll_handle)
                                .with_track_along(
                                    ScrollAxes::Horizontal,
                                    cx.theme().colors().panel_background,
                                )
                                .tracked_entity(cx.entity_id()),
                            window,
                            cx,
                        )
                        .child(self.render_table_with_cols(cx))
                        .into_any_element()
                }
            });

        let render_prep_duration = render_prep_start.elapsed();
        self.performance_metrics.last_render_preparation_took = Some(render_prep_duration);

        div()
            // .id("csv-preview-pane")
            // .overflow_scroll()
            .relative()
            .w_full()
            .h_full()
            .child(table_with_settings)
            .when(self.settings.show_perf_metrics_overlay, |div| {
                div.child(self.render_performance_metrics_overlay(cx))
            })
    }
}
