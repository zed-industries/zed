use crate::{chat_panel, collab_panel, project_panel, status_bar, tab_bar, theme, title_bar};

use gpui2::{
    elements::{div, div::ScrollState},
    style::StyleHelpers,
    Element, IntoElement, ParentElement, ViewContext,
};

#[derive(Element, Default)]
struct WorkspaceElement {
    project_panel_scroll_state: ScrollState,
    collab_panel_scroll_state: ScrollState,
    right_scroll_state: ScrollState,
    tab_bar_scroll_state: ScrollState,
    palette_scroll_state: ScrollState,
}

pub fn workspace<V: 'static>() -> impl Element<V> {
    WorkspaceElement::default()
}

impl WorkspaceElement {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        div()
            // Elevation Level 0
            .size_full()
            .flex()
            .flex_col()
            .font("Zed Sans Extended")
            .gap_0()
            .justify_start()
            .items_start()
            .text_color(theme.lowest.base.default.foreground)
            .fill(theme.lowest.base.default.background)
            .relative()
            // Elevation Level 1
            .child(title_bar())
            .child(
                div()
                    .flex_1()
                    .w_full()
                    .flex()
                    .flex_row()
                    .overflow_hidden()
                    .child(project_panel(self.project_panel_scroll_state.clone()))
                    .child(collab_panel(self.collab_panel_scroll_state.clone()))
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
                                    .child(tab_bar(self.tab_bar_scroll_state.clone())),
                            ),
                    )
                    .child(chat_panel(self.right_scroll_state.clone())),
            )
            .child(status_bar())
        // Elevation Level 3
        // .child(
        //     div()
        //         .absolute()
        //         .top_0()
        //         .left_0()
        //         .size_full()
        //         .flex()
        //         .justify_center()
        //         .items_center()
        //         // .fill(theme.lowest.base.default.background)
        //         // Elevation Level 4
        //         .child(command_palette(self.palette_scroll_state.clone())),
        // )
    }
}
