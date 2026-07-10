use std::time::Instant;

use ui::{SpinnerLabel, div, prelude::*};

use crate::CsvPreviewView;

impl Render for CsvPreviewView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let row_height = window.pixel_snap(window.line_height());
        if row_height != self.row_height {
            self.row_height = row_height;
            // Font size (rem size, buffer font override, ...) changed since the list was last
            // measured: existing rows and unmeasured-item height hints are now the wrong size.
            // Unlike `reset_with_uniform_height`, this preserves scroll position and keeps each
            // item's prior size as a hint rather than dropping straight to a fresh guess.
            self.list_state.remeasure();
        }
        let render_prep_start = Instant::now();
        let table_with_settings = v_flex()
            .size_full()
            .bg(theme.colors().editor_background)
            .track_focus(&self.focus_handle)
            .child(self.render_settings_panel(window, cx))
            .child({
                let is_parsing = self.is_parsing;
                if is_parsing || self.engine.contents.number_of_cols == 0 {
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .h_32()
                        .text_ui(cx)
                        .font_buffer(cx)
                        .text_color(cx.theme().colors().text_muted)
                        .when(is_parsing, |div| {
                            div.child(
                                h_flex()
                                    .gap_2()
                                    .child(SpinnerLabel::new())
                                    .child("Loading…"),
                            )
                        })
                        .when(!is_parsing, |div| div.child("No CSV content to display"))
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

        let div = div()
            .relative()
            .w_full()
            .h_full()
            .child(table_with_settings);

        #[cfg(feature = "dev-tools")]
        let show_perf_metrics_overlay = self.settings.show_perf_metrics_overlay;

        #[cfg(feature = "dev-tools")]
        let div = div.when(show_perf_metrics_overlay, |div| {
            div.child(self.render_performance_metrics_overlay(cx))
        });

        #[cfg(feature = "dev-tools")]
        if !show_perf_metrics_overlay {
            self.performance_metrics.rendered_indices.clear();
        }

        #[cfg(not(feature = "dev-tools"))]
        self.performance_metrics.rendered_indices.clear();

        div
    }
}
