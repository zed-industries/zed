use gpui::{Animation, AnimationExt, FontWeight, pulsating_between};
use std::time::Duration;
use ui::prelude::*;

#[derive(IntoElement)]
pub struct AnimatedLabel {
    base: Label,
    text: SharedString,
}

impl AnimatedLabel {
    pub fn new(text: impl Into<SharedString>) -> Self {
        let text = text.into();
        AnimatedLabel {
            base: Label::new(text.clone()),
            text,
        }
    }
}

impl LabelCommon for AnimatedLabel {
    fn size(mut self, size: LabelSize) -> Self {
        self.base = self.base.size(size);
        self
    }

    fn weight(mut self, weight: FontWeight) -> Self {
        self.base = self.base.weight(weight);
        self
    }

    fn line_height_style(mut self, line_height_style: LineHeightStyle) -> Self {
        self.base = self.base.line_height_style(line_height_style);
        self
    }

    fn color(mut self, color: Color) -> Self {
        self.base = self.base.color(color);
        self
    }

    fn strikethrough(mut self) -> Self {
        self.base = self.base.strikethrough();
        self
    }

    fn italic(mut self) -> Self {
        self.base = self.base.italic();
        self
    }

    fn alpha(mut self, alpha: f32) -> Self {
        self.base = self.base.alpha(alpha);
        self
    }

    fn underline(mut self) -> Self {
        self.base = self.base.underline();
        self
    }

    fn truncate(mut self) -> Self {
        self.base = self.base.truncate();
        self
    }

    fn single_line(mut self) -> Self {
        self.base = self.base.single_line();
        self
    }

    fn buffer_font(mut self, cx: &App) -> Self {
        self.base = self.base.buffer_font(cx);
        self
    }
}

impl RenderOnce for AnimatedLabel {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let text = self.text.clone();

        self.base
            .color(Color::Muted)
            .with_animations(
                "animated-label",
                vec![
                    Animation::new(Duration::from_secs(1)),
                    Animation::new(Duration::from_secs(1)).repeat(),
                ],
                move |mut label, animation_ix, delta| {
                    match animation_ix {
                        0 => {
                            let chars_to_show = (delta * text.len() as f32).ceil() as usize;
                            let text = SharedString::from(text[0..chars_to_show].to_string());
                            label.set_text(text);
                        }
                        1 => match delta {
                            d if d < 0.25 => label.set_text(text.clone()),
                            d if d < 0.5 => label.set_text(format!("{}.", text)),
                            d if d < 0.75 => label.set_text(format!("{}..", text)),
                            _ => label.set_text(format!("{}...", text)),
                        },
                        _ => {}
                    }
                    label
                },
            )
            .with_animation(
                "pulsating-label",
                Animation::new(Duration::from_secs(2))
                    .repeat()
                    .with_easing(pulsating_between(0.6, 1.)),
                |label, delta| label.map_element(|label| label.alpha(delta)),
            )
    }
}
