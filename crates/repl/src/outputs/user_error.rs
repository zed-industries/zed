use gpui::{AnyElement, App, Entity, FontWeight, Window};
use ui::{Label, h_flex, prelude::*, v_flex};

use crate::outputs::plain::TerminalOutput;

/// Userspace error from the kernel
pub struct ErrorView {
    pub ename: String,
    pub evalue: String,
    pub traceback: Entity<TerminalOutput>,
}

impl ErrorView {
    pub fn render(&self, window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let theme = cx.theme();

        let padding = window.line_height() / 2.;

        Some(
            v_flex()
                .gap_3()
                .child(
                    h_flex()
                        .font_buffer(cx)
                        .child(
                            Label::new(format!("{}: ", self.ename.clone()))
                                // .size(LabelSize::Large)
                                .color(Color::Error)
                                .weight(FontWeight::BOLD),
                        )
                        .child(
                            Label::new(self.evalue.clone())
                                // .size(LabelSize::Large)
                                .weight(FontWeight::BOLD),
                        ),
                )
                .child(
                    div()
                        .w_full()
                        .px(padding)
                        .py(padding)
                        .border_l_1()
                        .border_color(theme.status().error_border)
                        .child(self.traceback.clone()),
                )
                .into_any_element(),
        )
    }
}
