use crate::prelude::*;
use gpui::{AnyElement, IntoElement, ParentElement, Styled};

/// InlineCode mimics the way inline code is rendered when wrapped in backticks in Markdown.
///
/// # Usage Example
///
/// ```
/// use ui::InlineCode;
///
/// let InlineCode = InlineCode::new("<div>hey</div>");
/// ```
#[derive(IntoElement, RegisterComponent)]
pub struct InlineCode {
    label: SharedString,
    label_size: LabelSize,
}

impl InlineCode {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            label_size: LabelSize::Default,
        }
    }

    /// Sets the size of the label.
    pub fn label_size(mut self, size: LabelSize) -> Self {
        self.label_size = size;
        self
    }
}

impl RenderOnce for InlineCode {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        h_flex()
            .min_w_0()
            .px_0p5()
            .overflow_hidden()
            .bg(cx.theme().colors().text.opacity(0.05))
            .child(Label::new(self.label).size(self.label_size).buffer_font(cx))
    }
}

impl Component for InlineCode {
    fn scope() -> ComponentScope {
        ComponentScope::DataDisplay
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .gap_6()
                .child(
                    example_group(vec![single_example(
                        "Simple",
                        InlineCode::new("zed.dev").into_any_element(),
                    )])
                    .vertical(),
                )
                .into_any_element(),
        )
    }
}
