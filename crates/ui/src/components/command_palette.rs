use gpui2::elements::div;
use gpui2::{elements::div::ScrollState, ViewContext};
use gpui2::{Element, IntoElement, ParentElement};
use std::marker::PhantomData;

use crate::{example_editor_actions, palette, OrderMethod};

#[derive(Element)]
pub struct CommandPalette<V: 'static> {
    view_type: PhantomData<V>,
    scroll_state: ScrollState,
}

pub fn command_palette<V: 'static>(scroll_state: ScrollState) -> CommandPalette<V> {
    CommandPalette {
        view_type: PhantomData,
        scroll_state,
    }
}

impl<V: 'static> CommandPalette<V> {
    fn render(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        div().child(
            palette(self.scroll_state.clone())
                .items(example_editor_actions())
                .placeholder("Execute a command...")
                .empty_string("No items found.")
                .default_order(OrderMethod::Ascending),
        )
    }
}
