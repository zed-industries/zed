use gpui2::elements::img;
use gpui2::ArcCow;

use crate::prelude::*;
use crate::theme;

#[derive(Element, Clone)]
pub struct Avatar {
    src: ArcCow<'static, str>,
    shape: Shape,
}

impl Avatar {
    pub fn new(src: impl Into<ArcCow<'static, str>>) -> Self {
        Self {
            src: src.into(),
            shape: Shape::Circle,
        }
    }

    pub fn shape(mut self, shape: Shape) -> Self {
        self.shape = shape;
        self
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
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
