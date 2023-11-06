use crate::{prelude::*, Toggle};
use crate::{
    static_collab_panel_channels, static_collab_panel_current_call, v_stack, Icon, List, ListHeader,
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
        v_stack()
            .id(self.id.clone())
            .h_full()
            .bg(cx.theme().colors().surface_background)
            .child(
                v_stack()
                    .id("crdb")
                    .w_full()
                    .overflow_y_scroll()
                    .child(
                        div()
                            .pb_1()
                            .border_color(cx.theme().colors().border)
                            .border_b()
                            .child(
                                List::new(static_collab_panel_current_call())
                                    .header(
                                        ListHeader::new("CRDB")
                                            .left_icon(Icon::Hash.into())
                                            .toggle(Toggle::Toggled(true)),
                                    )
                                    .toggle(Toggle::Toggled(true)),
                            ),
                    )
                    .child(
                        v_stack().id("channels").py_1().child(
                            List::new(static_collab_panel_channels())
                                .header(ListHeader::new("CHANNELS").toggle(Toggle::Toggled(true)))
                                .empty_message("No channels yet. Add a channel to get started.")
                                .toggle(Toggle::Toggled(true)),
                        ),
                    )
                    .child(
                        v_stack().id("contacts-online").py_1().child(
                            List::new(static_collab_panel_current_call())
                                .header(
                                    ListHeader::new("CONTACTS – ONLINE")
                                        .toggle(Toggle::Toggled(true)),
                                )
                                .toggle(Toggle::Toggled(true)),
                        ),
                    )
                    .child(
                        v_stack().id("contacts-offline").py_1().child(
                            List::new(static_collab_panel_current_call())
                                .header(
                                    ListHeader::new("CONTACTS – OFFLINE")
                                        .toggle(Toggle::Toggled(false)),
                                )
                                .toggle(Toggle::Toggled(false)),
                        ),
                    ),
            )
            .child(
                div()
                    .h_7()
                    .px_2()
                    .border_t()
                    .border_color(cx.theme().colors().border)
                    .flex()
                    .items_center()
                    .child(
                        div()
                            .text_sm()
                            .text_color(cx.theme().colors().text_placeholder)
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
