use gpui2::elements::div;
use gpui2::style::StyleHelpers;
use gpui2::{Element, IntoElement, ParentElement, ViewContext};

use crate::{facepile, indicator, theme, Avatar};

#[derive(Element)]
pub struct FollowGroup {
    player: usize,
    players: Vec<Avatar>,
}

pub fn follow_group(players: Vec<Avatar>) -> FollowGroup {
    FollowGroup { player: 0, players }
}

impl FollowGroup {
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
                    .child(indicator().player(self.player)),
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
                    .child(facepile(self.players.clone())),
            )
    }
}
