use crate::component_prelude::*;
use crate::prelude::*;
use gpui::{AnyElement, StyleRefinement};
use smallvec::SmallVec;

use super::Avatar;

/// An element that displays a collection of (usually) faces stacked
/// horizontally, with the left-most face on top, visually descending
/// from left to right.
///
/// Facepiles are used to display a group of people or things,
/// such as a list of participants in a collaboration session.
///
/// # Examples
///
/// ## Default
///
/// A default, horizontal facepile.
///
/// ```
/// use ui::{Avatar, Facepile, EXAMPLE_FACES};
///
/// Facepile::new(
/// EXAMPLE_FACES.iter().take(3).iter().map(|&url|
///    Avatar::new(url).into_any_element()).collect())
/// ```
#[derive(IntoElement, Documented, RegisterComponent)]
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
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
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

pub const EXAMPLE_FACES: [&'static str; 6] = [
    "https://avatars.githubusercontent.com/u/326587?s=60&v=4",
    "https://avatars.githubusercontent.com/u/2280405?s=60&v=4",
    "https://avatars.githubusercontent.com/u/1789?s=60&v=4",
    "https://avatars.githubusercontent.com/u/67129314?s=60&v=4",
    "https://avatars.githubusercontent.com/u/482957?s=60&v=4",
    "https://avatars.githubusercontent.com/u/1714999?s=60&v=4",
];

impl Component for Facepile {
    fn scope() -> ComponentScope {
        ComponentScope::Collaboration
    }

    fn description() -> Option<&'static str> {
        Some(
            "Displays a collection of avatars or initials in a compact format. Often used to represent active collaborators or a subset of contributors.",
        )
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .gap_6()
                .children(vec![example_group_with_title(
                    "Facepile Examples",
                    vec![
                        single_example(
                            "Default",
                            Facepile::new(
                                EXAMPLE_FACES
                                    .iter()
                                    .map(|&url| Avatar::new(url).into_any_element())
                                    .collect(),
                            )
                            .into_any_element(),
                        ),
                        single_example(
                            "Custom Size",
                            Facepile::new(
                                EXAMPLE_FACES
                                    .iter()
                                    .map(|&url| Avatar::new(url).size(px(24.)).into_any_element())
                                    .collect(),
                            )
                            .into_any_element(),
                        ),
                    ],
                )])
                .into_any_element(),
        )
    }
}
