use std::marker::PhantomData;

use ui::prelude::*;
use ui::{static_players, Facepile};

use crate::story::Story;

#[derive(Element)]
pub struct FacepileStory<S: 'static + Send + Sync> {
    state_type: PhantomData<S>,
}

impl<S: 'static + Send + Sync> FacepileStory<S> {
    pub fn new() -> Self {
        Self {
            state_type: PhantomData,
        }
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        let players = static_players();

        Story::container(cx)
            .child(Story::title_for::<_, Facepile<S>>(cx))
            .child(Story::label(cx, "Default"))
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(Facepile::new(players.clone().into_iter().take(1)))
                    .child(Facepile::new(players.clone().into_iter().take(2)))
                    .child(Facepile::new(players.clone().into_iter().take(3))),
            )
    }
}
