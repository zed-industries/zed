use gpui::{IntoElement, Render, ViewContext};
use ui::{prelude::*, tooltip_container, Divider};

pub struct TerminalTooltip {
    title: SharedString,
    pid: String,
    cmd: String,
    tty: String,
}

impl TerminalTooltip {
    pub fn new(title: impl Into<SharedString>, pid: String, cmd: String, tty: String) -> Self {
        Self {
            title: title.into(),
            pid,
            cmd,
            tty,
        }
    }
}

impl Render for TerminalTooltip {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        tooltip_container(cx, move |this, _cx| {
            this.occlude()
                .on_mouse_move(|_, cx| cx.stop_propagation())
                .child(
                    v_flex()
                        .gap_1()
                        .child(Label::new(self.title.clone()))
                        .child(Divider::horizontal())
                        .child(
                            Label::new(format!("{}", self.pid))
                                .color(Color::Muted)
                                .size(LabelSize::Small),
                        )
                        .child(
                            Label::new(format!("{}", self.cmd))
                                .color(Color::Muted)
                                .size(LabelSize::Small),
                        )
                        .child(
                            Label::new(format!("{}", self.tty))
                                .color(Color::Muted)
                                .size(LabelSize::Small),
                        ),
                )
        })
    }
}
