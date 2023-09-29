use std::marker::PhantomData;

use crate::prelude::*;
use crate::{Buffer, Toolbar};

#[derive(Element)]
struct Editor<V: 'static> {
    view_type: PhantomData<V>,
    toolbar: Toolbar,
    buffer: Buffer<V>,
}

impl<V: 'static> Editor<V> {
    pub fn new(toolbar: Toolbar, buffer: Buffer<V>) -> Self {
        Self {
            view_type: PhantomData,
            toolbar,
            buffer,
        }
    }

    fn render(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        div().child(self.toolbar.clone())
    }
}
