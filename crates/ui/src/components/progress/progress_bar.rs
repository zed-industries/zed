use std::time::Duration;

use documented::Documented;
use gpui::{StyleRefinement, point};
use util::ResultExt;

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
}

impl ProgressBar {
    /// Create a new progress bar with the given value and maximum value.
    pub fn new(id: impl Into<ElementId>, value: f32, max_value: f32) -> Self {
        Self {
            id: id.into(),
            value,
            max_value,
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
}

impl Render for ProgressBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let fill_width = (self.value / self.max_value).clamp(0.0, 1.0);

        div()
            .id(self.id.clone())
            .w_full()
            .h(px(8.0))
            .rounded_full()
            .p(px(2.0))
            .child(
                div()
                    .h_full()
                    .rounded_full()
                    .bg(cx.theme().status().info)
                    .shadow(smallvec::smallvec![gpui::BoxShadow {
                        color: cx.theme().colors().text.opacity(0.15),
                        offset: point(px(0.), px(1.)),
                        blur_radius: px(0.),
                        spread_radius: px(0.),
                    }])
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
        let mut current_percent: f32 = 0.0;

        let empty_progress_bar = cx.new(|_| ProgressBar::new("empty", 0.0, max_value));
        let partial_progress_bar =
            cx.new(|_| ProgressBar::new("partial", max_value * 0.35, max_value));
        let filled_progress_bar = cx.new(|_| ProgressBar::new("filled", max_value, max_value));
        let animated_progress_bar =
            cx.new(|_| ProgressBar::new("animated", current_percent, max_value));

        cx.spawn({
            let animated_progress_bar = animated_progress_bar.clone();
            async move |cx| {
                loop {
                    cx.background_executor()
                        .timer(Duration::from_millis(25))
                        .await;

                    animated_progress_bar
                        .update(cx, |progress_bar, cx| {
                            current_percent += 0.01;
                            if current_percent > 1.0 {
                                current_percent = 0.0;
                            }

                            progress_bar.value(current_percent * max_value);
                            cx.notify();
                        })
                        .log_err();
                }
            }
        })
        .detach();

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
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child(div().flex().child(Label::new("Animated")))
                        .child(animated_progress_bar.clone()),
                )
                .into_any_element(),
        )
    }
}
