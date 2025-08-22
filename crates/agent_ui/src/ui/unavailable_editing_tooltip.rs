use gpui::{Context, IntoElement, Render, Window};
use ui::{prelude::*, tooltip_container};

pub struct UnavailableEditingTooltip {}

impl UnavailableEditingTooltip {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for UnavailableEditingTooltip {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        tooltip_container(window, cx, |this, _, _| {
            this.child(Label::new("Editing Unavailable")).child(
                div().max_w_64().child(
                    Label::new(
                        "Editing previosus user messages is not available for this provider yet.",
                    )
                    .size(LabelSize::Small)
                    .color(Color::Muted),
                ),
            )
        })
    }
}
