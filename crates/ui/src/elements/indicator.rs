use gpui2::elements::div;
use gpui2::style::StyleHelpers;
use gpui2::{Element, IntoElement, ViewContext};

use crate::theme;

#[derive(Element)]
pub struct Indicator {
    player: usize,
}

pub fn indicator() -> Indicator {
    Indicator { player: 0 }
}

impl Indicator {
    pub fn player(mut self, player: usize) -> Self {
        self.player = player;
        self
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);
        let player_color = theme.players[self.player].cursor;

        div()
            .w_4()
            .h_1()
            .rounded_bl_sm()
            .rounded_br_sm()
            .fill(player_color)
    }
}
