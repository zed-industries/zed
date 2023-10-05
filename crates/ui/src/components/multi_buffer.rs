use std::marker::PhantomData;

use crate::prelude::*;
use crate::{v_stack, Buffer};

#[derive(Element)]
pub struct MultiBuffer<V: 'static> {
    view_type: PhantomData<V>,
    buffers: Vec<Buffer>,
}

impl<V: 'static> MultiBuffer<V> {
    pub fn new(buffers: Vec<Buffer>) -> Self {
        Self {
            view_type: PhantomData,
            buffers,
        }
    }

    fn render(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        v_stack()
            .w_full()
            .h_full()
            .flex_1()
            .children(self.buffers.clone())
    }
}
