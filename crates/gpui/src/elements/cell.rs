use crate::{color::Color, geometry::vector::Vector2F};

struct Cell {}

impl Cell {
    fn new(style: CellStyle) -> Self {
        Self { style }
    }

    fn interactive(style: Interactive<CellStyle>) -> Self {}
}

impl CellStyle {
    fn interactive(self) -> Interactive<CellStyle> {
        Interactive {
            default: self.clone(),
            hovered: self.clone(),
            active: self.clone(),
            disabled: self,
        }
    }

    fn hover(self, f: impl FnOnce(&mut CellStyle)) -> Interactive<CellStyle> {
        let mut style = self.interactive();
        f(&mut style.hovered);
        style
    }
}

fn foo() {

    struct WidgetStyle {
        foo: CellStyle,
        bar: CellStyle,
        button: Interactive<CellStyle>,
    }

    let mut header_style = CellStyle::default();
    header_style.fill = Fill::Color(Color::red());

    let style = CellStyle::default().hover(|style| {

    })

    let interactive = style.hover(|style| {
        style.fill = Fill::Color(Color::red());
    });


    style.hover(|style| {
        style
            .fill(Color(red))
            .text_color(Color(red));
    })
}
struct Interactive<Style> {
    default: Style,
    hovered: Style,
    active: Style,
    disabled: Style,
}

#[derive(Clone, Default)]
struct CellStyle {
    axis: Axis,
    wrap: bool,
    align: Vector2F,
    overflow_x: Overflow,
    overflow_y: Overflow,
    gap_x: Gap,
    gap_y: Gap,

    width: Length,
    height: Length,

    text_color: Option<Color>,
    font_size: Option<f32>,
    font_style: Option<FontStyle>,
    font_weight: Option<FontWeight>,

    opacity: f32,
    fill: Fill,
    border: Border,
    corner_radii: CornerRadii,
    shadows: Vec<Shadow>,
}

struct CornerRadii {
    top_left: f32,
    top_right: f32,
    bottom_right: f32,
    bottom_left: f32,
}

enum Fill {
    Color(Color),
    Svg(String),
}

struct Border {
    color: Color,
    width: f32,
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}

enum Length {
    Fixed(f32),
    Auto(f32),
}

enum Axis {
    X,
    Y,
    Z,
}

enum Overflow {
    Hidden,
    Auto,
}

enum Gap {
    Fixed(f32),
    Around,
    Between,
    Even,
}

struct Shadow {
    offset: Vector2F,
    blur: f32,
    color: Color,
}

enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    Normal,
    Medium,
    Semibold,
    Bold,
    ExtraBold,
    Black,
}
