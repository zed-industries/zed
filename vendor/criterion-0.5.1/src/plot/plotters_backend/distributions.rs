use super::*;
use crate::estimate::Estimate;
use crate::estimate::Statistic;
use crate::measurement::ValueFormatter;
use crate::report::{BenchmarkId, MeasurementData, ReportContext};
use crate::stats::Distribution;

fn abs_distribution(
    id: &BenchmarkId,
    context: &ReportContext,
    formatter: &dyn ValueFormatter,
    statistic: Statistic,
    distribution: &Distribution<f64>,
    estimate: &Estimate,
    size: Option<(u32, u32)>,
) {
    let ci = &estimate.confidence_interval;
    let typical = ci.upper_bound;
    let mut ci_values = [ci.lower_bound, ci.upper_bound, estimate.point_estimate];
    let unit = formatter.scale_values(typical, &mut ci_values);
    let (lb, ub, point) = (ci_values[0], ci_values[1], ci_values[2]);

    let start = lb - (ub - lb) / 9.;
    let end = ub + (ub - lb) / 9.;
    let mut scaled_xs: Vec<f64> = distribution.iter().cloned().collect();
    let _ = formatter.scale_values(typical, &mut scaled_xs);
    let scaled_xs_sample = Sample::new(&scaled_xs);
    let (kde_xs, ys) = kde::sweep(scaled_xs_sample, KDE_POINTS, Some((start, end)));

    // interpolate between two points of the KDE sweep to find the Y position at the point estimate.
    let n_point = kde_xs
        .iter()
        .position(|&x| x >= point)
        .unwrap_or(kde_xs.len() - 1)
        .max(1); // Must be at least the second element or this will panic
    let slope = (ys[n_point] - ys[n_point - 1]) / (kde_xs[n_point] - kde_xs[n_point - 1]);
    let y_point = ys[n_point - 1] + (slope * (point - kde_xs[n_point - 1]));

    let start = kde_xs
        .iter()
        .enumerate()
        .find(|&(_, &x)| x >= lb)
        .unwrap()
        .0;
    let end = kde_xs
        .iter()
        .enumerate()
        .rev()
        .find(|&(_, &x)| x <= ub)
        .unwrap()
        .0;
    let len = end - start;

    let kde_xs_sample = Sample::new(&kde_xs);

    let path = context.report_path(id, &format!("{}.svg", statistic));
    let root_area = SVGBackend::new(&path, size.unwrap_or(SIZE)).into_drawing_area();

    let x_range = plotters::data::fitting_range(kde_xs_sample.iter());
    let mut y_range = plotters::data::fitting_range(ys.iter());

    y_range.end *= 1.1;

    let mut chart = ChartBuilder::on(&root_area)
        .margin((5).percent())
        .caption(
            format!("{}:{}", id.as_title(), statistic),
            (DEFAULT_FONT, 20),
        )
        .set_label_area_size(LabelAreaPosition::Left, (5).percent_width().min(60))
        .set_label_area_size(LabelAreaPosition::Bottom, (5).percent_height().min(40))
        .build_cartesian_2d(x_range, y_range)
        .unwrap();

    chart
        .configure_mesh()
        .disable_mesh()
        .x_desc(format!("Average time ({})", unit))
        .y_desc("Density (a.u.)")
        .x_label_formatter(&|&v| pretty_print_float(v, true))
        .y_label_formatter(&|&v| pretty_print_float(v, true))
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            kde_xs.iter().zip(ys.iter()).map(|(&x, &y)| (x, y)),
            DARK_BLUE,
        ))
        .unwrap()
        .label("Bootstrap distribution")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], DARK_BLUE));

    chart
        .draw_series(AreaSeries::new(
            kde_xs
                .iter()
                .zip(ys.iter())
                .skip(start)
                .take(len)
                .map(|(&x, &y)| (x, y)),
            0.0,
            DARK_BLUE.mix(0.25).filled().stroke_width(3),
        ))
        .unwrap()
        .label("Confidence interval")
        .legend(|(x, y)| {
            Rectangle::new([(x, y - 5), (x + 20, y + 5)], DARK_BLUE.mix(0.25).filled())
        });

    chart
        .draw_series(std::iter::once(PathElement::new(
            vec![(point, 0.0), (point, y_point)],
            DARK_BLUE.filled().stroke_width(3),
        )))
        .unwrap()
        .label("Point estimate")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], DARK_BLUE));

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .draw()
        .unwrap();
}

pub(crate) fn abs_distributions(
    id: &BenchmarkId,
    context: &ReportContext,
    formatter: &dyn ValueFormatter,
    measurements: &MeasurementData<'_>,
    size: Option<(u32, u32)>,
) {
    crate::plot::REPORT_STATS
        .iter()
        .filter_map(|stat| {
            measurements.distributions.get(*stat).and_then(|dist| {
                measurements
                    .absolute_estimates
                    .get(*stat)
                    .map(|est| (*stat, dist, est))
            })
        })
        .for_each(|(statistic, distribution, estimate)| {
            abs_distribution(
                id,
                context,
                formatter,
                statistic,
                distribution,
                estimate,
                size,
            )
        })
}

fn rel_distribution(
    id: &BenchmarkId,
    context: &ReportContext,
    statistic: Statistic,
    distribution: &Distribution<f64>,
    estimate: &Estimate,
    noise_threshold: f64,
    size: Option<(u32, u32)>,
) {
    let ci = &estimate.confidence_interval;
    let (lb, ub) = (ci.lower_bound, ci.upper_bound);

    let start = lb - (ub - lb) / 9.;
    let end = ub + (ub - lb) / 9.;
    let (xs, ys) = kde::sweep(distribution, KDE_POINTS, Some((start, end)));
    let xs_ = Sample::new(&xs);

    // interpolate between two points of the KDE sweep to find the Y position at the point estimate.
    let point = estimate.point_estimate;
    let n_point = xs
        .iter()
        .position(|&x| x >= point)
        .unwrap_or(ys.len() - 1)
        .max(1);
    let slope = (ys[n_point] - ys[n_point - 1]) / (xs[n_point] - xs[n_point - 1]);
    let y_point = ys[n_point - 1] + (slope * (point - xs[n_point - 1]));

    let start = xs.iter().enumerate().find(|&(_, &x)| x >= lb).unwrap().0;
    let end = xs
        .iter()
        .enumerate()
        .rev()
        .find(|&(_, &x)| x <= ub)
        .unwrap()
        .0;
    let len = end - start;

    let x_min = xs_.min();
    let x_max = xs_.max();

    let (fc_start, fc_end) = if noise_threshold < x_min || -noise_threshold > x_max {
        let middle = (x_min + x_max) / 2.;

        (middle, middle)
    } else {
        (
            if -noise_threshold < x_min {
                x_min
            } else {
                -noise_threshold
            },
            if noise_threshold > x_max {
                x_max
            } else {
                noise_threshold
            },
        )
    };
    let y_range = plotters::data::fitting_range(ys.iter());
    let path = context.report_path(id, &format!("change/{}.svg", statistic));
    let root_area = SVGBackend::new(&path, size.unwrap_or(SIZE)).into_drawing_area();

    let mut chart = ChartBuilder::on(&root_area)
        .margin((5).percent())
        .caption(
            format!("{}:{}", id.as_title(), statistic),
            (DEFAULT_FONT, 20),
        )
        .set_label_area_size(LabelAreaPosition::Left, (5).percent_width().min(60))
        .set_label_area_size(LabelAreaPosition::Bottom, (5).percent_height().min(40))
        .build_cartesian_2d(x_min..x_max, y_range.clone())
        .unwrap();

    chart
        .configure_mesh()
        .disable_mesh()
        .x_desc("Relative change (%)")
        .y_desc("Density (a.u.)")
        .x_label_formatter(&|&v| pretty_print_float(v, true))
        .y_label_formatter(&|&v| pretty_print_float(v, true))
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            xs.iter().zip(ys.iter()).map(|(x, y)| (*x, *y)),
            DARK_BLUE,
        ))
        .unwrap()
        .label("Bootstrap distribution")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], DARK_BLUE));

    chart
        .draw_series(AreaSeries::new(
            xs.iter()
                .zip(ys.iter())
                .skip(start)
                .take(len)
                .map(|(x, y)| (*x, *y)),
            0.0,
            DARK_BLUE.mix(0.25).filled().stroke_width(3),
        ))
        .unwrap()
        .label("Confidence interval")
        .legend(|(x, y)| {
            Rectangle::new([(x, y - 5), (x + 20, y + 5)], DARK_BLUE.mix(0.25).filled())
        });

    chart
        .draw_series(std::iter::once(PathElement::new(
            vec![(point, 0.0), (point, y_point)],
            DARK_BLUE.filled().stroke_width(3),
        )))
        .unwrap()
        .label("Point estimate")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], DARK_BLUE));

    chart
        .draw_series(std::iter::once(Rectangle::new(
            [(fc_start, y_range.start), (fc_end, y_range.end)],
            DARK_RED.mix(0.1).filled(),
        )))
        .unwrap()
        .label("Noise threshold")
        .legend(|(x, y)| {
            Rectangle::new([(x, y - 5), (x + 20, y + 5)], DARK_RED.mix(0.25).filled())
        });
    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .draw()
        .unwrap();
}

pub(crate) fn rel_distributions(
    id: &BenchmarkId,
    context: &ReportContext,
    _measurements: &MeasurementData<'_>,
    comparison: &ComparisonData,
    size: Option<(u32, u32)>,
) {
    crate::plot::CHANGE_STATS.iter().for_each(|&statistic| {
        rel_distribution(
            id,
            context,
            statistic,
            comparison.relative_distributions.get(statistic),
            comparison.relative_estimates.get(statistic),
            comparison.noise_threshold,
            size,
        )
    });
}
