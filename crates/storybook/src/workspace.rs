use crate::{collab_panel::collab_panel, theme::theme};
use gpui2::{
    elements::{div, img, svg},
    style::{StyleHelpers, Styleable},
    Element, IntoElement, ParentElement, ViewContext,
};

#[derive(Element)]
struct TitleBar;

pub fn titlebar<V: 'static>() -> impl Element<V> {
    TitleBar
}

impl TitleBar {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);
        div()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .h_8()
            .fill(theme.lowest.base.default.background)
            .child(self.left_group(cx))
            .child(self.right_group(cx))
    }

    fn left_group<V: 'static>(&mut self, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);
        div()
            .flex()
            .items_center()
            .h_full()
            .gap_4()
            .px_2()
            // === Traffic Lights === //
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .w_3()
                            .h_3()
                            .rounded_full()
                            .fill(theme.lowest.positive.default.foreground),
                    )
                    .child(
                        div()
                            .w_3()
                            .h_3()
                            .rounded_full()
                            .fill(theme.lowest.warning.default.foreground),
                    )
                    .child(
                        div()
                            .w_3()
                            .h_3()
                            .rounded_full()
                            .fill(theme.lowest.negative.default.foreground),
                    ),
            )
            // === Project Info === //
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .child(
                        div()
                            .h_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .px_2()
                            .rounded_md()
                            .hover()
                            .fill(theme.lowest.base.hovered.background)
                            .active()
                            .fill(theme.lowest.base.pressed.background)
                            .child(div().text_sm().child("project")),
                    )
                    .child(
                        div()
                            .h_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .px_2()
                            .rounded_md()
                            .text_color(theme.lowest.variant.default.foreground)
                            .hover()
                            .fill(theme.lowest.base.hovered.background)
                            .active()
                            .fill(theme.lowest.base.pressed.background)
                            .child(div().text_sm().child("branch")),
                    ),
            )
    }

    fn right_group<V: 'static>(&mut self, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);
        div()
            .flex()
            .items_center()
            .h_full()
            .gap_3()
            .px_2()
            // === Actions === //
            .child(
                div().child(
                    div().flex().items_center().gap_1().child(
                        div().size_4().flex().items_center().justify_center().child(
                            svg()
                                .path("icons/exit.svg")
                                .size_4()
                                .fill(theme.lowest.base.default.foreground),
                        ),
                    ),
                ),
            )
            .child(div().w_px().h_3().fill(theme.lowest.base.default.border))
            // === Comms === //
            .child(
                div().child(
                    div()
                        .flex()
                        .items_center()
                        .gap_px()
                        .child(
                            div()
                                .px_2()
                                .py_1()
                                .rounded_md()
                                .h_full()
                                .flex()
                                .items_center()
                                .justify_center()
                                .hover()
                                .fill(theme.lowest.base.hovered.background)
                                .active()
                                .fill(theme.lowest.base.pressed.background)
                                .child(
                                    svg()
                                        .path("icons/microphone.svg")
                                        .size_3p5()
                                        .fill(theme.lowest.base.default.foreground),
                                ),
                        )
                        .child(
                            div()
                                .px_2()
                                .py_1()
                                .rounded_md()
                                .h_full()
                                .flex()
                                .items_center()
                                .justify_center()
                                .hover()
                                .fill(theme.lowest.base.hovered.background)
                                .active()
                                .fill(theme.lowest.base.pressed.background)
                                .child(
                                    svg()
                                        .path("icons/radix/speaker-loud.svg")
                                        .size_3p5()
                                        .fill(theme.lowest.base.default.foreground),
                                ),
                        )
                        .child(
                            div()
                                .px_2()
                                .py_1()
                                .rounded_md()
                                .h_full()
                                .flex()
                                .items_center()
                                .justify_center()
                                .hover()
                                .fill(theme.lowest.base.hovered.background)
                                .active()
                                .fill(theme.lowest.base.pressed.background)
                                .child(
                                    svg()
                                        .path("icons/radix/desktop.svg")
                                        .size_3p5()
                                        .fill(theme.lowest.base.default.foreground),
                                ),
                        ),
                ),
            )
            .child(div().w_px().h_3().fill(theme.lowest.base.default.border))
            // User Group
            .child(
                div().child(
                    div()
                        .px_1()
                        .py_1()
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded_md()
                        .gap_0p5()
                        .hover()
                        .fill(theme.lowest.base.hovered.background)
                        .active()
                        .fill(theme.lowest.base.pressed.background)
                        .child(
                            img()
                                .uri("https://avatars.githubusercontent.com/u/1714999?v=4")
                                .size_4()
                                .rounded_md()
                                .fill(theme.middle.on.default.foreground),
                        )
                        .child(
                            svg()
                                .path("icons/caret_down_8.svg")
                                .w_2()
                                .h_2()
                                .fill(theme.lowest.variant.default.foreground),
                        ),
                ),
            )
    }
}

// ================================================================================ //

#[derive(Element)]
struct StatusBar;

pub fn statusbar<V: 'static>() -> impl Element<V> {
    StatusBar
}

impl StatusBar {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);
        div()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .h_8()
            .fill(theme.lowest.base.default.background)
            .child(self.left_group(cx))
            .child(self.right_group(cx))
    }

    fn left_group<V: 'static>(&mut self, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);
        div()
            .flex()
            .items_center()
            .h_full()
            .gap_4()
            .px_2()
            // === Tools === //
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .child(
                        div()
                            .w_6()
                            .h_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                svg()
                                    .path("icons/project.svg")
                                    .w_4()
                                    .h_4()
                                    .fill(theme.lowest.base.default.foreground),
                            ),
                    )
                    .child(
                        div()
                            .w_6()
                            .h_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                svg()
                                    .path("icons/conversations.svg")
                                    .w_4()
                                    .h_4()
                                    .fill(theme.lowest.base.default.foreground),
                            ),
                    )
                    .child(
                        div()
                            .w_6()
                            .h_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                svg()
                                    .path("icons/file_icons/notebook.svg")
                                    .w_4()
                                    .h_4()
                                    .fill(theme.lowest.accent.default.foreground),
                            ),
                    ),
            )
            // === Diagnostics === //
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .h_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .gap_0p5()
                            .px_1()
                            .text_color(theme.lowest.variant.default.foreground)
                            .hover()
                            .fill(theme.lowest.base.hovered.background)
                            .active()
                            .fill(theme.lowest.base.pressed.background)
                            .child(
                                svg()
                                    .path("icons/error.svg")
                                    .w_4()
                                    .h_4()
                                    .fill(theme.lowest.negative.default.foreground),
                            )
                            .child(div().text_sm().child("2")),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.lowest.variant.default.foreground)
                            .child("Something is wrong"),
                    ),
            )
    }

    fn right_group<V: 'static>(&mut self, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);
        div()
            .flex()
            .items_center()
            .h_full()
            .gap_4()
            .px_2()
            // === Tools === //
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .child(
                        div()
                            .w_6()
                            .h_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                svg()
                                    .path("icons/check_circle.svg")
                                    .w_4()
                                    .h_4()
                                    .fill(theme.lowest.base.default.foreground),
                            ),
                    )
                    .child(
                        div()
                            .w_6()
                            .h_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                svg()
                                    .path("icons/copilot.svg")
                                    .w_4()
                                    .h_4()
                                    .fill(theme.lowest.accent.default.foreground),
                            ),
                    ),
            )
    }
}

// ================================================================================ //

#[derive(Element)]
struct WorkspaceElement;

pub fn workspace<V: 'static>() -> impl Element<V> {
    WorkspaceElement
}

impl WorkspaceElement {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);
        div()
            .size_full()
            .flex()
            .flex_col()
            .font("Zed Sans Extended")
            .gap_0()
            .justify_start()
            .items_start()
            .text_color(theme.lowest.base.default.foreground)
            .fill(theme.middle.warning.default.background)
            .child(titlebar())
            .child(collab_panel())
            .child(statusbar())
    }
}

// Hover over things
// Paint its space... padding, margin, border, content

/*
* h_8, grow_0/flex_grow_0, shrink_0/flex_shrink_0
* flex_grow
* h_8, grow_0/flex_grow_0, shrink_0/flex_shrink_0
*/
