use crate::prelude::*;
use crate::{
    static_project_panel_project_items, static_project_panel_single_items, Input, List, ListHeader,
};

#[derive(Component)]
pub struct ProjectPanel {
    id: ElementId,
}

impl ProjectPanel {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into() }
    }

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
        div()
            .id(self.id.clone())
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(cx.theme().colors().surface)
            .child(
                div()
                    .id("project-panel-contents")
                    .w_full()
                    .flex()
                    .flex_col()
                    .overflow_y_scroll()
                    .child(
                        List::new(static_project_panel_single_items())
                            .header(ListHeader::new("FILES").toggle(ToggleState::Toggled))
                            .empty_message("No files in directory")
                            .toggle(ToggleState::Toggled),
                    )
                    .child(
                        List::new(static_project_panel_project_items())
                            .header(ListHeader::new("PROJECT").toggle(ToggleState::Toggled))
                            .empty_message("No folders in directory")
                            .toggle(ToggleState::Toggled),
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
    use super::*;
    use crate::{Panel, Story};
    use gpui2::{Div, Render};

    pub struct ProjectPanelStory;

    impl Render for ProjectPanelStory {
        type Element = Div<Self>;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            Story::container(cx)
                .child(Story::title_for::<_, ProjectPanel>(cx))
                .child(Story::label(cx, "Default"))
                .child(
                    Panel::new("project-panel-outer", cx)
                        .child(ProjectPanel::new("project-panel-inner")),
                )
        }
    }
}
