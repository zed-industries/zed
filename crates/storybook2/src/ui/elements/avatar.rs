use std::marker::PhantomData;

use gpui3::{img, ArcCow};

use crate::theme::theme;
use crate::ui::prelude::*;

#[derive(Element, Clone)]
pub struct Avatar<S: 'static + Send + Sync> {
    state_type: PhantomData<S>,
    src: ArcCow<'static, str>,
    shape: Shape,
}

impl<S: 'static + Send + Sync> Avatar<S> {
    pub fn new(src: impl Into<ArcCow<'static, str>>) -> Self {
        Self {
            state_type: PhantomData,
            src: src.into(),
            shape: Shape::Circle,
        }
    }

    pub fn shape(mut self, shape: Shape) -> Self {
        self.shape = shape;
        self
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        let theme = theme(cx);

        let mut img = img();

        if self.shape == Shape::Circle {
            img = img.rounded_full();
        } else {
            img = img.rounded_md();
        }

        img.uri(self.src.clone())
            .size_4()
            .fill(theme.middle.warning.default.foreground)
    }
}
