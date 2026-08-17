use super::*;
use crate::report::ComparisonData;
use std::path::Path;

pub(crate) fn t_test(
    path: &Path,
    title: &str,
    comparison: &ComparisonData,
    size: Option<(u32, u32)>,
) {
    let t = comparison.t_value;
    let (xs, ys) = kde::sweep(&comparison.t_distribution, KDE_POINTS, None);

    let x_range = plotters::data::fitting_range(xs.iter());
    let mut y_range = plotters::data::fitting_range(ys.iter());
    y_range.start = 0.0;
    y_range.end *= 1.1;

    let root_area = SVGBackend::new(&path, size.unwrap_or(SIZE)).into_drawing_area();

    let mut chart = ChartBuilder::on(&root_area)
        .margin((5).percent())
        .caption(format!("{}: Welch t test", title), (DEFAULT_FONT, 20))
        .set_label_area_size(LabelAreaPosition::Left, (5).percent_width().min(60))
        .set_label_area_size(LabelAreaPosition::Bottom, (5).percent_height().min(40))
        .build_cartesian_2d(x_range, y_range.clone())
        .unwrap();

    chart
        .configure_mesh()
        .disable_mesh()
        .y_desc("Density")
        .x_desc("t score")
        .draw()
        .unwrap();

    chart
        .draw_series(AreaSeries::new(
            xs.iter().zip(ys.iter()).map(|(x, y)| (*x, *y)),
            0.0,
            DARK_BLUE.mix(0.25),
        ))
        .unwrap()
        .label("t distribution")
        .legend(|(x, y)| {
            Rectangle::new([(x, y - 5), (x + 20, y + 5)], DARK_BLUE.mix(0.25).filled())
        });

    chart
        .draw_series(std::iter::once(PathElement::new(
            vec![(t, 0.0), (t, y_range.end)],
            DARK_BLUE.filled().stroke_width(2),
        )))
        .unwrap()
        .label("t statistic")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], DARK_BLUE));

    chart.configure_series_labels().draw().unwrap();
}
