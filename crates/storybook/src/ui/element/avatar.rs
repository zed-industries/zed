use crate::prelude::Shape;
use crate::theme::theme;
use gpui2::elements::img;
use gpui2::style::StyleHelpers;
use gpui2::{ArcCow, IntoElement};
use gpui2::{Element, ViewContext};

pub type UnknownString = ArcCow<'static, str>;

#[derive(Element, Clone)]
pub struct Avatar {
    src: ArcCow<'static, str>,
    shape: Shape,
}

pub fn avatar(src: impl Into<ArcCow<'static, str>>) -> Avatar {
    Avatar {
        src: src.into(),
        shape: Shape::Circle,
    }
}

impl Avatar {
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
