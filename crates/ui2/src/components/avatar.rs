use std::sync::Arc;

use crate::prelude::*;
use gpui::{img, rems, Div, ImageData, ImageSource, IntoElement, Styled};

#[derive(Debug, Default, PartialEq, Clone)]
pub enum Shape {
    #[default]
    Circle,
    RoundedRectangle,
}

#[derive(IntoElement)]
pub struct Avatar {
    src: ImageSource,
    is_available: Option<bool>,
    shape: Shape,
}

impl RenderOnce for Avatar {
    type Rendered = Div;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        let mut img = img();

        if self.shape == Shape::Circle {
            img = img.rounded_full();
        } else {
            img = img.rounded_md();
        }

        let size = rems(1.0);

        div()
            .size(size)
            .child(
                img.source(self.src.clone())
                    .size(size)
                    // todo!(Pull the avatar fallback background from the theme.)
                    .bg(gpui::red()),
            )
            .children(self.is_available.map(|is_free| {
                // HACK: non-integer sizes result in oval indicators.
                let indicator_size = (size.0 * cx.rem_size() * 0.4).round();

                div()
                    .absolute()
                    .z_index(1)
                    .bg(if is_free { gpui::green() } else { gpui::red() })
                    .size(indicator_size)
                    .rounded(indicator_size)
                    .bottom_0()
                    .right_0()
            }))
    }
}

impl Avatar {
    pub fn uri(src: impl Into<SharedString>) -> Self {
        Self {
            src: src.into().into(),
            shape: Shape::Circle,
            is_available: None,
        }
    }
    pub fn data(src: Arc<ImageData>) -> Self {
        Self {
            src: src.into(),
            shape: Shape::Circle,
            is_available: None,
        }
    }

    pub fn source(src: ImageSource) -> Self {
        Self {
            src,
            shape: Shape::Circle,
            is_available: None,
        }
    }
    pub fn shape(mut self, shape: Shape) -> Self {
        self.shape = shape;
        self
    }
    pub fn availability_indicator(mut self, is_available: impl Into<Option<bool>>) -> Self {
        self.is_available = is_available.into();
        self
    }
}
