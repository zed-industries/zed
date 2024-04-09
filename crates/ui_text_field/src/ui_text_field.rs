use editor::*;
use gpui::*;
use settings::Settings;
use theme::ThemeSettings;
use ui::*;

#[derive(Default)]
pub enum FieldLabelLayout {
    Inline,
    #[default]
    Stacked,
}

pub struct TextFieldStyle {
    text_color: Hsla,
    background_color: Hsla,
    border_color: Hsla,
}

pub struct TextField {
    label: Option<SharedString>,
    placeholder: SharedString,
    /// Short term hack: This probably won't stay pub forever
    pub editor: View<Editor>,
    start_icon: Option<IconName>,
    error_message: Option<SharedString>,
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
            error_message: None,
            label_layout: FieldLabelLayout::default(),
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
}

impl Render for TextField {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let theme_color = cx.theme().colors();

        let mut style = TextFieldStyle {
            text_color: theme_color.text,
            background_color: theme_color.ghost_element_background,
            border_color: theme_color.border_focused,
        };

        // if self.disabled {
        //     style.text_color = theme_color.text_disabled;
        //     style.background_color = theme_color.ghost_element_disabled;
        //     style.border_color = theme_color.border_disabled;
        // }

        if self.error_message.is_some() {
            style.text_color = cx.theme().status().error;
            style.border_color = cx.theme().status().error_border
        }

        let text_style = TextStyle {
            font_family: settings.buffer_font.family.clone(),
            font_features: settings.buffer_font.features,
            font_size: rems(0.875).into(),
            font_weight: FontWeight::NORMAL,
            font_style: FontStyle::Normal,
            line_height: relative(1.),
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
            .when_some(self.label.clone(), |this, label| {
                this.child(Label::new(label).size(LabelSize::Small))
            })
            .child(
                v_flex()
                    .w_full()
                    .p_2()
                    .bg(style.background_color)
                    .text_color(style.text_color)
                    .rounded_md()
                    .border()
                    .border_color(style.border_color)
                    .w_48()
                    .child(
                        h_flex()
                            .gap_1()
                            .when_some(self.start_icon, |this, icon| {
                                this.child(
                                    Icon::new(icon).size(IconSize::Small).color(Color::Muted),
                                )
                            })
                            .child(EditorElement::new(&self.editor, editor_style)),
                    ),
            )
    }
}
