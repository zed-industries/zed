use crate::prelude::*;
use gpui::{FontWeight, StyleRefinement, UnderlineStyle};
use settings::Settings;
use smallvec::SmallVec;
use theme::ThemeSettings;

/// Convenience function, returns a default [`Label`]
pub fn label(label: impl Into<String>) -> Label {
    Label::new(label.into())
}

/// Convenience function, returns a [`Label`] with [`Color::Muted`] applied.
pub fn label_muted(label: impl Into<String>) -> Label {
    Label::new(label.into()).color(Color::Muted)
}

/// Convenience function, returns a [`Label`] with [`LabelSize::Small`] applied.
pub fn label_sm(label: impl Into<String>) -> Label {
    Label::new(label.into()).size(LabelSize::Small)
}

/// Convenience function, returns a [`Label`] with [`LabelSize::XSmall`] applied.
pub fn label_xs(label: impl Into<String>) -> Label {
    Label::new(label.into()).size(LabelSize::XSmall)
}

/// Convenience function, returns a [`Label`] with [`LabelSize::Large`] applied.
pub fn label_lg(label: impl Into<String>) -> Label {
    Label::new(label.into()).size(LabelSize::Large)
}

/// Sets the size of a label
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum LabelSize {
    /// The default size of a label.
    #[default]
    Default,
    /// The large size of a label.
    Large,
    /// The small size of a label.
    Small,
    /// The extra small size of a label.
    XSmall,
}

/// Sets the line height of a label
#[derive(Default, PartialEq, Copy, Clone)]
pub enum LineHeightStyle {
    /// The default line height style of a label,
    /// set by either the UI's default line height,
    /// or the developer's default buffer line height.
    #[default]
    TextLabel,
    /// Sets the line height to 1.
    UiLabel,
}

/// A common set of traits all labels must implement.
pub trait LabelCommon {
    /// Sets the size of the label using a [`LabelSize`].
    fn size(self, size: LabelSize) -> Self;

    /// Sets the font weight of the label.
    fn weight(self, weight: FontWeight) -> Self;

    /// Sets the line height style of the label using a [`LineHeightStyle`].
    fn line_height_style(self, line_height_style: LineHeightStyle) -> Self;

    /// Sets the color of the label using a [`Color`].
    fn color(self, color: Color) -> Self;

    /// Sets the strikethrough property of the label.
    fn strikethrough(self) -> Self;

    /// Sets the italic property of the label.
    fn italic(self) -> Self;

    /// Sets the underline property of the label
    fn underline(self) -> Self;

    /// Sets the alpha property of the label, overwriting the alpha value of the color.
    fn alpha(self, alpha: f32) -> Self;

    /// Truncates overflowing text with an ellipsis (`…`) if needed.
    fn text_ellipsis(self) -> Self;

    /// Sets the label to render as a single line.
    fn single_line(self) -> Self;

    /// Sets the font to the buffer's
    fn buffer_font(self, cx: &App) -> Self;
}

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
#[derive(IntoElement, IntoComponent)]
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

    fn text_ellipsis(mut self) -> Self {
        self.base = self.base.text_ellipsis();
        self
    }

    fn single_line(mut self) -> Self {
        self.label = SharedString::from(self.label.replace('\n', "␤"));
        self.base = self.base.single_line();
        self
    }

    fn buffer_font(mut self, cx: &App) -> Self {
        self.base = self.base.buffer_font(cx);
        self
    }
}

impl RenderOnce for Label {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.base.child(self.label)
    }
}

/// A label-like element that can be used to create a custom label when
/// prebuilt labels are not sufficient. Use this sparingly, as it is
/// unconstrained and may make the UI feel less consistent.
///
/// This is also used to build the prebuilt labels.
#[derive(IntoElement)]
pub struct LabelLike {
    pub(super) base: Div,
    size: LabelSize,
    weight: Option<FontWeight>,
    line_height_style: LineHeightStyle,
    pub(crate) color: Color,
    strikethrough: bool,
    italic: bool,
    children: SmallVec<[AnyElement; 2]>,
    alpha: Option<f32>,
    underline: bool,
    single_line: bool,
    text_ellipsis: bool,
}

impl Default for LabelLike {
    fn default() -> Self {
        Self::new()
    }
}

impl LabelLike {
    /// Creates a new, fully custom label.
    /// Prefer using [`Label`] or [`HighlightedLabel`] where possible.
    pub fn new() -> Self {
        Self {
            base: div(),
            size: LabelSize::Default,
            weight: None,
            line_height_style: LineHeightStyle::default(),
            color: Color::Default,
            strikethrough: false,
            italic: false,
            children: SmallVec::new(),
            alpha: None,
            underline: false,
            single_line: false,
            text_ellipsis: false,
        }
    }
}

// Style methods.
impl LabelLike {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }

    gpui::margin_style_methods!({
        visibility: pub
    });
}

impl LabelCommon for LabelLike {
    fn size(mut self, size: LabelSize) -> Self {
        self.size = size;
        self
    }

    fn weight(mut self, weight: FontWeight) -> Self {
        self.weight = Some(weight);
        self
    }

    fn line_height_style(mut self, line_height_style: LineHeightStyle) -> Self {
        self.line_height_style = line_height_style;
        self
    }

    fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }

    fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    fn alpha(mut self, alpha: f32) -> Self {
        self.alpha = Some(alpha);
        self
    }

    fn text_ellipsis(mut self) -> Self {
        self.text_ellipsis = true;
        self
    }

    fn single_line(mut self) -> Self {
        self.single_line = true;
        self
    }

    fn buffer_font(mut self, cx: &App) -> Self {
        self.base = self
            .base
            .font(theme::ThemeSettings::get_global(cx).buffer_font.clone());
        self
    }
}

impl ParentElement for LabelLike {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for LabelLike {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let mut color = self.color.color(cx);
        if let Some(alpha) = self.alpha {
            color.fade_out(1.0 - alpha);
        }

        self.base
            .map(|this| match self.size {
                LabelSize::Large => this.text_ui_lg(cx),
                LabelSize::Default => this.text_ui(cx),
                LabelSize::Small => this.text_ui_sm(cx),
                LabelSize::XSmall => this.text_ui_xs(cx),
            })
            .when(self.line_height_style == LineHeightStyle::UiLabel, |this| {
                this.line_height(relative(1.))
            })
            .when(self.italic, |this| this.italic())
            .when(self.underline, |mut this| {
                this.text_style()
                    .get_or_insert_with(Default::default)
                    .underline = Some(UnderlineStyle {
                    thickness: px(1.),
                    color: None,
                    wavy: false,
                });
                this
            })
            .when(self.strikethrough, |this| this.line_through())
            .when(self.single_line, |this| this.whitespace_nowrap())
            .when(self.text_ellipsis, |this| {
                this.overflow_x_hidden().text_ellipsis()
            })
            .text_color(color)
            .font_weight(
                self.weight
                    .unwrap_or(ThemeSettings::get_global(cx).ui_font.weight),
            )
            .children(self.children)
    }
}

mod label_preview {
    use crate::prelude::*;

    impl ComponentPreview for Label {
        fn preview(_window: &mut Window, _cx: &App) -> AnyElement {
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
                            single_example("Text Ellipsis", div().max_w_24().child(Label::new("This is a very long file name that should be truncated: very_long_file_name_with_many_words.rs").text_ellipsis()).into_any_element()),
                        ],
                    ),
                ])
                .into_any_element()
        }
    }
}
