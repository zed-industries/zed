use crate::theme::theme;
use crate::ui::Avatar;
use gpui2::geometry::rems;
use gpui2::style::StyleHelpers;
use gpui2::{elements::div, IntoElement};
use gpui2::{Element, ParentElement, ViewContext};

#[derive(Element)]
pub struct Facepile {
    players: Vec<Avatar>,
}

pub fn facepile(players: Vec<Avatar>) -> Facepile {
    Facepile { players }
}

impl Facepile {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);
        let player_count = self.players.len();
        let player_list = self.players.iter().enumerate().map(|(i, player)| {
            let element = div().child(player.clone());
            if i < player_count - 1 {
                element.mr(-rems(0.5))
            } else {
                element
            }
        });
        div().p_1().flex().items_center().children(player_list)
    }
}
