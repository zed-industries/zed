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
    Hidden,
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
    label: SharedString,
    /// The placeholder text for the text field.
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
    with_label: FieldLabelLayout,
    /// Whether the text field is disabled.
    disabled: bool,
}

impl FocusableView for TextField {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl TextField {
    pub fn new(
        cx: &mut WindowContext,
        label: impl Into<SharedString>,
        placeholder: impl Into<SharedString>,
    ) -> Self {
        let placeholder_text = placeholder.into();

        let editor = cx.new_view(|cx| {
            let mut input = Editor::single_line(cx);
            input.set_placeholder_text(placeholder_text.clone(), cx);
            input
        });

        Self {
            label: label.into(),
            placeholder: placeholder_text,
            editor,
            start_icon: None,
            with_label: FieldLabelLayout::Hidden,
            disabled: false,
        }
    }

    pub fn start_icon(mut self, icon: IconName) -> Self {
        self.start_icon = Some(icon);
        self
    }

    pub fn with_label(mut self, layout: FieldLabelLayout) -> Self {
        self.with_label = layout;
        self
    }

    pub fn set_disabled(&mut self, disabled: bool, cx: &mut ViewContext<Self>) {
        self.disabled = disabled;
        self.editor
            .update(cx, |editor, _| editor.set_read_only(disabled))
    }

    pub fn editor(&self) -> &View<Editor> {
        &self.editor
    }
}

impl Render for TextField {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let theme_color = cx.theme().colors();

        let mut style = TextFieldStyle {
            text_color: theme_color.text,
            background_color: theme_color.ghost_element_background,
            border_color: theme_color.border,
        };

        if self.disabled {
            style.text_color = theme_color.text_disabled;
            style.background_color = theme_color.ghost_element_disabled;
            style.border_color = theme_color.border_disabled;
        }

        // if self.error_message.is_some() {
        //     style.text_color = cx.theme().status().error;
        //     style.border_color = cx.theme().status().error_border
        // }

        let text_style = TextStyle {
            font_family: settings.buffer_font.family.clone(),
            font_features: settings.buffer_font.features.clone(),
            font_size: rems(0.875).into(),
            font_weight: settings.buffer_font.weight,
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

        div()
            .id(self.placeholder.clone())
            .group("text-field")
            .w_full()
            .when(self.with_label == FieldLabelLayout::Stacked, |this| {
                this.child(
                    Label::new(self.label.clone())
                        .size(LabelSize::Default)
                        .color(if self.disabled {
                            Color::Disabled
                        } else {
                            Color::Muted
                        }),
                )
            })
            .child(
                v_flex().w_full().child(
                    h_flex()
                        .w_full()
                        .flex_grow()
                        .gap_2()
                        .when(self.with_label == FieldLabelLayout::Inline, |this| {
                            this.child(Label::new(self.label.clone()).size(LabelSize::Default))
                        })
                        .child(
                            h_flex()
                                .px_2()
                                .py_1()
                                .bg(style.background_color)
                                .text_color(style.text_color)
                                .rounded_lg()
                                .border_1()
                                .border_color(style.border_color)
                                .min_w_48()
                                .w_full()
                                .flex_grow()
                                .gap_1()
                                .when_some(self.start_icon, |this, icon| {
                                    this.child(
                                        Icon::new(icon).size(IconSize::Small).color(Color::Muted),
                                    )
                                })
                                .child(EditorElement::new(&self.editor, editor_style)),
                        ),
                ),
            )
    }
}
