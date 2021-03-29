pub use pathfinder_geometry::*;

use vector::{vec2f, Vector2F};

pub(crate) struct Vertex {
    xy_position: Vector2F,
    st_position: Vector2F,
}

pub struct Path {
    vertices: Vec<Vertex>,
    start: Vector2F,
    current: Vector2F,
    countours_len: usize,
}

enum Kind {
    Solid,
    Quadratic,
}

impl Path {
    fn new() -> Self {
        Self {
            vertices: Vec::new(),
            start: vec2f(0., 0.),
            current: vec2f(0., 0.),
            countours_len: 0,
        }
    }

    pub fn reset(&mut self, point: Vector2F) {
        self.vertices.clear();
        self.start = point;
        self.current = point;
        self.countours_len = 0;
    }

    pub fn line_to(&mut self, point: Vector2F) {
        self.countours_len += 1;
        if self.countours_len > 1 {
            self.push_triangle(self.start, self.current, point, Kind::Solid);
        }

        self.current = point;
    }

    pub fn curve_to(&mut self, point: Vector2F, ctrl: Vector2F) {
        self.countours_len += 1;
        if self.countours_len > 1 {
            self.push_triangle(self.start, self.current, point, Kind::Solid);
        }

        self.push_triangle(self.current, ctrl, point, Kind::Quadratic);
        self.current = point;
    }

    pub(crate) fn close(self) -> Vec<Vertex> {
        self.vertices
    }

    fn push_triangle(&mut self, a: Vector2F, b: Vector2F, c: Vector2F, kind: Kind) {
        match kind {
            Kind::Solid => {
                self.vertices.push(Vertex {
                    xy_position: a,
                    st_position: vec2f(0., 1.),
                });
                self.vertices.push(Vertex {
                    xy_position: b,
                    st_position: vec2f(0., 1.),
                });
                self.vertices.push(Vertex {
                    xy_position: c,
                    st_position: vec2f(0., 1.),
                });
            }
            Kind::Quadratic => {
                self.vertices.push(Vertex {
                    xy_position: a,
                    st_position: vec2f(0., 0.),
                });
                self.vertices.push(Vertex {
                    xy_position: b,
                    st_position: vec2f(0.5, 0.),
                });
                self.vertices.push(Vertex {
                    xy_position: c,
                    st_position: vec2f(1., 1.),
                });
            }
        }
    }
}
