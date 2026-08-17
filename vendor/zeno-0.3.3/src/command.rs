//! Path commands.

use super::geometry::{Point, Transform};
use super::path_builder::PathBuilder;

use core::borrow::Borrow;

/// Path command.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Command {
    /// Begins a new subpath at the specified point.
    MoveTo(Point),
    /// A straight line from the previous point to the specified point.
    LineTo(Point),
    /// A cubic bezier curve from the previous point to the final point with
    /// two intermediate control points.
    CurveTo(Point, Point, Point),
    /// A quadratic curve from the previous point to the final point with one
    /// intermediate control point.
    QuadTo(Point, Point),
    /// Closes a subpath, connecting the final point to the initial point.
    Close,
}

impl Command {
    /// Returns the associated verb for the command.
    pub fn verb(&self) -> Verb {
        use Command::*;
        match self {
            MoveTo(..) => Verb::MoveTo,
            LineTo(..) => Verb::LineTo,
            QuadTo(..) => Verb::QuadTo,
            CurveTo(..) => Verb::CurveTo,
            Close => Verb::CurveTo,
        }
    }

    /// Returns the result of a transformation matrix applied to the command.
    #[inline]
    pub fn transform(&self, transform: &Transform) -> Self {
        use Command::*;
        let t = transform;
        match self {
            MoveTo(p) => MoveTo(t.transform_point(*p)),
            LineTo(p) => LineTo(t.transform_point(*p)),
            QuadTo(c, p) => QuadTo(t.transform_point(*c), t.transform_point(*p)),
            CurveTo(c1, c2, p) => CurveTo(
                t.transform_point(*c1),
                t.transform_point(*c2),
                t.transform_point(*p),
            ),
            Close => Close,
        }
    }
}

/// Action of a path command.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Verb {
    MoveTo,
    LineTo,
    CurveTo,
    QuadTo,
    Close,
}

#[derive(Clone)]
pub struct PointsCommands<'a> {
    points: &'a [Point],
    verbs: &'a [Verb],
    point: usize,
    verb: usize,
}

impl<'a> PointsCommands<'a> {
    pub(super) fn new(points: &'a [Point], verbs: &'a [Verb]) -> Self {
        Self {
            points,
            verbs,
            point: 0,
            verb: 0,
        }
    }

    #[inline(always)]
    pub(super) fn copy_to(&self, sink: &mut impl PathBuilder) {
        self.copy_to_inner(sink);
    }

    #[inline(always)]
    fn copy_to_inner(&self, sink: &mut impl PathBuilder) -> Option<()> {
        let mut i = 0;
        for verb in self.verbs {
            match verb {
                Verb::MoveTo => {
                    let p = self.points.get(i)?;
                    i += 1;
                    sink.move_to(*p);
                }
                Verb::LineTo => {
                    let p = self.points.get(i)?;
                    i += 1;
                    sink.line_to(*p);
                }
                Verb::QuadTo => {
                    let p = self.points.get(i + 1)?;
                    let c = self.points.get(i)?;
                    i += 2;
                    sink.quad_to(*c, *p);
                }
                Verb::CurveTo => {
                    let p = self.points.get(i + 2)?;
                    let c2 = self.points.get(i + 1)?;
                    let c1 = self.points.get(i)?;
                    i += 3;
                    sink.curve_to(*c1, *c2, *p);
                }
                Verb::Close => {
                    sink.close();
                }
            }
        }
        Some(())
    }
}

impl Iterator for PointsCommands<'_> {
    type Item = Command;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        use Command::*;
        let verb = self.verbs.get(self.verb)?;
        self.verb += 1;
        Some(match verb {
            Verb::MoveTo => {
                let p = self.points.get(self.point)?;
                self.point += 1;
                MoveTo(*p)
            }
            Verb::LineTo => {
                let p = self.points.get(self.point)?;
                self.point += 1;
                LineTo(*p)
            }
            Verb::QuadTo => {
                let p = self.points.get(self.point..self.point + 2)?;
                self.point += 2;
                QuadTo(p[0], p[1])
            }
            Verb::CurveTo => {
                let p = self.points.get(self.point..self.point + 3)?;
                self.point += 3;
                CurveTo(p[0], p[1], p[2])
            }
            Verb::Close => Close,
        })
    }
}

#[derive(Clone)]
pub struct TransformCommands<D> {
    pub data: D,
    pub transform: Transform,
}

impl<D> Iterator for TransformCommands<D>
where
    D: Iterator + Clone,
    D::Item: Borrow<Command>,
{
    type Item = Command;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.data.next()?.borrow().transform(&self.transform))
    }
}
