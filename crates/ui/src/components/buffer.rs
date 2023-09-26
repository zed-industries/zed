use gpui2::{
    elements::{div, div::ScrollState},
    ViewContext,
};
use gpui2::{Element, IntoElement};
use std::marker::PhantomData;

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
