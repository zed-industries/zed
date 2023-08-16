use super::scene::{Path, PathVertex};
use crate::{color::Color, json::ToJson};
pub use pathfinder_geometry::*;
use rect::RectF;
use serde::{Deserialize, Deserializer};
use serde_json::json;
use vector::{vec2f, Vector2F};

pub struct PathBuilder {
    vertices: Vec<PathVertex>,
    start: Vector2F,
    current: Vector2F,
    contour_count: usize,
    bounds: RectF,
}

enum PathVertexKind {
    Solid,
    Quadratic,
}

impl Default for PathBuilder {
    fn default() -> Self {
        PathBuilder::new()
    }
}

impl PathBuilder {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            start: vec2f(0., 0.),
            current: vec2f(0., 0.),
            contour_count: 0,
            bounds: RectF::default(),
        }
    }

    pub fn reset(&mut self, point: Vector2F) {
        self.vertices.clear();
        self.start = point;
        self.current = point;
        self.contour_count = 0;
    }

    pub fn line_to(&mut self, point: Vector2F) {
        self.contour_count += 1;
        if self.contour_count > 1 {
            self.push_triangle(self.start, self.current, point, PathVertexKind::Solid);
        }

        self.current = point;
    }

    pub fn curve_to(&mut self, point: Vector2F, ctrl: Vector2F) {
        self.contour_count += 1;
        if self.contour_count > 1 {
            self.push_triangle(self.start, self.current, point, PathVertexKind::Solid);
        }

        self.push_triangle(self.current, ctrl, point, PathVertexKind::Quadratic);
        self.current = point;
    }

    pub fn build(mut self, color: Color, clip_bounds: Option<RectF>) -> Path {
        if let Some(clip_bounds) = clip_bounds {
            self.bounds = self.bounds.intersection(clip_bounds).unwrap_or_default();
        }
        Path {
            bounds: self.bounds,
            color,
            vertices: self.vertices,
        }
    }

    fn push_triangle(&mut self, a: Vector2F, b: Vector2F, c: Vector2F, kind: PathVertexKind) {
        if self.vertices.is_empty() {
            self.bounds = RectF::new(a, Vector2F::zero());
        }
        self.bounds = self.bounds.union_point(a).union_point(b).union_point(c);

        match kind {
            PathVertexKind::Solid => {
                self.vertices.push(PathVertex {
                    xy_position: a,
                    st_position: vec2f(0., 1.),
                });
                self.vertices.push(PathVertex {
                    xy_position: b,
                    st_position: vec2f(0., 1.),
                });
                self.vertices.push(PathVertex {
                    xy_position: c,
                    st_position: vec2f(0., 1.),
                });
            }
            PathVertexKind::Quadratic => {
                self.vertices.push(PathVertex {
                    xy_position: a,
                    st_position: vec2f(0., 0.),
                });
                self.vertices.push(PathVertex {
                    xy_position: b,
                    st_position: vec2f(0.5, 0.),
                });
                self.vertices.push(PathVertex {
                    xy_position: c,
                    st_position: vec2f(1., 1.),
                });
            }
        }
    }
}

pub fn deserialize_vec2f<'de, D>(deserializer: D) -> Result<Vector2F, D::Error>
where
    D: Deserializer<'de>,
{
    let [x, y]: [f32; 2] = Deserialize::deserialize(deserializer)?;
    Ok(vec2f(x, y))
}

impl ToJson for Vector2F {
    fn to_json(&self) -> serde_json::Value {
        json!([self.x(), self.y()])
    }
}

impl ToJson for RectF {
    fn to_json(&self) -> serde_json::Value {
        json!({"origin": self.origin().to_json(), "size": self.size().to_json()})
    }
}

#[derive(Clone)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Into<taffy::geometry::Point<T>> for Point<T> {
    fn into(self) -> taffy::geometry::Point<T> {
        taffy::geometry::Point {
            x: self.x,
            y: self.y,
        }
    }
}

#[derive(Clone)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

impl<S, T> From<taffy::geometry::Size<S>> for Size<T>
where
    S: Into<T>,
{
    fn from(value: taffy::geometry::Size<S>) -> Self {
        Self {
            width: value.width.into(),
            height: value.height.into(),
        }
    }
}

impl<S, T> Into<taffy::geometry::Size<S>> for Size<T>
where
    T: Into<S>,
{
    fn into(self) -> taffy::geometry::Size<S> {
        taffy::geometry::Size {
            width: self.width.into(),
            height: self.height.into(),
        }
    }
}

impl Size<DefinedLength> {
    pub const fn zero() -> Self {
        Self {
            width: DefinedLength::Pixels(0.),
            height: DefinedLength::Pixels(0.),
        }
    }

    pub fn to_taffy(&self, rem_size: f32) -> taffy::geometry::Size<taffy::style::LengthPercentage> {
        taffy::geometry::Size {
            width: self.width.to_taffy(rem_size),
            height: self.height.to_taffy(rem_size),
        }
    }
}

impl Size<Length> {
    pub const fn auto() -> Self {
        Self {
            width: Length::Auto,
            height: Length::Auto,
        }
    }

    pub fn to_taffy<T: From<taffy::prelude::LengthPercentageAuto>>(
        &self,
        rem_size: f32,
    ) -> taffy::geometry::Size<T> {
        taffy::geometry::Size {
            width: self.width.to_taffy(rem_size).into(),
            height: self.height.to_taffy(rem_size).into(),
        }
    }
}

#[derive(Clone)]
pub struct Edges<T> {
    pub top: T,
    pub right: T,
    pub bottom: T,
    pub left: T,
}

impl Edges<DefinedLength> {
    pub const fn zero() -> Self {
        Self {
            top: DefinedLength::Pixels(0.0),
            right: DefinedLength::Pixels(0.0),
            bottom: DefinedLength::Pixels(0.0),
            left: DefinedLength::Pixels(0.0),
        }
    }

    pub fn to_taffy(&self, rem_size: f32) -> taffy::geometry::Rect<taffy::style::LengthPercentage> {
        taffy::geometry::Rect {
            top: self.top.to_taffy(rem_size),
            right: self.right.to_taffy(rem_size),
            bottom: self.bottom.to_taffy(rem_size),
            left: self.left.to_taffy(rem_size),
        }
    }
}

impl Edges<Length> {
    pub const fn auto() -> Self {
        Self {
            top: Length::Auto,
            right: Length::Auto,
            bottom: Length::Auto,
            left: Length::Auto,
        }
    }

    pub const fn zero() -> Self {
        Self {
            top: Length::Defined(DefinedLength::Pixels(0.0)),
            right: Length::Defined(DefinedLength::Pixels(0.0)),
            bottom: Length::Defined(DefinedLength::Pixels(0.0)),
            left: Length::Defined(DefinedLength::Pixels(0.0)),
        }
    }

    pub fn to_taffy(
        &self,
        rem_size: f32,
    ) -> taffy::geometry::Rect<taffy::style::LengthPercentageAuto> {
        taffy::geometry::Rect {
            top: self.top.to_taffy(rem_size),
            right: self.right.to_taffy(rem_size),
            bottom: self.bottom.to_taffy(rem_size),
            left: self.left.to_taffy(rem_size),
        }
    }
}

/// A non-auto length that can be defined in pixels, rems, or percent of parent.
#[derive(Clone, Copy)]
pub enum DefinedLength {
    Pixels(f32),
    Rems(f32),
    Percent(f32), // 0. - 100.
}

impl DefinedLength {
    fn to_taffy(&self, rem_size: f32) -> taffy::style::LengthPercentage {
        match self {
            DefinedLength::Pixels(pixels) => taffy::style::LengthPercentage::Length(*pixels),
            DefinedLength::Rems(rems) => taffy::style::LengthPercentage::Length(rems * rem_size),
            DefinedLength::Percent(percent) => {
                taffy::style::LengthPercentage::Percent(*percent / 100.)
            }
        }
    }
}

/// A length that can be defined in pixels, rems, percent of parent, or auto.
#[derive(Clone, Copy)]
pub enum Length {
    Defined(DefinedLength),
    Auto,
}

pub fn auto() -> Length {
    Length::Auto
}

pub fn percent(percent: f32) -> DefinedLength {
    DefinedLength::Percent(percent)
}

pub fn rems(rems: f32) -> DefinedLength {
    DefinedLength::Rems(rems)
}

pub fn pixels(pixels: f32) -> DefinedLength {
    DefinedLength::Pixels(pixels)
}

impl Length {
    pub fn to_taffy(&self, rem_size: f32) -> taffy::prelude::LengthPercentageAuto {
        match self {
            Length::Defined(length) => length.to_taffy(rem_size).into(),
            Length::Auto => taffy::prelude::LengthPercentageAuto::Auto,
        }
    }
}

impl From<DefinedLength> for Length {
    fn from(value: DefinedLength) -> Self {
        Length::Defined(value)
    }
}
