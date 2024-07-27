use crate::prelude::*;
use gpui::{AnyElement, StyleRefinement};
use smallvec::SmallVec;

/// A facepile is a collection of faces stacked horizontally–
/// always with the leftmost face on top and descending in z-index
///
/// Facepiles are used to display a group of people or things,
/// such as a list of participants in a collaboration session.
#[derive(IntoElement)]
pub struct Facepile {
    base: Div,
    faces: SmallVec<[AnyElement; 2]>,
}

impl Facepile {
    pub fn empty() -> Self {
        Self::new(SmallVec::new())
    }

    pub fn new(faces: SmallVec<[AnyElement; 2]>) -> Self {
        Self { base: div(), faces }
    }
}

impl ParentElement for Facepile {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.faces.extend(elements);
    }
}

// Style methods.
impl Facepile {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }

    gpui::padding_style_methods!({
        visibility: pub
    });
}

impl RenderOnce for Facepile {
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
                    .map(|(ix, player)| div().when(ix > 0, |div| div.ml_neg_1()).child(player)),
            )
    }
}
