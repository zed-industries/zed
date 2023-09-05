use crate::theme::theme;
use gpui2::{elements::div, style::StyleHelpers, Element, IntoElement, ParentElement, ViewContext};
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

// #[derive(Element)]
// struct ListItem {
//     label: Option<ArcCow<'static, str>>,
// }

// pub fn list_item() -> ListItem {
//     ListItem { label: None }
// }

// impl ListItem {
//     pub fn render<V: 'static>(
//         &mut self,
//         view: &mut V,
//         cx: &mut ViewContext<Self>,
//     ) -> impl Element<V> {
//         let theme = theme(cx);

//         div()
//             .px_2()
//             .flex()
//             .justify_between()
//             .hover()
//             .fill(theme.middle.variant.hovered.background)
//             .active()
//             .fill(theme.middle.variant.pressed.background)
//             .child(div().flex().gap_1().child("#").child(self.label))
//             .child(div().flex().gap_1().child("v"))
//     }

//     pub fn label(mut self, label: impl Into<ArcCow<'static, str>>) -> Self {
//         self.label = Some(label.into());
//         self
//     }
// }

// fn list_item<V: 'static>(cx: &mut ViewContext<V>, label: &mut str) -> impl IntoElement<V> {
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

impl<V: 'static> CollabPanelElement<V> {
    fn render(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        // Panel
        div()
            .full()
            .flex()
            .flex_col()
            .font("Zed Sans Extended")
            .text_color(theme.middle.variant.default.foreground)
            // .border_color(theme.middle.base.default.border)
            .fill(theme.middle.base.default.background)
            // List Container
            .child(
                div()
                    .fill(theme.lowest.base.default.background)
                    .pb_1()
                    // .border_b()
                    //:: https://tailwindcss.com/docs/hover-focus-and-other-states#styling-based-on-parent-state
                    // .group()
                    // List Section Header
                    .child(
                        div()
                            .px_2()
                            .flex()
                            .justify_between()
                            .items_center()
                            // .hover()
                            // .fill(theme.middle.variant.hovered.background)
                            // .active()
                            // .fill(theme.middle.variant.pressed.background)
                            .child(
                                div()
                                    .flex()
                                    .gap_1()
                                    //:: State based on group interaction state
                                    // .group_hover().text_color(theme.middle.variant.hovered.foreground)
                                    .text_sm()
                                    .child("#")
                                    .child("CRDB"),
                            )
                            .child(
                                div().flex().h_full().gap_1().items_center().child(
                                    div()
                                        .w_3p5()
                                        .h_3p5()
                                        .fill(theme.middle.positive.default.foreground),
                                ),
                            ),
                    )
                    // List Item Large
                    .child(
                        div()
                            .px_2()
                            .h_7()
                            .flex()
                            .justify_between()
                            .items_center()
                            .child(
                                div()
                                    .flex()
                                    .h_full()
                                    .gap_1()
                                    .items_center()
                                    .text_sm()
                                    .child(
                                        div()
                                            .w_4()
                                            .h_4()
                                            .fill(theme.middle.negative.default.foreground),
                                    )
                                    .child("maxbrunsfeld"),
                            )
                            .child(
                                div().flex().h_full().gap_1().items_center().child(
                                    div()
                                        .w_3p5()
                                        .h_3p5()
                                        .fill(theme.middle.positive.default.foreground),
                                ),
                            ),
                    ),
            )
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_col()
                    .gap_y_1()
                    // List Section Header
                    .child(
                        div()
                            .h_7()
                            .px_2()
                            .flex()
                            .justify_between()
                            .items_center()
                            .child(div().flex().gap_1().text_sm().child("CHANNELS"))
                            .child(
                                div().flex().h_full().gap_1().items_center().child(
                                    div()
                                        .w_3p5()
                                        .h_3p5()
                                        .fill(theme.middle.positive.default.foreground),
                                ),
                            ),
                    ),
            )
            // Large List Item
            .child(
                div()
                    .h_7()
                    .px_2()
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(div().flex().gap_1().text_sm().child("CONTACTS"))
                    .child(
                        div().flex().h_full().gap_1().items_center().child(
                            div()
                                .w_3p5()
                                .h_3p5()
                                .fill(theme.middle.positive.default.foreground),
                        ),
                    ),
            )
    }
}
