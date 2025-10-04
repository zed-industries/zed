use gpui::{IntoElement, Render};
use ui::{Divider, prelude::*, tooltip_container};

pub struct TerminalTooltip {
    title: SharedString,
    pid: u32,
}

impl TerminalTooltip {
    pub fn new(title: impl Into<SharedString>, pid: u32) -> Self {
        Self {
            title: title.into(),
            pid,
        }
    }
}

impl Render for TerminalTooltip {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        tooltip_container(cx, move |this, _cx| {
            this.occlude()
                .on_mouse_move(|_, _window, cx| cx.stop_propagation())
                .child(
                    v_flex()
                        .gap_1()
                        .child(Label::new(self.title.clone()))
                        .child(Divider::horizontal())
                        .child(
                            Label::new(format!("Process ID (PID): {}", self.pid))
                                .color(Color::Muted)
                                .size(LabelSize::Small),
                        ),
                )
        })
    }
}
