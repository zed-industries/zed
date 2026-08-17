use super::*;
use crate::kde;
use crate::measurement::ValueFormatter;
use crate::report::{BenchmarkId, ComparisonData, MeasurementData, ReportContext};
use std::process::Child;

pub(crate) fn pdf(
    id: &BenchmarkId,
    context: &ReportContext,
    formatter: &dyn ValueFormatter,
    measurements: &MeasurementData<'_>,
    size: Option<Size>,
) -> Child {
    let avg_times = &measurements.avg_times;
    let typical = avg_times.max();
    let mut scaled_avg_times: Vec<f64> = (avg_times as &Sample<f64>).iter().cloned().collect();
    let unit = formatter.scale_values(typical, &mut scaled_avg_times);
    let scaled_avg_times = Sample::new(&scaled_avg_times);

    let mean = scaled_avg_times.mean();

    let iter_counts = measurements.iter_counts();
    let &max_iters = iter_counts
        .iter()
        .max_by_key(|&&iters| iters as u64)
        .unwrap();
    let exponent = (max_iters.log10() / 3.).floor() as i32 * 3;
    let y_scale = 10f64.powi(-exponent);

    let y_label = if exponent == 0 {
        "Iterations".to_owned()
    } else {
        format!("Iterations (x 10^{})", exponent)
    };

    let (xs, ys) = kde::sweep(scaled_avg_times, KDE_POINTS, None);
    let (lost, lomt, himt, hist) = avg_times.fences();
    let mut fences = [lost, lomt, himt, hist];
    let _ = formatter.scale_values(typical, &mut fences);
    let [lost, lomt, himt, hist] = fences;

    let vertical = &[0., max_iters];
    let zeros = iter::repeat(0);

    let mut figure = Figure::new();
    figure
        .set(Font(DEFAULT_FONT))
        .set(size.unwrap_or(SIZE))
        .configure(Axis::BottomX, |a| {
            let xs_ = Sample::new(&xs);
            a.set(Label(format!("Average time ({})", unit)))
                .set(Range::Limits(xs_.min(), xs_.max()))
        })
        .configure(Axis::LeftY, |a| {
            a.set(Label(y_label))
                .set(Range::Limits(0., max_iters * y_scale))
                .set(ScaleFactor(y_scale))
        })
        .configure(Axis::RightY, |a| a.set(Label("Density (a.u.)")))
        .configure(Key, |k| {
            k.set(Justification::Left)
                .set(Order::SampleText)
                .set(Position::Outside(Vertical::Top, Horizontal::Right))
        })
        .plot(
            FilledCurve {
                x: &*xs,
                y1: &*ys,
                y2: zeros,
            },
            |c| {
                c.set(Axes::BottomXRightY)
                    .set(DARK_BLUE)
                    .set(Label("PDF"))
                    .set(Opacity(0.25))
            },
        )
        .plot(
            Lines {
                x: &[mean, mean],
                y: vertical,
            },
            |c| {
                c.set(DARK_BLUE)
                    .set(LINEWIDTH)
                    .set(LineType::Dash)
                    .set(Label("Mean"))
            },
        )
        .plot(
            Points {
                x: avg_times
                    .iter()
                    .zip(scaled_avg_times.iter())
                    .filter_map(
                        |((_, label), t)| {
                            if label.is_outlier() {
                                None
                            } else {
                                Some(t)
                            }
                        },
                    ),
                y: avg_times
                    .iter()
                    .zip(iter_counts.iter())
                    .filter_map(
                        |((_, label), i)| {
                            if label.is_outlier() {
                                None
                            } else {
                                Some(i)
                            }
                        },
                    ),
            },
            |c| {
                c.set(DARK_BLUE)
                    .set(Label("\"Clean\" sample"))
                    .set(PointType::FilledCircle)
                    .set(POINT_SIZE)
            },
        )
        .plot(
            Points {
                x: avg_times
                    .iter()
                    .zip(scaled_avg_times.iter())
                    .filter_map(
                        |((_, label), t)| {
                            if label.is_mild() {
                                Some(t)
                            } else {
                                None
                            }
                        },
                    ),
                y: avg_times
                    .iter()
                    .zip(iter_counts.iter())
                    .filter_map(
                        |((_, label), i)| {
                            if label.is_mild() {
                                Some(i)
                            } else {
                                None
                            }
                        },
                    ),
            },
            |c| {
                c.set(DARK_ORANGE)
                    .set(Label("Mild outliers"))
                    .set(POINT_SIZE)
                    .set(PointType::FilledCircle)
            },
        )
        .plot(
            Points {
                x: avg_times
                    .iter()
                    .zip(scaled_avg_times.iter())
                    .filter_map(
                        |((_, label), t)| {
                            if label.is_severe() {
                                Some(t)
                            } else {
                                None
                            }
                        },
                    ),
                y: avg_times
                    .iter()
                    .zip(iter_counts.iter())
                    .filter_map(
                        |((_, label), i)| {
                            if label.is_severe() {
                                Some(i)
                            } else {
                                None
                            }
                        },
                    ),
            },
            |c| {
                c.set(DARK_RED)
                    .set(Label("Severe outliers"))
                    .set(POINT_SIZE)
                    .set(PointType::FilledCircle)
            },
        )
        .plot(
            Lines {
                x: &[lomt, lomt],
                y: vertical,
            },
            |c| c.set(DARK_ORANGE).set(LINEWIDTH).set(LineType::Dash),
        )
        .plot(
            Lines {
                x: &[himt, himt],
                y: vertical,
            },
            |c| c.set(DARK_ORANGE).set(LINEWIDTH).set(LineType::Dash),
        )
        .plot(
            Lines {
                x: &[lost, lost],
                y: vertical,
            },
            |c| c.set(DARK_RED).set(LINEWIDTH).set(LineType::Dash),
        )
        .plot(
            Lines {
                x: &[hist, hist],
                y: vertical,
            },
            |c| c.set(DARK_RED).set(LINEWIDTH).set(LineType::Dash),
        );
    figure.set(Title(gnuplot_escape(id.as_title())));

    let path = context.report_path(id, "pdf.svg");
    debug_script(&path, &figure);
    figure.set(Output(path)).draw().unwrap()
}

pub(crate) fn pdf_small(
    id: &BenchmarkId,
    context: &ReportContext,
    formatter: &dyn ValueFormatter,
    measurements: &MeasurementData<'_>,
    size: Option<Size>,
) -> Child {
    let avg_times = &*measurements.avg_times;
    let typical = avg_times.max();
    let mut scaled_avg_times: Vec<f64> = (avg_times as &Sample<f64>).iter().cloned().collect();
    let unit = formatter.scale_values(typical, &mut scaled_avg_times);
    let scaled_avg_times = Sample::new(&scaled_avg_times);
    let mean = scaled_avg_times.mean();

    let (xs, ys, mean_y) = kde::sweep_and_estimate(scaled_avg_times, KDE_POINTS, None, mean);
    let xs_ = Sample::new(&xs);
    let ys_ = Sample::new(&ys);

    let y_limit = ys_.max() * 1.1;
    let zeros = iter::repeat(0);

    let mut figure = Figure::new();
    figure
        .set(Font(DEFAULT_FONT))
        .set(size.unwrap_or(SIZE))
        .configure(Axis::BottomX, |a| {
            a.set(Label(format!("Average time ({})", unit)))
                .set(Range::Limits(xs_.min(), xs_.max()))
        })
        .configure(Axis::LeftY, |a| {
            a.set(Label("Density (a.u.)"))
                .set(Range::Limits(0., y_limit))
        })
        .configure(Axis::RightY, |a| a.hide())
        .configure(Key, |k| k.hide())
        .plot(
            FilledCurve {
                x: &*xs,
                y1: &*ys,
                y2: zeros,
            },
            |c| {
                c.set(Axes::BottomXRightY)
                    .set(DARK_BLUE)
                    .set(Label("PDF"))
                    .set(Opacity(0.25))
            },
        )
        .plot(
            Lines {
                x: &[mean, mean],
                y: &[0., mean_y],
            },
            |c| c.set(DARK_BLUE).set(LINEWIDTH).set(Label("Mean")),
        );

    let path = context.report_path(id, "pdf_small.svg");
    debug_script(&path, &figure);
    figure.set(Output(path)).draw().unwrap()
}

fn pdf_comparison_figure(
    formatter: &dyn ValueFormatter,
    measurements: &MeasurementData<'_>,
    comparison: &ComparisonData,
    size: Option<Size>,
) -> Figure {
    let base_avg_times = Sample::new(&comparison.base_avg_times);
    let typical = base_avg_times.max().max(measurements.avg_times.max());
    let mut scaled_base_avg_times: Vec<f64> = comparison.base_avg_times.clone();
    let unit = formatter.scale_values(typical, &mut scaled_base_avg_times);
    let scaled_base_avg_times = Sample::new(&scaled_base_avg_times);

    let mut scaled_new_avg_times: Vec<f64> = (&measurements.avg_times as &Sample<f64>)
        .iter()
        .cloned()
        .collect();
    let _ = formatter.scale_values(typical, &mut scaled_new_avg_times);
    let scaled_new_avg_times = Sample::new(&scaled_new_avg_times);

    let base_mean = scaled_base_avg_times.mean();
    let new_mean = scaled_new_avg_times.mean();

    let (base_xs, base_ys, base_y_mean) =
        kde::sweep_and_estimate(scaled_base_avg_times, KDE_POINTS, None, base_mean);
    let (xs, ys, y_mean) =
        kde::sweep_and_estimate(scaled_new_avg_times, KDE_POINTS, None, new_mean);

    let zeros = iter::repeat(0);

    let mut figure = Figure::new();
    figure
        .set(Font(DEFAULT_FONT))
        .set(size.unwrap_or(SIZE))
        .configure(Axis::BottomX, |a| {
            a.set(Label(format!("Average time ({})", unit)))
        })
        .configure(Axis::LeftY, |a| a.set(Label("Density (a.u.)")))
        .configure(Axis::RightY, |a| a.hide())
        .configure(Key, |k| {
            k.set(Justification::Left)
                .set(Order::SampleText)
                .set(Position::Outside(Vertical::Top, Horizontal::Right))
        })
        .plot(
            FilledCurve {
                x: &*base_xs,
                y1: &*base_ys,
                y2: zeros.clone(),
            },
            |c| c.set(DARK_RED).set(Label("Base PDF")).set(Opacity(0.5)),
        )
        .plot(
            Lines {
                x: &[base_mean, base_mean],
                y: &[0., base_y_mean],
            },
            |c| c.set(DARK_RED).set(Label("Base Mean")).set(LINEWIDTH),
        )
        .plot(
            FilledCurve {
                x: &*xs,
                y1: &*ys,
                y2: zeros,
            },
            |c| c.set(DARK_BLUE).set(Label("New PDF")).set(Opacity(0.5)),
        )
        .plot(
            Lines {
                x: &[new_mean, new_mean],
                y: &[0., y_mean],
            },
            |c| c.set(DARK_BLUE).set(Label("New Mean")).set(LINEWIDTH),
        );
    figure
}

pub(crate) fn pdf_comparison(
    id: &BenchmarkId,
    context: &ReportContext,
    formatter: &dyn ValueFormatter,
    measurements: &MeasurementData<'_>,
    comparison: &ComparisonData,
    size: Option<Size>,
) -> Child {
    let mut figure = pdf_comparison_figure(formatter, measurements, comparison, size);
    figure.set(Title(gnuplot_escape(id.as_title())));
    let path = context.report_path(id, "both/pdf.svg");
    debug_script(&path, &figure);
    figure.set(Output(path)).draw().unwrap()
}

pub(crate) fn pdf_comparison_small(
    id: &BenchmarkId,
    context: &ReportContext,
    formatter: &dyn ValueFormatter,
    measurements: &MeasurementData<'_>,
    comparison: &ComparisonData,
    size: Option<Size>,
) -> Child {
    let mut figure = pdf_comparison_figure(formatter, measurements, comparison, size);
    figure.configure(Key, |k| k.hide());
    let path = context.report_path(id, "relative_pdf_small.svg");
    debug_script(&path, &figure);
    figure.set(Output(path)).draw().unwrap()
}
