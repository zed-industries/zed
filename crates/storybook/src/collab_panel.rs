use crate::theme::theme;
use gpui2::{
    elements::div,
    style::{StyleHelpers, Styleable},
    Element, IntoElement, ParentElement, ViewContext,
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

// fn list_item<V: 'static>(cx: &mut ViewContext<V>) -> impl IntoElement<V> {
//     let theme = theme(cx);
//     div()
//         .px_2()
//         .flex()
//         .justify_between()
//         //:: States - https://tailwindcss.com/docs/hover-focus-and-other-states#hover-focus-and-active
//         .hover()
//         .fill(theme.middle.variant.hovered.background)
//         .active()
//         .fill(theme.middle.variant.pressed.background)
//         .child(div().flex().gap_1().child("#").child("Collab Panel"))
//         .child(div().flex().gap_1().child("v"))
// }

// Macros to impl:
// - border (b, t, l, r, x, y)
// - border_[foo]_[size] (border_b_2, border_t_4, etc)
// - border_color
// - items_[center, start, end]

impl<V: 'static> CollabPanelElement<V> {
    fn render(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        // Panel
        div()
            .full()
            .font("Zed Sans")
            .text_color(theme.middle.variant.default.foreground)
            // .border_color(theme.middle.base.default.border)
            .fill(theme.middle.base.default.background)
            // List Container
            .child(
                div()
                    .full()
                    .fill(theme.middle.variant.default.background)
                    .py_2()
                    // .border_b()
                    //:: https://tailwindcss.com/docs/hover-focus-and-other-states#styling-based-on-parent-state
                    // .group()
                    // List Section Header
                    .child(
                        div()
                            .px_2()
                            .flex()
                            .justify_between()
                            //:: States - https://tailwindcss.com/docs/hover-focus-and-other-states#hover-focus-and-active
                            .hover()
                            .fill(theme.middle.variant.hovered.background)
                            // .focus().fill(theme.middle.variant.active.background)
                            .active()
                            .fill(theme.middle.variant.pressed.background)
                            .child(
                                div()
                                    .flex()
                                    .gap_1()
                                    //:: State based on group interaction state
                                    // .group_hover().text_color(theme.middle.variant.hovered.foreground)
                                    .child("#")
                                    .child("Collab Panel"),
                            )
                            .child(div().flex().gap_1().child("v")),
                    )
                    // List Item Large
                    .child(
                        div()
                            .px_2()
                            .h_7()
                            .flex()
                            .justify_between()
                            // .items_center()
                            .child(
                                div()
                                    .flex()
                                    .gap_1()
                                    .child(div().w_4().h_4().child("img"))
                                    .child("maxbrunsfeld"),
                            )
                            .child(div().flex().gap_2().child("icon")),
                    ),
            )
    }
}
