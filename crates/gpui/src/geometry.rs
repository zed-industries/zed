use std::fmt::Debug;

use super::scene::{Path, PathVertex};
use crate::{color::Color, json::ToJson};
pub use pathfinder_geometry::*;
use rect::RectF;
use refineable::Refineable;
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

#[derive(Refineable, Debug)]
#[refineable(debug)]
pub struct Point<T: Clone + Default + Debug> {
    pub x: T,
    pub y: T,
}

impl<T: Clone + Default + Debug> Clone for Point<T> {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

impl<T: Clone + Default + Debug> Into<taffy::geometry::Point<T>> for Point<T> {
    fn into(self) -> taffy::geometry::Point<T> {
        taffy::geometry::Point {
            x: self.x,
            y: self.y,
        }
    }
}

#[derive(Refineable, Clone, Debug)]
#[refineable(debug)]
pub struct Size<T: Clone + Default + Debug> {
    pub width: T,
    pub height: T,
}

impl Size<Length> {
    pub fn full() -> Self {
        Self {
            width: relative(1.),
            height: relative(1.),
        }
    }
}

impl<S, T: Clone + Default + Debug> From<taffy::geometry::Size<S>> for Size<T>
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

impl<S, T: Clone + Default + Debug> Into<taffy::geometry::Size<S>> for Size<T>
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

impl Size<DefiniteLength> {
    pub fn zero() -> Self {
        Self {
            width: px(0.),
            height: px(0.),
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
    pub fn auto() -> Self {
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

#[derive(Clone, Default, Refineable, Debug)]
#[refineable(debug)]
pub struct Edges<T: Clone + Default + Debug> {
    pub top: T,
    pub right: T,
    pub bottom: T,
    pub left: T,
}

impl Edges<Length> {
    pub fn auto() -> Self {
        Self {
            top: Length::Auto,
            right: Length::Auto,
            bottom: Length::Auto,
            left: Length::Auto,
        }
    }

    pub fn zero() -> Self {
        Self {
            top: px(0.),
            right: px(0.),
            bottom: px(0.),
            left: px(0.),
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

impl Edges<DefiniteLength> {
    pub fn zero() -> Self {
        Self {
            top: px(0.),
            right: px(0.),
            bottom: px(0.),
            left: px(0.),
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

impl Edges<AbsoluteLength> {
    pub fn zero() -> Self {
        Self {
            top: px(0.),
            right: px(0.),
            bottom: px(0.),
            left: px(0.),
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

    pub fn to_pixels(&self, rem_size: f32) -> Edges<f32> {
        Edges {
            top: self.top.to_pixels(rem_size),
            right: self.right.to_pixels(rem_size),
            bottom: self.bottom.to_pixels(rem_size),
            left: self.left.to_pixels(rem_size),
        }
    }
}

impl Edges<f32> {
    pub fn is_empty(&self) -> bool {
        self.top == 0.0 && self.right == 0.0 && self.bottom == 0.0 && self.left == 0.0
    }
}

#[derive(Clone, Copy)]
pub enum AbsoluteLength {
    Pixels(f32),
    Rems(f32),
}

impl std::fmt::Debug for AbsoluteLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AbsoluteLength::Pixels(pixels) => write!(f, "{}px", pixels),
            AbsoluteLength::Rems(rems) => write!(f, "{}rems", rems),
        }
    }
}

impl AbsoluteLength {
    pub fn to_pixels(&self, rem_size: f32) -> f32 {
        match self {
            AbsoluteLength::Pixels(pixels) => *pixels,
            AbsoluteLength::Rems(rems) => rems * rem_size,
        }
    }

    pub fn to_taffy(&self, rem_size: f32) -> taffy::style::LengthPercentage {
        match self {
            AbsoluteLength::Pixels(pixels) => taffy::style::LengthPercentage::Length(*pixels),
            AbsoluteLength::Rems(rems) => taffy::style::LengthPercentage::Length(rems * rem_size),
        }
    }
}

impl Default for AbsoluteLength {
    fn default() -> Self {
        Self::Pixels(0.0)
    }
}

/// A non-auto length that can be defined in pixels, rems, or percent of parent.
#[derive(Clone, Copy)]
pub enum DefiniteLength {
    Absolute(AbsoluteLength),
    Relative(f32), // 0. to 1.
}

impl DefiniteLength {
    fn to_taffy(&self, rem_size: f32) -> taffy::style::LengthPercentage {
        match self {
            DefiniteLength::Absolute(length) => match length {
                AbsoluteLength::Pixels(pixels) => taffy::style::LengthPercentage::Length(*pixels),
                AbsoluteLength::Rems(rems) => {
                    taffy::style::LengthPercentage::Length(rems * rem_size)
                }
            },
            DefiniteLength::Relative(fraction) => {
                taffy::style::LengthPercentage::Percent(*fraction)
            }
        }
    }
}

impl std::fmt::Debug for DefiniteLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DefiniteLength::Absolute(length) => std::fmt::Debug::fmt(length, f),
            DefiniteLength::Relative(fract) => write!(f, "{}%", (fract * 100.0) as i32),
        }
    }
}

impl From<AbsoluteLength> for DefiniteLength {
    fn from(length: AbsoluteLength) -> Self {
        Self::Absolute(length)
    }
}

impl Default for DefiniteLength {
    fn default() -> Self {
        Self::Absolute(AbsoluteLength::default())
    }
}

/// A length that can be defined in pixels, rems, percent of parent, or auto.
#[derive(Clone, Copy)]
pub enum Length {
    Definite(DefiniteLength),
    Auto,
}

impl std::fmt::Debug for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Length::Definite(definite_length) => write!(f, "{:?}", definite_length),
            Length::Auto => write!(f, "auto"),
        }
    }
}

pub fn relative<T: From<DefiniteLength>>(fraction: f32) -> T {
    DefiniteLength::Relative(fraction).into()
}

pub fn rems<T: From<AbsoluteLength>>(rems: f32) -> T {
    AbsoluteLength::Rems(rems).into()
}

pub fn px<T: From<AbsoluteLength>>(pixels: f32) -> T {
    AbsoluteLength::Pixels(pixels).into()
}

pub fn pixels<T: From<AbsoluteLength>>(pixels: f32) -> T {
    AbsoluteLength::Pixels(pixels).into()
}

pub fn auto() -> Length {
    Length::Auto
}

impl Length {
    pub fn to_taffy(&self, rem_size: f32) -> taffy::prelude::LengthPercentageAuto {
        match self {
            Length::Definite(length) => length.to_taffy(rem_size).into(),
            Length::Auto => taffy::prelude::LengthPercentageAuto::Auto,
        }
    }
}

impl From<DefiniteLength> for Length {
    fn from(length: DefiniteLength) -> Self {
        Self::Definite(length)
    }
}

impl From<AbsoluteLength> for Length {
    fn from(length: AbsoluteLength) -> Self {
        Self::Definite(length.into())
    }
}

impl Default for Length {
    fn default() -> Self {
        Self::Definite(DefiniteLength::default())
    }
}

impl From<()> for Length {
    fn from(_: ()) -> Self {
        Self::Definite(DefiniteLength::default())
    }
}
