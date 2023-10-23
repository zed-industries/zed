use std::marker::PhantomData;

use crate::prelude::*;
use crate::{
    static_project_panel_project_items, static_project_panel_single_items, Input, List, ListHeader,
};

#[derive(Element)]
pub struct ProjectPanel<S: 'static + Send + Sync> {
    id: ElementId,
    state_type: PhantomData<S>,
}

impl<S: 'static + Send + Sync> ProjectPanel<S> {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            state_type: PhantomData,
        }
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let color = ThemeColor::new(cx);

        div()
            .id(self.id.clone())
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(color.surface)
            .child(
                div()
                    .id("project-panel-contents")
                    .w_full()
                    .flex()
                    .flex_col()
                    .overflow_y_scroll()
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

use gpui2::ElementId;
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

        fn render(
            &mut self,
            _view: &mut S,
            cx: &mut ViewContext<S>,
        ) -> impl Element<ViewState = S> {
            Story::container(cx)
                .child(Story::title_for::<_, ProjectPanel<S>>(cx))
                .child(Story::label(cx, "Default"))
                .child(
                    Panel::new("project-panel-outer", cx)
                        .child(ProjectPanel::new("project-panel-inner")),
                )
        }
    }
}
