use crate::{
    input, list, list_section_header, prelude::*, static_project_panel_project_items,
    static_project_panel_single_items, theme,
};

use gpui2::{
    elements::{div, div::ScrollState},
    style::StyleHelpers,
    ParentElement, ViewContext,
};
use gpui2::{Element, IntoElement};
use std::marker::PhantomData;

#[derive(Element)]
pub struct ProjectPanel<V: 'static> {
    view_type: PhantomData<V>,
    scroll_state: ScrollState,
}

pub fn project_panel<V: 'static>(scroll_state: ScrollState) -> ProjectPanel<V> {
    ProjectPanel {
        view_type: PhantomData,
        scroll_state,
    }
}

impl<V: 'static> ProjectPanel<V> {
    fn render(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        div()
            .w_56()
            .h_full()
            .flex()
            .flex_col()
            .fill(theme.middle.base.default.background)
            .child(
                div()
                    .w_56()
                    .flex()
                    .flex_col()
                    .overflow_y_scroll(self.scroll_state.clone())
                    .child(
                        list(static_project_panel_single_items())
                            .header(list_section_header("FILES").set_toggle(ToggleState::Toggled))
                            .empty_message("No files in directory")
                            .set_toggle(ToggleState::Toggled),
                    )
                    .child(
                        list(static_project_panel_project_items())
                            .header(list_section_header("PROJECT").set_toggle(ToggleState::Toggled))
                            .empty_message("No folders in directory")
                            .set_toggle(ToggleState::Toggled),
                    ),
            )
            .child(
                input("Find something...")
                    .value("buffe".to_string())
                    .state(InteractionState::Focused),
            )
    }
}
