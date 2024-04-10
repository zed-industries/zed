//! # UI – Text Field
//!
//! This crate provides a text field component that can be used to create text fields like search inputs, form fields, etc.
//!
//! It can't be located in the `ui` crate because it depends on `editor`.
//!

use editor::*;
use gpui::*;
use settings::Settings;
use theme::ThemeSettings;
use ui::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FieldLabelLayout {
    Inline,
    Stacked,
}

pub struct TextFieldStyle {
    text_color: Hsla,
    background_color: Hsla,
    border_color: Hsla,
}

/// A Text Field view that can be used to create text fields like search inputs, form fields, etc.
///
/// It wraps a single line [`Editor`] view and allows for common field properties like labels, placeholders, icons, etc.
pub struct TextField {
    /// An optional label for the text field.
    ///
    /// Its position is determined by the [`FieldLabelLayout`].
    label: Option<SharedString>,
    /// The placeholder text for the text field.
    ///
    /// All text fields must have placeholder text that is displayed when the field is empty.
    placeholder: SharedString,
    /// Exposes the underlying [`View<Editor>`] to allow for customizing the editor beyond the provided API.
    ///
    /// This likely will only be public in the short term, ideally the API will be expanded to cover necessary use cases.
    pub editor: View<Editor>,
    /// An optional icon that is displayed at the start of the text field.
    ///
    /// For example, a magnifying glass icon in a search field.
    start_icon: Option<IconName>,
    /// The layout of the label relative to the text field.
    label_layout: FieldLabelLayout,
}

impl FocusableView for TextField {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl TextField {
    pub fn new(placeholder: impl Into<SharedString>, cx: &mut WindowContext) -> Self {
        let placeholder_text = placeholder.into();

        let editor = cx.new_view(|cx| {
            let mut input = Editor::single_line(cx);
            input.set_placeholder_text(placeholder_text.clone(), cx);
            input
        });

        Self {
            label: None,
            placeholder: placeholder_text,
            editor,
            start_icon: None,
            label_layout: FieldLabelLayout::Stacked,
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn start_icon(mut self, icon: IconName) -> Self {
        self.start_icon = Some(icon);
        self
    }

    pub fn label_layout(mut self, layout: FieldLabelLayout) -> Self {
        self.label_layout = layout;
        self
    }
}

impl Render for TextField {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let theme_color = cx.theme().colors();

        let style = TextFieldStyle {
            text_color: theme_color.text,
            background_color: theme_color.ghost_element_background,
            border_color: theme_color.border,
        };

        // if self.disabled {
        //     style.text_color = theme_color.text_disabled;
        //     style.background_color = theme_color.ghost_element_disabled;
        //     style.border_color = theme_color.border_disabled;
        // }

        // if self.error_message.is_some() {
        //     style.text_color = cx.theme().status().error;
        //     style.border_color = cx.theme().status().error_border
        // }

        let text_style = TextStyle {
            font_family: settings.buffer_font.family.clone(),
            font_features: settings.buffer_font.features,
            font_size: rems(0.875).into(),
            font_weight: FontWeight::NORMAL,
            font_style: FontStyle::Normal,
            line_height: relative(1.2),
            color: style.text_color,
            ..Default::default()
        };

        let editor_style = EditorStyle {
            background: theme_color.ghost_element_background,
            local_player: cx.theme().players().local(),
            text: text_style,
            ..Default::default()
        };

        let stacked_label: Option<Label> = if self.label_layout == FieldLabelLayout::Stacked {
            self.label
                .clone()
                .map(|label| Label::new(label).size(LabelSize::Small))
        } else {
            None
        };

        let inline_label: Option<Label> = if self.label_layout == FieldLabelLayout::Inline {
            self.label
                .clone()
                .map(|label| Label::new(label).size(LabelSize::Small))
        } else {
            None
        };

        div()
            .when_some(stacked_label, |this, label| this.child(label))
            .child(
                v_flex()
                    .w_full()
                    .px_2()
                    .py_1()
                    .bg(style.background_color)
                    .text_color(style.text_color)
                    .rounded_lg()
                    .border()
                    .border_color(style.border_color)
                    .w_48()
                    .child(
                        h_flex()
                            .gap_2()
                            .when_some(inline_label, |this, label| this.child(label))
                            .child(
                                h_flex()
                                    .gap_1()
                                    .when_some(self.start_icon, |this, icon| {
                                        this.child(
                                            Icon::new(icon)
                                                .size(IconSize::Small)
                                                .color(Color::Muted),
                                        )
                                    })
                                    .child(EditorElement::new(&self.editor, editor_style)),
                            ),
                    ),
            )
    }
}
