use gpui::{Context, IntoElement, Render, Window};
use ui::{prelude::*, tooltip_container};

pub struct MaxModeTooltip;

impl MaxModeTooltip {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for MaxModeTooltip {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        tooltip_container(_window, cx, |this, _, _| {
            this.gap_1()
                .child(
                    h_flex()
                        .gap_1p5()
                        .child(Icon::new(IconName::ZedMaxMode).size(IconSize::Small))
                        .child(Label::new("Zed's Max Mode"))
                )
                .child(
                    div()
                        .max_w_72()
                        .child(
                            Label::new("In Max Mode, we enable models to use large context windows, unlimited tool calls, and other capabilities for expanded reasoning, to allow an unfettered agentic experience.")
                                .size(LabelSize::Small)
                                .color(Color::Muted)
                        )
                )
        })
    }
}
