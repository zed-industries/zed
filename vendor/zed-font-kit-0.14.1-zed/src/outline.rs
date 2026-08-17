// font-kit/src/outline.rs
//
// Copyright © 2020 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Bézier paths.

use pathfinder_geometry::line_segment::LineSegment2F;
use pathfinder_geometry::vector::Vector2F;
use std::mem;

/// Receives Bézier path rendering commands.
pub trait OutlineSink {
    /// Moves the pen to a point.
    fn move_to(&mut self, to: Vector2F);
    /// Draws a line to a point.
    fn line_to(&mut self, to: Vector2F);
    /// Draws a quadratic Bézier curve to a point.
    fn quadratic_curve_to(&mut self, ctrl: Vector2F, to: Vector2F);
    /// Draws a cubic Bézier curve to a point.
    fn cubic_curve_to(&mut self, ctrl: LineSegment2F, to: Vector2F);
    /// Closes the path, returning to the first point in it.
    fn close(&mut self);
}

/// A glyph vector outline or path.
#[derive(Clone, PartialEq, Debug)]
pub struct Outline {
    /// The individual subpaths that make up this outline.
    pub contours: Vec<Contour>,
}

/// A single curve or subpath within a glyph outline.
#[derive(Clone, PartialEq, Debug)]
pub struct Contour {
    /// Positions of each point.
    ///
    /// This must have the same length as the `flags` field.
    pub positions: Vec<Vector2F>,
    /// Flags that specify what type of point the corresponding position represents.
    ///
    /// This must have the same length as the `positions` field.
    pub flags: Vec<PointFlags>,
}

bitflags! {
    /// Flags that specify what type of point the corresponding position represents.
    #[derive(Clone, Debug, PartialEq)]
    pub struct PointFlags: u8 {
        /// This point is the control point of a quadratic Bézier curve or the first control point
        /// of a cubic Bézier curve.
        ///
        /// This flag is mutually exclusive with `CONTROL_POINT_1`.
        const CONTROL_POINT_0 = 0x01;
        /// This point is the second control point of a cubic Bézier curve.
        ///
        /// This flag is mutually exclusive with `CONTROL_POINT_0`.
        const CONTROL_POINT_1 = 0x02;
    }
}

/// Accumulates Bézier path rendering commands into an `Outline` structure.
#[derive(Clone, Debug)]
pub struct OutlineBuilder {
    outline: Outline,
    current_contour: Contour,
}

impl Default for Outline {
    fn default() -> Self {
        Self::new()
    }
}

impl Outline {
    /// Creates a new empty outline.
    #[inline]
    pub fn new() -> Outline {
        Outline { contours: vec![] }
    }

    /// Sends this outline to an `OutlineSink`.
    pub fn copy_to<S>(&self, sink: &mut S)
    where
        S: OutlineSink,
    {
        for contour in &self.contours {
            contour.copy_to(sink);
        }
    }
}

impl Default for Contour {
    fn default() -> Self {
        Self::new()
    }
}

impl Contour {
    /// Creates a new empty contour.
    #[inline]
    pub fn new() -> Contour {
        Contour {
            positions: vec![],
            flags: vec![],
        }
    }

    /// Adds a new point with the given flags to the contour.
    #[inline]
    pub fn push(&mut self, position: Vector2F, flags: PointFlags) {
        self.positions.push(position);
        self.flags.push(flags);
    }

    /// Sends this contour to an `OutlineSink`.
    pub fn copy_to<S>(&self, sink: &mut S)
    where
        S: OutlineSink,
    {
        debug_assert_eq!(self.positions.len(), self.flags.len());
        if self.positions.is_empty() {
            return;
        }
        sink.move_to(self.positions[0]);

        let mut iter = self.positions[1..].iter().zip(self.flags[1..].iter());
        while let Some((&position_0, flags_0)) = iter.next() {
            if flags_0.is_empty() {
                sink.line_to(position_0);
                continue;
            }

            let (&position_1, flags_1) = iter.next().expect("Invalid outline!");
            if flags_1.is_empty() {
                sink.quadratic_curve_to(position_0, position_1);
                continue;
            }

            let (&position_2, flags_2) = iter.next().expect("Invalid outline!");
            debug_assert!(flags_2.is_empty());
            sink.cubic_curve_to(LineSegment2F::new(position_0, position_1), position_2);
        }

        sink.close();
    }
}

impl Default for OutlineBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl OutlineBuilder {
    /// Creates a new empty `OutlineBuilder`.
    #[inline]
    pub fn new() -> OutlineBuilder {
        OutlineBuilder {
            outline: Outline::new(),
            current_contour: Contour::new(),
        }
    }

    /// Consumes this outline builder and returns the resulting outline.
    #[inline]
    pub fn into_outline(self) -> Outline {
        self.outline
    }

    /// Resets the outline builder and returns the old outline.
    #[inline]
    pub fn take_outline(&mut self) -> Outline {
        assert!(self.current_contour.positions.is_empty());
        self.current_contour = Contour::new();
        mem::replace(&mut self.outline, Outline::new())
    }
}

impl OutlineSink for OutlineBuilder {
    #[inline]
    fn move_to(&mut self, to: Vector2F) {
        self.current_contour.push(to, PointFlags::empty());
    }

    #[inline]
    fn line_to(&mut self, to: Vector2F) {
        self.current_contour.push(to, PointFlags::empty());
    }

    #[inline]
    fn quadratic_curve_to(&mut self, ctrl: Vector2F, to: Vector2F) {
        self.current_contour.push(ctrl, PointFlags::CONTROL_POINT_0);
        self.current_contour.push(to, PointFlags::empty());
    }

    #[inline]
    fn cubic_curve_to(&mut self, ctrl: LineSegment2F, to: Vector2F) {
        self.current_contour
            .push(ctrl.from(), PointFlags::CONTROL_POINT_0);
        self.current_contour
            .push(ctrl.to(), PointFlags::CONTROL_POINT_1);
        self.current_contour.push(to, PointFlags::empty());
    }

    #[inline]
    fn close(&mut self) {
        self.outline
            .contours
            .push(mem::replace(&mut self.current_contour, Contour::new()));
    }
}
