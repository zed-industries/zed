use super::context::ChartContext;

use crate::coord::cartesian::{Cartesian2d, Cartesian3d};
use crate::coord::ranged1d::AsRangedCoord;
use crate::coord::Shift;

use crate::drawing::{DrawingArea, DrawingAreaErrorKind};
use crate::style::{IntoTextStyle, SizeDesc, TextStyle};

use plotters_backend::DrawingBackend;

/**
Specifies one of the four label positions around the figure.

This is used to configure the label area size with function
[`ChartBuilder::set_label_area_size()`].

# Example

```
use plotters::prelude::*;
let drawing_area = SVGBackend::new("label_area_position.svg", (300, 200)).into_drawing_area();
drawing_area.fill(&WHITE).unwrap();
let mut chart_builder = ChartBuilder::on(&drawing_area);
chart_builder.set_label_area_size(LabelAreaPosition::Bottom, 60).set_label_area_size(LabelAreaPosition::Left, 35);
let mut chart_context = chart_builder.build_cartesian_2d(0.0..4.0, 0.0..3.0).unwrap();
chart_context.configure_mesh().x_desc("Spacious X label area").y_desc("Narrow Y label area").draw().unwrap();
```

The result is a chart with a spacious X label area and a narrow Y label area:

![](https://cdn.jsdelivr.net/gh/facorread/plotters-doc-data@9ca6541/apidoc/label_area_position.svg)

# See also

[`ChartBuilder::set_left_and_bottom_label_area_size()`]
*/
#[derive(Copy, Clone)]
pub enum LabelAreaPosition {
    /// Top of the figure
    Top = 0,
    /// Bottom of the figure
    Bottom = 1,
    /// Left side of the figure
    Left = 2,
    /// Right side of the figure
    Right = 3,
}

/**
The helper object to create a chart context, which is used for the high-level figure drawing.

With the help of this object, we can convert a basic drawing area into a chart context, which
allows the high-level charting API being used on the drawing area.

See [`ChartBuilder::on()`] for more information and examples.
*/
pub struct ChartBuilder<'a, 'b, DB: DrawingBackend> {
    label_area_size: [u32; 4], // [upper, lower, left, right]
    overlap_plotting_area: [bool; 4],
    root_area: &'a DrawingArea<DB, Shift>,
    title: Option<(String, TextStyle<'b>)>,
    margin: [u32; 4],
}

impl<'a, 'b, DB: DrawingBackend> ChartBuilder<'a, 'b, DB> {
    /**
    Create a chart builder on the given drawing area

    - `root`: The root drawing area
    - Returns: The chart builder object

    # Example

    ```
    use plotters::prelude::*;
    let drawing_area = SVGBackend::new("chart_builder_on.svg", (300, 200)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();
    let mut chart_builder = ChartBuilder::on(&drawing_area);
    chart_builder.margin(5).set_left_and_bottom_label_area_size(35)
    .caption("Figure title or caption", ("Calibri", 20, FontStyle::Italic, &RED).into_text_style(&drawing_area));
    let mut chart_context = chart_builder.build_cartesian_2d(0.0..3.8, 0.0..2.8).unwrap();
    chart_context.configure_mesh().draw().unwrap();
    ```
    The result is a chart with customized margins, label area sizes, and title:

    ![](https://cdn.jsdelivr.net/gh/facorread/plotters-doc-data@42ecf52/apidoc/chart_builder_on.svg)

    */
    pub fn on(root: &'a DrawingArea<DB, Shift>) -> Self {
        Self {
            label_area_size: [0; 4],
            root_area: root,
            title: None,
            margin: [0; 4],
            overlap_plotting_area: [false; 4],
        }
    }

    /**
    Sets the size of the four margins of the chart.

    - `size`: The desired size of the four chart margins in backend units (pixels).

    See [`ChartBuilder::on()`] for more information and examples.
    */
    pub fn margin<S: SizeDesc>(&mut self, size: S) -> &mut Self {
        let size = size.in_pixels(self.root_area).max(0) as u32;
        self.margin = [size, size, size, size];
        self
    }

    /**
    Sets the size of the top margin of the chart.

    - `size`: The desired size of the margin in backend units (pixels).

    See [`ChartBuilder::on()`] for more information and examples.
    */
    pub fn margin_top<S: SizeDesc>(&mut self, size: S) -> &mut Self {
        let size = size.in_pixels(self.root_area).max(0) as u32;
        self.margin[0] = size;
        self
    }

    /**
    Sets the size of the bottom margin of the chart.

    - `size`: The desired size of the margin in backend units (pixels).

    See [`ChartBuilder::on()`] for more information and examples.
    */
    pub fn margin_bottom<S: SizeDesc>(&mut self, size: S) -> &mut Self {
        let size = size.in_pixels(self.root_area).max(0) as u32;
        self.margin[1] = size;
        self
    }

    /**
    Sets the size of the left margin of the chart.

    - `size`: The desired size of the margin in backend units (pixels).

    See [`ChartBuilder::on()`] for more information and examples.
    */
    pub fn margin_left<S: SizeDesc>(&mut self, size: S) -> &mut Self {
        let size = size.in_pixels(self.root_area).max(0) as u32;
        self.margin[2] = size;
        self
    }

    /**
    Sets the size of the right margin of the chart.

    - `size`: The desired size of the margin in backend units (pixels).

    See [`ChartBuilder::on()`] for more information and examples.
    */
    pub fn margin_right<S: SizeDesc>(&mut self, size: S) -> &mut Self {
        let size = size.in_pixels(self.root_area).max(0) as u32;
        self.margin[3] = size;
        self
    }

    /**
    Sets the size of the four label areas of the chart.

    - `size`: The desired size of the four label areas in backend units (pixels).

    See [`ChartBuilder::on()`] for more information and examples.
    */
    pub fn set_all_label_area_size<S: SizeDesc>(&mut self, size: S) -> &mut Self {
        let size = size.in_pixels(self.root_area);
        self.set_label_area_size(LabelAreaPosition::Top, size)
            .set_label_area_size(LabelAreaPosition::Bottom, size)
            .set_label_area_size(LabelAreaPosition::Left, size)
            .set_label_area_size(LabelAreaPosition::Right, size)
    }

    /**
    Sets the size of the left and bottom label areas of the chart.

    - `size`: The desired size of the left and bottom label areas in backend units (pixels).

    See [`ChartBuilder::on()`] for more information and examples.
    */
    pub fn set_left_and_bottom_label_area_size<S: SizeDesc>(&mut self, size: S) -> &mut Self {
        let size = size.in_pixels(self.root_area);
        self.set_label_area_size(LabelAreaPosition::Left, size)
            .set_label_area_size(LabelAreaPosition::Bottom, size)
    }

    /**
    Sets the size of the X label area at the bottom of the chart.

    - `size`: The desired size of the X label area in backend units (pixels).
      If set to 0, the X label area is removed.

    See [`ChartBuilder::on()`] for more information and examples.
    */
    pub fn x_label_area_size<S: SizeDesc>(&mut self, size: S) -> &mut Self {
        self.set_label_area_size(LabelAreaPosition::Bottom, size)
    }

    /**
    Sets the size of the Y label area to the left of the chart.

    - `size`: The desired size of the Y label area in backend units (pixels).
      If set to 0, the Y label area is removed.

    See [`ChartBuilder::on()`] for more information and examples.
    */
    pub fn y_label_area_size<S: SizeDesc>(&mut self, size: S) -> &mut Self {
        self.set_label_area_size(LabelAreaPosition::Left, size)
    }

    /**
    Sets the size of the X label area at the top of the chart.

    - `size`: The desired size of the top X label area in backend units (pixels).
      If set to 0, the top X label area is removed.

    See [`ChartBuilder::on()`] for more information and examples.
    */
    pub fn top_x_label_area_size<S: SizeDesc>(&mut self, size: S) -> &mut Self {
        self.set_label_area_size(LabelAreaPosition::Top, size)
    }

    /**
    Sets the size of the Y label area to the right of the chart.

    - `size`: The desired size of the Y label area in backend units (pixels).
      If set to 0, the Y label area to the right is removed.

    See [`ChartBuilder::on()`] for more information and examples.
    */
    pub fn right_y_label_area_size<S: SizeDesc>(&mut self, size: S) -> &mut Self {
        self.set_label_area_size(LabelAreaPosition::Right, size)
    }

    /**
    Sets the size of a chart label area.

    - `pos`: The position of the desired label area to adjust
    - `size`: The desired size of the label area in backend units (pixels).
      If set to 0, the label area is removed.

    See [`ChartBuilder::on()`] for more information and examples.
    */
    pub fn set_label_area_size<S: SizeDesc>(
        &mut self,
        pos: LabelAreaPosition,
        size: S,
    ) -> &mut Self {
        let size = size.in_pixels(self.root_area);
        self.label_area_size[pos as usize] = size.unsigned_abs();
        self.overlap_plotting_area[pos as usize] = size < 0;
        self
    }

    /**
    Sets the title or caption of the chart.

    - `caption`: The caption of the chart
    - `style`: The text style

    The title or caption will be centered at the top of the drawing area.

    See [`ChartBuilder::on()`] for more information and examples.
    */
    pub fn caption<S: AsRef<str>, Style: IntoTextStyle<'b>>(
        &mut self,
        caption: S,
        style: Style,
    ) -> &mut Self {
        self.title = Some((
            caption.as_ref().to_string(),
            style.into_text_style(self.root_area),
        ));
        self
    }

    /// This function has been renamed to [`ChartBuilder::build_cartesian_2d()`] and is to be removed in the future.
    #[allow(clippy::type_complexity)]
    #[deprecated(
        note = "`build_ranged` has been renamed to `build_cartesian_2d` and is to be removed in the future."
    )]
    pub fn build_ranged<'c, X: AsRangedCoord, Y: AsRangedCoord>(
        &mut self,
        x_spec: X,
        y_spec: Y,
    ) -> Result<
        ChartContext<'c, DB, Cartesian2d<X::CoordDescType, Y::CoordDescType>>,
        DrawingAreaErrorKind<DB::ErrorType>,
    > {
        self.build_cartesian_2d(x_spec, y_spec)
    }

    /**
    Builds a chart with a 2D Cartesian coordinate system.

    - `x_spec`: Specifies the X axis range and data properties
    - `y_spec`: Specifies the Y axis range and data properties
    - Returns: A `ChartContext` object, ready to visualize data.

    See [`ChartBuilder::on()`] and [`ChartContext::configure_mesh()`] for more information and examples.
    */
    #[allow(clippy::type_complexity)]
    pub fn build_cartesian_2d<'c, X: AsRangedCoord, Y: AsRangedCoord>(
        &mut self,
        x_spec: X,
        y_spec: Y,
    ) -> Result<
        ChartContext<'c, DB, Cartesian2d<X::CoordDescType, Y::CoordDescType>>,
        DrawingAreaErrorKind<DB::ErrorType>,
    > {
        let mut label_areas = [None, None, None, None];

        let mut drawing_area = DrawingArea::clone(self.root_area);

        if *self.margin.iter().max().unwrap_or(&0) > 0 {
            drawing_area = drawing_area.margin(
                self.margin[0] as i32,
                self.margin[1] as i32,
                self.margin[2] as i32,
                self.margin[3] as i32,
            );
        }

        let (title_dx, title_dy) = if let Some((ref title, ref style)) = self.title {
            let (origin_dx, origin_dy) = drawing_area.get_base_pixel();
            drawing_area = drawing_area.titled(title, style.clone())?;
            let (current_dx, current_dy) = drawing_area.get_base_pixel();
            (current_dx - origin_dx, current_dy - origin_dy)
        } else {
            (0, 0)
        };

        let (w, h) = drawing_area.dim_in_pixel();

        let mut actual_drawing_area_pos = [0, h as i32, 0, w as i32];

        const DIR: [(i16, i16); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

        for (idx, (dx, dy)) in (0..4).map(|idx| (idx, DIR[idx])) {
            if self.overlap_plotting_area[idx] {
                continue;
            }

            let size = self.label_area_size[idx] as i32;

            let split_point = if dx + dy < 0 { size } else { -size };

            actual_drawing_area_pos[idx] += split_point;
        }

        // Now the root drawing area is to be split into
        //
        // +----------+------------------------------+------+
        // |    0     |    1 (Top Label Area)        |   2  |
        // +----------+------------------------------+------+
        // |    3     |                              |   5  |
        // |  Left    |       4 (Plotting Area)      | Right|
        // |  Labels  |                              | Label|
        // +----------+------------------------------+------+
        // |    6     |        7 (Bottom Labels)     |   8  |
        // +----------+------------------------------+------+

        let mut split: Vec<_> = drawing_area
            .split_by_breakpoints(
                &actual_drawing_area_pos[2..4],
                &actual_drawing_area_pos[0..2],
            )
            .into_iter()
            .map(Some)
            .collect();

        // Take out the plotting area
        std::mem::swap(&mut drawing_area, split[4].as_mut().unwrap());

        // Initialize the label areas - since the label area might be overlapping
        // with the plotting area, in this case, we need handle them differently
        for (src_idx, dst_idx) in [1, 7, 3, 5].iter().zip(0..4) {
            if !self.overlap_plotting_area[dst_idx] {
                let (h, w) = split[*src_idx].as_ref().unwrap().dim_in_pixel();
                if h > 0 && w > 0 {
                    std::mem::swap(&mut label_areas[dst_idx], &mut split[*src_idx]);
                }
            } else if self.label_area_size[dst_idx] != 0 {
                let size = self.label_area_size[dst_idx] as i32;
                let (dw, dh) = drawing_area.dim_in_pixel();
                let x0 = if DIR[dst_idx].0 > 0 {
                    dw as i32 - size
                } else {
                    0
                };
                let y0 = if DIR[dst_idx].1 > 0 {
                    dh as i32 - size
                } else {
                    0
                };
                let x1 = if DIR[dst_idx].0 >= 0 { dw as i32 } else { size };
                let y1 = if DIR[dst_idx].1 >= 0 { dh as i32 } else { size };

                label_areas[dst_idx] = Some(
                    drawing_area
                        .clone()
                        .shrink((x0, y0), ((x1 - x0), (y1 - y0))),
                );
            }
        }

        let mut pixel_range = drawing_area.get_pixel_range();
        pixel_range.0.end -= 1;
        pixel_range.1.end -= 1;
        pixel_range.1 = pixel_range.1.end..pixel_range.1.start;

        let mut x_label_area = [None, None];
        let mut y_label_area = [None, None];

        std::mem::swap(&mut x_label_area[0], &mut label_areas[0]);
        std::mem::swap(&mut x_label_area[1], &mut label_areas[1]);
        std::mem::swap(&mut y_label_area[0], &mut label_areas[2]);
        std::mem::swap(&mut y_label_area[1], &mut label_areas[3]);

        Ok(ChartContext {
            x_label_area,
            y_label_area,
            drawing_area: drawing_area.apply_coord_spec(Cartesian2d::new(
                x_spec,
                y_spec,
                pixel_range,
            )),
            series_anno: vec![],
            drawing_area_pos: (
                actual_drawing_area_pos[2] + title_dx + self.margin[2] as i32,
                actual_drawing_area_pos[0] + title_dy + self.margin[0] as i32,
            ),
        })
    }

    /**
    Builds a chart with a 3D Cartesian coordinate system.

    - `x_spec`: Specifies the X axis range and data properties
    - `y_spec`: Specifies the Y axis range and data properties
    - `z_sepc`: Specifies the Z axis range and data properties
    - Returns: A `ChartContext` object, ready to visualize data.

    See [`ChartBuilder::on()`] and [`ChartContext::configure_axes()`] for more information and examples.
    */
    #[allow(clippy::type_complexity)]
    pub fn build_cartesian_3d<'c, X: AsRangedCoord, Y: AsRangedCoord, Z: AsRangedCoord>(
        &mut self,
        x_spec: X,
        y_spec: Y,
        z_spec: Z,
    ) -> Result<
        ChartContext<'c, DB, Cartesian3d<X::CoordDescType, Y::CoordDescType, Z::CoordDescType>>,
        DrawingAreaErrorKind<DB::ErrorType>,
    > {
        let mut drawing_area = DrawingArea::clone(self.root_area);

        if *self.margin.iter().max().unwrap_or(&0) > 0 {
            drawing_area = drawing_area.margin(
                self.margin[0] as i32,
                self.margin[1] as i32,
                self.margin[2] as i32,
                self.margin[3] as i32,
            );
        }

        let (title_dx, title_dy) = if let Some((ref title, ref style)) = self.title {
            let (origin_dx, origin_dy) = drawing_area.get_base_pixel();
            drawing_area = drawing_area.titled(title, style.clone())?;
            let (current_dx, current_dy) = drawing_area.get_base_pixel();
            (current_dx - origin_dx, current_dy - origin_dy)
        } else {
            (0, 0)
        };

        let pixel_range = drawing_area.get_pixel_range();

        Ok(ChartContext {
            x_label_area: [None, None],
            y_label_area: [None, None],
            drawing_area: drawing_area.apply_coord_spec(Cartesian3d::new(
                x_spec,
                y_spec,
                z_spec,
                pixel_range,
            )),
            series_anno: vec![],
            drawing_area_pos: (
                title_dx + self.margin[2] as i32,
                title_dy + self.margin[0] as i32,
            ),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::*;
    #[test]
    fn test_label_area_size() {
        let drawing_area = create_mocked_drawing_area(200, 200, |_| {});
        let mut chart = ChartBuilder::on(&drawing_area);

        chart
            .x_label_area_size(10)
            .y_label_area_size(20)
            .top_x_label_area_size(30)
            .right_y_label_area_size(40);
        assert_eq!(chart.label_area_size[1], 10);
        assert_eq!(chart.label_area_size[2], 20);
        assert_eq!(chart.label_area_size[0], 30);
        assert_eq!(chart.label_area_size[3], 40);

        chart.set_label_area_size(LabelAreaPosition::Left, 100);
        chart.set_label_area_size(LabelAreaPosition::Right, 200);
        chart.set_label_area_size(LabelAreaPosition::Top, 300);
        chart.set_label_area_size(LabelAreaPosition::Bottom, 400);

        assert_eq!(chart.label_area_size[0], 300);
        assert_eq!(chart.label_area_size[1], 400);
        assert_eq!(chart.label_area_size[2], 100);
        assert_eq!(chart.label_area_size[3], 200);
    }

    #[test]
    fn test_margin_configure() {
        let drawing_area = create_mocked_drawing_area(200, 200, |_| {});
        let mut chart = ChartBuilder::on(&drawing_area);

        chart.margin(5);
        assert_eq!(chart.margin[0], 5);
        assert_eq!(chart.margin[1], 5);
        assert_eq!(chart.margin[2], 5);
        assert_eq!(chart.margin[3], 5);

        chart.margin_top(10);
        chart.margin_bottom(11);
        chart.margin_left(12);
        chart.margin_right(13);
        assert_eq!(chart.margin[0], 10);
        assert_eq!(chart.margin[1], 11);
        assert_eq!(chart.margin[2], 12);
        assert_eq!(chart.margin[3], 13);
    }

    #[test]
    fn test_caption() {
        let drawing_area = create_mocked_drawing_area(200, 200, |_| {});
        let mut chart = ChartBuilder::on(&drawing_area);

        chart.caption("This is a test case", ("serif", 10));

        assert_eq!(chart.title.as_ref().unwrap().0, "This is a test case");
        assert_eq!(chart.title.as_ref().unwrap().1.font.get_name(), "serif");
        assert_eq!(chart.title.as_ref().unwrap().1.font.get_size(), 10.0);
        check_color(chart.title.as_ref().unwrap().1.color, BLACK.to_rgba());

        chart.caption("This is a test case", ("serif", 10));
        assert_eq!(chart.title.as_ref().unwrap().1.font.get_name(), "serif");
    }

    #[test]
    fn test_zero_limit_with_log_scale() {
        let drawing_area = create_mocked_drawing_area(640, 480, |_| {});

        let mut chart = ChartBuilder::on(&drawing_area)
            .build_cartesian_2d(0f32..10f32, (1e-6f32..1f32).log_scale())
            .unwrap();

        let data = vec![
            (2f32, 1e-4f32),
            (4f32, 1e-3f32),
            (6f32, 1e-2f32),
            (8f32, 1e-1f32),
        ];

        chart
            .draw_series(
                data.iter()
                    .map(|&(x, y)| Rectangle::new([(x - 0.5, 0.0), (x + 0.5, y)], RED.filled())),
            )
            .unwrap();
    }
}
