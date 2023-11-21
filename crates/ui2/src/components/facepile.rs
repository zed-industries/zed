use crate::prelude::*;
use crate::{Avatar, Player};

#[derive(RenderOnce)]
pub struct Facepile {
    players: Vec<Player>,
}

impl Component for Facepile {
    type Rendered = Div;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
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

impl Facepile {
    pub fn new<P: Iterator<Item = Player>>(players: P) -> Self {
        Self {
            players: players.collect(),
        }
    }
}

use gpui::{Div, RenderOnce};
