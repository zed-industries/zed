use crate::theme::{theme, Theme};
use gpui2::{
    elements::{div, svg},
    style::{StyleHelpers, Styleable},
    ArcCow, Element, IntoElement, ParentElement, ViewContext,
};
use std::marker::PhantomData;

#[derive(Element)]
pub struct CollabPanelElement<V: 'static> {
    view_type: PhantomData<V>,
}

pub fn collab_panel<V: 'static>() -> CollabPanelElement<V> {
    CollabPanelElement {
        view_type: PhantomData,
    }
}

impl<V: 'static> CollabPanelElement<V> {
    fn render(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        // Panel
        div()
            .full()
            .flex()
            .flex_col()
            .font("Zed Sans Extended")
            .text_color(theme.middle.base.default.foreground)
            .border_color(theme.middle.base.default.border)
            .border()
            .fill(theme.middle.base.default.background)
            .child(
                div()
                    .full()
                    .flex()
                    .flex_col()
                    // List Container
                    .child(
                        div()
                            .fill(theme.lowest.base.default.background)
                            .pb_1()
                            .border_color(theme.lowest.base.default.border)
                            .border_b()
                            //:: https://tailwindcss.com/docs/hover-focus-and-other-states#styling-based-on-parent-state
                            // .group()
                            // List Section Header
                            .child(self.list_section_header("#CRDB", true, theme))
                            // List Item Large
                            .child(self.list_item("maxbrunsfeld", theme)),
                    )
                    .child(
                        div()
                            .py_2()
                            .flex()
                            .flex_col()
                            .child(self.list_section_header("CHANNELS", true, theme)),
                    )
                    .child(
                        div()
                            .py_2()
                            .flex()
                            .flex_col()
                            .child(self.list_section_header("CONTACTS", true, theme))
                            .child(self.list_item("as-cii", theme))
                            .child(self.list_item("nathansobo", theme))
                            .child(self.list_item("maxbrunsfeld", theme)),
                    ),
            )
            .child(
                div().h_7().px_2().flex().items_center().child(
                    div()
                        .text_sm()
                        .text_color(theme.middle.variant.default.foreground)
                        .child("Find..."),
                ),
            )
    }

    fn list_section_header(
        &self,
        label: impl Into<ArcCow<'static, str>>,
        expanded: bool,
        theme: &Theme,
    ) -> impl Element<V> {
        div()
            .h_7()
            .px_2()
            .flex()
            .justify_between()
            .items_center()
            .child(div().flex().gap_1().text_sm().child(label))
            .child(
                div().flex().h_full().gap_1().items_center().child(
                    svg()
                        .path(if expanded {
                            "icons/radix/caret-down.svg"
                        } else {
                            "icons/radix/caret-up.svg"
                        })
                        .w_3p5()
                        .h_3p5()
                        .fill(theme.middle.positive.default.foreground),
                ),
            )
    }

    fn list_item(&self, label: impl Into<ArcCow<'static, str>>, theme: &Theme) -> impl Element<V> {
        div()
            .h_7()
            .px_2()
            .flex()
            .items_center()
            .hover()
            .fill(theme.lowest.variant.hovered.background)
            .active()
            .fill(theme.lowest.variant.pressed.background)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .text_sm()
                    .child(
                        div()
                            .w_3p5()
                            .h_3p5()
                            .fill(theme.middle.positive.default.foreground),
                    )
                    .child(label),
            )
    }
}
