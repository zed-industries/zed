use std::iter;
use std::process::Child;

use crate::stats::univariate::Sample;
use crate::stats::Distribution;
use criterion_plot::prelude::*;

use super::*;
use crate::estimate::Estimate;
use crate::estimate::Statistic;
use crate::kde;
use crate::measurement::ValueFormatter;
use crate::report::{BenchmarkId, ComparisonData, MeasurementData, ReportContext};

fn abs_distribution(
    id: &BenchmarkId,
    context: &ReportContext,
    formatter: &dyn ValueFormatter,
    statistic: Statistic,
    distribution: &Distribution<f64>,
    estimate: &Estimate,
    size: Option<Size>,
) -> Child {
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

    let zero = iter::repeat(0);

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

    let mut figure = Figure::new();
    figure
        .set(Font(DEFAULT_FONT))
        .set(size.unwrap_or(SIZE))
        .set(Title(format!(
            "{}: {}",
            gnuplot_escape(id.as_title()),
            statistic
        )))
        .configure(Axis::BottomX, |a| {
            a.set(Label(format!("Average time ({})", unit)))
                .set(Range::Limits(kde_xs_sample.min(), kde_xs_sample.max()))
        })
        .configure(Axis::LeftY, |a| a.set(Label("Density (a.u.)")))
        .configure(Key, |k| {
            k.set(Justification::Left)
                .set(Order::SampleText)
                .set(Position::Outside(Vertical::Top, Horizontal::Right))
        })
        .plot(
            Lines {
                x: &*kde_xs,
                y: &*ys,
            },
            |c| {
                c.set(DARK_BLUE)
                    .set(LINEWIDTH)
                    .set(Label("Bootstrap distribution"))
                    .set(LineType::Solid)
            },
        )
        .plot(
            FilledCurve {
                x: kde_xs.iter().skip(start).take(len),
                y1: ys.iter().skip(start),
                y2: zero,
            },
            |c| {
                c.set(DARK_BLUE)
                    .set(Label("Confidence interval"))
                    .set(Opacity(0.25))
            },
        )
        .plot(
            Lines {
                x: &[point, point],
                y: &[0., y_point],
            },
            |c| {
                c.set(DARK_BLUE)
                    .set(LINEWIDTH)
                    .set(Label("Point estimate"))
                    .set(LineType::Dash)
            },
        );

    let path = context.report_path(id, &format!("{}.svg", statistic));
    debug_script(&path, &figure);
    figure.set(Output(path)).draw().unwrap()
}

pub(crate) fn abs_distributions(
    id: &BenchmarkId,
    context: &ReportContext,
    formatter: &dyn ValueFormatter,
    measurements: &MeasurementData<'_>,
    size: Option<Size>,
) -> Vec<Child> {
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
        .map(|(statistic, distribution, estimate)| {
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
        .collect::<Vec<_>>()
}

fn rel_distribution(
    id: &BenchmarkId,
    context: &ReportContext,
    statistic: Statistic,
    distribution: &Distribution<f64>,
    estimate: &Estimate,
    noise_threshold: f64,
    size: Option<Size>,
) -> Child {
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

    let one = iter::repeat(1);
    let zero = iter::repeat(0);

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

    let mut figure = Figure::new();

    figure
        .set(Font(DEFAULT_FONT))
        .set(size.unwrap_or(SIZE))
        .configure(Axis::LeftY, |a| a.set(Label("Density (a.u.)")))
        .configure(Key, |k| {
            k.set(Justification::Left)
                .set(Order::SampleText)
                .set(Position::Outside(Vertical::Top, Horizontal::Right))
        })
        .set(Title(format!(
            "{}: {}",
            gnuplot_escape(id.as_title()),
            statistic
        )))
        .configure(Axis::BottomX, |a| {
            a.set(Label("Relative change (%)"))
                .set(Range::Limits(x_min * 100., x_max * 100.))
                .set(ScaleFactor(100.))
        })
        .plot(Lines { x: &*xs, y: &*ys }, |c| {
            c.set(DARK_BLUE)
                .set(LINEWIDTH)
                .set(Label("Bootstrap distribution"))
                .set(LineType::Solid)
        })
        .plot(
            FilledCurve {
                x: xs.iter().skip(start).take(len),
                y1: ys.iter().skip(start),
                y2: zero.clone(),
            },
            |c| {
                c.set(DARK_BLUE)
                    .set(Label("Confidence interval"))
                    .set(Opacity(0.25))
            },
        )
        .plot(
            Lines {
                x: &[point, point],
                y: &[0., y_point],
            },
            |c| {
                c.set(DARK_BLUE)
                    .set(LINEWIDTH)
                    .set(Label("Point estimate"))
                    .set(LineType::Dash)
            },
        )
        .plot(
            FilledCurve {
                x: &[fc_start, fc_end],
                y1: one,
                y2: zero,
            },
            |c| {
                c.set(Axes::BottomXRightY)
                    .set(DARK_RED)
                    .set(Label("Noise threshold"))
                    .set(Opacity(0.1))
            },
        );

    let path = context.report_path(id, &format!("change/{}.svg", statistic));
    debug_script(&path, &figure);
    figure.set(Output(path)).draw().unwrap()
}

pub(crate) fn rel_distributions(
    id: &BenchmarkId,
    context: &ReportContext,
    _measurements: &MeasurementData<'_>,
    comparison: &ComparisonData,
    size: Option<Size>,
) -> Vec<Child> {
    crate::plot::CHANGE_STATS
        .iter()
        .map(|&statistic| {
            rel_distribution(
                id,
                context,
                statistic,
                comparison.relative_distributions.get(statistic),
                comparison.relative_estimates.get(statistic),
                comparison.noise_threshold,
                size,
            )
        })
        .collect::<Vec<_>>()
}
