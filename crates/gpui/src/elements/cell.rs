use crate::{color::Color, geometry::vector::Vector2F};

struct Cell {}

impl Cell {
    fn new(style: CellStyle) -> Self {
        Self {}
    }
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

#[derive(Clone, Default)]
struct CornerRadii {
    top_left: f32,
    top_right: f32,
    bottom_right: f32,
    bottom_left: f32,
}

#[derive(Clone)]
enum Fill {
    Color(Color),
}

impl Default for Fill {
    fn default() -> Self {
        Fill::Color(Color::default())
    }
}

#[derive(Clone, Default)]
struct Border {
    color: Color,
    width: f32,
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}

#[derive(Clone, Copy)]
enum Length {
    Fixed(f32),
    Auto(f32),
}

impl Default for Length {
    fn default() -> Self {
        Length::Auto(1.)
    }
}

#[derive(Clone, Copy, Default)]
enum Axis {
    X,
    #[default]
    Y,
    Z,
}

#[derive(Clone, Copy, Default)]
enum Overflow {
    #[default]
    Visible,
    Hidden,
    Auto,
}

#[derive(Clone, Copy)]
enum Gap {
    Fixed(f32),
    Around,
    Between,
    Even,
}

impl Default for Gap {
    fn default() -> Self {
        Gap::Fixed(0.)
    }
}

#[derive(Clone, Copy, Default)]
struct Shadow {
    offset: Vector2F,
    blur: f32,
    color: Color,
}

#[derive(Clone, Copy, Default)]
enum FontStyle {
    #[default]
    Normal,
    Italic,
    Oblique,
}

#[derive(Clone, Copy, Default)]
enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    #[default]
    Normal,
    Medium,
    Semibold,
    Bold,
    ExtraBold,
    Black,
}
