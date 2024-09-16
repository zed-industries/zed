use gpui::{
    default_style::{colors_iter, default_style},
    div,
    prelude::*,
    Render, View, ViewContext, WindowAppearance, WindowContext,
};
use story::Story;
use ui::h_flex;

pub struct DefaultColorsStory;

impl DefaultColorsStory {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self)
    }
}

impl Render for DefaultColorsStory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let appearances = [WindowAppearance::Light, WindowAppearance::Dark];

        Story::container(cx)
            .child(Story::title(cx, "Default Colors"))
            .children(appearances.iter().map(|&appearance| {
                let default_style = default_style(appearance);

                let color = default_style.color;

                let colors = colors_iter(appearance);

                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .p_4()
                    .bg(color.background)
                    .text_color(color.foreground)
                    .child(
                        div()
                            .text_xs()
                            .text_color(color.foreground)
                            .child(format!("{:?} Appearance", appearance)),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .children(colors.iter().map(|c| {
                                let fill: gpui::Fill = c.clone().into();

                                div()
                                    .w_12()
                                    .h_12()
                                    .bg(fill)
                                    .border_1()
                                    .border_color(color.border)
                            })),
                    )
                    .child(
                        h_flex()
                            .gap_1()
                            .child(
                                h_flex()
                                    .bg(color.background)
                                    .h_8()
                                    .p_2()
                                    .text_sm()
                                    .text_color(color.foreground)
                                    .child("Default Text"),
                            )
                            .child(
                                h_flex()
                                    .bg(color.container)
                                    .h_8()
                                    .p_2()
                                    .text_sm()
                                    .text_color(color.foreground)
                                    .child("Text on Container"),
                            )
                            .child(
                                h_flex()
                                    .bg(color.background_selected)
                                    .h_8()
                                    .p_2()
                                    .text_sm()
                                    .text_color(color.foreground_selected)
                                    .child("Selected Text"),
                            ),
                    )
            }))
    }
}
