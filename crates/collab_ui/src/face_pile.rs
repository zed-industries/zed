use gpui::AnyElement;
use smallvec::SmallVec;
use ui::prelude::*;

#[derive(IntoElement)]
pub struct FacePile {
    base: Div,
    faces: SmallVec<[AnyElement; 2]>,
}

impl FacePile {
    pub fn empty() -> Self {
        Self::new(SmallVec::new())
    }

    pub fn new(faces: SmallVec<[AnyElement; 2]>) -> Self {
        Self {
            base: h_flex(),
            faces,
        }
    }
}

impl RenderOnce for FacePile {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let player_count = self.faces.len();
        let player_list = self.faces.into_iter().enumerate().map(|(ix, player)| {
            let isnt_last = ix < player_count - 1;

            div()
                .z_index((player_count - ix) as u16)
                .when(isnt_last, |div| div.neg_mr_1())
                .child(player)
        });
        self.base.children(player_list)
    }
}

impl ParentElement for FacePile {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.faces.extend(elements);
    }
}

impl Styled for FacePile {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}
