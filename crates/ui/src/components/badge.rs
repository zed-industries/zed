use crate::Divider;
use crate::DividerColor;
use crate::component_prelude::*;
use crate::prelude::*;
use gpui::{AnyElement, IntoElement, SharedString, Window};

#[derive(IntoElement, RegisterComponent)]
pub struct Badge {
    label: SharedString,
    icon: IconName,
}

impl Badge {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            icon: IconName::Check,
        }
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = icon;
        self
    }
}

impl RenderOnce for Badge {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        h_flex()
            .h_full()
            .gap_1()
            .pl_1()
            .pr_2()
            .border_1()
            .border_color(cx.theme().colors().border.opacity(0.6))
            .bg(cx.theme().colors().element_background)
            .rounded_sm()
            .overflow_hidden()
            .child(
                Icon::new(self.icon)
                    .size(IconSize::XSmall)
                    .color(Color::Muted),
            )
            .child(Divider::vertical().color(DividerColor::Border))
            .child(Label::new(self.label.clone()).size(LabelSize::Small).ml_1())
    }
}

impl Component for Badge {
    fn scope() -> ComponentScope {
        ComponentScope::DataDisplay
    }

    fn description() -> Option<&'static str> {
        Some(
            "A compact, labeled component with optional icon for displaying status, categories, or metadata.",
        )
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            single_example("Basic Badge", Badge::new("Default").into_any_element())
                .into_any_element(),
        )
    }
}
