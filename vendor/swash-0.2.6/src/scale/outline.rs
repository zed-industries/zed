/*!
Glyph outline.
*/

use alloc::vec::Vec;
use zeno::{Bounds, PathData, Point, Transform, Verb};

/// Scaled glyph outline represented as a collection of layers and a sequence
/// of points and verbs.
#[derive(Clone, Default)]
pub struct Outline {
    layers: Vec<LayerData>,
    points: Vec<Point>,
    verbs: Vec<Verb>,
    is_color: bool,
}

impl Outline {
    /// Creates a new empty outline.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns true if the outline has color layers.
    pub fn is_color(&self) -> bool {
        self.is_color
    }

    /// Returns the number of layers in the outline.
    pub fn len(&self) -> usize {
        self.layers.len()
    }

    /// Returns true if there are no layers in the outline.
    pub fn is_empty(&self) -> bool {
        self.layers.is_empty()
    }

    /// Returns a reference to the layer at the specified index.
    pub fn get<'a>(&'a self, index: usize) -> Option<Layer<'a>> {
        let data = self.layers.get(index)?;
        let points = self.points.get(data.points.0..data.points.1)?;
        let verbs = self.verbs.get(data.verbs.0..data.verbs.1)?;
        let color_index = data.color_index;
        Some(Layer {
            points,
            verbs,
            color_index,
        })
    }

    /// Returns a mutable reference to the layer at the specified index.
    pub fn get_mut<'a>(&'a mut self, index: usize) -> Option<LayerMut<'a>> {
        let data = self.layers.get(index)?;
        let points = self.points.get_mut(data.points.0..data.points.1)?;
        let verbs = self.verbs.get(data.verbs.0..data.verbs.1)?;
        let color_index = data.color_index;
        Some(LayerMut {
            points,
            verbs,
            color_index,
        })
    }

    /// Returns a reference to the sequence of points in the outline.
    pub fn points(&self) -> &[Point] {
        &self.points
    }

    /// Returns a mutable reference to the sequence of points in the outline.
    pub fn points_mut(&mut self) -> &mut [Point] {
        &mut self.points
    }

    /// Returns a reference to the sequence of verbs in the outline.
    pub fn verbs(&self) -> &[Verb] {
        &self.verbs
    }

    /// Returns path data for the outline.
    pub fn path(&self) -> impl PathData + '_ {
        (&self.points[..], &self.verbs[..])
    }

    /// Computes the bounding box of the outline.
    pub fn bounds(&self) -> Bounds {
        Bounds::from_points(&self.points)
    }

    /// Transforms the outline by the specified matrix.
    pub fn transform(&mut self, transform: &Transform) {
        for p in &mut self.points {
            *p = transform.transform_point(*p);
        }
    }

    /// Applies a faux bold to the outline with the specified strengths in the
    /// x and y directions.
    pub fn embolden(&mut self, x_strength: f32, y_strength: f32) {
        for i in 0..self.len() {
            if let Some(mut layer) = self.get_mut(i) {
                layer.embolden(x_strength, y_strength);
            }
        }
    }

    /// Clears the outline.
    pub fn clear(&mut self) {
        self.points.clear();
        self.verbs.clear();
        self.layers.clear();
        self.is_color = false;
    }
}

/// Reference to a layer in a scaled outline.
#[derive(Copy, Clone)]
pub struct Layer<'a> {
    points: &'a [Point],
    verbs: &'a [Verb],
    color_index: Option<u16>,
}

impl<'a> Layer<'a> {
    /// Returns the sequence of points for the layer.
    pub fn points(&self) -> &'a [Point] {
        self.points
    }

    /// Returns the sequence of verbs for the layer.
    pub fn verbs(&self) -> &'a [Verb] {
        self.verbs
    }

    /// Returns path data for the layer.
    pub fn path(&self) -> impl PathData + 'a {
        (self.points(), self.verbs())
    }

    /// Computes the bounding box of the layer.
    pub fn bounds(&self) -> Bounds {
        Bounds::from_points(self.points())
    }

    /// Returns the color index for the layer.
    pub fn color_index(&self) -> Option<u16> {
        self.color_index
    }
}

/// Mutable reference to a layer in a scaled outline.
pub struct LayerMut<'a> {
    points: &'a mut [Point],
    verbs: &'a [Verb],
    color_index: Option<u16>,
}

impl<'a> LayerMut<'a> {
    /// Returns the sequence of points for the layer.
    pub fn points(&'a self) -> &'a [Point] {
        self.points
    }

    /// Returns a mutable reference the sequence of points for the layer.
    pub fn points_mut(&'a mut self) -> &'a mut [Point] {
        &mut *self.points
    }

    /// Returns the sequence of verbs for the layer.
    pub fn verbs(&self) -> &'a [Verb] {
        self.verbs
    }

    /// Returns path data for the layer.
    pub fn path(&'a self) -> impl PathData + 'a {
        (self.points(), self.verbs())
    }

    /// Computes the bounding box of the layer.
    pub fn bounds(&self) -> Bounds {
        Bounds::from_points(self.points())
    }

    /// Returns the color index for the layer.
    pub fn color_index(&self) -> Option<u16> {
        self.color_index
    }
    /// Transforms this layer by the specified matrix.
    pub fn transform(&'a mut self, transform: &Transform) {
        for p in self.points.iter_mut() {
            *p = transform.transform_point(*p);
        }
    }

    /// Applies a faux bold to this layer with the specified strengths in the
    /// x and y directions.
    pub fn embolden(&mut self, x_strength: f32, y_strength: f32) {
        let mut point_start = 0;
        let mut pos = 0;
        let winding = compute_winding(self.points);
        for verb in self.verbs {
            match verb {
                Verb::MoveTo | Verb::Close => {
                    if let Some(points) = self.points.get_mut(point_start..pos) {
                        if !points.is_empty() {
                            embolden(points, winding, x_strength, y_strength);
                        }
                        point_start = pos;
                        if *verb == Verb::MoveTo {
                            pos += 1;
                        }
                    } else {
                        return;
                    }
                }
                Verb::LineTo => pos += 1,
                Verb::QuadTo => pos += 2,
                Verb::CurveTo => pos += 3,
            }
        }
        if pos > point_start {
            if let Some(points) = self.points.get_mut(point_start..pos) {
                embolden(points, winding, x_strength, y_strength);
            }
        }
    }
}

#[derive(Copy, Clone, Default)]
struct LayerData {
    points: (usize, usize),
    verbs: (usize, usize),
    color_index: Option<u16>,
}

impl Outline {
    pub(super) fn set_color(&mut self, color: bool) {
        self.is_color = color;
    }

    pub(super) fn move_to(&mut self, p: Point) {
        self.maybe_close();
        self.points.push(p);
        self.verbs.push(Verb::MoveTo);
    }

    pub(super) fn line_to(&mut self, p: Point) {
        self.points.push(p);
        self.verbs.push(Verb::LineTo);
    }

    pub(super) fn quad_to(&mut self, p0: Point, p1: Point) {
        self.points.push(p0);
        self.points.push(p1);
        self.verbs.push(Verb::QuadTo);
    }

    pub(super) fn curve_to(&mut self, p0: Point, p1: Point, p2: Point) {
        self.points.push(p0);
        self.points.push(p1);
        self.points.push(p2);
        self.verbs.push(Verb::CurveTo);
    }

    pub(super) fn close(&mut self) {
        self.verbs.push(Verb::Close);
    }

    pub(super) fn maybe_close(&mut self) {
        if !self.verbs.is_empty() && self.verbs.last() != Some(&Verb::Close) {
            self.close();
        }
    }

    pub(super) fn begin_layer(&mut self, color_index: Option<u16>) {
        let points_end = self.points.len();
        let verbs_end = self.verbs.len();
        if let Some(last) = self.layers.last_mut() {
            last.points.1 = points_end;
            last.verbs.1 = verbs_end;
        }
        self.layers.push(LayerData {
            points: (points_end, points_end),
            verbs: (verbs_end, verbs_end),
            color_index,
        });
    }

    pub(super) fn finish(&mut self) {
        let points_end = self.points.len();
        let verbs_end = self.verbs.len();
        if let Some(last) = self.layers.last_mut() {
            last.points.1 = points_end;
            last.verbs.1 = verbs_end;
        } else {
            self.layers.push(LayerData {
                points: (0, points_end),
                verbs: (0, verbs_end),
                color_index: None,
            });
        }
    }
}

fn embolden(points: &mut [Point], winding: u8, x_strength: f32, y_strength: f32) {
    if points.is_empty() {
        return;
    }
    let last = points.len() - 1;
    let mut i = last;
    let mut j = 0;
    let mut k = !0;
    let mut out_len;
    let mut in_len = 0.;
    let mut anchor_len = 0.;
    let mut anchor = Point::ZERO;
    let mut out;
    let mut in_ = Point::ZERO;
    while j != i && i != k {
        if j != k {
            out = points[j] - points[i];
            out_len = out.length();
            if out_len == 0. {
                j = if j < last { j + 1 } else { 0 };
                continue;
            } else {
                let s = 1. / out_len;
                out.x *= s;
                out.y *= s;
            }
        } else {
            out = anchor;
            out_len = anchor_len;
        }
        if in_len != 0. {
            if k == !0 {
                k = i;
                anchor = in_;
                anchor_len = in_len;
            }
            let mut d = (in_.x * out.x) + (in_.y * out.y);
            let shift = if d > -0.9396 {
                d += 1.;
                let mut sx = in_.y + out.y;
                let mut sy = in_.x + out.x;
                if winding == 0 {
                    sx = -sx;
                } else {
                    sy = -sy;
                }
                let mut q = (out.x * in_.y) - (out.y * in_.x);
                if winding == 0 {
                    q = -q;
                }
                let l = in_len.min(out_len);
                if x_strength * q <= l * d {
                    sx = sx * x_strength / d;
                } else {
                    sx = sx * l / q;
                }
                if y_strength * q <= l * d {
                    sy = sy * y_strength / d;
                } else {
                    sy = sy * l / q;
                }
                Point::new(sx, sy)
            } else {
                Point::ZERO
            };

            while i != j {
                points[i].x += x_strength + shift.x;
                points[i].y += y_strength + shift.y;
                i = if i < last { i + 1 } else { 0 };
            }
        } else {
            i = j;
        }
        in_ = out;
        in_len = out_len;
        j = if j < last { j + 1 } else { 0 };
    }
}

fn compute_winding(points: &[Point]) -> u8 {
    if points.is_empty() {
        return 0;
    }
    let mut area = 0.;
    let last = points.len() - 1;
    let mut prev = points[last];
    for cur in points[0..=last].iter() {
        area += (cur.y - prev.y) * (cur.x + prev.x);
        prev = *cur;
    }
    if area > 0. {
        1
    } else {
        0
    }
}

// The OutlineWriter wrapper allows us to make the trait implementation of OutlinePen
// private which allows us to keep skrifa as a private dependency of swash
pub(crate) struct OutlineWriter<'a>(pub &'a mut Outline);
impl skrifa::outline::OutlinePen for OutlineWriter<'_> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.0.move_to((x, y).into());
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.0.line_to((x, y).into());
    }

    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) {
        self.0.quad_to((cx0, cy0).into(), (x, y).into());
    }

    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        self.0
            .curve_to((cx0, cy0).into(), (cx1, cy1).into(), (x, y).into());
    }

    fn close(&mut self) {
        self.0.close();
    }
}
