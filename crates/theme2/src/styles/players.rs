use gpui::Hsla;

#[derive(Debug, Clone, Copy, Default)]
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

impl Default for PlayerColors {
    /// Don't use this!
    /// We have to have a default to be `[refineable::Refinable]`.
    /// todo!("Find a way to not need this for Refinable")
    fn default() -> Self {
        Self::dark()
    }
}

impl PlayerColors {
    pub fn dark() -> Self {
        Self(vec![
            PlayerColor {
                cursor: blue().dark().step_9(),
                background: blue().dark().step_5(),
                selection: blue().dark().step_3(),
            },
            PlayerColor {
                cursor: orange().dark().step_9(),
                background: orange().dark().step_5(),
                selection: orange().dark().step_3(),
            },
            PlayerColor {
                cursor: pink().dark().step_9(),
                background: pink().dark().step_5(),
                selection: pink().dark().step_3(),
            },
            PlayerColor {
                cursor: lime().dark().step_9(),
                background: lime().dark().step_5(),
                selection: lime().dark().step_3(),
            },
            PlayerColor {
                cursor: purple().dark().step_9(),
                background: purple().dark().step_5(),
                selection: purple().dark().step_3(),
            },
            PlayerColor {
                cursor: amber().dark().step_9(),
                background: amber().dark().step_5(),
                selection: amber().dark().step_3(),
            },
            PlayerColor {
                cursor: jade().dark().step_9(),
                background: jade().dark().step_5(),
                selection: jade().dark().step_3(),
            },
            PlayerColor {
                cursor: red().dark().step_9(),
                background: red().dark().step_5(),
                selection: red().dark().step_3(),
            },
        ])
    }

    pub fn light() -> Self {
        Self(vec![
            PlayerColor {
                cursor: blue().light().step_9(),
                background: blue().light().step_4(),
                selection: blue().light().step_3(),
            },
            PlayerColor {
                cursor: orange().light().step_9(),
                background: orange().light().step_4(),
                selection: orange().light().step_3(),
            },
            PlayerColor {
                cursor: pink().light().step_9(),
                background: pink().light().step_4(),
                selection: pink().light().step_3(),
            },
            PlayerColor {
                cursor: lime().light().step_9(),
                background: lime().light().step_4(),
                selection: lime().light().step_3(),
            },
            PlayerColor {
                cursor: purple().light().step_9(),
                background: purple().light().step_4(),
                selection: purple().light().step_3(),
            },
            PlayerColor {
                cursor: amber().light().step_9(),
                background: amber().light().step_4(),
                selection: amber().light().step_3(),
            },
            PlayerColor {
                cursor: jade().light().step_9(),
                background: jade().light().step_4(),
                selection: jade().light().step_3(),
            },
            PlayerColor {
                cursor: red().light().step_9(),
                background: red().light().step_4(),
                selection: red().light().step_3(),
            },
        ])
    }
}

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

use crate::{amber, blue, jade, lime, orange, pink, purple, red};

#[cfg(feature = "stories")]
mod stories {
    use super::*;
    use crate::{ActiveTheme, Story};
    use gpui::{div, img, px, Div, ParentElement, Render, Styled, ViewContext};

    pub struct PlayerStory;

    impl Render for PlayerStory {
        type Element = Div;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            Story::container(cx).child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(Story::title_for::<PlayerColors>(cx))
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
