use gpui::WindowContext;

use crate::{prelude::*, LabelCommon, LabelLike, LabelSize, LineHeightStyle};

/// A struct representing a label element in the UI.
///
/// The `Label` struct stores the label text and common properties for a label element.
/// It provides methods for modifying these properties.
///
/// # Examples
///
/// ```
/// Label::new("Hello, World!")
/// ```
///
/// **A colored label**, for example labeling a dangerous action:
///
/// ```
/// let my_label = Label::new("Delete").color(Color::Error);
/// ```
///
/// **A label with a strikethrough**, for example labeling something that has been deleted:
///
/// ```
/// let my_label = Label::new("Deleted").strikethrough(true);
/// ```
#[derive(IntoElement)]
pub struct Label {
    base: LabelLike,
    label: SharedString,
}

impl Label {
    /// Create a new `Label` with the given text.
    ///
    /// # Examples
    ///
    /// ```
    /// let my_label = Label::new("Hello, World!");
    /// ```
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            base: LabelLike::new(),
            label: label.into(),
        }
    }
}

impl LabelCommon for Label {
    /// Sets the size of the label using a [LabelSize].
    ///
    /// # Examples
    ///
    /// ```
    /// let my_label = Label::new("Hello, World!").size(LabelSize::Large);
    /// ```
    fn size(mut self, size: LabelSize) -> Self {
        self.base = self.base.size(size);
        self
    }

    /// Sets the line height style of the label using a [LineHeightStyle].
    ///
    /// # Examples
    ///
    /// ```
    /// let my_label = Label::new("Hello, World!").line_height_style(LineHeightStyle::Normal);
    /// ```
    fn line_height_style(mut self, line_height_style: LineHeightStyle) -> Self {
        self.base = self.base.line_height_style(line_height_style);
        self
    }

    /// Sets the color of the label using a [Color].
    ///
    /// # Examples
    ///
    /// ```
    /// let my_label = Label::new("Hello, World!").color(Color::Primary);
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
    /// let my_label = Label::new("Hello, World!").strikethrough(true);
    /// ```
    fn strikethrough(mut self, strikethrough: bool) -> Self {
        self.base = self.base.strikethrough(strikethrough);
        self
    }
}

impl RenderOnce for Label {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        self.base.child(self.label)
    }
}
