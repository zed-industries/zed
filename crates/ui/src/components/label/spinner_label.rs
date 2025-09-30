use crate::prelude::*;
use gpui::{Animation, AnimationExt, FontWeight};
use std::time::Duration;

/// Different types of spinner animations
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum SpinnerVariant {
    #[default]
    Dots,
    DotsVariant,
}

/// A spinner indication, based on the label component, that loops through
/// frames of the specified animation. It implements `LabelCommon` as well.
///
/// # Default Example
///
/// ```
/// use ui::{SpinnerLabel};
///
/// SpinnerLabel::new();
/// ```
///
/// # Variant Example
///
/// ```
/// use ui::{SpinnerLabel};
///
/// SpinnerLabel::dots_variant();
/// ```
#[derive(IntoElement, RegisterComponent)]
pub struct SpinnerLabel {
    base: Label,
    variant: SpinnerVariant,
    frames: Vec<&'static str>,
    duration: Duration,
}

impl SpinnerVariant {
    fn frames(&self) -> Vec<&'static str> {
        match self {
            SpinnerVariant::Dots => vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            SpinnerVariant::DotsVariant => vec!["⣼", "⣹", "⢻", "⠿", "⡟", "⣏", "⣧", "⣶"],
        }
    }

    fn duration(&self) -> Duration {
        match self {
            SpinnerVariant::Dots => Duration::from_millis(1000),
            SpinnerVariant::DotsVariant => Duration::from_millis(1000),
        }
    }

    fn animation_id(&self) -> &'static str {
        match self {
            SpinnerVariant::Dots => "spinner_label_dots",
            SpinnerVariant::DotsVariant => "spinner_label_dots_variant",
        }
    }
}

impl SpinnerLabel {
    pub fn new() -> Self {
        Self::with_variant(SpinnerVariant::default())
    }

    pub fn with_variant(variant: SpinnerVariant) -> Self {
        let frames = variant.frames();
        let duration = variant.duration();

        SpinnerLabel {
            base: Label::new(frames[0]),
            variant,
            frames,
            duration,
        }
    }

    pub fn dots() -> Self {
        Self::with_variant(SpinnerVariant::Dots)
    }

    pub fn dots_variant() -> Self {
        Self::with_variant(SpinnerVariant::DotsVariant)
    }
}

impl LabelCommon for SpinnerLabel {
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

    fn inline_code(mut self, cx: &App) -> Self {
        self.base = self.base.inline_code(cx);
        self
    }
}

impl RenderOnce for SpinnerLabel {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let frames = self.frames.clone();
        let duration = self.duration;

        self.base.color(Color::Muted).with_animation(
            self.variant.animation_id(),
            Animation::new(duration).repeat(),
            move |mut label, delta| {
                let frame_index = (delta * frames.len() as f32) as usize % frames.len();

                label.set_text(frames[frame_index]);
                label
            },
        )
    }
}

impl Component for SpinnerLabel {
    fn scope() -> ComponentScope {
        ComponentScope::Loading
    }

    fn name() -> &'static str {
        "Spinner Label"
    }

    fn sort_name() -> &'static str {
        "Spinner Label"
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        let examples = vec![
            single_example("Default", SpinnerLabel::new().into_any_element()),
            single_example(
                "Dots Variant",
                SpinnerLabel::dots_variant().into_any_element(),
            ),
        ];

        Some(example_group(examples).vertical().into_any_element())
    }
}
