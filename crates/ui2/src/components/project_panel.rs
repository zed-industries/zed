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

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let theme = theme(cx);
        let color = ThemeColor::new(cx);

        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .fill(color.surface)
            .child(
                div()
                    .w_full()
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

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use crate::{Panel, Story};

    use super::*;

    #[derive(Element)]
    pub struct ProjectPanelStory<S: 'static + Send + Sync + Clone> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync + Clone> ProjectPanelStory<S> {
        pub fn new() -> Self {
            Self {
                state_type: PhantomData,
            }
        }

        fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
            Story::container(cx)
                .child(Story::title_for::<_, ProjectPanel<S>>(cx))
                .child(Story::label(cx, "Default"))
                .child(Panel::new(
                    ScrollState::default(),
                    |_, _| vec![ProjectPanel::new(ScrollState::default()).into_any()],
                    Box::new(()),
                ))
        }
    }
}
