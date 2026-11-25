use crate::prelude::*;
use gpui::{Animation, AnimationExt, FontWeight};
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

        self.base.with_animation(
            self.variant.animation_id(),
            // Each render also re-renders the views containing the spinner, so render only when
            // the glyph changes. A shared clock lets spinners change, and render, together.
            Animation::new(duration)
                .repeat_synced()
                .with_steps(frames.len()),
            move |mut label, delta| {
                let frame_index = (delta * frames.len() as f32).round() as usize % frames.len();

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

    fn description() -> &'static str {
        "A text-based loading indicator that animates through a sequence of \
        unicode glyphs to show ongoing or indeterminate work."
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> AnyElement {
        let examples = vec![
            single_example("Default", SpinnerLabel::new().into_any_element()),
            single_example(
                "Dots Variant",
                SpinnerLabel::dots_variant().into_any_element(),
            ),
            single_example("Sand Variant", SpinnerLabel::sand().into_any_element()),
        ];

        example_group(examples).into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use std::{cell::Cell, rc::Rc};

    struct SpinnerView {
        renders: Rc<Cell<usize>>,
    }

    impl Render for SpinnerView {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            self.renders.set(self.renders.get() + 1);
            div().child(SpinnerLabel::new())
        }
    }

    #[gpui::test]
    fn test_spinner_renders_only_as_often_as_its_glyph_changes(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
        });
        let renders = Rc::new(Cell::new(0));
        let (_, cx) = cx.add_window_view({
            let renders = renders.clone();
            |_, _| SpinnerView { renders }
        });
        cx.run_until_parked();
        let initial_renders = renders.get();

        // One second of a 100 Hz display, during which the glyph changes 10 times.
        for _ in 0..100 {
            cx.executor().advance_clock(Duration::from_millis(10));
            cx.update(|window, cx| window.simulate_next_frame(cx));
            cx.run_until_parked();
        }

        assert_eq!(renders.get() - initial_renders, 10);
    }
}
