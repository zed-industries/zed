//! Stroking and dashing of paths.

#![allow(clippy::needless_lifetimes)]

use super::command::Command;
use super::geometry::*;
use super::path_builder::*;
use super::segment::*;
use super::style::*;
#[allow(unused)]
use super::F32Ext;

use crate::lib::Vec;
use core::borrow::Borrow;

pub fn stroke_into<'a, I>(commands: I, style: &Stroke<'a>, sink: &mut impl PathBuilder)
where
    I: Iterator + Clone,
    I::Item: Borrow<Command>,
{
    let mut stroker = Stroker::new(segments(commands, true), sink, style);
    let (dashes, dash_offset, empty_gaps) = validate_dashes(style.dashes, style.offset);
    let mut segment_buf = SmallBuf::new();
    if !dashes.is_empty() {
        stroker.dash(&mut segment_buf, dashes, dash_offset, empty_gaps);
    } else {
        stroker.stroke(&mut segment_buf);
    }
}

pub fn stroke_with_storage<'a, I>(
    commands: I,
    style: &Stroke<'a>,
    sink: &mut impl PathBuilder,
    storage: &mut impl StrokerStorage,
) where
    I: Iterator + Clone,
    I::Item: Borrow<Command>,
{
    let mut stroker = Stroker::new(segments(commands, true), sink, style);
    let (dashes, dash_offset, empty_gaps) = validate_dashes(style.dashes, style.offset);
    if !dashes.is_empty() {
        stroker.dash(storage, dashes, dash_offset, empty_gaps);
    } else {
        stroker.stroke(storage);
    }
}

pub struct Stroker<'a, I, S> {
    source: Segments<I>,
    sink: &'a mut S,
    radius: f32,
    radius_abs: f32,
    join: Join,
    inv_miter_limit: f32,
    start_cap: Cap,
    end_cap: Cap,
}

impl<'a, I, S> Stroker<'a, I, S>
where
    I: Iterator + Clone,
    I::Item: Borrow<Command>,
    S: PathBuilder,
{
    pub(super) fn new(source: Segments<I>, sink: &'a mut S, style: &Stroke) -> Self {
        let radius = style.width.max(0.01) * 0.5;
        Self {
            source,
            sink,
            radius,
            radius_abs: radius.abs(),
            join: style.join,
            inv_miter_limit: if style.miter_limit >= 1. {
                1. / style.miter_limit
            } else {
                1.
            },
            start_cap: style.start_cap,
            end_cap: style.end_cap,
        }
    }

    fn stroke(&mut self, segment_buf: &mut impl StrokerStorage) {
        loop {
            let (closed, done) = segment_buf.collect(&mut self.source);
            self.stroke_segments(segment_buf.get(), closed);
            if done {
                break;
            }
        }
    }

    fn stroke_segments(&mut self, segments: &[Segment], is_closed: bool) {
        let len = segments.len();
        if len == 0 {
            return;
        }
        if len == 1
            && segments[0].length() == 0.
            && (self.start_cap != Cap::Butt || self.end_cap != Cap::Butt)
        {
            let segment = segments[0];
            let from = match &segment {
                Segment::Line(_, line) => line.a,
                Segment::Curve(_, curve) => curve.a,
                Segment::End(..) => Point::ZERO,
            };
            let n = Vector::new(0., 1.);
            let nr = n * self.radius;
            let start = from + nr;
            let rstart = from - nr;
            self.sink.move_to(start);
            self.add_end_cap(start, rstart, n);
            self.add_start_cap(rstart, start, n * -1.);
            return;
        }
        let radius = self.radius;
        let mut last_dir = Vector::ZERO;
        let mut first_point = Point::ZERO;
        let mut last_point = Point::ZERO;
        let mut pivot = Point::ZERO;
        let mut last_id = 0xFF;
        if is_closed {
            let segment = segments[len - 1].offset(radius);
            let end_point = segment.end;
            let out_dir = segment.end_normal;
            pivot = segment.end_pivot;
            last_dir = out_dir;
            last_point = end_point;
            first_point = end_point;
            self.sink.move_to(last_point);
        }
        // Forward for the outer stroke.
        let mut is_first = !is_closed;
        for segment in segments {
            let segment = segment.offset(radius);
            let id = segment.id;
            let start = segment.start;
            if is_first {
                self.sink.move_to(start);
                first_point = start;
                is_first = false;
            } else {
                self.add_join(last_point, start, pivot, last_dir, segment.start_normal);
            }
            last_id = id;
            last_dir = segment.end_normal;
            pivot = segment.end_pivot;
            last_point = self.emit(&segment.segment);
        }
        // Now backward for the inner stroke.
        is_first = true;
        for segment in segments.iter().rev() {
            let segment = segment.reverse().offset(radius);
            let id = segment.id;
            let start = segment.start;
            if is_first {
                if is_closed {
                    let init = segments[0].reverse().offset(self.radius);
                    last_point = init.end;
                    last_dir = init.end_normal;
                    pivot = init.end_pivot;
                    self.sink.line_to(init.end);
                    self.add_join(last_point, start, pivot, last_dir, segment.start_normal);
                } else {
                    self.add_end_cap(last_point, start, last_dir);
                }
                is_first = false;
            } else if id != last_id {
                self.add_join(last_point, start, pivot, last_dir, segment.start_normal);
            } else {
                self.add_split_join(last_point, start, pivot, last_dir, segment.start_normal);
            };
            last_id = id;
            last_dir = segment.end_normal;
            pivot = segment.end_pivot;
            last_point = self.emit(&segment.segment);
        }
        if !is_closed {
            self.add_start_cap(last_point, first_point, last_dir);
        }
        self.sink.close();
    }

    #[allow(clippy::field_reassign_with_default)]
    fn dash(
        &mut self,
        segment_buf: &mut impl StrokerStorage,
        dashes: &[f32],
        offset: f32,
        empty_gaps: bool,
    ) {
        let mut dasher = Dasher::default();
        dasher.empty_gaps = empty_gaps;
        let mut done = false;
        while !done {
            let (is_closed, is_done) = segment_buf.collect(&mut self.source);
            done = is_done;
            let segments = segment_buf.get();
            if segments.is_empty() {
                continue;
            }
            dasher.init(is_closed, dashes, offset);
            loop {
                match dasher.next(segments, dashes) {
                    DashOp::Done => break,
                    DashOp::Continue => {}
                    DashOp::Emit => {
                        let (start, end) = dasher.range;
                        let (t0, t1) = dasher.trange;
                        self.dash_segments(segments, start, end, t0, t1);
                    }
                    DashOp::Stroke => {
                        self.stroke_segments(segments, true);
                        break;
                    }
                }
            }
        }
    }

    fn dash_segments(&mut self, segments: &[Segment], start: isize, end: isize, t0: f32, t1: f32) {
        let radius = self.radius;
        if t0 == t1 && start == end {
            if self.start_cap == Cap::Butt && self.end_cap == Cap::Butt {
                return;
            }
            let (t0, t1) = if t0 >= 1. {
                (t0 - 0.001, t0)
            } else {
                (t0, t0 + 0.001)
            };
            let segment = get_signed(segments, start).slice(t0, t1).offset(radius);
            let start = segment.start;
            let rstart = segment.start - (segment.start_normal * (2. * radius));
            self.sink.move_to(start);
            self.add_end_cap(start, rstart, segment.start_normal);
            self.add_start_cap(rstart, start, segment.start_normal * -1.);
            self.sink.close();
            return;
        }
        let mut last_dir = Vector::ZERO;
        let mut first_point = Point::ZERO;
        let mut last_point = Point::ZERO;
        let mut pivot = Point::ZERO;
        let mut is_first = true;
        let mut last_id = 0xFF;
        for i in start..=end {
            let t0 = if i == start { t0 } else { 0. };
            let t1 = if i == end { t1 } else { 1. };
            if t0 >= 1. {
                continue;
            }
            let segment = get_signed(segments, i).slice(t0, t1).offset(radius);
            let id = segment.id;
            let start = segment.start;
            if is_first {
                self.sink.move_to(start);
                first_point = start;
                is_first = false;
            } else if id != last_id {
                self.add_join(last_point, start, pivot, last_dir, segment.start_normal);
            } else {
                self.add_split_join(last_point, start, pivot, last_dir, segment.start_normal);
            };
            last_id = id;
            pivot = segment.end_pivot;
            last_dir = segment.end_normal;
            last_point = self.emit(&segment.segment);
        }
        is_first = true;
        last_id = 0xFF;
        for i in (start..=end).rev() {
            let t0 = if i == start { t0 } else { 0. };
            let t1 = if i == end { t1 } else { 1. };
            if t0 >= 1. {
                continue;
            }
            let segment = get_signed(segments, i)
                .slice(t0, t1)
                .reverse()
                .offset(radius);
            let id = segment.id;
            let start = segment.start;
            if is_first {
                self.add_end_cap(last_point, start, last_dir);
                is_first = false;
            } else if id != last_id {
                self.add_join(last_point, start, pivot, last_dir, segment.start_normal);
            } else {
                self.add_split_join(last_point, start, pivot, last_dir, segment.start_normal);
            };
            last_id = id;
            pivot = segment.end_pivot;
            last_dir = segment.end_normal;
            last_point = self.emit(&segment.segment);
        }
        self.add_start_cap(last_point, first_point, last_dir);
        self.sink.close();
    }

    #[inline(always)]
    fn emit(&mut self, segment: &Segment) -> Point {
        match segment {
            Segment::Line(_, line) => {
                self.sink.line_to(line.b);
                line.b
            }
            Segment::Curve(_, curve) => {
                self.sink.curve_to(curve.b, curve.c, curve.d);
                curve.d
            }
            _ => Point::ZERO,
        }
    }

    fn add_join(
        &mut self,
        from: Point,
        to: Point,
        pivot: Point,
        from_normal: Vector,
        to_normal: Vector,
    ) -> Point {
        if from.nearly_eq(to) {
            return from;
        }
        if !is_clockwise(from_normal, to_normal) {
            self.sink.line_to(pivot);
            self.sink.line_to(to);
            return to;
        }
        match self.join {
            Join::Bevel => {
                self.sink.line_to(to);
                to
            }
            Join::Round => {
                let r = self.radius_abs;
                let (size, sweep) = (ArcSize::Small, ArcSweep::Positive);
                arc(self.sink, from, r, r, 0., size, sweep, to);
                to
            }
            Join::Miter => {
                let inv_limit = self.inv_miter_limit;
                let dot = from_normal.dot(to_normal);
                let sin_half = ((1. + dot) * 0.5).sqrt();
                if dot < 0.0 || sin_half < inv_limit {
                    self.sink.line_to(to);
                    to
                } else {
                    let mid = (from_normal + to_normal).normalize() * (self.radius / sin_half);
                    let p = pivot + mid;
                    self.sink.line_to(p);
                    self.sink.line_to(to);
                    to
                }
            }
        }
    }

    fn add_split_join(
        &mut self,
        from: Point,
        to: Point,
        pivot: Point,
        from_normal: Vector,
        to_normal: Vector,
    ) -> Point {
        if from.nearly_eq(to) {
            return from;
        }
        if !is_clockwise(from_normal, to_normal) {
            self.sink.line_to(pivot);
            self.sink.line_to(to);
            return to;
        }
        let r = self.radius_abs;
        let (size, sweep) = (ArcSize::Small, ArcSweep::Positive);
        arc(self.sink, from, r, r, 0., size, sweep, to);
        to
    }

    fn add_cap(&mut self, from: Point, to: Point, dir: Vector, cap: Cap) {
        match cap {
            Cap::Butt => {
                self.sink.line_to(to);
            }
            Cap::Square => {
                let dir = Vector::new(-dir.y, dir.x);
                self.sink.line_to(from + dir * self.radius_abs);
                self.sink.line_to(to + dir * self.radius_abs);
                self.sink.line_to(to);
            }
            Cap::Round => {
                let r = self.radius_abs;
                let (size, sweep) = (ArcSize::Small, ArcSweep::Positive);
                arc(self.sink, from, r, r, 0., size, sweep, to);
            }
        }
    }

    fn add_start_cap(&mut self, from: Point, to: Point, dir: Vector) {
        self.add_cap(from, to, dir, self.start_cap);
    }

    fn add_end_cap(&mut self, from: Point, to: Point, dir: Vector) {
        self.add_cap(from, to, dir, self.end_cap);
    }
}

enum DashOp {
    Done,
    Continue,
    Emit,
    Stroke,
}

#[derive(Copy, Clone, Default)]
struct Dasher {
    done: bool,
    is_closed: bool,
    empty_gaps: bool,
    on: bool,
    cur: isize,
    t0: f32,
    t0_offset: f32,
    index: usize,
    is_first: bool,
    first_on: bool,
    first_dash: f32,
    is_dot: bool,
    range: (isize, isize),
    trange: (f32, f32),
}

impl Dasher {
    fn init(&mut self, is_closed: bool, dashes: &[f32], offset: f32) {
        self.done = false;
        self.is_closed = is_closed;
        self.on = true;
        self.cur = 0;
        self.t0 = 0.;
        self.t0_offset = 0.;
        self.index = 0;
        self.is_first = true;
        self.first_on = true;
        let mut first_dash = self.next_dash(dashes);
        if offset > 0. {
            let mut accum = first_dash;
            while accum < offset {
                self.on = !self.on;
                accum += self.next_dash(dashes);
            }
            self.first_on = self.on;
            first_dash = accum - offset;
        }
        self.first_dash = first_dash;
    }

    #[inline(always)]
    fn next_dash(&mut self, dashes: &[f32]) -> f32 {
        let len = dashes.len();
        let mut dash = dashes[self.index % len];
        if self.on && self.empty_gaps {
            loop {
                let next_dash = dashes[(self.index + 1) % len];
                if next_dash != 0. {
                    break;
                }
                self.index += 2;
                dash += dashes[self.index % len];
            }
        }
        self.index += 1;
        dash
    }

    #[inline(always)]
    fn next_segments(
        dash: f32,
        segments: &[Segment],
        limit: isize,
        start: isize,
        start_offset: f32,
    ) -> (bool, isize, f32, f32) {
        let mut cur = start;
        let mut goal = dash + start_offset;
        let mut segment = get_signed(segments, cur);
        loop {
            let td = segment.time(goal, 1.);
            let dist = td.distance;
            let t2 = td.time;
            goal -= dist;
            if goal <= 0. {
                return (true, cur, dist, t2);
            }
            if cur + 1 >= limit {
                return (false, cur, dist, t2);
            }
            cur += 1;
            segment = get_signed(segments, cur);
        }
    }

    #[inline(always)]
    fn next(&mut self, segments: &[Segment], dashes: &[f32]) -> DashOp {
        if self.done {
            return DashOp::Done;
        }
        let first = self.is_first;
        let first_and_closed = first && self.is_closed;
        let mut dash = if first {
            self.first_dash
        } else {
            self.next_dash(dashes)
        };
        let mut on = self.on;
        let mut start = self.cur;
        let limit = segments.len() as isize;
        if self.t0 == 1. && start < limit - 1 {
            start += 1;
            self.t0 = 0.;
            self.t0_offset = 0.;
            self.cur = start;
        }
        let (cont, mut end, mut t1_offset, mut t1) = if dash == 0. {
            (true, start, self.t0_offset, self.t0)
        } else {
            Self::next_segments(dash, segments, limit, start, self.t0_offset)
        };
        if !cont {
            self.done = true;
        }
        // This is tricky. If the subpath is closed and the last dash is
        // "on" we need to join the last dash to the first. Otherwise, we
        // need to go back and produce the initial dash that was skipped
        // in anticipation of joining to the final dash.
        if self.done && self.is_closed {
            if on {
                // Recompute the final dash including the first.
                if first_and_closed {
                    // The first dash consumed the whole path: emit a single stroke.
                    return DashOp::Stroke;
                }
                if self.first_on {
                    self.cur = start - limit;
                    start = self.cur;
                    let (_, end2, end_offset, end_t) =
                        Self::next_segments(self.first_dash, segments, limit, 0, 0.);
                    end = end2;
                    t1_offset = end_offset;
                    t1 = end_t;
                }
            } else {
                // Emit the first dash.
                if !self.first_on {
                    return DashOp::Done;
                }
                dash = self.first_dash;
                self.cur = 0;
                self.t0 = 0.;
                self.t0_offset = 0.;
                self.on = true;
                on = true;
                start = self.cur;
                let (_, end2, end_offset, end_t) =
                    Self::next_segments(self.first_dash, segments, limit, 0, 0.);
                end = end2;
                t1_offset = end_offset;
                t1 = end_t;
            }
        } else if self.done && !on {
            return DashOp::Done;
        }
        self.is_dot = dash == 0.;
        let t0 = self.t0;

        self.is_first = false;
        self.cur = end;
        self.t0 = t1;
        self.t0_offset = t1_offset;
        self.on = !self.on;
        if on && !first_and_closed {
            self.range = (start, end);
            self.trange = (t0, t1);
            return DashOp::Emit;
        }
        DashOp::Continue
    }
}

fn validate_dashes(dashes: &[f32], offset: f32) -> (&[f32], f32, bool) {
    let len = dashes.len();
    if len > 0 {
        // Generate a full stroke under any of the following conditions:
        // 1. The array contains any negative values.
        // 2. All dashes are less than 1 unit.
        // 3. All gap dashes are less than 1 unit.
        let mut small_count = 0;
        let mut gap_sum = 0.;
        let mut empty_gaps = false;
        let is_odd = len & 1 != 0;
        for (i, dash) in dashes.iter().enumerate() {
            let is_gap = i & 1 == 1;
            if *dash < 1. {
                small_count += 1;
                if *dash < 0. {
                    return (&[], 0., false);
                } else if *dash == 0. && (is_gap || is_odd) {
                    empty_gaps = true;
                }
            } else if is_gap {
                gap_sum += *dash;
            }
        }
        if dashes.len() == 1 {
            gap_sum = 1.;
        }
        if small_count < dashes.len() && gap_sum > 0. {
            let offset = if offset != 0. {
                let mut s: f32 = dashes.iter().sum();
                if is_odd {
                    s *= 2.;
                }
                if offset < 0. {
                    s - (offset.abs() % s)
                } else {
                    offset % s
                }
            } else {
                0.
            };
            return (dashes, offset, empty_gaps);
        }
    }
    (&[], 0., false)
}

#[inline(always)]
fn get_signed(segments: &[Segment], index: isize) -> Segment {
    let index = if index < 0 {
        segments.len() - (-index) as usize
    } else {
        index as usize
    };
    segments[index]
}

fn is_clockwise(a: Vector, b: Vector) -> bool {
    a.x * b.y > a.y * b.x
}

impl Segment {
    fn offset(&self, radius: f32) -> OffsetSegment {
        OffsetSegment::new(self, radius)
    }
}

#[derive(Copy, Clone)]
pub struct OffsetSegment {
    pub segment: Segment,
    pub id: u8,
    pub start: Point,
    pub end: Point,
    pub start_normal: Vector,
    pub end_normal: Vector,
    pub end_pivot: Point,
}

impl OffsetSegment {
    fn new(segment: &Segment, radius: f32) -> Self {
        match segment {
            Segment::Line(id, Line { a, b }) => {
                let n = normal(*a, *b);
                let nr = n * radius;
                let start = *a + nr;
                let end = *b + nr;
                Self {
                    segment: Segment::Line(*id, Line { a: start, b: end }),
                    id: *id,
                    start,
                    end,
                    start_normal: n,
                    end_normal: n,
                    end_pivot: *b,
                }
            }
            Segment::Curve(id, c) => {
                const EPS: f32 = 0.5;
                //const EPS: f32 = CURVE_EPSILON;
                let normal_ab = if c.a.nearly_eq_by(c.b, EPS) {
                    if c.a.nearly_eq_by(c.c, EPS) {
                        normal(c.a, c.d)
                    } else {
                        normal(c.a, c.c)
                    }
                } else {
                    normal(c.a, c.b)
                };
                let normal_bc = if c.b.nearly_eq_by(c.c, EPS) {
                    if c.b.nearly_eq_by(c.d, EPS) {
                        normal(c.a, c.d)
                    } else {
                        normal(c.b, c.d)
                    }
                } else {
                    normal(c.b, c.c)
                };
                let normal_cd = if c.c.nearly_eq_by(c.d, EPS) {
                    if c.b.nearly_eq_by(c.d, EPS) {
                        normal(c.a, c.d)
                    } else {
                        normal(c.b, c.d)
                    }
                } else {
                    normal(c.c, c.d)
                };
                let mut normal_b = normal_ab + normal_bc;
                let mut normal_c = normal_cd + normal_bc;
                let dot = normal_ab.dot(normal_bc);
                normal_b = normal_b.normalize() * (radius / ((1. + dot) * 0.5).sqrt());
                let dot = normal_cd.dot(normal_bc);
                normal_c = normal_c.normalize() * (radius / ((1. + dot) * 0.5).sqrt());
                let start = c.a + normal_ab * radius;
                let end = c.d + normal_cd * radius;
                Self {
                    segment: Segment::Curve(
                        *id,
                        Curve::new(start, c.b + normal_b, c.c + normal_c, end),
                    ),
                    id: *id,
                    start,
                    end,
                    start_normal: normal_ab,
                    end_normal: normal_cd,
                    end_pivot: c.d,
                }
            }
            Segment::End(..) => Self {
                segment: *segment,
                id: 0,
                start: Point::ZERO,
                end: Point::ZERO,
                start_normal: Vector::ZERO,
                end_normal: Vector::ZERO,
                end_pivot: Point::ZERO,
            },
        }
    }
}

pub trait StrokerStorage {
    fn clear(&mut self);
    fn push(&mut self, segment: &Segment);
    fn get(&self) -> &[Segment];

    fn collect(&mut self, segments: &mut impl Iterator<Item = Segment>) -> (bool, bool) {
        self.clear();
        let mut is_closed = false;
        let mut done = false;
        loop {
            if let Some(segment) = segments.next() {
                match segment {
                    Segment::End(closed) => {
                        is_closed = closed;
                        break;
                    }
                    _ => self.push(&segment),
                }
            } else {
                done = true;
                break;
            }
        }
        (is_closed, done)
    }
}

impl StrokerStorage for SmallBuf<Segment> {
    fn clear(&mut self) {
        self.clear();
    }

    #[inline(always)]
    fn push(&mut self, segment: &Segment) {
        self.push(*segment);
    }

    fn get(&self) -> &[Segment] {
        self.data()
    }
}

impl StrokerStorage for Vec<Segment> {
    fn clear(&mut self) {
        self.clear();
    }

    #[inline(always)]
    fn push(&mut self, segment: &Segment) {
        self.push(*segment);
    }

    fn get(&self) -> &[Segment] {
        self
    }
}

const MAX_SMALL_BUF: usize = 128;

#[derive(Clone)]
enum SmallBuf<T> {
    Array([T; MAX_SMALL_BUF], usize),
    Vec(Vec<T>),
}

impl<T: Copy + Default> SmallBuf<T> {
    pub fn new() -> Self {
        Self::Array([T::default(); MAX_SMALL_BUF], 0)
    }

    pub fn data(&self) -> &[T] {
        match self {
            Self::Array(ref buf, len) => &buf[..*len],
            Self::Vec(ref buf) => buf,
        }
    }

    pub fn push(&mut self, value: T) {
        match self {
            Self::Vec(ref mut buf) => buf.push(value),
            Self::Array(ref mut buf, ref mut len) => {
                if *len == MAX_SMALL_BUF {
                    let mut vec = Vec::from(&buf[..]);
                    vec.push(value);
                    *self = Self::Vec(vec);
                } else {
                    buf[*len] = value;
                    *len += 1;
                }
            }
        }
    }

    pub fn clear(&mut self) {
        match self {
            Self::Array(_, ref mut len) => *len = 0,
            Self::Vec(ref mut buf) => buf.clear(),
        }
    }
}
