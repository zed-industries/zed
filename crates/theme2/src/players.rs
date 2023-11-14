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
    use gpui::{div, img, px, Node, ParentComponent, Render, Styled, ViewContext};

    pub struct PlayerStory;

    impl Render for PlayerStory {
        type Element = Node<Self>;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            Story::container(cx).child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(Story::title_for::<_, PlayerColors>(cx))
                    .child(Story::label(cx, "Player Colors"))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div().flex().gap_1().children(
                                    cx.theme().players().0.clone().iter_mut().map(|player| {
                                        div().w_8().h_8().rounded_md().bg(player.cursor)
                                    }),
                                ),
                            )
                            .child(div().flex().gap_1().children(
                                cx.theme().players().0.clone().iter_mut().map(|player| {
                                    div().w_8().h_8().rounded_md().bg(player.background)
                                }),
                            ))
                            .child(div().flex().gap_1().children(
                                cx.theme().players().0.clone().iter_mut().map(|player| {
                                    div().w_8().h_8().rounded_md().bg(player.selection)
                                }),
                            )),
                    )
                    .child(Story::label(cx, "Avatar Rings"))
                    .child(div().flex().gap_1().children(
                        cx.theme().players().0.clone().iter_mut().map(|player| {
                            div()
                                .my_1()
                                .rounded_full()
                                .border_2()
                                .border_color(player.cursor)
                                .child(
                                    img()
                                        .rounded_full()
                                        .uri("https://avatars.githubusercontent.com/u/1714999?v=4")
                                        .size_6()
                                        .bg(gpui::red()),
                                )
                        }),
                    ))
                    .child(Story::label(cx, "Player Backgrounds"))
                    .child(div().flex().gap_1().children(
                        cx.theme().players().0.clone().iter_mut().map(|player| {
                            div()
                                .my_1()
                                .rounded_xl()
                                .flex()
                                .items_center()
                                .h_8()
                                .py_0p5()
                                .px_1p5()
                                .bg(player.background)
                                .child(
                                div().relative().neg_mx_1().rounded_full().z_index(3)
                                    .border_2()
                                    .border_color(player.background)
                                    .size(px(28.))
                                    .child(
                                    img()
                                        .rounded_full()
                                        .uri("https://avatars.githubusercontent.com/u/1714999?v=4")
                                        .size(px(24.))
                                        .bg(gpui::red()),
                                ),
                            ).child(
                            div().relative().neg_mx_1().rounded_full().z_index(2)
                                .border_2()
                                .border_color(player.background)
                                .size(px(28.))
                                .child(
                                img()
                                    .rounded_full()
                                    .uri("https://avatars.githubusercontent.com/u/1714999?v=4")
                                    .size(px(24.))
                                    .bg(gpui::red()),
                            ),
                        ).child(
                        div().relative().neg_mx_1().rounded_full().z_index(1)
                            .border_2()
                            .border_color(player.background)
                            .size(px(28.))
                            .child(
                            img()
                                .rounded_full()
                                .uri("https://avatars.githubusercontent.com/u/1714999?v=4")
                                .size(px(24.))
                                .bg(gpui::red()),
                        ),
                    )
                        }),
                    ))
                    .child(Story::label(cx, "Player Selections"))
                    .child(div().flex().flex_col().gap_px().children(
                        cx.theme().players().0.clone().iter_mut().map(|player| {
                            div()
                                .flex()
                                .child(
                                    div()
                                        .flex()
                                        .flex_none()
                                        .rounded_sm()
                                        .px_0p5()
                                        .text_color(cx.theme().colors().text)
                                        .bg(player.selection)
                                        .child("The brown fox jumped over the lazy dog."),
                                )
                                .child(div().flex_1())
                        }),
                    )),
            )
        }
    }
}
