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
        Self { base: div(), faces }
    }
}

impl RenderOnce for FacePile {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        // Lay the faces out in reverse so they overlap in the desired order (left to right, front to back)
        self.base
            .flex()
            .flex_row_reverse()
            .items_center()
            .justify_start()
            .children(
                self.faces
                    .into_iter()
                    .enumerate()
                    .rev()
                    .map(|(ix, player)| div().when(ix > 0, |div| div.neg_ml_1()).child(player)),
            )
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
