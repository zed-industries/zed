use gpui2::{
    elements::{div, div::ScrollState},
    style::StyleHelpers,
    Element, IntoElement, ParentElement, ViewContext,
};
use ui::{collab_panel, status_bar, tab_bar, theme, title_bar, ChatPanel, Toolbar};

#[derive(Element, Default)]
pub struct WorkspaceElement {
    left_scroll_state: ScrollState,
    right_scroll_state: ScrollState,
    tab_bar_scroll_state: ScrollState,
}

impl WorkspaceElement {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx).clone();

        div()
            .size_full()
            .flex()
            .flex_col()
            .font("Zed Sans Extended")
            .gap_0()
            .justify_start()
            .items_start()
            .text_color(theme.lowest.base.default.foreground)
            .fill(theme.lowest.base.default.background)
            .child(title_bar(cx))
            .child(
                div()
                    .flex_1()
                    .w_full()
                    .flex()
                    .flex_row()
                    .overflow_hidden()
                    .child(collab_panel(self.left_scroll_state.clone()))
                    .child(
                        div()
                            .h_full()
                            .flex_1()
                            .fill(theme.highest.base.default.background)
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .flex_1()
                                    .child(tab_bar(self.tab_bar_scroll_state.clone()))
                                    .child(Toolbar::new()),
                            ),
                    )
                    .child(ChatPanel::new(self.right_scroll_state.clone())),
            )
            .child(status_bar())
    }
}
