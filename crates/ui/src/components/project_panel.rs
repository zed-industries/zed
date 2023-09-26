use crate::prelude::*;
use crate::{
    static_project_panel_project_items, static_project_panel_single_items, theme, Input, List,
    ListSectionHeader,
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

impl<V: 'static> ProjectPanel<V> {
    pub fn new(scroll_state: ScrollState) -> Self {
        Self {
            view_type: PhantomData,
            scroll_state,
        }
    }

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
                        List::new(static_project_panel_single_items())
                            .header(
                                ListSectionHeader::new("FILES").set_toggle(ToggleState::Toggled),
                            )
                            .empty_message("No files in directory")
                            .set_toggle(ToggleState::Toggled),
                    )
                    .child(
                        List::new(static_project_panel_project_items())
                            .header(
                                ListSectionHeader::new("PROJECT").set_toggle(ToggleState::Toggled),
                            )
                            .empty_message("No folders in directory")
                            .set_toggle(ToggleState::Toggled),
                    ),
            )
            .child(
                Input::new("Find something...")
                    .value("buffe".to_string())
                    .state(InteractionState::Focused),
            )
    }
}
