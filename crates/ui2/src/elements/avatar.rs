use gpui2::img;

use crate::prelude::*;

#[derive(Component)]
pub struct Avatar {
    src: SharedString,
    shape: Shape,
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

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
        let mut img = img();

        if self.shape == Shape::Circle {
            img = img.rounded_full();
        } else {
            img = img.rounded_md();
        }

        img.uri(self.src.clone())
            .size_4()
            // todo!(Pull the avatar fallback background from the theme.)
            .bg(gpui2::red())
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use super::*;
    use crate::Story;
    use gpui2::{Div, Render};

    pub struct AvatarStory;

    impl Render for AvatarStory {
        type Element = Div<Self>;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            Story::container(cx)
                .child(Story::title_for::<_, Avatar>(cx))
                .child(Story::label(cx, "Default"))
                .child(Avatar::new(
                    "https://avatars.githubusercontent.com/u/1714999?v=4",
                ))
                .child(Avatar::new(
                    "https://avatars.githubusercontent.com/u/326587?v=4",
                ))
                // .child(Avatar::new(
                //     "https://avatars.githubusercontent.com/u/326587?v=4",
                // ))
                // .child(Avatar::new(
                //     "https://avatars.githubusercontent.com/u/482957?v=4",
                // ))
                // .child(Avatar::new(
                //     "https://avatars.githubusercontent.com/u/1714999?v=4",
                // ))
                // .child(Avatar::new(
                //     "https://avatars.githubusercontent.com/u/1486634?v=4",
                // ))
                .child(Story::label(cx, "Rounded rectangle"))
            // .child(
            //     Avatar::new("https://avatars.githubusercontent.com/u/1714999?v=4")
            //         .shape(Shape::RoundedRectangle),
            // )
        }
    }
}
