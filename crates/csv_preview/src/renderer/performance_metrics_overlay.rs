//! Performance metrics overlay for CSV preview debugging.
//!
//! Provides a semi-transparent overlay in the bottom-right corner showing
//! CSV parsing performance metrics for developer experience.

use std::time::Duration;

use ui::{ActiveTheme, Context, IntoElement, ParentElement, Styled, StyledTypography, div};

use crate::{CsvPreviewView, PerformanceMetrics};

impl CsvPreviewView {
    /// Renders a semi-transparent performance metrics overlay in the bottom-right corner.
    ///
    /// Shows CSV parsing duration for debugging and performance monitoring.
    /// The overlay is positioned absolutely and styled with reduced opacity.
    pub(crate) fn render_performance_metrics_overlay(
        &mut self,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.theme();

        let children = div()
            .absolute()
            .top_24()
            .right_4()
            .px_3()
            .py_2()
            .bg(theme.colors().editor_background)
            .border_1()
            .border_color(theme.colors().border)
            .rounded_md()
            .opacity(0.75)
            .text_xs()
            .font_buffer(cx)
            .text_color(theme.colors().text_muted)
            .flex()
            .flex_col()
            .gap_1()
            .child("Performance metrics:")
            .children(
                format_performance_metrics(&self.performance_metrics)
                    .into_iter()
                    .map(|line| div().child(line)),
            );
        // Clear rendered indices to prepare for next frame
        self.performance_metrics.rendered_indices.clear();
        children
    }
}

fn format_performance_metrics(this: &PerformanceMetrics) -> Vec<String> {
    let format_duration = |duration: Option<Duration>| -> String {
        match duration {
            Some(d) => format!("{:.2}ms", d.as_secs_f64() * 1000.0),
            None => "--".to_string(),
        }
    };

    let mut lines = vec![
        format!("- Parse: {}", format_duration(this.last_parse_took)),
        format!("- Order: {}", format_duration(this.last_ordering_took)),
        format!("- Copy: {}", format_duration(this.last_copy_took)),
        format!("- Selection: {}", format_duration(this.last_selection_took)),
        format!(
            "- Render Prep: {}",
            format_duration(this.last_render_preparation_took)
        ),
    ];

    // Add rendered indices information
    if this.rendered_indices.is_empty() {
        lines.push("- Rendered: none".to_string());
    } else {
        lines.push(format!("- Rendered: {} rows", this.rendered_indices.len()));
        if this.rendered_indices.len() <= 20 {
            // Show indices if not too many
            lines.push(format!("  {:?}", this.rendered_indices));
        } else {
            // Show first/last few if too many
            let first_few = &this.rendered_indices[..5];
            let last_few = &this.rendered_indices[this.rendered_indices.len() - 5..];
            lines.push(format!("  {:?}\n..{:?}", first_few, last_few));
        }
    }

    lines
}
