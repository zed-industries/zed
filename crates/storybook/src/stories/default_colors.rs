use gpui::{
    colors, div, prelude::*, DefaultColor, DefaultThemeAppearance, Hsla, Render, View, ViewContext,
    WindowContext,
};
use story::Story;
use strum::IntoEnumIterator;
use ui::{h_flex, ActiveTheme};

pub struct DefaultColorsStory;

impl DefaultColorsStory {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self)
    }
}

impl Render for DefaultColorsStory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let appearances = [DefaultThemeAppearance::Light, DefaultThemeAppearance::Dark];

        Story::container()
            .child(Story::title("Default Colors"))
            .children(appearances.iter().map(|&appearance| {
                let colors = colors(appearance);
                let color_types = DefaultColor::iter()
                    .map(|color| {
                        let name = format!("{:?}", color);
                        let rgba = color.hsla(&colors);
                        (name, rgba)
                    })
                    .collect::<Vec<_>>();

                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .p_4()
                    .child(Story::label(format!("{:?} Appearance", appearance)))
                    .children(color_types.iter().map(|(name, color)| {
                        let color: Hsla = *color;

                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .w_12()
                                    .h_12()
                                    .bg(color)
                                    .border_1()
                                    .border_color(cx.theme().colors().border),
                            )
                            .child(Story::label(format!("{}: {:?}", name, color.clone())))
                    }))
                    .child(
                        h_flex()
                            .gap_1()
                            .child(
                                h_flex()
                                    .bg(DefaultColor::Background.hsla(&colors))
                                    .h_8()
                                    .p_2()
                                    .text_sm()
                                    .text_color(DefaultColor::Text.hsla(&colors))
                                    .child("Default Text"),
                            )
                            .child(
                                h_flex()
                                    .bg(DefaultColor::Container.hsla(&colors))
                                    .h_8()
                                    .p_2()
                                    .text_sm()
                                    .text_color(DefaultColor::Text.hsla(&colors))
                                    .child("Text on Container"),
                            )
                            .child(
                                h_flex()
                                    .bg(DefaultColor::Selected.hsla(&colors))
                                    .h_8()
                                    .p_2()
                                    .text_sm()
                                    .text_color(DefaultColor::SelectedText.hsla(&colors))
                                    .child("Selected Text"),
                            ),
                    )
            }))
    }
}
