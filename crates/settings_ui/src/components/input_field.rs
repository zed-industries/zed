use editor::Editor;
use gpui::{Focusable, div};
use ui::{
    ActiveTheme as _, App, FluentBuilder as _, InteractiveElement as _, IntoElement,
    ParentElement as _, RenderOnce, Styled as _, Window,
};

#[derive(IntoElement)]
pub struct SettingsInputField {
    initial_text: Option<String>,
    placeholder: Option<&'static str>,
    confirm: Option<Box<dyn Fn(Option<String>, &mut App)>>,
    tab_index: Option<isize>,
}

// TODO: Update the `ui_input::InputField` to use `window.use_state` and `RenceOnce` and remove this component
impl SettingsInputField {
    pub fn new() -> Self {
        Self {
            initial_text: None,
            placeholder: None,
            confirm: None,
            tab_index: None,
        }
    }

    pub fn with_initial_text(mut self, initial_text: String) -> Self {
        self.initial_text = Some(initial_text);
        self
    }

    pub fn with_placeholder(mut self, placeholder: &'static str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    pub fn on_confirm(mut self, confirm: impl Fn(Option<String>, &mut App) + 'static) -> Self {
        self.confirm = Some(Box::new(confirm));
        self
    }

    pub(crate) fn tab_index(mut self, arg: isize) -> Self {
        self.tab_index = Some(arg);
        self
    }
}

impl RenderOnce for SettingsInputField {
    fn render(self, window: &mut Window, cx: &mut App) -> impl ui::IntoElement {
        let editor = window.use_state(cx, {
            move |window, cx| {
                let mut editor = Editor::single_line(window, cx);
                if let Some(text) = self.initial_text {
                    editor.set_text(text, window, cx);
                }

                if let Some(placeholder) = self.placeholder {
                    editor.set_placeholder_text(placeholder, window, cx);
                }
                // todo(settings_ui): We should have an observe global use for settings store
                // so whenever a settings file is updated, the settings ui updates too
                editor
            }
        });

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
                    move |_, _, cx| {
                        let Some(editor) = weak_editor.upgrade() else {
                            return;
                        };
                        let new_value = editor.read_with(cx, |editor, cx| editor.text(cx));
                        let new_value = (!new_value.is_empty()).then_some(new_value);
                        confirm(new_value, cx);
                    }
                })
            })
    }
}
