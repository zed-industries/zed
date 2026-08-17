//! Path segmentation.

#![allow(clippy::excessive_precision)]

use super::command::Command;
use super::geometry::*;
#[allow(unused)]
use super::F32Ext;

use core::borrow::Borrow;
use core::f32;

/// Represents the time parameter for a specific distance along
/// a segment.
#[derive(Copy, Clone, Debug)]
pub struct SegmentTime {
    pub distance: f32,
    pub time: f32,
}

/// Line segment.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Line {
    pub a: Point,
    pub b: Point,
}

impl Line {
    /// Creates a new line segment.
    pub fn new(a: impl Into<Vector>, b: impl Into<Vector>) -> Self {
        Self {
            a: a.into(),
            b: b.into(),
        }
    }

    /// Returns the length of the line segment.
    pub fn length(&self) -> f32 {
        (self.b - self.a).length()
    }

    /// Returns a slice of the line segment described by the specified start and end times.
    #[allow(unused)]
    pub fn slice(&self, start: f32, end: f32) -> Self {
        let dir = self.b - self.a;
        Self::new(self.a + dir * start, self.a + dir * end)
    }

    #[allow(unused)]
    pub fn time(&self, distance: f32) -> SegmentTime {
        let len = (self.b - self.a).length();
        if distance > len {
            return SegmentTime {
                distance: len,
                time: 1.,
            };
        }
        SegmentTime {
            distance,
            time: distance / len,
        }
    }

    #[allow(unused)]
    pub fn reverse(&self) -> Self {
        Self::new(self.b, self.a)
    }
}

/// Cubic bezier curve.
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Curve {
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub d: Point,
}

impl Curve {
    /// Creates a new curve.
    pub fn new(
        a: impl Into<Point>,
        b: impl Into<Point>,
        c: impl Into<Point>,
        d: impl Into<Point>,
    ) -> Self {
        Curve {
            a: a.into(),
            b: b.into(),
            c: c.into(),
            d: d.into(),
        }
    }

    /// Creates a new curve from a quadratic bezier curve.
    pub fn from_quadratic(a: impl Into<Point>, b: impl Into<Point>, c: impl Into<Point>) -> Self {
        let a = a.into();
        let b = b.into();
        let c = c.into();
        Self {
            a,
            b: Point::new(a.x + 2. / 3. * (b.x - a.x), a.y + 2. / 3. * (b.y - a.y)),
            c: Point::new(c.x + 2. / 3. * (b.x - c.x), c.y + 2. / 3. * (b.y - c.y)),
            d: c,
        }
    }

    /// Returns the length of the curve.
    pub fn length(&self) -> f32 {
        let mut len = 0.;
        let mut prev = self.a;
        let steps = 64;
        let step = 1. / steps as f32;
        let mut t = 0.;
        for _ in 0..=steps {
            t += step;
            let next = self.evaluate(t);
            len += (next - prev).length();
            prev = next;
        }
        len
    }

    /// Returns a slice of the curve described by the specified start and end times.
    pub fn slice(&self, start: f32, end: f32) -> Self {
        let t0 = start;
        let t1 = end;
        let u0 = 1. - t0;
        let u1 = 1. - t1;
        let v0 = self.a;
        let v1 = self.b;
        let v2 = self.c;
        let v3 = self.d;
        Self::new(
            (v0 * (u0 * u0 * u0))
                + (v1 * (t0 * u0 * u0 + u0 * t0 * u0 + u0 * u0 * t0))
                + (v2 * (t0 * t0 * u0 + u0 * t0 * t0 + t0 * u0 * t0))
                + (v3 * (t0 * t0 * t0)),
            (v0 * (u0 * u0 * u1))
                + (v1 * (t0 * u0 * u1 + u0 * t0 * u1 + u0 * u0 * t1))
                + (v2 * (t0 * t0 * u1 + u0 * t0 * t1 + t0 * u0 * t1))
                + (v3 * (t0 * t0 * t1)),
            (v0 * (u0 * u1 * u1))
                + (v1 * (t0 * u1 * u1 + u0 * t1 * u1 + u0 * u1 * t1))
                + (v2 * (t0 * t1 * u1 + u0 * t1 * t1 + t0 * u1 * t1))
                + (v3 * (t0 * t1 * t1)),
            (v0 * (u1 * u1 * u1))
                + (v1 * (t1 * u1 * u1 + u1 * t1 * u1 + u1 * u1 * t1))
                + (v2 * (t1 * t1 * u1 + u1 * t1 * t1 + t1 * u1 * t1))
                + (v3 * (t1 * t1 * t1)),
        )
    }

    /// Returns a curve with the direction reversed.
    #[allow(unused)]
    pub fn reverse(&self) -> Self {
        Self::new(self.d, self.c, self.b, self.a)
    }

    /// Returns the time parameter for the specified linear distance along
    /// the curve.
    #[allow(unused)]
    pub fn time(&self, distance: f32, tolerance: f32) -> SegmentTime {
        let (distance, time) = self.time_impl(distance, tolerance, 1., 0);
        SegmentTime { distance, time }
    }

    /// Returns true if the curve can be represented as a line within some
    /// tolerance.
    pub fn is_line(&self, tolerance: f32) -> bool {
        let degen_ab = self.a.nearly_eq_by(self.b, tolerance);
        let degen_bc = self.b.nearly_eq_by(self.c, tolerance);
        let degen_cd = self.c.nearly_eq_by(self.d, tolerance);
        degen_ab as u8 + degen_bc as u8 + degen_cd as u8 >= 2
    }

    /// Evaluates the curve at the specified time.
    pub fn evaluate(&self, time: f32) -> Point {
        let t = time;
        let t0 = 1. - t;
        (self.a * (t0 * t0 * t0))
            + (self.b * (3. * t0 * t0 * t))
            + (self.c * (3. * t0 * t * t))
            + (self.d * (t * t * t))
    }

    #[allow(clippy::wrong_self_convention)]
    fn to_segment(&self, id: SegmentId) -> Option<Segment> {
        if self.is_line(MERGE_EPSILON) {
            if self.a.nearly_eq_by(self.d, MERGE_EPSILON) {
                None
            } else {
                Some(Segment::Line(id, Line::new(self.a, self.d)))
            }
        } else {
            Some(Segment::Curve(id, *self))
        }
    }

    fn split_at_max_curvature(&self, splits: &mut [Curve; 4]) -> usize {
        let mut tmp = [0f32; 3];
        let count1 = self.max_curvature(&mut tmp);
        let mut count = 0;
        let mut ts = [0f32; 4];
        for &t in &tmp[..count1] {
            if t > 0. && t < 1. {
                ts[count] = t;
                count += 1;
            }
        }
        if count == 0 {
            splits[0] = *self;
        } else {
            let mut i = 0;
            let mut last_t = 0.;
            for &t in &ts[..count] {
                splits[i] = self.slice(last_t, t);
                i += 1;
                last_t = t;
            }
            splits[i] = self.slice(last_t, 1.);
        }
        count + 1
    }

    fn split(&self, t: f32) -> (Self, Self) {
        (self.slice(0., t), self.slice(t, 1.))
    }

    fn time_impl(&self, distance: f32, tolerance: f32, t: f32, level: u8) -> (f32, f32) {
        if level < 5 && self.too_curvy(tolerance) {
            let c0 = self.slice(0., 0.5);
            let (dist0, t0) = c0.time_impl(distance, tolerance, t * 0.5, level + 1);
            if dist0 < distance {
                let c1 = self.slice(0.5, 1.);
                let (dist1, t1) = c1.time_impl(distance - dist0, tolerance, t * 0.5, level + 1);
                (dist0 + dist1, t0 + t1)
            } else {
                (dist0, t0)
            }
        } else {
            let dist = (self.d - self.a).length();
            if dist >= distance {
                let s = distance / dist;
                (distance, t * s)
            } else {
                (dist, t)
            }
        }
    }

    fn max_curvature(&self, ts: &mut [f32; 3]) -> usize {
        let comps_x = [self.a.x, self.b.x, self.c.x, self.d.x];
        let comps_y = [self.a.y, self.b.y, self.c.y, self.d.y];
        fn get_coeffs(src: [f32; 4]) -> [f32; 4] {
            let a = src[1] - src[0];
            let b = src[2] - 2. * src[1] + src[0];
            let c = src[3] + 3. * (src[1] - src[2]) - src[0];
            [c * c, 3. * b * c, 2. * b * b + c * a, a * b]
        }
        let mut coeffs = get_coeffs(comps_x);
        let coeffs_y = get_coeffs(comps_y);
        for i in 0..4 {
            coeffs[i] += coeffs_y[i];
        }
        Self::solve(coeffs, ts)
    }

    fn solve(coeff: [f32; 4], ts: &mut [f32; 3]) -> usize {
        const PI: f32 = core::f32::consts::PI;
        let i = 1. / coeff[0];
        let a = coeff[1] * i;
        let b = coeff[2] * i;
        let c = coeff[3] * i;
        let q = (a * a - b * 3.) / 9.;
        let r = (2. * a * a * a - 9. * a * b + 27. * c) / 54.;
        let q3 = q * q * q;
        let r2_sub_q3 = r * r - q3;
        let adiv3 = a / 3.;
        if r2_sub_q3 < 0. {
            let theta = satf32(r / q3.sqrt()).acos();
            let neg2_root_q = -2. * q.sqrt();
            ts[0] = satf32(neg2_root_q * (theta / 3.).cos() - adiv3);
            ts[1] = satf32(neg2_root_q * ((theta + 2. * PI) / 3.).cos() - adiv3);
            ts[2] = satf32(neg2_root_q * ((theta - 2. * PI) / 3.).cos() - adiv3);
            ts.sort_unstable_by(|x, y| x.partial_cmp(y).unwrap_or(core::cmp::Ordering::Less));
            let mut count = 3;
            if ts[0] == ts[1] {
                ts[1] = ts[2];
                count -= 1;
            }
            if ts[1] == ts[2] {
                count -= 1;
            }
            count
        } else {
            let mut a = r.abs() + r2_sub_q3.sqrt();
            a = a.powf(0.3333333);
            if r > 0. {
                a = -a;
            }
            if a != 0. {
                a += q / a;
            }
            ts[0] = satf32(a - adiv3);
            1
        }
    }

    fn too_curvy(&self, tolerance: f32) -> bool {
        (2. * self.d.x - 3. * self.c.x + self.a.x).abs() > tolerance
            || (2. * self.d.y - 3. * self.c.y + self.a.y).abs() > tolerance
            || (self.d.x - 3. * self.b.x + 2. * self.a.x).abs() > tolerance
            || (self.d.y - 3. * self.b.y + 2. * self.a.y).abs() > tolerance
    }

    fn needs_split(&self) -> bool {
        if self.b.nearly_eq_by(self.c, MERGE_EPSILON) {
            return true;
        }
        let normal_ab = normal(self.a, self.b);
        let normal_bc = normal(self.b, self.c);
        fn too_curvy(n0: Vector, n1: Vector) -> bool {
            const FLAT_ENOUGH: f32 = f32::consts::SQRT_2 / 2. + 1. / 10.;
            n0.dot(n1) <= FLAT_ENOUGH
        }
        too_curvy(normal_ab, normal_bc) || too_curvy(normal_bc, normal(self.c, self.d))
    }
}

fn satf32(x: f32) -> f32 {
    x.clamp(0., 1.)
}

/// Marker that allows regrouping of previously split segments due to simplification.
pub type SegmentId = u8;

/// Segment of a path.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Segment {
    /// Line segment..
    Line(SegmentId, Line),
    /// Cubic bezier segment.
    Curve(SegmentId, Curve),
    /// Marks the end of a subpath. Contains the value `true` if the subpath
    /// is closed.
    End(bool),
}

impl Segment {
    pub fn length(&self) -> f32 {
        match self {
            Self::Line(_, line) => line.length(),
            Self::Curve(_, curve) => curve.length(),
            _ => 0.,
        }
    }

    #[allow(unused)]
    pub fn slice(&self, start: f32, end: f32) -> Self {
        match self {
            Self::Line(id, line) => Self::Line(*id, line.slice(start, end)),
            Self::Curve(id, curve) => Self::Curve(*id, curve.slice(start, end)),
            Self::End(..) => *self,
        }
    }

    #[allow(unused)]
    pub fn reverse(&self) -> Self {
        match self {
            Self::Line(id, line) => Self::Line(*id, line.reverse()),
            Self::Curve(id, curve) => Self::Curve(*id, curve.reverse()),
            Self::End(..) => *self,
        }
    }

    #[allow(unused)]
    pub fn time(&self, distance: f32, tolerance: f32) -> SegmentTime {
        match self {
            Self::Line(_, line) => line.time(distance),
            Self::Curve(_, curve) => curve.time(distance, tolerance),
            _ => SegmentTime {
                distance: 0.,
                time: 0.,
            },
        }
    }

    #[allow(unused)]
    pub fn point_normal(&self, time: f32) -> (Point, Vector) {
        match self {
            Self::Line(_, line) => {
                let dir = line.b - line.a;
                let p = line.a + dir * time;
                let n = normal(line.a, line.b);
                (p, n)
            }
            Self::Curve(_, curve) => {
                let p = curve.evaluate(time);
                let a = curve.evaluate(time - 0.05);
                let b = curve.evaluate(time + 0.05);
                let n = normal(a, b);
                (p, n)
            }
            Self::End(..) => (Point::ZERO, Vector::ZERO),
        }
    }
}

impl Default for Segment {
    fn default() -> Self {
        Self::End(false)
    }
}

// This large epsilon trades fidelity for performance, visual continuity
// and numeric stability.
const MERGE_EPSILON: f32 = 0.01;

/// Creates a segment iterator from a command iterator, optionally producing
/// simplified curves.
pub fn segments<I>(commands: I, simplify_curves: bool) -> Segments<I>
where
    I: Iterator + Clone,
    I::Item: Borrow<Command>,
{
    Segments::new(simplify_curves, commands)
}

/// Iterator over path segments.
#[derive(Clone)]
pub struct Segments<I> {
    commands: I,
    start: Vector,
    prev: Vector,
    close: bool,
    split: bool,
    splits: [Curve; 16],
    split_count: usize,
    split_index: usize,
    last_was_end: bool,
    id: u8,
    count: u32,
}

impl<I> Segments<I>
where
    I: Iterator + Clone,
    I::Item: Borrow<Command>,
{
    fn new(split: bool, commands: I) -> Self {
        Self {
            commands,
            start: Vector::ZERO,
            prev: Vector::ZERO,
            close: false,
            split,
            splits: [Curve::default(); 16],
            split_count: 0,
            split_index: 0,
            last_was_end: true,
            id: 0,
            count: 0,
        }
    }

    #[allow(clippy::needless_range_loop)]
    fn split_curve(&mut self, id: SegmentId, c: &Curve) -> Option<Segment> {
        if c.is_line(MERGE_EPSILON) {
            if c.a.nearly_eq_by(c.d, MERGE_EPSILON) {
                return None;
            }
            return Some(Segment::Line(id, Line::new(c.a, c.d)));
        }
        let mut splits = [Curve::default(); 4];
        let count = c.split_at_max_curvature(&mut splits);
        let mut i = 0;
        for j in 0..count {
            let curve = splits[j];
            if curve.needs_split() {
                let (a, b) = curve.split(0.5);
                if a.needs_split() {
                    let (c, d) = a.split(0.5);
                    self.splits[i] = c;
                    self.splits[i + 1] = d;
                    i += 2;
                } else {
                    self.splits[i] = a;
                    i += 1;
                }
                if b.needs_split() {
                    let (c, d) = b.split(0.5);
                    self.splits[i] = c;
                    self.splits[i + 1] = d;
                    i += 2;
                } else {
                    self.splits[i] = b;
                    i += 1;
                }
            } else {
                self.splits[i] = curve;
                i += 1;
            }
        }
        self.split_count = i;
        self.split_index = 1;
        self.splits[0].to_segment(id)
    }

    fn inc_id(&mut self) {
        if self.id == 254 {
            self.id = 0;
        } else {
            self.id += 1;
        }
    }
}

impl<I> Iterator for Segments<I>
where
    I: Iterator + Clone,
    I::Item: Borrow<Command>,
{
    type Item = Segment;

    fn next(&mut self) -> Option<Self::Item> {
        use Command::*;
        if self.close {
            self.close = false;
            self.last_was_end = true;
            return Some(Segment::End(true));
        }
        if self.split {
            loop {
                if self.split_index < self.split_count {
                    let curve = self.splits[self.split_index];
                    self.split_index += 1;
                    if let Some(segment) = curve.to_segment(self.id) {
                        self.count += 1;
                        self.last_was_end = false;
                        self.prev = curve.d;
                        return Some(segment);
                    }
                    continue;
                }
                self.inc_id();
                let id = self.id;
                let from = self.prev;
                match *self.commands.next()?.borrow() {
                    MoveTo(to) => {
                        self.start = to;
                        self.prev = to;
                        self.count = 0;
                        if !self.last_was_end {
                            self.last_was_end = true;
                            return Some(Segment::End(false));
                        }
                    }
                    LineTo(to) => {
                        if !from.nearly_eq_by(to, MERGE_EPSILON) {
                            self.count += 1;
                            self.prev = to;
                            self.last_was_end = false;
                            return Some(Segment::Line(id, Line::new(from, to)));
                        }
                    }
                    CurveTo(c1, c2, to) => {
                        if let Some(segment) = self.split_curve(id, &Curve::new(from, c1, c2, to)) {
                            self.count += 1;
                            self.prev = to;
                            self.last_was_end = false;
                            return Some(segment);
                        }
                    }
                    QuadTo(c, to) => {
                        if let Some(segment) =
                            self.split_curve(id, &Curve::from_quadratic(from, c, to))
                        {
                            self.count += 1;
                            self.prev = to;
                            self.last_was_end = false;
                            return Some(segment);
                        }
                    }
                    Close => {
                        self.prev = self.start;
                        if self.count == 0 || !from.nearly_eq_by(self.start, MERGE_EPSILON) {
                            self.close = true;
                            return Some(Segment::Line(id, Line::new(from, self.start)));
                        } else {
                            self.count = 0;
                            self.last_was_end = true;
                            return Some(Segment::End(true));
                        }
                    }
                }
            }
        } else {
            let id = self.id;
            self.inc_id();
            loop {
                let from = self.prev;
                match *self.commands.next()?.borrow() {
                    MoveTo(to) => {
                        self.start = to;
                        self.prev = to;
                        self.count = 0;
                        if !self.last_was_end {
                            self.last_was_end = true;
                            return Some(Segment::End(false));
                        }
                    }
                    LineTo(to) => {
                        if !from.nearly_eq_by(to, MERGE_EPSILON) {
                            self.count += 1;
                            self.prev = to;
                            self.last_was_end = false;
                            return Some(Segment::Line(id, Line::new(from, to)));
                        }
                    }
                    CurveTo(c1, c2, to) => {
                        let segment = Segment::Curve(id, Curve::new(from, c1, c2, to));
                        self.count += 1;
                        self.prev = to;
                        self.last_was_end = false;
                        return Some(segment);
                    }
                    QuadTo(c, to) => {
                        let segment = Segment::Curve(id, Curve::from_quadratic(from, c, to));
                        self.count += 1;
                        self.prev = to;
                        self.last_was_end = false;
                        return Some(segment);
                    }
                    Close => {
                        self.prev = self.start;
                        if self.count == 0 || !from.nearly_eq_by(self.start, 0.01) {
                            self.close = true;
                            return Some(Segment::Line(id, Line::new(from, self.start)));
                        } else {
                            self.count = 0;
                            self.last_was_end = true;
                            return Some(Segment::End(true));
                        }
                    }
                }
            }
        }
    }
}
