use std::marker::PhantomData;

use crate::data::Quartiles;
use crate::element::{Drawable, PointCollection};
use crate::style::{Color, ShapeStyle, BLACK};
use plotters_backend::{BackendCoord, DrawingBackend, DrawingErrorKind};

/// The boxplot orientation trait
pub trait BoxplotOrient<K, V> {
    type XType;
    type YType;

    fn make_coord(key: K, val: V) -> (Self::XType, Self::YType);
    fn with_offset(coord: BackendCoord, offset: f64) -> BackendCoord;
}

/// The vertical boxplot phantom
pub struct BoxplotOrientV<K, V>(PhantomData<(K, V)>);

/// The horizontal boxplot phantom
pub struct BoxplotOrientH<K, V>(PhantomData<(K, V)>);

impl<K, V> BoxplotOrient<K, V> for BoxplotOrientV<K, V> {
    type XType = K;
    type YType = V;

    fn make_coord(key: K, val: V) -> (K, V) {
        (key, val)
    }

    fn with_offset(coord: BackendCoord, offset: f64) -> BackendCoord {
        (coord.0 + offset as i32, coord.1)
    }
}

impl<K, V> BoxplotOrient<K, V> for BoxplotOrientH<K, V> {
    type XType = V;
    type YType = K;

    fn make_coord(key: K, val: V) -> (V, K) {
        (val, key)
    }

    fn with_offset(coord: BackendCoord, offset: f64) -> BackendCoord {
        (coord.0, coord.1 + offset as i32)
    }
}

const DEFAULT_WIDTH: u32 = 10;

/// The boxplot element
pub struct Boxplot<K, O: BoxplotOrient<K, f32>> {
    style: ShapeStyle,
    width: u32,
    whisker_width: f64,
    offset: f64,
    key: K,
    values: [f32; 5],
    _p: PhantomData<O>,
}

impl<K: Clone> Boxplot<K, BoxplotOrientV<K, f32>> {
    /// Create a new vertical boxplot element.
    ///
    /// - `key`: The key (the X axis value)
    /// - `quartiles`: The quartiles values for the Y axis
    /// - **returns** The newly created boxplot element
    ///
    /// ```rust
    /// use plotters::prelude::*;
    ///
    /// let quartiles = Quartiles::new(&[7, 15, 36, 39, 40, 41]);
    /// let plot = Boxplot::new_vertical("group", &quartiles);
    /// ```
    pub fn new_vertical(key: K, quartiles: &Quartiles) -> Self {
        Self {
            style: Into::<ShapeStyle>::into(BLACK),
            width: DEFAULT_WIDTH,
            whisker_width: 1.0,
            offset: 0.0,
            key,
            values: quartiles.values(),
            _p: PhantomData,
        }
    }
}

impl<K: Clone> Boxplot<K, BoxplotOrientH<K, f32>> {
    /// Create a new horizontal boxplot element.
    ///
    /// - `key`: The key (the Y axis value)
    /// - `quartiles`: The quartiles values for the X axis
    /// - **returns** The newly created boxplot element
    ///
    /// ```rust
    /// use plotters::prelude::*;
    ///
    /// let quartiles = Quartiles::new(&[7, 15, 36, 39, 40, 41]);
    /// let plot = Boxplot::new_horizontal("group", &quartiles);
    /// ```
    pub fn new_horizontal(key: K, quartiles: &Quartiles) -> Self {
        Self {
            style: Into::<ShapeStyle>::into(BLACK),
            width: DEFAULT_WIDTH,
            whisker_width: 1.0,
            offset: 0.0,
            key,
            values: quartiles.values(),
            _p: PhantomData,
        }
    }
}

impl<K, O: BoxplotOrient<K, f32>> Boxplot<K, O> {
    /// Set the style of the boxplot.
    ///
    /// - `S`: The required style
    /// - **returns** The up-to-dated boxplot element
    ///
    /// ```rust
    /// use plotters::prelude::*;
    ///
    /// let quartiles = Quartiles::new(&[7, 15, 36, 39, 40, 41]);
    /// let plot = Boxplot::new_horizontal("group", &quartiles).style(&BLUE);
    /// ```
    pub fn style<S: Into<ShapeStyle>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    /// Set the bar width.
    ///
    /// - `width`: The required width
    /// - **returns** The up-to-dated boxplot element
    ///
    /// ```rust
    /// use plotters::prelude::*;
    ///
    /// let quartiles = Quartiles::new(&[7, 15, 36, 39, 40, 41]);
    /// let plot = Boxplot::new_horizontal("group", &quartiles).width(10);
    /// ```
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    /// Set the width of the whiskers as a fraction of the bar width.
    ///
    /// - `whisker_width`: The required fraction
    /// - **returns** The up-to-dated boxplot element
    ///
    /// ```rust
    /// use plotters::prelude::*;
    ///
    /// let quartiles = Quartiles::new(&[7, 15, 36, 39, 40, 41]);
    /// let plot = Boxplot::new_horizontal("group", &quartiles).whisker_width(0.5);
    /// ```
    pub fn whisker_width(mut self, whisker_width: f64) -> Self {
        self.whisker_width = whisker_width;
        self
    }

    /// Set the element offset on the key axis.
    ///
    /// - `offset`: The required offset (on the X axis for vertical, on the Y axis for horizontal)
    /// - **returns** The up-to-dated boxplot element
    ///
    /// ```rust
    /// use plotters::prelude::*;
    ///
    /// let quartiles = Quartiles::new(&[7, 15, 36, 39, 40, 41]);
    /// let plot = Boxplot::new_horizontal("group", &quartiles).offset(-5);
    /// ```
    pub fn offset<T: Into<f64> + Copy>(mut self, offset: T) -> Self {
        self.offset = offset.into();
        self
    }
}

impl<'a, K: Clone, O: BoxplotOrient<K, f32>> PointCollection<'a, (O::XType, O::YType)>
    for &'a Boxplot<K, O>
{
    type Point = (O::XType, O::YType);
    type IntoIter = Vec<Self::Point>;
    fn point_iter(self) -> Self::IntoIter {
        self.values
            .iter()
            .map(|v| O::make_coord(self.key.clone(), *v))
            .collect()
    }
}

impl<K, DB: DrawingBackend, O: BoxplotOrient<K, f32>> Drawable<DB> for Boxplot<K, O> {
    fn draw<I: Iterator<Item = BackendCoord>>(
        &self,
        points: I,
        backend: &mut DB,
        _: (u32, u32),
    ) -> Result<(), DrawingErrorKind<DB::ErrorType>> {
        let points: Vec<_> = points.take(5).collect();
        if points.len() == 5 {
            let width = f64::from(self.width);
            let moved = |coord| O::with_offset(coord, self.offset);
            let start_bar = |coord| O::with_offset(moved(coord), -width / 2.0);
            let end_bar = |coord| O::with_offset(moved(coord), width / 2.0);
            let start_whisker =
                |coord| O::with_offset(moved(coord), -width * self.whisker_width / 2.0);
            let end_whisker =
                |coord| O::with_offset(moved(coord), width * self.whisker_width / 2.0);

            // |---[   |  ]----|
            // ^________________
            backend.draw_line(
                start_whisker(points[0]),
                end_whisker(points[0]),
                &self.style,
            )?;

            // |---[   |  ]----|
            // _^^^_____________

            backend.draw_line(
                moved(points[0]),
                moved(points[1]),
                &self.style.color.to_backend_color(),
            )?;

            // |---[   |  ]----|
            // ____^______^_____
            let corner1 = start_bar(points[3]);
            let corner2 = end_bar(points[1]);
            let upper_left = (corner1.0.min(corner2.0), corner1.1.min(corner2.1));
            let bottom_right = (corner1.0.max(corner2.0), corner1.1.max(corner2.1));
            backend.draw_rect(upper_left, bottom_right, &self.style, false)?;

            // |---[   |  ]----|
            // ________^________
            backend.draw_line(start_bar(points[2]), end_bar(points[2]), &self.style)?;

            // |---[   |  ]----|
            // ____________^^^^_
            backend.draw_line(moved(points[3]), moved(points[4]), &self.style)?;

            // |---[   |  ]----|
            // ________________^
            backend.draw_line(
                start_whisker(points[4]),
                end_whisker(points[4]),
                &self.style,
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_draw_v() {
        let root = MockedBackend::new(1024, 768).into_drawing_area();
        let chart = ChartBuilder::on(&root)
            .build_cartesian_2d(0..2, 0f32..100f32)
            .unwrap();

        let values = Quartiles::new(&[6]);
        assert!(chart
            .plotting_area()
            .draw(&Boxplot::new_vertical(1, &values))
            .is_ok());
    }

    #[test]
    fn test_draw_h() {
        let root = MockedBackend::new(1024, 768).into_drawing_area();
        let chart = ChartBuilder::on(&root)
            .build_cartesian_2d(0f32..100f32, 0..2)
            .unwrap();

        let values = Quartiles::new(&[6]);
        assert!(chart
            .plotting_area()
            .draw(&Boxplot::new_horizontal(1, &values))
            .is_ok());
    }
}
