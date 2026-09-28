use crate::prelude::*;
use gpui::{Animation, AnimationExt, FontWeight};
use std::time::Duration;

#[derive(IntoElement)]
pub struct LoadingLabel {
    base: Label,
    width_reserver: Label,
    text: SharedString,
}

impl LoadingLabel {
    pub fn new(text: impl Into<SharedString>) -> Self {
        let text = text.into();
        LoadingLabel {
            base: Label::new(text.clone()),
            width_reserver: Label::new(format!("{text}...")),
            text,
        }
    }
}

impl LabelCommon for LoadingLabel {
    fn size(mut self, size: LabelSize) -> Self {
        self.base = self.base.size(size);
        self.width_reserver = self.width_reserver.size(size);
        self
    }

    fn weight(mut self, weight: FontWeight) -> Self {
        self.base = self.base.weight(weight);
        self.width_reserver = self.width_reserver.weight(weight);
        self
    }

    fn line_height_style(mut self, line_height_style: LineHeightStyle) -> Self {
        self.base = self.base.line_height_style(line_height_style);
        self.width_reserver = self.width_reserver.line_height_style(line_height_style);
        self
    }

    fn color(mut self, color: Color) -> Self {
        self.base = self.base.color(color);
        self.width_reserver = self.width_reserver.color(color);
        self
    }

    fn strikethrough(mut self) -> Self {
        self.base = self.base.strikethrough();
        self.width_reserver = self.width_reserver.strikethrough();
        self
    }

    fn italic(mut self) -> Self {
        self.base = self.base.italic();
        self.width_reserver = self.width_reserver.italic();
        self
    }

    fn alpha(mut self, alpha: f32) -> Self {
        self.base = self.base.alpha(alpha);
        self.width_reserver = self.width_reserver.alpha(alpha);
        self
    }

    fn underline(mut self) -> Self {
        self.base = self.base.underline();
        self.width_reserver = self.width_reserver.underline();
        self
    }

    fn truncate(mut self) -> Self {
        self.base = self.base.truncate();
        self.width_reserver = self.width_reserver.truncate();
        self
    }

    fn single_line(mut self) -> Self {
        self.base = self.base.single_line();
        self.width_reserver = self.width_reserver.single_line();
        self
    }

    fn buffer_font(mut self, cx: &App) -> Self {
        self.base = self.base.buffer_font(cx);
        self.width_reserver = self.width_reserver.buffer_font(cx);
        self
    }

    fn inline_code(mut self, cx: &App) -> Self {
        self.base = self.base.inline_code(cx);
        self.width_reserver = self.width_reserver.inline_code(cx);
        self
    }
}

impl RenderOnce for LoadingLabel {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let text = self.text.clone();

        let animated_label = self.base.color(Color::Muted).with_animations(
            "loading_label",
            vec![
                Animation::new(Duration::from_secs(1)),
                Animation::new(Duration::from_secs(1)).repeat(),
            ],
            move |mut label, animation_ix, delta| {
                match animation_ix {
                    0 => {
                        let byte_end =
                            text.floor_char_boundary((delta * text.len() as f32).ceil() as usize);
                        let visible_text = SharedString::new(&text[0..byte_end]);
                        label.set_text(visible_text);
                    }
                    1 => match delta {
                        ..0.25 => label.set_text(text.clone()),
                        ..0.5 => label.set_text(format!("{}.", text)),
                        ..0.75 => label.set_text(format!("{}..", text)),
                        _ => label.set_text(format!("{}...", text)),
                    },
                    _ => {}
                }
                label
            },
        );

        // The animated text changes length every frame, so reserve the space of the
        // fully-rendered text to keep it from re-wrapping and shifting nearby layout.
        div()
            .relative()
            .child(div().invisible().child(self.width_reserver))
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full()
                    .child(animated_label),
            )
    }
}
