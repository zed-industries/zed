use gpui2::elements::div;
use gpui2::{Element, IntoElement, ParentElement, ViewContext};

use crate::prelude::*;
use crate::{theme, Avatar, Facepile, Indicator};

#[derive(Element)]
pub struct FollowGroup {
    player: usize,
    players: Vec<Avatar>,
}

impl FollowGroup {
    pub fn new(players: Vec<Avatar>) -> Self {
        Self { player: 0, players }
    }

    pub fn player(mut self, player: usize) -> Self {
        self.player = player;
        self
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);
        let player_bg = theme.players[self.player].selection;

        div()
            .h_full()
            .flex()
            .flex_col()
            .gap_px()
            .justify_center()
            .child(
                div()
                    .flex()
                    .justify_center()
                    .w_full()
                    .child(Indicator::new().player(self.player)),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .h_6()
                    .px_1()
                    .rounded_lg()
                    .fill(player_bg)
                    .child(Facepile::new(self.players.clone().into_iter())),
            )
    }
}
