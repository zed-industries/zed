use gpui::FontWeight;

use crate::prelude::*;

/// A small, pill-shaped badge that displays a numeric count.
///
/// The count is capped at 99 and displayed as "99+" beyond that.
#[derive(IntoElement, RegisterComponent)]
pub struct CountBadge {
    count: usize,
}

impl CountBadge {
    pub fn new(count: usize) -> Self {
        Self { count }
    }
}

impl RenderOnce for CountBadge {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let label = if self.count > 99 {
            "99+".to_string()
        } else {
            self.count.to_string()
        };

        let bg = cx
            .theme()
            .colors()
            .editor_background
            .blend(cx.theme().status().error.opacity(0.4));

        h_flex()
            .absolute()
            .top_0()
            .right_0()
            .p_px()
            .h_3p5()
            .min_w_3p5()
            .rounded_full()
            .justify_center()
            .text_center()
            .border_1()
            .border_color(cx.theme().colors().border)
            .bg(bg)
            .shadow_sm()
            .child(
                Label::new(label)
                    .size(LabelSize::Custom(rems_from_px(9.)))
                    .weight(FontWeight::MEDIUM),
            )
    }
}

impl Component for CountBadge {
    fn scope() -> ComponentScope {
        ComponentScope::Status
    }

    fn description() -> Option<&'static str> {
        Some("A small, pill-shaped badge that displays a numeric count.")
    }

    fn preview(_window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let container = || {
            div()
                .relative()
                .size_8()
                .border_1()
                .border_color(cx.theme().colors().border)
                .bg(cx.theme().colors().background)
        };

        Some(
            v_flex()
                .gap_6()
                .child(example_group_with_title(
                    "Count Badge",
                    vec![
                        single_example(
                            "Basic Count",
                            container().child(CountBadge::new(3)).into_any_element(),
                        ),
                        single_example(
                            "Capped Count",
                            container().child(CountBadge::new(150)).into_any_element(),
                        ),
                    ],
                ))
                .into_any_element(),
        )
    }
}
