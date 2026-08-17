use std::iter;
use std::process::Child;

use criterion_plot::prelude::*;

use super::*;
use crate::kde;
use crate::report::{BenchmarkId, ComparisonData, MeasurementData, ReportContext};

pub(crate) fn t_test(
    id: &BenchmarkId,
    context: &ReportContext,
    _measurements: &MeasurementData<'_>,
    comparison: &ComparisonData,
    size: Option<Size>,
) -> Child {
    let t = comparison.t_value;
    let (xs, ys) = kde::sweep(&comparison.t_distribution, KDE_POINTS, None);
    let zero = iter::repeat(0);

    let mut figure = Figure::new();
    figure
        .set(Font(DEFAULT_FONT))
        .set(size.unwrap_or(SIZE))
        .set(Title(format!(
            "{}: Welch t test",
            gnuplot_escape(id.as_title())
        )))
        .configure(Axis::BottomX, |a| a.set(Label("t score")))
        .configure(Axis::LeftY, |a| a.set(Label("Density")))
        .configure(Key, |k| {
            k.set(Justification::Left)
                .set(Order::SampleText)
                .set(Position::Outside(Vertical::Top, Horizontal::Right))
        })
        .plot(
            FilledCurve {
                x: &*xs,
                y1: &*ys,
                y2: zero,
            },
            |c| {
                c.set(DARK_BLUE)
                    .set(Label("t distribution"))
                    .set(Opacity(0.25))
            },
        )
        .plot(
            Lines {
                x: &[t, t],
                y: &[0, 1],
            },
            |c| {
                c.set(Axes::BottomXRightY)
                    .set(DARK_BLUE)
                    .set(LINEWIDTH)
                    .set(Label("t statistic"))
                    .set(LineType::Solid)
            },
        );

    let path = context.report_path(id, "change/t-test.svg");
    debug_script(&path, &figure);
    figure.set(Output(path)).draw().unwrap()
}
