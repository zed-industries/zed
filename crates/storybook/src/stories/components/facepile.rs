use ui::Facepile;
use ui::{prelude::*, static_players};

use crate::story::Story;

#[derive(Element, Default)]
pub struct FacepileStory {}

impl FacepileStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let players = static_players();

        Story::container(cx)
            .child(Story::title_for::<_, ui::Facepile>(cx))
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
