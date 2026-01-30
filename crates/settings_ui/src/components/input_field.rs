use editor::Editor;
use gpui::{ElementId, Focusable, TextStyleRefinement, div};
use settings::Settings as _;
use theme::ThemeSettings;
use ui::{prelude::*, rems};

#[derive(IntoElement)]
pub struct SettingsInputField {
    id: Option<ElementId>,
    initial_text: Option<String>,
    placeholder: Option<&'static str>,
    confirm: Option<Box<dyn Fn(Option<String>, &mut Window, &mut App)>>,
    tab_index: Option<isize>,
    use_buffer_font: bool,
}

impl SettingsInputField {
    pub fn new() -> Self {
        Self {
            id: None,
            initial_text: None,
            placeholder: None,
            confirm: None,
            tab_index: None,
            use_buffer_font: false,
        }
    }

    pub fn with_id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn with_initial_text(mut self, initial_text: String) -> Self {
        self.initial_text = Some(initial_text);
        self
    }

    pub fn with_placeholder(mut self, placeholder: &'static str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    pub fn on_confirm(
        mut self,
        confirm: impl Fn(Option<String>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.confirm = Some(Box::new(confirm));
        self
    }

    pub(crate) fn tab_index(mut self, arg: isize) -> Self {
        self.tab_index = Some(arg);
        self
    }

    pub fn with_buffer_font(mut self) -> Self {
        self.use_buffer_font = true;
        self
    }
}

impl RenderOnce for SettingsInputField {
    fn render(self, window: &mut Window, cx: &mut App) -> impl ui::IntoElement {
        let use_buffer_font = self.use_buffer_font;

        let editor = if let Some(id) = self.id {
            window.use_keyed_state(id, cx, {
                let initial_text = self.initial_text.clone();
                let placeholder = self.placeholder;
                move |window, cx| {
                    let mut editor = Editor::single_line(window, cx);
                    if let Some(text) = initial_text {
                        editor.set_text(text, window, cx);
                    }

                    if let Some(placeholder) = placeholder {
                        editor.set_placeholder_text(placeholder, window, cx);
                    }
                    if use_buffer_font {
                        let settings = ThemeSettings::get_global(cx);
                        editor.set_text_style_refinement(TextStyleRefinement {
                            font_family: Some(settings.buffer_font.family.clone()),
                            font_size: Some(rems(0.75).into()),
                            ..Default::default()
                        });
                    }
                    editor
                }
            })
        } else {
            window.use_state(cx, {
                let initial_text = self.initial_text.clone();
                let placeholder = self.placeholder;
                move |window, cx| {
                    let mut editor = Editor::single_line(window, cx);
                    if let Some(text) = initial_text {
                        editor.set_text(text, window, cx);
                    }

                    if let Some(placeholder) = placeholder {
                        editor.set_placeholder_text(placeholder, window, cx);
                    }
                    if use_buffer_font {
                        let settings = ThemeSettings::get_global(cx);
                        editor.set_text_style_refinement(TextStyleRefinement {
                            font_family: Some(settings.buffer_font.family.clone()),
                            font_size: Some(rems(0.75).into()),
                            ..Default::default()
                        });
                    }
                    editor
                }
            })
        };

        let weak_editor = editor.downgrade();

        let theme_colors = cx.theme().colors();

        div()
            .py_1()
            .px_2()
            .min_w_64()
            .rounded_md()
            .border_1()
            .border_color(theme_colors.border)
            .bg(theme_colors.editor_background)
            .when_some(self.tab_index, |this, tab_index| {
                let focus_handle = editor.focus_handle(cx).tab_index(tab_index).tab_stop(true);
                this.track_focus(&focus_handle)
                    .focus(|s| s.border_color(theme_colors.border_focused))
            })
            .child(editor)
            .when_some(self.confirm, |this, confirm| {
                this.on_action::<menu::Confirm>({
                    move |_, window, cx| {
                        let Some(editor) = weak_editor.upgrade() else {
                            return;
                        };
                        let new_value = editor.read_with(cx, |editor, cx| editor.text(cx));
                        let new_value = (!new_value.is_empty()).then_some(new_value);
                        confirm(new_value, window, cx);
                    }
                })
            })
    }
}
