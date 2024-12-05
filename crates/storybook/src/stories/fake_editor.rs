use editor::*;
use gpui::*;
use settings::Settings as _;
use theme::ThemeSettings;
use ui::*;

// const DEFAULT_LINE_HEIGHT: Pixels = Pixels(20.);

pub struct EditorPrototype {
    editor: View<Editor>,
    text_style: TextStyle,
    elements_above: Vec<(u32, Box<dyn Fn(Pixels, &mut WindowContext) -> AnyElement>)>,
    height: Length,
}

impl EditorPrototype {
    pub fn build(
        cx: &mut WindowContext,
        f: impl FnOnce(Self, &mut ViewContext<Self>) -> Self,
    ) -> View<Self> {
        let theme = ThemeSettings::get_global(cx);
        let line_height = rems(theme.buffer_line_height.value()).to_pixels(cx.rem_size());

        let mut text_style = cx.text_style();
        let refinement = TextStyleRefinement {
            font_family: Some(theme.buffer_font.family.clone()),
            line_height: Some(line_height.into()),
            background_color: Some(gpui::transparent_black()),
            ..Default::default()
        };
        text_style.refine(&refinement);

        let editor = cx.new_view(|cx| {
            let mut editor = Editor::auto_height(20, cx);
            editor.set_text_style_refinement(refinement);
            editor.set_show_gutter(true, cx);
            editor.set_show_line_numbers(true, cx);
            editor
        });

        let mut height = Length::Auto;

        editor.update(cx, |editor, cx| {
            let line_count = editor.max_point(cx).row() + 1;
            println!("line_count: {}", line_count.as_f32());
            let line_height = line_height.0;
            println!("line_height: {}", line_height);
            height = px((line_count.as_f32() * line_height).round()).into();
            println!("height: {:?}", height);
        });

        println!("final height: {:?}", height);

        cx.new_view(|cx| {
            cx.refresh();
            f(
                Self {
                    editor,
                    text_style,
                    elements_above: Vec::new(),
                    height,
                },
                cx,
            )
        })
    }

    pub fn line_height(&self, cx: &ViewContext<Self>) -> Pixels {
        self.text_style
            .line_height
            .to_pixels(self.text_style.font_size, cx.rem_size())
    }

    pub fn text(mut self, initial_text: &str, cx: &mut ViewContext<Self>) -> Self {
        let mut height = self.height;
        let line_height = self.line_height(cx);

        self.editor.update(cx, |editor, cx| {
            editor.set_text(initial_text, cx);
            let line_count = editor.max_point(cx).row() + 1;
            println!("line_count: {}", line_count.as_f32());
            let line_height = line_height.0;
            println!("line_height: {}", line_height);
            height = px((line_count.as_f32() * line_height).round()).into();
            println!("height: {:?}", height);
        });

        self.height = height;

        println!("final height: {:?}", height);

        cx.notify();

        self
    }

    fn element_below(
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
        let line_height = self.line_height(cx);

        div()
            .relative()
            .flex_shrink_0()
            .w_full()
            .h(self.height)
            .bg(cx.theme().colors().editor_background)
            .child(div().absolute().top_0().left_0().size_full().children(
                self.elements_above.iter().map(|(row, element_fn)| {
                    div()
                        .absolute()
                        .top(px(*row as f32) * line_height)
                        .left_0()
                        .w_full()
                        .child(element_fn(line_height, cx))
                }),
            ))
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full()
                    .child(self.editor.clone()),
            )
    }
}

pub struct FakeEditorStory {
    fake_editor_1: View<EditorPrototype>,
}

impl FakeEditorStory {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        let editor_text = r###"fn main() {
    println!("Hello, World!");

    let name = "Rust";
    println!("Welcome to {}", name);

    let x = 5;
    let y = 7;
    println!("{} + {} = {}", x, y, x + y);

    if x < y {
        println!("{} is less than {}", x, y);
    }

    greet("Rustacean");
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}"###;

        cx.new_view(|cx| {
            let fake_editor_1 = EditorPrototype::build(cx, |fake_editor, cx| {
                fake_editor
                    .text(editor_text, cx)
                    .element_below(3, |line_height, _| {
                        let green_bg = hsla(142. / 360., 0.68, 0.45, 0.06);
                        div()
                            .id("foo")
                            .bg(green_bg)
                            .w_full()
                            .h(line_height)
                            .into_any_element()
                    })
            });
            Self { fake_editor_1 }
        })
    }
}

impl Render for FakeEditorStory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex().size_full().bg(white()).text_sm().flex_1().child(
            div().size_full().p_4().child(
                v_flex()
                    .size_full()
                    .rounded_md()
                    .border_1()
                    .border_color(cx.theme().colors().border)
                    .bg(cx.theme().colors().background)
                    .child(self.fake_editor_1.clone()),
            ),
        )
    }
}
