use super::*;
use crate::measurement::ValueFormatter;
use crate::report::{BenchmarkId, ComparisonData, MeasurementData, ReportContext};
use plotters::data;
use plotters::style::RGBAColor;
use std::path::Path;

pub(crate) fn pdf_comparison_figure(
    path: &Path,
    title: Option<&str>,
    formatter: &dyn ValueFormatter,
    measurements: &MeasurementData<'_>,
    comparison: &ComparisonData,
    size: Option<(u32, u32)>,
) {
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

    let x_range = data::fitting_range(base_xs.iter().chain(xs.iter()));
    let y_range = data::fitting_range(base_ys.iter().chain(ys.iter()));

    let size = size.unwrap_or(SIZE);
    let root_area = SVGBackend::new(&path, (size.0, size.1)).into_drawing_area();

    let mut cb = ChartBuilder::on(&root_area);

    if let Some(title) = title {
        cb.caption(title, (DEFAULT_FONT, 20));
    }

    let mut chart = cb
        .margin((5).percent())
        .set_label_area_size(LabelAreaPosition::Left, (5).percent_width().min(60))
        .set_label_area_size(LabelAreaPosition::Bottom, (5).percent_height().min(40))
        .build_cartesian_2d(x_range, y_range.clone())
        .unwrap();

    chart
        .configure_mesh()
        .disable_mesh()
        .y_desc("Density (a.u.)")
        .x_desc(format!("Average Time ({})", unit))
        .x_label_formatter(&|&x| pretty_print_float(x, true))
        .y_label_formatter(&|&y| pretty_print_float(y, true))
        .x_labels(5)
        .draw()
        .unwrap();

    chart
        .draw_series(AreaSeries::new(
            base_xs.iter().zip(base_ys.iter()).map(|(x, y)| (*x, *y)),
            y_range.start,
            DARK_RED.mix(0.5).filled(),
        ))
        .unwrap()
        .label("Base PDF")
        .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 20, y + 5)], DARK_RED.mix(0.5).filled()));

    chart
        .draw_series(AreaSeries::new(
            xs.iter().zip(ys.iter()).map(|(x, y)| (*x, *y)),
            y_range.start,
            DARK_BLUE.mix(0.5).filled(),
        ))
        .unwrap()
        .label("New PDF")
        .legend(|(x, y)| {
            Rectangle::new([(x, y - 5), (x + 20, y + 5)], DARK_BLUE.mix(0.5).filled())
        });

    chart
        .draw_series(std::iter::once(PathElement::new(
            vec![(base_mean, 0.0), (base_mean, base_y_mean)],
            DARK_RED.filled().stroke_width(2),
        )))
        .unwrap()
        .label("Base Mean")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], DARK_RED));

    chart
        .draw_series(std::iter::once(PathElement::new(
            vec![(new_mean, 0.0), (new_mean, y_mean)],
            DARK_BLUE.filled().stroke_width(2),
        )))
        .unwrap()
        .label("New Mean")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], DARK_BLUE));

    if title.is_some() {
        chart.configure_series_labels().draw().unwrap();
    }
}

pub(crate) fn pdf_small(
    id: &BenchmarkId,
    context: &ReportContext,
    formatter: &dyn ValueFormatter,
    measurements: &MeasurementData<'_>,
    size: Option<(u32, u32)>,
) {
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

    let path = context.report_path(id, "pdf_small.svg");

    let size = size.unwrap_or(SIZE);
    let root_area = SVGBackend::new(&path, (size.0, size.1)).into_drawing_area();

    let mut chart = ChartBuilder::on(&root_area)
        .margin((5).percent())
        .set_label_area_size(LabelAreaPosition::Left, (5).percent_width().min(60))
        .set_label_area_size(LabelAreaPosition::Bottom, (5).percent_height().min(40))
        .build_cartesian_2d(xs_.min()..xs_.max(), 0.0..y_limit)
        .unwrap();

    chart
        .configure_mesh()
        .disable_mesh()
        .y_desc("Density (a.u.)")
        .x_desc(format!("Average Time ({})", unit))
        .x_label_formatter(&|&x| pretty_print_float(x, true))
        .y_label_formatter(&|&y| pretty_print_float(y, true))
        .x_labels(5)
        .draw()
        .unwrap();

    chart
        .draw_series(AreaSeries::new(
            xs.iter().zip(ys.iter()).map(|(x, y)| (*x, *y)),
            0.0,
            DARK_BLUE.mix(0.25).filled(),
        ))
        .unwrap();

    chart
        .draw_series(std::iter::once(PathElement::new(
            vec![(mean, 0.0), (mean, mean_y)],
            DARK_BLUE.filled().stroke_width(2),
        )))
        .unwrap();
}

pub(crate) fn pdf(
    id: &BenchmarkId,
    context: &ReportContext,
    formatter: &dyn ValueFormatter,
    measurements: &MeasurementData<'_>,
    size: Option<(u32, u32)>,
) {
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

    let path = context.report_path(id, "pdf.svg");

    let xs_ = Sample::new(&xs);

    let size = size.unwrap_or(SIZE);
    let root_area = SVGBackend::new(&path, (size.0, size.1)).into_drawing_area();

    let range = data::fitting_range(ys.iter());

    let mut chart = ChartBuilder::on(&root_area)
        .margin((5).percent())
        .caption(id.as_title(), (DEFAULT_FONT, 20))
        .set_label_area_size(LabelAreaPosition::Left, (5).percent_width().min(60))
        .set_label_area_size(LabelAreaPosition::Right, (5).percent_width().min(60))
        .set_label_area_size(LabelAreaPosition::Bottom, (5).percent_height().min(40))
        .build_cartesian_2d(xs_.min()..xs_.max(), 0.0..max_iters)
        .unwrap()
        .set_secondary_coord(xs_.min()..xs_.max(), 0.0..range.end);

    chart
        .configure_mesh()
        .disable_mesh()
        .y_desc(y_label)
        .x_desc(format!("Average Time ({})", unit))
        .x_label_formatter(&|&x| pretty_print_float(x, true))
        .y_label_formatter(&|&y| pretty_print_float(y * y_scale, true))
        .draw()
        .unwrap();

    chart
        .configure_secondary_axes()
        .y_desc("Density (a.u.)")
        .x_label_formatter(&|&x| pretty_print_float(x, true))
        .y_label_formatter(&|&y| pretty_print_float(y, true))
        .draw()
        .unwrap();

    chart
        .draw_secondary_series(AreaSeries::new(
            xs.iter().zip(ys.iter()).map(|(x, y)| (*x, *y)),
            0.0,
            DARK_BLUE.mix(0.5).filled(),
        ))
        .unwrap()
        .label("PDF")
        .legend(|(x, y)| {
            Rectangle::new([(x, y - 5), (x + 20, y + 5)], DARK_BLUE.mix(0.5).filled())
        });

    chart
        .draw_series(std::iter::once(PathElement::new(
            vec![(mean, 0.0), (mean, max_iters)],
            DARK_BLUE,
        )))
        .unwrap()
        .label("Mean")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], DARK_BLUE));

    chart
        .draw_series(vec![
            PathElement::new(vec![(lomt, 0.0), (lomt, max_iters)], DARK_ORANGE),
            PathElement::new(vec![(himt, 0.0), (himt, max_iters)], DARK_ORANGE),
            PathElement::new(vec![(lost, 0.0), (lost, max_iters)], DARK_RED),
            PathElement::new(vec![(hist, 0.0), (hist, max_iters)], DARK_RED),
        ])
        .unwrap();
    use crate::stats::univariate::outliers::tukey::Label;

    let mut draw_data_point_series =
        |filter: &dyn Fn(&Label) -> bool, color: RGBAColor, name: &str| {
            chart
                .draw_series(
                    avg_times
                        .iter()
                        .zip(scaled_avg_times.iter())
                        .zip(iter_counts.iter())
                        .filter_map(|(((_, label), t), i)| {
                            if filter(&label) {
                                Some(Circle::new((*t, *i), POINT_SIZE, color.filled()))
                            } else {
                                None
                            }
                        }),
                )
                .unwrap()
                .label(name)
                .legend(move |(x, y)| Circle::new((x + 10, y), POINT_SIZE, color.filled()));
        };

    draw_data_point_series(
        &|l| !l.is_outlier(),
        DARK_BLUE.to_rgba(),
        "\"Clean\" sample",
    );
    draw_data_point_series(
        &|l| l.is_mild(),
        RGBColor(255, 127, 0).to_rgba(),
        "Mild outliers",
    );
    draw_data_point_series(&|l| l.is_severe(), DARK_RED.to_rgba(), "Severe outliers");
    chart.configure_series_labels().draw().unwrap();
}
