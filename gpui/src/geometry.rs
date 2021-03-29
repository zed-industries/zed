pub use pathfinder_geometry::*;

use super::scene::{Path, PathVertex};
use vector::{vec2f, Vector2F};

pub struct PathBuilder {
    vertices: Vec<PathVertex>,
    start: Vector2F,
    current: Vector2F,
    contour_count: usize,
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

    pub fn build(self) -> Path {
        Path {
            vertices: self.vertices,
        }
    }

    fn push_triangle(&mut self, a: Vector2F, b: Vector2F, c: Vector2F, kind: PathVertexKind) {
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
