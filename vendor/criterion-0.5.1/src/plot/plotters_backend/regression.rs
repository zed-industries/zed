use super::*;

use std::path::Path;

use crate::estimate::{ConfidenceInterval, Estimate};
use crate::stats::bivariate::regression::Slope;
use crate::stats::bivariate::Data;

pub(crate) fn regression_figure(
    title: Option<&str>,
    path: &Path,
    formatter: &dyn ValueFormatter,
    measurements: &MeasurementData<'_>,
    size: Option<(u32, u32)>,
) {
    let slope_estimate = measurements.absolute_estimates.slope.as_ref().unwrap();
    let slope_dist = measurements.distributions.slope.as_ref().unwrap();
    let (lb, ub) =
        slope_dist.confidence_interval(slope_estimate.confidence_interval.confidence_level);

    let data = &measurements.data;
    let (max_iters, typical) = (data.x().max(), data.y().max());
    let mut scaled_y: Vec<f64> = data.y().iter().cloned().collect();
    let unit = formatter.scale_values(typical, &mut scaled_y);
    let scaled_y = Sample::new(&scaled_y);

    let point_estimate = Slope::fit(&measurements.data).0;
    let mut scaled_points = [point_estimate * max_iters, lb * max_iters, ub * max_iters];
    let _ = formatter.scale_values(typical, &mut scaled_points);
    let [point, lb, ub] = scaled_points;

    let exponent = (max_iters.log10() / 3.).floor() as i32 * 3;

    let x_scale = 10f64.powi(-exponent);
    let x_label = if exponent == 0 {
        "Iterations".to_owned()
    } else {
        format!("Iterations (x 10^{})", exponent)
    };

    let size = size.unwrap_or(SIZE);
    let root_area = SVGBackend::new(path, size).into_drawing_area();

    let mut cb = ChartBuilder::on(&root_area);
    if let Some(title) = title {
        cb.caption(title, (DEFAULT_FONT, 20));
    }

    let x_range = plotters::data::fitting_range(data.x().iter());
    let y_range = plotters::data::fitting_range(scaled_y.iter());

    let mut chart = cb
        .margin((5).percent())
        .set_label_area_size(LabelAreaPosition::Left, (5).percent_width().min(60))
        .set_label_area_size(LabelAreaPosition::Bottom, (5).percent_height().min(40))
        .build_cartesian_2d(x_range, y_range)
        .unwrap();

    chart
        .configure_mesh()
        .x_desc(x_label)
        .y_desc(format!("Total sample time ({})", unit))
        .x_label_formatter(&|x| pretty_print_float(x * x_scale, true))
        .light_line_style(TRANSPARENT)
        .draw()
        .unwrap();

    chart
        .draw_series(
            data.x()
                .iter()
                .zip(scaled_y.iter())
                .map(|(x, y)| Circle::new((*x, *y), POINT_SIZE, DARK_BLUE.filled())),
        )
        .unwrap()
        .label("Sample")
        .legend(|(x, y)| Circle::new((x + 10, y), POINT_SIZE, DARK_BLUE.filled()));

    chart
        .draw_series(std::iter::once(PathElement::new(
            vec![(0.0, 0.0), (max_iters, point)],
            DARK_BLUE,
        )))
        .unwrap()
        .label("Linear regression")
        .legend(|(x, y)| {
            PathElement::new(
                vec![(x, y), (x + 20, y)],
                DARK_BLUE.filled().stroke_width(2),
            )
        });

    chart
        .draw_series(std::iter::once(Polygon::new(
            vec![(0.0, 0.0), (max_iters, lb), (max_iters, ub)],
            DARK_BLUE.mix(0.25).filled(),
        )))
        .unwrap()
        .label("Confidence interval")
        .legend(|(x, y)| {
            Rectangle::new([(x, y - 5), (x + 20, y + 5)], DARK_BLUE.mix(0.25).filled())
        });

    if title.is_some() {
        chart
            .configure_series_labels()
            .position(SeriesLabelPosition::UpperLeft)
            .draw()
            .unwrap();
    }
}

pub(crate) fn regression_comparison_figure(
    title: Option<&str>,
    path: &Path,
    formatter: &dyn ValueFormatter,
    measurements: &MeasurementData<'_>,
    comparison: &ComparisonData,
    base_data: &Data<'_, f64, f64>,
    size: Option<(u32, u32)>,
) {
    let data = &measurements.data;
    let max_iters = base_data.x().max().max(data.x().max());
    let typical = base_data.y().max().max(data.y().max());

    let exponent = (max_iters.log10() / 3.).floor() as i32 * 3;
    let x_scale = 10f64.powi(-exponent);

    let x_label = if exponent == 0 {
        "Iterations".to_owned()
    } else {
        format!("Iterations (x 10^{})", exponent)
    };

    let Estimate {
        confidence_interval:
            ConfidenceInterval {
                lower_bound: base_lb,
                upper_bound: base_ub,
                ..
            },
        point_estimate: base_point,
        ..
    } = comparison.base_estimates.slope.as_ref().unwrap();

    let Estimate {
        confidence_interval:
            ConfidenceInterval {
                lower_bound: lb,
                upper_bound: ub,
                ..
            },
        point_estimate: point,
        ..
    } = measurements.absolute_estimates.slope.as_ref().unwrap();

    let mut points = [
        base_lb * max_iters,
        base_point * max_iters,
        base_ub * max_iters,
        lb * max_iters,
        point * max_iters,
        ub * max_iters,
    ];
    let unit = formatter.scale_values(typical, &mut points);
    let [base_lb, base_point, base_ub, lb, point, ub] = points;

    let y_max = point.max(base_point);

    let size = size.unwrap_or(SIZE);
    let root_area = SVGBackend::new(path, size).into_drawing_area();

    let mut cb = ChartBuilder::on(&root_area);
    if let Some(title) = title {
        cb.caption(title, (DEFAULT_FONT, 20));
    }

    let mut chart = cb
        .margin((5).percent())
        .set_label_area_size(LabelAreaPosition::Left, (5).percent_width().min(60))
        .set_label_area_size(LabelAreaPosition::Bottom, (5).percent_height().min(40))
        .build_cartesian_2d(0.0..max_iters, 0.0..y_max)
        .unwrap();

    chart
        .configure_mesh()
        .x_desc(x_label)
        .y_desc(format!("Total sample time ({})", unit))
        .x_label_formatter(&|x| pretty_print_float(x * x_scale, true))
        .light_line_style(TRANSPARENT)
        .draw()
        .unwrap();

    chart
        .draw_series(vec![
            PathElement::new(vec![(0.0, 0.0), (max_iters, base_point)], DARK_RED).into_dyn(),
            Polygon::new(
                vec![(0.0, 0.0), (max_iters, base_lb), (max_iters, base_ub)],
                DARK_RED.mix(0.25).filled(),
            )
            .into_dyn(),
        ])
        .unwrap()
        .label("Base Sample")
        .legend(|(x, y)| {
            PathElement::new(vec![(x, y), (x + 20, y)], DARK_RED.filled().stroke_width(2))
        });

    chart
        .draw_series(vec![
            PathElement::new(vec![(0.0, 0.0), (max_iters, point)], DARK_BLUE).into_dyn(),
            Polygon::new(
                vec![(0.0, 0.0), (max_iters, lb), (max_iters, ub)],
                DARK_BLUE.mix(0.25).filled(),
            )
            .into_dyn(),
        ])
        .unwrap()
        .label("New Sample")
        .legend(|(x, y)| {
            PathElement::new(
                vec![(x, y), (x + 20, y)],
                DARK_BLUE.filled().stroke_width(2),
            )
        });

    if title.is_some() {
        chart
            .configure_series_labels()
            .position(SeriesLabelPosition::UpperLeft)
            .draw()
            .unwrap();
    }
}
