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

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
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
