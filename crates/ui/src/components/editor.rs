use std::marker::PhantomData;

use crate::{hello_world_rust_buffer_with_status_example, prelude::*, v_stack, TabBar};
use crate::{static_tabs_example, Toolbar};

#[derive(Element)]
pub struct Editor<V: 'static> {
    view_type: PhantomData<V>,
    // toolbar: Toolbar,
    // buffer: Buffer<V>,
}

impl<V: 'static> Editor<V> {
    pub fn new(// toolbar: Toolbar, buffer: Buffer<V>
    ) -> Self {
        Self {
            view_type: PhantomData,
            // toolbar,
            // buffer,
        }
    }

    fn render(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        v_stack()
            .w_full()
            .h_full()
            .flex_1()
            .child(TabBar::new(static_tabs_example()))
            .child(Toolbar::new())
            .child(hello_world_rust_buffer_with_status_example(cx))
    }
}
