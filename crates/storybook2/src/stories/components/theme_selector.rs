use std::marker::PhantomData;

use ui::prelude::*;
use ui::ThemeSelector;

use crate::story::Story;

#[derive(Element)]
pub struct ThemeSelectorStory<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
}

impl<S: 'static + Send + Sync + Clone> ThemeSelectorStory<S> {
    pub fn new() -> Self {
        Self {
            state_type: PhantomData,
        }
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        Story::container(cx)
            .child(Story::title_for::<_, ThemeSelector<S>>(cx))
            .child(Story::label(cx, "Default"))
            .child(ThemeSelector::new())
    }
}
