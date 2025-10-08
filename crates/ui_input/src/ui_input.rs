//! # UI – Text Field
//!
//! This crate provides a text field component that can be used to create text fields like search inputs, form fields, etc.
//!
//! It can't be located in the `ui` crate because it depends on `editor`.
//!
mod font_picker;
mod number_field;

use component::{example_group, single_example};
use editor::{Editor, EditorElement, EditorStyle};
pub use font_picker::*;
use gpui::{App, Entity, FocusHandle, Focusable, FontStyle, Hsla, Length, TextStyle};
pub use number_field::*;
use settings::Settings;
use std::sync::Arc;
use theme::ThemeSettings;
use ui::prelude::*;

pub struct SingleLineInputStyle {
    text_color: Hsla,
    background_color: Hsla,
    border_color: Hsla,
}

/// A Text Field that can be used to create text fields like search inputs, form fields, etc.
///
/// It wraps a single line [`Editor`] and allows for common field properties like labels, placeholders, icons, etc.
#[derive(RegisterComponent)]
pub struct SingleLineInput {
    /// An optional label for the text field.
    ///
    /// Its position is determined by the [`FieldLabelLayout`].
    label: Option<SharedString>,
    /// The size of the label text.
    label_size: LabelSize,
    /// The placeholder text for the text field.
    placeholder: SharedString,
    /// Exposes the underlying [`Entity<Editor>`] to allow for customizing the editor beyond the provided API.
    ///
    /// This likely will only be public in the short term, ideally the API will be expanded to cover necessary use cases.
    pub editor: Entity<Editor>,
    /// An optional icon that is displayed at the start of the text field.
    ///
    /// For example, a magnifying glass icon in a search field.
    start_icon: Option<IconName>,
    /// Whether the text field is disabled.
    disabled: bool,
    /// The minimum width of for the input
    min_width: Length,
}

impl Focusable for SingleLineInput {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl SingleLineInput {
    pub fn new(window: &mut Window, cx: &mut App, placeholder: impl Into<SharedString>) -> Self {
        let placeholder_text = placeholder.into();

        let editor = cx.new(|cx| {
            let mut input = Editor::single_line(window, cx);
            input.set_placeholder_text(&placeholder_text, window, cx);
            input
        });

        Self {
            label: None,
            label_size: LabelSize::Small,
            placeholder: placeholder_text,
            editor,
            start_icon: None,
            disabled: false,
            min_width: px(192.).into(),
        }
    }

    pub fn start_icon(mut self, icon: IconName) -> Self {
        self.start_icon = Some(icon);
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn label_size(mut self, size: LabelSize) -> Self {
        self.label_size = size;
        self
    }

    pub fn label_min_width(mut self, width: impl Into<Length>) -> Self {
        self.min_width = width.into();
        self
    }

    pub fn set_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        self.disabled = disabled;
        self.editor
            .update(cx, |editor, _| editor.set_read_only(disabled))
    }

    pub fn is_empty(&self, cx: &App) -> bool {
        self.editor().read(cx).text(cx).trim().is_empty()
    }

    pub fn editor(&self) -> &Entity<Editor> {
        &self.editor
    }

    pub fn text(&self, cx: &App) -> String {
        self.editor().read(cx).text(cx)
    }

    pub fn set_text(&self, text: impl Into<Arc<str>>, window: &mut Window, cx: &mut App) {
        self.editor()
            .update(cx, |editor, cx| editor.set_text(text, window, cx))
    }
}

impl Render for SingleLineInput {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let theme_color = cx.theme().colors();

        let mut style = SingleLineInputStyle {
            text_color: theme_color.text,
            background_color: theme_color.editor_background,
            border_color: theme_color.border_variant,
        };

        if self.disabled {
            style.text_color = theme_color.text_disabled;
            style.background_color = theme_color.editor_background;
            style.border_color = theme_color.border_disabled;
        }

        // if self.error_message.is_some() {
        //     style.text_color = cx.theme().status().error;
        //     style.border_color = cx.theme().status().error_border
        // }

        let text_style = TextStyle {
            font_family: settings.ui_font.family.clone(),
            font_features: settings.ui_font.features.clone(),
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
            syntax: cx.theme().syntax().clone(),
            text: text_style,
            ..Default::default()
        };

        v_flex()
            .id(self.placeholder.clone())
            .w_full()
            .gap_1()
            .when_some(self.label.clone(), |this, label| {
                this.child(
                    Label::new(label)
                        .size(self.label_size)
                        .color(if self.disabled {
                            Color::Disabled
                        } else {
                            Color::Default
                        }),
                )
            })
            .child(
                h_flex()
                    .min_w(self.min_width)
                    .min_h_8()
                    .w_full()
                    .px_2()
                    .py_1p5()
                    .flex_grow()
                    .text_color(style.text_color)
                    .rounded_md()
                    .bg(style.background_color)
                    .border_1()
                    .border_color(style.border_color)
                    .when_some(self.start_icon, |this, icon| {
                        this.gap_1()
                            .child(Icon::new(icon).size(IconSize::Small).color(Color::Muted))
                    })
                    .child(EditorElement::new(&self.editor, editor_style)),
            )
    }
}

impl Component for SingleLineInput {
    fn scope() -> ComponentScope {
        ComponentScope::Input
    }

    fn preview(window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let input_small =
            cx.new(|cx| SingleLineInput::new(window, cx, "placeholder").label("Small Label"));

        let input_regular = cx.new(|cx| {
            SingleLineInput::new(window, cx, "placeholder")
                .label("Regular Label")
                .label_size(LabelSize::Default)
        });

        Some(
            v_flex()
                .gap_6()
                .children(vec![example_group(vec![
                    single_example(
                        "Small Label (Default)",
                        div().child(input_small).into_any_element(),
                    ),
                    single_example(
                        "Regular Label",
                        div().child(input_regular).into_any_element(),
                    ),
                ])])
                .into_any_element(),
        )
    }
}
