use std::time::Instant;

use ui::{div, prelude::*};

use crate::{CsvPreviewView, settings::FontType};

impl Render for CsvPreviewView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        let render_prep_start = Instant::now();
        let table_with_settings = v_flex()
            .size_full()
            .p_4()
            .bg(theme.colors().editor_background)
            // Apparently, this should make newly created CSV preview to get focus automatically
            .track_focus(&self.focus_handle)
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
                    self.create_table(&self.column_widths.widths, cx)
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
