//! Performance metrics overlay for CSV preview debugging.
//!
//! Provides a semi-transparent overlay in the bottom-right corner showing
//! CSV parsing performance metrics for developer experience.

use std::time::Duration;

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
}

impl PerformanceMetrics {
    fn format_lines(&self) -> Vec<String> {
        let format_duration = |duration: Option<Duration>| -> String {
            match duration {
                Some(d) => format!("{:.2}ms", d.as_secs_f64() * 1000.0),
                None => "--".to_string(),
            }
        };

        vec![
            format!("Parse: {}", format_duration(self.last_parse_took)),
            format!("Order: {}", format_duration(self.last_ordering_took)),
            format!("Copy: {}", format_duration(self.last_copy_took)),
            format!("Selection: {}", format_duration(self.last_selection_took)),
            format!(
                "Render Prep: {}",
                format_duration(self.last_render_preparation_took)
            ),
        ]
    }
}

impl CsvPreviewView {
    /// Renders a semi-transparent performance metrics overlay in the bottom-right corner.
    ///
    /// Shows CSV parsing duration for debugging and performance monitoring.
    /// The overlay is positioned absolutely and styled with reduced opacity.
    pub(crate) fn render_performance_metrics_overlay(
        &self,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.theme();

        div()
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
            .children(
                self.performance_metrics
                    .format_lines()
                    .into_iter()
                    .map(|line| div().child(line)),
            )
    }
}
