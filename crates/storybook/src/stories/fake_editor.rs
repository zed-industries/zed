use editor::*;
use gpui::*;
use settings::Settings;
use theme::ThemeSettings;
use ui::*;

pub struct FakeEditor {
    editor: View<Editor>,
    size: Option<Size<Length>>,
}

impl FakeEditor {
    pub fn new(cx: &mut WindowContext) -> Self {
        let editor = cx.new_view(|cx| Editor::multi_line(cx));

        Self { editor, size: None }
    }
}

impl Render for FakeEditor {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .size_full()
            .debug_bg_cyan()
            .child(self.editor.clone())
    }
}

pub struct FakeEditorStory {
    fake_editor_1: View<FakeEditor>,
}

impl FakeEditorStory {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let fake_editor_1 = cx.new_view(|cx| FakeEditor::new(cx));
            Self { fake_editor_1 }
        })
    }
}

impl Render for FakeEditorStory {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .bg(white())
            .text_sm()
            .flex_1()
            .child(self.fake_editor_1.clone())
    }
}
