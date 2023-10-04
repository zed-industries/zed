use std::marker::PhantomData;

use crate::ui::prelude::*;
use crate::ui::Avatar;

use crate::story::Story;

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
