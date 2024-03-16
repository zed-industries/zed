use gpui::{div, img, px, IntoElement, ParentElement, Render, Styled, ViewContext};
use story::Story;

use crate::{ActiveTheme, PlayerColors};

pub struct PlayerStory;

impl Render for PlayerStory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        Story::container().child(
            div()
                .flex()
                .flex_col()
                .gap_4()
                .child(Story::title_for::<PlayerColors>())
                .child(Story::label("Player Colors"))
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
                                    .map(|player| div().w_8().h_8().rounded_md().bg(player.cursor)),
                            ),
                        )
                        .child(
                            div().flex().gap_1().children(
                                cx.theme().players().0.clone().iter_mut().map(|player| {
                                    div().w_8().h_8().rounded_md().bg(player.background)
                                }),
                            ),
                        )
                        .child(
                            div().flex().gap_1().children(
                                cx.theme().players().0.clone().iter_mut().map(|player| {
                                    div().w_8().h_8().rounded_md().bg(player.selection)
                                }),
                            ),
                        ),
                )
                .child(Story::label("Avatar Rings"))
                .child(div().flex().gap_1().children(
                    cx.theme().players().0.clone().iter_mut().map(|player| {
                        div()
                            .my_1()
                            .rounded_full()
                            .border_2()
                            .border_color(player.cursor)
                            .child(
                                img("https://avatars.githubusercontent.com/u/1714999?v=4")
                                    .rounded_full()
                                    .size_6()
                                    .bg(gpui::red()),
                            )
                    }),
                ))
                .child(Story::label("Player Backgrounds"))
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
                                div()
                                    .relative()
                                    .neg_mx_1()
                                    .rounded_full()
                                    .border_2()
                                    .border_color(player.background)
                                    .size(px(28.))
                                    .child(
                                        img("https://avatars.githubusercontent.com/u/1714999?v=4")
                                            .rounded_full()
                                            .size(px(24.))
                                            .bg(gpui::red()),
                                    ),
                            )
                            .child(
                                div()
                                    .relative()
                                    .neg_mx_1()
                                    .rounded_full()
                                    .border_2()
                                    .border_color(player.background)
                                    .size(px(28.))
                                    .child(
                                        img("https://avatars.githubusercontent.com/u/1714999?v=4")
                                            .rounded_full()
                                            .size(px(24.))
                                            .bg(gpui::red()),
                                    ),
                            )
                            .child(
                                div()
                                    .relative()
                                    .neg_mx_1()
                                    .rounded_full()
                                    .border_2()
                                    .border_color(player.background)
                                    .size(px(28.))
                                    .child(
                                        img("https://avatars.githubusercontent.com/u/1714999?v=4")
                                            .rounded_full()
                                            .size(px(24.))
                                            .bg(gpui::red()),
                                    ),
                            )
                    }),
                ))
                .child(Story::label("Player Selections"))
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
