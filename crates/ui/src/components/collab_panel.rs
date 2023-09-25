use crate::{
    h_stack, list, list_section_header, static_collab_panel_channels,
    static_collab_panel_current_call, static_project_panel_single_items,
    theme::{theme, Theme},
    v_stack, ToggleState,
};
use gpui2::{
    elements::{div, div::ScrollState, img, svg},
    style::{StyleHelpers, Styleable},
    ArcCow, Element, IntoElement, ParentElement, ViewContext,
};
use std::marker::PhantomData;

#[derive(Element)]
pub struct CollabPanelElement<V: 'static> {
    view_type: PhantomData<V>,
    scroll_state: ScrollState,
}

pub fn collab_panel<V: 'static>(scroll_state: ScrollState) -> CollabPanelElement<V> {
    CollabPanelElement {
        view_type: PhantomData,
        scroll_state,
    }
}

impl<V: 'static> CollabPanelElement<V> {
    fn render(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        v_stack()
            .w_64()
            .h_full()
            .fill(theme.middle.base.default.background)
            .child(
                v_stack()
                    .w_full()
                    .overflow_y_scroll(self.scroll_state.clone())
                    .child(
                        div()
                            .fill(theme.lowest.base.default.background)
                            .pb_1()
                            .border_color(theme.lowest.base.default.border)
                            .border_b()
                            .child(
                                list(static_collab_panel_current_call())
                                    .header(
                                        list_section_header("CURRENT CALL")
                                            .set_toggle(ToggleState::Toggled),
                                    )
                                    .set_toggle(ToggleState::Toggled),
                            ),
                    )
                    .child(
                        v_stack().py_2().child(
                            list(static_collab_panel_channels())
                                .header(
                                    list_section_header("CHANNELS")
                                        .set_toggle(ToggleState::Toggled),
                                )
                                .empty_message("No channels yet. Add a channel to get started.")
                                .set_toggle(ToggleState::Toggled),
                        ),
                    )
                    .child(
                        v_stack()
                            .py_2()
                            .child(self.list_section_header("CONTACTS", true, &theme))
                            .children(
                                std::iter::repeat_with(|| {
                                    vec![
                                        self.list_item(
                                            "http://github.com/as-cii.png?s=50",
                                            "as-cii",
                                            &theme,
                                        ),
                                        self.list_item(
                                            "http://github.com/nathansobo.png?s=50",
                                            "nathansobo",
                                            &theme,
                                        ),
                                        self.list_item(
                                            "http://github.com/maxbrunsfeld.png?s=50",
                                            "maxbrunsfeld",
                                            &theme,
                                        ),
                                    ]
                                })
                                .take(3)
                                .flatten(),
                            ),
                    ),
            )
            .child(
                div()
                    .h_7()
                    .px_2()
                    .border_t()
                    .border_color(theme.middle.variant.default.border)
                    .flex()
                    .items_center()
                    .child(
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
                            "icons/caret_down.svg"
                        } else {
                            "icons/caret_up.svg"
                        })
                        .w_3p5()
                        .h_3p5()
                        .fill(theme.middle.variant.default.foreground),
                ),
            )
    }

    fn list_item(
        &self,
        avatar_uri: impl Into<ArcCow<'static, str>>,
        label: impl Into<ArcCow<'static, str>>,
        theme: &Theme,
    ) -> impl Element<V> {
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
                        img()
                            .uri(avatar_uri)
                            .size_3p5()
                            .rounded_full()
                            .fill(theme.middle.positive.default.foreground),
                    )
                    .child(label),
            )
    }
}
