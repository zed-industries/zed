use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use gpui::{
    AbsoluteLength, App, Application, Context, DefiniteLength, ElementId, Global, Hsla, Menu,
    SharedString, TextStyle, TitlebarOptions, Window, WindowBounds, WindowOptions, bounds, div,
    point, prelude::*, px, relative, rgb, size,
};
use std::iter;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct TextContext {
    font_size: f32,
    line_height: f32,
    type_scale: f32,
}

impl Default for TextContext {
    fn default() -> Self {
        TextContext {
            font_size: 16.0,
            line_height: 1.3,
            type_scale: 1.33,
        }
    }
}

impl TextContext {
    pub fn get_global(cx: &App) -> &Arc<TextContext> {
        &cx.global::<GlobalTextContext>().0
    }
}

#[derive(Clone, Debug)]
pub struct GlobalTextContext(pub Arc<TextContext>);

impl Deref for GlobalTextContext {
    type Target = Arc<TextContext>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GlobalTextContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Global for GlobalTextContext {}

pub trait ActiveTextContext {
    fn text_context(&self) -> &Arc<TextContext>;
}

impl ActiveTextContext for App {
    fn text_context(&self) -> &Arc<TextContext> {
        &self.global::<GlobalTextContext>().0
    }
}

#[derive(Clone, PartialEq)]
pub struct SpecimenTheme {
    pub bg: Hsla,
    pub fg: Hsla,
}

impl Default for SpecimenTheme {
    fn default() -> Self {
        Self {
            bg: gpui::white(),
            fg: gpui::black(),
        }
    }
}

impl SpecimenTheme {
    pub fn invert(&self) -> Self {
        Self {
            bg: self.fg,
            fg: self.bg,
        }
    }
}

#[derive(Debug, Clone, PartialEq, IntoElement)]
struct Specimen {
    id: Uuid,
    scale: f32,
    text_style: Option<TextStyle>,
    string: SharedString,
    invert: bool,
}

impl Specimen {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            scale: 1.0,
            text_style: None,
            string: SharedString::new_static("The quick brown fox jumps over the lazy dog"),
            invert: false,
        }
    }

    pub fn invert(mut self) -> Self {
        self.invert = !self.invert;
        self
    }

    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }
}

impl RenderOnce for Specimen {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let rem_size = window.rem_size();
        let scale = self.scale;
        let global_style = cx.text_context();

        let style_override = self.text_style;

        let mut font_size = global_style.font_size;
        let mut line_height = global_style.line_height;

        if let Some(style_override) = style_override {
            font_size = style_override.font_size.to_pixels(rem_size).0;
            line_height = match style_override.line_height {
                DefiniteLength::Absolute(absolute_len) => match absolute_len {
                    AbsoluteLength::Rems(absolute_len) => absolute_len.to_pixels(rem_size).0,
                    AbsoluteLength::Pixels(absolute_len) => absolute_len.0,
                },
                DefiniteLength::Fraction(value) => value,
            };
        }

        let mut theme = SpecimenTheme::default();

        if self.invert {
            theme = theme.invert();
        }

        div()
            .id(ElementId::Uuid(self.id))
            .bg(theme.bg)
            .text_color(theme.fg)
            .text_size(px(font_size * scale))
            .line_height(relative(line_height))
            .p(px(10.0))
            .child(self.string.clone())
    }
}

#[derive(Debug, Clone, PartialEq, IntoElement)]
struct CharacterGrid {
    id: Uuid,
    scale: f32,
    invert: bool,
    text_style: Option<TextStyle>,
}

impl CharacterGrid {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            scale: 1.0,
            invert: false,
            text_style: None,
        }
    }

    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }
}

impl RenderOnce for CharacterGrid {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut theme = SpecimenTheme::default();

        if self.invert {
            theme = theme.invert();
        }

        let characters = vec![
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "A", "B", "C", "D", "E", "F", "G",
            "H", "I", "J", "K", "L", "M", "N", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y",
            "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "p", "q",
            "r", "s", "t", "u", "v", "w", "x", "y", "z", "ẞ", "ſ", "ß", "ð", "Þ", "þ", "α", "β",
            "Γ", "γ", "Δ", "δ", "η", "θ", "ι", "κ", "Λ", "λ", "μ", "ν", "ξ", "π", "τ", "υ", "φ",
            "χ", "ψ", "∂", "а", "в", "Ж", "ж", "З", "з", "К", "к", "л", "м", "Н", "н", "Р", "р",
            "У", "у", "ф", "ч", "ь", "ы", "Э", "э", "Я", "я", "ij", "öẋ", ".,", "⣝⣑", "~", "*",
            "_", "^", "`", "'", "(", "{", "«", "#", "&", "@", "$", "¢", "%", "|", "?", "¶", "µ",
            "❮", "<=", "!=", "==", "--", "++", "=>", "->",
        ];

        let columns = 11;
        let rows = characters.len().div_ceil(columns);

        let grid_rows = (0..rows).map(|row_idx| {
            let start_idx = row_idx * columns;
            let end_idx = (start_idx + columns).min(characters.len());

            div()
                .w_full()
                .flex()
                .flex_row()
                .children((start_idx..end_idx).map(|i| {
                    div()
                        .id(ElementId::Uuid(Uuid::new_v4()))
                        .text_center()
                        .size(px(62.))
                        .bg(theme.bg)
                        .text_color(theme.fg)
                        .text_size(px(24.0))
                        .line_height(relative(1.0))
                        .child(characters[i])
                }))
                .when(end_idx - start_idx < columns, |d| {
                    d.children(
                        iter::repeat_with(|| div().flex_1().id(ElementId::Uuid(Uuid::new_v4())))
                            .take(columns - (end_idx - start_idx)),
                    )
                })
        });

        div()
            .id(ElementId::Uuid(self.id))
            .p_4()
            .gap_2()
            .flex()
            .flex_col()
            .children(grid_rows)
    }
}

struct TextExample {}

impl Render for TextExample {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let tcx = cx.text_context();
        let type_scale = tcx.type_scale;

        let step_down_2 = 1.0 / (type_scale * type_scale);
        let step_down_1 = 1.0 / type_scale;
        let base = 1.0;
        let step_up_1 = base * type_scale;
        let step_up_2 = step_up_1 * type_scale;
        let step_up_3 = step_up_2 * type_scale;
        let step_up_4 = step_up_3 * type_scale;
        let step_up_5 = step_up_4 * type_scale;
        let step_up_6 = step_up_5 * type_scale;

        div()
            .id("text-example")
            .overflow_y_scroll()
            .overflow_x_hidden()
            .bg(rgb(0xffffff))
            .w_full()
            .child(div().child(CharacterGrid::new().scale(base)))
            .child(
                div()
                    .child(Specimen::new().scale(step_down_2))
                    .child(Specimen::new().scale(step_down_2).invert())
                    .child(Specimen::new().scale(step_down_1))
                    .child(Specimen::new().scale(step_down_1).invert())
                    .child(Specimen::new().scale(base))
                    .child(Specimen::new().scale(base).invert())
                    .child(Specimen::new().scale(step_up_1))
                    .child(Specimen::new().scale(step_up_1).invert())
                    .child(Specimen::new().scale(step_up_2))
                    .child(Specimen::new().scale(step_up_2).invert())
                    .child(Specimen::new().scale(step_up_3))
                    .child(Specimen::new().scale(step_up_3).invert())
                    .child(Specimen::new().scale(step_up_4))
                    .child(Specimen::new().scale(step_up_4).invert())
                    .child(Specimen::new().scale(step_up_5))
                    .child(Specimen::new().scale(step_up_5).invert())
                    .child(Specimen::new().scale(step_up_6))
                    .child(Specimen::new().scale(step_up_6).invert()),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.set_menus(vec![Menu {
            name: "GPUI Typography".into(),
            items: vec![],
        }]);

        cx.set_global(GlobalTextContext(Arc::new(TextContext::default())));

        let window = cx
            .open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("GPUI Typography".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(bounds(
                        point(px(0.0), px(0.0)),
                        size(px(920.), px(720.)),
                    ))),
                    ..Default::default()
                },
                |_window, cx| cx.new(|_cx| TextExample {}),
            )
            .unwrap();

        window
            .update(cx, |_view, _window, cx| {
                cx.activate(true);
            })
            .unwrap();
    });
}
