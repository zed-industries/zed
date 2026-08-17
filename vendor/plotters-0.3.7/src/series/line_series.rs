use crate::element::{
    Circle, DashedPathElement, DottedPathElement, DynElement, IntoDynElement, PathElement,
};
use crate::style::{ShapeStyle, SizeDesc};
use plotters_backend::{BackendCoord, DrawingBackend};
use std::marker::PhantomData;

/**
The line series object, which takes an iterator of data points in guest coordinate system
and creates appropriate lines and points with the given style.

# Example

```
use plotters::prelude::*;
let x_values = [0.0f64, 1., 2., 3., 4.];
let drawing_area = SVGBackend::new("line_series_point_size.svg", (300, 200)).into_drawing_area();
drawing_area.fill(&WHITE).unwrap();
let mut chart_builder = ChartBuilder::on(&drawing_area);
chart_builder.margin(10).set_left_and_bottom_label_area_size(20);
let mut chart_context = chart_builder.build_cartesian_2d(0.0..4.0, 0.0..3.0).unwrap();
chart_context.configure_mesh().draw().unwrap();
chart_context.draw_series(LineSeries::new(x_values.map(|x| (x, 0.3 * x)), BLACK)).unwrap();
chart_context.draw_series(LineSeries::new(x_values.map(|x| (x, 2.5 - 0.05 * x * x)), RED)
    .point_size(5)).unwrap();
chart_context.draw_series(LineSeries::new(x_values.map(|x| (x, 2. - 0.1 * x * x)), BLUE.filled())
    .point_size(4)).unwrap();
```

The result is a chart with three line series; two of them have their data points highlighted:

![](https://cdn.jsdelivr.net/gh/facorread/plotters-doc-data@64e0a28/apidoc/line_series_point_size.svg)
*/
pub struct LineSeries<DB: DrawingBackend, Coord> {
    style: ShapeStyle,
    data: Vec<Coord>,
    point_idx: usize,
    point_size: u32,
    phantom: PhantomData<DB>,
}

impl<DB: DrawingBackend, Coord: Clone + 'static> Iterator for LineSeries<DB, Coord> {
    type Item = DynElement<'static, DB, Coord>;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.data.is_empty() {
            if self.point_size > 0 && self.point_idx < self.data.len() {
                let idx = self.point_idx;
                self.point_idx += 1;
                return Some(
                    Circle::new(self.data[idx].clone(), self.point_size, self.style).into_dyn(),
                );
            }
            let mut data = vec![];
            std::mem::swap(&mut self.data, &mut data);
            Some(PathElement::new(data, self.style).into_dyn())
        } else {
            None
        }
    }
}

impl<DB: DrawingBackend, Coord> LineSeries<DB, Coord> {
    /**
    Creates a new line series based on a data iterator and a given style.

    See [`LineSeries`] for more information and examples.
    */
    pub fn new<I: IntoIterator<Item = Coord>, S: Into<ShapeStyle>>(iter: I, style: S) -> Self {
        Self {
            style: style.into(),
            data: iter.into_iter().collect(),
            point_size: 0,
            point_idx: 0,
            phantom: PhantomData,
        }
    }

    /**
    Sets the size of the points in the series, in pixels.

    See [`LineSeries`] for more information and examples.
    */
    pub fn point_size(mut self, size: u32) -> Self {
        self.point_size = size;
        self
    }
}

/// A dashed line series, map an iterable object to the dashed line element. Can be used to draw simple dashed and dotted lines.
///
/// If you want to use more complex shapes as points in the line, you can use `plotters::series::line_series::DottedLineSeries`.
///
/// # Examples
///
/// Dashed line:
/// ```Rust
/// chart_context
///     .draw_series(DashedLineSeries::new(
///         data_series,
///         5, /* size = length of dash */
///         10, /* spacing */
///         ShapeStyle {
///             color: BLACK.mix(1.0),
///             filled: false,
///             stroke_width: 1,
///         },
///     ))
///     .unwrap();
/// ```
///
/// Dotted line: (keep `size` and `stroke_width` the same to achieve dots)
/// ```Rust
/// chart_context
///     .draw_series(DashedLineSeries::new(
///         data_series,
///         1, /* size = length of dash */
///         4, /* spacing, best to keep this at least 1 larger than size */
///         ShapeStyle {
///             color: BLACK.mix(1.0),
///             filled: false,
///             stroke_width: 1,
///         },
///     ))
///     .unwrap();
/// ```
pub struct DashedLineSeries<I: Iterator + Clone, Size: SizeDesc> {
    points: I,
    size: Size,
    spacing: Size,
    style: ShapeStyle,
}

impl<I: Iterator + Clone, Size: SizeDesc> DashedLineSeries<I, Size> {
    /// Create a new line series from
    /// - `points`: The iterator of the points
    /// - `size`: The dash size
    /// - `spacing`: The dash-to-dash spacing (gap size)
    /// - `style`: The shape style
    /// - returns the created element
    pub fn new<I0>(points: I0, size: Size, spacing: Size, style: ShapeStyle) -> Self
    where
        I0: IntoIterator<IntoIter = I>,
    {
        Self {
            points: points.into_iter(),
            size,
            spacing,
            style,
        }
    }
}

impl<I: Iterator + Clone, Size: SizeDesc> IntoIterator for DashedLineSeries<I, Size> {
    type Item = DashedPathElement<I, Size>;
    type IntoIter = std::iter::Once<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(DashedPathElement::new(
            self.points,
            self.size,
            self.spacing,
            self.style,
        ))
    }
}

/// A dotted line series, map an iterable object to the dotted line element.
pub struct DottedLineSeries<I: Iterator + Clone, Size: SizeDesc, Marker> {
    points: I,
    shift: Size,
    spacing: Size,
    func: Box<dyn Fn(BackendCoord) -> Marker>,
}

impl<I: Iterator + Clone, Size: SizeDesc, Marker> DottedLineSeries<I, Size, Marker> {
    /// Create a new line series from
    /// - `points`: The iterator of the points
    /// - `shift`: The shift of the first marker
    /// - `spacing`: The spacing between markers
    /// - `func`: The marker function
    /// - returns the created element
    pub fn new<I0, F>(points: I0, shift: Size, spacing: Size, func: F) -> Self
    where
        I0: IntoIterator<IntoIter = I>,
        F: Fn(BackendCoord) -> Marker + 'static,
    {
        Self {
            points: points.into_iter(),
            shift,
            spacing,
            func: Box::new(func),
        }
    }
}

impl<I: Iterator + Clone, Size: SizeDesc, Marker: 'static> IntoIterator
    for DottedLineSeries<I, Size, Marker>
{
    type Item = DottedPathElement<I, Size, Marker>;
    type IntoIter = std::iter::Once<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(DottedPathElement::new(
            self.points,
            self.shift,
            self.spacing,
            self.func,
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    #[test]
    fn test_line_series() {
        let drawing_area = create_mocked_drawing_area(200, 200, |m| {
            m.check_draw_path(|c, s, _path| {
                assert_eq!(c, RED.to_rgba());
                assert_eq!(s, 3);
                // TODO when cleanup the backend coordinate definition, then we uncomment the
                // following check
                //for i in 0..100 {
                //    assert_eq!(path[i], (i as i32 * 2, 199 - i as i32 * 2));
                //}
            });

            m.drop_check(|b| {
                assert_eq!(b.num_draw_path_call, 8);
                assert_eq!(b.draw_count, 27);
            });
        });

        let mut chart = ChartBuilder::on(&drawing_area)
            .build_cartesian_2d(0..100, 0..100)
            .expect("Build chart error");

        chart
            .draw_series(LineSeries::new(
                (0..100).map(|x| (x, x)),
                Into::<ShapeStyle>::into(RED).stroke_width(3),
            ))
            .expect("Drawing Error");
        chart
            .draw_series(DashedLineSeries::new(
                (0..=50).map(|x| (0, x)),
                10,
                5,
                Into::<ShapeStyle>::into(RED).stroke_width(3),
            ))
            .expect("Drawing Error");
        let mk_f = |c| Circle::new(c, 3, Into::<ShapeStyle>::into(RED).filled());
        chart
            .draw_series(DottedLineSeries::new((0..=50).map(|x| (x, 0)), 5, 5, mk_f))
            .expect("Drawing Error");
    }
}
