use gpui::{
    colors, div, prelude::*, rgb, DefaultColor, DefaultColors, DefaultThemeApperance, Hsla, Render,
    Rgba, View, ViewContext, WindowContext,
};
use story::Story;
use strum::IntoEnumIterator;
use ui::ActiveTheme;

pub struct DefaultColorsStory;

impl DefaultColorsStory {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self)
    }
}

impl Render for DefaultColorsStory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let appearances = [DefaultThemeApperance::Light, DefaultThemeApperance::Dark];

        Story::container()
            .child(Story::title("Default Colors"))
            .children(appearances.iter().map(|&appearance| {
                let colors = colors(appearance);
                let color_types = DefaultColor::iter()
                    .map(|color| {
                        let name = format!("{:?}", color);
                        let rgba = colors.color(color);
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
                        let color: Hsla = color.clone().into();

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
                            .child(Story::label(format!("{}: {:?}", name, color)))
                    }))
            }))
    }
}
