use std::marker::PhantomData;

use crate::prelude::*;
use crate::{
    static_project_panel_project_items, static_project_panel_single_items, theme, Input, List,
    ListHeader,
};

#[derive(Element)]
pub struct ProjectPanel<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
    scroll_state: ScrollState,
}

impl<S: 'static + Send + Sync + Clone> ProjectPanel<S> {
    pub fn new(scroll_state: ScrollState) -> Self {
        Self {
            state_type: PhantomData,
            scroll_state,
        }
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        let theme = theme(cx);

        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .px_2()
            .fill(theme.middle.base.default.background)
            .child(
                div()
                    .w_56()
                    .flex()
                    .flex_col()
                    .overflow_y_scroll(ScrollState::default())
                    .child(
                        List::new(static_project_panel_single_items())
                            .header(ListHeader::new("FILES").set_toggle(ToggleState::Toggled))
                            .empty_message("No files in directory")
                            .set_toggle(ToggleState::Toggled),
                    )
                    .child(
                        List::new(static_project_panel_project_items())
                            .header(ListHeader::new("PROJECT").set_toggle(ToggleState::Toggled))
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
