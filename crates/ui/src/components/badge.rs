use std::rc::Rc;

use crate::Divider;
use crate::DividerColor;
use crate::Tooltip;
use crate::component_prelude::*;
use crate::prelude::*;
use gpui::AnyView;
use gpui::{AnyElement, IntoElement, SharedString, Window};

#[derive(IntoElement, RegisterComponent)]
pub struct Badge {
    label: SharedString,
    icon: IconName,
    tooltip: Option<Rc<dyn Fn(&mut Window, &mut App) -> AnyView>>,
}

impl Badge {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            icon: IconName::Check,
            tooltip: None,
        }
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = icon;
        self
    }

    pub fn tooltip(mut self, tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static) -> Self {
        self.tooltip = Some(Rc::new(tooltip));
        self
    }
}

impl RenderOnce for Badge {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let tooltip = self.tooltip;

        h_flex()
            .id(self.label.clone())
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
            .when_some(tooltip, |this, tooltip| {
                this.tooltip(move |window, cx| tooltip(window, cx))
            })
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
            v_flex()
                .gap_6()
                .child(single_example(
                    "Basic Badge",
                    Badge::new("Default").into_any_element(),
                ))
                .child(single_example(
                    "With Tooltip",
                    Badge::new("Tooltip")
                        .tooltip(Tooltip::text("This is a tooltip."))
                        .into_any_element(),
                ))
                .into_any_element(),
        )
    }
}
