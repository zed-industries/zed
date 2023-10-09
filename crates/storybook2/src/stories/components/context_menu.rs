use std::marker::PhantomData;

use ui::prelude::*;
use ui::{ContextMenu, ContextMenuItem, Label};

use crate::story::Story;

#[derive(Element)]
pub struct ContextMenuStory<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
}

impl<S: 'static + Send + Sync + Clone> ContextMenuStory<S> {
    pub fn new() -> Self {
        Self {
            state_type: PhantomData,
        }
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        Story::container(cx)
            .child(Story::title_for::<_, ContextMenu<S>>(cx))
            .child(Story::label(cx, "Default"))
            .child(ContextMenu::new([
                ContextMenuItem::header("Section header"),
                ContextMenuItem::Separator,
                ContextMenuItem::entry(Label::new("Some entry")),
            ]))
    }
}
