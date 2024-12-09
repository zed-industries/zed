use editor::Editor;
use gpui::{
    div, white, IntoElement, KeyBinding, ParentElement, Render, Styled, Model,
    VisualContext,
};

pub struct AutoHeightEditorStory {
    editor: Model<Editor>,
}

impl AutoHeightEditorStory {
    pub fn new(window: &mut gpui::Window, cx: &mut gpui::AppContext) -> Model<Self> {
        cx.bind_keys([KeyBinding::new(
            "enter",
            editor::actions::Newline,
            Some("Editor"),
        )]);
        cx.new_model(|model, cx| Self {
            editor: cx.new_model(|model, cx| {
                let mut editor = Editor::auto_height(3, cx);
                editor.set_soft_wrap_mode(language::language_settings::SoftWrap::EditorWidth, cx);
                editor
            }),
        })
    }
}

impl Render for AutoHeightEditorStory {
    fn render(&mut self, model: &Model<>Self, _cx: &mut AppContext) -> impl IntoElement {
        div()
            .size_full()
            .bg(white())
            .text_sm()
            .child(div().w_32().bg(gpui::black()).child(self.editor.clone()))
    }
}
