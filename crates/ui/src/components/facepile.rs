use crate::prelude::*;
use crate::{theme, Avatar, Player};

#[derive(Element)]
pub struct Facepile {
    players: Vec<Player>,
}

impl Facepile {
    pub fn new<P: Iterator<Item = Player>>(players: P) -> Self {
        Self {
            players: players.collect(),
        }
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
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
