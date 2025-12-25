//! Performance metrics overlay for CSV preview debugging.
//!
//! Provides a semi-transparent overlay in the bottom-right corner showing
//! CSV parsing performance metrics for developer experience.

use std::time::{Duration, Instant};

use ui::{ActiveTheme, Context, IntoElement, ParentElement, Styled, StyledTypography, div};

use crate::CsvPreviewView;

/// Performance metrics for CSV operations.
#[derive(Debug, Default)]
pub struct PerformanceMetrics {
    /// Duration of the last CSV parsing operation.
    pub last_parse_took: Option<Duration>,
    /// Duration of the last table ordering/sorting operation.
    pub last_ordering_took: Option<Duration>,
    /// Duration of the last copy operation.
    pub last_copy_took: Option<Duration>,
    /// Duration of the last selection operation (navigation or select_all).
    pub last_selection_took: Option<Duration>,
    /// Duration of the last render preparation (table_with_settings div creation).
    pub last_render_preparation_took: Option<Duration>,
    /// List of display indices that were rendered in the current frame.
    pub rendered_indices: Vec<usize>,
}
/// Extension trait for timing the execution of a closure and storing the duration.
///
/// This trait is implemented for `Option<Duration>`, allowing you to easily
/// time an operation and store its duration in place. For example:
///
/// ```rust
/// self.performance_metrics
///     .last_selection_took
///     .record_timing(|| self.engine.change_selection(direction, operation));
/// ```
///
/// The previous value is replaced with the new duration.
pub(crate) trait TimingRecorder {
    /// Runs the provided closure, records its execution time, and stores it in `self`.
    ///
    /// # Arguments
    ///
    /// * `f` - The closure to execute and time.
    ///
    /// # Returns
    ///
    /// Returns the result of the closure.
    fn record_timing<F, R>(&mut self, f: F) -> R
    where
        F: FnMut() -> R;
}

impl TimingRecorder for Option<Duration> {
    fn record_timing<F, R>(&mut self, mut f: F) -> R
    where
        F: FnMut() -> R,
    {
        let start_time = Instant::now();
        let ret = f();
        let duration = start_time.elapsed();
        self.replace(duration);
        ret
    }
}
impl PerformanceMetrics {
    fn format_lines(&self) -> Vec<String> {
        let format_duration = |duration: Option<Duration>| -> String {
            match duration {
                Some(d) => format!("{:.2}ms", d.as_secs_f64() * 1000.0),
                None => "--".to_string(),
            }
        };

        let mut lines = vec![
            format!("- Parse: {}", format_duration(self.last_parse_took)),
            format!("- Order: {}", format_duration(self.last_ordering_took)),
            format!("- Copy: {}", format_duration(self.last_copy_took)),
            format!("- Selection: {}", format_duration(self.last_selection_took)),
            format!(
                "- Render Prep: {}",
                format_duration(self.last_render_preparation_took)
            ),
        ];

        // Add rendered indices information
        if self.rendered_indices.is_empty() {
            lines.push("- Rendered: none".to_string());
        } else {
            lines.push(format!("- Rendered: {} rows", self.rendered_indices.len()));
            if self.rendered_indices.len() <= 20 {
                // Show indices if not too many
                lines.push(format!("  {:?}", self.rendered_indices));
            } else {
                // Show first/last few if too many
                let first_few = &self.rendered_indices[..5];
                let last_few = &self.rendered_indices[self.rendered_indices.len() - 5..];
                lines.push(format!("  {:?}\n..{:?}", first_few, last_few));
            }
        }

        lines
    }
}

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
                self.performance_metrics
                    .format_lines()
                    .into_iter()
                    .map(|line| div().child(line)),
            );
        // Clear rendered indices to prepare for next frame
        self.performance_metrics.rendered_indices.clear();
        children
    }
}
