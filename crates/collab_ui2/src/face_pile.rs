use gpui::{
    div, AnyElement, Div, ElementId, IntoElement, ParentElement, RenderOnce, Styled, WindowContext,
};
use smallvec::SmallVec;

#[derive(Default, IntoElement)]
pub struct FacePile {
    pub faces: SmallVec<[AnyElement; 2]>,
}

impl RenderOnce for FacePile {
    type Rendered = Div;

    fn render(self, _: &mut WindowContext) -> Self::Rendered {
        let player_count = self.faces.len();
        let player_list = self.faces.into_iter().enumerate().map(|(ix, player)| {
            let isnt_last = ix < player_count - 1;

            div()
                .z_index((player_count - ix) as u32)
                .when(isnt_last, |div| div.neg_mr_1())
                .child(player)
        });
        div().p_1().flex().items_center().children(player_list)
    }
}

impl ParentElement for FacePile {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement; 2]> {
        &mut self.faces
    }
}
