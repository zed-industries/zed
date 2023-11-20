use gpui::{Div, RenderOnce};

use crate::prelude::*;
use crate::{Avatar, Facepile, PlayerWithCallStatus};

#[derive(RenderOnce)]
pub struct PlayerStack {
    player_with_call_status: PlayerWithCallStatus,
}

impl Component for PlayerStack {
    type Rendered = Div;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        let player = self.player_with_call_status.get_player();

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
                div()
                    .flex()
                    .justify_center()
                    .w_full()
                    .child(div().w_4().h_0p5().rounded_sm().bg(player.cursor_color(cx))),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .h_6()
                    .pl_1()
                    .rounded_lg()
                    .bg(if followers.is_none() {
                        cx.theme().styles.system.transparent
                    } else {
                        player.selection_color(cx)
                    })
                    .child(Avatar::new(player.avatar_src().to_string()))
                    .children(followers.map(|followers| {
                        div().neg_ml_2().child(Facepile::new(followers.into_iter()))
                    })),
            )
    }
}

impl PlayerStack {
    pub fn new(player_with_call_status: PlayerWithCallStatus) -> Self {
        Self {
            player_with_call_status,
        }
    }
}
