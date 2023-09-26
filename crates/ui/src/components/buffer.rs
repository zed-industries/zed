use std::marker::PhantomData;

use gpui2::elements::div;
use gpui2::elements::div::ScrollState;
use gpui2::{Element, IntoElement, ViewContext};

#[derive(Element)]
pub struct Buffer<V: 'static> {
    view_type: PhantomData<V>,
    scroll_state: ScrollState,
}

impl<V: 'static> Buffer<V> {
    pub fn new(scroll_state: ScrollState) -> Self {
        Self {
            view_type: PhantomData,
            scroll_state,
        }
    }

    fn render(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        div()
    }
}
