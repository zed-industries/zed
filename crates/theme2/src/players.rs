use gpui::Hsla;

#[derive(Debug, Clone, Copy)]
pub struct PlayerColor {
    pub cursor: Hsla,
    pub background: Hsla,
    pub selection: Hsla,
}

/// A collection of colors that are used to color players in the editor.
///
/// The first color is always the local player's color, usually a blue.
///
/// The rest of the default colors crisscross back and forth on the
/// color wheel so that the colors are as distinct as possible.
#[derive(Clone)]
pub struct PlayerColors(pub Vec<PlayerColor>);

impl PlayerColors {
    pub fn local(&self) -> PlayerColor {
        // todo!("use a valid color");
        *self.0.first().unwrap()
    }

    pub fn absent(&self) -> PlayerColor {
        // todo!("use a valid color");
        *self.0.last().unwrap()
    }

    pub fn color_for_participant(&self, participant_index: u32) -> PlayerColor {
        let len = self.0.len() - 1;
        self.0[(participant_index as usize % len) + 1]
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use super::*;
    use crate::{ActiveTheme, Story};
    use gpui::{div, Div, ParentElement, Render, Styled, ViewContext};

    pub struct PlayerStory;

    impl Render for PlayerStory {
        type Element = Div<Self>;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            Story::container(cx)
                .child(Story::title_for::<_, PlayerColors>(cx))
                .child(Story::label(cx, "Player Colors"))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(
                            div().flex().gap_1().children(
                                cx.theme()
                                    .players()
                                    .0
                                    .clone()
                                    .iter_mut()
                                    .map(|color| div().w_8().h_8().rounded_md().bg(color.cursor)),
                            ),
                        )
                        .child(
                            div().flex().gap_1().children(
                                cx.theme().players().0.clone().iter_mut().map(|color| {
                                    div().w_8().h_8().rounded_md().bg(color.background)
                                }),
                            ),
                        )
                        .child(
                            div().flex().gap_1().children(
                                cx.theme().players().0.clone().iter_mut().map(|color| {
                                    div().w_8().h_8().rounded_md().bg(color.selection)
                                }),
                            ),
                        ),
                )
        }
    }
}
