use std::marker::PhantomData;

use gpui2::img;

use crate::prelude::*;

#[derive(Component)]
pub struct Avatar<S: 'static + Send + Sync> {
    state_type: PhantomData<S>,
    src: SharedString,
    shape: Shape,
}

impl<S: 'static + Send + Sync> Avatar<S> {
    pub fn new(src: impl Into<SharedString>) -> Self {
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

    fn render(self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Component<S> {
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
    pub struct AvatarStory<S: 'static + Send + Sync> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync> AvatarStory<S> {
        pub fn new() -> Self {
            Self {
                state_type: PhantomData,
            }
        }

        fn render(self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Component<S> {
            Story::container(cx)
                .child(Story::title_for::<_, Avatar<S>>(cx))
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
