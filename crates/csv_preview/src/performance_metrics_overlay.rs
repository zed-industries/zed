//! Performance metrics overlay for CSV preview debugging.
//!
//! Provides a semi-transparent overlay in the bottom-right corner showing
//! CSV parsing performance metrics for developer experience.

use std::time::Duration;

use ui::{
    ActiveTheme, Context, FluentBuilder, IntoElement, ParentElement, Styled, StyledTypography, div,
};

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
            .child(
                div().map(|div| match self.performance_metrics.last_parse_took {
                    Some(duration) => {
                        div.child(format!("Parse: {:.2}ms", duration.as_secs_f64() * 1000.0))
                    }
                    None => div.child("Parse: --"),
                }),
            )
            .child(
                div().map(|div| match self.performance_metrics.last_ordering_took {
                    Some(duration) => {
                        div.child(format!("Order: {:.2}ms", duration.as_secs_f64() * 1000.0))
                    }
                    None => div.child("Order: --"),
                }),
            )
            .child(
                div().map(|div| match self.performance_metrics.last_copy_took {
                    Some(duration) => {
                        div.child(format!("Copy: {:.2}ms", duration.as_secs_f64() * 1000.0))
                    }
                    None => div.child("Copy: --"),
                }),
            )
    }
}
