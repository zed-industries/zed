use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

/// `rgb({r},{g},{b})`
#[derive(Copy, Clone, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Default for Color {
    fn default() -> Self {
        black()
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "rgb({},{},{})", self.r, self.g, self.b)
    }
}

pub fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color { r, g, b }
}
pub fn black() -> Color {
    rgb(0, 0, 0)
}
pub fn white() -> Color {
    rgb(255, 255, 255)
}
pub fn red() -> Color {
    rgb(255, 0, 0)
}
pub fn green() -> Color {
    rgb(0, 255, 0)
}
pub fn blue() -> Color {
    rgb(0, 0, 255)
}

/// `fill:{self}`
#[derive(Copy, Clone, PartialEq)]
pub enum Fill {
    Color(Color),
    None,
}

impl Default for Fill {
    fn default() -> Self {
        Fill::None
    }
}

/// `stroke:{self}`
#[derive(Copy, Clone, PartialEq)]
pub enum Stroke {
    Color(Color, f32),
    None,
}

impl Default for Stroke {
    fn default() -> Self {
        Stroke::Color(black(), 1.0)
    }
}

/// `fill:{fill};stroke:{stroke};fill-opacity:{opacity};`
#[derive(Copy, Clone, PartialEq)]
pub struct Style {
    pub fill: Fill,
    pub stroke: Stroke,
    pub opacity: f32,
    pub stroke_opacity: f32,
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{};{};fill-opacity:{};stroke-opacity:{};",
            self.fill, self.stroke, self.opacity, self.stroke_opacity,
        )
    }
}

impl Default for Style {
    fn default() -> Self {
        Style {
            fill: Fill::Color(black()),
            stroke: Stroke::None,
            opacity: 1.0,
            stroke_opacity: 1.0,
        }
    }
}

impl From<Fill> for Style {
    fn from(fill: Fill) -> Style {
        Style {
            fill,
            stroke: Stroke::None,
            .. Default::default()
        }
    }
}

impl From<Stroke> for Style {
    fn from(stroke: Stroke) -> Style {
        Style {
            fill: Fill::None,
            stroke,
            .. Default::default()
        }
    }
}

impl fmt::Display for Fill {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Fill::Color(color) => write!(f, "fill:{}", color),
            Fill::None => write!(f, "fill:none"),
        }
    }
}

impl fmt::Display for Stroke {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stroke::Color(color, radius) => write!(f, "stroke:{};stroke-width:{}", color, radius),
            Stroke::None => write!(f, "stroke:none"),
        }
    }
}

impl Into<Fill> for Color {
    fn into(self) -> Fill {
        Fill::Color(self)
    }
}

impl Into<Stroke> for Color {
    fn into(self) -> Stroke {
        Stroke::Color(self, 1.0)
    }
}

/// `<rect x="{x}" y="{y}" width="{w}" height="{h}" ... />`,
#[derive(Clone, PartialEq)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub style: Style,
    pub border_radius: f32,
    pub comment: Option<Comment>,
}

pub fn rectangle(x: f32, y: f32, w: f32, h: f32) -> Rectangle {
    Rectangle {
        x,
        y,
        w,
        h,
        style: Style::default(),
        border_radius: 0.0,
        comment: None,
    }
}

impl Rectangle {
    pub fn fill<F>(mut self, fill: F) -> Self
    where
        F: Into<Fill>,
    {
        self.style.fill = fill.into();
        self
    }

    pub fn stroke<S>(mut self, stroke: S) -> Self
    where
        S: Into<Stroke>,
    {
        self.style.stroke = stroke.into();
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.style.opacity = opacity;
        self
    }

    pub fn stroke_opacity(mut self, opacity: f32) -> Self {
        self.style.stroke_opacity = opacity;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn border_radius(mut self, r: f32) -> Self {
        self.border_radius = r;
        self
    }

    pub fn offset(mut self, dx: f32, dy: f32) -> Self {
        self.x += dx;
        self.y += dy;
        self
    }

    pub fn inflate(mut self, dx: f32, dy: f32) -> Self {
        self.x -= dx;
        self.y -= dy;
        self.w += 2.0 * dx;
        self.h += 2.0 * dy;
        self
    }

    pub fn comment(mut self, text: &str) -> Self {
        self.comment = Some(comment(text));
        self
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            r#"<rect x="{}" y="{}" width="{}" height="{}" ry="{}" style="{}""#,
            self.x, self.y, self.w, self.h, self.border_radius, self.style,
        )?;
        if let Some(comment) = &self.comment {
            write!(f, r#">{}</rect>"#, comment)?;
        } else {
            write!(f, r#" />"#)?;
        }
        Ok(())
    }
}

/// `<circle cx="{x}" cy="{y}" r="{radius}" .../>`
#[derive(Clone, PartialEq)]
pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub style: Style,
    pub comment: Option<Comment>,
}

impl Circle {
    pub fn fill<F>(mut self, fill: F) -> Self
    where
        F: Into<Fill>,
    {
        self.style.fill = fill.into();
        self
    }

    pub fn stroke<S>(mut self, stroke: S) -> Self
    where
        S: Into<Stroke>,
    {
        self.style.stroke = stroke.into();
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.style.opacity = opacity;
        self
    }

    pub fn stroke_opacity(mut self, opacity: f32) -> Self {
        self.style.stroke_opacity = opacity;
        self
    }

    pub fn offset(mut self, dx: f32, dy: f32) -> Self {
        self.x += dx;
        self.y += dy;
        self
    }

    pub fn inflate(mut self, by: f32) -> Self {
        self.radius += by;
        self
    }

    pub fn comment(mut self, text: &str) -> Self {
        self.comment = Some(comment(text));
        self
    }
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            r#"<circle cx="{}" cy="{}" r="{}" style="{}""#,
            self.x, self.y, self.radius, self.style,
        )?;
        if let Some(comment) = &self.comment {
            write!(f, r#">{}</circle>"#, comment)?;
        } else {
            write!(f, r#" />"#)?;
        }
        Ok(())
    }
}

/// `<path d="..." style="..."/>`
#[derive(Clone, PartialEq)]
pub struct Polygon {
    pub points: Vec<[f32; 2]>,
    pub closed: bool,
    pub style: Style,
    pub comment: Option<Comment>,
}

impl fmt::Display for Polygon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#"<path d="#)?;
        if self.points.len() > 0 {
            write!(f, "M {} {} ", self.points[0][0], self.points[0][1])?;
            for &p in &self.points[1..] {
                write!(f, "L {} {} ", p[0], p[1])?;
            }
            if self.closed {
                write!(f, "Z")?;
            }
        }
        write!(f, r#"" style="{}"#, self.style)?;
        if let Some(comment) = &self.comment {
            write!(f, r#">{}</path>"#, comment)?;
        } else {
            write!(f, r#" />"#)?;
        }
        Ok(())
    }
}

pub fn polygon<T: Copy + Into<[f32; 2]>>(pts: &[T]) -> Polygon {
    let mut points = Vec::with_capacity(pts.len());
    for p in pts {
        points.push((*p).into());
    }
    Polygon {
        points,
        closed: true,
        style: Style::default(),
        comment: None,
    }
}

pub fn triangle(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) -> Polygon {
    polygon(&[[x1, y1], [x2, y2], [x3, y3]])
}

impl Polygon {
    pub fn open(mut self) -> Self {
        self.closed = false;
        self
    }

    pub fn fill<F>(mut self, fill: F) -> Self
    where
        F: Into<Fill>,
    {
        self.style.fill = fill.into();
        self
    }

    pub fn stroke<S>(mut self, stroke: S) -> Self
    where
        S: Into<Stroke>,
    {
        self.style.stroke = stroke.into();
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.style.opacity = opacity;
        self
    }

    pub fn stroke_opacity(mut self, opacity: f32) -> Self {
        self.style.stroke_opacity = opacity;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn comment(mut self, text: &str) -> Self {
        self.comment = Some(comment(text));
        self
    }
}

/// `<path d="M {x1} {y1} L {x2} {y2}" ... />`
#[derive(Clone, PartialEq)]
pub struct LineSegment {
    pub x1: f32,
    pub x2: f32,
    pub y1: f32,
    pub y2: f32,
    pub color: Color,
    pub width: f32,
    pub comment: Option<Comment>,
}

impl fmt::Display for LineSegment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            r#"<path d="M {} {} L {} {}" style="stroke:{};stroke-width:{}""#,
            self.x1, self.y1, self.x2, self.y2, self.color, self.width
        )?;
        if let Some(comment) = &self.comment {
            write!(f, r#">{}</path>"#, comment)?;
        } else {
            write!(f, r#" />"#)?;
        }
        Ok(())
    }
}

pub fn line_segment(x1: f32, y1: f32, x2: f32, y2: f32) -> LineSegment {
    LineSegment {
        x1,
        y1,
        x2,
        y2,
        color: black(),
        width: 1.0,
        comment: None,
    }
}

impl LineSegment {
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn offset(mut self, dx: f32, dy: f32) -> Self {
        self.x1 += dx;
        self.y1 += dy;
        self.x2 += dx;
        self.y2 += dy;
        self
    }

    pub fn comment(mut self, text: &str) -> Self {
        self.comment = Some(comment(text));
        self
    }
}

/// `<path d="..." />`
#[derive(Clone, PartialEq)]
pub struct Path {
    pub ops: Vec<PathOp>,
    pub style: Style,
    pub comment: Option<Comment>,
}

/// `M {} {} L {} {} ...`
#[derive(Copy, Clone, PartialEq)]
pub enum PathOp {
    MoveTo {
        x: f32,
        y: f32,
    },
    LineTo {
        x: f32,
        y: f32,
    },
    QuadraticTo {
        ctrl_x: f32,
        ctrl_y: f32,
        x: f32,
        y: f32,
    },
    CubicTo {
        ctrl1_x: f32,
        ctrl1_y: f32,
        ctrl2_x: f32,
        ctrl2_y: f32,
        x: f32,
        y: f32,
    },
    Close,
}
impl fmt::Display for PathOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PathOp::MoveTo { x, y } => write!(f, "M {} {} ", x, y),
            PathOp::LineTo { x, y } => write!(f, "L {} {} ", x, y),
            PathOp::QuadraticTo {
                ctrl_x,
                ctrl_y,
                x,
                y,
            } => write!(f, "Q {} {} {} {} ", ctrl_x, ctrl_y, x, y),
            PathOp::CubicTo {
                ctrl1_x,
                ctrl1_y,
                ctrl2_x,
                ctrl2_y,
                x,
                y,
            } => write!(
                f,
                "C {} {} {} {} {} {} ",
                ctrl1_x, ctrl1_y, ctrl2_x, ctrl2_y, x, y
            ),
            PathOp::Close => write!(f, "Z "),
        }
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#"<path d=""#)?;
        for op in &self.ops {
            op.fmt(f)?;
        }
        write!(f, r#"" style="{}""#, self.style)?;
        if let Some(comment) = &self.comment {
            write!(f, r#">{}</path>"#, comment)?;
        } else {
            write!(f, r#"/>"#)?;
        }
        Ok(())
    }
}

impl Path {
    pub fn move_to(mut self, x: f32, y: f32) -> Self {
        self.ops.push(PathOp::MoveTo { x, y });
        self
    }

    pub fn line_to(mut self, x: f32, y: f32) -> Self {
        self.ops.push(PathOp::LineTo { x, y });
        self
    }

    pub fn quadratic_bezier_to(mut self, ctrl_x: f32, ctrl_y: f32, x: f32, y: f32) -> Self {
        self.ops.push(PathOp::QuadraticTo {
            ctrl_x,
            ctrl_y,
            x,
            y,
        });
        self
    }

    pub fn cubic_bezier_to(
        mut self,
        ctrl1_x: f32,
        ctrl1_y: f32,
        ctrl2_x: f32,
        ctrl2_y: f32,
        x: f32,
        y: f32,
    ) -> Self {
        self.ops.push(PathOp::CubicTo {
            ctrl1_x,
            ctrl1_y,
            ctrl2_x,
            ctrl2_y,
            x,
            y,
        });
        self
    }

    pub fn close(mut self) -> Self {
        self.ops.push(PathOp::Close);
        self
    }

    pub fn fill<F>(mut self, fill: F) -> Self
    where
        F: Into<Fill>,
    {
        self.style.fill = fill.into();
        self
    }

    pub fn stroke<S>(mut self, stroke: S) -> Self
    where
        S: Into<Stroke>,
    {
        self.style.stroke = stroke.into();
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.style.opacity = opacity;
        self
    }

    pub fn stroke_opacity(mut self, opacity: f32) -> Self {
        self.style.stroke_opacity = opacity;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

pub fn path() -> Path {
    Path {
        ops: Vec::new(),
        style: Style::default(),
        comment: None,
    }
}

/// `<text x="{x}" y="{y}" ... > {text} </text>`
#[derive(Clone, PartialEq)]
pub struct Text {
    pub x: f32,
    pub y: f32,
    pub text: String,
    pub color: Color,
    pub align: Align,
    pub size: f32,
    pub comment: Option<Comment>,
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            r#"<text x="{}" y="{}" style="font-size:{}px;fill:{};{}">"#,
            self.x, self.y, self.size, self.color, self.align,
        )?;
        if let Some(comment) = &self.comment {
            write!(f, r#" {}"#, comment)?;
        }
        write!(f, r#" {} </text>"#, self.text)
    }
}

pub fn text<T: Into<String>>(x: f32, y: f32, txt: T) -> Text {
    Text {
        x,
        y,
        text: txt.into(),
        color: black(),
        align: Align::Left,
        size: 10.0,
        comment: None,
    }
}

impl Text {
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    pub fn offset(mut self, dx: f32, dy: f32) -> Self {
        self.x += dx;
        self.y += dy;
        self
    }

    pub fn comment(mut self, text: &str) -> Self {
        self.comment = Some(comment(text));
        self
    }
}

#[derive(Clone, PartialEq)]
pub struct Comment {
    pub text: String,
}

pub fn comment<T: Into<String>>(text: T) -> Comment {
    Comment { text: text.into() }
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<!-- {} -->", self.text)
    }
}

/// `text-align:{self}`
#[derive(Copy, Clone, PartialEq)]
pub enum Align {
    Left,
    Right,
    Center,
}

impl fmt::Display for Align {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Align::Left => write!(f, "text-anchor:start;text-align:left;"),
            Align::Right => write!(f, "text-anchor:end;text-align:right;"),
            Align::Center => write!(f, "text-anchor:middle;text-align:center;"),
        }
    }
}

/// `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {y}">`
#[derive(Copy, Clone, PartialEq)]
pub struct BeginSvg {
    pub w: f32,
    pub h: f32,
}

impl fmt::Display for BeginSvg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}">"#,
            self.w, self.h,
        )
    }
}

/// `</svg>`
#[derive(Copy, Clone, PartialEq)]
pub struct EndSvg;

impl fmt::Display for EndSvg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "</svg>")
    }
}

/// `"    "`
pub struct Indentation {
    pub n: u32,
}

pub fn indent(n: u32) -> Indentation {
    Indentation { n }
}

impl Indentation {
    pub fn push(&mut self) {
        self.n += 1;
    }

    pub fn pop(&mut self) {
        self.n -= 1;
    }
}

impl fmt::Display for Indentation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for _ in 0..self.n {
            write!(f, "    ")?;
        }
        Ok(())
    }
}
