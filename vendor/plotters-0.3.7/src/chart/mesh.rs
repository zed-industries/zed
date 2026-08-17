use std::marker::PhantomData;

use super::builder::LabelAreaPosition;
use super::context::ChartContext;
use crate::coord::cartesian::{Cartesian2d, MeshLine};
use crate::coord::ranged1d::{BoldPoints, LightPoints, Ranged, ValueFormatter};
use crate::drawing::DrawingAreaErrorKind;
use crate::style::{
    AsRelative, Color, FontDesc, FontFamily, FontStyle, IntoTextStyle, RGBColor, ShapeStyle,
    SizeDesc, TextStyle,
};

use plotters_backend::DrawingBackend;

/// The style used to describe the mesh and axis for a secondary coordinate system.
pub struct SecondaryMeshStyle<'a, 'b, X: Ranged, Y: Ranged, DB: DrawingBackend> {
    style: MeshStyle<'a, 'b, X, Y, DB>,
}

impl<'a, 'b, XT, YT, X: Ranged<ValueType = XT>, Y: Ranged<ValueType = YT>, DB: DrawingBackend>
    SecondaryMeshStyle<'a, 'b, X, Y, DB>
where
    X: ValueFormatter<XT>,
    Y: ValueFormatter<YT>,
{
    pub(super) fn new(target: &'b mut ChartContext<'a, DB, Cartesian2d<X, Y>>) -> Self {
        let mut style = target.configure_mesh();
        style.draw_x_mesh = false;
        style.draw_y_mesh = false;
        Self { style }
    }

    /// Set the style definition for the axis
    /// - `style`: The style for the axis
    pub fn axis_style<T: Into<ShapeStyle>>(&mut self, style: T) -> &mut Self {
        self.style.axis_style(style);
        self
    }

    /// The offset of x labels. This is used when we want to place the label in the middle of
    /// the grid. This is used to adjust label position for histograms, but since plotters 0.3, this
    /// use case is deprecated, see [SegmentedCoord coord decorator](../coord/ranged1d/trait.IntoSegmentedCoord.html) for more details
    /// - `value`: The offset in pixel
    pub fn x_label_offset<S: SizeDesc>(&mut self, value: S) -> &mut Self {
        self.style.x_label_offset(value);
        self
    }

    /// The offset of y labels. This is used when we want to place the label in the middle of
    /// the grid. This is used to adjust label position for histograms, but since plotters 0.3, this
    /// use case is deprecated, see [SegmentedCoord coord decorator](../coord/ranged1d/trait.IntoSegmentedCoord.html) for more details
    /// - `value`: The offset in pixel
    pub fn y_label_offset<S: SizeDesc>(&mut self, value: S) -> &mut Self {
        self.style.y_label_offset(value);
        self
    }

    /// Set how many labels for the X axis at most
    /// - `value`: The maximum desired number of labels in the X axis
    pub fn x_labels(&mut self, value: usize) -> &mut Self {
        self.style.x_labels(value);
        self
    }

    /// Set how many label for the Y axis at most
    /// - `value`: The maximum desired number of labels in the Y axis
    pub fn y_labels(&mut self, value: usize) -> &mut Self {
        self.style.y_labels(value);
        self
    }

    /// Set the formatter function for the X label text
    /// - `fmt`: The formatter function
    pub fn x_label_formatter(&mut self, fmt: &'b dyn Fn(&X::ValueType) -> String) -> &mut Self {
        self.style.x_label_formatter(fmt);
        self
    }

    /// Set the formatter function for the Y label text
    /// - `fmt`: The formatter function
    pub fn y_label_formatter(&mut self, fmt: &'b dyn Fn(&Y::ValueType) -> String) -> &mut Self {
        self.style.y_label_formatter(fmt);
        self
    }

    /// Set the axis description's style. If not given, use label style instead.
    /// - `style`: The text style that would be applied to descriptions
    pub fn axis_desc_style<T: IntoTextStyle<'b>>(&mut self, style: T) -> &mut Self {
        self.style
            .axis_desc_style(style.into_text_style(&self.style.parent_size));
        self
    }

    /// Set the X axis's description
    /// - `desc`: The description of the X axis
    pub fn x_desc<T: Into<String>>(&mut self, desc: T) -> &mut Self {
        self.style.x_desc(desc);
        self
    }

    /// Set the Y axis's description
    /// - `desc`: The description of the Y axis
    pub fn y_desc<T: Into<String>>(&mut self, desc: T) -> &mut Self {
        self.style.y_desc(desc);
        self
    }

    /// Draw the axes for the secondary coordinate system
    pub fn draw(&mut self) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>> {
        self.style.draw()
    }

    /// Set the label style for the secondary axis
    pub fn label_style<T: IntoTextStyle<'b>>(&mut self, style: T) -> &mut Self {
        self.style.label_style(style);
        self
    }

    /// Set all the tick marks to the same size
    /// `value`: The new size
    pub fn set_all_tick_mark_size<S: SizeDesc>(&mut self, value: S) -> &mut Self {
        let size = value.in_pixels(&self.style.parent_size);
        self.style.x_tick_size = [size, size];
        self.style.y_tick_size = [size, size];
        self
    }
    /// Sets the tick mark size for a given label area position.
    /// `value`: The new size
    pub fn set_tick_mark_size<S: SizeDesc>(
        &mut self,
        pos: LabelAreaPosition,
        value: S,
    ) -> &mut Self {
        *match pos {
            LabelAreaPosition::Top => &mut self.style.x_tick_size[0],
            LabelAreaPosition::Bottom => &mut self.style.x_tick_size[1],
            LabelAreaPosition::Left => &mut self.style.y_tick_size[0],
            LabelAreaPosition::Right => &mut self.style.y_tick_size[1],
        } = value.in_pixels(&self.style.parent_size);
        self
    }
}

/// The struct that is used for tracking the configuration of a mesh of any chart
pub struct MeshStyle<'a, 'b, X: Ranged, Y: Ranged, DB: DrawingBackend> {
    pub(super) parent_size: (u32, u32),
    pub(super) draw_x_mesh: bool,
    pub(super) draw_y_mesh: bool,
    pub(super) draw_x_axis: bool,
    pub(super) draw_y_axis: bool,
    pub(super) x_label_offset: i32,
    pub(super) y_label_offset: i32,
    pub(super) x_light_lines_limit: usize,
    pub(super) y_light_lines_limit: usize,
    pub(super) n_x_labels: usize,
    pub(super) n_y_labels: usize,
    pub(super) axis_desc_style: Option<TextStyle<'b>>,
    pub(super) x_desc: Option<String>,
    pub(super) y_desc: Option<String>,
    pub(super) bold_line_style: Option<ShapeStyle>,
    pub(super) light_line_style: Option<ShapeStyle>,
    pub(super) axis_style: Option<ShapeStyle>,
    pub(super) x_label_style: Option<TextStyle<'b>>,
    pub(super) y_label_style: Option<TextStyle<'b>>,
    pub(super) format_x: Option<&'b dyn Fn(&X::ValueType) -> String>,
    pub(super) format_y: Option<&'b dyn Fn(&Y::ValueType) -> String>,
    pub(super) target: Option<&'b mut ChartContext<'a, DB, Cartesian2d<X, Y>>>,
    pub(super) _phantom_data: PhantomData<(X, Y)>,
    pub(super) x_tick_size: [i32; 2],
    pub(super) y_tick_size: [i32; 2],
}

impl<'a, 'b, X, Y, XT, YT, DB> MeshStyle<'a, 'b, X, Y, DB>
where
    X: Ranged<ValueType = XT> + ValueFormatter<XT>,
    Y: Ranged<ValueType = YT> + ValueFormatter<YT>,
    DB: DrawingBackend,
{
    pub(crate) fn new(chart: &'b mut ChartContext<'a, DB, Cartesian2d<X, Y>>) -> Self {
        let base_tick_size = (5u32).percent().max(5).in_pixels(chart.plotting_area());

        let mut x_tick_size = [base_tick_size, base_tick_size];
        let mut y_tick_size = [base_tick_size, base_tick_size];

        for idx in 0..2 {
            if chart.is_overlapping_drawing_area(chart.x_label_area[idx].as_ref()) {
                x_tick_size[idx] = -x_tick_size[idx];
            }
            if chart.is_overlapping_drawing_area(chart.y_label_area[idx].as_ref()) {
                y_tick_size[idx] = -y_tick_size[idx];
            }
        }

        MeshStyle {
            parent_size: chart.drawing_area.dim_in_pixel(),
            axis_style: None,
            x_label_offset: 0,
            y_label_offset: 0,
            draw_x_mesh: true,
            draw_y_mesh: true,
            draw_x_axis: true,
            draw_y_axis: true,
            x_light_lines_limit: 10,
            y_light_lines_limit: 10,
            n_x_labels: 11,
            n_y_labels: 11,
            bold_line_style: None,
            light_line_style: None,
            x_label_style: None,
            y_label_style: None,
            format_x: None,
            format_y: None,
            target: Some(chart),
            _phantom_data: PhantomData,
            x_desc: None,
            y_desc: None,
            axis_desc_style: None,
            x_tick_size,
            y_tick_size,
        }
    }
}

impl<'a, 'b, X, Y, DB> MeshStyle<'a, 'b, X, Y, DB>
where
    X: Ranged,
    Y: Ranged,
    DB: DrawingBackend,
{
    /// Set all the tick mark to the same size
    /// `value`: The new size
    pub fn set_all_tick_mark_size<S: SizeDesc>(&mut self, value: S) -> &mut Self {
        let size = value.in_pixels(&self.parent_size);
        self.x_tick_size = [size, size];
        self.y_tick_size = [size, size];
        self
    }

    /// Set the tick mark size on the axes. When this is set to negative, the axis value label will
    /// become inward.
    ///
    /// - `pos`: The which label area we want to set
    /// - `value`: The size specification
    pub fn set_tick_mark_size<S: SizeDesc>(
        &mut self,
        pos: LabelAreaPosition,
        value: S,
    ) -> &mut Self {
        *match pos {
            LabelAreaPosition::Top => &mut self.x_tick_size[0],
            LabelAreaPosition::Bottom => &mut self.x_tick_size[1],
            LabelAreaPosition::Left => &mut self.y_tick_size[0],
            LabelAreaPosition::Right => &mut self.y_tick_size[1],
        } = value.in_pixels(&self.parent_size);
        self
    }

    /// The offset of x labels. This is used when we want to place the label in the middle of
    /// the grid. This is used to adjust label position for histograms, but since plotters 0.3, this
    /// use case is deprecated, see [SegmentedCoord coord decorator](../coord/ranged1d/trait.IntoSegmentedCoord.html) for more details
    /// - `value`: The offset in pixel
    pub fn x_label_offset<S: SizeDesc>(&mut self, value: S) -> &mut Self {
        self.x_label_offset = value.in_pixels(&self.parent_size);
        self
    }

    /// The offset of y labels. This is used when we want to place the label in the middle of
    /// the grid. This is used to adjust label position for histograms, but since plotters 0.3, this
    /// use case is deprecated, see [SegmentedCoord coord decorator](../coord/ranged1d/trait.IntoSegmentedCoord.html) for more details
    /// - `value`: The offset in pixel
    pub fn y_label_offset<S: SizeDesc>(&mut self, value: S) -> &mut Self {
        self.y_label_offset = value.in_pixels(&self.parent_size);
        self
    }

    /// Disable the mesh for the x axis.
    pub fn disable_x_mesh(&mut self) -> &mut Self {
        self.draw_x_mesh = false;
        self
    }

    /// Disable the mesh for the y axis
    pub fn disable_y_mesh(&mut self) -> &mut Self {
        self.draw_y_mesh = false;
        self
    }

    /// Disable drawing the X axis
    pub fn disable_x_axis(&mut self) -> &mut Self {
        self.draw_x_axis = false;
        self
    }

    /// Disable drawing the Y axis
    pub fn disable_y_axis(&mut self) -> &mut Self {
        self.draw_y_axis = false;
        self
    }

    /// Disable drawing all meshes
    pub fn disable_mesh(&mut self) -> &mut Self {
        self.disable_x_mesh().disable_y_mesh()
    }

    /// Disable drawing all axes
    pub fn disable_axes(&mut self) -> &mut Self {
        self.disable_x_axis().disable_y_axis()
    }

    /// Set the style definition for the axis
    /// - `style`: The style for the axis
    pub fn axis_style<T: Into<ShapeStyle>>(&mut self, style: T) -> &mut Self {
        self.axis_style = Some(style.into());
        self
    }

    /// Set the maximum number of divisions for the minor grid
    /// - `value`: Maximum desired divisions between two consecutive X labels
    pub fn x_max_light_lines(&mut self, value: usize) -> &mut Self {
        self.x_light_lines_limit = value;
        self
    }

    /// Set the maximum number of divisions for the minor grid
    /// - `value`: Maximum desired divisions between two consecutive Y labels
    pub fn y_max_light_lines(&mut self, value: usize) -> &mut Self {
        self.y_light_lines_limit = value;
        self
    }

    /// Set the maximum number of divisions for the minor grid
    /// - `value`: Maximum desired divisions between two consecutive labels in X and Y
    pub fn max_light_lines(&mut self, value: usize) -> &mut Self {
        self.x_light_lines_limit = value;
        self.y_light_lines_limit = value;
        self
    }

    /// Set how many labels for the X axis at most
    /// - `value`: The maximum desired number of labels in the X axis
    pub fn x_labels(&mut self, value: usize) -> &mut Self {
        self.n_x_labels = value;
        self
    }

    /// Set how many label for the Y axis at most
    /// - `value`: The maximum desired number of labels in the Y axis
    pub fn y_labels(&mut self, value: usize) -> &mut Self {
        self.n_y_labels = value;
        self
    }

    /// Set the style for the coarse grind grid
    /// - `style`: This is the coarse grind grid style
    pub fn bold_line_style<T: Into<ShapeStyle>>(&mut self, style: T) -> &mut Self {
        self.bold_line_style = Some(style.into());
        self
    }

    /// Set the style for the fine grind grid
    /// - `style`: The fine grind grid style
    pub fn light_line_style<T: Into<ShapeStyle>>(&mut self, style: T) -> &mut Self {
        self.light_line_style = Some(style.into());
        self
    }

    /// Set the style of the label text
    /// - `style`: The text style that would be applied to the labels
    pub fn label_style<T: IntoTextStyle<'b>>(&mut self, style: T) -> &mut Self {
        let style = style.into_text_style(&self.parent_size);
        self.x_label_style = Some(style.clone());
        self.y_label_style = Some(style);
        self
    }

    /// Set the style of the label X axis text
    /// - `style`: The text style that would be applied to the labels
    pub fn x_label_style<T: IntoTextStyle<'b>>(&mut self, style: T) -> &mut Self {
        self.x_label_style = Some(style.into_text_style(&self.parent_size));
        self
    }

    /// Set the style of the label Y axis text
    /// - `style`: The text style that would be applied to the labels
    pub fn y_label_style<T: IntoTextStyle<'b>>(&mut self, style: T) -> &mut Self {
        self.y_label_style = Some(style.into_text_style(&self.parent_size));
        self
    }

    /// Set the formatter function for the X label text
    /// - `fmt`: The formatter function
    pub fn x_label_formatter(&mut self, fmt: &'b dyn Fn(&X::ValueType) -> String) -> &mut Self {
        self.format_x = Some(fmt);
        self
    }

    /// Set the formatter function for the Y label text
    /// - `fmt`: The formatter function
    pub fn y_label_formatter(&mut self, fmt: &'b dyn Fn(&Y::ValueType) -> String) -> &mut Self {
        self.format_y = Some(fmt);
        self
    }

    /// Set the axis description's style. If not given, use label style instead.
    /// - `style`: The text style that would be applied to descriptions
    pub fn axis_desc_style<T: IntoTextStyle<'b>>(&mut self, style: T) -> &mut Self {
        self.axis_desc_style = Some(style.into_text_style(&self.parent_size));
        self
    }

    /// Set the X axis's description
    /// - `desc`: The description of the X axis
    pub fn x_desc<T: Into<String>>(&mut self, desc: T) -> &mut Self {
        self.x_desc = Some(desc.into());
        self
    }

    /// Set the Y axis's description
    /// - `desc`: The description of the Y axis
    pub fn y_desc<T: Into<String>>(&mut self, desc: T) -> &mut Self {
        self.y_desc = Some(desc.into());
        self
    }

    /// Draw the configured mesh on the target plot
    pub fn draw(&mut self) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>>
    where
        X: ValueFormatter<<X as Ranged>::ValueType>,
        Y: ValueFormatter<<Y as Ranged>::ValueType>,
    {
        let target = self.target.take().unwrap();

        let default_mesh_color_1 = RGBColor(0, 0, 0).mix(0.2);
        let default_mesh_color_2 = RGBColor(0, 0, 0).mix(0.1);
        let default_axis_color = RGBColor(0, 0, 0);
        let default_label_font = FontDesc::new(
            FontFamily::SansSerif,
            f64::from((12i32).percent().max(12).in_pixels(&self.parent_size)),
            FontStyle::Normal,
        );

        let bold_style = self
            .bold_line_style
            .unwrap_or_else(|| (&default_mesh_color_1).into());
        let light_style = self
            .light_line_style
            .unwrap_or_else(|| (&default_mesh_color_2).into());
        let axis_style = self
            .axis_style
            .unwrap_or_else(|| (&default_axis_color).into());

        let x_label_style = self
            .x_label_style
            .clone()
            .unwrap_or_else(|| default_label_font.clone().into());

        let y_label_style = self
            .y_label_style
            .clone()
            .unwrap_or_else(|| default_label_font.into());

        let axis_desc_style = self
            .axis_desc_style
            .clone()
            .unwrap_or_else(|| x_label_style.clone());

        target.draw_mesh(
            (
                LightPoints::new(self.n_y_labels, self.n_y_labels * self.y_light_lines_limit),
                LightPoints::new(self.n_x_labels, self.n_x_labels * self.x_light_lines_limit),
            ),
            &light_style,
            &x_label_style,
            &y_label_style,
            |_, _, _| None,
            self.draw_x_mesh,
            self.draw_y_mesh,
            self.x_label_offset,
            self.y_label_offset,
            false,
            false,
            &axis_style,
            &axis_desc_style,
            self.x_desc.clone(),
            self.y_desc.clone(),
            self.x_tick_size,
            self.y_tick_size,
        )?;

        target.draw_mesh(
            (BoldPoints(self.n_y_labels), BoldPoints(self.n_x_labels)),
            &bold_style,
            &x_label_style,
            &y_label_style,
            |xr, yr, m| match m {
                MeshLine::XMesh(_, _, v) => {
                    if self.draw_x_axis {
                        if let Some(fmt_func) = self.format_x {
                            Some(fmt_func(v))
                        } else {
                            Some(xr.format_ext(v))
                        }
                    } else {
                        None
                    }
                }
                MeshLine::YMesh(_, _, v) => {
                    if self.draw_y_axis {
                        if let Some(fmt_func) = self.format_y {
                            Some(fmt_func(v))
                        } else {
                            Some(yr.format_ext(v))
                        }
                    } else {
                        None
                    }
                }
            },
            self.draw_x_mesh,
            self.draw_y_mesh,
            self.x_label_offset,
            self.y_label_offset,
            self.draw_x_axis,
            self.draw_y_axis,
            &axis_style,
            &axis_desc_style,
            None,
            None,
            self.x_tick_size,
            self.y_tick_size,
        )
    }
}
