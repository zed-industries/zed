use editor::Editor;
use gpui::{
    div, IntoElement, KeyBinding, ParentElement, Render, Styled, View, ViewContext, VisualContext,
    WindowContext,
};
use story::Story;

pub struct AutoHeightEditorStory {
    editor: View<Editor>,
}

impl AutoHeightEditorStory {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        cx.bind_keys([KeyBinding::new(
            "enter",
            editor::actions::Newline,
            Some("Editor"),
        )]);
        cx.new_view(|cx| Self {
            editor: cx.new_view(|cx| {
                let mut editor = Editor::auto_height(3, cx);
                editor.set_soft_wrap_mode(language::language_settings::SoftWrap::EditorWidth, cx);
                editor.set_placeholder_text("Type some text & hit enter", cx);
                editor
            }),
        })
    }
}

impl Render for AutoHeightEditorStory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let color = cx.default_style().color;

        Story::container(cx)
            .child(Story::title(cx, "Auto Height Editor"))
            .child(div().w_64().bg(color.container).child(self.editor.clone()))
    }
}
