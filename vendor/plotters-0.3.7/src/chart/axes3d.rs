use std::marker::PhantomData;

use super::ChartContext;
use crate::coord::cartesian::Cartesian3d;
use crate::coord::ranged1d::{BoldPoints, LightPoints, Ranged, ValueFormatter};
use crate::style::colors::{BLACK, TRANSPARENT};
use crate::style::Color;
use crate::style::{AsRelative, ShapeStyle, SizeDesc, TextStyle};

use super::Coord3D;

use crate::drawing::DrawingAreaErrorKind;

use plotters_backend::DrawingBackend;

/**
Implements 3D plot axes configurations.

The best way to use this struct is by way of the [`configure_axes()`] function.
See [`ChartContext::configure_axes()`] for more information and examples.
*/
pub struct Axes3dStyle<'a, 'b, X: Ranged, Y: Ranged, Z: Ranged, DB: DrawingBackend> {
    pub(super) parent_size: (u32, u32),
    pub(super) target: Option<&'b mut ChartContext<'a, DB, Cartesian3d<X, Y, Z>>>,
    pub(super) tick_size: i32,
    pub(super) light_lines_limit: [usize; 3],
    pub(super) n_labels: [usize; 3],
    pub(super) bold_line_style: ShapeStyle,
    pub(super) light_line_style: ShapeStyle,
    pub(super) axis_panel_style: ShapeStyle,
    pub(super) axis_style: ShapeStyle,
    pub(super) label_style: TextStyle<'b>,
    pub(super) format_x: &'b dyn Fn(&X::ValueType) -> String,
    pub(super) format_y: &'b dyn Fn(&Y::ValueType) -> String,
    pub(super) format_z: &'b dyn Fn(&Z::ValueType) -> String,
    _phantom: PhantomData<&'a (X, Y, Z)>,
}

impl<'a, 'b, X, Y, Z, XT, YT, ZT, DB> Axes3dStyle<'a, 'b, X, Y, Z, DB>
where
    X: Ranged<ValueType = XT> + ValueFormatter<XT>,
    Y: Ranged<ValueType = YT> + ValueFormatter<YT>,
    Z: Ranged<ValueType = ZT> + ValueFormatter<ZT>,
    DB: DrawingBackend,
{
    /**
    Set the size of the tick marks.

    - `value` Desired tick mark size, in pixels.

    See [`ChartContext::configure_axes()`] for more information and examples.
    */
    pub fn tick_size<Size: SizeDesc>(&mut self, size: Size) -> &mut Self {
        let actual_size = size.in_pixels(&self.parent_size);
        self.tick_size = actual_size;
        self
    }

    /**
    Set the maximum number of divisions for the minor grid in the X axis.

    - `value`: Maximum desired divisions between two consecutive X labels.

    See [`ChartContext::configure_axes()`] for more information and examples.
    */
    pub fn x_max_light_lines(&mut self, value: usize) -> &mut Self {
        self.light_lines_limit[0] = value;
        self
    }

    /**
    Set the maximum number of divisions for the minor grid in the Y axis.

    - `value`: Maximum desired divisions between two consecutive Y labels.

    See [`ChartContext::configure_axes()`] for more information and examples.
    */
    pub fn y_max_light_lines(&mut self, value: usize) -> &mut Self {
        self.light_lines_limit[1] = value;
        self
    }

    /**
    Set the maximum number of divisions for the minor grid in the Z axis.

    - `value`: Maximum desired divisions between two consecutive Z labels.

    See [`ChartContext::configure_axes()`] for more information and examples.
    */
    pub fn z_max_light_lines(&mut self, value: usize) -> &mut Self {
        self.light_lines_limit[2] = value;
        self
    }

    /**
    Set the maximum number of divisions for the minor grid.

    - `value`: Maximum desired divisions between two consecutive labels in X, Y, and Z.

    See [`ChartContext::configure_axes()`] for more information and examples.
    */
    pub fn max_light_lines(&mut self, value: usize) -> &mut Self {
        self.light_lines_limit[0] = value;
        self.light_lines_limit[1] = value;
        self.light_lines_limit[2] = value;
        self
    }

    /**
    Set the number of labels on the X axes.

    See [`ChartContext::configure_axes()`] for more information and examples.
    */
    pub fn x_labels(&mut self, n: usize) -> &mut Self {
        self.n_labels[0] = n;
        self
    }

    /**
    Set the number of labels on the Y axes.

    See [`ChartContext::configure_axes()`] for more information and examples.
    */
    pub fn y_labels(&mut self, n: usize) -> &mut Self {
        self.n_labels[1] = n;
        self
    }

    /**
    Set the number of labels on the Z axes.

    See [`ChartContext::configure_axes()`] for more information and examples.
    */
    pub fn z_labels(&mut self, n: usize) -> &mut Self {
        self.n_labels[2] = n;
        self
    }

    /**
    Sets the style of the panels in the background.

    See [`ChartContext::configure_axes()`] for more information and examples.
    */
    pub fn axis_panel_style<S: Into<ShapeStyle>>(&mut self, style: S) -> &mut Self {
        self.axis_panel_style = style.into();
        self
    }

    /**
    Sets the style of the major grid lines.

    See [`ChartContext::configure_axes()`] for more information and examples.
    */
    pub fn bold_grid_style<S: Into<ShapeStyle>>(&mut self, style: S) -> &mut Self {
        self.bold_line_style = style.into();
        self
    }

    /**
    Sets the style of the minor grid lines.

    See [`ChartContext::configure_axes()`] for more information and examples.
    */
    pub fn light_grid_style<S: Into<ShapeStyle>>(&mut self, style: S) -> &mut Self {
        self.light_line_style = style.into();
        self
    }

    /**
    Sets the text style of the axis labels.

    See [`ChartContext::configure_axes()`] for more information and examples.
    */
    pub fn label_style<S: Into<TextStyle<'b>>>(&mut self, style: S) -> &mut Self {
        self.label_style = style.into();
        self
    }

    /**
    Specifies the string format of the X axis labels.

    See [`ChartContext::configure_axes()`] for more information and examples.
    */
    pub fn x_formatter<F: Fn(&X::ValueType) -> String>(&mut self, f: &'b F) -> &mut Self {
        self.format_x = f;
        self
    }

    /**
    Specifies the string format of the Y axis labels.

    See [`ChartContext::configure_axes()`] for more information and examples.
    */
    pub fn y_formatter<F: Fn(&Y::ValueType) -> String>(&mut self, f: &'b F) -> &mut Self {
        self.format_y = f;
        self
    }

    /**
    Specifies the string format of the Z axis labels.

    See [`ChartContext::configure_axes()`] for more information and examples.
    */
    pub fn z_formatter<F: Fn(&Z::ValueType) -> String>(&mut self, f: &'b F) -> &mut Self {
        self.format_z = f;
        self
    }

    /**
    Constructs a new configuration object and defines the defaults.

    This is used internally by Plotters and should probably not be included in user code.
    See [`ChartContext::configure_axes()`] for more information and examples.
    */
    pub(crate) fn new(chart: &'b mut ChartContext<'a, DB, Cartesian3d<X, Y, Z>>) -> Self {
        let parent_size = chart.drawing_area.dim_in_pixel();
        let base_tick_size = (5u32).percent().max(5).in_pixels(chart.plotting_area());
        let tick_size = base_tick_size;
        Self {
            parent_size,
            tick_size,
            light_lines_limit: [10, 10, 10],
            n_labels: [10, 10, 10],
            bold_line_style: Into::<ShapeStyle>::into(BLACK.mix(0.2)),
            light_line_style: Into::<ShapeStyle>::into(TRANSPARENT),
            axis_panel_style: Into::<ShapeStyle>::into(BLACK.mix(0.1)),
            axis_style: Into::<ShapeStyle>::into(BLACK.mix(0.8)),
            label_style: ("sans-serif", (12).percent().max(12).in_pixels(&parent_size)).into(),
            format_x: &X::format,
            format_y: &Y::format,
            format_z: &Z::format,
            _phantom: PhantomData,
            target: Some(chart),
        }
    }

    pub fn draw(&mut self) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>>
    where
        XT: Clone,
        YT: Clone,
        ZT: Clone,
    {
        let chart = self.target.take().unwrap();
        let kps_bold = chart.get_key_points(
            BoldPoints(self.n_labels[0]),
            BoldPoints(self.n_labels[1]),
            BoldPoints(self.n_labels[2]),
        );
        let kps_light = chart.get_key_points(
            LightPoints::new(
                self.n_labels[0],
                self.n_labels[0] * self.light_lines_limit[0],
            ),
            LightPoints::new(
                self.n_labels[1],
                self.n_labels[1] * self.light_lines_limit[1],
            ),
            LightPoints::new(
                self.n_labels[2],
                self.n_labels[2] * self.light_lines_limit[2],
            ),
        );

        let panels = chart.draw_axis_panels(
            &kps_bold,
            &kps_light,
            self.axis_panel_style,
            self.bold_line_style,
            self.light_line_style,
        )?;

        for i in 0..3 {
            let axis = chart.draw_axis(i, &panels, self.axis_style)?;
            let labels: Vec<_> = match i {
                0 => kps_bold
                    .x_points
                    .iter()
                    .map(|x| {
                        let x_text = (self.format_x)(x);
                        let mut p = axis[0].clone();
                        p[0] = Coord3D::X(x.clone());
                        (p, x_text)
                    })
                    .collect(),
                1 => kps_bold
                    .y_points
                    .iter()
                    .map(|y| {
                        let y_text = (self.format_y)(y);
                        let mut p = axis[0].clone();
                        p[1] = Coord3D::Y(y.clone());
                        (p, y_text)
                    })
                    .collect(),
                _ => kps_bold
                    .z_points
                    .iter()
                    .map(|z| {
                        let z_text = (self.format_z)(z);
                        let mut p = axis[0].clone();
                        p[2] = Coord3D::Z(z.clone());
                        (p, z_text)
                    })
                    .collect(),
            };
            chart.draw_axis_ticks(
                axis,
                &labels[..],
                self.tick_size,
                self.axis_style,
                self.label_style.clone(),
            )?;
        }

        Ok(())
    }
}
