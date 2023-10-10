use std::marker::PhantomData;

use gpui3::{img, ArcCow};

use crate::prelude::*;
use crate::theme::theme;

#[derive(Element, Clone)]
pub struct Avatar<S: 'static + Send + Sync> {
    state_type: PhantomData<S>,
    src: ArcCow<'static, str>,
    shape: Shape,
}

impl<S: 'static + Send + Sync> Avatar<S> {
    pub fn new(src: impl Into<ArcCow<'static, str>>) -> Self {
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

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
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

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use crate::Story;

    use super::*;

    #[derive(Element)]
    pub struct AvatarStory<S: 'static + Send + Sync> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync> AvatarStory<S> {
        pub fn new() -> Self {
            Self {
                state_type: PhantomData,
            }
        }

        fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
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
