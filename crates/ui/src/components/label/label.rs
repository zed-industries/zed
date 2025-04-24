use crate::{LabelLike, prelude::*};
use gpui::StyleRefinement;

/// A struct representing a label element in the UI.
///
/// The `Label` struct stores the label text and common properties for a label element.
/// It provides methods for modifying these properties.
///
/// # Examples
///
/// ```
/// use ui::prelude::*;
///
/// Label::new("Hello, World!");
/// ```
///
/// **A colored label**, for example labeling a dangerous action:
///
/// ```
/// use ui::prelude::*;
///
/// let my_label = Label::new("Delete").color(Color::Error);
/// ```
///
/// **A label with a strikethrough**, for example labeling something that has been deleted:
///
/// ```
/// use ui::prelude::*;
///
/// let my_label = Label::new("Deleted").strikethrough(true);
/// ```
#[derive(IntoElement, RegisterComponent)]
pub struct Label {
    base: LabelLike,
    label: SharedString,
}

impl Label {
    /// Creates a new [`Label`] with the given text.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let my_label = Label::new("Hello, World!");
    /// ```
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            base: LabelLike::new(),
            label: label.into(),
        }
    }

    /// Sets the text of the [`Label`].
    pub fn set_text(&mut self, text: impl Into<SharedString>) {
        self.label = text.into();
    }
}

// Style methods.
impl Label {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.base.style()
    }

    gpui::margin_style_methods!({
        visibility: pub
    });
}

impl LabelCommon for Label {
    /// Sets the size of the label using a [`LabelSize`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let my_label = Label::new("Hello, World!").size(LabelSize::Small);
    /// ```
    fn size(mut self, size: LabelSize) -> Self {
        self.base = self.base.size(size);
        self
    }

    /// Sets the weight of the label using a [`FontWeight`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let my_label = Label::new("Hello, World!").weight(FontWeight::Bold);
    /// ```
    fn weight(mut self, weight: gpui::FontWeight) -> Self {
        self.base = self.base.weight(weight);
        self
    }

    /// Sets the line height style of the label using a [`LineHeightStyle`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let my_label = Label::new("Hello, World!").line_height_style(LineHeightStyle::UiLabel);
    /// ```
    fn line_height_style(mut self, line_height_style: LineHeightStyle) -> Self {
        self.base = self.base.line_height_style(line_height_style);
        self
    }

    /// Sets the color of the label using a [`Color`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let my_label = Label::new("Hello, World!").color(Color::Accent);
    /// ```
    fn color(mut self, color: Color) -> Self {
        self.base = self.base.color(color);
        self
    }

    /// Sets the strikethrough property of the label.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let my_label = Label::new("Hello, World!").strikethrough(true);
    /// ```
    fn strikethrough(mut self) -> Self {
        self.base = self.base.strikethrough();
        self
    }

    /// Sets the italic property of the label.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let my_label = Label::new("Hello, World!").italic(true);
    /// ```
    fn italic(mut self) -> Self {
        self.base = self.base.italic();
        self
    }

    /// Sets the alpha property of the color of label.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let my_label = Label::new("Hello, World!").alpha(0.5);
    /// ```
    fn alpha(mut self, alpha: f32) -> Self {
        self.base = self.base.alpha(alpha);
        self
    }

    fn underline(mut self) -> Self {
        self.base = self.base.underline();
        self
    }

    /// Truncates overflowing text with an ellipsis (`…`) if needed.
    fn truncate(mut self) -> Self {
        self.base = self.base.truncate();
        self
    }

    fn single_line(mut self) -> Self {
        self.label = SharedString::from(self.label.replace('\n', "⏎"));
        self.base = self.base.single_line();
        self
    }

    fn buffer_font(mut self, cx: &App) -> Self {
        self.base = self.base.buffer_font(cx);
        self
    }

    /// Styles the label to look like inline code.
    fn inline_code(mut self, cx: &App) -> Self {
        self.base = self.base.inline_code(cx);
        self
    }
}

impl RenderOnce for Label {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.base.child(self.label)
    }
}

impl Component for Label {
    fn scope() -> ComponentScope {
        ComponentScope::Typography
    }

    fn description() -> Option<&'static str> {
        Some("A text label component that supports various styles, sizes, and formatting options.")
    }

    fn preview(_window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .gap_6()
                .children(vec![
                    example_group_with_title(
                        "Sizes",
                        vec![
                            single_example("Default", Label::new("Project Explorer").into_any_element()),
                            single_example("Small", Label::new("File: main.rs").size(LabelSize::Small).into_any_element()),
                            single_example("Large", Label::new("Welcome to Zed").size(LabelSize::Large).into_any_element()),
                        ],
                    ),
                    example_group_with_title(
                        "Colors",
                        vec![
                            single_example("Default", Label::new("Status: Ready").into_any_element()),
                            single_example("Accent", Label::new("New Update Available").color(Color::Accent).into_any_element()),
                            single_example("Error", Label::new("Build Failed").color(Color::Error).into_any_element()),
                        ],
                    ),
                    example_group_with_title(
                        "Styles",
                        vec![
                            single_example("Default", Label::new("Normal Text").into_any_element()),
                            single_example("Bold", Label::new("Important Notice").weight(gpui::FontWeight::BOLD).into_any_element()),
                            single_example("Italic", Label::new("Code Comment").italic().into_any_element()),
                            single_example("Strikethrough", Label::new("Deprecated Feature").strikethrough().into_any_element()),
                            single_example("Underline", Label::new("Clickable Link").underline().into_any_element()),
                            single_example("Inline Code", Label::new("fn main() {}").inline_code(cx).into_any_element()),
                        ],
                    ),
                    example_group_with_title(
                        "Line Height Styles",
                        vec![
                            single_example("Default", Label::new("Multi-line\nText\nExample").into_any_element()),
                            single_example("UI Label", Label::new("Compact\nUI\nLabel").line_height_style(LineHeightStyle::UiLabel).into_any_element()),
                        ],
                    ),
                    example_group_with_title(
                        "Special Cases",
                        vec![
                            single_example("Single Line", Label::new("Line 1\nLine 2\nLine 3").single_line().into_any_element()),
                            single_example("Text Ellipsis", div().max_w_24().child(Label::new("This is a very long file name that should be truncated: very_long_file_name_with_many_words.rs").truncate()).into_any_element()),
                        ],
                    ),
                ])
                .into_any_element()
        )
    }
}
