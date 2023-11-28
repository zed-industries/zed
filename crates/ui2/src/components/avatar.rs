use std::sync::Arc;

use crate::prelude::*;
use gpui::{img, ImageData, ImageSource, Img, IntoElement};

#[derive(Debug, Default, PartialEq, Clone)]
pub enum Shape {
    #[default]
    Circle,
    RoundedRectangle,
}

#[derive(IntoElement)]
pub struct Avatar {
    src: ImageSource,
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

        img.source(self.src.clone())
            .size_4()
            // todo!(Pull the avatar fallback background from the theme.)
            .bg(gpui::red())
    }
}

impl Avatar {
    pub fn uri(src: impl Into<SharedString>) -> Self {
        Self {
            src: src.into().into(),
            shape: Shape::Circle,
        }
    }
    pub fn data(src: Arc<ImageData>) -> Self {
        Self {
            src: src.into(),
            shape: Shape::Circle,
        }
    }

    pub fn source(src: ImageSource) -> Self {
        Self {
            src,
            shape: Shape::Circle,
        }
    }
    pub fn shape(mut self, shape: Shape) -> Self {
        self.shape = shape;
        self
    }
}
