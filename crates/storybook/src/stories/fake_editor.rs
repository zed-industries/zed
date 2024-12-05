use editor::*;
use gpui::*;
use language::Buffer;
use settings::Settings;
use theme::ThemeSettings;
use ui::*;

pub struct FakeEditor {
    editor: View<Editor>,
    elements_above: Vec<(u32, Box<dyn Fn(Pixels, &mut WindowContext) -> AnyElement>)>,
}

// div().top(y_for_row(3)).into_element()

impl FakeEditor {
    pub fn new(initial_text: &str, cx: &mut WindowContext) -> Self {
        let editor = cx.new_view(|cx| {
            let buffer = cx.new_model(|cx| Buffer::local(initial_text, cx));
            let buffer = cx.new_model(|cx| MultiBuffer::singleton(buffer, cx));
            Editor::new(EditorMode::Full, buffer, None, false, cx)
        });
        Self {
            editor,
            elements_above: Vec::new(),
        }
    }

    fn element_above(
        &mut self,
        row: u32,
        element_fn: impl Fn(Pixels, &mut WindowContext) -> AnyElement + 'static,
    ) {
        self.elements_above.push((row, Box::new(element_fn)));
    }

    fn line_height(&self, cx: &mut WindowContext) -> Option<Pixels> {
        Some(
            self.editor
                .read(cx)
                .style()?
                .text
                .line_height_in_pixels(cx.rem_size()),
        )
    }
}

impl Render for FakeEditor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let line_height = self.line_height(cx).unwrap_or(px(12.));

        div()
            .relative()
            .flex_1()
            .size_full()
            .child(self.editor.clone())
            .child(div().absolute().top_0().left_0().size_full().children(
                self.elements_above.iter().map(|(row, element_fn)| {
                    div()
                        .absolute()
                        .top(px(*row as f32) * line_height)
                        .child(element_fn(line_height, cx))
                }),
            ))
    }
}

pub struct FakeEditorStory {
    fake_editor_1: View<FakeEditor>,
}

impl FakeEditorStory {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let fake_editor_1 = cx.new_view(|cx| {
                let editor_text = r###"fn main() {

.iter().sum();i32=numbers:sumlet
4,5];,3,![1,2=vecnumberslet   println!("Sum of numbers: {}", sum);

    let mut counter = 0;
    while counter < 5 {
        println!("Counter: {}", counter);
        counter += 1;
    }

    for i in 0..3 {
        println!("Iteration: {}", i);
    }
}"###;

                let mut fake_editor = FakeEditor::new(editor_text, cx);
                fake_editor.element_above(3, |line_height, _| {
                    div().bg(hsla(0.5, 1.,1.,0.2)).w_16().h(line_height).into_any_element()
                });
                fake_editor
            });

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
