/*!
 The 2-dimensional cartesian coordinate system.

 This module provides the 2D cartesian coordinate system, which is composed by two independent
 ranged 1D coordinate specification.

 This type of coordinate system is used by the chart constructed with [ChartBuilder::build_cartesian_2d](../../chart/ChartBuilder.html#method.build_cartesian_2d).
*/

use crate::coord::ranged1d::{KeyPointHint, Ranged, ReversibleRanged};
use crate::coord::{CoordTranslate, ReverseCoordTranslate};

use crate::style::ShapeStyle;
use plotters_backend::{BackendCoord, DrawingBackend, DrawingErrorKind};

use std::ops::Range;

/// A 2D Cartesian coordinate system described by two 1D ranged coordinate specs.
#[derive(Clone)]
pub struct Cartesian2d<X: Ranged, Y: Ranged> {
    logic_x: X,
    logic_y: Y,
    back_x: (i32, i32),
    back_y: (i32, i32),
}

impl<X: Ranged, Y: Ranged> Cartesian2d<X, Y> {
    /// Create a new 2D cartesian coordinate system
    /// - `logic_x` and `logic_y` : The description for the 1D coordinate system
    /// - `actual`: The pixel range on the screen for this coordinate system
    pub fn new<IntoX: Into<X>, IntoY: Into<Y>>(
        logic_x: IntoX,
        logic_y: IntoY,
        actual: (Range<i32>, Range<i32>),
    ) -> Self {
        Self {
            logic_x: logic_x.into(),
            logic_y: logic_y.into(),
            back_x: (actual.0.start, actual.0.end),
            back_y: (actual.1.start, actual.1.end),
        }
    }

    /// Draw the mesh for the coordinate system
    pub fn draw_mesh<
        E,
        DrawMesh: FnMut(MeshLine<X, Y>) -> Result<(), E>,
        XH: KeyPointHint,
        YH: KeyPointHint,
    >(
        &self,
        h_limit: YH,
        v_limit: XH,
        mut draw_mesh: DrawMesh,
    ) -> Result<(), E> {
        let (xkp, ykp) = (
            self.logic_x.key_points(v_limit),
            self.logic_y.key_points(h_limit),
        );

        for logic_x in xkp {
            let x = self.logic_x.map(&logic_x, self.back_x);
            draw_mesh(MeshLine::XMesh(
                (x, self.back_y.0),
                (x, self.back_y.1),
                &logic_x,
            ))?;
        }

        for logic_y in ykp {
            let y = self.logic_y.map(&logic_y, self.back_y);
            draw_mesh(MeshLine::YMesh(
                (self.back_x.0, y),
                (self.back_x.1, y),
                &logic_y,
            ))?;
        }

        Ok(())
    }

    /// Get the range of X axis
    pub fn get_x_range(&self) -> Range<X::ValueType> {
        self.logic_x.range()
    }

    /// Get the range of Y axis
    pub fn get_y_range(&self) -> Range<Y::ValueType> {
        self.logic_y.range()
    }

    /// Get the horizental backend coordinate range where X axis should be drawn
    pub fn get_x_axis_pixel_range(&self) -> Range<i32> {
        self.logic_x.axis_pixel_range(self.back_x)
    }

    /// Get the vertical backend coordinate range where Y axis should be drawn
    pub fn get_y_axis_pixel_range(&self) -> Range<i32> {
        self.logic_y.axis_pixel_range(self.back_y)
    }

    /// Get the 1D coordinate spec for X axis
    pub fn x_spec(&self) -> &X {
        &self.logic_x
    }

    /// Get the 1D coordinate spec for Y axis
    pub fn y_spec(&self) -> &Y {
        &self.logic_y
    }
}

impl<X: Ranged, Y: Ranged> CoordTranslate for Cartesian2d<X, Y> {
    type From = (X::ValueType, Y::ValueType);

    fn translate(&self, from: &Self::From) -> BackendCoord {
        (
            self.logic_x.map(&from.0, self.back_x),
            self.logic_y.map(&from.1, self.back_y),
        )
    }
}

impl<X: ReversibleRanged, Y: ReversibleRanged> ReverseCoordTranslate for Cartesian2d<X, Y> {
    fn reverse_translate(&self, input: BackendCoord) -> Option<Self::From> {
        Some((
            self.logic_x.unmap(input.0, self.back_x)?,
            self.logic_y.unmap(input.1, self.back_y)?,
        ))
    }
}

/// Represent a coordinate mesh for the two ranged value coordinate system
pub enum MeshLine<'a, X: Ranged, Y: Ranged> {
    /// Used to plot the horizontal lines of the mesh
    XMesh(BackendCoord, BackendCoord, &'a X::ValueType),
    /// Used to plot the vertical lines of the mesh
    YMesh(BackendCoord, BackendCoord, &'a Y::ValueType),
}

impl<'a, X: Ranged, Y: Ranged> MeshLine<'a, X, Y> {
    /// Draw a single mesh line onto the backend
    pub fn draw<DB: DrawingBackend>(
        &self,
        backend: &mut DB,
        style: &ShapeStyle,
    ) -> Result<(), DrawingErrorKind<DB::ErrorType>> {
        let (&left, &right) = match self {
            MeshLine::XMesh(a, b, _) => (a, b),
            MeshLine::YMesh(a, b, _) => (a, b),
        };
        backend.draw_line(left, right, style)
    }
}
