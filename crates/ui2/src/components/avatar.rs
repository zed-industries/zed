use crate::prelude::*;
use gpui::{img, ImageSource, Img, RenderOnce};

#[derive(Debug, Default, PartialEq, Clone)]
pub enum Shape {
    #[default]
    Circle,
    RoundedRectangle,
}

#[derive(RenderOnce)]
pub struct Avatar {
    src: ImageSource,
    shape: Shape,
}

impl Component for Avatar {
    type Rendered = Img;

    fn render(self, _: &mut WindowContext) -> Self::Rendered {
        let mut img = img();

        if self.shape == Shape::Circle {
            img = img.rounded_full();
        } else {
            img = img.rounded_md();
        }

        img.source(self.src.clone())
            .size_4()
            // todo!(Pull the avatar fallback background from the theme.)
            .bg(gpui::red())
    }
}

impl Avatar {
    pub fn new(src: impl Into<ImageSource>) -> Self {
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
