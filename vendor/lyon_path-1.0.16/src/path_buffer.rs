//! A container to store multiple paths contiguously.

use crate::builder::*;
use crate::math::*;
use crate::path;
use crate::{Attributes, EndpointId, PathSlice, NO_ATTRIBUTES};

use core::fmt;
use core::iter::{FromIterator, FusedIterator, IntoIterator};
use core::ops::Range;

use alloc::vec::Vec;

#[derive(Clone, Debug)]
struct PathDescriptor {
    points: (u32, u32),
    verbs: (u32, u32),
    num_attributes: u32,
}

/// An object that stores multiple paths contiguously.
#[derive(Clone, Default)]
pub struct PathBuffer {
    points: Vec<Point>,
    verbs: Vec<path::Verb>,
    paths: Vec<PathDescriptor>,
}

impl PathBuffer {
    #[inline]
    pub fn new() -> Self {
        PathBuffer {
            points: Vec::new(),
            verbs: Vec::new(),
            paths: Vec::new(),
        }
    }

    #[inline]
    pub fn with_capacity(endpoints: usize, ctrl_points: usize, paths: usize) -> Self {
        let mut buffer = PathBuffer::new();
        buffer.reserve(endpoints, ctrl_points, paths);

        buffer
    }

    #[inline]
    pub fn as_slice(&self) -> PathBufferSlice {
        PathBufferSlice {
            points: &self.points,
            verbs: &self.verbs,
            paths: &self.paths,
        }
    }

    #[inline]
    pub fn get(&self, index: usize) -> PathSlice {
        let desc = &self.paths[index];
        PathSlice {
            points: &self.points[desc.points.0 as usize..desc.points.1 as usize],
            verbs: &self.verbs[desc.verbs.0 as usize..desc.verbs.1 as usize],
            num_attributes: desc.num_attributes as usize,
        }
    }

    #[inline]
    pub fn indices(&self) -> Range<usize> {
        0..self.paths.len()
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_> {
        Iter::new(&self.points, &self.verbs, &self.paths)
    }

    #[inline]
    /// Returns the number of paths in the path buffer.
    pub fn len(&self) -> usize {
        self.paths.len()
    }

    /// Returns whether the path buffer is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.paths.is_empty()
    }

    #[inline]
    pub fn builder(&mut self) -> Builder {
        Builder::new(self)
    }

    #[inline]
    pub fn clear(&mut self) {
        self.points.clear();
        self.verbs.clear();
        self.paths.clear();
    }

    #[inline]
    pub fn reserve(&mut self, endpoints: usize, ctrl_points: usize, paths: usize) {
        self.points.reserve(endpoints + ctrl_points);
        self.verbs.reserve(endpoints);
        self.paths.reserve(paths);
    }
}

impl fmt::Debug for PathBuffer {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.as_slice().fmt(formatter)
    }
}

impl<'l> FromIterator<PathSlice<'l>> for PathBuffer {
    fn from_iter<T: IntoIterator<Item = PathSlice<'l>>>(iter: T) -> PathBuffer {
        iter.into_iter()
            .fold(PathBuffer::new(), |mut buffer, path| {
                let builder = buffer.builder();
                path.iter()
                    .fold(builder, |mut builder, event| {
                        builder.path_event(event, NO_ATTRIBUTES);
                        builder
                    })
                    .build();
                buffer
            })
    }
}

/// A view on a `PathBuffer`.
#[derive(Clone)]
pub struct PathBufferSlice<'l> {
    points: &'l [Point],
    verbs: &'l [path::Verb],
    paths: &'l [PathDescriptor],
}

impl<'l> PathBufferSlice<'l> {
    #[inline]
    pub fn get(&self, index: usize) -> PathSlice {
        let desc = &self.paths[index];
        PathSlice {
            points: &self.points[desc.points.0 as usize..desc.points.1 as usize],
            verbs: &self.verbs[desc.verbs.0 as usize..desc.verbs.1 as usize],
            num_attributes: desc.num_attributes as usize,
        }
    }

    #[inline]
    pub fn indices(&self) -> Range<usize> {
        0..self.paths.len()
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_> {
        Iter::new(self.points, self.verbs, self.paths)
    }

    /// Returns the number of paths in the path buffer.
    #[inline]
    pub fn len(&self) -> usize {
        self.paths.len()
    }

    /// Returns whether the path buffer is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.paths.is_empty()
    }
}

impl<'l> fmt::Debug for PathBufferSlice<'l> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "PathBuffer {{ paths: {:?}, points: {:?}, verbs: {:?}, ",
            self.paths.len(),
            self.points.len(),
            self.verbs.len(),
        )?;
        for idx in self.indices() {
            write!(formatter, "#{idx:?}: ")?;
            self.get(idx).fmt(formatter)?;
            write!(formatter, ", ")?;
        }
        write!(formatter, " }}")
    }
}

/// A Builder that appends a path to an existing PathBuffer.
///
/// Implements the `PathBuilder` trait.
pub struct Builder<'l> {
    buffer: &'l mut PathBuffer,
    builder: path::Builder,
    points_start: u32,
    verbs_start: u32,
}

impl<'l> Builder<'l> {
    #[inline]
    fn new(buffer: &'l mut PathBuffer) -> Self {
        let mut builder = path::Path::builder();
        core::mem::swap(&mut buffer.points, &mut builder.inner_mut().points);
        core::mem::swap(&mut buffer.verbs, &mut builder.inner_mut().verbs);
        let points_start = builder.inner().points.len() as u32;
        let verbs_start = builder.inner().verbs.len() as u32;
        Builder {
            buffer,
            builder,
            points_start,
            verbs_start,
        }
    }

    #[inline]
    pub fn with_attributes(self, num_attributes: usize) -> BuilderWithAttributes<'l> {
        assert_eq!(self.builder.inner().verbs.len(), self.verbs_start as usize);

        BuilderWithAttributes {
            buffer: self.buffer,
            builder: path::BuilderWithAttributes {
                builder: self.builder.into_inner(),
                num_attributes,
                first_attributes: alloc::vec![0.0; num_attributes],
            },
            points_start: self.points_start,
            verbs_start: self.verbs_start,
        }
    }

    #[inline]
    pub fn build(mut self) -> usize {
        let points_end = self.builder.inner().points.len() as u32;
        let verbs_end = self.builder.inner().verbs.len() as u32;
        core::mem::swap(
            &mut self.builder.inner_mut().points,
            &mut self.buffer.points,
        );
        core::mem::swap(&mut self.builder.inner_mut().verbs, &mut self.buffer.verbs);

        let index = self.buffer.paths.len();
        self.buffer.paths.push(PathDescriptor {
            points: (self.points_start, points_end),
            verbs: (self.verbs_start, verbs_end),
            num_attributes: 0,
        });

        index
    }

    #[inline]
    fn adjust_id(&self, mut id: EndpointId) -> EndpointId {
        id.0 -= self.points_start;

        id
    }

    #[inline]
    pub fn begin(&mut self, at: Point) -> EndpointId {
        let id = self.builder.begin(at);
        self.adjust_id(id)
    }

    #[inline]
    pub fn end(&mut self, close: bool) {
        self.builder.end(close)
    }

    #[inline]
    pub fn line_to(&mut self, to: Point) -> EndpointId {
        let id = self.builder.line_to(to);
        self.adjust_id(id)
    }

    #[inline]
    pub fn quadratic_bezier_to(&mut self, ctrl: Point, to: Point) -> EndpointId {
        let id = self.builder.quadratic_bezier_to(ctrl, to);
        self.adjust_id(id)
    }

    #[inline]
    pub fn cubic_bezier_to(&mut self, ctrl1: Point, ctrl2: Point, to: Point) -> EndpointId {
        let id = self.builder.cubic_bezier_to(ctrl1, ctrl2, to);
        self.adjust_id(id)
    }

    #[inline]
    pub fn reserve(&mut self, endpoints: usize, ctrl_points: usize) {
        self.builder.reserve(endpoints, ctrl_points);
    }
}

impl<'l> PathBuilder for Builder<'l> {
    #[inline]
    fn num_attributes(&self) -> usize {
        0
    }

    #[inline]
    fn begin(&mut self, at: Point, _attributes: Attributes) -> EndpointId {
        self.begin(at)
    }

    #[inline]
    fn end(&mut self, close: bool) {
        self.end(close);
    }

    #[inline]
    fn line_to(&mut self, to: Point, _attributes: Attributes) -> EndpointId {
        self.line_to(to)
    }

    #[inline]
    fn quadratic_bezier_to(
        &mut self,
        ctrl: Point,
        to: Point,
        _attributes: Attributes,
    ) -> EndpointId {
        self.quadratic_bezier_to(ctrl, to)
    }

    #[inline]
    fn cubic_bezier_to(
        &mut self,
        ctrl1: Point,
        ctrl2: Point,
        to: Point,
        _attributes: Attributes,
    ) -> EndpointId {
        self.cubic_bezier_to(ctrl1, ctrl2, to)
    }

    #[inline]
    fn reserve(&mut self, endpoints: usize, ctrl_points: usize) {
        self.reserve(endpoints, ctrl_points);
    }
}

impl<'l> Build for Builder<'l> {
    type PathType = usize;
    fn build(self) -> usize {
        self.build()
    }
}

/// A Builder that appends a path to an existing PathBuffer, with custom attributes.
pub struct BuilderWithAttributes<'l> {
    buffer: &'l mut PathBuffer,
    builder: path::BuilderWithAttributes,
    points_start: u32,
    verbs_start: u32,
}

impl<'l> BuilderWithAttributes<'l> {
    #[inline]
    pub fn new(buffer: &'l mut PathBuffer, num_attributes: usize) -> Self {
        let mut builder = path::Path::builder().into_inner();
        core::mem::swap(&mut buffer.points, &mut builder.points);
        core::mem::swap(&mut buffer.verbs, &mut builder.verbs);
        let points_start = builder.points.len() as u32;
        let verbs_start = builder.verbs.len() as u32;
        BuilderWithAttributes {
            buffer,
            builder: path::BuilderWithAttributes {
                builder,
                num_attributes,
                first_attributes: alloc::vec![0.0; num_attributes],
            },
            points_start,
            verbs_start,
        }
    }

    #[inline]
    pub fn build(mut self) -> usize {
        let points_end = self.builder.builder.points.len() as u32;
        let verbs_end = self.builder.builder.verbs.len() as u32;
        core::mem::swap(&mut self.builder.builder.points, &mut self.buffer.points);
        core::mem::swap(&mut self.builder.builder.verbs, &mut self.buffer.verbs);

        let index = self.buffer.paths.len();
        self.buffer.paths.push(PathDescriptor {
            points: (self.points_start, points_end),
            verbs: (self.verbs_start, verbs_end),
            num_attributes: 0,
        });

        index
    }

    #[inline]
    fn adjust_id(&self, mut id: EndpointId) -> EndpointId {
        id.0 -= self.points_start;

        id
    }

    #[inline]
    pub fn begin(&mut self, at: Point, attributes: Attributes) -> EndpointId {
        let id = self.builder.begin(at, attributes);
        self.adjust_id(id)
    }

    #[inline]
    pub fn end(&mut self, close: bool) {
        self.builder.end(close)
    }

    #[inline]
    pub fn line_to(&mut self, to: Point, attributes: Attributes) -> EndpointId {
        let id = self.builder.line_to(to, attributes);
        self.adjust_id(id)
    }

    #[inline]
    pub fn quadratic_bezier_to(
        &mut self,
        ctrl: Point,
        to: Point,
        attributes: Attributes,
    ) -> EndpointId {
        let id = self.builder.quadratic_bezier_to(ctrl, to, attributes);
        self.adjust_id(id)
    }

    #[inline]
    pub fn cubic_bezier_to(
        &mut self,
        ctrl1: Point,
        ctrl2: Point,
        to: Point,
        attributes: Attributes,
    ) -> EndpointId {
        let id = self.builder.cubic_bezier_to(ctrl1, ctrl2, to, attributes);
        self.adjust_id(id)
    }

    #[inline]
    pub fn reserve(&mut self, endpoints: usize, ctrl_points: usize) {
        self.builder.reserve(endpoints, ctrl_points);
    }
}

impl<'l> PathBuilder for BuilderWithAttributes<'l> {
    #[inline]
    fn num_attributes(&self) -> usize {
        self.builder.num_attributes()
    }

    #[inline]
    fn begin(&mut self, at: Point, attributes: Attributes) -> EndpointId {
        self.begin(at, attributes)
    }

    #[inline]
    fn end(&mut self, close: bool) {
        self.end(close);
    }

    #[inline]
    fn line_to(&mut self, to: Point, attributes: Attributes) -> EndpointId {
        self.line_to(to, attributes)
    }

    #[inline]
    fn quadratic_bezier_to(
        &mut self,
        ctrl: Point,
        to: Point,
        attributes: Attributes,
    ) -> EndpointId {
        self.quadratic_bezier_to(ctrl, to, attributes)
    }

    #[inline]
    fn cubic_bezier_to(
        &mut self,
        ctrl1: Point,
        ctrl2: Point,
        to: Point,
        attributes: Attributes,
    ) -> EndpointId {
        self.cubic_bezier_to(ctrl1, ctrl2, to, attributes)
    }

    #[inline]
    fn reserve(&mut self, endpoints: usize, ctrl_points: usize) {
        self.reserve(endpoints, ctrl_points);
    }
}

impl<'l> Build for BuilderWithAttributes<'l> {
    type PathType = usize;
    fn build(self) -> usize {
        self.build()
    }
}

/// Iterator over the paths in a [`PathBufferSlice`].
#[derive(Clone)]
pub struct Iter<'l> {
    points: &'l [Point],
    verbs: &'l [path::Verb],
    paths: ::core::slice::Iter<'l, PathDescriptor>,
}

impl<'l> Iter<'l> {
    fn new(points: &'l [Point], verbs: &'l [path::Verb], paths: &'l [PathDescriptor]) -> Iter<'l> {
        Iter {
            points,
            verbs,
            paths: paths.iter(),
        }
    }
}

impl<'l> Iterator for Iter<'l> {
    type Item = PathSlice<'l>;

    fn next(&mut self) -> Option<PathSlice<'l>> {
        let path = self.paths.next()?;
        Some(PathSlice {
            points: &self.points[path.points.0 as usize..path.points.1 as usize],
            verbs: &self.verbs[path.verbs.0 as usize..path.verbs.1 as usize],
            num_attributes: path.num_attributes as usize,
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.paths.size_hint()
    }
}

// slice::Iter is Fused and ExactSize
impl<'l> FusedIterator for Iter<'l> {}
impl<'l> ExactSizeIterator for Iter<'l> {}

impl<'l> DoubleEndedIterator for Iter<'l> {
    fn next_back(&mut self) -> Option<PathSlice<'l>> {
        let path = self.paths.next_back()?;
        Some(PathSlice {
            points: &self.points[path.points.0 as usize..path.points.1 as usize],
            verbs: &self.verbs[path.verbs.0 as usize..path.verbs.1 as usize],
            num_attributes: path.num_attributes as usize,
        })
    }
}

#[test]
fn simple() {
    use crate::PathEvent;

    let mut buffer = PathBuffer::new();

    let mut builder = buffer.builder();
    builder.begin(point(0.0, 0.0));
    builder.line_to(point(10.0, 0.0));
    builder.line_to(point(10.0, 10.0));
    let a = builder.line_to(point(0.0, 10.0));
    builder.end(true);

    let p1 = builder.build();

    let mut builder = buffer.builder();
    builder.begin(point(0.0, 0.0));
    builder.line_to(point(20.0, 0.0));
    builder.line_to(point(20.0, 20.0));
    let b = builder.line_to(point(0.0, 20.0));
    builder.end(false);

    let p2 = builder.build();

    let mut iter = buffer.get(p1).iter();
    assert_eq!(
        iter.next(),
        Some(PathEvent::Begin {
            at: point(0.0, 0.0)
        })
    );
    assert_eq!(
        iter.next(),
        Some(PathEvent::Line {
            from: point(0.0, 0.0),
            to: point(10.0, 0.0)
        })
    );
    assert_eq!(
        iter.next(),
        Some(PathEvent::Line {
            from: point(10.0, 0.0),
            to: point(10.0, 10.0)
        })
    );
    assert_eq!(
        iter.next(),
        Some(PathEvent::Line {
            from: point(10.0, 10.0),
            to: point(0.0, 10.0)
        })
    );
    assert_eq!(
        iter.next(),
        Some(PathEvent::End {
            last: point(0.0, 10.0),
            first: point(0.0, 0.0),
            close: true
        })
    );
    assert_eq!(iter.next(), None);

    let mut iter = buffer.get(p2).iter();
    assert_eq!(
        iter.next(),
        Some(PathEvent::Begin {
            at: point(0.0, 0.0)
        })
    );
    assert_eq!(
        iter.next(),
        Some(PathEvent::Line {
            from: point(0.0, 0.0),
            to: point(20.0, 0.0)
        })
    );
    assert_eq!(
        iter.next(),
        Some(PathEvent::Line {
            from: point(20.0, 0.0),
            to: point(20.0, 20.0)
        })
    );
    assert_eq!(
        iter.next(),
        Some(PathEvent::Line {
            from: point(20.0, 20.0),
            to: point(0.0, 20.0)
        })
    );
    assert_eq!(
        iter.next(),
        Some(PathEvent::End {
            last: point(0.0, 20.0),
            first: point(0.0, 0.0),
            close: false
        })
    );
    assert_eq!(iter.next(), None);

    assert_eq!(buffer.get(p1)[a], point(0.0, 10.0));
    assert_eq!(buffer.get(p2)[b], point(0.0, 20.0));
}
