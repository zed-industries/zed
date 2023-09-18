use crate::{
    prelude::InteractionState,
    theme::theme,
    ui::{input, label, LabelColor},
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
            .w_64()
            .h_full()
            .flex()
            .flex_col()
            .fill(theme.middle.base.default.background)
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_col()
                    .overflow_y_scroll(self.scroll_state.clone())
                    .child(
                        div().py_2().flex().flex_col().children(
                            std::iter::repeat_with(|| {
                                vec![
                                    label("File"),
                                    label("Modified File").color(LabelColor::Modified),
                                    label("Created File").color(LabelColor::Created),
                                    label("Deleted File").color(LabelColor::Deleted),
                                    label("Hidden File").color(LabelColor::Hidden),
                                ]
                            })
                            .take(60)
                            .flatten(),
                        ),
                    ),
            )
            .child(
                input("Find something...")
                    .value("buffe".to_string())
                    .state(InteractionState::Focused),
            )
    }
}
