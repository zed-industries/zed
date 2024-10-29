use std::fmt::format;

use gpui::{
    colors, div, prelude::*, uniform_list, DefaultColor, DefaultThemeAppearance, Hsla, Render,
    View, ViewContext, WindowContext,
};
use story::Story;
use strum::IntoEnumIterator;
use ui::{
    h_flex, px, v_flex, AbsoluteLength, ActiveTheme, Color, DefiniteLength, Label, LabelCommon,
};

const LENGTH: usize = 100;

pub struct IndentGuidesStory {
    depths: Vec<usize>,
}

impl IndentGuidesStory {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        let mut depths = Vec::new();
        depths.push(0);
        depths.push(1);
        depths.push(2);
        for _ in 0..LENGTH - 6 {
            depths.push(3);
        }
        depths.push(2);
        depths.push(1);
        depths.push(0);

        cx.new_view(|_cx| Self { depths })
    }
}

impl Render for IndentGuidesStory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        Story::container()
            .child(Story::title("Indent guides"))
            .child(
                v_flex().size_full().child(
                    uniform_list(
                        cx.view().clone(),
                        "some-list",
                        self.depths.len(),
                        |this, range, cx| {
                            this.depths
                                .iter()
                                .enumerate()
                                .skip(range.start)
                                .take(range.end - range.start)
                                .map(|(i, depth)| {
                                    div()
                                        .pl(DefiniteLength::Absolute(AbsoluteLength::Pixels(px(
                                            16. * (*depth as f32),
                                        ))))
                                        .child(Label::new(format!("Item {}", i)).color(Color::Info))
                                })
                                .collect()
                        },
                    )
                    .with_sizing_behavior(gpui::ListSizingBehavior::Infer)
                    .with_decoration(ui::indent_guides(
                        cx.view().clone(),
                        px(16.),
                        ui::IndentGuideColors {
                            default: Color::Info.color(cx),
                            hovered: Color::Accent.color(cx),
                            active: Color::Accent.color(cx),
                        },
                        |this, range, cx| {
                            this.depths
                                .iter()
                                .skip(range.start)
                                .take(range.end - range.start)
                                .cloned()
                                .collect()
                        },
                    )),
                ),
            )
    }
}
