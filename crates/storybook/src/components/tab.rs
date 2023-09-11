use crate::theme::theme;
use gpui2::style::{StyleHelpers, Styleable};
use gpui2::{elements::div, IntoElement};
use gpui2::{Element, ParentElement, ViewContext};

#[derive(Element)]
pub(crate) struct Tab {
    title: &'static str,
    active: bool,
}

pub fn tab<V: 'static>(title: &'static str, active: bool) -> impl Element<V> {
    Tab { title, active }
}

impl Tab {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        div()
            .px_2()
            .py_0p5()
            .flex()
            .items_center()
            .justify_center()
            .rounded_lg()
            .fill(if self.active {
                theme.highest.on.default.background
            } else {
                theme.highest.base.default.background
            })
            .hover()
            .fill(if self.active {
                theme.highest.on.hovered.background
            } else {
                theme.highest.base.hovered.background
            })
            .active()
            .fill(if self.active {
                theme.highest.on.pressed.background
            } else {
                theme.highest.base.pressed.background
            })
            .child(
                div()
                    .text_sm()
                    .text_color(if self.active {
                        theme.highest.base.default.foreground
                    } else {
                        theme.highest.variant.default.foreground
                    })
                    .child(self.title),
            )
    }
}
