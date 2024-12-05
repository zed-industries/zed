use editor::*;
use gpui::*;
use ui::*;

const DEFAULT_LINE_HEIGHT: Pixels = Pixels(75.);

pub struct EditorPrototype {
    editor: View<Editor>,
    text_style: TextStyle,
    elements_above: Vec<(u32, Box<dyn Fn(Pixels, &mut WindowContext) -> AnyElement>)>,
}

impl EditorPrototype {
    pub fn build(
        cx: &mut WindowContext,
        f: impl FnOnce(Self, &mut ViewContext<Self>) -> Self,
    ) -> View<Self> {
        let mut text_style = cx.text_style();
        let refinement = TextStyleRefinement {
            line_height: Some(DEFAULT_LINE_HEIGHT.into()),
            background_color: Some(gpui::transparent_black()),
            ..Default::default()
        };
        text_style.refine(&refinement);

        cx.new_view(|cx| {
            cx.refresh();
            f(
                Self {
                    editor: cx.new_view(|cx| {
                        let mut editor = Editor::multi_line(cx);
                        editor.set_text_style_refinement(refinement);
                        editor
                    }),
                    text_style,
                    elements_above: Vec::new(),
                },
                cx,
            )
        })
    }

    pub fn line_height(mut self, line_height: Pixels, cx: &mut ViewContext<Self>) -> Self {
        let refinement = TextStyleRefinement {
            line_height: Some(line_height.into()),
            ..Default::default()
        };
        self.text_style.refine(&refinement);
        self.editor
            .update(cx, |editor, _| editor.set_text_style_refinement(refinement));
        self
    }

    pub fn text(self, initial_text: &str, cx: &mut ViewContext<Self>) -> Self {
        self.editor
            .update(cx, |editor, cx| editor.set_text(initial_text, cx));
        self
    }

    fn element_above(
        mut self,
        row: u32,
        element_fn: impl Fn(Pixels, &mut WindowContext) -> AnyElement + 'static,
    ) -> Self {
        self.elements_above.push((row, Box::new(element_fn)));
        self
    }
}

impl Render for EditorPrototype {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let line_height = self
            .text_style
            .line_height
            .to_pixels(self.text_style.font_size, cx.rem_size());

        div()
            .relative()
            .flex_shrink_0()
            .size_full()
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full()
                    .child(self.editor.clone()),
            )
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
    fake_editor_1: View<EditorPrototype>,
}

impl FakeEditorStory {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
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

        cx.new_view(|cx| {
            let fake_editor_1 = EditorPrototype::build(cx, |fake_editor, cx| {
                fake_editor
                    .text(editor_text, cx)
                    .element_above(0, |line_height, _| {
                        div()
                            .id("foo")
                            .bg(gpui::yellow().opacity(0.7))
                            .w_32()
                            .h(line_height)
                            .into_any_element()
                    })
                    .element_above(1, |line_height, _| {
                        div()
                            .id("foo")
                            .bg(gpui::green().opacity(0.7))
                            .w_32()
                            .h(line_height)
                            .into_any_element()
                    })
                    .element_above(2, |line_height, _| {
                        div()
                            .id("foo")
                            .bg(gpui::red().opacity(0.7))
                            .w_32()
                            .h(line_height)
                            .into_any_element()
                    })
                    .element_above(3, |line_height, _| {
                        div()
                            .id("foo")
                            .bg(gpui::blue().opacity(0.7))
                            .w_32()
                            .h(line_height)
                            .into_any_element()
                    })
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
