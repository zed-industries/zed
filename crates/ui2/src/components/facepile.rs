use std::marker::PhantomData;

use crate::prelude::*;
use crate::{theme, Avatar, Player};

#[derive(Element)]
pub struct Facepile<S: 'static + Send + Sync> {
    state_type: PhantomData<S>,
    players: Vec<Player>,
}

impl<S: 'static + Send + Sync> Facepile<S> {
    pub fn new<P: Iterator<Item = Player>>(players: P) -> Self {
        Self {
            state_type: PhantomData,
            players: players.collect(),
        }
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let theme = theme(cx);
        let player_count = self.players.len();
        let player_list = self.players.iter().enumerate().map(|(ix, player)| {
            let isnt_last = ix < player_count - 1;

            div()
                .when(isnt_last, |div| div.neg_mr_1())
                .child(Avatar::new(player.avatar_src().to_string()))
        });
        div().p_1().flex().items_center().children(player_list)
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use crate::{static_players, Story};

    use super::*;

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

        fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
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
}
