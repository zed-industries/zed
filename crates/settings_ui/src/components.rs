use editor::Editor;
use gpui::div;
use ui::{
    ActiveTheme as _, App, FluentBuilder as _, InteractiveElement as _, IntoElement,
    ParentElement as _, RenderOnce, Styled as _, Window,
};

#[derive(IntoElement)]
pub struct SettingsEditor {
    initial_text: String,
    confirm: Option<Box<dyn Fn(Option<String>, &mut App)>>,
}

impl SettingsEditor {
    pub fn new(initial_text: String) -> Self {
        Self {
            initial_text,
            confirm: None,
        }
    }

    pub fn on_confirm(mut self, confirm: impl Fn(Option<String>, &mut App) + 'static) -> Self {
        self.confirm = Some(Box::new(confirm));
        self
    }
}

impl RenderOnce for SettingsEditor {
    fn render(self, window: &mut Window, cx: &mut App) -> impl ui::IntoElement {
        let editor = window.use_state(cx, {
            move |window, cx| {
                let mut editor = Editor::single_line(window, cx);
                editor.set_text(self.initial_text, window, cx);
                editor
            }
        });

        let weak_editor = editor.downgrade();
        let theme_colors = cx.theme().colors();

        div()
            .child(editor)
            .bg(theme_colors.editor_background)
            .border_1()
            .rounded_lg()
            .border_color(theme_colors.border)
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
