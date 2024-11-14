use crate::{prelude::*, Avatar};
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
    /// Creates a new empty facepile.
    pub fn empty() -> Self {
        Self::new(SmallVec::new())
    }

    /// Creates a new facepile with the given faces.
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

impl ComponentPreview for Facepile {
    fn description() -> impl Into<Option<&'static str>> {
        "A facepile is a collection of faces stacked horizontally–\
        always with the leftmost face on top and descending in z-index.\
        \n\nFacepiles are used to display a group of people or things,\
        such as a list of participants in a collaboration session."
    }
    fn examples(_: &WindowContext) -> Vec<ComponentExampleGroup<Self>> {
        let few_faces: [&'static str; 3] = [
            "https://avatars.githubusercontent.com/u/1714999?s=60&v=4",
            "https://avatars.githubusercontent.com/u/67129314?s=60&v=4",
            "https://avatars.githubusercontent.com/u/482957?s=60&v=4",
        ];

        let many_faces: [&'static str; 6] = [
            "https://avatars.githubusercontent.com/u/326587?s=60&v=4",
            "https://avatars.githubusercontent.com/u/2280405?s=60&v=4",
            "https://avatars.githubusercontent.com/u/1789?s=60&v=4",
            "https://avatars.githubusercontent.com/u/67129314?s=60&v=4",
            "https://avatars.githubusercontent.com/u/482957?s=60&v=4",
            "https://avatars.githubusercontent.com/u/1714999?s=60&v=4",
        ];

        vec![example_group_with_title(
            "Examples",
            vec![
                single_example(
                    "Few Faces",
                    Facepile::new(
                        few_faces
                            .iter()
                            .map(|&url| Avatar::new(url).into_any_element())
                            .collect(),
                    ),
                ),
                single_example(
                    "Many Faces",
                    Facepile::new(
                        many_faces
                            .iter()
                            .map(|&url| Avatar::new(url).into_any_element())
                            .collect(),
                    ),
                ),
                single_example(
                    "Custom Size",
                    Facepile::new(
                        few_faces
                            .iter()
                            .map(|&url| Avatar::new(url).size(px(24.)).into_any_element())
                            .collect(),
                    ),
                ),
            ],
        )]
    }
}
