use gpui2::elements::div;
use gpui2::{elements::div::ScrollState, ViewContext};
use gpui2::{Element, IntoElement, ParentElement};
use std::marker::PhantomData;

use crate::{example_editor_actions, OrderMethod, Palette};

#[derive(Element)]
pub struct CommandPalette<V: 'static> {
    view_type: PhantomData<V>,
    scroll_state: ScrollState,
}

impl<V: 'static> CommandPalette<V> {
    pub fn new(scroll_state: ScrollState) -> Self {
        Self {
            view_type: PhantomData,
            scroll_state,
        }
    }

    fn render(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        div().child(
            Palette::new(self.scroll_state.clone())
                .items(example_editor_actions())
                .placeholder("Execute a command...")
                .empty_string("No items found.")
                .default_order(OrderMethod::Ascending),
        )
    }
}
