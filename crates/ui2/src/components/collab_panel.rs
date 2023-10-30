use crate::prelude::*;
use crate::{
    static_collab_panel_channels, static_collab_panel_current_call, v_stack, Icon, List,
    ListHeader, ToggleState,
};

#[derive(Component)]
pub struct CollabPanel {
    id: ElementId,
}

impl CollabPanel {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into() }
    }

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
        let theme = theme(cx);

        v_stack()
            .id(self.id.clone())
            .h_full()
            .bg(theme.surface)
            .child(
                v_stack()
                    .id("crdb")
                    .w_full()
                    .overflow_y_scroll()
                    .child(
                        div().pb_1().border_color(theme.border).border_b().child(
                            List::new(static_collab_panel_current_call())
                                .header(
                                    ListHeader::new("CRDB")
                                        .left_icon(Icon::Hash.into())
                                        .toggle(ToggleState::Toggled),
                                )
                                .toggle(ToggleState::Toggled),
                        ),
                    )
                    .child(
                        v_stack().id("channels").py_1().child(
                            List::new(static_collab_panel_channels())
                                .header(ListHeader::new("CHANNELS").toggle(ToggleState::Toggled))
                                .empty_message("No channels yet. Add a channel to get started.")
                                .toggle(ToggleState::Toggled),
                        ),
                    )
                    .child(
                        v_stack().id("contacts-online").py_1().child(
                            List::new(static_collab_panel_current_call())
                                .header(
                                    ListHeader::new("CONTACTS – ONLINE")
                                        .toggle(ToggleState::Toggled),
                                )
                                .toggle(ToggleState::Toggled),
                        ),
                    )
                    .child(
                        v_stack().id("contacts-offline").py_1().child(
                            List::new(static_collab_panel_current_call())
                                .header(
                                    ListHeader::new("CONTACTS – OFFLINE")
                                        .toggle(ToggleState::NotToggled),
                                )
                                .toggle(ToggleState::NotToggled),
                        ),
                    ),
            )
            .child(
                div()
                    .h_7()
                    .px_2()
                    .border_t()
                    .border_color(theme.border)
                    .flex()
                    .items_center()
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.text_placeholder)
                            .child("Find..."),
                    ),
            )
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use super::*;
    use crate::Story;
    use gpui2::{Div, Render};

    pub struct CollabPanelStory;

    impl Render for CollabPanelStory {
        type Element = Div<Self>;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            Story::container(cx)
                .child(Story::title_for::<_, CollabPanel>(cx))
                .child(Story::label(cx, "Default"))
                .child(CollabPanel::new("collab-panel"))
        }
    }
}
