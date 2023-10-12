use crate::{
    collab_panel::{collab_panel, CollabPanel},
    themes::rose_pine,
};
use gpui3::{
    div, img, svg, view, Context, Element, ParentElement, RootView, StyleHelpers, Styled, View,
    ViewContext, WindowContext,
};
use ui::{theme, themed};

pub struct Workspace {
    left_panel: View<CollabPanel, Self>,
    right_panel: View<CollabPanel, Self>,
}

pub fn workspace(cx: &mut WindowContext) -> RootView<Workspace> {
    view(cx.entity(|cx| Workspace::new(cx)), Workspace::render)
}

impl Workspace {
    fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            left_panel: collab_panel(cx),
            right_panel: collab_panel(cx),
        }
    }

    fn hover_test(&self, cx: &mut ViewContext<Self>) -> impl Element<ViewState = Self> {
        let theme = theme(cx);

        div().size_full().child(
            div()
                .group("")
                .w_full()
                .h_5()
                .mt_10()
                .fill(theme.middle.warning.default.foreground)
                .flex()
                .flex_row()
                .justify_center()
                .child(
                    div()
                        .size_5()
                        .fill(theme.middle.negative.default.foreground)
                        .group_hover("")
                        .fill(theme.middle.positive.default.foreground)
                        .hover()
                        .fill(theme.middle.variant.default.foreground),
                ),
        )
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl Element<ViewState = Self> {
        themed(rose_pine(), cx, |cx| {
            // self.hover_test(cx)
            let theme = theme(cx);
            div()
                .size_full()
                .flex()
                .flex_col()
                .font("Courier")
                .gap_0()
                .justify_start()
                .items_start()
                .text_color(theme.lowest.base.default.foreground)
                .fill(theme.middle.base.default.background)
                .child(titlebar(cx))
                .child(
                    div()
                        .flex_1()
                        .w_full()
                        .flex()
                        .flex_row()
                        .overflow_hidden()
                        .child(self.left_panel.clone())
                        .child(div().h_full().flex_1()), // .child(self.right_panel.clone()),
                )
                .child(statusbar::statusbar(cx))
        })
    }
}

struct Titlebar;

pub fn titlebar<S: 'static + Send + Sync>(cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
    let ref mut this = Titlebar;
    let theme = theme(cx);
    div()
        .flex()
        .items_center()
        .justify_between()
        .w_full()
        .h_8()
        .fill(theme.lowest.base.default.background)
        .child(this.left_group(cx))
        .child(this.right_group(cx))
}

impl Titlebar {
    fn render<V: 'static + Send + Sync>(
        &mut self,
        cx: &mut ViewContext<V>,
    ) -> impl Element<ViewState = V> {
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

    fn left_group<S: 'static + Send + Sync>(
        &mut self,
        cx: &mut ViewContext<S>,
    ) -> impl Element<ViewState = S> {
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
                            // .hover()
                            // .fill(theme.lowest.base.hovered.background)
                            // .active()
                            // .fill(theme.lowest.base.pressed.background)
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
                            // .hover()
                            // .fill(theme.lowest.base.hovered.background)
                            // .active()
                            // .fill(theme.lowest.base.pressed.background)
                            .child(
                                div()
                                    .text_sm()
                                    .text_decoration_1()
                                    .text_decoration_wavy()
                                    .child("branch"),
                            ),
                    ),
            )
    }

    fn right_group<S: 'static + Send + Sync>(
        &mut self,
        cx: &mut ViewContext<S>,
    ) -> impl Element<ViewState = S> {
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
                                // .hover()
                                // .fill(theme.lowest.base.hovered.background)
                                // .active()
                                // .fill(theme.lowest.base.pressed.background)
                                .child(
                                    svg()
                                        .path("icons/mic.svg")
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
                                // .hover()
                                // .fill(theme.lowest.base.hovered.background)
                                // .active()
                                // .fill(theme.lowest.base.pressed.background)
                                .child(
                                    svg()
                                        .path("icons/speaker-loud.svg")
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
                                // .hover()
                                // .fill(theme.lowest.base.hovered.background)
                                // .active()
                                // .fill(theme.lowest.base.pressed.background)
                                .child(
                                    svg()
                                        .path("icons/desktop.svg")
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
                        // .hover()
                        // .fill(theme.lowest.base.hovered.background)
                        // .active()
                        // .fill(theme.lowest.base.pressed.background)
                        .child(
                            img()
                                .uri("https://avatars.githubusercontent.com/u/1714999?v=4")
                                .size_4()
                                .rounded_md()
                                .fill(theme.middle.on.default.foreground),
                        )
                        .child(
                            svg()
                                .path("icons/caret_down.svg")
                                .w_2()
                                .h_2()
                                .fill(theme.lowest.variant.default.foreground),
                        ),
                ),
            )
    }
}

// ================================================================================ //

mod statusbar {

    use super::*;

    pub fn statusbar<S: 'static + Send + Sync>(
        cx: &mut ViewContext<S>,
    ) -> impl Element<ViewState = S> {
        let theme = theme(cx);
        div()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .h_8()
            .fill(theme.lowest.base.default.background)
        // .child(left_group(cx))
        // .child(right_group(cx))
    }

    fn left_group<V: 'static + Send + Sync>(
        cx: &mut ViewContext<V>,
    ) -> impl Element<ViewState = V> {
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
                            // .hover()
                            // .fill(theme.lowest.base.hovered.background)
                            // .active()
                            // .fill(theme.lowest.base.pressed.background)
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

    fn right_group<S: 'static + Send + Sync>(
        cx: &mut ViewContext<S>,
    ) -> impl Element<ViewState = S> {
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
