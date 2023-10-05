use std::marker::PhantomData;

use crate::prelude::*;
use crate::{v_stack, Buffer, Icon, IconElement, IconSize, Label, LabelSize};

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
                            .h_8()
                            .px_4()
                            .py_2()
                            .fill(theme.lowest.base.default.background)
                            .child(Label::new("main.rs").size(LabelSize::Small))
                            .child(IconElement::new(Icon::ArrowUpRight).size(IconSize::Small)),
                    )
                    .child(buffer)
            }))
    }
}
