use component::{example_group, single_example};

use gpui::{App, FocusHandle, Focusable, Hsla, Length};
use std::sync::Arc;

use ui::prelude::*;

use crate::ErasedEditor;

pub struct InputFieldStyle {
    text_color: Hsla,
    background_color: Hsla,
    border_color: Hsla,
}

/// An Input Field component that can be used to create text fields like search inputs, form fields, etc.
///
/// It wraps a single line [`Editor`] and allows for common field properties like labels, placeholders, icons, etc.
#[derive(RegisterComponent)]
pub struct InputField {
    /// An optional label for the text field.
    ///
    /// Its position is determined by the [`FieldLabelLayout`].
    label: Option<SharedString>,
    /// The size of the label text.
    label_size: LabelSize,
    /// The placeholder text for the text field.
    placeholder: SharedString,

    editor: Arc<dyn ErasedEditor>,
    /// An optional icon that is displayed at the start of the text field.
    ///
    /// For example, a magnifying glass icon in a search field.
    start_icon: Option<IconName>,
    /// The minimum width of for the input
    min_width: Length,
    /// The tab index for keyboard navigation order.
    tab_index: Option<isize>,
    /// Whether this field is a tab stop (can be focused via Tab key).
    tab_stop: bool,
}

impl Focusable for InputField {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl InputField {
    pub fn new(window: &mut Window, cx: &mut App, placeholder_text: &str) -> Self {
        let editor_factory = crate::ERASED_EDITOR_FACTORY
            .get()
            .expect("ErasedEditorFactory to be initialized");
        let editor = (editor_factory)(window, cx);
        editor.set_placeholder_text(placeholder_text, window, cx);

        Self {
            label: None,
            label_size: LabelSize::Small,
            placeholder: SharedString::new(placeholder_text),
            editor,
            start_icon: None,
            min_width: px(192.).into(),
            tab_index: None,
            tab_stop: true,
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

    pub fn tab_index(mut self, index: isize) -> Self {
        self.tab_index = Some(index);
        self
    }

    pub fn tab_stop(mut self, tab_stop: bool) -> Self {
        self.tab_stop = tab_stop;
        self
    }

    pub fn is_empty(&self, cx: &App) -> bool {
        self.editor().text(cx).trim().is_empty()
    }

    pub fn editor(&self) -> &Arc<dyn ErasedEditor> {
        &self.editor
    }

    pub fn text(&self, cx: &App) -> String {
        self.editor().text(cx)
    }

    pub fn clear(&self, window: &mut Window, cx: &mut App) {
        self.editor().clear(window, cx)
    }

    pub fn set_text(&self, text: &str, window: &mut Window, cx: &mut App) {
        self.editor().set_text(text, window, cx)
    }
}

impl Render for InputField {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let editor = self.editor.clone();

        let theme_color = cx.theme().colors();

        let style = InputFieldStyle {
            text_color: theme_color.text,
            background_color: theme_color.editor_background,
            border_color: theme_color.border_variant,
        };

        let focus_handle = self.editor.focus_handle(cx);

        let configured_handle = if let Some(tab_index) = self.tab_index {
            focus_handle.tab_index(tab_index).tab_stop(self.tab_stop)
        } else if !self.tab_stop {
            focus_handle.tab_stop(false)
        } else {
            focus_handle
        };

        v_flex()
            .id(self.placeholder.clone())
            .w_full()
            .gap_1()
            .when_some(self.label.clone(), |this, label| {
                this.child(
                    Label::new(label)
                        .size(self.label_size)
                        .color(Color::Default),
                )
            })
            .child(
                h_flex()
                    .track_focus(&configured_handle)
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
                    .when(
                        editor.focus_handle(cx).contains_focused(window, cx),
                        |this| this.border_color(theme_color.border_focused),
                    )
                    .when_some(self.start_icon, |this, icon| {
                        this.gap_1()
                            .child(Icon::new(icon).size(IconSize::Small).color(Color::Muted))
                    })
                    .child(self.editor.render(window, cx)),
            )
    }
}

impl Component for InputField {
    fn scope() -> ComponentScope {
        ComponentScope::Input
    }

    fn preview(window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let input_small =
            cx.new(|cx| InputField::new(window, cx, "placeholder").label("Small Label"));

        let input_regular = cx.new(|cx| {
            InputField::new(window, cx, "placeholder")
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
