use std::ops::Range;

use gpui::{Entity, Render, div, uniform_list};
use gpui::{prelude::*, *};
use ui::{AbsoluteLength, Color, DefiniteLength, Label, LabelCommon, px, v_flex};

use story::Story;

const LENGTH: usize = 100;

pub struct IndentGuidesStory {
    depths: Vec<usize>,
}

impl IndentGuidesStory {
    pub fn model(_window: &mut Window, cx: &mut App) -> Entity<Self> {
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

        cx.new(|_cx| Self { depths })
    }
}

impl Render for IndentGuidesStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Story::container(cx)
            .child(Story::title("Indent guides", cx))
            .child(
                v_flex().size_full().child(
                    uniform_list(
                        "some-list",
                        self.depths.len(),
                        cx.processor(move |this, range: Range<usize>, _window, _cx| {
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
                        }),
                    )
                    .with_sizing_behavior(gpui::ListSizingBehavior::Infer)
                    .with_decoration(
                        ui::indent_guides(
                            px(16.),
                            ui::IndentGuideColors {
                                default: Color::Info.color(cx),
                                hover: Color::Accent.color(cx),
                                active: Color::Accent.color(cx),
                            },
                        )
                        .with_compute_indents_fn(
                            cx.entity(),
                            |this, range, _cx, _context| {
                                this.depths
                                    .iter()
                                    .skip(range.start)
                                    .take(range.end - range.start)
                                    .cloned()
                                    .collect()
                            },
                        ),
                    ),
                ),
            )
    }
}
