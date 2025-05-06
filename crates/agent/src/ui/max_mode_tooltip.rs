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
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        tooltip_container(_window, cx, |this, _, _| {
            this.gap_1()
                .map(|header| if self.selected {
                    header.child(
                        h_flex()
                            .justify_between()
                            .child(
                                h_flex()
                                    .gap_1p5()
                                    .child(Icon::new(IconName::ZedMaxMode).size(IconSize::Small).color(Color::Accent))
                                    .child(Label::new("Zed's Max Mode"))
                            )
                            .child(
                                h_flex()
                                    .gap_0p5()
                                    .child(Icon::new(IconName::Check).size(IconSize::XSmall).color(Color::Accent))
                                    .child(Label::new("On").size(LabelSize::XSmall).color(Color::Accent))
                            )
                    )
                } else {
                    header.child(
                        h_flex()
                            .gap_1p5()
                            .child(Icon::new(IconName::ZedMaxMode).size(IconSize::Small))
                            .child(Label::new("Zed's Max Mode"))
                    )
                })
                .child(
                    div()
                        .max_w_72()
                        .child(
                            Label::new("This mode enables models to use large context windows, unlimited tool calls, and other capabilities for expanded reasoning, offering an unfettered agentic experience.")
                                .size(LabelSize::Small)
                                .color(Color::Muted)
                        )
                )
        })
    }
}
