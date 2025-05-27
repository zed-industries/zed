use gpui::{Context, IntoElement, Render, Window};
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
        let icon = if self.selected {
            IconName::ZedLitModeOn
        } else {
            IconName::ZedLitMode
        };

        let title = h_flex()
            .gap_1()
            .child(Icon::new(icon).size(IconSize::Small))
            .child(Label::new("Burn Mode"));

        tooltip_container(window, cx, |this, _, _| {
            this.gap_0p5()
                .map(|header| if self.selected {
                    header.child(
                        h_flex()
                            .justify_between()
                            .child(title)
                            .child(
                                h_flex()
                                    .gap_0p5()
                                    .child(Icon::new(IconName::Check).size(IconSize::XSmall).color(Color::Accent))
                                    .child(Label::new("Turned On").size(LabelSize::XSmall).color(Color::Accent))
                            )
                    )
                } else {
                    header.child(title)
                })
                .child(
                    div()
                        .max_w_72()
                        .child(
                            Label::new("Enables models to use large context windows, unlimited tool calls, and other capabilities for expanded reasoning, offering an unfettered agentic experience.")
                                .size(LabelSize::Small)
                                .color(Color::Muted)
                        )
                )
        })
    }
}
