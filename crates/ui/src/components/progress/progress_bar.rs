use documented::Documented;
use gpui::StyleRefinement;

use crate::components::Label;
use crate::prelude::*;

/// A progress bar is a horizontal bar that communicates the status of a process.
///
/// A progress bar should not be used to represent indeterminate progress.
#[derive(IntoElement, RegisterComponent, Documented)]
pub struct ProgressBar {
    base: Div,
    value: f32,
    max_value: f32,
}

impl ProgressBar {
    /// Create a new progress bar with the given value and maximum value.
    pub fn new(value: f32, max_value: f32) -> Self {
        Self {
            base: div().h(px(8.0)).rounded_full(),
            value,
            max_value,
        }
    }

    /// Set the current value of the progress bar.
    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    /// Set the maximum value of the progress bar.
    pub fn max_value(mut self, max_value: f32) -> Self {
        self.max_value = max_value;
        self
    }
}

impl RenderOnce for ProgressBar {
    fn render(self, window: &mut gpui::Window, cx: &mut gpui::App) -> impl IntoElement {
        let fill_width = (self.value / self.max_value).clamp(0.0, 1.0);

        self.base.child(
            div()
                .w_full()
                .h_full()
                .bg(cx.theme().colors().element_background)
                .w(relative(fill_width)),
        )
    }
}

impl Styled for ProgressBar {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl Component for ProgressBar {
    fn scope() -> ComponentScope {
        ComponentScope::Status
    }

    fn description() -> Option<&'static str> {
        Some(Self::DOCS)
    }

    fn preview(_window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        Some(
            div()
                .flex()
                .flex_col()
                .gap_4()
                .p_4()
                .w(px(240.0))
                .child(div().child("Progress Bar"))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child(
                            div()
                                .flex()
                                .justify_between()
                                .child(Label::new("0%"))
                                .child(Label::new("Empty")),
                        )
                        .child(ProgressBar::new(0.0, 100.0)),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child(
                            div()
                                .flex()
                                .justify_between()
                                .child(Label::new("38%"))
                                .child(Label::new("Partial")),
                        )
                        .child(ProgressBar::new(38.0, 100.0)),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child(
                            div()
                                .flex()
                                .justify_between()
                                .child(Label::new("100%"))
                                .child(Label::new("Complete")),
                        )
                        .child(ProgressBar::new(100.0, 100.0)),
                )
                .into_any_element(),
        )
    }
}
