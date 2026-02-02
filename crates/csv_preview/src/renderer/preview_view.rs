use std::time::Instant;

use ui::{ScrollAxes, WithScrollbar, div, prelude::*};

use crate::{CsvPreviewView, settings::FontType};

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
            .child(self.render_settings_panel(window, cx))
            .child({
                if self.engine.contents.number_of_cols == 0 {
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
                        .child(self.create_table(&self.column_widths.widths, cx))
                        .into_any_element()
                }
            });

        let render_prep_duration = render_prep_start.elapsed();
        self.performance_metrics.timings.insert(
            "render_prep",
            (render_prep_duration, std::time::Instant::now()),
        );

        div()
            .relative()
            .w_full()
            .h_full()
            .child(table_with_settings)
    }
}
