use std::process::Child;

use criterion_plot::prelude::*;

use super::*;
use crate::report::{BenchmarkId, ComparisonData, MeasurementData, ReportContext};

use crate::measurement::ValueFormatter;

fn iteration_times_figure(
    formatter: &dyn ValueFormatter,
    measurements: &MeasurementData<'_>,
    size: Option<Size>,
) -> Figure {
    let data = &measurements.avg_times;
    let max_avg_time = data.max();
    let mut scaled_y: Vec<_> = data.iter().map(|(f, _)| f).collect();
    let unit = formatter.scale_values(max_avg_time, &mut scaled_y);
    let scaled_y = Sample::new(&scaled_y);

    let mut figure = Figure::new();
    figure
        .set(Font(DEFAULT_FONT))
        .set(size.unwrap_or(SIZE))
        .configure(Axis::BottomX, |a| {
            a.configure(Grid::Major, |g| g.show()).set(Label("Sample"))
        })
        .configure(Axis::LeftY, |a| {
            a.configure(Grid::Major, |g| g.show())
                .set(Label(format!("Average Iteration Time ({})", unit)))
        })
        .plot(
            Points {
                x: 1..(data.len() + 1),
                y: scaled_y.as_ref(),
            },
            |c| {
                c.set(DARK_BLUE)
                    .set(PointSize(0.5))
                    .set(PointType::FilledCircle)
            },
        );
    figure
}

pub(crate) fn iteration_times(
    id: &BenchmarkId,
    context: &ReportContext,
    formatter: &dyn ValueFormatter,
    measurements: &MeasurementData<'_>,
    size: Option<Size>,
) -> Child {
    let mut figure = iteration_times_figure(formatter, measurements, size);
    figure.set(Title(gnuplot_escape(id.as_title())));
    figure.configure(Key, |k| {
        k.set(Justification::Left)
            .set(Order::SampleText)
            .set(Position::Inside(Vertical::Top, Horizontal::Left))
    });

    let path = context.report_path(id, "iteration_times.svg");
    debug_script(&path, &figure);
    figure.set(Output(path)).draw().unwrap()
}

pub(crate) fn iteration_times_small(
    id: &BenchmarkId,
    context: &ReportContext,
    formatter: &dyn ValueFormatter,
    measurements: &MeasurementData<'_>,
    size: Option<Size>,
) -> Child {
    let mut figure = iteration_times_figure(formatter, measurements, size);
    figure.configure(Key, |k| k.hide());

    let path = context.report_path(id, "iteration_times_small.svg");
    debug_script(&path, &figure);
    figure.set(Output(path)).draw().unwrap()
}

fn iteration_times_comparison_figure(
    formatter: &dyn ValueFormatter,
    measurements: &MeasurementData<'_>,
    comparison: &ComparisonData,
    size: Option<Size>,
) -> Figure {
    let current_data = &measurements.avg_times;
    let base_data = &comparison.base_avg_times;

    let mut all_data: Vec<f64> = current_data.iter().map(|(f, _)| f).collect();
    all_data.extend_from_slice(base_data);

    let typical_value = Sample::new(&all_data).max();
    let unit = formatter.scale_values(typical_value, &mut all_data);

    let (scaled_current_y, scaled_base_y) = all_data.split_at(current_data.len());
    let scaled_current_y = Sample::new(scaled_current_y);
    let scaled_base_y = Sample::new(scaled_base_y);

    let mut figure = Figure::new();
    figure
        .set(Font(DEFAULT_FONT))
        .set(size.unwrap_or(SIZE))
        .configure(Axis::BottomX, |a| {
            a.configure(Grid::Major, |g| g.show()).set(Label("Sample"))
        })
        .configure(Axis::LeftY, |a| {
            a.configure(Grid::Major, |g| g.show())
                .set(Label(format!("Average Iteration Time ({})", unit)))
        })
        .configure(Key, |k| {
            k.set(Justification::Left)
                .set(Order::SampleText)
                .set(Position::Inside(Vertical::Top, Horizontal::Left))
        })
        .plot(
            Points {
                x: 1..(current_data.len() + 1),
                y: scaled_base_y.as_ref(),
            },
            |c| {
                c.set(DARK_RED)
                    .set(Label("Base"))
                    .set(PointSize(0.5))
                    .set(PointType::FilledCircle)
            },
        )
        .plot(
            Points {
                x: 1..(current_data.len() + 1),
                y: scaled_current_y.as_ref(),
            },
            |c| {
                c.set(DARK_BLUE)
                    .set(Label("Current"))
                    .set(PointSize(0.5))
                    .set(PointType::FilledCircle)
            },
        );
    figure
}

pub(crate) fn iteration_times_comparison(
    id: &BenchmarkId,
    context: &ReportContext,
    formatter: &dyn ValueFormatter,
    measurements: &MeasurementData<'_>,
    comparison: &ComparisonData,
    size: Option<Size>,
) -> Child {
    let mut figure = iteration_times_comparison_figure(formatter, measurements, comparison, size);
    figure.set(Title(gnuplot_escape(id.as_title())));

    let path = context.report_path(id, "both/iteration_times.svg");
    debug_script(&path, &figure);
    figure.set(Output(path)).draw().unwrap()
}

pub(crate) fn iteration_times_comparison_small(
    id: &BenchmarkId,
    context: &ReportContext,
    formatter: &dyn ValueFormatter,
    measurements: &MeasurementData<'_>,
    comparison: &ComparisonData,
    size: Option<Size>,
) -> Child {
    let mut figure = iteration_times_comparison_figure(formatter, measurements, comparison, size);
    figure.configure(Key, |k| k.hide());

    let path = context.report_path(id, "relative_iteration_times_small.svg");
    debug_script(&path, &figure);
    figure.set(Output(path)).draw().unwrap()
}
