use gpui::{AnyElement, Hsla, Render};
use story::Story;

use ui::{prelude::*, utils::WithRemSize};

pub struct WithRemSizeStory;

impl Render for WithRemSizeStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Story::container(cx).child(
            Example::new(16., gpui::red())
                .child(
                    Example::new(24., gpui::green())
                        .child(Example::new(8., gpui::blue()))
                        .child(Example::new(16., gpui::yellow())),
                )
                .child(
                    Example::new(12., gpui::green())
                        .child(Example::new(48., gpui::blue()))
                        .child(Example::new(16., gpui::yellow())),
                ),
        )
    }
}

#[derive(IntoElement)]
struct Example {
    rem_size: Pixels,
    border_color: Hsla,
    children: Vec<AnyElement>,
}

impl Example {
    pub fn new(rem_size: impl Into<Pixels>, border_color: Hsla) -> Self {
        Self {
            rem_size: rem_size.into(),
            border_color,
            children: Vec::new(),
        }
    }
}

impl ParentElement for Example {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Example {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        WithRemSize::new(self.rem_size).child(
            v_flex()
                .gap_2()
                .p_2()
                .border_2()
                .border_color(self.border_color)
                .child(Label::new(format!("1rem = {}px", f32::from(self.rem_size))))
                .children(self.children),
        )
    }
}
