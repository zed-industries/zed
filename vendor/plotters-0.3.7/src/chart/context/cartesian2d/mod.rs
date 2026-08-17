use std::ops::Range;

use plotters_backend::{BackendCoord, DrawingBackend};

use crate::chart::{ChartContext, DualCoordChartContext, MeshStyle};
use crate::coord::{
    cartesian::Cartesian2d,
    ranged1d::{AsRangedCoord, Ranged, ValueFormatter},
    Shift,
};
use crate::drawing::DrawingArea;

mod draw_impl;

impl<'a, DB, XT, YT, X, Y> ChartContext<'a, DB, Cartesian2d<X, Y>>
where
    DB: DrawingBackend,
    X: Ranged<ValueType = XT> + ValueFormatter<XT>,
    Y: Ranged<ValueType = YT> + ValueFormatter<YT>,
{
    pub(crate) fn is_overlapping_drawing_area(
        &self,
        area: Option<&DrawingArea<DB, Shift>>,
    ) -> bool {
        if let Some(area) = area {
            let (x0, y0) = area.get_base_pixel();
            let (w, h) = area.dim_in_pixel();
            let (x1, y1) = (x0 + w as i32, y0 + h as i32);
            let (dx0, dy0) = self.drawing_area.get_base_pixel();
            let (w, h) = self.drawing_area.dim_in_pixel();
            let (dx1, dy1) = (dx0 + w as i32, dy0 + h as i32);

            let (ox0, ox1) = (x0.max(dx0), x1.min(dx1));
            let (oy0, oy1) = (y0.max(dy0), y1.min(dy1));

            ox1 > ox0 && oy1 > oy0
        } else {
            false
        }
    }

    /// Initialize a mesh configuration object and mesh drawing can be finalized by calling
    /// the function `MeshStyle::draw`.
    pub fn configure_mesh(&mut self) -> MeshStyle<'a, '_, X, Y, DB> {
        MeshStyle::new(self)
    }
}

impl<'a, DB: DrawingBackend, X: Ranged, Y: Ranged> ChartContext<'a, DB, Cartesian2d<X, Y>> {
    /// Get the range of X axis
    pub fn x_range(&self) -> Range<X::ValueType> {
        self.drawing_area.get_x_range()
    }

    /// Get range of the Y axis
    pub fn y_range(&self) -> Range<Y::ValueType> {
        self.drawing_area.get_y_range()
    }

    /// Maps the coordinate to the backend coordinate. This is typically used
    /// with an interactive chart.
    pub fn backend_coord(&self, coord: &(X::ValueType, Y::ValueType)) -> BackendCoord {
        self.drawing_area.map_coordinate(coord)
    }
}

impl<'a, DB: DrawingBackend, X: Ranged, Y: Ranged> ChartContext<'a, DB, Cartesian2d<X, Y>> {
    /// Convert this chart context into a dual axis chart context and attach a second coordinate spec
    /// on the chart context. For more detailed information, see documentation for [struct DualCoordChartContext](struct.DualCoordChartContext.html)
    ///
    /// - `x_coord`: The coordinate spec for the X axis
    /// - `y_coord`: The coordinate spec for the Y axis
    /// - **returns** The newly created dual spec chart context
    #[allow(clippy::type_complexity)]
    pub fn set_secondary_coord<SX: AsRangedCoord, SY: AsRangedCoord>(
        self,
        x_coord: SX,
        y_coord: SY,
    ) -> DualCoordChartContext<
        'a,
        DB,
        Cartesian2d<X, Y>,
        Cartesian2d<SX::CoordDescType, SY::CoordDescType>,
    > {
        let mut pixel_range = self.drawing_area.get_pixel_range();
        pixel_range.1 = pixel_range.1.end..pixel_range.1.start;

        DualCoordChartContext::new(self, Cartesian2d::new(x_coord, y_coord, pixel_range))
    }
}
