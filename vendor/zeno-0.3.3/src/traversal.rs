//! Path traversal algorithms.

use super::command::{Command, TransformCommands};
use super::geometry::*;
use super::path_data::PathData;
use super::segment::{segments, Segment, Segments};

use core::borrow::Borrow;
use core::cell::RefCell;

/// A vertex of a path.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Vertex {
    /// The start point and direction of a subpath.
    Start(Point, Vector),
    /// The incoming direction, location, and outgoing direction of an
    /// intermediate vertex in a subpath.
    Middle(Vector, Point, Vector),
    /// The incoming direction and location of the final vertex in a subpath.
    /// The boolean value is true if the subpath is closed.
    End(Vector, Point, bool),
}

/// An iterator over the vertices of a path.
#[derive(Clone)]
pub struct Vertices<D> {
    segments: Segments<D>,
    prev_point: Point,
    prev_dir: Vector,
    is_first: bool,
}

impl<D> Vertices<D>
where
    D: Iterator<Item = Command> + Clone,
{
    /// Creates a new iterator over the vertices of a path.
    pub fn new(data: impl PathData<Commands = D>) -> Self {
        Self {
            segments: segments(data.commands(), false),
            prev_point: Point::ZERO,
            prev_dir: Vector::new(1., 0.),
            is_first: true,
        }
    }
}

impl<D> Vertices<TransformCommands<D>>
where
    D: Iterator<Item = Command> + Clone,
{
    /// Creates a new iterator over the vertices of a transformed path.
    pub fn with_transform(data: impl PathData<Commands = D>, transform: Transform) -> Self {
        let data = TransformCommands {
            data: data.commands(),
            transform,
        };
        Self {
            segments: segments(data, false),
            prev_point: Point::ZERO,
            prev_dir: Vector::new(1., 0.),
            is_first: true,
        }
    }
}

impl<D> Iterator for Vertices<D>
where
    D: Iterator + Clone,
    D::Item: Borrow<Command>,
{
    type Item = Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        use Segment::*;
        if self.is_first {
            self.is_first = false;
            match self.segments.next()?.borrow() {
                End(closed) => {
                    self.is_first = true;
                    Some(Vertex::End(self.prev_dir, self.prev_point, *closed))
                }
                segment => {
                    let (start, in_dir, out_dir, end) = get_components(segment);
                    self.prev_dir = out_dir;
                    self.prev_point = end;
                    Some(Vertex::Start(start, in_dir))
                }
            }
        } else {
            match self.segments.next()?.borrow() {
                End(closed) => {
                    self.is_first = true;
                    Some(Vertex::End(self.prev_dir, self.prev_point, *closed))
                }
                segment => {
                    let (start, in_dir, out_dir, end) = get_components(segment);
                    let prev_dir = self.prev_dir;
                    self.prev_dir = out_dir;
                    self.prev_point = end;
                    Some(Vertex::Middle(prev_dir, start, in_dir))
                }
            }
        }
    }
}

fn get_components(segment: &Segment) -> (Point, Vector, Vector, Point) {
    match segment {
        Segment::Curve(_, curve) => {
            let a = curve.evaluate(0.05);
            let b = curve.evaluate(0.95);
            let a_dir = (a - curve.a).normalize();
            let b_dir = (curve.d - b).normalize();
            (curve.a, a_dir, b_dir, curve.d)
        }
        Segment::Line(_, line) => {
            let dir = (line.b - line.a).normalize();
            (line.a, dir, dir, line.b)
        }
        Segment::End(..) => (Point::ZERO, Vector::ZERO, Vector::ZERO, Point::ZERO),
    }
}

/// An iterator like type that walks along a path by arbitrary steps.
pub struct Walk<D> {
    init: Segments<D>,
    iter: Segments<D>,
    segment: Segment,
    segment_offset: f32,
    first: bool,
    length: RefCell<Option<f32>>,
    walked: f32,
}

impl<D> Walk<D>
where
    D: Iterator<Item = Command> + Clone,
{
    /// Creates a new iterator like type that steps along a path by arbitrary distances.
    pub fn new(data: impl PathData<Commands = D>) -> Self {
        let data = data.commands();
        Self {
            init: segments(data.clone(), false),
            iter: segments(data, false),
            segment: Segment::default(),
            segment_offset: 0.,
            first: true,
            length: RefCell::new(None),
            walked: 0.,
        }
    }
}

impl<D> Walk<TransformCommands<D>>
where
    D: Iterator<Item = Command> + Clone,
{
    /// Creates a new iterator like type that steps along a transformed path by arbitrary distances.
    pub fn with_transform(data: impl PathData<Commands = D>, transform: Transform) -> Self {
        let data = data.commands();
        let data = TransformCommands { data, transform };
        Self {
            init: segments(data.clone(), false),
            iter: segments(data, false),
            segment: Segment::default(),
            segment_offset: 0.,
            first: true,
            length: RefCell::new(None),
            walked: 0.,
        }
    }
}

impl<D> Walk<D>
where
    D: Iterator + Clone,
    D::Item: Borrow<Command>,
{
    /// Steps by the specified distance and returns the point at the new
    /// location and the normal vector describing the left-ward direction at
    /// that point. Returns `None` if the distance steps beyond the end
    /// of the path.
    pub fn step(&mut self, distance: f32) -> Option<(Point, Vector)> {
        if self.first {
            self.segment = self.next_segment()?;
            self.segment_offset = 0.;
            self.first = false;
        }
        let mut t;
        let mut offset = self.segment_offset;
        let mut segment = self.segment;
        let mut remaining = distance;
        loop {
            let dt = segment.time(offset + remaining, 1.);
            remaining -= dt.distance - offset;
            t = dt.time;
            offset = dt.distance;
            if remaining <= 0. {
                break;
            }
            segment = self.next_segment()?;
            offset = 0.;
        }
        self.segment = segment;
        self.segment_offset = offset;
        self.walked += distance;
        let (p, n) = segment.point_normal(t);
        Some((p, n))
    }

    /// Returns the remaining distance available to walk on the path.
    pub fn remaining(&self) -> f32 {
        let mut l = self.length.borrow_mut();
        if l.is_none() {
            let iter = self.init.clone();
            let mut sum = 0.;
            for s in iter {
                sum += s.length();
            }
            *l = Some(sum);
        }
        l.unwrap() - self.walked
    }

    fn next_segment(&mut self) -> Option<Segment> {
        for s in self.iter.by_ref() {
            match s {
                Segment::End(..) => continue,
                _ => return Some(s),
            }
        }
        None
    }
}
