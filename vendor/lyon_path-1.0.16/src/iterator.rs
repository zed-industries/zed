//! Tools to iterate over paths.
//!
//! # Lyon path iterators
//!
//! ## Overview
//!
//! This module provides a collection of traits to extend the `Iterator` trait when
//! iterating over paths.
//!
//! ## Examples
//!
//! ```
//! use lyon_path::iterator::*;
//! use lyon_path::math::{point, vector};
//! use lyon_path::{Path, PathEvent};
//!
//! // Start with a path.
//! let mut builder = Path::builder();
//! builder.begin(point(0.0, 0.0));
//! builder.line_to(point(10.0, 0.0));
//! builder.cubic_bezier_to(point(10.0, 10.0), point(0.0, 10.0), point(0.0, 5.0));
//! builder.end(true);
//! let path = builder.build();
//!
//! // A simple std::iter::Iterator<PathEvent>,
//! let simple_iter = path.iter();
//!
//! // Make it an iterator over simpler primitives flattened events,
//! // which do not contain any curve. To do so we approximate each curve
//! // linear segments according to a tolerance threshold which controls
//! // the tradeoff between fidelity of the approximation and amount of
//! // generated events. Let's use a tolerance threshold of 0.01.
//! // The beauty of this approach is that the flattening happens lazily
//! // while iterating without allocating memory for the path.
//! let flattened_iter = path.iter().flattened(0.01);
//!
//! for evt in flattened_iter {
//!     match evt {
//!         PathEvent::Begin { at } => { println!(" - move to {:?}", at); }
//!         PathEvent::Line { from, to } => { println!(" - line {:?} -> {:?}", from, to); }
//!         PathEvent::End { last, first, close } => {
//!             if close {
//!                 println!(" - close {:?} -> {:?}", last, first);
//!             } else {
//!                 println!(" - end");
//!             }
//!         }
//!         _ => { panic!() }
//!     }
//! }
//! ```
//!
//! Chaining the provided iterators allow performing some path manipulations lazily
//! without allocating actual path objects to hold the result of the transformations.
//!
//! ```
//! extern crate lyon_path;
//! use lyon_path::iterator::*;
//! use lyon_path::math::{point, Angle, Rotation};
//! use lyon_path::Path;
//!
//! fn main() {
//!     // In practice it is more common to iterate over Path objects than vectors
//!     // of SVG commands (the former can be constructed from the latter).
//!     let mut builder = Path::builder();
//!     builder.begin(point(1.0, 1.0));
//!     builder.line_to(point(2.0, 1.0));
//!     builder.quadratic_bezier_to(point(2.0, 2.0), point(1.0, 2.0));
//!     builder.cubic_bezier_to(point(0.0, 2.0), point(0.0, 0.0), point(1.0, 0.0));
//!     builder.end(true);
//!     let path = builder.build();
//!
//!     let transform = Rotation::new(Angle::radians(1.0));
//!
//!     for evt in path.iter().transformed(&transform).flattened(0.1) {
//!         // ...
//!     }
//! }
//! ```

use crate::geom::traits::Transformation;
use crate::geom::{cubic_bezier, quadratic_bezier, CubicBezierSegment, QuadraticBezierSegment};
use crate::math::*;
use crate::{Attributes, Event, PathEvent};

// TODO: It would be great to add support for attributes in PathIterator.

/// An extension trait for `PathEvent` iterators.
pub trait PathIterator: Iterator<Item = PathEvent> + Sized {
    /// Returns an iterator that turns curves into line segments.
    fn flattened(self, tolerance: f32) -> Flattened<Self> {
        Flattened::new(tolerance, self)
    }

    /// Returns an iterator applying a 2D transform to all of its events.
    fn transformed<T: Transformation<f32>>(self, mat: &T) -> Transformed<Self, T> {
        Transformed::new(mat, self)
    }
}

impl<Iter> PathIterator for Iter where Iter: Iterator<Item = PathEvent> {}

pub struct NoAttributes<Iter>(pub(crate) Iter);

impl<'l, Iter> NoAttributes<Iter>
where
    Iter: Iterator<Item = Event<(Point, Attributes<'l>), Point>>,
{
    pub fn with_attributes(self) -> Iter {
        self.0
    }
}

impl<'l, Iter> Iterator for NoAttributes<Iter>
where
    Iter: Iterator<Item = Event<(Point, Attributes<'l>), Point>>,
{
    type Item = PathEvent;
    fn next(&mut self) -> Option<PathEvent> {
        self.0.next().map(|event| event.with_points())
    }
}

/// An iterator that consumes `Event` iterator and yields flattened path events (with no curves).
pub struct Flattened<Iter> {
    it: Iter,
    current_position: Point,
    current_curve: TmpFlatteningIter,
    tolerance: f32,
}

enum TmpFlatteningIter {
    Quadratic(quadratic_bezier::Flattened<f32>),
    Cubic(cubic_bezier::Flattened<f32>),
    None,
}

impl<Iter: Iterator<Item = PathEvent>> Flattened<Iter> {
    /// Create the iterator.
    pub fn new(tolerance: f32, it: Iter) -> Self {
        Flattened {
            it,
            current_position: point(0.0, 0.0),
            current_curve: TmpFlatteningIter::None,
            tolerance,
        }
    }
}

impl<Iter> Iterator for Flattened<Iter>
where
    Iter: Iterator<Item = PathEvent>,
{
    type Item = PathEvent;
    fn next(&mut self) -> Option<PathEvent> {
        match self.current_curve {
            TmpFlatteningIter::Quadratic(ref mut it) => {
                if let Some(to) = it.next() {
                    let from = self.current_position;
                    self.current_position = to;
                    return Some(PathEvent::Line { from, to });
                }
            }
            TmpFlatteningIter::Cubic(ref mut it) => {
                if let Some(to) = it.next() {
                    let from = self.current_position;
                    self.current_position = to;
                    return Some(PathEvent::Line { from, to });
                }
            }
            _ => {}
        }
        self.current_curve = TmpFlatteningIter::None;
        match self.it.next() {
            Some(PathEvent::Begin { at }) => Some(PathEvent::Begin { at }),
            Some(PathEvent::Line { from, to }) => Some(PathEvent::Line { from, to }),
            Some(PathEvent::End { last, first, close }) => {
                Some(PathEvent::End { last, first, close })
            }
            Some(PathEvent::Quadratic { from, ctrl, to }) => {
                self.current_position = from;
                self.current_curve = TmpFlatteningIter::Quadratic(
                    QuadraticBezierSegment { from, ctrl, to }.flattened(self.tolerance),
                );
                self.next()
            }
            Some(PathEvent::Cubic {
                from,
                ctrl1,
                ctrl2,
                to,
            }) => {
                self.current_position = from;
                self.current_curve = TmpFlatteningIter::Cubic(
                    CubicBezierSegment {
                        from,
                        ctrl1,
                        ctrl2,
                        to,
                    }
                    .flattened(self.tolerance),
                );
                self.next()
            }
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // At minimum, the inner iterator size hint plus the flattening iterator size hint can form the lower
        // bracket.
        // We can't determine a maximum limit.
        let mut lo = self.it.size_hint().0;
        match &self.current_curve {
            TmpFlatteningIter::Quadratic(t) => {
                lo += t.size_hint().0;
            }
            TmpFlatteningIter::Cubic(t) => {
                lo += t.size_hint().0;
            }
            _ => {}
        }
        (lo, None)
    }
}

/// Applies a 2D transform to a path iterator and yields the resulting path iterator.
pub struct Transformed<'l, I, T> {
    it: I,
    transform: &'l T,
}

impl<'l, I, T: Transformation<f32>> Transformed<'l, I, T>
where
    I: Iterator<Item = PathEvent>,
{
    /// Creates a new transformed path iterator from a path iterator.
    #[inline]
    pub fn new(transform: &'l T, it: I) -> Transformed<'l, I, T> {
        Transformed { it, transform }
    }
}

impl<'l, I, T> Iterator for Transformed<'l, I, T>
where
    I: Iterator<Item = PathEvent>,
    T: Transformation<f32>,
{
    type Item = PathEvent;
    fn next(&mut self) -> Option<PathEvent> {
        match self.it.next() {
            None => None,
            Some(ref evt) => Some(evt.transformed(self.transform)),
        }
    }
}

/// An iterator that consumes an iterator of `Point`s and produces `Event`s.
///
/// # Example
///
/// ```
/// # extern crate lyon_path;
/// # use lyon_path::iterator::FromPolyline;
/// # use lyon_path::math::point;
/// # fn main() {
/// let points = [
///     point(1.0, 1.0),
///     point(2.0, 1.0),
///     point(1.0, 2.0)
/// ];
/// let iter = FromPolyline::closed(points.iter().cloned());
/// # }
/// ```
pub struct FromPolyline<Iter> {
    iter: Iter,
    current: Point,
    first: Point,
    is_first: bool,
    done: bool,
    close: bool,
}

impl<Iter: Iterator<Item = Point>> FromPolyline<Iter> {
    pub fn new(close: bool, iter: Iter) -> Self {
        FromPolyline {
            iter,
            current: point(0.0, 0.0),
            first: point(0.0, 0.0),
            is_first: true,
            done: false,
            close,
        }
    }

    pub fn closed(iter: Iter) -> Self {
        FromPolyline::new(true, iter)
    }

    pub fn open(iter: Iter) -> Self {
        FromPolyline::new(false, iter)
    }
}

impl<Iter> Iterator for FromPolyline<Iter>
where
    Iter: Iterator<Item = Point>,
{
    type Item = PathEvent;

    fn next(&mut self) -> Option<PathEvent> {
        if self.done {
            return None;
        }

        if let Some(next) = self.iter.next() {
            debug_assert!(next.x.is_finite());
            debug_assert!(next.y.is_finite());
            let from = self.current;
            self.current = next;
            return if self.is_first {
                self.is_first = false;
                self.first = next;
                Some(PathEvent::Begin { at: next })
            } else {
                Some(PathEvent::Line { from, to: next })
            };
        }

        self.done = true;

        Some(PathEvent::End {
            last: self.current,
            first: self.first,
            close: self.close,
        })
    }
}

#[test]
fn test_from_polyline_open() {
    let points = &[
        point(1.0, 1.0),
        point(3.0, 1.0),
        point(4.0, 5.0),
        point(5.0, 2.0),
    ];

    let mut evts = FromPolyline::open(points.iter().cloned());

    assert_eq!(
        evts.next(),
        Some(PathEvent::Begin {
            at: point(1.0, 1.0)
        })
    );
    assert_eq!(
        evts.next(),
        Some(PathEvent::Line {
            from: point(1.0, 1.0),
            to: point(3.0, 1.0)
        })
    );
    assert_eq!(
        evts.next(),
        Some(PathEvent::Line {
            from: point(3.0, 1.0),
            to: point(4.0, 5.0)
        })
    );
    assert_eq!(
        evts.next(),
        Some(PathEvent::Line {
            from: point(4.0, 5.0),
            to: point(5.0, 2.0)
        })
    );
    assert_eq!(
        evts.next(),
        Some(PathEvent::End {
            last: point(5.0, 2.0),
            first: point(1.0, 1.0),
            close: false
        })
    );
    assert_eq!(evts.next(), None);
}

#[test]
fn test_from_polyline_closed() {
    let points = &[
        point(1.0, 1.0),
        point(3.0, 1.0),
        point(4.0, 5.0),
        point(5.0, 2.0),
    ];

    let mut evts = FromPolyline::closed(points.iter().cloned());

    assert_eq!(
        evts.next(),
        Some(PathEvent::Begin {
            at: point(1.0, 1.0)
        })
    );
    assert_eq!(
        evts.next(),
        Some(PathEvent::Line {
            from: point(1.0, 1.0),
            to: point(3.0, 1.0)
        })
    );
    assert_eq!(
        evts.next(),
        Some(PathEvent::Line {
            from: point(3.0, 1.0),
            to: point(4.0, 5.0)
        })
    );
    assert_eq!(
        evts.next(),
        Some(PathEvent::Line {
            from: point(4.0, 5.0),
            to: point(5.0, 2.0)
        })
    );
    assert_eq!(
        evts.next(),
        Some(PathEvent::End {
            last: point(5.0, 2.0),
            first: point(1.0, 1.0),
            close: true
        })
    );
    assert_eq!(evts.next(), None);
}
