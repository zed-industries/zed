use gpui2::elements::div;
use gpui2::style::StyleHelpers;
use gpui2::{Element, IntoElement, ParentElement, ViewContext};

use crate::{theme, Avatar};

#[derive(Element)]
pub struct Facepile {
    players: Vec<Avatar>,
}

impl Facepile {
    pub fn new<P: Iterator<Item = Avatar>>(players: P) -> Self {
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
                .child(player.clone())
        });
        div().p_1().flex().items_center().children(player_list)
    }
}
