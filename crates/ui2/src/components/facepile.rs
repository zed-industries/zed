use crate::prelude::*;
use crate::{Avatar, Player};

#[derive(RenderOnce)]
pub struct Facepile {
    players: Vec<Player>,
}

impl<V: 'static> Component<V> for Facepile {
    type Rendered = Div<V>;

    fn render(self, view: &mut V, cx: &mut ViewContext<V>) -> Self::Rendered {
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
#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use super::*;
    use crate::{static_players, Story};
    use gpui::{Div, Render};

    pub struct FacepileStory;

    impl Render<Self> for FacepileStory {
        type Element = Div<Self>;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            let players = static_players();

            Story::container(cx)
                .child(Story::title_for::<_, Facepile>(cx))
                .child(Story::label(cx, "Default"))
                .child(
                    div()
                        .flex()
                        .gap_3()
                        .child(Facepile::new(players.clone().into_iter().take(1)))
                        .child(Facepile::new(players.clone().into_iter().take(2)))
                        .child(Facepile::new(players.clone().into_iter().take(3))),
                )
        }
    }
}
