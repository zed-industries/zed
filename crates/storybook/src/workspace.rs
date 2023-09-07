use crate::{collab_panel::collab_panel, theme::theme};
use gpui2::{
    elements::{div, svg},
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
                            .px_1()
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
                            .px_1()
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
            .gap_4()
            .px_2()
            // === Comms === //
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
                                    .path("icons/microphone.svg")
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
                                    .path("icons/screen.svg")
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
                                    .path("icons/exit.svg")
                                    .w_4()
                                    .h_4()
                                    .fill(theme.lowest.base.default.foreground),
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
            .h_full()
            .w_full()
            .flex()
            .flex_col()
            .gap_y_0()
            .font("Zed Sans Extended")
            .text_color(theme.lowest.base.default.foreground)
            .fill(theme.middle.base.default.background)
            .child(titlebar())
            .child(collab_panel())
            .child(statusbar())
    }
}
