use crate::element::Polygon;
use crate::style::{colors::BLUE, Color, ShapeStyle};
use std::marker::PhantomData;

/// Any type that describe a surface orientation
pub trait Direction<X, Y, Z> {
    /// The type for the first input argument
    type Input1Type;
    /// The type for the second input argument
    type Input2Type;
    /// The output of the surface function
    type OutputType;

    /// The function that maps a point on surface into the coordinate system
    fn make_coord(
        free_vars: (Self::Input1Type, Self::Input2Type),
        result: Self::OutputType,
    ) -> (X, Y, Z);
}

macro_rules! define_panel_descriptor {
    ($name: ident, $var1: ident, $var2: ident, $out: ident, ($first: ident, $second:ident) -> $result: ident = $output: expr) => {
        #[allow(clippy::upper_case_acronyms)]
        pub struct $name;
        impl<X, Y, Z> Direction<X, Y, Z> for $name {
            type Input1Type = $var1;
            type Input2Type = $var2;
            type OutputType = $out;
            fn make_coord(
                ($first, $second): (Self::Input1Type, Self::Input2Type),
                $result: Self::OutputType,
            ) -> (X, Y, Z) {
                $output
            }
        }
    };
}

define_panel_descriptor!(XOY, X, Y, Z, (x, y) -> z = (x,y,z));
define_panel_descriptor!(XOZ, X, Z, Y, (x, z) -> y = (x,y,z));
define_panel_descriptor!(YOZ, Y, Z, X, (y, z) -> x = (x,y,z));

enum StyleConfig<'a, T> {
    Fixed(ShapeStyle),
    Function(&'a dyn Fn(&T) -> ShapeStyle),
}

impl<T> StyleConfig<'_, T> {
    fn get_style(&self, v: &T) -> ShapeStyle {
        match self {
            StyleConfig::Fixed(s) => *s,
            StyleConfig::Function(f) => f(v),
        }
    }
}

/**
Represents functions of two variables.

# Examples

```
use plotters::prelude::*;
let drawing_area = SVGBackend::new("surface_series_xoz.svg", (640, 480)).into_drawing_area();
drawing_area.fill(&WHITE).unwrap();
let mut chart_context = ChartBuilder::on(&drawing_area)
    .margin(10)
    .build_cartesian_3d(-3.0..3.0f64, -3.0..3.0f64, -3.0..3.0f64)
    .unwrap();
chart_context.configure_axes().draw().unwrap();
let axis_title_style = ("sans-serif", 20, &BLACK).into_text_style(&drawing_area);
chart_context.draw_series([("x", (3., -3., -3.)), ("y", (-3., 3., -3.)), ("z", (-3., -3., 3.))]
.map(|(label, position)| Text::new(label, position, &axis_title_style))).unwrap();
chart_context.draw_series(SurfaceSeries::xoz(
    (-30..30).map(|v| v as f64 / 10.0),
    (-30..30).map(|v| v as f64 / 10.0),
    |x:f64,z:f64|(0.7 * (x * x + z * z)).cos()).style(&BLUE.mix(0.5))
).unwrap();
```

The code above with [`SurfaceSeries::xoy()`] produces a surface that depends on x and y and
points in the z direction:

![](https://cdn.jsdelivr.net/gh/facorread/plotters-doc-data@10ace42/apidoc/surface_series_xoy.svg)

The code above with [`SurfaceSeries::xoz()`] produces a surface that depends on x and z and
points in the y direction:

![](https://cdn.jsdelivr.net/gh/facorread/plotters-doc-data@10ace42/apidoc/surface_series_xoz.svg)

The code above with [`SurfaceSeries::yoz()`] produces a surface that depends on y and z and
points in the x direction:

![](https://cdn.jsdelivr.net/gh/facorread/plotters-doc-data@10ace42/apidoc/surface_series_yoz.svg)
*/
pub struct SurfaceSeries<'a, X, Y, Z, D, SurfaceFunc>
where
    D: Direction<X, Y, Z>,
    SurfaceFunc: Fn(D::Input1Type, D::Input2Type) -> D::OutputType,
{
    free_var_1: Vec<D::Input1Type>,
    free_var_2: Vec<D::Input2Type>,
    surface_f: SurfaceFunc,
    style: StyleConfig<'a, D::OutputType>,
    vidx_1: usize,
    vidx_2: usize,
    _phantom: PhantomData<(X, Y, Z, D)>,
}

impl<'a, X, Y, Z, D, SurfaceFunc> SurfaceSeries<'a, X, Y, Z, D, SurfaceFunc>
where
    D: Direction<X, Y, Z>,
    SurfaceFunc: Fn(D::Input1Type, D::Input2Type) -> D::OutputType,
{
    /// Create a new surface series, the surface orientation is determined by D  
    pub fn new<IterA: Iterator<Item = D::Input1Type>, IterB: Iterator<Item = D::Input2Type>>(
        first_iter: IterA,
        second_iter: IterB,
        func: SurfaceFunc,
    ) -> Self {
        Self {
            free_var_1: first_iter.collect(),
            free_var_2: second_iter.collect(),
            surface_f: func,
            style: StyleConfig::Fixed(BLUE.mix(0.4).filled()),
            vidx_1: 0,
            vidx_2: 0,
            _phantom: PhantomData,
        }
    }

    /**
    Sets the style as a function of the value of the dependent coordinate of the surface.

    # Examples

    ```
    use plotters::prelude::*;
    let drawing_area = SVGBackend::new("surface_series_style_func.svg", (640, 480)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();
    let mut chart_context = ChartBuilder::on(&drawing_area)
        .margin(10)
        .build_cartesian_3d(-3.0..3.0f64, -3.0..3.0f64, -3.0..3.0f64)
        .unwrap();
    chart_context.configure_axes().draw().unwrap();
    let axis_title_style = ("sans-serif", 20, &BLACK).into_text_style(&drawing_area);
    chart_context.draw_series([("x", (3., -3., -3.)), ("y", (-3., 3., -3.)), ("z", (-3., -3., 3.))]
    .map(|(label, position)| Text::new(label, position, &axis_title_style))).unwrap();
    chart_context.draw_series(SurfaceSeries::xoz(
        (-30..30).map(|v| v as f64 / 10.0),
        (-30..30).map(|v| v as f64 / 10.0),
        |x:f64,z:f64|(0.4 * (x * x + z * z)).cos()).style_func(
            &|y| HSLColor(0.6666, y + 0.5, 0.5).mix(0.8).filled()
        )
    ).unwrap();
    ```

    The resulting style varies from gray to blue according to the value of y:

    ![](https://cdn.jsdelivr.net/gh/facorread/plotters-doc-data@da8400f/apidoc/surface_series_style_func.svg)
    */
    pub fn style_func<F: Fn(&D::OutputType) -> ShapeStyle>(mut self, f: &'a F) -> Self {
        self.style = StyleConfig::Function(f);
        self
    }

    /// Sets the style of the plot. See [`SurfaceSeries`] for more information and examples.
    pub fn style<S: Into<ShapeStyle>>(mut self, s: S) -> Self {
        self.style = StyleConfig::Fixed(s.into());
        self
    }
}

macro_rules! impl_constructor {
    ($dir: ty, $name: ident) => {
        impl<'a, X, Y, Z, SurfaceFunc> SurfaceSeries<'a, X, Y, Z, $dir, SurfaceFunc>
        where
            SurfaceFunc: Fn(
                <$dir as Direction<X, Y, Z>>::Input1Type,
                <$dir as Direction<X, Y, Z>>::Input2Type,
            ) -> <$dir as Direction<X, Y, Z>>::OutputType,
        {
            /// Implements the constructor. See [`SurfaceSeries`] for more information and examples.
            pub fn $name<IterA, IterB>(a: IterA, b: IterB, f: SurfaceFunc) -> Self
            where
                IterA: Iterator<Item = <$dir as Direction<X, Y, Z>>::Input1Type>,
                IterB: Iterator<Item = <$dir as Direction<X, Y, Z>>::Input2Type>,
            {
                Self::new(a, b, f)
            }
        }
    };
}

impl_constructor!(XOY, xoy);
impl_constructor!(XOZ, xoz);
impl_constructor!(YOZ, yoz);
impl<'a, X, Y, Z, D, SurfaceFunc> Iterator for SurfaceSeries<'a, X, Y, Z, D, SurfaceFunc>
where
    D: Direction<X, Y, Z>,
    D::Input1Type: Clone,
    D::Input2Type: Clone,
    SurfaceFunc: Fn(D::Input1Type, D::Input2Type) -> D::OutputType,
{
    type Item = Polygon<(X, Y, Z)>;
    fn next(&mut self) -> Option<Self::Item> {
        let (b0, b1) = if let (Some(b0), Some(b1)) = (
            self.free_var_2.get(self.vidx_2),
            self.free_var_2.get(self.vidx_2 + 1),
        ) {
            self.vidx_2 += 1;
            (b0, b1)
        } else {
            self.vidx_1 += 1;
            self.vidx_2 = 1;
            if let (Some(b0), Some(b1)) = (self.free_var_2.first(), self.free_var_2.get(1)) {
                (b0, b1)
            } else {
                return None;
            }
        };

        match (
            self.free_var_1.get(self.vidx_1),
            self.free_var_1.get(self.vidx_1 + 1),
        ) {
            (Some(a0), Some(a1)) => {
                let value = (self.surface_f)(a0.clone(), b0.clone());
                let style = self.style.get_style(&value);
                let vert = vec![
                    D::make_coord((a0.clone(), b0.clone()), value),
                    D::make_coord(
                        (a0.clone(), b1.clone()),
                        (self.surface_f)(a0.clone(), b1.clone()),
                    ),
                    D::make_coord(
                        (a1.clone(), b1.clone()),
                        (self.surface_f)(a1.clone(), b1.clone()),
                    ),
                    D::make_coord(
                        (a1.clone(), b0.clone()),
                        (self.surface_f)(a1.clone(), b0.clone()),
                    ),
                ];
                Some(Polygon::new(vert, style))
            }
            _ => None,
        }
    }
}
