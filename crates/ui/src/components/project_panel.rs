use std::marker::PhantomData;
use std::sync::Arc;

use crate::{prelude::*, Panel, PanelSide, Theme};
use crate::{
    static_project_panel_project_items, static_project_panel_single_items, theme, Input, List,
    ListHeader,
};

#[derive(Element)]
pub struct ProjectPanel<V: 'static> {
    view_type: PhantomData<V>,
    scroll_state: ScrollState,
    current_side: PanelSide,
}

impl<V: 'static> ProjectPanel<V> {
    pub fn new(scroll_state: ScrollState) -> Self {
        Self {
            view_type: PhantomData,
            scroll_state,
            current_side: PanelSide::default(),
        }
    }

    pub fn side(mut self, side: PanelSide) -> Self {
        self.current_side = side;
        self
    }

    fn render(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        struct PanelPayload {
            pub theme: Arc<Theme>,
            pub scroll_state: ScrollState,
        }

        Panel::new(
            self.scroll_state.clone(),
            |_, payload| {
                let payload = payload.downcast_ref::<PanelPayload>().unwrap();

                let theme = payload.theme.clone();

                vec![div()
                    .flex()
                    .flex_col()
                    .w_56()
                    .h_full()
                    .px_2()
                    .fill(theme.middle.base.default.background)
                    .child(
                        div()
                            .w_56()
                            .flex()
                            .flex_col()
                            .overflow_y_scroll(payload.scroll_state.clone())
                            .child(
                                List::new(static_project_panel_single_items())
                                    .header(
                                        ListHeader::new("FILES").set_toggle(ToggleState::Toggled),
                                    )
                                    .empty_message("No files in directory")
                                    .set_toggle(ToggleState::Toggled),
                            )
                            .child(
                                List::new(static_project_panel_project_items())
                                    .header(
                                        ListHeader::new("PROJECT").set_toggle(ToggleState::Toggled),
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
                    .into_any()]
            },
            Box::new(PanelPayload {
                theme: theme(cx),
                scroll_state: self.scroll_state.clone(),
            }),
        )
    }
}
