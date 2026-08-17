use std::marker::PhantomData;

use crate::element::{Drawable, PointCollection};
use crate::style::ShapeStyle;
use plotters_backend::{BackendCoord, DrawingBackend, DrawingErrorKind};

/**
Used to reuse code between horizontal and vertical error bars.

This is used internally by Plotters and should probably not be included in user code.
See [`ErrorBar`] for more information and examples.
*/
pub trait ErrorBarOrient<K, V> {
    type XType;
    type YType;

    fn make_coord(key: K, val: V) -> (Self::XType, Self::YType);
    fn ending_coord(coord: BackendCoord, w: u32) -> (BackendCoord, BackendCoord);
}

/**
Used for the production of horizontal error bars.

This is used internally by Plotters and should probably not be included in user code.
See [`ErrorBar`] for more information and examples.
*/
pub struct ErrorBarOrientH<K, V>(PhantomData<(K, V)>);

/**
Used for the production of vertical error bars.

This is used internally by Plotters and should probably not be included in user code.
See [`ErrorBar`] for more information and examples.
*/
pub struct ErrorBarOrientV<K, V>(PhantomData<(K, V)>);

impl<K, V> ErrorBarOrient<K, V> for ErrorBarOrientH<K, V> {
    type XType = V;
    type YType = K;

    fn make_coord(key: K, val: V) -> (V, K) {
        (val, key)
    }

    fn ending_coord(coord: BackendCoord, w: u32) -> (BackendCoord, BackendCoord) {
        (
            (coord.0, coord.1 - w as i32 / 2),
            (coord.0, coord.1 + w as i32 / 2),
        )
    }
}

impl<K, V> ErrorBarOrient<K, V> for ErrorBarOrientV<K, V> {
    type XType = K;
    type YType = V;

    fn make_coord(key: K, val: V) -> (K, V) {
        (key, val)
    }

    fn ending_coord(coord: BackendCoord, w: u32) -> (BackendCoord, BackendCoord) {
        (
            (coord.0 - w as i32 / 2, coord.1),
            (coord.0 + w as i32 / 2, coord.1),
        )
    }
}

/**
An error bar, which visualizes the minimum, average, and maximum of a dataset.

Unlike [`crate::series::Histogram`], the `ErrorBar` code does not classify or aggregate data.
These operations must be done before building error bars.

# Examples

```
use plotters::prelude::*;
let data = [(1.0, 3.3), (2., 2.1), (3., 1.5), (4., 1.9), (5., 1.0)];
let drawing_area = SVGBackend::new("error_bars_vertical.svg", (300, 200)).into_drawing_area();
drawing_area.fill(&WHITE).unwrap();
let mut chart_builder = ChartBuilder::on(&drawing_area);
chart_builder.margin(10).set_left_and_bottom_label_area_size(20);
let mut chart_context = chart_builder.build_cartesian_2d(0.0..6.0, 0.0..6.0).unwrap();
chart_context.configure_mesh().draw().unwrap();
chart_context.draw_series(data.map(|(x, y)| {
    ErrorBar::new_vertical(x, y - 0.4, y, y + 0.3, BLUE.filled(), 10)
})).unwrap();
chart_context.draw_series(data.map(|(x, y)| {
    ErrorBar::new_vertical(x, y + 1.0, y + 1.9, y + 2.4, RED, 10)
})).unwrap();
```

This code produces two series of five error bars each, showing minima, maxima, and average values:

![](https://cdn.jsdelivr.net/gh/facorread/plotters-doc-data@06d370f/apidoc/error_bars_vertical.svg)

[`ErrorBar::new_vertical()`] is used to create vertical error bars. Here is an example using
[`ErrorBar::new_horizontal()`] instead:

![](https://cdn.jsdelivr.net/gh/facorread/plotters-doc-data@06d370f/apidoc/error_bars_horizontal.svg)
*/
pub struct ErrorBar<K, V, O: ErrorBarOrient<K, V>> {
    style: ShapeStyle,
    width: u32,
    key: K,
    values: [V; 3],
    _p: PhantomData<O>,
}

impl<K, V> ErrorBar<K, V, ErrorBarOrientV<K, V>> {
    /**
    Creates a vertical error bar.
    `
    - `key`: Horizontal position of the bar
    - `min`: Minimum of the data
    - `avg`: Average of the data
    - `max`: Maximum of the data
    - `style`: Color, transparency, and fill of the error bar. See [`ShapeStyle`] for more information and examples.
    - `width`: Width of the error marks in backend coordinates.

    See [`ErrorBar`] for more information and examples.
    */
    pub fn new_vertical<S: Into<ShapeStyle>>(
        key: K,
        min: V,
        avg: V,
        max: V,
        style: S,
        width: u32,
    ) -> Self {
        Self {
            style: style.into(),
            width,
            key,
            values: [min, avg, max],
            _p: PhantomData,
        }
    }
}

impl<K, V> ErrorBar<K, V, ErrorBarOrientH<K, V>> {
    /**
    Creates a horizontal error bar.

    - `key`: Vertical position of the bar
    - `min`: Minimum of the data
    - `avg`: Average of the data
    - `max`: Maximum of the data
    - `style`: Color, transparency, and fill of the error bar. See [`ShapeStyle`] for more information and examples.
    - `width`: Width of the error marks in backend coordinates.

    See [`ErrorBar`] for more information and examples.
    */
    pub fn new_horizontal<S: Into<ShapeStyle>>(
        key: K,
        min: V,
        avg: V,
        max: V,
        style: S,
        width: u32,
    ) -> Self {
        Self {
            style: style.into(),
            width,
            key,
            values: [min, avg, max],
            _p: PhantomData,
        }
    }
}

impl<'a, K: Clone, V: Clone, O: ErrorBarOrient<K, V>> PointCollection<'a, (O::XType, O::YType)>
    for &'a ErrorBar<K, V, O>
{
    type Point = (O::XType, O::YType);
    type IntoIter = Vec<Self::Point>;
    fn point_iter(self) -> Self::IntoIter {
        self.values
            .iter()
            .map(|v| O::make_coord(self.key.clone(), v.clone()))
            .collect()
    }
}

impl<K, V, O: ErrorBarOrient<K, V>, DB: DrawingBackend> Drawable<DB> for ErrorBar<K, V, O> {
    fn draw<I: Iterator<Item = BackendCoord>>(
        &self,
        points: I,
        backend: &mut DB,
        _: (u32, u32),
    ) -> Result<(), DrawingErrorKind<DB::ErrorType>> {
        let points: Vec<_> = points.take(3).collect();

        let (from, to) = O::ending_coord(points[0], self.width);
        backend.draw_line(from, to, &self.style)?;

        let (from, to) = O::ending_coord(points[2], self.width);
        backend.draw_line(from, to, &self.style)?;

        backend.draw_line(points[0], points[2], &self.style)?;

        backend.draw_circle(points[1], self.width / 2, &self.style, self.style.filled)?;

        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_preserve_stroke_width() {
    let v = ErrorBar::new_vertical(100, 20, 50, 70, WHITE.filled().stroke_width(5), 3);
    let h = ErrorBar::new_horizontal(100, 20, 50, 70, WHITE.filled().stroke_width(5), 3);

    use crate::prelude::*;
    let da = crate::create_mocked_drawing_area(300, 300, |m| {
        m.check_draw_line(|_, w, _, _| {
            assert_eq!(w, 5);
        });
    });
    da.draw(&h).expect("Drawing Failure");
    da.draw(&v).expect("Drawing Failure");
}
