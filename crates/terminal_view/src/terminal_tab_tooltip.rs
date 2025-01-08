use gpui::{IntoElement, Render, ViewContext};
use ui::{prelude::*, tooltip_container};

pub struct TerminalTooltip {
    title: SharedString,
}

impl TerminalTooltip {
    pub fn new(title: SharedString) -> Self {
        Self { title }
    }
}

impl Render for TerminalTooltip {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        tooltip_container(cx, move |this, _cx| {
            this.occlude()
                .on_mouse_move(|_, cx| cx.stop_propagation())
                .child(v_flex().gap_1().child(Label::new(self.title.clone())))
        })
    }
}
