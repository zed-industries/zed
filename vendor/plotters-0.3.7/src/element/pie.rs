use crate::{
    element::{Drawable, PointCollection},
    style::{IntoFont, RGBColor, TextStyle, BLACK},
};
use plotters_backend::{BackendCoord, DrawingBackend, DrawingErrorKind};
use std::{error::Error, f64::consts::PI, fmt::Display};

#[derive(Debug)]
enum PieError {
    LengthMismatch,
}
impl Display for PieError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &PieError::LengthMismatch => write!(f, "Length Mismatch"),
        }
    }
}

impl Error for PieError {}

/// A Pie Graph
pub struct Pie<'a, Coord, Label: Display> {
    center: &'a Coord, // cartesian coord
    radius: &'a f64,
    sizes: &'a [f64],
    colors: &'a [RGBColor],
    labels: &'a [Label],
    total: f64,
    start_radian: f64,
    label_style: TextStyle<'a>,
    label_offset: f64,
    percentage_style: Option<TextStyle<'a>>,
    donut_hole: f64, // radius of the hole in case of a donut chart
}

impl<'a, Label: Display> Pie<'a, (i32, i32), Label> {
    /// Build a Pie object.
    /// Assumes a start angle at 0.0, which is aligned to the horizontal axis.
    pub fn new(
        center: &'a (i32, i32),
        radius: &'a f64,
        sizes: &'a [f64],
        colors: &'a [RGBColor],
        labels: &'a [Label],
    ) -> Self {
        // fold iterator to pre-calculate total from given slice sizes
        let total = sizes.iter().sum();

        // default label style and offset as 5% of the radius
        let radius_5pct = radius * 0.05;

        // strong assumption that the background is white for legibility.
        let label_style = TextStyle::from(("sans-serif", radius_5pct).into_font()).color(&BLACK);
        Self {
            center,
            radius,
            sizes,
            colors,
            labels,
            total,
            start_radian: 0.0,
            label_style,
            label_offset: radius_5pct,
            percentage_style: None,
            donut_hole: 0.0,
        }
    }

    /// Pass an angle in degrees to change the default.
    /// Default is set to start at 0, which is aligned on the x axis.
    /// ```
    /// use plotters::prelude::*;
    /// let mut pie = Pie::new(&(50,50), &10.0, &[50.0, 25.25, 20.0, 5.5], &[RED, BLUE, GREEN, WHITE], &["Red", "Blue", "Green", "White"]);
    /// pie.start_angle(-90.0);  // retract to a right angle, so it starts aligned to a vertical Y axis.
    /// ```
    pub fn start_angle(&mut self, start_angle: f64) {
        // angle is more intuitive in degrees as an API, but we use it as radian offset internally.
        self.start_radian = start_angle.to_radians();
    }

    /// Set the label style.
    pub fn label_style<T: Into<TextStyle<'a>>>(&mut self, label_style: T) {
        self.label_style = label_style.into();
    }

    /// Sets the offset to labels, to distanciate them further/closer from the center.
    pub fn label_offset(&mut self, offset_to_radius: f64) {
        self.label_offset = offset_to_radius
    }

    /// enables drawing the wedge's percentage in the middle of the wedge, with the given style
    pub fn percentages<T: Into<TextStyle<'a>>>(&mut self, label_style: T) {
        self.percentage_style = Some(label_style.into());
    }

    /// Enables creating a donut chart with a hole of the specified radius.
    ///
    /// The passed value must be greater than zero and lower than the chart overall radius, otherwise it'll be ignored.
    pub fn donut_hole(&mut self, hole_radius: f64) {
        if hole_radius > 0.0 && hole_radius < *self.radius {
            self.donut_hole = hole_radius;
        }
    }
}

impl<'a, DB: DrawingBackend, Label: Display> Drawable<DB> for Pie<'a, (i32, i32), Label> {
    fn draw<I: Iterator<Item = BackendCoord>>(
        &self,
        _pos: I,
        backend: &mut DB,
        _parent_dim: (u32, u32),
    ) -> Result<(), DrawingErrorKind<DB::ErrorType>> {
        let mut offset_theta = self.start_radian;

        // const reused for every radian calculation
        // the bigger the radius, the more fine-grained it should calculate
        // to avoid being aliasing from being too noticeable.
        // this all could be avoided if backend could draw a curve/bezier line as part of a polygon.
        let radian_increment = PI / 180.0 / self.radius.sqrt() * 2.0;
        let mut perc_labels = Vec::new();
        for (index, slice) in self.sizes.iter().enumerate() {
            let slice_style = self
                .colors
                .get(index)
                .ok_or_else(|| DrawingErrorKind::FontError(Box::new(PieError::LengthMismatch)))?;
            let label = self
                .labels
                .get(index)
                .ok_or_else(|| DrawingErrorKind::FontError(Box::new(PieError::LengthMismatch)))?;
            // start building wedge line against the previous edge
            let mut points = if self.donut_hole == 0.0 {
                vec![*self.center]
            } else {
                vec![]
            };
            let ratio = slice / self.total;
            let theta_final = ratio * 2.0 * PI + offset_theta; // end radian for the wedge

            // calculate middle for labels before mutating offset
            let middle_theta = ratio * PI + offset_theta;

            let slice_start = offset_theta;

            // calculate every fraction of radian for the wedge, offsetting for every iteration, clockwise
            //
            // a custom Range such as `for theta in offset_theta..=theta_final` would be more elegant
            // but f64 doesn't implement the Range trait, and it would requires the Step trait (increment by 1.0 or 0.0001?)
            // which is unstable therefore cannot be implemented outside of std, even as a newtype for radians.
            while offset_theta <= theta_final {
                let coord = theta_to_ordinal_coord(*self.radius, offset_theta, self.center);
                points.push(coord);
                offset_theta += radian_increment;
            }
            // final point of the wedge may not fall exactly on a radian, so add it extra
            let final_coord = theta_to_ordinal_coord(*self.radius, theta_final, self.center);
            points.push(final_coord);

            if self.donut_hole > 0.0 {
                while offset_theta >= slice_start {
                    let coord = theta_to_ordinal_coord(self.donut_hole, offset_theta, self.center);
                    points.push(coord);
                    offset_theta -= radian_increment;
                }
                // final point of the wedge may not fall exactly on a radian, so add it extra
                let final_coord_inner =
                    theta_to_ordinal_coord(self.donut_hole, slice_start, self.center);
                points.push(final_coord_inner);
            }

            // next wedge calculation will start from previous wedges's last radian
            offset_theta = theta_final;

            // draw wedge
            // TODO: Currently the backend doesn't have API to draw an arc. We need add that in the
            // future
            backend.fill_polygon(points, slice_style)?;

            // label coords from the middle
            let mut mid_coord =
                theta_to_ordinal_coord(self.radius + self.label_offset, middle_theta, self.center);

            // ensure label's doesn't fall in the circle
            let label_size = backend.estimate_text_size(&label.to_string(), &self.label_style)?;
            // if on the left hand side of the pie, offset whole label to the left
            if mid_coord.0 <= self.center.0 {
                mid_coord.0 -= label_size.0 as i32;
            }
            // put label
            backend.draw_text(&label.to_string(), &self.label_style, mid_coord)?;
            if let Some(percentage_style) = &self.percentage_style {
                let perc_label = format!("{:.1}%", (ratio * 100.0));
                let label_size = backend.estimate_text_size(&perc_label, percentage_style)?;
                let text_x_mid = (label_size.0 as f64 / 2.0).round() as i32;
                let text_y_mid = (label_size.1 as f64 / 2.0).round() as i32;
                let perc_radius = (self.radius + self.donut_hole) / 2.0;
                let perc_coord = theta_to_ordinal_coord(
                    perc_radius,
                    middle_theta,
                    &(self.center.0 - text_x_mid, self.center.1 - text_y_mid),
                );
                // perc_coord.0 -= middle_label_size.0.round() as i32;
                perc_labels.push((perc_label, perc_coord));
            }
        }
        // while percentages are generated during the first main iterations,
        // they have to go on top of the already drawn wedges, so require a new iteration.
        for (label, coord) in perc_labels {
            let style = self.percentage_style.as_ref().unwrap();
            backend.draw_text(&label, style, coord)?;
        }
        Ok(())
    }
}

impl<'a, Label: Display> PointCollection<'a, (i32, i32)> for &'a Pie<'a, (i32, i32), Label> {
    type Point = &'a (i32, i32);
    type IntoIter = std::iter::Once<&'a (i32, i32)>;
    fn point_iter(self) -> std::iter::Once<&'a (i32, i32)> {
        std::iter::once(self.center)
    }
}

fn theta_to_ordinal_coord(radius: f64, theta: f64, ordinal_offset: &(i32, i32)) -> (i32, i32) {
    // polar coordinates are (r, theta)
    // convert to (x, y) coord, with center as offset

    let (sin, cos) = theta.sin_cos();
    (
        // casting f64 to discrete i32 pixels coordinates is inevitably going to lose precision
        // if plotters can support float coordinates, this place would surely benefit, especially for small sizes.
        // so far, the result isn't so bad though
        (radius * cos + ordinal_offset.0 as f64).round() as i32, // x
        (radius * sin + ordinal_offset.1 as f64).round() as i32, // y
    )
}
#[cfg(test)]
mod test {
    use super::*;
    // use crate::prelude::*;

    #[test]
    fn polar_coord_to_cartestian_coord() {
        let coord = theta_to_ordinal_coord(800.0, 1.5_f64.to_radians(), &(5, 5));
        // rounded tends to be more accurate. this gets truncated to (804, 25) without rounding.
        assert_eq!(coord, (805, 26)); //coord calculated from theta
    }
    #[test]
    fn pie_calculations() {
        let mut center = (5, 5);
        let mut radius = 800.0;

        let sizes = vec![50.0, 25.0];
        // length isn't validated in new()
        let colors = vec![];
        let labels: Vec<&str> = vec![];
        let pie = Pie::new(&center, &radius, &sizes, &colors, &labels);
        assert_eq!(pie.total, 75.0); // total calculated from sizes

        // not ownership greedy
        center.1 += 1;
        radius += 1.0;
        assert!(colors.get(0).is_none());
        assert!(labels.first().is_none());
        assert_eq!(radius, 801.0);
    }
}
