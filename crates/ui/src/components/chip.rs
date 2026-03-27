use crate::prelude::*;
use gpui::{AnyElement, AnyView, Hsla, IntoElement, ParentElement, Styled};

/// Chips provide a container for an informative label.
///
/// # Usage Example
///
/// ```
/// use ui::Chip;
///
/// let chip = Chip::new("This Chip");
/// ```
#[derive(IntoElement, RegisterComponent)]
pub struct Chip {
    label: SharedString,
    label_color: Color,
    label_size: LabelSize,
    bg_color: Option<Hsla>,
    border_color: Option<Hsla>,
    height: Option<Pixels>,
    tooltip: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyView + 'static>>,
}

impl Chip {
    /// Creates a new `Chip` component with the specified label.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            label_color: Color::Default,
            label_size: LabelSize::XSmall,
            bg_color: None,
            border_color: None,
            height: None,
            tooltip: None,
        }
    }

    /// Sets the color of the label.
    pub fn label_color(mut self, color: Color) -> Self {
        self.label_color = color;
        self
    }

    /// Sets the size of the label.
    pub fn label_size(mut self, size: LabelSize) -> Self {
        self.label_size = size;
        self
    }

    /// Sets a custom background color for the callout content.
    pub fn bg_color(mut self, color: Hsla) -> Self {
        self.bg_color = Some(color);
        self
    }

    /// Sets a custom border color for the chip.
    pub fn border_color(mut self, color: Hsla) -> Self {
        self.border_color = Some(color);
        self
    }

    /// Sets a custom height for the chip.
    pub fn height(mut self, height: Pixels) -> Self {
        self.height = Some(height);
        self
    }

    pub fn tooltip(mut self, tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static) -> Self {
        self.tooltip = Some(Box::new(tooltip));
        self
    }
}

impl RenderOnce for Chip {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let bg_color = self
            .bg_color
            .unwrap_or(cx.theme().colors().element_background);

        let border_color = self.border_color.unwrap_or(cx.theme().colors().border);

        h_flex()
            .when_some(self.height, |this, h| this.h(h))
            .min_w_0()
            .flex_initial()
            .px_1()
            .border_1()
            .rounded_sm()
            .border_color(border_color)
            .bg(bg_color)
            .overflow_hidden()
            .child(
                Label::new(self.label.clone())
                    .size(self.label_size)
                    .color(self.label_color)
                    .buffer_font(cx)
                    .truncate(),
            )
            .id(self.label.clone())
            .when_some(self.tooltip, |this, tooltip| this.tooltip(tooltip))
    }
}

impl Component for Chip {
    fn scope() -> ComponentScope {
        ComponentScope::DataDisplay
    }

    fn preview(_window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let chip_examples = vec![
            single_example("Default", Chip::new("Chip Example").into_any_element()),
            single_example(
                "Customized Label Color",
                Chip::new("Chip Example")
                    .label_color(Color::Accent)
                    .into_any_element(),
            ),
            single_example(
                "Customized Label Size",
                Chip::new("Chip Example")
                    .label_size(LabelSize::Large)
                    .label_color(Color::Accent)
                    .into_any_element(),
            ),
            single_example(
                "Customized Background Color",
                Chip::new("Chip Example")
                    .bg_color(cx.theme().colors().text_accent.opacity(0.1))
                    .into_any_element(),
            ),
        ];

        Some(example_group(chip_examples).vertical().into_any_element())
    }
}
