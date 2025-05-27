use editor::Editor;
use gpui::{
    App, AppContext as _, Context, Entity, IntoElement, KeyBinding, ParentElement, Render, Styled,
    Window, div, white,
};

pub struct AutoHeightEditorStory {
    editor: Entity<Editor>,
}

impl AutoHeightEditorStory {
    pub fn new(window: &mut Window, cx: &mut App) -> gpui::Entity<Self> {
        cx.bind_keys([KeyBinding::new(
            "enter",
            editor::actions::Newline,
            Some("Editor"),
        )]);
        cx.new(|cx| Self {
            editor: cx.new(|cx| {
                let mut editor = Editor::auto_height(3, window, cx);
                editor.set_soft_wrap_mode(language::language_settings::SoftWrap::EditorWidth, cx);
                editor
            }),
        })
    }
}

impl Render for AutoHeightEditorStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(white())
            .text_sm()
            .child(div().w_32().bg(gpui::black()).child(self.editor.clone()))
    }
}
