use crate::{prelude::*, Player, PlayerWithCallStatus};
use crate::{Facepile, Indicator};

#[derive(Element)]
pub struct PlayerStack {
    player_with_call_status: PlayerWithCallStatus,
}

impl PlayerStack {
    pub fn new(player_with_call_status: PlayerWithCallStatus) -> Self {
        Self {
            player_with_call_status,
        }
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let player = self.player_with_call_status.get_player();

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
                    .child(Indicator::new().player(player.get_index())),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .h_6()
                    .px_1()
                    .rounded_lg()
                    .fill(player.selection_color(cx, player.get_index()))
                    .child(Facepile::new(self.players.clone().into_iter())),
            )
    }
}
