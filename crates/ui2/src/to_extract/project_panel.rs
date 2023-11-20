use crate::{
    prelude::*, static_project_panel_project_items, static_project_panel_single_items, Input, List,
    ListHeader,
};
use gpui::prelude::*;
use gpui::Div;
use gpui::Stateful;

#[derive(RenderOnce)]
pub struct ProjectPanel {
    id: ElementId,
}

impl Component for ProjectPanel {
    type Rendered = gpui::Stateful<Div>;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        div()
            .id(self.id.clone())
            .flex()
            .flex_col()
            .size_full()
            .bg(cx.theme().colors().surface_background)
            .child(
                div()
                    .id("project-panel-contents")
                    .w_full()
                    .flex()
                    .flex_col()
                    .overflow_y_scroll()
                    .child(
                        List::new()
                            .header(ListHeader::new("FILES"))
                            .empty_message("No files in directory")
                            .children(static_project_panel_single_items()),
                    )
                    .child(
                        List::new()
                            .header(ListHeader::new("PROJECT"))
                            .empty_message("No folders in directory")
                            .children(static_project_panel_project_items()),
                    ),
            )
            .child(
                Input::new("Find something...")
                    .value("buffe".to_string())
                    .state(InteractionState::Focused),
            )
    }
}

impl ProjectPanel {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into() }
    }

    fn render(self, cx: &mut WindowContext) -> impl Element {
        div()
            .id(self.id.clone())
            .flex()
            .flex_col()
            .size_full()
            .bg(cx.theme().colors().surface_background)
            .child(
                div()
                    .id("project-panel-contents")
                    .w_full()
                    .flex()
                    .flex_col()
                    .overflow_y_scroll()
                    .child(
                        List::new()
                            .header(ListHeader::new("FILES"))
                            .empty_message("No files in directory")
                            .children(static_project_panel_single_items()),
                    )
                    .child(
                        List::new()
                            .header(ListHeader::new("PROJECT"))
                            .empty_message("No folders in directory")
                            .children(static_project_panel_project_items()),
                    ),
            )
            .child(
                Input::new("Find something...")
                    .value("buffe".to_string())
                    .state(InteractionState::Focused),
            )
    }
}

use gpui::ElementId;
#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use super::*;
    use crate::{Panel, Story};
    use gpui::{Div, Render};

    pub struct ProjectPanelStory;

    impl Render for ProjectPanelStory {
        type Element = Div;

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
