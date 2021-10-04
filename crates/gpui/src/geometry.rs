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
            self.bounds = self
                .bounds
                .intersection(clip_bounds)
                .unwrap_or(RectF::default());
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
