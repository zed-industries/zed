//! Perform cached measurements and split operations on a path.
//!
use crate::geom::{CubicBezierSegment, LineSegment, QuadraticBezierSegment, Segment};
use crate::math::*;
use crate::path::{
    builder::PathBuilder, AttributeStore, Attributes, EndpointId, IdEvent, Path, PathSlice,
    PositionStore,
};
use core::ops::Range;

use alloc::vec::Vec;

/// Whether to measure real or normalized (between 0 and 1) distances.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SampleType {
    Distance,
    Normalized,
}

type EndpointPair = (EndpointId, EndpointId);

enum SegmentWrapper {
    Empty,
    Line(LineSegment<f32>, EndpointPair),
    Quadratic(QuadraticBezierSegment<f32>, EndpointPair),
    Cubic(CubicBezierSegment<f32>, EndpointPair),
}

impl SegmentWrapper {
    fn split(&self, range: Range<f32>) -> Self {
        match self {
            Self::Empty => Self::Empty,
            Self::Line(segment, pair) => Self::Line(segment.split_range(range), *pair),
            Self::Quadratic(segment, pair) => Self::Quadratic(segment.split_range(range), *pair),
            Self::Cubic(segment, pair) => Self::Cubic(segment.split_range(range), *pair),
        }
    }
}

struct Edge {
    // distance from the beginning of the path
    distance: f32,
    // which segment this edge is on
    index: usize,
    // t-value of the endpoint on the segment
    t: f32,
}

/// The result of sampling a path.
#[derive(PartialEq, Debug)]
pub struct PathSample<'l> {
    position: Point,
    tangent: Vector,
    attributes: Attributes<'l>,
}

impl<'l> PathSample<'l> {
    #[inline]
    pub fn position(&self) -> Point {
        self.position
    }

    #[inline]
    pub fn tangent(&self) -> Vector {
        self.tangent
    }

    // Takes &mut self to allow interpolating attributes lazily (like the stroke tessellator) without changing
    // the API.
    #[inline]
    pub fn attributes(&mut self) -> Attributes<'l> {
        self.attributes
    }
}

/// An acceleration structure for sampling distances along a specific path.
///
/// Building the path measurements can be an expensive operation depending on the complexity of the
/// measured path, so it is usually a good idea to cache and reuse it whenever possible.
///
/// Queries on path measurements are made via a sampler object (see `PathSampler`) which can be configured
/// to measure real distance or normalized ones (values between 0 and 1 with zero indicating the start
/// of the path and 1 indicating the end).
///
/// ## Differences with the `PathWalker`
///
/// The `walker` module provides a similar functionality via the `PathWalker`. The main differences are:
///  - The path walker does all of its computation on the fly without storing any information for later use.
///  - `PathMeasurements` stores potentially large amounts of data to speed up sample queries.
///  - The cost of creating `PathMeasurements` is similar to that of walking the entire path once.
///  - Once the `PathMeasurements` have been created, random samples on the path are much faster than path walking.
///  - The PathWalker does not handle normalized distances since the length of the path cannot be known without
///    traversing the entire path at least once.
///
/// Prefer `PathMeasurements` over `PathWalker` if the measurements can be cached and reused for a large number of
/// queries.
///
/// ## Example
///
/// ```
/// use lyon_algorithms::{
///     math::point,
///     path::Path,
///     length::approximate_length,
///     measure::{PathMeasurements, SampleType},
/// };
///
/// let mut path = Path::builder();
/// path.begin(point(0.0, 0.0));
/// path.quadratic_bezier_to(point(1.0, 1.0), point(2.0, 0.0));
/// path.end(false);
/// let path = path.build();
///
/// // Build the acceleration structure.
/// let measurements = PathMeasurements::from_path(&path, 1e-3);
/// let mut sampler = measurements.create_sampler(&path, SampleType::Normalized);
///
/// let sample  = sampler.sample(0.5);
/// println!("Mid-point position: {:?}, tangent: {:?}", sample.position(), sample.tangent());
///
/// let mut second_half = Path::builder();
/// sampler.split_range(0.5..1.0, &mut second_half);
/// let second_half = second_half.build();
/// assert!((sampler.length() / 2.0 - approximate_length(&second_half, 1e-3)).abs() < 1e-3);
/// ```
///
pub struct PathMeasurements {
    events: Vec<IdEvent>,
    edges: Vec<Edge>,
}

impl PathMeasurements {
    /// Create empty path measurements.
    ///
    /// The measurements cannot be used until it has been initialized.
    pub fn empty() -> Self {
        PathMeasurements {
            events: Vec::new(),
            edges: Vec::new(),
        }
    }

    /// Create path measurements initialized with a `Path`.
    pub fn from_path(path: &Path, tolerance: f32) -> Self {
        let mut m = Self::empty();
        m.initialize(path.id_iter(), path, tolerance);

        m
    }

    /// Create path measurements initialized with a `PathSlice`.
    pub fn from_path_slice(path: &PathSlice, tolerance: f32) -> Self {
        let mut m = Self::empty();
        m.initialize(path.id_iter(), path, tolerance);

        m
    }

    /// Create path measurements initialized with a generic iterator and position store.
    pub fn from_iter<Iter, PS>(path: Iter, positions: &PS, tolerance: f32) -> Self
    where
        Iter: IntoIterator<Item = IdEvent>,
        PS: PositionStore,
    {
        let mut m = Self::empty();
        m.initialize(path, positions, tolerance);

        m
    }

    /// Initialize the path measurements with a path.
    pub fn initialize<Iter, PS>(&mut self, path: Iter, position_store: &PS, tolerance: f32)
    where
        Iter: IntoIterator<Item = IdEvent>,
        PS: PositionStore,
    {
        let tolerance = tolerance.max(1e-4);
        let mut events = core::mem::take(&mut self.events);
        events.clear();
        events.extend(path);
        let mut edges = core::mem::take(&mut self.edges);
        edges.clear();

        let mut distance = 0.0;
        for (index, event) in events.iter().cloned().enumerate() {
            match event {
                IdEvent::Begin { .. } => {
                    edges.push(Edge {
                        distance,
                        index,
                        t: 1.0,
                    });
                }
                IdEvent::Line { from, to } => {
                    let from = position_store.get_endpoint(from);
                    let to = position_store.get_endpoint(to);
                    distance += (from - to).length();
                    edges.push(Edge {
                        distance,
                        index,
                        t: 1.0,
                    })
                }
                IdEvent::Quadratic { from, ctrl, to } => {
                    let from = position_store.get_endpoint(from);
                    let to = position_store.get_endpoint(to);
                    let ctrl = position_store.get_control_point(ctrl);
                    let segment = QuadraticBezierSegment { from, ctrl, to };
                    segment.for_each_flattened_with_t(tolerance, &mut |line, t| {
                        distance += line.length();
                        edges.push(Edge {
                            distance,
                            index,
                            t: t.end,
                        });
                    });
                }
                IdEvent::Cubic {
                    from,
                    ctrl1,
                    ctrl2,
                    to,
                } => {
                    let from = position_store.get_endpoint(from);
                    let to = position_store.get_endpoint(to);
                    let ctrl1 = position_store.get_control_point(ctrl1);
                    let ctrl2 = position_store.get_control_point(ctrl2);
                    let segment = CubicBezierSegment {
                        from,
                        ctrl1,
                        ctrl2,
                        to,
                    };
                    segment.for_each_flattened_with_t(tolerance, &mut |line, t| {
                        distance += line.length();
                        edges.push(Edge {
                            distance,
                            index,
                            t: t.end,
                        });
                    });
                }
                IdEvent::End {
                    last,
                    first,
                    close: true,
                } => {
                    let last = position_store.get_endpoint(last);
                    let first = position_store.get_endpoint(first);
                    distance += (last - first).length();
                    edges.push(Edge {
                        distance,
                        index,
                        t: 1.0,
                    })
                }
                _ => {}
            }
        }

        if !edges.is_empty() {
            debug_assert_eq!(edges.first().unwrap().distance, 0.0);
        }

        self.events = events;
        self.edges = edges;
    }

    /// Initialize the path measurements with a path.
    pub fn initialize_with_path(&mut self, path: &Path, tolerance: f32) {
        self.initialize_with_path_slice(path.as_slice(), tolerance)
    }

    /// Initialize the path measurements with a path.
    pub fn initialize_with_path_slice(&mut self, path: PathSlice, tolerance: f32) {
        self.initialize(path.id_iter(), &path, tolerance)
    }

    /// Returns the approximate length of the path.
    pub fn length(&self) -> f32 {
        if self.edges.is_empty() {
            0.0
        } else {
            self.edges.last().unwrap().distance
        }
    }

    /// Create an object that can perform fast sample queries on a path using the cached measurements.
    ///
    /// The returned sampler does not compute interpolated attributes.
    pub fn create_sampler<'l, PS: PositionStore>(
        &'l self,
        positions: &'l PS,
        ty: SampleType,
    ) -> PathSampler<'l, PS, ()> {
        let attr: &'static () = &();
        PathSampler::new(self, positions, attr, ty)
    }

    /// Create an object that can perform fast sample queries on a path using the cached measurements.
    ///
    /// The returned sampler computes interpolated attributes.
    pub fn create_sampler_with_attributes<'l, PS, AS>(
        &'l self,
        positions: &'l PS,
        attributes: &'l AS,
        ty: SampleType,
    ) -> PathSampler<'l, PS, AS>
    where
        PS: PositionStore,
        AS: AttributeStore,
    {
        PathSampler::new(self, positions, attributes, ty)
    }
}

/// Performs fast sample queries on a path with cached measurements.
///
/// This object contains the mutable state necessary for speeding up the queries, this allows the
/// path measurements to be immutable and accessible concurrently from multiple threads if needed.
///
/// Reusing a sampler over multiple queries saves a memory allocation if there are custom attributes,
/// And speeds up queries if they are sequentially ordered along the path.
pub struct PathSampler<'l, PS, AS> {
    events: &'l [IdEvent],
    edges: &'l [Edge],
    positions: &'l PS,
    attributes: &'l AS,
    attribute_buffer: Vec<f32>,
    cursor: usize,
    sample_type: SampleType,
}

impl<'l, PS: PositionStore, AS: AttributeStore> PathSampler<'l, PS, AS> {
    /// Create a sampler.
    ///
    /// The provided positions must be the ones used when initializing the path measurements.
    pub fn new(
        measurements: &'l PathMeasurements,
        positions: &'l PS,
        attributes: &'l AS,
        sample_type: SampleType,
    ) -> Self {
        PathSampler {
            events: &measurements.events,
            edges: &measurements.edges,
            positions,
            attributes,
            attribute_buffer: alloc::vec![0.0; attributes.num_attributes()],
            cursor: 0,
            sample_type,
        }
    }

    /// Sample at a given distance along the path.
    ///
    /// If the path is empty, the produced sample will contain NaNs.
    pub fn sample(&mut self, dist: f32) -> PathSample {
        self.sample_impl(dist, self.sample_type)
    }

    /// Construct a path for a specific sub-range of the measured path.
    ///
    /// The path measurements must have been initialized with the same path.
    /// The distance is clamped to the beginning and end of the path.
    /// Panics if the path is empty.
    pub fn split_range(&mut self, mut range: Range<f32>, output: &mut dyn PathBuilder) {
        let length = self.length();
        if self.sample_type == SampleType::Normalized {
            range.start *= length;
            range.end *= length;
        }
        range.start = range.start.max(0.0);
        range.end = range.end.max(range.start);
        range.start = range.start.min(length);
        range.end = range.end.min(length);

        if range.is_empty() {
            return;
        }

        let result = self.sample_impl(range.start, SampleType::Distance);
        output.begin(result.position, result.attributes);
        let (ptr1, seg1) = (self.cursor, self.edges[self.cursor].index);
        self.move_cursor(range.end);
        let (ptr2, seg2) = (self.cursor, self.edges[self.cursor].index);

        let mut is_in_subpath = true;
        if seg1 == seg2 {
            self.cursor = ptr1;
            let t_begin = self.t(range.start);
            self.cursor = ptr2;
            let t_end = self.t(range.end);
            self.add_segment(seg1, Some(t_begin..t_end), output, &mut is_in_subpath);
        } else {
            self.cursor = ptr1;
            self.add_segment(
                seg1,
                Some(self.t(range.start)..1.0),
                output,
                &mut is_in_subpath,
            );
            for seg in (seg1 + 1)..seg2 {
                self.add_segment(seg, None, output, &mut is_in_subpath);
            }
            self.cursor = ptr2;
            self.add_segment(
                seg2,
                Some(0.0..self.t(range.end)),
                output,
                &mut is_in_subpath,
            );
        }

        output.end(false);
    }

    /// Returns the approximate length of the path.
    pub fn length(&self) -> f32 {
        if self.edges.is_empty() {
            0.0
        } else {
            self.edges.last().unwrap().distance
        }
    }

    fn to_segment(&self, event: IdEvent) -> SegmentWrapper {
        match event {
            IdEvent::Line { from, to } => SegmentWrapper::Line(
                LineSegment {
                    from: self.positions.get_endpoint(from),
                    to: self.positions.get_endpoint(to),
                },
                (from, to),
            ),
            IdEvent::Quadratic { from, ctrl, to } => SegmentWrapper::Quadratic(
                QuadraticBezierSegment {
                    from: self.positions.get_endpoint(from),
                    to: self.positions.get_endpoint(to),
                    ctrl: self.positions.get_control_point(ctrl),
                },
                (from, to),
            ),
            IdEvent::Cubic {
                from,
                ctrl1,
                ctrl2,
                to,
            } => SegmentWrapper::Cubic(
                CubicBezierSegment {
                    from: self.positions.get_endpoint(from),
                    to: self.positions.get_endpoint(to),
                    ctrl1: self.positions.get_control_point(ctrl1),
                    ctrl2: self.positions.get_control_point(ctrl2),
                },
                (from, to),
            ),
            IdEvent::End {
                last,
                first,
                close: true,
            } => SegmentWrapper::Line(
                LineSegment {
                    from: self.positions.get_endpoint(last),
                    to: self.positions.get_endpoint(first),
                },
                (last, first),
            ),
            _ => SegmentWrapper::Empty,
        }
    }

    fn in_bounds(&self, dist: f32) -> bool {
        self.cursor != 0
            && self.edges[self.cursor - 1].distance <= dist
            && dist <= self.edges[self.cursor].distance
    }

    /// Move the pointer so the given point is on the current segment.
    fn move_cursor(&mut self, dist: f32) {
        if dist == 0.0 {
            self.cursor = 1;
            return;
        }
        if self.in_bounds(dist) {
            // No need to move
            return;
        }

        // Performs on [first, last)
        // ...TTFFF...
        //      ^
        //      sample this point
        fn partition_point(first: usize, last: usize, pred: impl Fn(usize) -> bool) -> usize {
            let mut l = first;
            let mut r = last;
            while l < r {
                let mid = (l + r) / 2;
                if pred(mid) {
                    l = mid + 1;
                } else {
                    r = mid;
                }
            }
            debug_assert_eq!(l, r);
            debug_assert_ne!(l, last);
            l
        }

        fn floor_log2(num: usize) -> u32 {
            core::mem::size_of::<usize>() as u32 * 8 - num.leading_zeros() - 1
        }

        // Here we use a heuristic method combining method 1 & 2
        // Method 1:        Move step by step until we found the corresponding segment, works well on short paths and near queries
        // Time complexity: (expected) (dist - start).abs() / len * num
        // Method 2.        Binary search on lengths, works well on long paths and random calls
        // Time complexity: (exact) floor(log2(num))
        // where `len` and `num` are given as follow
        //
        // According to the benchmark, this method works well in both cases and has low overhead in relative to Method 1 & 2.
        // Benchmark code: https://gist.github.com/Mivik/5f67ae5a72eae3884b2f386370554966
        let start = self.edges[self.cursor].distance;
        if start < dist {
            let (len, num) = (self.length() - start, self.edges.len() - self.cursor - 1);
            debug_assert_ne!(num, 0);
            if (dist - start) / len * (num as f32) < floor_log2(num) as f32 {
                loop {
                    self.cursor += 1;
                    if dist <= self.edges[self.cursor].distance {
                        break;
                    }
                }
            } else {
                self.cursor = partition_point(self.cursor + 1, self.edges.len(), |p| {
                    self.edges[p].distance < dist
                });
            }
        } else {
            let (len, num) = (start, self.cursor + 1);
            debug_assert_ne!(num, 0);
            if (start - dist) / len * (num as f32) < floor_log2(num) as f32 {
                loop {
                    self.cursor -= 1;
                    if self.cursor == 0 || self.edges[self.cursor - 1].distance < dist {
                        break;
                    }
                }
            } else {
                self.cursor = partition_point(0, self.cursor, |p| self.edges[p].distance < dist);
            }
        }

        debug_assert!(self.in_bounds(dist));
    }

    /// Interpolate the custom attributes.
    fn interpolate_attributes(&mut self, from: EndpointId, to: EndpointId, t: f32) {
        let from = self.attributes.get(from);
        let to = self.attributes.get(to);
        for i in 0..self.attribute_buffer.len() {
            self.attribute_buffer[i] = from[i] * (1.0 - t) + to[i] * t;
        }
    }

    /// Returns the relative position (0 ~ 1) of the given point on the current segment.
    fn t(&self, dist: f32) -> f32 {
        debug_assert!(self.in_bounds(dist));
        let prev = &self.edges[self.cursor - 1];
        let cur = &self.edges[self.cursor];
        let t_begin = if prev.index == cur.index { prev.t } else { 0.0 };
        let t_end = cur.t;
        t_begin + (t_end - t_begin) * ((dist - prev.distance) / (cur.distance - prev.distance))
    }

    fn sample_impl(&mut self, mut dist: f32, sample_type: SampleType) -> PathSample {
        let length = self.length();
        if length == 0.0 {
            return self.sample_zero_length();
        }
        if sample_type == SampleType::Normalized {
            dist *= length;
        }
        dist = dist.max(0.0);
        dist = dist.min(length);

        self.move_cursor(dist);
        let t = self.t(dist);
        macro_rules! dispatched_call {
            ([$v:expr] ($seg:ident, $pair:ident) => $code:block) => {
                #[allow(unused_parens)]
                match $v {
                    SegmentWrapper::Line($seg, $pair) => $code,
                    SegmentWrapper::Quadratic($seg, $pair) => $code,
                    SegmentWrapper::Cubic($seg, $pair) => $code,
                    _ => {}
                }
            };
        }

        dispatched_call!([self.to_segment(self.events[self.edges[self.cursor].index])] (segment, pair) => {
            self.interpolate_attributes(pair.0, pair.1, t);
            return PathSample {
                position: segment.sample(t),
                tangent: segment.derivative(t).normalize(),
                attributes: &self.attribute_buffer,
            }
        });

        unreachable!();
    }

    #[cold]
    fn sample_zero_length(&mut self) -> PathSample {
        if let Some(IdEvent::Begin { at }) = self.events.first() {
            return PathSample {
                position: self.positions.get_endpoint(*at),
                tangent: vector(0.0, 0.0),
                attributes: self.attributes.get(*at),
            };
        }

        for value in &mut self.attribute_buffer {
            *value = f32::NAN;
        }

        PathSample {
            position: point(f32::NAN, f32::NAN),
            tangent: vector(f32::NAN, f32::NAN),
            attributes: &self.attribute_buffer,
        }
    }

    /// Caller needs to hold a parameter to keep track of whether we're in a subpath or not, as this would be determined
    /// by prior segments. This function will update `is_in_subpath` based on the segment it adds.
    fn add_segment(
        &mut self,
        ptr: usize,
        range: Option<Range<f32>>,
        dest: &mut dyn PathBuilder,
        is_in_subpath: &mut bool,
    ) {
        let segment = self.to_segment(self.events[ptr]);
        let segment = match range.clone() {
            Some(range) => segment.split(range),
            None => segment,
        };
        macro_rules! obtain_attrs {
            ($p:ident, $index:tt) => {
                match range.clone() {
                    Some(range) => {
                        if range.end == 1.0 {
                            self.attributes.get($p.$index)
                        } else {
                            self.interpolate_attributes($p.0, $p.1, range.end);
                            &mut self.attribute_buffer
                        }
                    }
                    None => self.attributes.get($p.$index),
                }
            };
        }

        match segment {
            SegmentWrapper::Line(LineSegment { from, to }, pair) => {
                if !*is_in_subpath {
                    dest.end(false);
                    dest.begin(from, obtain_attrs!(pair, 0));
                }
                dest.line_to(to, obtain_attrs!(pair, 1));
            }
            SegmentWrapper::Quadratic(QuadraticBezierSegment { from, ctrl, to }, pair) => {
                if !*is_in_subpath {
                    dest.end(false);
                    dest.begin(from, obtain_attrs!(pair, 0));
                }
                dest.quadratic_bezier_to(ctrl, to, obtain_attrs!(pair, 1));
            }
            SegmentWrapper::Cubic(
                CubicBezierSegment {
                    from,
                    ctrl1,
                    ctrl2,
                    to,
                },
                pair,
            ) => {
                if !*is_in_subpath {
                    dest.end(false);
                    dest.begin(from, obtain_attrs!(pair, 0));
                }
                dest.cubic_bezier_to(ctrl1, ctrl2, to, obtain_attrs!(pair, 1));
            }
            _ => {}
        }

        *is_in_subpath = !matches!(
            self.events[ptr],
            IdEvent::End { .. } | IdEvent::Begin { .. }
        );
    }
}

#[cfg(test)]
fn slice(a: &[f32]) -> &[f32] {
    a
}

#[test]
fn measure_line() {
    let mut path = Path::builder();
    path.begin(point(1.0, 1.0));
    path.line_to(point(0.0, 0.0));
    path.end(false);
    let path = path.build();
    let measure = PathMeasurements::from_path(&path, 0.01);
    let mut sampler = measure.create_sampler(&path, SampleType::Normalized);
    for t in [0.0, 0.2, 0.3, 0.5, 1.0] {
        let result = sampler.sample(t);
        assert!((result.position - point(1.0 - t, 1.0 - t)).length() < 1e-5);
        assert_eq!(result.tangent, vector(-1.0, -1.0).normalize());
    }
}

#[test]
fn measure_square() {
    let mut path = Path::builder();
    path.begin(point(0.0, 0.0));
    path.line_to(point(1.0, 0.0));
    path.line_to(point(1.0, 1.0));
    path.line_to(point(0.0, 1.0));
    path.close();
    let path = path.build();
    let measure = PathMeasurements::from_path(&path, 0.01);
    let mut sampler = measure.create_sampler(&path, SampleType::Normalized);
    for (t, position, tangent) in [
        (0.125, point(0.5, 0.0), vector(1.0, 0.0)),
        (0.375, point(1.0, 0.5), vector(0.0, 1.0)),
        (0.625, point(0.5, 1.0), vector(-1.0, 0.0)),
        (0.875, point(0.0, 0.5), vector(0.0, -1.0)),
    ] {
        let result = sampler.sample(t);
        assert!((result.position - position).length() < 1e-5);
        assert_eq!(result.tangent, tangent);
    }
}

#[test]
fn measure_attributes() {
    let mut path = Path::builder_with_attributes(2);
    path.begin(point(0.0, 0.0), &[1.0, 2.0]);
    path.line_to(point(1.0, 0.0), &[2.0, 3.0]);
    path.line_to(point(1.0, 1.0), &[0.0, 0.0]);
    path.end(false);
    let path = path.build();
    let measure = PathMeasurements::from_path(&path, 0.01);
    let mut sampler = measure.create_sampler_with_attributes(&path, &path, SampleType::Normalized);

    for (t, position, attrs) in [
        (0.25, point(0.5, 0.0), &[1.5, 2.5]),
        (0.5, point(1.0, 0.0), &[2.0, 3.0]),
        (0.75, point(1.0, 0.5), &[1.0, 1.5]),
    ] {
        let result = sampler.sample(t);
        assert!((result.position - position).length() < 1e-5);
        for i in 0..2 {
            assert!((result.attributes[i] - attrs[i]).abs() < 1e-5);
        }
    }
}

#[test]
fn measure_bezier_curve() {
    let mut path = Path::builder();
    path.begin(point(0.0, 0.0));
    path.quadratic_bezier_to(point(0.5, 0.7), point(1.0, 0.0));
    path.quadratic_bezier_to(point(1.5, -0.7), point(2.0, 0.0));
    path.end(false);
    let path = path.build();

    let measure = PathMeasurements::from_path(&path, 0.01);
    let mut sampler = measure.create_sampler(&path, SampleType::Normalized);

    for t in [0.25, 0.75] {
        let result = sampler.sample(t);
        assert_eq!(result.tangent, vector(1.0, 0.0));
    }
    for (t, position) in [
        (0.0, point(0.0, 0.0)),
        (0.5, point(1.0, 0.0)),
        (1.0, point(2.0, 0.0)),
    ] {
        let result = sampler.sample(t);
        assert_eq!(result.position, position);
    }
}

#[test]
fn split_square() {
    use crate::path::Event;

    let mut path = Path::builder();
    path.begin(point(0.0, 0.0));
    path.line_to(point(1.0, 0.0));
    path.line_to(point(1.0, 1.0));
    path.line_to(point(0.0, 1.0));
    path.close();
    let path = path.build();
    let measure = PathMeasurements::from_path(&path, 0.01);
    let mut sampler = measure.create_sampler(&path, SampleType::Normalized);
    let mut path2 = Path::builder();
    sampler.split_range(0.125..0.625, &mut path2);
    let path2 = path2.build();
    assert_eq!(
        path2.iter().collect::<Vec<_>>(),
        alloc::vec![
            Event::Begin {
                at: point(0.5, 0.0)
            },
            Event::Line {
                from: point(0.5, 0.0),
                to: point(1.0, 0.0)
            },
            Event::Line {
                from: point(1.0, 0.0),
                to: point(1.0, 1.0)
            },
            Event::Line {
                from: point(1.0, 1.0),
                to: point(0.5, 1.0)
            },
            Event::End {
                last: point(0.5, 1.0),
                first: point(0.5, 0.0),
                close: false
            },
        ]
    );
}

#[test]
fn split_bezier_curve() {
    use crate::path::Event;

    let mut path = Path::builder();
    path.begin(point(0.0, 0.0));
    path.quadratic_bezier_to(point(1.0, 1.0), point(2.0, 0.0));
    path.end(false);
    let path = path.build();
    let measure = PathMeasurements::from_path(&path, 0.01);
    let mut sampler = measure.create_sampler(&path, SampleType::Normalized);

    let mut path2 = Path::builder();
    sampler.split_range(0.5..1.0, &mut path2);
    let path2 = path2.build();

    assert_eq!(
        path2.iter().collect::<Vec<_>>(),
        alloc::vec![
            Event::Begin {
                at: point(1.0, 0.5)
            },
            Event::Quadratic {
                from: point(1.0, 0.5),
                ctrl: point(1.5, 0.5),
                to: point(2.0, 0.0),
            },
            Event::End {
                last: point(2.0, 0.0),
                first: point(1.0, 0.5),
                close: false
            }
        ]
    );
}

#[test]
fn split_attributes() {
    use crate::path::Event;

    let mut path = Path::builder_with_attributes(2);
    path.begin(point(0.0, 0.0), &[1.0, 2.0]);
    path.line_to(point(1.0, 0.0), &[2.0, 3.0]);
    path.line_to(point(1.0, 1.0), &[0.0, 0.0]);
    path.end(false);
    let path = path.build();
    let measure = PathMeasurements::from_path(&path, 0.01);
    let mut sampler = measure.create_sampler_with_attributes(&path, &path, SampleType::Normalized);

    let mut path2 = Path::builder_with_attributes(2);
    sampler.split_range(0.0..1.0, &mut path2);
    let path2 = path2.build();

    assert_eq!(
        path2.iter_with_attributes().collect::<Vec<_>>(),
        path.iter_with_attributes().collect::<Vec<_>>()
    );

    let mut path3 = Path::builder_with_attributes(2);
    sampler.split_range(0.25..0.75, &mut path3);
    let path3 = path3.build();

    assert_eq!(
        path3.iter_with_attributes().collect::<Vec<_>>(),
        alloc::vec![
            Event::Begin {
                at: (point(0.5, 0.0), slice(&[1.5, 2.5]))
            },
            Event::Line {
                from: (point(0.5, 0.0), slice(&[1.5, 2.5])),
                to: (point(1.0, 0.0), slice(&[2.0, 3.0])),
            },
            Event::Line {
                from: (point(1.0, 0.0), slice(&[2.0, 3.0])),
                to: (point(1.0, 0.5), slice(&[1.0, 1.5])),
            },
            Event::End {
                last: (point(1.0, 0.5), slice(&[1.0, 1.5])),
                first: (point(0.5, 0.0), slice(&[1.5, 2.5])),
                close: false
            }
        ]
    );
}

#[test]
fn zero_length() {
    fn expect_nans(sample: PathSample, num_attribs: usize) {
        assert!(sample.position.x.is_nan());
        assert!(sample.position.y.is_nan());
        assert!(sample.tangent.x.is_nan());
        assert!(sample.tangent.y.is_nan());
        for attr in sample.attributes {
            assert!(attr.is_nan());
        }
        assert_eq!(sample.attributes.len(), num_attribs);
    }

    let mut path = Path::builder_with_attributes(2);
    path.begin(point(1.0, 2.0), &[3.0, 4.0]);
    path.end(false);
    let path = path.build();
    let measure = PathMeasurements::from_path(&path, 0.01);
    let mut sampler = measure.create_sampler_with_attributes(&path, &path, SampleType::Normalized);
    let expected = PathSample {
        position: point(1.0, 2.0),
        tangent: vector(0.0, 0.0),
        attributes: &[3.0, 4.0],
    };
    assert_eq!(sampler.sample(0.0), expected);
    assert_eq!(sampler.sample(0.5), expected);
    assert_eq!(sampler.sample(1.0), expected);

    let mut path = Path::builder_with_attributes(2);
    path.begin(point(1.0, 2.0), &[3.0, 4.0]);
    path.end(false);
    let path = path.build();
    let measure = PathMeasurements::from_path(&path, 0.01);
    let mut sampler = measure.create_sampler_with_attributes(&path, &path, SampleType::Distance);
    let expected = PathSample {
        position: point(1.0, 2.0),
        tangent: vector(0.0, 0.0),
        attributes: &[3.0, 4.0],
    };
    assert_eq!(sampler.sample(0.0), expected);
    assert_eq!(sampler.sample(0.5), expected);
    assert_eq!(sampler.sample(1.0), expected);

    let path = Path::builder_with_attributes(2).build();
    let measure = PathMeasurements::from_path(&path, 0.01);
    let mut sampler = measure.create_sampler_with_attributes(&path, &path, SampleType::Normalized);
    expect_nans(sampler.sample(0.0), 2);
    expect_nans(sampler.sample(0.5), 2);
    expect_nans(sampler.sample(1.0), 2);

    let path = Path::builder_with_attributes(2).build();
    let measure = PathMeasurements::from_path(&path, 0.01);
    let mut sampler = measure.create_sampler_with_attributes(&path, &path, SampleType::Distance);
    expect_nans(sampler.sample(0.0), 2);
    expect_nans(sampler.sample(0.5), 2);
    expect_nans(sampler.sample(1.0), 2);
}


#[test]
fn multiple_sub_paths() {
    let mut path = Path::builder();

    path.begin(point(0.0, 0.0));
    path.line_to(point(10.0, 0.0));
    path.end(false);

    path.begin(point(10.0, 10.0));
    path.line_to(point(20.0, 10.0));
    path.end(false);

    let path = path.build();
    let measure = PathMeasurements::from_path(&path, 0.01);
    let mut sampler = measure.create_sampler(&path, SampleType::Normalized);

    let mut dashes = Path::builder();
    sampler.split_range(0.0 .. 0.25, &mut dashes);
    sampler.split_range(0.25 .. 0.5, &mut dashes);
    // Avoid starting subpaths exactly on the join as we may begin with a zero-length subpath
    sampler.split_range(0.6 .. 0.75, &mut dashes);
    sampler.split_range(0.75 .. 1.0, &mut dashes);
    let dashes = dashes.build();

    let mut iter = dashes.iter();

    use crate::path::geom::euclid::approxeq::ApproxEq;
    fn expect_begin(event: Option<path::PathEvent>, pos: Point) {
        std::eprintln!("- {:?}", event);
        if let Some(path::PathEvent::Begin { at }) = event {
            assert!(at.approx_eq(&pos), "Expected Begin {:?}, got {:?}", pos, at);
        } else {
            panic!("Expected begin, got {:?}", event);
        }
    }

    fn expect_end(event: Option<path::PathEvent>, pos: Point) {
        std::eprintln!("- {:?}", event);
        if let Some(path::PathEvent::End { last, .. }) = event {
            assert!(last.approx_eq(&pos), "Expected End {:?}, got {:?}", pos, last);
        } else {
            panic!("Expected end, got {:?}", event);
        }
    }
    fn expect_line(event: Option<path::PathEvent>, expect_from: Point, expect_to: Point) {
        std::eprintln!("- {:?}", event);
        if let Some(path::PathEvent::Line { from, to }) = event {
            assert!(from.approx_eq(&expect_from), "Expected line {:?} {:?}, got {:?} {:?}", expect_from, expect_to, from, to);
            assert!(to.approx_eq(&expect_to), "Expected line {:?} {:?}, got {:?} {:?}", expect_from, expect_to, from, to);
        } else {
            panic!("Expected a line {:?} {:?}, got {:?}", expect_from, expect_to, event);
        }
    }

    expect_begin(iter.next(), point(0.0, 0.0));
    expect_line(iter.next(), point(0.0, 0.0), point(5.0, 0.0));
    expect_end(iter.next(), point(5.0, 0.0));

    expect_begin(iter.next(), point(5.0, 0.0));
    expect_line(iter.next(), point(5.0, 0.0), point(10.0, 0.0));
    expect_end(iter.next(), point(10.0, 0.0));

    expect_begin(iter.next(), point(12.0, 10.0));
    expect_line(iter.next(), point(12.0, 10.0), point(15.0, 10.0));
    expect_end(iter.next(), point(15.0, 10.0));

    expect_begin(iter.next(), point(15.0, 10.0));
    expect_line(iter.next(), point(15.0, 10.0), point(20.0, 10.0));
    expect_end(iter.next(), point(20.0, 10.0));
}
