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
        let theme = theme(cx);

        let mut img = img();

        if self.shape == Shape::Circle {
            img = img.rounded_full();
        } else {
            img = img.rounded_md();
        }

        img.uri(self.src.clone())
            .size_4()
            .bg(theme.image_fallback_background)
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use crate::Story;

    use super::*;

    #[derive(Component)]
    pub struct AvatarStory;

    impl AvatarStory {
        fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
            Story::container(cx)
                .child(Story::title_for::<_, Avatar>(cx))
                .child(Story::label(cx, "Default"))
                .child(Avatar::new(
                    "https://avatars.githubusercontent.com/u/1714999?v=4",
                ))
                .child(Story::label(cx, "Rounded rectangle"))
                .child(
                    Avatar::new("https://avatars.githubusercontent.com/u/1714999?v=4")
                        .shape(Shape::RoundedRectangle),
                )
        }
    }
}
