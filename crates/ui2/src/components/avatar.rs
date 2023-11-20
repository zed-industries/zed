use crate::prelude::*;
use gpui::{img, Img, RenderOnce};

#[derive(RenderOnce)]
pub struct Avatar {
    src: SharedString,
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

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use super::*;
    use crate::Story;
    use gpui::{Div, Render};

    pub struct AvatarStory;

    impl Render for AvatarStory {
        type Element = Div;

        fn render(&mut self, cx: &mut WindowContext) -> Self::Element {
            Story::container(cx)
                .child(Story::title_for::<Avatar>(cx))
                .child(Story::label(cx, "Default"))
                .child(Avatar::new(
                    "https://avatars.githubusercontent.com/u/1714999?v=4",
                ))
                .child(Avatar::new(
                    "https://avatars.githubusercontent.com/u/326587?v=4",
                ))
        }
    }
}
