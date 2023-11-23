use crate::prelude::*;
use gpui::{img, Img, IntoElement};

#[derive(Debug, Default, PartialEq, Clone)]
pub enum Shape {
    #[default]
    Circle,
    RoundedRectangle,
}

#[derive(IntoElement)]
pub struct Avatar {
    src: SharedString,
    shape: Shape,
}

impl RenderOnce for Avatar {
    type Rendered = Img;

    fn render(self, _: &mut WindowContext) -> Self::Rendered {
        let mut img = img();

        if self.shape == Shape::Circle {
            img = img.rounded_full();
        } else {
            img = img.rounded_md();
        }

        img.uri(self.src.clone())
            .size_4()
            // todo!(Pull the avatar fallback background from the theme.)
            .bg(gpui::red())
    }
}

impl Avatar {
    pub fn new(src: impl Into<SharedString>) -> Self {
        Self {
            src: src.into(),
            shape: Shape::Circle,
        }
    }

    pub fn shape(mut self, shape: Shape) -> Self {
        self.shape = shape;
        self
    }
}
