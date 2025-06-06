use gpui::{Context, FontWeight, IntoElement, Render, Window};
use ui::{prelude::*, tooltip_container};

pub struct MaxModeTooltip {
    selected: bool,
}

impl MaxModeTooltip {
    pub fn new() -> Self {
        Self { selected: false }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl Render for MaxModeTooltip {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let (icon, color) = if self.selected {
            (IconName::ZedBurnModeOn, Color::Error)
        } else {
            (IconName::ZedBurnMode, Color::Default)
        };

        let turned_on = h_flex()
            .h_4()
            .px_1()
            .border_1()
            .border_color(cx.theme().colors().border)
            .bg(cx.theme().colors().text_accent.opacity(0.1))
            .rounded_sm()
            .child(
                Label::new("ON")
                    .size(LabelSize::XSmall)
                    .weight(FontWeight::SEMIBOLD)
                    .color(Color::Accent),
            );

        let title = h_flex()
            .gap_1p5()
            .child(Icon::new(icon).size(IconSize::Small).color(color))
            .child(Label::new("Burn Mode"))
            .when(self.selected, |title| title.child(turned_on));

        tooltip_container(window, cx, |this, _, _| {
            this
                .child(title)
                .child(
                    div()
                        .max_w_64()
                        .child(
                            Label::new("Enables models to use large context windows, unlimited tool calls, and other capabilities for expanded reasoning.")
                                .size(LabelSize::Small)
                                .color(Color::Muted)
                        )
                )
        })
    }
}
