use crate::prelude::*;
use crate::{Avatar, Player};

#[derive(Component)]
pub struct Facepile {
    players: Vec<Player>,
}

impl Facepile {
    pub fn new<P: Iterator<Item = Player>>(players: P) -> Self {
        Self {
            players: players.collect(),
        }
    }

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
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

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use crate::{static_players, Story};

    use super::*;

    #[derive(Component)]
    pub struct FacepileStory;

    impl FacepileStory {
        fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
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
