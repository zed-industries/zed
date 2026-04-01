use crate::prelude::*;
use gpui::{Animation, AnimationExt};
use std::time::Duration;

/// Different types of spinner animations
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum SpinnerVariant {
    #[default]
    Dots,
    DotsVariant,
    Sand,
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
            SpinnerVariant::Sand => vec![
                "⠁", "⠂", "⠄", "⡀", "⡈", "⡐", "⡠", "⣀", "⣁", "⣂", "⣄", "⣌", "⣔", "⣤", "⣥", "⣦",
                "⣮", "⣶", "⣷", "⣿", "⡿", "⠿", "⢟", "⠟", "⡛", "⠛", "⠫", "⢋", "⠋", "⠍", "⡉", "⠉",
                "⠑", "⠡", "⢁",
            ],
        }
    }

    fn duration(&self) -> Duration {
        match self {
            SpinnerVariant::Dots => Duration::from_millis(1000),
            SpinnerVariant::DotsVariant => Duration::from_millis(1000),
            SpinnerVariant::Sand => Duration::from_millis(2000),
        }
    }

    fn animation_id(&self) -> &'static str {
        match self {
            SpinnerVariant::Dots => "spinner_label_dots",
            SpinnerVariant::DotsVariant => "spinner_label_dots_variant",
            SpinnerVariant::Sand => "spinner_label_dots_variant_2",
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
            base: Label::new(frames[0]).color(Color::Muted),
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

    pub fn sand() -> Self {
        Self::with_variant(SpinnerVariant::Sand)
    }
}

impl_label_common!(SpinnerLabel);

impl RenderOnce for SpinnerLabel {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let frames = self.frames.clone();
        let duration = self.duration;

        self.base.with_animation(
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
            single_example("Sand Variant", SpinnerLabel::sand().into_any_element()),
        ];

        Some(example_group(examples).vertical().into_any_element())
    }
}
