//! Path builder.

#![allow(clippy::excessive_precision)]

use super::command::Command;
use super::geometry::{Angle, BoundsBuilder, Point, Transform};
#[allow(unused)]
use super::F32Ext;

use crate::lib::Vec;
use core::f32;

/// Describes the size of an arc.
#[derive(Copy, Clone, PartialEq)]
pub enum ArcSize {
    /// An arc of <= 180 degrees will be drawn.
    Small,
    /// An arc of >= 180 degrees will be drawn.
    Large,
}

/// Describes the sweep direction for an arc.
#[derive(Copy, Clone, PartialEq)]
pub enum ArcSweep {
    /// The arc is drawn in a positive angle direction.
    Positive,
    /// The arc is drawn in a negative angle direction.
    Negative,
}

/// Trait for types that accept path commands.
pub trait PathBuilder: Sized {
    /// Returns the current point of the path.
    fn current_point(&self) -> Point;

    /// Moves to the specified point, beginning a new subpath.
    fn move_to(&mut self, to: impl Into<Point>) -> &mut Self;

    /// Moves to the specified point, relative to the current point,
    /// beginning a new subpath.
    fn rel_move_to(&mut self, to: impl Into<Point>) -> &mut Self {
        self.move_to(to.into() + self.current_point())
    }

    /// Adds a line to the specified point. This will begin a new subpath
    /// if the path is empty or the previous subpath was closed.
    fn line_to(&mut self, to: impl Into<Point>) -> &mut Self;

    /// Adds a line to the specified point, relative to the current point. This
    /// will begin a new subpath if the path is empty or the previous subpath
    /// was closed.
    fn rel_line_to(&mut self, to: impl Into<Point>) -> &mut Self {
        self.line_to(to.into() + self.current_point())
    }

    /// Adds a cubic bezier curve from the current point through the specified
    /// control points to the final point. This will begin a new subpath if the
    /// path is empty or the previous subpath was closed.
    fn curve_to(
        &mut self,
        control1: impl Into<Point>,
        control2: impl Into<Point>,
        to: impl Into<Point>,
    ) -> &mut Self;

    /// Adds a cubic bezier curve from the current point through the specified
    /// control points to the final point. All points are considered relative to the
    /// current point. This will begin a new subpath if the path is empty or the
    /// previous subpath was closed.
    fn rel_curve_to(
        &mut self,
        control1: impl Into<Point>,
        control2: impl Into<Point>,
        to: impl Into<Point>,
    ) -> &mut Self {
        let r = self.current_point();
        self.curve_to(control1.into() + r, control2.into() + r, to.into() + r)
    }

    /// Adds a quadratic bezier curve from the current point through the specified
    /// control point to the final point. This will begin a new subpath if the
    /// path is empty or the previous subpath was closed.   
    fn quad_to(&mut self, control1: impl Into<Point>, to: impl Into<Point>) -> &mut Self;

    /// Adds a quadratic bezier curve from the current point through the specified
    /// control point to the final point. All points are considered relative to the
    /// current point. This will begin a new subpath if the path is empty or the
    /// previous subpath was closed.    
    fn rel_quad_to(&mut self, control: impl Into<Point>, to: impl Into<Point>) -> &mut Self {
        let r = self.current_point();
        self.quad_to(control.into() + r, to.into() + r)
    }

    /// Adds an arc with the specified x- and y-radius, rotation angle, arc size,
    /// and arc sweep from the current point to the specified end point. The center
    /// point of the arc will be computed from the parameters. This will begin a
    /// new subpath if the path is empty or the previous subpath was closed.
    fn arc_to(
        &mut self,
        rx: f32,
        ry: f32,
        angle: Angle,
        size: ArcSize,
        sweep: ArcSweep,
        to: impl Into<Point>,
    ) -> &mut Self {
        let from = self.current_point();
        arc(
            self,
            from,
            rx,
            ry,
            angle.to_radians(),
            size,
            sweep,
            to.into(),
        );
        self
    }

    /// Adds an arc with the specified x- and y-radius, rotation angle, arc size,
    /// and arc sweep from the current point to the specified end point. The end
    /// point is considered relative to the current point. The center point of the
    /// arc will be computed from the parameters. This will begin a new subpath if
    /// the path is empty or the previous subpath was closed.
    fn rel_arc_to(
        &mut self,
        rx: f32,
        ry: f32,
        angle: Angle,
        size: ArcSize,
        sweep: ArcSweep,
        to: impl Into<Point>,
    ) -> &mut Self {
        self.arc_to(rx, ry, angle, size, sweep, to.into() + self.current_point())
    }

    /// Closes the current subpath.
    fn close(&mut self) -> &mut Self;

    /// Adds a rectangle with the specified position and size to the path. This
    /// will create a new closed subpath.
    fn add_rect(&mut self, xy: impl Into<Point>, w: f32, h: f32) -> &mut Self {
        let p = xy.into();
        let (l, t, r, b) = (p.x, p.y, p.x + w, p.y + h);
        self.move_to(p);
        self.line_to((r, t));
        self.line_to((r, b));
        self.line_to((l, b));
        self.close()
    }

    /// Adds a rounded rectangle with the specified position, size and radii to
    /// the path. This will create a new closed subpath.
    fn add_round_rect(
        &mut self,
        xy: impl Into<Point>,
        w: f32,
        h: f32,
        rx: f32,
        ry: f32,
    ) -> &mut Self {
        let p = xy.into();
        let size = ArcSize::Small;
        let sweep = ArcSweep::Positive;
        let a = Angle::from_radians(0.);
        let hw = w * 0.5;
        let rx = rx.max(0.).min(hw);
        let hh = h * 0.5;
        let ry = ry.max(0.).min(hh);
        self.move_to((p.x + rx, p.y));
        self.line_to((p.x + w - rx, p.y));
        self.arc_to(rx, ry, a, size, sweep, (p.x + w, p.y + ry));
        self.line_to((p.x + w, p.y + h - ry));
        self.arc_to(rx, ry, a, size, sweep, (p.x + w - rx, p.y + h));
        self.line_to((p.x + rx, p.y + h));
        self.arc_to(rx, ry, a, size, sweep, (p.x, p.y + h - ry));
        self.line_to((p.x, p.y + ry));
        self.arc_to(rx, ry, a, size, sweep, (p.x + rx, p.y));
        self.close()
    }

    /// Adds an ellipse with the specified center and radii to the path. This
    /// will create a new closed subpath.
    fn add_ellipse(&mut self, center: impl Into<Point>, rx: f32, ry: f32) -> &mut Self {
        let center = center.into();
        let cx = center.x;
        let cy = center.y;
        let a = 0.551915024494;
        let arx = a * rx;
        let ary = a * ry;
        self.move_to((cx + rx, cy));
        self.curve_to((cx + rx, cy + ary), (cx + arx, cy + ry), (cx, cy + ry));
        self.curve_to((cx - arx, cy + ry), (cx - rx, cy + ary), (cx - rx, cy));
        self.curve_to((cx - rx, cy - ary), (cx - arx, cy - ry), (cx, cy - ry));
        self.curve_to((cx + arx, cy - ry), (cx + rx, cy - ary), (cx + rx, cy));
        self.close()
    }

    /// Adds a circle with the specified center and radius to the path. This
    /// will create a new closed subpath.
    fn add_circle(&mut self, center: impl Into<Point>, r: f32) -> &mut Self {
        self.add_ellipse(center, r, r)
    }
}

impl PathBuilder for Vec<Command> {
    fn current_point(&self) -> Point {
        match self.last() {
            None => Point::ZERO,
            Some(cmd) => match cmd {
                Command::MoveTo(p)
                | Command::LineTo(p)
                | Command::QuadTo(_, p)
                | Command::CurveTo(_, _, p) => *p,
                Command::Close => {
                    for cmd in self.iter().rev().skip(1) {
                        if let Command::MoveTo(p) = cmd {
                            return *p;
                        }
                    }
                    Point::ZERO
                }
            },
        }
    }

    fn move_to(&mut self, to: impl Into<Point>) -> &mut Self {
        self.push(Command::MoveTo(to.into()));
        self
    }

    fn line_to(&mut self, to: impl Into<Point>) -> &mut Self {
        self.push(Command::LineTo(to.into()));
        self
    }

    fn quad_to(&mut self, control: impl Into<Point>, to: impl Into<Point>) -> &mut Self {
        self.push(Command::QuadTo(control.into(), to.into()));
        self
    }

    fn curve_to(
        &mut self,
        control1: impl Into<Point>,
        control2: impl Into<Point>,
        to: impl Into<Point>,
    ) -> &mut Self {
        self.push(Command::CurveTo(
            control1.into(),
            control2.into(),
            to.into(),
        ));
        self
    }

    fn close(&mut self) -> &mut Self {
        self.push(Command::Close);
        self
    }
}

pub struct TransformSink<'a, S> {
    pub sink: &'a mut S,
    pub transform: Transform,
}

impl<S: PathBuilder> PathBuilder for TransformSink<'_, S> {
    fn current_point(&self) -> Point {
        self.sink.current_point()
    }

    fn move_to(&mut self, to: impl Into<Point>) -> &mut Self {
        self.sink.move_to(self.transform.transform_point(to.into()));
        self
    }

    fn line_to(&mut self, to: impl Into<Point>) -> &mut Self {
        self.sink.line_to(self.transform.transform_point(to.into()));
        self
    }

    fn quad_to(&mut self, control: impl Into<Point>, to: impl Into<Point>) -> &mut Self {
        self.sink.quad_to(
            self.transform.transform_point(control.into()),
            self.transform.transform_point(to.into()),
        );
        self
    }

    fn curve_to(
        &mut self,
        control1: impl Into<Point>,
        control2: impl Into<Point>,
        to: impl Into<Point>,
    ) -> &mut Self {
        self.sink.curve_to(
            self.transform.transform_point(control1.into()),
            self.transform.transform_point(control2.into()),
            self.transform.transform_point(to.into()),
        );
        self
    }

    fn close(&mut self) -> &mut Self {
        self.sink.close();
        self
    }
}

impl PathBuilder for BoundsBuilder {
    fn current_point(&self) -> Point {
        self.current
    }

    fn move_to(&mut self, to: impl Into<Point>) -> &mut Self {
        let p = to.into();
        self.add(p);
        self.current = p;
        self
    }

    fn line_to(&mut self, to: impl Into<Point>) -> &mut Self {
        let p = to.into();
        self.add(p);
        self.current = p;
        self
    }

    fn quad_to(&mut self, control: impl Into<Point>, to: impl Into<Point>) -> &mut Self {
        self.add(control.into());
        let p = to.into();
        self.add(p);
        self.current = p;
        self
    }

    fn curve_to(
        &mut self,
        control1: impl Into<Point>,
        control2: impl Into<Point>,
        to: impl Into<Point>,
    ) -> &mut Self {
        self.add(control1.into());
        self.add(control2.into());
        let p = to.into();
        self.add(p);
        self.current = p;
        self
    }

    fn close(&mut self) -> &mut Self {
        self
    }
}

/// An iterator that generates cubic bezier curves for an arc.
#[derive(Copy, Clone, Default)]
pub struct Arc {
    count: usize,
    center: (f32, f32),
    radii: (f32, f32),
    cosphi: f32,
    sinphi: f32,
    ang1: f32,
    ang2: f32,
    a: f32,
}

impl Arc {
    pub fn new(
        from: impl Into<[f32; 2]>,
        rx: f32,
        ry: f32,
        angle: f32,
        size: ArcSize,
        sweep: ArcSweep,
        to: impl Into<[f32; 2]>,
    ) -> Self {
        let from = from.into();
        let to = to.into();
        let (px, py) = (from[0], from[1]);
        const TAU: f32 = 3.141579 * 2.;
        let (sinphi, cosphi) = angle.sin_cos();
        let pxp = cosphi * (px - to[0]) / 2. + sinphi * (py - to[1]) / 2.;
        let pyp = -sinphi * (px - to[0]) / 2. + cosphi * (py - to[1]) / 2.;
        if pxp == 0. && pyp == 0. {
            return Self::default();
        }
        let mut rx = rx.abs();
        let mut ry = ry.abs();
        let lambda = pxp.powi(2) / rx.powi(2) + pyp.powi(2) / ry.powi(2);
        if lambda > 1. {
            let s = lambda.sqrt();
            rx *= s;
            ry *= s;
        }
        let large_arc = size == ArcSize::Large;
        let sweep = sweep == ArcSweep::Positive;
        let (cx, cy, ang1, mut ang2) = {
            fn vec_angle(ux: f32, uy: f32, vx: f32, vy: f32) -> f32 {
                let sign = if (ux * vy - uy * vx) < 0. { -1. } else { 1. };
                let dot = (ux * vx + uy * vy).clamp(-1., 1.);
                sign * dot.acos()
            }
            let rxsq = rx * rx;
            let rysq = ry * ry;
            let pxpsq = pxp * pxp;
            let pypsq = pyp * pyp;
            let mut radicant = (rxsq * rysq) - (rxsq * pypsq) - (rysq * pxpsq);
            if radicant < 0. {
                radicant = 0.;
            }
            radicant /= (rxsq * pypsq) + (rysq * pxpsq);
            radicant = radicant.sqrt() * if large_arc == sweep { -1. } else { 1. };
            let cxp = radicant * rx / ry * pyp;
            let cyp = radicant * -ry / rx * pxp;
            let cx = cosphi * cxp - sinphi * cyp + (px + to[0]) / 2.;
            let cy = sinphi * cxp + cosphi * cyp + (py + to[1]) / 2.;
            let vx1 = (pxp - cxp) / rx;
            let vy1 = (pyp - cyp) / ry;
            let vx2 = (-pxp - cxp) / rx;
            let vy2 = (-pyp - cyp) / ry;
            let ang1 = vec_angle(1., 0., vx1, vy1);
            let mut ang2 = vec_angle(vx1, vy1, vx2, vy2);
            if !sweep && ang2 > 0. {
                ang2 -= TAU;
            }
            if sweep && ang2 < 0. {
                ang2 += TAU;
            }
            (cx, cy, ang1, ang2)
        };
        let mut ratio = ang2.abs() / (TAU / 4.);
        if (1. - ratio).abs() < 0.0000001 {
            ratio = 1.
        }
        let segments = ratio.ceil().max(1.);
        ang2 /= segments;
        let a = if ang2 == f32::consts::FRAC_PI_2 {
            0.551915024494
        } else if ang2 == -f32::consts::FRAC_PI_2 {
            -0.551915024494
        } else {
            4. / 3. * (ang2 / 4.).tan()
        };
        Self {
            count: segments as usize,
            center: (cx, cy),
            radii: (rx, ry),
            sinphi,
            cosphi,
            ang1,
            ang2,
            a,
        }
    }
}

impl Iterator for Arc {
    type Item = Command;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            return None;
        }
        self.count -= 1;
        let (y1, x1) = self.ang1.sin_cos();
        let (y2, x2) = (self.ang1 + self.ang2).sin_cos();
        let a = self.a;
        let (cx, cy) = self.center;
        let (rx, ry) = self.radii;
        let sinphi = self.sinphi;
        let cosphi = self.cosphi;
        let c1 = Point::new((x1 - y1 * a) * rx, (y1 + x1 * a) * ry);
        let c1 = Point::new(
            cx + (cosphi * c1.x - sinphi * c1.y),
            cy + (sinphi * c1.x + cosphi * c1.y),
        );
        let c2 = Point::new((x2 + y2 * a) * rx, (y2 - x2 * a) * ry);
        let c2 = Point::new(
            cx + (cosphi * c2.x - sinphi * c2.y),
            cy + (sinphi * c2.x + cosphi * c2.y),
        );
        let p = Point::new(x2 * rx, y2 * ry);
        let p = Point::new(
            cx + (cosphi * p.x - sinphi * p.y),
            cy + (sinphi * p.x + cosphi * p.y),
        );
        self.ang1 += self.ang2;
        Some(Command::CurveTo(c1, c2, p))
    }
}

#[allow(clippy::too_many_arguments)]
pub fn arc(
    sink: &mut impl PathBuilder,
    from: Point,
    rx: f32,
    ry: f32,
    angle: f32,
    size: ArcSize,
    sweep: ArcSweep,
    to: Point,
) {
    let p = from;
    let (px, py) = (p.x, p.y);
    const TAU: f32 = core::f32::consts::PI * 2.;
    let (sinphi, cosphi) = angle.sin_cos();
    let pxp = cosphi * (px - to.x) / 2. + sinphi * (py - to.y) / 2.;
    let pyp = -sinphi * (px - to.x) / 2. + cosphi * (py - to.y) / 2.;
    if pxp == 0. && pyp == 0. {
        return;
    }
    let mut rx = rx.abs();
    let mut ry = ry.abs();
    let lambda = pxp.powi(2) / rx.powi(2) + pyp.powi(2) / ry.powi(2);
    if lambda > 1. {
        let s = lambda.sqrt();
        rx *= s;
        ry *= s;
    }
    let large_arc = size == ArcSize::Large;
    let sweep = sweep == ArcSweep::Positive;
    let (cx, cy, mut ang1, mut ang2) = {
        fn vec_angle(ux: f32, uy: f32, vx: f32, vy: f32) -> f32 {
            let sign = if (ux * vy - uy * vx) < 0. { -1. } else { 1. };
            let dot = (ux * vx + uy * vy).clamp(-1., 1.);
            sign * dot.acos()
        }
        let rxsq = rx * rx;
        let rysq = ry * ry;
        let pxpsq = pxp * pxp;
        let pypsq = pyp * pyp;
        let mut radicant = (rxsq * rysq) - (rxsq * pypsq) - (rysq * pxpsq);
        if radicant < 0. {
            radicant = 0.;
        }
        radicant /= (rxsq * pypsq) + (rysq * pxpsq);
        radicant = radicant.sqrt() * if large_arc == sweep { -1. } else { 1. };
        let cxp = radicant * rx / ry * pyp;
        let cyp = radicant * -ry / rx * pxp;
        let cx = cosphi * cxp - sinphi * cyp + (px + to.x) / 2.;
        let cy = sinphi * cxp + cosphi * cyp + (py + to.y) / 2.;
        let vx1 = (pxp - cxp) / rx;
        let vy1 = (pyp - cyp) / ry;
        let vx2 = (-pxp - cxp) / rx;
        let vy2 = (-pyp - cyp) / ry;
        let ang1 = vec_angle(1., 0., vx1, vy1);
        let mut ang2 = vec_angle(vx1, vy1, vx2, vy2);
        if !sweep && ang2 > 0. {
            ang2 -= TAU;
        }
        if sweep && ang2 < 0. {
            ang2 += TAU;
        }
        (cx, cy, ang1, ang2)
    };
    let mut ratio = ang2.abs() / (TAU / 4.);
    if (1. - ratio).abs() < 0.0000001 {
        ratio = 1.
    }
    let segments = ratio.ceil().max(1.);
    ang2 /= segments;
    let a = if ang2 == f32::consts::FRAC_PI_2 {
        0.551915024494
    } else if ang2 == -f32::consts::FRAC_PI_2 {
        -0.551915024494
    } else {
        4. / 3. * (ang2 / 4.).tan()
    };
    for _ in 0..segments as usize {
        let (y1, x1) = ang1.sin_cos();
        let (y2, x2) = (ang1 + ang2).sin_cos();
        let c1 = Point::new((x1 - y1 * a) * rx, (y1 + x1 * a) * ry);
        let c1 = Point::new(
            cx + (cosphi * c1.x - sinphi * c1.y),
            cy + (sinphi * c1.x + cosphi * c1.y),
        );
        let c2 = Point::new((x2 + y2 * a) * rx, (y2 - x2 * a) * ry);
        let c2 = Point::new(
            cx + (cosphi * c2.x - sinphi * c2.y),
            cy + (sinphi * c2.x + cosphi * c2.y),
        );
        let p = Point::new(x2 * rx, y2 * ry);
        let p = Point::new(
            cx + (cosphi * p.x - sinphi * p.y),
            cy + (sinphi * p.x + cosphi * p.y),
        );
        sink.curve_to(c1, c2, p);
        ang1 += ang2;
    }
}
