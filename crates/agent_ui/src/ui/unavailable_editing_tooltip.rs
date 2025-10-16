use gpui::{Context, IntoElement, Render, Window};
use ui::{prelude::*, tooltip_container};

pub struct UnavailableEditingTooltip {
    agent_name: SharedString,
}

impl UnavailableEditingTooltip {
    pub fn new(agent_name: SharedString) -> Self {
        Self { agent_name }
    }
}

impl Render for UnavailableEditingTooltip {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        tooltip_container(cx, |this, _| {
            this.child(Label::new("Unavailable Editing")).child(
                div().max_w_64().child(
                    Label::new(format!(
                        "Editing previous messages is not available for {} yet.",
                        self.agent_name
                    ))
                    .size(LabelSize::Small)
                    .color(Color::Muted),
                ),
            )
        })
    }
}
