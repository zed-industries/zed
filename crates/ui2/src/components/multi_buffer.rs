use std::marker::PhantomData;

use crate::prelude::*;
use crate::{v_stack, Buffer, Icon, IconButton, Label, LabelSize};

#[derive(Element)]
pub struct MultiBuffer<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
    buffers: Vec<Buffer<S>>,
}

impl<S: 'static + Send + Sync + Clone> MultiBuffer<S> {
    pub fn new(buffers: Vec<Buffer<S>>) -> Self {
        Self {
            state_type: PhantomData,
            buffers,
        }
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        let theme = theme(cx);

        v_stack()
            .w_full()
            .h_full()
            .flex_1()
            .children(self.buffers.clone().into_iter().map(|buffer| {
                v_stack()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .p_4()
                            .fill(theme.lowest.base.default.background)
                            .child(Label::new("main.rs").size(LabelSize::Small))
                            .child(IconButton::new(Icon::ArrowUpRight)),
                    )
                    .child(buffer)
            }))
    }
}
