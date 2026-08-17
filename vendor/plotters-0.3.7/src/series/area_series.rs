use crate::element::{DynElement, IntoDynElement, PathElement, Polygon};
use crate::style::colors::TRANSPARENT;
use crate::style::ShapeStyle;
use plotters_backend::DrawingBackend;

/**
An area series is similar to a line series but uses a filled polygon.
It takes an iterator of data points in guest coordinate system
and creates appropriate lines and points with the given style.

# Example

```
use plotters::prelude::*;
let x_values = [0.0f64, 1., 2., 3., 4.];
let drawing_area = SVGBackend::new("area_series.svg", (300, 200)).into_drawing_area();
drawing_area.fill(&WHITE).unwrap();
let mut chart_builder = ChartBuilder::on(&drawing_area);
chart_builder.margin(10).set_left_and_bottom_label_area_size(20);
let mut chart_context = chart_builder.build_cartesian_2d(0.0..4.0, 0.0..3.0).unwrap();
chart_context.configure_mesh().draw().unwrap();
chart_context.draw_series(AreaSeries::new(x_values.map(|x| (x, 0.3 * x)), 0., BLACK.mix(0.2))).unwrap();
chart_context.draw_series(AreaSeries::new(x_values.map(|x| (x, 2.5 - 0.05 * x * x)), 0., RED.mix(0.2))).unwrap();
chart_context.draw_series(AreaSeries::new(x_values.map(|x| (x, 2. - 0.1 * x * x)), 0., BLUE.mix(0.2)).border_style(BLUE)).unwrap();
```

The result is a chart with three line series; one of them has a highlighted blue border:

![](https://cdn.jsdelivr.net/gh/facorread/plotters-doc-data@b6703f7/apidoc/area_series.svg)
*/
pub struct AreaSeries<DB: DrawingBackend, X: Clone, Y: Clone> {
    area_style: ShapeStyle,
    border_style: ShapeStyle,
    baseline: Y,
    data: Vec<(X, Y)>,
    state: u32,
    _p: std::marker::PhantomData<DB>,
}

impl<DB: DrawingBackend, X: Clone, Y: Clone> AreaSeries<DB, X, Y> {
    /**
    Creates an area series with transparent border.

    See [`AreaSeries`] for more information and examples.
    */
    pub fn new<S: Into<ShapeStyle>, I: IntoIterator<Item = (X, Y)>>(
        iter: I,
        baseline: Y,
        area_style: S,
    ) -> Self {
        Self {
            area_style: area_style.into(),
            baseline,
            data: iter.into_iter().collect(),
            state: 0,
            border_style: (&TRANSPARENT).into(),
            _p: std::marker::PhantomData,
        }
    }

    /**
    Sets the border style of the area series.

    See [`AreaSeries`] for more information and examples.
    */
    pub fn border_style<S: Into<ShapeStyle>>(mut self, style: S) -> Self {
        self.border_style = style.into();
        self
    }
}

impl<DB: DrawingBackend, X: Clone + 'static, Y: Clone + 'static> Iterator for AreaSeries<DB, X, Y> {
    type Item = DynElement<'static, DB, (X, Y)>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.state == 0 {
            let mut data: Vec<_> = self.data.clone();

            if !data.is_empty() {
                data.push((data[data.len() - 1].0.clone(), self.baseline.clone()));
                data.push((data[0].0.clone(), self.baseline.clone()));
            }

            self.state = 1;

            Some(Polygon::new(data, self.area_style).into_dyn())
        } else if self.state == 1 {
            let data: Vec<_> = self.data.clone();

            self.state = 2;

            Some(PathElement::new(data, self.border_style).into_dyn())
        } else {
            None
        }
    }
}
