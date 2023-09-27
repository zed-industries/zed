use crate::{prelude::*, PlayerWithCallStatus};
use crate::{Avatar, Facepile};

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
        let system_color = SystemColor::new();
        let player = self.player_with_call_status.get_player();
        self.player_with_call_status.get_call_status();

        let followers = self
            .player_with_call_status
            .get_call_status()
            .followers
            .as_ref()
            .map(|followers| followers.clone());

        // if we have no followers return a slightly different element
        // if mic_status == muted add a red ring to avatar

        div()
            .h_full()
            .flex()
            .flex_col()
            .gap_px()
            .justify_center()
            .child(
                div().flex().justify_center().w_full().child(
                    div()
                        .w_4()
                        .h_1()
                        .rounded_bl_sm()
                        .rounded_br_sm()
                        .fill(player.cursor_color(cx)),
                ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .h_6()
                    .px_1()
                    .rounded_lg()
                    .fill(if followers.is_none() {
                        system_color.transparent
                    } else {
                        player.selection_color(cx)
                    })
                    .child(Avatar::new(player.avatar_src().to_string()))
                    .children(followers.map(|followers| {
                        div().neg_mr_1().child(Facepile::new(followers.into_iter()))
                    })),
            )
    }
}
