use documented::Documented;
use gpui::{Hsla, point};

use crate::components::Label;
use crate::prelude::*;

/// A progress bar is a horizontal bar that communicates the status of a process.
///
/// A progress bar should not be used to represent indeterminate progress.
#[derive(RegisterComponent, Documented)]
pub struct ProgressBar {
    id: ElementId,
    value: f32,
    max_value: f32,
    bg_color: Hsla,
    fg_color: Hsla,
}

impl ProgressBar {
    /// Create a new progress bar with the given value and maximum value.
    pub fn new(
        id: impl Into<ElementId>,
        value: f32,
        max_value: f32,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            id: id.into(),
            value,
            max_value,
            bg_color: cx.theme().colors().background,
            fg_color: cx.theme().status().info,
        }
    }

    /// Set the current value of the progress bar.
    pub fn value(&mut self, value: f32) -> &mut Self {
        self.value = value;
        self
    }

    /// Set the maximum value of the progress bar.
    pub fn max_value(&mut self, max_value: f32) -> &mut Self {
        self.max_value = max_value;
        self
    }

    /// Set the background color of the progress bar.
    pub fn bg_color(&mut self, color: Hsla) -> &mut Self {
        self.bg_color = color;
        self
    }

    /// Set the foreground color of the progress bar.
    pub fn fg_color(&mut self, color: Hsla) -> &mut Self {
        self.fg_color = color;
        self
    }
}

impl Render for ProgressBar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let fill_width = (self.value / self.max_value).clamp(0.02, 1.0);

        div()
            .id(self.id.clone())
            .w_full()
            .h(px(8.0))
            .rounded_full()
            .py(px(2.0))
            .px(px(4.0))
            .bg(self.bg_color)
            .shadow(smallvec::smallvec![gpui::BoxShadow {
                color: gpui::black().opacity(0.08),
                offset: point(px(0.), px(1.)),
                blur_radius: px(0.),
                spread_radius: px(0.),
            }])
            .child(
                div()
                    .h_full()
                    .rounded_full()
                    .bg(self.fg_color)
                    .w(relative(fill_width)),
            )
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
        let max_value = 180.0;

        let empty_progress_bar = cx.new(|cx| ProgressBar::new("empty", 0.0, max_value, cx));
        let partial_progress_bar =
            cx.new(|cx| ProgressBar::new("partial", max_value * 0.35, max_value, cx));
        let filled_progress_bar = cx.new(|cx| ProgressBar::new("filled", max_value, max_value, cx));

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
                        .child(empty_progress_bar.clone()),
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
                        .child(partial_progress_bar.clone()),
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
                        .child(filled_progress_bar.clone()),
                )
                .into_any_element(),
        )
    }
}
